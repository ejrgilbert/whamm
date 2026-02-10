use std::alloc::{alloc, dealloc, Layout};
use std::ptr;

#[no_mangle]
pub fn write_alphabet(ptr: *mut u8, max_len: usize) -> usize {
    let s = String::from("abcdefghijklmnopqrstuvwxyz\n");

    // let max_bytes = max_len.saturating_sub(1);
    let bytes = s.as_bytes();
    let len = bytes.len().min(max_len);

    unsafe {
        ptr::copy_nonoverlapping(bytes.as_ptr(), ptr, len);
        // ptr.add(len).write(b'\0');
    }

    len // Return bytes written, including null
}

/// Allocates a contiguous region of memory and returns a pointer to it.
///
/// # ABI & Memory Layout
///
/// This function allocates `len` bytes of usable memory, but internally
/// reserves extra space to store allocation metadata.
///
/// The actual layout in memory is:
///
/// ```text
/// [ usize: len ][ len bytes of user data ... ]
/// ^
/// |
/// pointer returned to the caller points here
/// ```
///
/// The stored `len` is used later by [`mem_free`] to correctly reconstruct
/// the allocation layout for deallocation.
///
/// # Safety & Usage
///
/// - The returned pointer is guaranteed to be valid for reads and writes
///   of `len` bytes.
/// - The pointer must eventually be passed to [`mem_free`] **exactly once**.
/// - The pointer must not be freed by any other allocator (e.g. `free`,
///   `delete`, or host runtime deallocators).
/// - Passing a pointer not produced by `mem_alloc` to `mem_free` is
///   **undefined behavior**.
///
/// # FFI / Wasm Notes
///
/// This design is intentionally compatible with C / JS / Wasm hosts:
/// - Callers do **not** need to track the allocation size.
/// - The allocation metadata is fully self-describing.
/// - The allocator used is Rust’s global allocator, so allocation and
///   deallocation remain consistent.
///
/// # Parameters
///
/// * `len` — Number of bytes requested.
///
/// # Returns
///
/// A non-null pointer to `len` bytes of memory on success, or a null
/// pointer if allocation fails or `len == 0`.
#[no_mangle]
pub fn mem_alloc(len: usize) -> *mut u8 {
    if len == 0 {
        return ptr::null_mut();
    }

    let header_size = size_of::<usize>();
    let total_size = header_size + len;

    // Align to usize so the header is properly aligned
    let layout = match Layout::from_size_align(
        total_size,
        align_of::<usize>(),
    ) {
        Ok(layout) => layout,
        Err(_) => return ptr::null_mut(),
    };

    unsafe {
        let raw = alloc(layout);
        if raw.is_null() {
            return ptr::null_mut();
        }

        // Write allocation length into header
        *(raw as *mut usize) = len;

        // Return pointer just past the header
        raw.add(header_size)
    }
}

/// Frees memory previously allocated by [`mem_alloc`].
///
/// # Safety & Invariants
///
/// This function assumes:
/// - `ptr` was returned by a previous call to [`mem_alloc`].
/// - `ptr` has not already been freed.
/// - The memory header preceding `ptr` is intact.
///
/// Violating any of these assumptions results in **undefined behavior**.
///
/// # How It Works
///
/// `mem_free` reconstructs the original allocation layout by:
/// 1. Moving the pointer backward to read the stored allocation size
/// 2. Rebuilding the original [`Layout`]
/// 3. Calling Rust’s global deallocator
///
/// # Parameters
///
/// * `ptr` — Pointer returned by [`mem_alloc`].
///
/// # Notes
///
/// - Passing a null pointer is a no-op.
/// - This function must be used instead of `free` or other host
///   deallocation functions.
#[no_mangle]
pub fn mem_free(ptr: *mut u8) {
    if ptr.is_null() {
        return;
    }

    let header_size = size_of::<usize>();

    unsafe {
        // Move back to the start of the allocation header
        let raw = ptr.sub(header_size);

        // Read stored length
        let len = *(raw as *mut usize);
        let total_size = header_size + len;

        let layout = match Layout::from_size_align(
            total_size,
            align_of::<usize>(),
        ) {
            Ok(layout) => layout,
            Err(_) => return, // layout corruption → best effort no-op
        };

        dealloc(raw, layout);
    }
}
