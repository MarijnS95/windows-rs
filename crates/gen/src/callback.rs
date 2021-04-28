use super::*;

pub fn gen_callback(def: &TypeDef, gen: &Gen) -> TokenStream {
    let name = gen_type_name(def, gen);
    let signature = def.invoke_method().signature(&[]);

    // Note that callbacks are C-style function pointers so the code gen will only use ABI types
    // to ensure the the ABI is faithfully preserved. Other types generally provide an abstraction
    // over the ABI but in this case that is not practical.

    let params = signature.params.iter().map(|p| {
        let name = gen_param_name(&p.param);
        let tokens = gen_win32_abi_param(p, gen);
        quote! { #name: #tokens }
    });

    let return_sig = gen_win32_return_sig(&signature, gen);

    let query_interface_fn = if signature.kind() == SignatureKind::Query {
        let constraints = gen_method_constraints(&signature.params, gen);
        let leading_params = &signature.params[..signature.params.len() - 2];
        let params = gen_win32_params(leading_params, gen);
        let args = leading_params.iter().map(|p| gen_win32_abi_arg(p));
        quote! {
            pub unsafe fn #name<#constraints T: ::windows::Interface>(func: &#name, #params) -> ::windows::Result<T> {
                let mut result__ = ::std::option::Option::None;
                (func)(#(#args,)* &<T as ::windows::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
            }
        }
    } else {
        quote!()
    };

    quote! {
        pub type #name = unsafe extern "system" fn(#(#params),*) #return_sig;
        #query_interface_fn
    }
}
