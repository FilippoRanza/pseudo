use crate::ast;
use crate::string_builder::StringBuilder;

pub fn generate(code: &ast::Code, indent_ch: char) -> String {
    let builder = translate_declaration(StringBuilder::new(2), &code.decl);
    let builder = translate_commands(builder, &code.code);
    let algo_code = builder.build_string(indent_ch);
    format!("\\begin{{algorithm}}\n{}\\centering\n{0}\\caption{{{}}}\n{0}\\begin{{algorithmic}}\n{}{0}\\end{{algorithmic}}\n\\end{{algorithm}}",
     indent_ch, code.caption, algo_code)
}

fn translate_commands(builder: StringBuilder, cmds: &[ast::Command]) -> StringBuilder {
    let builder = cmds.iter().fold(builder, translate_command);

    builder
}

fn translate_command(builder: StringBuilder, cmd: &ast::Command) -> StringBuilder {
    match cmd {
        ast::Command::Assign(assign) => translate_assign(builder, assign),
        ast::Command::Condition(cond) => translate_condition(builder, cond),
        ast::Command::ForLoop(for_loop) => translate_for_loop(builder, for_loop),
        ast::Command::Return(return_blk) => translate_return(builder, return_blk),
        ast::Command::WhileLoop(while_loop) => translate_while_loop(builder, while_loop),
    }
}

fn translate_assign(builder: StringBuilder, assign: &ast::Assign) -> StringBuilder {
    let line = format!(r"\STATE {} = {};", assign.0, assign.1);
    builder.add_line(line)
}

fn translate_condition(builder: StringBuilder, cond: &ast::Condition) -> StringBuilder {
    let builder = translate_if_block(builder, &cond.if_block);
    let builder = cond.elif_blocks.iter().fold(builder, translate_elif_block);

    let builder = if let Some(else_block) = &cond.else_block {
        translate_else_block(builder, else_block)
    } else {
        builder
    };
    let if_end = String::from(r"\ENDIF");
    let builder = builder.add_line(if_end);

    builder
}

fn translate_if_block(builder: StringBuilder, cond: &ast::ConditionPair) -> StringBuilder {
    translate_cond_block(builder, cond, "IF")
}

fn translate_elif_block(builder: StringBuilder, cond: &ast::ConditionPair) -> StringBuilder {
    translate_cond_block(builder, cond, "ELSIF")
}

fn translate_else_block(builder: StringBuilder, block: &[ast::Command]) -> StringBuilder {
    translate_heading_block(builder, String::from(r"\ELSE"), block)
}

fn translate_cond_block(
    builder: StringBuilder,
    cond: &ast::ConditionPair,
    name: &str,
) -> StringBuilder {
    let cond_line = format!(r"\{}{{{}}}", name, cond.cond);
    translate_heading_block(builder, cond_line, &cond.body)
}

fn translate_declaration(builder: StringBuilder, decl: &ast::DeclBlock) -> StringBuilder {
    if decl.len() > 0 {
        builder.add_line(generate_declaration(decl))
    } else {
        builder
    }
}

fn generate_declaration(decl: &ast::DeclBlock) -> String {
    let str_vec: Vec<String> = decl
        .iter()
        .map(|(name, prop)| format!("{}: {}", name, prop))
        .collect();
    let decl_str = str_vec.join(", ");
    format!(r"\REQUIRE {}", decl_str)
}

fn translate_for_loop(builder: StringBuilder, fl: &ast::ForLoop) -> StringBuilder {
    let heading = translate_for_heading(&fl.kind);
    translate_heading_block(builder, heading, &fl.body).add_line(String::from(r"\ENDFOR"))
}

fn translate_for_heading(kind: &ast::ForLoopKind) -> String {
    match kind {
        ast::ForLoopKind::Count((var, begin, end)) => {
            format!(r"\FOR{{{} =  {}:{}}}", var, begin, end)
        }
        ast::ForLoopKind::Iter((var, iter)) => format!(r"\FOR{{{} \textbf{{in}}  {}}}", var, iter),
    }
}

fn translate_return(builder: StringBuilder, rb: &str) -> StringBuilder {
    let line = format!(r"\RETURN {};", rb);
    builder.add_line(line)
}

fn translate_while_loop(builder: StringBuilder, wl: &ast::ConditionPair) -> StringBuilder {
    let cond_line = format!(r"\WHILE{{ {} }}", wl.cond);
    let builder = translate_heading_block(builder, cond_line, &wl.body);
    builder.add_line(String::from(r"\ENDWHILE"))
}

fn translate_heading_block(
    builder: StringBuilder,
    heading: String,
    block: &[ast::Command],
) -> StringBuilder {
    let builder = builder.add_line(heading);
    let builder = builder.increase_indent();
    let builder = translate_commands(builder, block);
    builder.decrease_indent()
}
