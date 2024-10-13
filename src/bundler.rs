use full_moon::tokenizer::{Token, TokenType};
use full_moon::visitors::VisitorMut;
use full_moon::{
    ast::{
        punctuated::{Pair, Punctuated},
        span::ContainedSpan,
        Assignment, Block, Call, Do, FunctionArgs, FunctionBody, FunctionCall, FunctionName,
        GenericFor, If, LastStmt, LocalAssignment, LocalFunction, MethodCall, NumericFor, Return,
    },
    tokenizer::TokenReference,
};

#[derive(Clone)]
pub struct BundledFile {
    pub file_path: String,
    pub ast: full_moon::ast::Ast,
}

impl BundledFile {
    pub fn prefix(&self) -> String {
        let file_path = self.file_path.clone();
        let file_path = file_path.replace("/", "__");

        // Split by . and remove last part of it
        let file_split = file_path.split(".").collect::<Vec<&str>>();
        let file_path = file_split[..file_split.len() - 1].join("__");

        file_path
    }
}

#[derive(Clone)]
pub struct BundleList {
    pub files: Vec<BundledFile>,
}

pub struct SymbolMangleVisitor {
    /// File prefix to use for symbol mangling
    prefix: String,
    /// Current depth of the visitor
    depth: usize,
}

impl SymbolMangleVisitor {
    fn incr_depth(&mut self) {
        //println!("depth: {}->{}", self.depth, self.depth + 1);
        self.depth += 1;
    }

    fn decr_depth(&mut self) {
        //println!("depth: {}->{}", self.depth, self.depth - 1);
        self.depth -= 1;
    }
}

impl VisitorMut for SymbolMangleVisitor {
    /// Record the depth of the visitor at function call start
    fn visit_function_call(&mut self, node: FunctionCall) -> FunctionCall {
        self.incr_depth();
        node
    }

    /// Record the depth of the visitor at function call end
    fn visit_function_call_end(&mut self, node: FunctionCall) -> FunctionCall {
        self.decr_depth();
        node
    }

    /// Record the depth of the visitor at anonymous function call
    fn visit_anonymous_call(&mut self, node: FunctionArgs) -> FunctionArgs {
        self.incr_depth();
        node
    }

    fn visit_anonymous_call_end(&mut self, node: FunctionArgs) -> FunctionArgs {
        self.decr_depth();
        node
    }

    fn visit_assignment(&mut self, node: Assignment) -> Assignment {
        if self.depth == 1 {
            panic!("Bundling safety error: Assignment at root level: {}", node);
        }
        self.incr_depth();
        node
    }

    fn visit_assignment_end(&mut self, node: Assignment) -> Assignment {
        self.decr_depth();
        node
    }

    fn visit_block(&mut self, node: Block) -> Block {
        node // We don't need to do anything with blocks
    }

    fn visit_block_end(&mut self, node: Block) -> Block {
        node // We don't need to do anything with blocks
    }

    fn visit_call(&mut self, node: Call) -> Call {
        node // TODO
    }

    fn visit_call_end(&mut self, node: Call) -> Call {
        node // TODO
    }

    fn visit_contained_span(&mut self, node: ContainedSpan) -> ContainedSpan {
        node // TODO
    }

    fn visit_contained_span_end(&mut self, node: ContainedSpan) -> ContainedSpan {
        node // TODO
    }

    fn visit_do(&mut self, node: Do) -> Do {
        if self.depth == 0 {
            panic!("Bundling safety error: Do at root level: {}\n\nDo end may have side-effects and are as such not allowed at root level", node);
        }

        self.incr_depth();
        node
    }

    fn visit_do_end(&mut self, node: Do) -> Do {
        self.decr_depth();
        node
    }

    fn visit_generic_for(&mut self, node: GenericFor) -> GenericFor {
        if self.depth == 0 {
            panic!("Bundling safety error: GenericFor at root level: {}\n\nGenericFor may have side-effects and are as such not allowed at root level", node);
        }

        self.incr_depth();
        node
    }

    fn visit_generic_for_end(&mut self, node: GenericFor) -> GenericFor {
        self.decr_depth();
        node
    }

    fn visit_if(&mut self, node: If) -> If {
        if self.depth == 0 {
            panic!("Bundling safety error: If at root level: {}\n\nIf may have side-effects and are as such not allowed at root level", node);
        }

        self.incr_depth();
        node
    }

    fn visit_if_end(&mut self, node: If) -> If {
        self.decr_depth();
        node
    }

    fn visit_local_assignment(&mut self, node: LocalAssignment) -> LocalAssignment {
        if self.depth == 0 {
            panic!("Bundling safety error: LocalAssignment at root level: {}\n\nLocalAssignment at root level is not yet supported", node);
        }
        self.incr_depth();
        node
    }

    fn visit_local_assignment_end(&mut self, node: LocalAssignment) -> LocalAssignment {
        self.decr_depth();
        node
    }

    fn visit_local_function(&mut self, node: LocalFunction) -> LocalFunction {
        if self.depth == 0 {
            panic!("Bundling safety error: LocalFunction at root level: {}\n\nLocalFunction at root level is not yet supported", node);
        }
        self.incr_depth();
        node
    }

    fn visit_local_function_end(&mut self, node: LocalFunction) -> LocalFunction {
        self.decr_depth();
        node
    }

    fn visit_last_stmt(&mut self, node: LastStmt) -> LastStmt {
        node // We don't need to do anything with last statements
    }

    fn visit_last_stmt_end(&mut self, node: LastStmt) -> LastStmt {
        node // We don't need to do anything with last statements
    }

    fn visit_method_call(&mut self, node: MethodCall) -> MethodCall {
        if self.depth == 0 {
            panic!("Bundling safety error: MethodCall at root level: {}\n\nMethodCall at root level is not supported and will likely never be supported!", node);
        }
        self.incr_depth();
        node // TODO
    }

    fn visit_method_call_end(&mut self, node: MethodCall) -> MethodCall {
        self.decr_depth();
        node // TODO
    }

    fn visit_numeric_for(&mut self, node: NumericFor) -> NumericFor {
        if self.depth == 0 {
            panic!("Bundling safety error: NumericFor at root level: {}\n\nNumericFor at root level is not supported and will likely never be supported!", node);
        }
        self.incr_depth();
        node
    }

    fn visit_numeric_for_end(&mut self, node: NumericFor) -> NumericFor {
        self.decr_depth();
        node
    }

    fn visit_return(&mut self, node: Return) -> Return {
        if self.depth == 0 {
            panic!("Bundling safety error: Return at root level: {}\n\nReturn at root level is not supported and will likely never be supported", node);
        }
        node
    }

    fn visit_return_end(&mut self, node: Return) -> Return {
        node
    }

    /// This converts functions at root level to methods on the table
    ///
    /// E.g. `function foo()` -> function ``example__file.foo()``    
    fn visit_function_name(&mut self, node: FunctionName) -> FunctionName {
        let is_nested = self.depth >= 1;

        self.incr_depth();

        if is_nested {
            return node; // Don't mangle nested functions
        }

        println!(
            "SymbolMangleVisitor: Adding function `{}` to bundle",
            node.to_string(),
        );

        let tok_names = node.names();

        let first_token = tok_names.last().unwrap();
        let mut first_token = first_token.clone();
        let mut ft_identifier = None;
        first_token = first_token.map(|f| {
            match f.token().token_type() {
                TokenType::Identifier { identifier } => {
                    // Get new function name
                    ft_identifier = Some(identifier.clone());
                    let new_func_name = self.prefix.clone();

                    // Create new token
                    let new_token_type = TokenType::Identifier {
                        identifier: new_func_name.into(),
                    };

                    f.with_token(Token::new(new_token_type))
                }
                _ => {
                    panic!(
                        "Unsupported token type for function name: {:?}",
                        f.token().token_type()
                    );
                }
            }
        });

        let mut new_names = Punctuated::new();

        new_names.push(first_token);

        if let Some(ft_identifier) = ft_identifier {
            let token = Token::new(TokenType::Identifier {
                identifier: ft_identifier,
            });

            let tok_ref = TokenReference::new(
                vec![Token::new(TokenType::Symbol {
                    symbol: full_moon::tokenizer::Symbol::Dot,
                })],
                token,
                vec![], // No trailing tokens
            );

            new_names.push(Pair::new(tok_ref, None));
        }

        for (i, tok) in tok_names.pairs().enumerate() {
            if i == 0 {
                continue; // We already processed the first token
            }

            new_names.push(tok.clone());
        }

        FunctionName::new(new_names)
    }

    fn visit_function_name_end(&mut self, node: FunctionName) -> FunctionName {
        self.decr_depth(); // Note: func body enter exit will be handled by visit_function_body/end
        node // Note: visit_function_body_end handles decr here
    }

    fn visit_function_body(&mut self, node: FunctionBody) -> FunctionBody {
        self.incr_depth();
        node
    }

    fn visit_function_body_end(&mut self, node: FunctionBody) -> FunctionBody {
        self.decr_depth();
        node
    }
}

pub fn apply_symbol_mangling(bundles: BundleList) -> BundleList {
    let files = bundles
        .files
        .into_iter()
        .map(|file| {
            let prefix = file.prefix();
            let ast = file.ast;

            let mut visitor = SymbolMangleVisitor { prefix, depth: 0 };

            let new_ast = visitor.visit_ast(ast);

            // If the depth is not 0, we have a problem [a bug]
            assert_eq!(visitor.depth, 0);

            BundledFile {
                ast: new_ast.update_positions(),
                ..file
            }
        })
        .collect::<Vec<BundledFile>>();

    BundleList { files }
}

pub struct ImportInlineVisitor {
    /// Known imports
    known_imports: Vec<String>,
    /// What imports to ignore
    ignore_imports: Vec<String>,
}

impl VisitorMut for ImportInlineVisitor {
    fn visit_function_call(&mut self, node: FunctionCall) -> FunctionCall {
        if node.prefix().to_string() != "require" {
            return node;
        }

        let mut suffixes = 0;
        for suffix in node.suffixes() {
            println!("FunctionCall: require_suffixes={}", suffix);
            suffixes += 1;
        }

        if suffixes != 1 {
            panic!("require() must have exactly one argument");
        }

        node
    }
}

pub fn apply_import_inling(bundles: BundleList) -> BundleList {
    let mut visitor = ImportInlineVisitor {
        known_imports: vec![],
        ignore_imports: vec!["@antiraid".to_string()],
    };

    let files = bundles
        .files
        .into_iter()
        .map(|file| {
            let _prefix = file.prefix();
            let ast = file.ast;

            let new_ast = visitor.visit_ast(ast);

            BundledFile {
                ast: new_ast.update_positions(),
                ..file
            }
        })
        .collect::<Vec<BundledFile>>();

    BundleList { files }
}
