use crate::{
    parse::{FuncKind, Node, NodeKind, Program, ScopeKind},
    ty::TypeKind,
};
use anyhow::{anyhow, Result};
use std::fmt::Write;

pub fn codegen(buf: &mut String, program: &Program) -> Result<()> {
    let funcs = &program.funcs;
    let globals = &program.globals;

    // Firstly, generate init function
    let init_func = funcs.iter().find(|f| f.kind == FuncKind::Init);
    match init_func {
        Some(func) => {
            writeln!(buf, "init:")?;
            if !func.is_naked {
                writeln!(buf, "  push ra")?;
                writeln!(buf, "  push fp")?;
                writeln!(buf, "  rsp fp")?;
                writeln!(buf, "  lil t0, 0x{:04x}@l", func.local_offset)?;
                writeln!(buf, "  lih t1, 0x{:04x}@h", func.local_offset)?;
                writeln!(buf, "  or t0, t1")?;
                writeln!(buf, "  mov t1, fp")?;
                writeln!(buf, "  sub t1, t0")?;
                writeln!(buf, "  wsp t1")?;
            }
            for node in &func.nodes {
                match node.kind {
                    NodeKind::Null => {}
                    NodeKind::Asm(_) => {
                        gen(buf, node)?;
                    }
                    _ => {
                        gen(buf, node)?;
                        writeln!(buf, "  pop a0")?;
                    }
                }
            }
        }
        None => {
            writeln!(buf, "init:")?;
            writeln!(buf, "  lil a0, 0xffff@l")?;
            writeln!(buf, "  lih a1, 0xffff@h")?;
            writeln!(buf, "  or a0, a1")?;
            writeln!(buf, "  wsp a0")?;
            writeln!(buf, "  mov fp, a0")?;
        }
    }

    // Secondly, generate main functions
    let main_func = funcs.iter().find(|f| f.kind == FuncKind::Main);
    match main_func {
        Some(func) => {
            writeln!(buf, "main:")?;
            if !func.is_naked {
                writeln!(buf, "  push ra")?;
                writeln!(buf, "  push fp")?;
                writeln!(buf, "  rsp fp")?;
                writeln!(buf, "  lil t0, 0x{:04x}@l", func.local_offset)?;
                writeln!(buf, "  lih t1, 0x{:04x}@h", func.local_offset)?;
                writeln!(buf, "  or t0, t1")?;
                writeln!(buf, "  mov t1, fp")?;
                writeln!(buf, "  sub t1, t0")?;
                writeln!(buf, "  wsp t1")?;
            }
            for node in &func.nodes {
                match node.kind {
                    NodeKind::Null => {}
                    NodeKind::Asm(_) => {
                        gen(buf, node)?;
                    }
                    _ => {
                        gen(buf, node)?;
                        writeln!(buf, "  pop a0")?;
                    }
                }
            }
        }
        None => {
            return Err(anyhow!("Main function is not defined"));
        }
    }

    // Finally, generate other functions
    for func in funcs {
        if func.kind == FuncKind::Other {
            writeln!(buf, "{}:", func.name)?;
            if !func.is_naked {
                writeln!(buf, "  push ra")?;
                writeln!(buf, "  push fp")?;
                writeln!(buf, "  rsp fp")?;
                writeln!(buf, "  lil t0, 0x{:04x}@l", func.local_offset)?;
                writeln!(buf, "  lih t1, 0x{:04x}@h", func.local_offset)?;
                writeln!(buf, "  or t0, t1")?;
                writeln!(buf, "  mov t1, fp")?;
                writeln!(buf, "  sub t1, t0")?;
                writeln!(buf, "  wsp t1")?;
            }
            for (i, arg) in func.args.iter().enumerate() {
                if arg.ty.clone().unwrap().kind == TypeKind::Char {
                    writeln!(buf, "  sh a{}, fp, {}", i, -((i as i8) + 1))?;
                } else {
                    writeln!(buf, "  sw a{}, fp, {}", i, -2 * ((i as i8) + 1))?;
                }
            }

            for node in &func.nodes {
                match node.kind {
                    NodeKind::Null => {}
                    NodeKind::Asm(_) => {
                        gen(buf, node)?;
                    }
                    _ => {
                        gen(buf, node)?;
                        writeln!(buf, "  pop a0")?;
                    }
                }
            }
        }
    }

    // global variables are generated
    for global in globals {
        match global.kind {
            ScopeKind::Var => {
                writeln!(buf, "{}:", global.name)?;
                for _ in 0..global.ty.clone().unwrap().size {
                    writeln!(buf, "  .byte 0x00")?;
                }
            }
            ScopeKind::Str => {
                writeln!(buf, "{}:", global.name)?;
                for c in global.str.clone().unwrap().chars() {
                    writeln!(buf, "  .byte 0x{:02x}", c as u8)?;
                }
            }
            _ => {}
        }
    }
    Ok(())
}

fn gen(buf: &mut String, node: &Node) -> Result<()> {
    match &node.kind {
        NodeKind::Num(n) => {
            writeln!(buf, "  lil a0, 0x{:04x}@l", n.val)?;
            writeln!(buf, "  lih a1, 0x{:04x}@h", n.val)?;
            writeln!(buf, "  or a0, a1")?;
            writeln!(buf, "  push a0")?;
        }

        NodeKind::Add(n) => {
            gen(buf, &n.left)?;
            gen(buf, &n.right)?;
            writeln!(buf, "  pop a1")?;
            writeln!(buf, "  pop a0")?;
            writeln!(buf, "  add a0, a1")?;
            writeln!(buf, "  push a0")?;
        }
        NodeKind::Sub(n) => {
            gen(buf, &n.left)?;
            gen(buf, &n.right)?;
            writeln!(buf, "  pop a1")?;
            writeln!(buf, "  pop a0")?;
            writeln!(buf, "  sub a0, a1")?;
            writeln!(buf, "  push a0")?;
        }
        NodeKind::Mul(n) => {
            gen(buf, &n.left)?;
            gen(buf, &n.right)?;
            writeln!(buf, "  pop a1")?;
            writeln!(buf, "  pop a0")?;
            writeln!(buf, "  mov t0, a0")?;
            writeln!(buf, "  subi a1, a1, 1")?;
            writeln!(buf, "  beq a1, zero, 6")?;
            writeln!(buf, "  add a0, t0")?;
            writeln!(buf, "  jal zero, -6")?;
            writeln!(buf, "  push a0")?;
        }
        NodeKind::Div(n) => {
            gen(buf, &n.left)?;
            gen(buf, &n.right)?;
            writeln!(buf, "  pop a1")?;
            writeln!(buf, "  pop a0")?;
            writeln!(buf, "  mov t0, zero")?;
            writeln!(buf, "  blt a0, a1, 8")?;
            writeln!(buf, "  addi t0, t0, 1")?;
            writeln!(buf, "  sub a0, a1")?;
            writeln!(buf, "  jal zero, -6")?;
            writeln!(buf, "  mov a0, t0")?;
            writeln!(buf, "  push a0")?;
        }
        NodeKind::Eq(n) => {
            gen(buf, &n.left)?;
            gen(buf, &n.right)?;
            writeln!(buf, "  pop a1")?;
            writeln!(buf, "  pop a0")?;
            writeln!(buf, "  addi t0, zero, 1")?;
            writeln!(buf, "  beq a0, a1, 4")?;
            writeln!(buf, "  subi t0, t0, 1")?;
            writeln!(buf, "  mov a0, t0")?;
            writeln!(buf, "  push a0")?;
        }
        NodeKind::Ne(n) => {
            gen(buf, &n.left)?;
            gen(buf, &n.right)?;
            writeln!(buf, "  pop a1")?;
            writeln!(buf, "  pop a0")?;
            writeln!(buf, "  addi t0, zero, 1")?;
            writeln!(buf, "  bnq a0, a1, 4")?;
            writeln!(buf, "  subi t0, t0, 1")?;
            writeln!(buf, "  mov a0, t0")?;
            writeln!(buf, "  push a0")?;
        }
        NodeKind::Lt(n) => {
            gen(buf, &n.left)?;
            gen(buf, &n.right)?;
            writeln!(buf, "  pop a1")?;
            writeln!(buf, "  pop a0")?;
            writeln!(buf, "  addi t0, zero, 1")?;
            writeln!(buf, "  blt a0, a1, 4")?;
            writeln!(buf, "  subi t0, t0, 1")?;
            writeln!(buf, "  mov a0, t0")?;
            writeln!(buf, "  push a0")?;
        }
        NodeKind::Le(n) => {
            gen(buf, &n.left)?;
            gen(buf, &n.right)?;
            writeln!(buf, "  pop a1")?;
            writeln!(buf, "  pop a0")?;
            writeln!(buf, "  addi t0, zero, 1")?;
            writeln!(buf, "  bge a1, a0, 4")?;
            writeln!(buf, "  subi t0, t0, 1")?;
            writeln!(buf, "  mov a0, t0")?;
            writeln!(buf, "  push a0")?;
        }
        NodeKind::Return(n) => {
            gen(buf, &n.expr)?;
            writeln!(buf, "  pop a0")?;
            writeln!(buf, "  wsp fp")?;
            writeln!(buf, "  pop fp")?;
            writeln!(buf, "  pop ra")?;
            writeln!(buf, "  jalr zero, ra, 0")?;
        }
        NodeKind::Var(_) | NodeKind::MemAccess(_) => {
            gen_lval(buf, node)?;
            if node.ty.clone().unwrap().kind != TypeKind::Array {
                writeln!(buf, "  pop a0")?;
                if node.ty.clone().unwrap().kind == TypeKind::Char {
                    writeln!(buf, "  lh a0, a0, 0")?;
                } else {
                    writeln!(buf, "  lw a0, a0, 0")?;
                }
                writeln!(buf, "  push a0")?;
            }
        }
        NodeKind::Assign(n) => {
            gen_lval(buf, &n.left)?;
            gen(buf, &n.right)?;
            writeln!(buf, "  pop a1")?;
            writeln!(buf, "  pop a0")?;
            if node.ty.clone().unwrap().kind == TypeKind::Char {
                writeln!(buf, "  sh a1, a0, 0")?;
            } else {
                writeln!(buf, "  sw a1, a0, 0")?;
            }
            writeln!(buf, "  push a1")?;
        }
        NodeKind::If(n) => {
            gen(buf, &n.cond)?;
            writeln!(buf, "  pop a0")?;
            writeln!(buf, "  bnq a0, zero, 10")?;
            writeln!(buf, "  lil a0, else{}@l", n.label)?;
            writeln!(buf, "  lih a1, else{}@h", n.label)?;
            writeln!(buf, "  or a0, a1")?;
            writeln!(buf, "  jalr zero, a0, 0")?;
            gen(buf, &n.then)?;
            writeln!(buf, "  lil a0, end{}@l", n.label)?;
            writeln!(buf, "  lih a1, end{}@h", n.label)?;
            writeln!(buf, "  or a0, a1")?;
            writeln!(buf, "  jalr zero, a0, 0")?;
            writeln!(buf, "else{}:", n.label)?;
            if let Some(n) = &n.els {
                gen(buf, n)?;
            } else {
                writeln!(buf, "  push a0")?; // for when n.then is not excuted
            }
            writeln!(buf, "end{}:", n.label)?;
        }
        NodeKind::While(n) => {
            writeln!(buf, "begin{}:", n.label)?;
            gen(buf, &n.cond)?;
            writeln!(buf, "  pop a0")?;
            writeln!(buf, "  bnq a0, zero, 10")?;
            writeln!(buf, "  lil a0, end{}@l", n.label)?;
            writeln!(buf, "  lih a1, end{}@h", n.label)?;
            writeln!(buf, "  or a0, a1")?;
            writeln!(buf, "  jalr zero, a0, 0")?;
            gen(buf, &n.then)?;
            writeln!(buf, "  lil a0, begin{}@l", n.label)?;
            writeln!(buf, "  lih a1, begin{}@h", n.label)?;
            writeln!(buf, "  or a0, a1")?;
            writeln!(buf, "  jalr zero, a0, 0")?;
            writeln!(buf, "end{}:", n.label)?;
        }
        NodeKind::For(n) => {
            if let Some(n) = &n.init {
                gen(buf, n)?;
            }
            writeln!(buf, "begin{}:", n.label)?;
            if let Some(c) = &n.cond {
                gen(buf, c)?;
                writeln!(buf, "  pop a0")?;
                writeln!(buf, "  bnq a0, zero, 10")?;
                writeln!(buf, "  lil a0, end{}@l", n.label)?;
                writeln!(buf, "  lih a1, end{}@h", n.label)?;
                writeln!(buf, "  or a0, a1")?;
                writeln!(buf, "  jalr zero, a0, 0")?;
            }
            if let Some(n) = &n.then {
                gen(buf, n)?;
            }
            if let Some(i) = &n.inc {
                writeln!(buf, "inc{}:", n.label)?;
                gen(buf, i)?;
            }
            writeln!(buf, "  lil a0, begin{}@l", n.label)?;
            writeln!(buf, "  lih a1, begin{}@h", n.label)?;
            writeln!(buf, "  or a0, a1")?;
            writeln!(buf, "  jalr zero, a0, 0")?;
            writeln!(buf, "end{}:", n.label)?;
        }
        NodeKind::Break(n) => {
            writeln!(buf, "  lil a0, end{}@l", n.label)?;
            writeln!(buf, "  lih a1, end{}@h", n.label)?;
            writeln!(buf, "  or a0, a1")?;
            writeln!(buf, "  jalr zero, a0, 0")?;
        }
        NodeKind::Continue(n) => {
            writeln!(buf, "  lil a0, inc{}@l", n.label)?;
            writeln!(buf, "  lih a1, inc{}@h", n.label)?;
            writeln!(buf, "  or a0, a1")?;
            writeln!(buf, "  jalr zero, a0, 0")?;
        }
        NodeKind::Block(n) => {
            for node in &n.body {
                gen(buf, node)?;
            }
        }
        NodeKind::FuncCall(n) => {
            for arg in &n.args {
                gen(buf, arg)?;
            }
            for n in (0..n.args.len()).rev() {
                writeln!(buf, "  pop a{}", n)?;
            }

            writeln!(buf, "  lil ra, {}@l", n.name)?;
            writeln!(buf, "  lih t0, {}@h", n.name)?;
            writeln!(buf, "  or ra, t0")?;
            writeln!(buf, "  jalr ra, ra, 0")?;
            writeln!(buf, "  push a0")?;
        }
        NodeKind::Addr(n) => {
            gen_lval(buf, &n.unary)?;
        }
        NodeKind::Deref(n) => {
            gen(buf, &n.unary)?;
            if node.ty.clone().unwrap().kind != TypeKind::Array {
                writeln!(buf, "  pop a0")?;
                if node.ty.clone().unwrap().kind == TypeKind::Char {
                    writeln!(buf, "  lh a0, a0, 0")?;
                } else {
                    writeln!(buf, "  lw a0, a0, 0")?;
                }
                writeln!(buf, "  push a0")?;
            }
        }
        NodeKind::PtrAdd(n) => {
            gen(buf, &n.left)?;
            gen(buf, &n.right)?;
            writeln!(
                buf,
                "  addi a0, zero, {}",
                node.ty.clone().unwrap().ptr_to.unwrap().size
            )?;
            writeln!(buf, "  push a0")?;
            writeln!(buf, "  pop a1")?;
            writeln!(buf, "  pop a0")?;
            writeln!(buf, "  mov t0, a0")?;
            writeln!(buf, "  subi a1, a1, 1")?;
            writeln!(buf, "  beq a1, zero, 6")?;
            writeln!(buf, "  add a0, t0")?;
            writeln!(buf, "  jal zero, -6")?;
            writeln!(buf, "  push a0")?;
            writeln!(buf, "  pop a1")?;
            writeln!(buf, "  pop a0")?;
            writeln!(buf, "  add a0, a1")?;
            writeln!(buf, "  push a0")?;
        }
        NodeKind::PtrSub(n) => {
            gen(buf, &n.left)?;
            gen(buf, &n.right)?;
            writeln!(
                buf,
                "  addi a0, zero, {}",
                node.ty.clone().unwrap().ptr_to.unwrap().size
            )?;
            writeln!(buf, "  push a0")?;
            writeln!(buf, "  pop a1")?;
            writeln!(buf, "  pop a0")?;
            writeln!(buf, "  mov t0, a0")?;
            writeln!(buf, "  subi a1, a1, 1")?;
            writeln!(buf, "  beq a1, zero, 6")?;
            writeln!(buf, "  add a0, t0")?;
            writeln!(buf, "  jal zero, -6")?;
            writeln!(buf, "  push a0")?;
            writeln!(buf, "  pop a1")?;
            writeln!(buf, "  pop a0")?;
            writeln!(buf, "  sub a0, a1")?;
            writeln!(buf, "  push a0")?;
        }

        NodeKind::Asm(n) => {
            writeln!(buf, "  {}", n.asm)?;
        }
        NodeKind::Null => unreachable!(),
    }
    Ok(())
}

fn gen_lval(buf: &mut String, node: &Node) -> Result<()> {
    match &node.kind {
        NodeKind::Var(n) => {
            if n.is_global {
                writeln!(buf, "  lil a0, {}@l", n.name)?;
                writeln!(buf, "  lih a1, {}@h", n.name)?;
                writeln!(buf, "  or a0, a1")?;
                writeln!(buf, "  push a0")?;
            } else {
                writeln!(buf, "  mov a0, fp")?;
                writeln!(buf, "  lil t0, 0x{:04x}@l", n.offset)?;
                writeln!(buf, "  lih t1, 0x{:04x}@h", n.offset)?;
                writeln!(buf, "  or t0, t1")?;
                writeln!(buf, "  sub a0, t0")?;
                writeln!(buf, "  push a0")?;
            }
        }
        NodeKind::Deref(n) => {
            gen(buf, &n.unary)?;
        }
        NodeKind::MemAccess(n) => {
            gen_lval(buf, &n.unary)?;
            writeln!(buf, "  lil t0, 0x{:04x}@l", n.member.offset)?;
            writeln!(buf, "  lih t1, 0x{:04x}@h", n.member.offset)?;
            writeln!(buf, "  or t0, t1")?;
            writeln!(buf, "  pop a0")?;
            writeln!(buf, "  add a0, t0")?;
            writeln!(buf, "  push a0")?;
        }
        _ => {
            return Err(anyhow!("Left side value is not local value"));
        }
    }
    Ok(())
}
