use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, DeriveInput, Ident, MetaNameValue, Token};
use syn::spanned::Spanned;

#[proc_macro_derive(StepDataActions, attributes(step_data))]
pub fn derive_step_data(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let data = match input.data {
        syn::Data::Enum(data) => data,
        _ => panic!("StepData can only be derived for enums"),
    };

    let variants = data.variants.iter().map(|variant| {
        let variant_name = &variant.ident;

        let mut skip_workbench = false;
        let mut workbench_field = None;
        let mut parent_type = None;

        for attr in &variant.attrs {
            if !attr.path.is_ident("step_data") {
                continue;
            }

            let name_values: Punctuated<MetaNameValue, Token![,]> = attr.parse_args_with(Punctuated::parse_terminated).unwrap(); // handle error instead of unwrap
            for nv in name_values {
                let ident = nv.path.get_ident().unwrap();

                match ident.to_string().as_str() {
                    "skip_workbench" => {
                        skip_workbench = true;
                    },
                    "workbench_field" => {
                        if let syn::Lit::Str(value) = nv.lit {
                            workbench_field = Some(value.value());
                        } else {
                            panic!("workbench_field must be a string literal");
                        }
                    },
                    "type" => {
                        if let syn::Lit::Str(value) = nv.lit {
                            parent_type = Some(value.value());
                        } else {
                            panic!("type must be a string literal");
                        }
                    },
                    &_ => {}
                }
            }
        }

        // Process not skipped workbench
        let mut wb_var = quote! {};
        if !skip_workbench {
            wb_var = quote! {
                let wb_ = self.native.workbenches
                    .get_mut(workbench_id as usize)
                    .ok_or(anyhow::anyhow!("Could not find workbench"))?;
            };
        }

        // Process type and workbench_field
        let mut field_var = quote! {};
        let id_arg_name = if let Some(f) = parent_type.clone() {
            Ident::new(format!("{}_id", f.to_string().to_case(Case::Snake)).as_str(), f.span())
        } else {
            Ident::new("id", variant_name.span())
        };
        if let Some(field_ident) = workbench_field.clone() {
            let field_name = Ident::new(field_ident.as_str(), field_ident.span());
            field_var = quote! {
                let parent_ref_ = wb_.#field_name
                    .get(& #id_arg_name)
                    .ok_or(anyhow::anyhow!("Could not find parent"))?;
                let parent_ = parent_ref_.borrow_mut();
            };
        } else if !skip_workbench {
            field_var = quote! { let parent_ = wb_; };
        } else {
            field_var = quote! { let parent_ = self; };
        }

        // Function declaration
        let add_func_name = Ident::new(format!("add_{}", variant_name.to_string().to_case(Case::Snake)).as_str(), variant_name.span());
        let function_defs = variant.fields.iter().map(|field| {
            let field_name = &field.ident;
            let field_type = &field.ty;

            quote! { #field_name: #field_type }
        }).collect::<Vec<_>>();
        let function_args_full = variant.fields.iter().map(|field| {
            let field_name = &field.ident;

            quote! { #field_name }
        }).collect::<Vec<_>>();

        let function_args2 = function_args_full.clone();
        let function_args_noauto = function_args2
            .iter()
            .filter(|field|
                field.to_string() != "workbench_id"
                && field.to_string() != id_arg_name.to_string()
            ).collect::<Vec<_>>();


        quote! {
            pub fn #add_func_name(&mut self, name: String, #( #function_defs ),*) -> Result<crate::IDType, anyhow::Error> {
                #wb_var
                #field_var
                let result_id_ = parent_.#add_func_name(#( #function_args_noauto ),*)?;

                let step_ = Step {
                    name,
                    id: result_id_,
                    operation: StepOperation::Add,
                    unique_id: format!(concat!("Add:", stringify!(#name), "-{}"), result_id_),
                    suppressed: false,
                    data: #name::#variant_name {
                        #( #function_args_full ),*
                    },
                };

                wb_.history.push(step_);

                Ok(result_id_)
            }
        }
    });

    let expanded = quote! {
        impl crate::Project {
            #( #variants )*
        }
    };

    TokenStream::from(expanded)
}
