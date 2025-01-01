use error_chain::error_chain;
use std::io::copy;
use std::fs::File;
use tempfile::Builder;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn  main()->Result<()> {
    let temp_dir=Builder::new().prefix("example").tempdir()?;
    let target = "https://www.rust-lang.org/logo/rust-logo-512x512.png";
    let res = reqwest::get(target).await?;
    let mut dest = {
        let fname = res.url().path_segments().and_then(|seg| seg.last()).and_then(|name| if name.is_empty(){None}else{Some(name.to_string())} )
            .unwrap_or("tmp.bin".into());
        println!("downloading {}...", fname);
        let fname = temp_dir.path().join(fname);
        println!("will be located under {}", fname.display());
        File::create(fname)?


    };
    let content = res.text().await?;
    copy(&mut content.as_bytes(), &mut dest)?;
    Ok(())



}
