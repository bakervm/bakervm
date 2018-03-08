pub struct Ast {
    modules: Vec<Module>,
}

struct Module {
    name: String,
    funcs: Vec<Func>,
    imports: Vec<Import>,
    exports: Vec<Export>,
}

struct Func {
    instr: Vec<Instr>,
}

struct Import {
    func: String,
    as: Option<String>,
    module: String
}

struct Export {
    func_id: usize,
    as: String,
}
