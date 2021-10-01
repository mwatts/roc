use crate::html::ToHtml;
use roc_ast::{ast_error::ASTResult, lang::{self, core::expr::expr_to_expr2::expr_to_expr2}, mem_pool::pool::Pool};
use roc_code_markup::{markup::nodes::expr2_to_markup, slow_pool::SlowPool};
use roc_module::symbol::Interns;
use roc_parse::ast::{Expr, StrLiteral};
use roc_region::all::{Region};
use roc_code_markup::{markup::nodes::{MarkupNode}};

impl<'a> ToHtml<'a> for MarkupNode {
    fn css_class(&self) -> Option<&'a str> {
        Some("operator")
    }
    fn html_body(&self, buf: &mut bumpalo::collections::String<'a>) {
        buf.push_str("MarkupNode")
    }
}

impl<'a> ToHtml<'a> for Expr<'a> {
    fn css_class(&self) -> Option<&'a str> {
        match self {
            Expr::Float(_) | Expr::Num(_) | Expr::NonBase10Int { .. } => Some("num"),
            Expr::Str(_) => Some("str"),
            // Expr::Access(_, _) => {}
            // Expr::AccessorFunction(_) => {}
            // Expr::List { .. } => {}
            // Expr::RecordUpdate { .. } => {}
            // Expr::Record { .. } => {}
            Expr::Var { .. } => None,
            // Expr::Underscore(_) => {}
            // Expr::GlobalTag(_) => {}
            // Expr::PrivateTag(_) => {}
            Expr::Closure(_, _) => None,

            // Expr::Defs(_, _) => {}
            // Expr::Backpassing(_, _, _) => {}
            // Expr::Expect(_, _) => {}
            // Expr::Apply(_, _, _) => {}
            // Expr::BinOps(_, _) => {}
            // Expr::UnaryOp(_, _) => {}
            // Expr::If(_, _) => {}
            // Expr::When(_, _) => {}
            Expr::SpaceBefore(_, _) => None,
            Expr::SpaceAfter(_, _) => None,
            // Expr::ParensAround(_) => {}
            // Expr::MalformedIdent(_, _) => {}
            // Expr::MalformedClosure => {}
            // Expr::PrecedenceConflict(_) => {}
            _ => None,
        }
    }
    fn html_body(&self, buf: &mut bumpalo::collections::String<'a>) {
        match self {
            Expr::Float(str) => {
                buf.push_str(str);
            }
            Expr::Num(str) => {
                buf.push_str(str);
            }
            Expr::NonBase10Int { string, .. } => {
                buf.push_str(string);
            }
            Expr::Str(str_literal) => match str_literal {
                StrLiteral::PlainLine(str) => {
                    buf.push('"');
                    buf.push_str(str);
                    buf.push('"');
                }
                StrLiteral::Line(line) => {
                    panic!("TODO str segments");
                }
                StrLiteral::Block(str) => {
                    panic!("TODO str segments");
                }
            },
            // Expr::Access(_, _) => {}
            // Expr::AccessorFunction(_) => {}
            // Expr::List { .. } => {}
            // Expr::RecordUpdate { .. } => {}
            // Expr::Record { .. } => {}
            Expr::Var { ident, module_name } => {
                if !module_name.is_empty() {
                    buf.push_str(module_name);
                    buf.push('.');
                }
                buf.push_str(ident);
            }
            // Expr::Underscore(_) => {}
            // Expr::GlobalTag(_) => {}
            // Expr::PrivateTag(_) => {}
            Expr::Closure(patterns, loc_sub_expr) => {
                ClosureDash.html(buf);

                let mut patterns_iter = patterns.iter().peekable();

                while let Some(pattern) = patterns_iter.next() {
                    pattern.value.html(buf);
                    if let Some(_) = patterns_iter.peek() {
                        ParamComma.html(buf);
                    }
                }

                ClosureArrow.html(buf);
                loc_sub_expr.html(buf);
            }
            Expr::Defs(defs, sub_expr) => {
                for def_loc in defs.iter() {
                    unimplemented!();
                    //def_loc.html(buf);
                }
                sub_expr.html(buf)
            }
            // Expr::Backpassing(_, _, _) => {}
            // Expr::Expect(_, _) => {}
            // Expr::Apply(_, _, _) => {}
            // Expr::BinOps(_, _) => {}
            // Expr::UnaryOp(_, _) => {}
            // Expr::If(_, _) => {}
            // Expr::When(_, _) => {}
            Expr::SpaceBefore(sub_expr, spaces) => {
                for space in spaces.iter() {
                    space.html(buf);
                }
                sub_expr.html(buf);
            }
            Expr::SpaceAfter(sub_expr, spaces) => {
                sub_expr.html(buf);
                for space in spaces.iter() {
                    space.html(buf);
                }
            }
            // Expr::ParensAround(_) => {}
            // Expr::MalformedIdent(_, _) => {}
            // Expr::MalformedClosure => {}
            // Expr::PrecedenceConflict(_) => {}
            _ => {}
        }
    }
}

struct ClosureDash;

impl<'a> ToHtml<'a> for ClosureDash {
    fn css_class(&self) -> Option<&'a str> {
        Some("closure-dash")
    }
    fn html_body(&self, buf: &mut bumpalo::collections::String<'a>) {
        buf.push('\\')
    }
}

struct ClosureArrow;

impl<'a> ToHtml<'a> for ClosureArrow {
    fn css_class(&self) -> Option<&'a str> {
        Some("closure-arrow")
    }
    fn html_body(&self, buf: &mut bumpalo::collections::String<'a>) {
        buf.push_str(" ->")
    }
}

struct ParamComma;

impl<'a> ToHtml<'a> for ParamComma {
    fn css_class(&self) -> Option<&'a str> {
        Some("param-comma")
    }
    fn html_body(&self, buf: &mut bumpalo::collections::String<'a>) {
        buf.push_str(", ")
    }
}

pub fn write_expr_to_bump_str_html<'a, 'b>(
    env: &mut lang::env::Env<'a>,
    scope: &mut lang::scope::Scope,
    region: Region,
    expr: &'a Expr,
    interns: &Interns,
    bump_str: &mut bumpalo::collections::String<'b>
) -> ASTResult<()> {
    let (expr2, _) = expr_to_expr2(env, scope, expr, region);

    let mut expr2_pool = Pool::with_capacity(1024);
    let expr2_id = expr2_pool.add(expr2);

    let mut mark_node_pool = SlowPool::default();

    let expr2_markup_id = expr2_to_markup(
        env,
        &expr2_pool.get(expr2_id),
        expr2_id,
        &mut mark_node_pool,
        interns,
    )?;

    let expr2_markup_node = mark_node_pool.get(expr2_markup_id);

    expr2_markup_node.html(bump_str);

    Ok(())
}
