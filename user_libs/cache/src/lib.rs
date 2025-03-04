use std::sync::Mutex;
use once_cell::sync::Lazy;

/// Used by instrumentation to check the cache hit/miss
/// Returns a bit-packed u32 which represents two bools.
/// 0:16 --> addr0 is hit
/// 17:32 --> addr1 is hit (0xFFFF if not calculated)

const ASSOC: usize = 4;        // 4 way set-associative
const BLOCK_SIZE: u32 = 128; // 128 byte block
const BLOCK_BITS: u8 = 7;  // 128 byte block
const INDEX_BITS: u8 = 11; // 32 - (BLOCK_BITS + TAG_BITS)
const TAG_BITS: u8 = 14;   // TagElement.tag: u32;
const NUM_SETS: u32 = 2047;  // Number of sets that can be represented with INDEX_BITS
static MY_CACHE: Lazy<Mutex<CacheInstance>> = Lazy::new(|| Mutex::new(CacheInstance::new(ASSOC, BLOCK_BITS, INDEX_BITS, TAG_BITS)));
#[no_mangle]
pub fn check_access(addr: u32, data_size: u8) -> u32 {
    let addrs = calculate_address(addr as u64, data_size);

    let cache_res0 = check_access_internal(addrs.primary);

    let cache_res1 = if let Some(secondary) = addrs.secondary {
        Some(check_access_internal(secondary))
    } else {
        None
    };

    let mut res = 0;
    if cache_res0 {
        res |= 1;
        res = res << 16;
    }
    if let Some(cache_res1) = cache_res1 {
        if cache_res1 {
            res |= 1;
        }
    } else {
        res |= 0xFFFF;
    }

    res
}

fn calculate_address(addr: u64, data_size: u8) -> CacheAddresses {
    // note: I don't have to store the actual data, just the addresses to see if there's a hit/miss
    // How to normalize a number: https://www.statology.org/normalize-data-between-0-and-100/
    let tag = get_tag(addr);
    let index = get_index(tag);
    let offset = get_block_offset(addr);

    let primary = get_address_from_parts(tag, index, offset);

    // see if the address is as-expected
    let mut secondary = None;
    if offset + data_size as u32 > BLOCK_SIZE {
        // data access spans two blocks! check the other as well
        let secondary_tag = tag + 1;
        let secondary_index = get_index(secondary_tag);
        let secondary_offset = 0u32;
        secondary = Some(get_address_from_parts(secondary_tag, secondary_index, secondary_offset));
    }

    CacheAddresses {
        primary,
        secondary
    }
}
fn get_tag(addr: u64) -> u32 {
    (addr / BLOCK_SIZE as u64) as u32
}
fn get_index(target_block: u32) -> u32 {
    (target_block % NUM_SETS) as u32
}
fn get_block_offset(addr: u64) -> u32 {
    (addr % BLOCK_SIZE as u64) as u32
}

fn get_address_from_parts(tag: u32, index: u32, offset: u32) -> u32 {
    let mut addr: u32 = 0x00000000;

    // add the tag
    addr |= tag;
    addr <<= INDEX_BITS;

    // add the index
    addr |= index;
    addr <<= BLOCK_BITS;

    // add the block offset
    addr |= offset;

    return addr;
}

fn check_access_internal(addr: u32) -> bool {
    matches!(MY_CACHE.lock().unwrap().access(addr), LRUResult::Hit{..})
}

struct CacheAddresses { primary: u32, secondary: Option<u32>}

#[derive(Debug, Default)]
struct TagElement {
    pub valid: bool,
    pub tag: u32,
    pub age: u64
}
impl TagElement {
    fn new(valid: bool, tag: u32, age: u64) -> Self {
        Self {
            valid,
            tag,
            age
        }
    }
}

enum LRUResult {
    Hit {t: u32 },
    Miss{t: u32, is_evict: bool, evict: u32}
}

#[derive(Default)]
struct TagStoreEntry {
    elems: [TagElement; ASSOC]
}
impl TagStoreEntry {
    fn is_elem_valid(&self, t: TagElement) -> bool {
        return t.valid;
    }
    pub fn access_update(&mut self, tag: u32) -> LRUResult {
        let mut rep_idx = None;
        let mut is_hit = false;
        let mut rep_invalid = false;
        let mut max_age: u64 = 0;

        let mut result: LRUResult;
        for (i, elem) in self.elems.iter_mut().enumerate() {
            if elem.valid {
                if elem.tag == tag {
                    // Check for Hit; reset age
                    is_hit = true;
                    *elem = TagElement::new(true, tag, 0);
                } else {
                    if !rep_invalid && (elem.age >= max_age) {
                        // Track oldest element for eviction as long as
                        // there is no invalid element to replace
                        max_age = elem.age;
                        rep_idx = Some(i);
                    }
                    // Increment age of all valid elements besides hit
                    *elem = TagElement::new(elem.valid, elem.tag, elem.age + 1);
                }
            } else {
                // There exists an invalid element to fill for miss
                rep_invalid = true;
                rep_idx = Some(i);
            }
        }

        if is_hit {
            return LRUResult::Hit{t: tag};
        } else if rep_invalid {
            if let Some(idx) = rep_idx {
                self.elems[idx as usize] = TagElement::new(true, tag, 0);
                return LRUResult::Miss{t: tag, is_evict: false, evict: 0};
            } else {
                panic!("rep_idx was never initialized")
            }
        } else {
            if let Some(idx) = rep_idx {
                // All elements are valid and no hit; evict with LRU
                let evict_tag = self.elems[idx as usize].tag;
                self.elems[idx as usize] = TagElement::new(true, tag, 0);
                return LRUResult::Miss{t: tag, is_evict: true, evict: evict_tag};
            } else {
                panic!("rep_idx was never initialized")
            }
        }
    }
}

struct CacheInstance {
    tag_store: Vec<TagStoreEntry>,

    assoc: usize,
    tag_bits: u8,
    block_bits: u8,
    index_bits: u8,

    block_size: i32,
    index_size: i32,
}
impl CacheInstance {
    fn new(assoc: usize, block_bits: u8, index_bits: u8, tag_bits: u8) -> Self {
        if (tag_bits + index_bits + block_bits) != 32 {
            panic!("Cache parameters do not add up to 32 bits.\n");
        }
        let index_size = 1 << index_bits;

        let mut tag_store = vec![];
        for i in 0..index_size {
            tag_store.push(TagStoreEntry::default());
        }
        Self {
            tag_store,
            assoc,
            tag_bits,
            block_bits,
            index_bits,
            index_size,
            block_size: (1 << block_bits),
        }
    }
    fn access(&mut self, mut addr: u32) -> LRUResult {
        let offset = addr & ((1u32 << self.block_bits) - 1);
        addr = addr >> self.block_bits;
        let index = addr & ((1u32 << self.index_bits) - 1);
        addr = addr >> self.index_bits;
        let tag_val = addr & ((1u32 << self.tag_bits) - 1);

        if let Some(store) = self.tag_store.get_mut(index as usize) {
            return store.access_update(tag_val);
        } else {
            panic!("Could not get TagStoreEntry at {index}");
        }
    }
}

fn run_test(addr: u64, exp_prim: u32, exp_second: Option<u32>) {
    let CacheAddresses {primary, secondary} = calculate_address(addr, 0);
    assert_eq!(exp_prim, primary);
    assert_eq!(exp_second, secondary);
}

#[test]
fn calc_addr() {
    run_test(3664, 7343696, None);
    run_test(70796, 145036428, None);
    run_test(3692, 7343724, None);
    run_test(4140, 8392748, None);
    run_test(4152, 8392760, None);
    run_test(4144, 8392752, None);
    run_test(4160, 8392768, None);
    run_test(4112, 8392720, None);
    run_test(70816, 145036448, None);
}