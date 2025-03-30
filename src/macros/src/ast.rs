use swc_common::{FileName, SourceMap, sync::Lrc};
use swc_ecma_ast::Module;
use swc_ecma_parser::{Parser, StringInput, Syntax, error::Error, lexer::Lexer};

pub fn get_ast_for_dts(source: &str) -> Result<Module, Error> {
    let cm: Lrc<SourceMap> = Default::default(); //the fuck is this?

    //appearently you cant just parse a string, you need a whole ass sourcefile struct
    let source_file =
        cm.new_source_file(FileName::Custom(Default::default()).into(), source.into());

    let lexer = Lexer::new(
        // We want to parse ecmascript
        Syntax::Typescript(Default::default()),
        // EsVersion defaults to es5
        Default::default(),
        StringInput::from(&*source_file),
        None,
    );
    let mut parser = Parser::new_from(lexer);

    parser.parse_typescript_module()
}