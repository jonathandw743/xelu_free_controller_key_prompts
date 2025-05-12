use std::{
    env, fs,
    path::{Path, PathBuf},
};

use fs_extra::dir::CopyOptions;

pub use fs_extra::error::Error;

pub fn copy_assets<P: AsRef<Path>>(dest: P) -> Result<(), Error> {
    let library_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Xelu_Free_Controller&Key_Prompts");
    let user_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join(dest);
    fs::create_dir_all(&user_path)?;
    fs_extra::copy_items(
        &[library_path],
        user_path,
        &CopyOptions {
            overwrite: false,
            skip_exist: true,
            content_only: false,
            copy_inside: false,
            depth: 0,
            buffer_size: Default::default(),
        },
    )?;
    Ok(())
}

#[cfg(feature = "use_tokenize_dir")]
pub mod tokenize_dir {
    tokenize_dir::tokenize_dir!("Xelu_Free_Controller&Key_Prompts"; "_");
}
