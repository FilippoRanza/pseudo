pub struct Code {
    pub caption: String,
    pub code: Vec<Command>,
    pub decl: DeclBlock,
}

pub enum Command {
    Assign(Assign),
    Condition(Condition),
    ForLoop(ForLoop),
    WhileLoop(ConditionPair),
    Return(CodeType),
    Function(Function)
}

pub enum CodeType {
    Name(String),
    Func(Function)
}

pub struct Function {
    pub name: String,
    pub args: Vec<String>
}

pub type DeclBlock = Vec<Decl>;
pub type Decl = (String, String);

pub type Assign = (String, CodeType);

pub struct Condition {
    pub if_block: ConditionPair,
    pub elif_blocks: Vec<ConditionPair>,
    pub else_block: Option<Vec<Command>>,
}

pub struct ConditionPair {
    pub cond: CodeType,
    pub body: Vec<Command>,
}

pub struct ForLoop {
    pub kind: ForLoopKind,
    pub body: Vec<Command>,
}

pub enum ForLoopKind {
    Count((String, CodeType, CodeType)),
    Iter((String, CodeType)),
}
