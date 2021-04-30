use crate::ast;
use crate::string_builder::StringBuilder;
use sailfish::TemplateOnce;

pub fn generate(code: ast::Code, label: Option<String>, indent_ch: char, here: bool) -> String {
    let builder = translate_declaration(StringBuilder::new(2), code.decl);
    let builder = translate_commands(builder, code.code);

    LatexAlgorithm::generate_code(builder, code.caption, label, indent_ch, here)
}

#[derive(TemplateOnce)]
#[template(path = "latex-algo.stpl", escape = false)]
struct LatexAlgorithm {
    code: String,
    caption: String,
    label: Option<String>,
    indent: char,
    here: bool
}
impl LatexAlgorithm {
    fn generate_code(
        code: StringBuilder,
        caption: String,
        label: Option<String>,
        indent: char,
        here: bool
    ) -> String {
        let code = code.build_string(indent);
        let tmp = Self {
            code,
            caption,
            label,
            indent,
            here
        };
        tmp.render_once().unwrap()
    }
}

fn translate_commands(builder: StringBuilder, cmds: Vec<ast::Command>) -> StringBuilder {
    let builder = cmds.into_iter().fold(builder, translate_command);

    builder
}

fn translate_command(builder: StringBuilder, cmd: ast::Command) -> StringBuilder {
    match cmd {
        ast::Command::Assign(assign) => translate_assign(builder, assign),
        ast::Command::Condition(cond) => translate_condition(builder, cond),
        ast::Command::ForLoop(for_loop) => translate_for_loop(builder, for_loop),
        ast::Command::Return(return_blk) => translate_return(builder, return_blk),
        ast::Command::WhileLoop(while_loop) => translate_while_loop(builder, while_loop),
        ast::Command::Function(func) => translate_function_block(builder, func)
    }
}

fn translate_function_block(builder: StringBuilder, func: ast::Function) -> StringBuilder {
    let call = translate_function_call(func);
    builder.add_line(format!(r"\STATE {};", call))
}

fn translate_assign(builder: StringBuilder, assign: ast::Assign) -> StringBuilder {
    let rhs = translate_code_type(assign.1);
    let line = format!(r"\STATE {} $\leftarrow$ {};", assign.0, rhs);
    builder.add_line(line)
}

fn translate_condition(builder: StringBuilder, cond: ast::Condition) -> StringBuilder {
    let builder = translate_if_block(builder, cond.if_block);
    let builder = cond.elif_blocks.into_iter().fold(builder, translate_elif_block);

    let builder = if let Some(else_block) = cond.else_block {
        translate_else_block(builder, else_block)
    } else {
        builder
    };
    let if_end = String::from(r"\ENDIF");
    let builder = builder.add_line(if_end);

    builder
}

fn translate_if_block(builder: StringBuilder, cond: ast::ConditionPair) -> StringBuilder {
    translate_cond_block(builder, cond, "IF")
}

fn translate_elif_block(builder: StringBuilder, cond: ast::ConditionPair) -> StringBuilder {
    translate_cond_block(builder, cond, "ELSIF")
}

fn translate_else_block(builder: StringBuilder, block: Vec<ast::Command>) -> StringBuilder {
    translate_heading_block(builder, String::from(r"\ELSE"), block)
}

fn translate_cond_block(
    builder: StringBuilder,
    cond: ast::ConditionPair,
    name: &str,
) -> StringBuilder {
    let cond_txt = translate_code_type(cond.cond);
    let cond_line = format!(r"\{}{{{}}}", name, cond_txt);
    translate_heading_block(builder, cond_line, cond.body)
}

fn translate_declaration(builder: StringBuilder, decl: ast::DeclBlock) -> StringBuilder {
    if decl.len() > 0 {
        builder.add_line(generate_declaration(decl))
    } else {
        builder
    }
}

fn generate_declaration(decl: ast::DeclBlock) -> String {
    let str_vec: Vec<String> = decl
        .iter()
        .map(|(name, prop)| format!("{}: {}", name, prop))
        .collect();
    let decl_str = str_vec.join(", ");
    format!(r"\REQUIRE {}", decl_str)
}

fn translate_for_loop(builder: StringBuilder, fl: ast::ForLoop) -> StringBuilder {
    let heading = translate_for_heading(fl.kind);
    translate_heading_block(builder, heading, fl.body).add_line(String::from(r"\ENDFOR"))
}

fn translate_for_heading(kind: ast::ForLoopKind) -> String {
    match kind {
        ast::ForLoopKind::Count((var, begin, end)) => {
            let begin = translate_code_type(begin);
            let end = translate_code_type(end);
            format!(r"\FOR{{{} =  {} \textbf{{to}} {}}}", var, begin, end)
        }
        ast::ForLoopKind::Iter((var, iter)) => format!(r"\FOR{{{} \textbf{{in}}  {}}}", var, translate_code_type(iter)),
    }
}

fn translate_return(builder: StringBuilder, rb: ast::CodeType) -> StringBuilder {
    let line = format!(r"\RETURN {};", translate_code_type(rb));
    builder.add_line(line)
}

fn translate_while_loop(builder: StringBuilder, wl: ast::ConditionPair) -> StringBuilder {
    let code = translate_code_type(wl.cond);
    let cond_line = format!(r"\WHILE{{ {} }}", code);
    let builder = translate_heading_block(builder, cond_line, wl.body);
    builder.add_line(String::from(r"\ENDWHILE"))
}

fn translate_heading_block(
    builder: StringBuilder,
    heading: String,
    block: Vec<ast::Command>,
) -> StringBuilder {
    let builder = builder.add_line(heading);
    let builder = builder.increase_indent();
    let builder = translate_commands(builder, block);
    builder.decrease_indent()
}

fn translate_code_type(ct: ast::CodeType) -> String {
    match ct {
        ast::CodeType::Func(f) => translate_function_call(f),
        ast::CodeType::Name(n) => n
    }
}

fn translate_function_call(func: ast::Function) -> String {
    let args = func.args.join(", ");
    format!("{}({})", func.name, args)   
}


