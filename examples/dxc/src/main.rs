use bindings::Windows::Win32::{Foundation::BOOL, Graphics::Hlsl::*};
use libloading::{Library, Symbol};
use std::path::Path;
use windows::*;

#[cfg(target_os = "windows")]
fn dxcompiler_lib_name() -> &'static Path {
    Path::new("dxcompiler.dll")
}

#[cfg(target_os = "linux")]
fn dxcompiler_lib_name() -> &'static Path {
    Path::new("./libdxcompiler.so")
}

#[cfg(target_os = "macos")]
fn dxcompiler_lib_name() -> &'static Path {
    Path::new("./libdxcompiler.dynlib")
}

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

fn main() -> windows::Result<()> {
    let lib = unsafe { Library::new(dxcompiler_lib_name()) }.unwrap();
    let create: Symbol<DxcCreateInstanceProc> = unsafe { lib.get(b"DxcCreateInstance\0") }.unwrap();
    dbg!(&create);

    let compiler: IDxcCompiler2 = unsafe { DxcCreateInstanceProc(&create, &CLSID_DxcCompiler) }?;
    let library: IDxcLibrary = unsafe { DxcCreateInstanceProc(&create, &CLSID_DxcLibrary) }?;

    dbg!(&compiler, &library);

    let main_shader = include_str!("copy.hlsl");

    let shader_blob = unsafe {
        library.CreateBlobWithEncodingFromPinned(
            main_shader.as_ptr() as *const _,
            main_shader.len() as u32,
            DXC_CP_UTF8,
        )
    }?;
    dbg!(&shader_blob);

    let mut args = vec![];
    let defines = vec![];

    let result = unsafe {
        compiler.Compile(
            shader_blob,
            "copy.hlsl",
            "copyCs",
            "cs_6_5",
            args.as_mut_ptr(), // Should not be mut?
            args.len() as u32,
            defines.as_ptr(),
            defines.len() as u32,
            None,
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
