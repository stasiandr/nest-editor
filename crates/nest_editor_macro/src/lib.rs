use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Block, Expr, ExprMethodCall, ItemFn, Stmt};

#[proc_macro_attribute]
pub fn app_builder(attr: TokenStream, item: TokenStream) -> TokenStream {
    let _args = attr;


    let input_fn = parse_macro_input!(item as ItemFn);

    let mut runtime_fn = input_fn.clone();
    replace_default_nest_plugins(&mut runtime_fn.block, false);

    let mut editor_fn = input_fn.clone();
    replace_default_nest_plugins(&mut editor_fn.block, true);

    let block = &editor_fn.block;

    let gen = quote! {

        use nest_editor_shared::*;

        #runtime_fn

        fn editor_app_builder() -> bevy::prelude::App {
            #block
        }

        #[no_mangle]
        pub extern "C" fn app_exporter() -> *mut ::bevy::prelude::App {
            let mut app = editor_app_builder();

            app.finish();
            app.cleanup();
            Box::into_raw(Box::new(app))
        }
    };

    gen.into()
}


fn replace_default_nest_plugins(block: &mut Block, is_editor: bool) {
    for stmt in &mut block.stmts {
        if let Stmt::Expr(Expr::MethodCall(ExprMethodCall { method, args, .. }), _) = stmt {
            if method == "add_plugins" {
                if let Some(Expr::Path(path)) = args.first() {
                    if path.path.segments.iter().any(|seg| seg.ident == "DefaultPlugins") {

                        if is_editor {
                            *stmt = syn::parse_quote! {
                                app.add_plugins(DefaultNestPlugins::editor());
                            };
                        }
                        else {
                            *stmt = syn::parse_quote! {
                                app.add_plugins(DefaultNestPlugins::default());
                            };
                        }
                    }
                }
            }
        }
    }
}