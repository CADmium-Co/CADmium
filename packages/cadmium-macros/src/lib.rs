use std::collections::HashMap;

use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::{Attribute, DeriveInput, Ident, MetaNameValue, Token, parse_macro_input};
use syn::spanned::Spanned;

const ATTR_NAME: &str = "step_data";

fn get_meta_kv(attrs: &Vec<Attribute>) -> HashMap<Ident, MetaNameValue> {
    let mut result = HashMap::new();

    for attr in attrs {
        if !attr.path.is_ident(ATTR_NAME) {
            continue;
        }

        let Ok(name_values): Result<Punctuated<MetaNameValue, Token![,]>, _> = attr
            .parse_args_with(Punctuated::parse_terminated) else { continue; };

        for nv in name_values {
            let Some(ident) = nv.path.get_ident() else { continue; };
            result.insert(ident.clone(), nv);
        }
    }

    result
}

#[proc_macro_derive(StepDataActions, attributes(step_data))]
pub fn derive_step_data(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let data = match input.data {
        syn::Data::Enum(data) => data,
        _ => panic!("StepData can only be derived for enums"),
    };
    let mut actions = vec![];

    let variants = data.variants.iter().map(|variant| {
        let variant_name = &variant.ident;

        let mut workbench_field = None;
        let mut parent_type = None;
        let mut skip_history = false;

        // Parse the attributes above each variant
        for (k, v) in get_meta_kv(&variant.attrs).iter() {
            match k.to_string().as_str() {
                "workbench_field" => {
                    if let syn::Lit::Str(value) = &v.lit {
                        workbench_field = Some(value.value());
                    } else {
                        panic!("workbench_field must be a string literal");
                    }
                },
                "type_name" => {
                    if let syn::Lit::Str(value) = &v.lit {
                        parent_type = Some(value.value());
                    } else {
                        panic!("type_name must be a string literal");
                    }
                },
                "skip_history" => {
                    if let syn::Lit::Bool(_value) = &v.lit {
                        skip_history = true;
                    } else {
                        panic!("skip_history must be a bool literal");
                    }
                },
                &_ => {}
            }
        }

        let needs_workbench = variant.fields.iter().any(|field| field.ident.as_ref().unwrap().to_string() == "workbench_id");

        // Process not skipped workbench
        let mut wb_var = quote! {};
        if needs_workbench {
            wb_var = quote! {
                let mut wb_ = self.workbenches
                    .get_mut(workbench_id as usize)
                    .ok_or(anyhow::anyhow!("Could not find workbench"))?;
            };
        }

        // Process type_name to expected id field (e.g. sketch_id for Sketch)
        let mut _field_var = quote! {};
        let mut _parent_var_ = quote! { wb_ };
        let id_arg_name = if let Some(f) = parent_type.clone() {
            Ident::new(format!("{}_id", f.to_string().to_case(Case::Snake)).as_str(), f.span())
        } else {
            Ident::new("id", variant_name.span())
        };

        // Generate the parent variable of which the actual function will be called on
        if let Some(field_ident) = workbench_field.clone() {
            let field_name = Ident::new(field_ident.as_str(), field_ident.span());
            _field_var = quote! {
                let parent_ref_ = wb_.#field_name
                    .get(& #id_arg_name)
                    .ok_or(anyhow::anyhow!(concat!("Could not find parent ", stringify!(#parent_type), " with ID {}"), #id_arg_name))?;
                let mut parent_ = parent_ref_.borrow_mut();
            };
            _parent_var_ = quote! { parent_ };
        } else if needs_workbench {
            _parent_var_ = quote! { wb_ };
        } else {
            _parent_var_ = quote! { self };
        }

        // Generated function names
        let add_func_name = Ident::new(format!("add_{}", variant_name.to_string().to_case(Case::Snake)).as_str(), variant_name.span());

        // Function arguments - both on definition and call
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

        // Generate history entry
        let history_entry = if skip_history {
            quote! {}
        } else {
            quote! {
                let step_ = crate::step::Step {
                    name,
                    id: result_id_,
                    operation: crate::step::StepOperation::Add,
                    unique_id: format!(concat!("Add:", stringify!(#variant_name), "-{}"), result_id_),
                    suppressed: false,
                    data: #name::#variant_name {
                        #( #function_args_full ),*
                    },
                };

                wb_.history.push(step_);
            }
        };

        // Populate the `do_action` function of StepData
        actions.push(quote! {
            #name::#variant_name {
                #( #function_args_full ),*
            } => project.#add_func_name(name, #( #function_args_full.clone() ),* ) , });

        quote! {
            pub fn #add_func_name(&mut self, name: String, #( #function_defs ),*) -> Result<crate::IDType, anyhow::Error> {
                #wb_var
                #_field_var
                let result_id_ = #_parent_var_.#add_func_name(#( #function_args_noauto.clone() ),*)?;

                #history_entry

                Ok(result_id_)
            }
        }
    });

    let expanded = quote! {
        impl crate::project::Project {
            #( #variants )*
        }

        impl #name {
            pub fn do_action(&self, project: &mut crate::project::Project, name: String) -> Result<IDType, anyhow::Error> {
                match self {
                    #( #actions )*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
