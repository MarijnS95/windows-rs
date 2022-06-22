pub use bindgen::metadata::*;

pub fn default_metadata() -> Vec<metadata::File> {
    vec![
        metadata::File::new(
            std::include_bytes!("../../../libs/bindgen/default/Windows.winmd").to_vec(),
        )
        .expect("invalid winmd"),
        metadata::File::new(
            std::include_bytes!("../../../libs/bindgen/default/Windows.Win32.winmd").to_vec(),
        )
        .expect("invalid winmd"),
        metadata::File::new(
            std::include_bytes!("../../../libs/bindgen/default/Windows.Wdk.winmd").to_vec(),
        )
        .expect("invalid winmd"),
    ]
}

fn main() {
    let files = default_metadata();
    let reader = Reader::new(files);

    for ns in reader.namespaces() {
        for def in reader.namespace_items(ns) {
            if let Item::Type(def) = def {
                if def.kind() == TypeKind::Delegate
                    && !def.flags().contains(TypeAttributes::WindowsRuntime)
                {
                    for method in def.methods() {
                        let signature = method_def_signature(ns, method, &[]);
                        match signature.kind() {
                            SignatureKind::Query(q) | SignatureKind::QueryOptional(q) => {
                                let QueryPosition { object, guid } = &q;
                                println!("{}", def.name());
                                // , reader.type_signature(&signature.params[*guid].ty), reader.type_signature(&signature.params[*object].ty));
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }
}
