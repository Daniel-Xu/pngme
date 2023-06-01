use crate::args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs};
use crate::chunk::Chunk;

use crate::png::Png;
use std::fs;

pub fn encode(args: EncodeArgs) -> anyhow::Result<()> {
    let file = fs::read(&args.file)?;

    let chunk = Chunk::new(args.chunk_type, args.content.as_bytes().to_vec());

    let mut png = Png::try_from(file.as_ref())?;
    png.append_chunk(chunk);

    fs::write(&args.file, png.as_bytes())?;

    println!("after encoding {png}");

    Ok(())
}

pub fn decode(args: DecodeArgs) -> anyhow::Result<()> {
    let file = fs::read(&args.file)?;
    let png = Png::try_from(file.as_ref())?;
    if let Some(chunk) = png.chunk_by_type(&args.chunk_type.to_string()) {
        println!("the content is: {}", chunk.data_as_string()?);
    } else {
        println!("no chunk of this type: {}", &args.chunk_type);
    }

    Ok(())
}

pub fn remove(args: RemoveArgs) -> anyhow::Result<()> {
    let file = fs::read(&args.file)?;
    let mut png = Png::try_from(file.as_ref())?;

    let _ = png.remove_chunk(&args.chunk_type.to_string())?;

    fs::write(&args.file, png.as_bytes())?;

    Ok(())
}

pub fn print(args: PrintArgs) -> anyhow::Result<()> {
    let file = fs::read(&args.file)?;
    let png = Png::try_from(file.as_ref())?;

    for chunk in png.chunks() {
        println!("chunk type is: {}", chunk.chunk_type());

        if let Ok(data) = chunk.data_as_string() {
            println!("chunk data is: {data}");
        } else {
            println!("chunk data is: {:?}", chunk.as_bytes());
        }
    }

    Ok(())
}
