extern crate proc_macro;
use quote;
use syn;

#[proc_macro_attribute]
pub fn config_file(args: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemStruct);
    let name = &input.ident;

    let args = syn::parse_macro_input!(args as syn::AttributeArgs);

    let file_path = match &args[0] {
        syn::NestedMeta::Meta(meta) => {
            match meta {
                syn::Meta::NameValue(attr) => {
                    if !attr.path.is_ident("file_path") {
                        panic!("Not supported arg name. Required file_path = \"*path*\"")
                    };

                    match &attr.lit {
                        syn::Lit::Str(str_lit) => {str_lit.value()},
                        _ => panic!("Not supported arg type. Required file_path = \"*path*\"")
                    }
                },
                _ => panic!("Not supported arg type. Required file_path = \"*path*\"")
            }
        },
        _ => panic!("Not supported arg type. Required file_path = \"*path*\"")
    };


    let result = quote::quote! {
        #[derive(serde::Serialize, serde::Deserialize, Debug)]
        #input

        impl #name {
            pub async fn load() -> Result<Self, String> {
                use serde::de::Deserialize;
                log::info!("Config [{}] loading...", #file_path);
                let data = tokio::fs::read(#file_path).await;
                //let data = data.map_err(|e:std::io::Error| e.to_string())?;
                let data = match data {
                    Result::Ok(result) => result,
                    Result::Err(err) => return Result::Err(err.to_string()),
                };
                let data = String::from_utf8(data).map_err(|e| e.to_string() )?;
                let obj = serde_json::from_str(&*data).map_err(|e| e.to_string())?;
                log::info!("Config [{}] loaded", #file_path);
                Result::Ok(obj)
            }

            
            pub async fn load_from(file_path: &str) -> Result<Self, String> {
                use serde::de::Deserialize;
                log::info!("Config [{}] loading...", file_path);
                let data = tokio::fs::read(file_path).await;
                let data = match data {
                    Result::Ok(result) => result,
                    Result::Err(err) => return Result::Err(err.to_string()),
                };
                let data = String::from_utf8(data).map_err(|e| e.to_string() )?;
                let obj = serde_json::from_str(&*data).map_err(|e| e.to_string())?;
                log::info!("Config [{}] loaded", file_path);
                Result::Ok(obj)
            }
        }
    };

    result.into()
}

/*
            pub async fn load() -> std::io::Result<Self> {
                use serde::de::Deserialize;
                log::info!("Config [{}] loading...", #file_path);
                let data = tokio::fs::read(#file_path).await?;
                let data = String::from_utf8(data).unwrap();
                let obj = serde_json::from_str(&*data).unwrap();
                log::info!("Config [{}] loaded", #file_path);
                Ok(obj)
            }

            pub async fn load_from(file_path: &str) -> std::io::Result<Self> {
                use serde::de::Deserialize;
                log::info!("Config [{}] loading...", file_path);
                let data = tokio::fs::read(file_path).await?;
                let data = String::from_utf8(data).unwrap();
                let obj = serde_json::from_str(&*data).unwrap();
                log::info!("Config [{}] loaded", file_path);
                Ok(obj)
            }
*/

/* #[proc_macro_attribute]
pub fn debug_print(args: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemFn);
    //let name = &input.ident;

    let args = syn::parse_macro_input!(args as syn::AttributeArgs);

    panic!("{:?}", input);
} */