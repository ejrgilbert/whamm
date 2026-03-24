use crate::emitter::memory_allocator::StringAddr;
use crate::emitter::utils::{emit_expr, EmitCtx};
use crate::emitter::InjectStrategy;
use crate::parser::types::{Definition, Expr, Value};
use std::collections::HashMap;
use wirm::ir::id::LocalID;
use wirm::ir::types::DataType as WirmType;
use wirm::module_builder::AddLocal;
use wirm::opcode::MacroOpcode;
use wirm::Opcode;

/// See Rust docs: https://doc.rust-lang.org/std/primitive.str.html
#[derive(Default)]
pub struct StringUtils {}
impl StringUtils {
    fn get_str(s: &Value) -> &str {
        let Value::Str { val: s } = s else {
            unreachable!("Should have gotten a string value for the variable.")
        };
        s
    }
    /// Returns the address of the string in memory.
    pub(crate) fn addr(s: &Value, emitted_strings: &HashMap<String, StringAddr>) -> u32 {
        let s = Self::get_str(s);
        emitted_strings.get(&s.to_string()).unwrap().mem_offset as u32
    }
    /// Returns the length of the string.
    /// This length is in bytes, not chars or graphemes. In other words, it might not be
    /// what a human considers the length of the string.
    pub(crate) fn len(s: &Value) -> u32 {
        let s = Self::get_str(s);
        s.len() as u32
    }
    /// Returns true if the given pattern matches a prefix of this string slice.
    /// Returns false if it does not.
    pub(crate) fn starts_with(s: &Value, args: &[Value]) -> bool {
        let s = Self::get_str(s);
        let prefix = Self::get_str(args.first().unwrap());
        s.starts_with(prefix)
    }
    /// Returns true if the given pattern matches a suffix of this string slice.
    /// Returns false if it does not.
    pub(crate) fn ends_with(s: &Value, args: &[Value]) -> bool {
        let s = Self::get_str(s);
        let suffix = Self::get_str(args.first().unwrap());
        s.ends_with(suffix)
    }
    /// Returns true if the given pattern matches a sub-slice of this string.
    /// Returns false if it does not.
    pub(crate) fn contains(s: &Value, args: &[Value]) -> bool {
        let s = Self::get_str(s);
        let pat = Self::get_str(args.first().unwrap());
        s.contains(pat)
    }

    // TODO: Once I support dynamic strings, add utilities
    //       that return a &str
    // TODO: Once I support chars, add utilities that use them
}
impl StringUtils {
    pub(crate) fn addr_of<'ir, T: Opcode<'ir> + MacroOpcode<'ir> + AddLocal>(
        target: &Expr,
        strategy: InjectStrategy,
        injector: &mut T,
        ctx: &mut EmitCtx,
    ) -> bool {
        // handle the string parameter
        emit_expr(target, None, strategy, injector, ctx);
        injector.drop(); // I don't care about the len pointer, just the addr!

        true
    }
    pub(crate) fn len_dynamic<'ir, T: Opcode<'ir> + MacroOpcode<'ir> + AddLocal>(
        target: &Expr,
        strategy: InjectStrategy,
        injector: &mut T,
        ctx: &mut EmitCtx,
    ) -> bool {
        // handle the string parameter
        emit_expr(target, None, strategy, injector, ctx);
        let str_len = LocalID(ctx.locals_tracker.use_local(WirmType::I32, injector));
        injector.local_set(str_len);
        injector.drop(); // I don't care about the addr pointer, just the len!

        injector.local_get(str_len); // bring back up the string len value to ToS
        true
    }

    pub(crate) fn starts_with_dynamic<'ir, T: Opcode<'ir> + MacroOpcode<'ir> + AddLocal>(
        target: &Expr,
        args: &[Expr],
        strategy: InjectStrategy,
        injector: &mut T,
        ctx: &mut EmitCtx,
    ) -> bool {
        let prefix = &args[0];

        // str0_addr = var.addr()
        Self::addr_of(target, strategy, injector, ctx);

        // str0_len = prefix.len()
        Self::len_dynamic(prefix, strategy, injector, ctx);

        // (str1_addr, str1_len)
        emit_expr(prefix, None, strategy, injector, ctx);

        emit_expr(
            &Expr::Call {
                fn_target: Box::new(Expr::VarId {
                    name: "strcmp".to_string(),
                    definition: Definition::CompilerDynamic,
                    loc: None,
                }),
                args: vec![],
                loc: None,
            },
            None,
            strategy,
            injector,
            ctx,
        )
    }

    pub(crate) fn ends_with_dynamic<'ir, T: Opcode<'ir> + MacroOpcode<'ir> + AddLocal>(
        target: &Expr,
        args: &[Expr],
        strategy: InjectStrategy,
        injector: &mut T,
        ctx: &mut EmitCtx,
    ) -> bool {
        let suffix = &args[0];

        // str0_addr = (var.addr() + (var.len() - 1)) - (suffix.len() - 1)
        Self::addr_of(target, strategy, injector, ctx);
        Self::len_dynamic(target, strategy, injector, ctx);
        injector.i32_const(1).i32_sub().i32_add();
        Self::len_dynamic(suffix, strategy, injector, ctx);
        injector.i32_const(1).i32_sub().i32_sub();

        // str0_len = suffix.len()
        Self::len_dynamic(suffix, strategy, injector, ctx);

        // (str1_addr, str1_len)
        emit_expr(suffix, None, strategy, injector, ctx);

        emit_expr(
            &Expr::Call {
                fn_target: Box::new(Expr::VarId {
                    name: "strcmp".to_string(),
                    definition: Definition::CompilerDynamic,
                    loc: None,
                }),
                args: vec![],
                loc: None,
            },
            None,
            strategy,
            injector,
            ctx,
        )
    }

    pub(crate) fn contains_dynamic<'ir, T: Opcode<'ir> + MacroOpcode<'ir> + AddLocal>(
        target: &Expr,
        args: &[Expr],
        strategy: InjectStrategy,
        injector: &mut T,
        ctx: &mut EmitCtx,
    ) -> bool {
        let needle = &args[0];

        // Push (hs_addr, hs_len, nd_addr, nd_len) onto the stack and call $strcontains.
        // Using a proper function call avoids inlining `return` instructions that would
        // exit the enclosing probe function rather than just this helper.

        // (hs_addr, hs_len)
        emit_expr(target, None, strategy, injector, ctx);
        // (nd_addr, nd_len)
        emit_expr(needle, None, strategy, injector, ctx);

        emit_expr(
            &Expr::Call {
                fn_target: Box::new(Expr::VarId {
                    name: "strcontains".to_string(),
                    definition: Definition::CompilerDynamic,
                    loc: None,
                }),
                args: vec![],
                loc: None,
            },
            None,
            strategy,
            injector,
            ctx,
        );

        true
    }
}
