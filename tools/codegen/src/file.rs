use std::path::{Path, PathBuf};

use anyhow::Result;
use fs_err as fs;
use proc_macro2::TokenStream;

pub(crate) fn workspace_root() -> PathBuf {
    let mut dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    dir.pop(); // codegen
    dir.pop(); // tools
    dir
}

fn header() -> String {
    concat!(
        "// This file is @generated by ",
        env!("CARGO_BIN_NAME"),
        ".\n",
        "// It is not intended for manual editing.\n",
        "\n",
    )
    .into()
}

pub(crate) fn write(path: &Path, contents: TokenStream) -> Result<()> {
    let mut out = header().into_bytes();
    out.extend_from_slice(prettyplease::unparse(&syn::parse2(contents).unwrap()).as_bytes());
    if path.is_file() && fs::read(&path)? == out {
        return Ok(());
    }
    fs::write(path, out)?;
    Ok(())
}
