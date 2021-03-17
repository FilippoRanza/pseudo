


pub struct Code {
    pub caption: String,
    pub code: Vec<Command>
}

pub enum Command {
    Declaration(DeclBlock),
    Assign(Assign),
    Condition(Condition),
    ForLoop(ForLoop),
    WhileLoop(ConditionPair),
    Return(String)
}

pub type DeclBlock = Vec<Decl>;
pub type Decl = (String, String);

pub type Assign = (String, String);

pub struct Condition {
    pub if_block : ConditionPair,
    pub elif_blocks: Vec<ConditionPair>,
    pub else_block: Option<Vec<Command>>
}


pub struct ConditionPair {
    pub cond: String, 
    pub body: Vec<Command>
}

pub struct ForLoop {
    pub kind: ForLoopKind,
    pub body: Vec<Command>
}

pub enum ForLoopKind {
    Count((String, String, String)),
    Iter((String, String))
}


