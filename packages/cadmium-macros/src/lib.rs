use convert_case::{Case, Casing};
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput, Ident, ItemFn, Meta, NestedMeta};

#[proc_macro_derive(MessageEnum)]
pub fn message_handler_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let data = match input.data {
        syn::Data::Enum(data) => data,
        _ => panic!("MessageEnum can only be derived for enums"),
    };

    let variants_type = data.variants.iter().map(|variant| {
        let syn::Fields::Unnamed(field) = &variant.fields else {
            panic!("MessageEnum can only be derived for enums with unnamed fields");
        };

        let field_type = &field.unnamed[0].ty;
        quote! { #field_type }
    }).collect::<Vec<_>>();

    let variants = data.variants.iter().map(|variant| {
        println!("Message Handler: {}", variant.ident);
        let variant_name = &variant.ident;

        quote! {
            #name::#variant_name(msg)
        }
    }).collect::<Vec<_>>();

    let variant_names = data.variants.iter().map(|variant| &variant.ident).collect::<Vec<_>>();
    let variants_clone = variants.clone();
    let variants_clone2 = variants.clone();

    quote! {
        impl #name {
            pub fn handle(&self, project: &mut crate::project::Project) -> anyhow::Result<Option<crate::IDType>> {
                match self {
                    #( #variants_clone => msg.handle_project_message(project), )*
                }
            }
        }

        #(
            impl From<#variants_type> for #name {
                fn from(msg: #variants_type) -> Self {
                    #variants_clone2
                }
            }
        )*

        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    #( #variants => write!(f, stringify!(#variant_names)), )*
                }
            }
        }
    }.into()
}

#[proc_macro_attribute]
pub fn message(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let args = parse_macro_input!(attr as syn::AttributeArgs);
    let input = parse_macro_input!(item as ItemFn);

    // Extract the function name and arguments
    let fn_name = &input.sig.ident;
    let fn_args = &input.sig.inputs;
    let mut parent_opt = None;
    let mut rename_parent = None;

    for arg in args.iter() {
        match arg {
            NestedMeta::Meta(Meta::Path(path)) => {
                parent_opt = Some(path.get_ident().expect("Parent type mut be an identifier (e.g. ISketch, not crate::ISketch)"));
            }
            NestedMeta::Meta(Meta::NameValue(name_value)) => {
                if name_value.path.is_ident("rename_parent") {
                    let syn::Lit::Str(ref rename_parent_val) = name_value.lit else {
                        panic!("rename_parent must be a string literal")
                    };
                    rename_parent = Some(rename_parent_val.value());
                }
            }
            _ => panic!("Invalid attribute argument")
        }
    }

    // Create a struct name based on the function name
    let parent = parent_opt.expect("Parent type must be specified");
    let struct_name = if let Some(rename_parent) = rename_parent {
        format_ident!("{}{}Message", rename_parent, fn_name.to_string().to_case(Case::Pascal))
    } else {
        format_ident!("{}{}Message", parent, fn_name.to_string().to_case(Case::Pascal))
    };
    let fn_message_name = Ident::new(struct_name.to_string().to_case(Case::Snake).as_str(), struct_name.span());

    // Generate struct fields from function arguments
    let fields = fn_args.iter().map(|arg| {
        if let syn::FnArg::Typed(pat_type) = arg {
            let pat = &pat_type.pat;
            let ty = &pat_type.ty;
            quote! {
                pub #pat: #ty,
            }
        } else {
            quote!()
        }
    });
    let parameters = fn_args.iter().filter_map(|arg| {
        if let syn::FnArg::Typed(pat_type) = arg {
            let pat = &pat_type.pat;
            Some(quote! { #pat })
        } else {
            None
        }
    });

    quote! {
        impl #parent {
            #input
        }

        #[derive(tsify_next::Tsify, Debug, Clone, serde::Serialize, serde::Deserialize)]
        #[tsify(from_wasm_abi, into_wasm_abi)]
        pub struct #struct_name {
            #(#fields)*
        }

        impl crate::message::MessageHandler for #struct_name {
            type Parent = Rc<RefCell<#parent>>;
            fn handle_message(&self, parent_ref: Self::Parent) -> anyhow::Result<Option<IDType>> {
                parent_ref.borrow_mut().#fn_name( #(self.#parameters.clone()),* )
            }
        }
    }.into()
}
