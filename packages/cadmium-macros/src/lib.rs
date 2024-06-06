use quote::quote;
use syn::{parse_macro_input, DeriveInput};

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
