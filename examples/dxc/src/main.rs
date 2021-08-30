use bindings::Windows::Win32::{
    Foundation::{BOOL, PWSTR},
    Graphics::Hlsl::*,
    System::Diagnostics::Debug::ERROR_FILE_NOT_FOUND,
};
use libloading::{Library, Symbol};
use std::path::Path;
use std::rc::Rc;
use windows::*;

fn blob_encoding_as_str(blob: &IDxcBlobEncoding) -> &str {
    let mut known: BOOL = false.into();
    let mut cp = DXC_CP::default();
    unsafe { blob.GetEncoding(&mut known as *mut _ as _, &mut cp as *mut _ as _) }.unwrap();
    assert!(bool::from(known));
    assert_eq!(cp, DXC_CP_UTF8);
    unsafe {
        let slice = std::slice::from_raw_parts(
            blob.GetBufferPointer() as *const u8,
            blob.GetBufferSize() - 1,
        );
        std::str::from_utf8_unchecked(slice)
    }
}

fn create_blob(library: &IDxcLibrary, data: &str) -> windows::Result<IDxcBlobEncoding> {
    unsafe {
        library.CreateBlobWithEncodingFromPinned(
            data.as_ptr() as *const _,
            data.len() as u32,
            DXC_CP_UTF8,
        )
    }
}

use bindings::Windows;

#[implement(Windows::Win32::Graphics::Hlsl::IDxcIncludeHandler)]
struct IncludeHandler {
    library: Rc<IDxcLibrary>,
}

impl IncludeHandler {
    #[allow(non_snake_case)]
    fn LoadSource(&self, pfilename: PWSTR) -> Result<IDxcBlobEncoding> {
        if pfilename != "./foo/bar/copy.hlsl" {
            return Err(Error::fast_error(ERROR_FILE_NOT_FOUND.into()));
        }
        let copy_shader = include_str!("copy.hlsl");
        Ok(create_blob(&self.library, copy_shader).unwrap())
    }
}

#[allow(non_snake_case)]
fn main() -> windows::Result<()> {
    let compiler: IDxcCompiler2 = unsafe { DxcCreateInstance(&CLSID_DxcCompiler) }?;
    let library: IDxcLibrary = unsafe { DxcCreateInstance(&CLSID_DxcLibrary) }?;

    dbg!(&compiler, &library);

    let main_shader = "#include \"copy.hlsl\"";
    let shader_blob = create_blob(&library, main_shader)?;

    let library = Rc::new(library);
    let handler = IncludeHandler { library };
    let handler: IDxcIncludeHandler = handler.into();
    dbg!(&handler);

    let mut args = vec![];
    let defines = vec![];
    let result = unsafe {
        compiler.Compile(
            shader_blob,
            "foo/bar/baz.hlsl",
            "copyCs",
            "cs_6_5",
            args.as_mut_ptr(), // Should not be mut?
            args.len() as u32,
            defines.as_ptr(),
            defines.len() as u32,
            // TODO: This also accepts a borrow which does not decrease our refcount to 0
            handler,
        )
    }?;

    let status = unsafe { result.GetStatus() }?;
    if status.is_err() {
        let errors = unsafe { result.GetErrorBuffer() }?;
        let errors = blob_encoding_as_str(&errors);
        eprintln!("Compilation failed with {:?}: `{}`", status, errors);
        status.ok()
    } else {
        let blob = unsafe { result.GetResult() }?;
        let shader = unsafe {
            std::slice::from_raw_parts(blob.GetBufferPointer().cast::<u8>(), blob.GetBufferSize())
        };
        for c in shader.chunks(16) {
            println!("{:02x?}", c);
        }
        Ok(())
    }
}
