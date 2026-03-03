use crate::emitter::utils::{emit_expr, EmitCtx};
use crate::emitter::InjectStrategy;
use crate::parser::types::{Definition, Expr, Value};
use wirm::ir::id::LocalID;
use wirm::ir::types::{BlockType, DataType as WirmType};
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
    pub(crate) fn addr_of<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
        target: &mut Expr,
        strategy: InjectStrategy,
        injector: &mut T,
        ctx: &mut EmitCtx,
    ) -> bool {
        // handle the string parameter
        emit_expr(target, None, strategy, injector, ctx);
        injector.drop(); // I don't care about the len pointer, just the addr!

        true
    }
    pub(crate) fn len_dynamic<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
        target: &mut Expr,
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

    pub(crate) fn starts_with_dynamic<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
        target: &mut Expr,
        args: &mut [Expr],
        strategy: InjectStrategy,
        injector: &mut T,
        ctx: &mut EmitCtx,
    ) -> bool {
        let prefix = &mut args[0];

        // str0_addr = var.addr()
        Self::addr_of(target, strategy, injector, ctx);

        // str0_len = prefix.len()
        Self::len_dynamic(prefix, strategy, injector, ctx);

        // (str1_addr, str1_len)
        emit_expr(prefix, None, strategy, injector, ctx);

        emit_expr(
            &mut Expr::Call {
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

    pub(crate) fn ends_with_dynamic<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
        target: &mut Expr,
        args: &mut [Expr],
        strategy: InjectStrategy,
        injector: &mut T,
        ctx: &mut EmitCtx,
    ) -> bool {
        let suffix = &mut args[0];

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
            &mut Expr::Call {
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

    pub(crate) fn contains_dynamic<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
        target: &mut Expr,
        args: &mut [Expr],
        strategy: InjectStrategy,
        injector: &mut T,
        ctx: &mut EmitCtx,
    ) -> bool {
        let needle = &mut args[0];

        // --- Locals ---
        let haystack_addr = LocalID(ctx.locals_tracker.use_local(WirmType::I32, injector));
        let haystack_len = LocalID(ctx.locals_tracker.use_local(WirmType::I32, injector));
        let needle_addr = LocalID(ctx.locals_tracker.use_local(WirmType::I32, injector));
        let needle_len = LocalID(ctx.locals_tracker.use_local(WirmType::I32, injector));
        let i_local = LocalID(ctx.locals_tracker.use_local(WirmType::I32, injector));

        // --- Evaluate haystack ---
        emit_expr(target, None, strategy, injector, ctx);
        injector.local_set(haystack_len).local_set(haystack_addr);

        // --- Evaluate needle ---
        emit_expr(needle, None, strategy, injector, ctx);
        injector.local_set(needle_len).local_set(needle_addr);

        // if needle_len == 0 → true
        injector
            .local_get(needle_len)
            .i32_eqz()
            .if_stmt(BlockType::Empty)
            .i32_const(1)
            .return_stmt()
            .end();

        // if needle_len > haystack_len → false
        injector
            .local_get(needle_len)
            .local_get(haystack_len)
            .i32_gt_unsigned()
            .if_stmt(BlockType::Empty)
            .i32_const(0)
            .return_stmt()
            .end();

        // i = 0
        injector.i32_const(0).local_set(i_local);

        // block (for FALSE)
        injector.block(BlockType::Empty);
        // block (for TRUE)
        injector.block(BlockType::Empty);

        // loop
        injector.loop_stmt(BlockType::Empty);

        // if i > haystack_len - needle_len → break false
        injector.local_get(i_local);
        injector.local_get(haystack_len);
        injector.local_get(needle_len);
        injector.i32_sub();
        injector.i32_gt_unsigned();
        injector.br_if(2); // break to FALSE block result

        // strcmp(haystack_addr + i, needle_len, needle_addr, needle_len)
        injector.local_get(haystack_addr);
        injector.local_get(i_local);
        injector.i32_add();

        injector.local_get(needle_len);

        injector.local_get(needle_addr);
        injector.local_get(needle_len);

        emit_expr(
            &mut Expr::Call {
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
        );

        // if match → break true
        injector.br_if(1);

        // i++
        injector.local_get(i_local);
        injector.i32_const(1);
        injector.i32_add();
        injector.local_set(i_local);

        injector.br(0); // continue loop

        injector.end(); // loop
        injector.end(); // block (for TRUE)

        // return true
        injector.i32_const(1);
        injector.return_stmt();

        injector.end(); // block (for FALSE)

        // return false
        injector.i32_const(0); // if needle_len > haystack_len OR not found
        injector.return_stmt();

        true
    }
}
