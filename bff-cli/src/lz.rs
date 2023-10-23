use std::fs::File;
use std::io::{self, BufWriter, Cursor, Read, Write};

use bff::lz::{
    lz4_compress_data_with_header_writer,
    lz4_decompress_data_with_header_parser,
    lzo_compress,
    lzo_decompress,
    lzrs_compress_data_with_header_writer,
    lzrs_decompress_data_with_header_parser,
};
use bff::{BufReader, Endian};
use clap::ValueEnum;

use crate::error::BffCliResult;
use crate::stdio_or_path::StdioOrPath;

#[derive(ValueEnum, Clone, Copy)]
pub enum LzEndian {
    Big,
    Little,
}

#[derive(ValueEnum, Clone, Copy)]
pub enum LzAlgorithm {
    Lzrs,
    Lzo,
    Lz4,
}

fn lz_internal<R: Read, W: Write>(
    uncompressed: &mut R,
    compressed: &mut W,
    endian: Endian,
    algorithm: LzAlgorithm,
) -> BffCliResult<()> {
    let mut buf: Vec<u8> = Vec::new();
    uncompressed.read_to_end(&mut buf)?;

    let mut writer = Cursor::new(Vec::new());

    match algorithm {
        LzAlgorithm::Lzrs => lzrs_compress_data_with_header_writer(&buf, &mut writer, endian)?,
        LzAlgorithm::Lzo => lzo_compress(&buf, &mut writer, endian)?,
        LzAlgorithm::Lz4 => lz4_compress_data_with_header_writer(&buf, &mut writer, endian)?,
    };

    compressed.write_all(&writer.into_inner())?;
    Ok(())
}

pub fn lz(
    uncompressed: &StdioOrPath,
    compressed: &StdioOrPath,
    endian: &LzEndian,
    algorithm: &LzAlgorithm,
) -> BffCliResult<()> {
    let endian = match endian {
        LzEndian::Big => Endian::Big,
        LzEndian::Little => Endian::Little,
    };
    let algorithm = *algorithm;

    match (uncompressed, compressed) {
        (StdioOrPath::Stdio, StdioOrPath::Stdio) => {
            let stdin = io::stdin();
            let stdout = io::stdout();
            lz_internal(&mut stdin.lock(), &mut stdout.lock(), endian, algorithm)
        }
        (StdioOrPath::Stdio, StdioOrPath::Path(output_path)) => {
            let stdin = io::stdin();
            let mut output = BufWriter::new(File::create(output_path)?);
            lz_internal(&mut stdin.lock(), &mut output, endian, algorithm)
        }
        (StdioOrPath::Path(input_path), StdioOrPath::Stdio) => {
            let mut input = BufReader::new(File::open(input_path)?);
            let stdout = io::stdout();
            lz_internal(&mut input, &mut stdout.lock(), endian, algorithm)
        }
        (StdioOrPath::Path(input_path), StdioOrPath::Path(output_path)) => {
            let mut input = BufReader::new(File::open(input_path)?);
            let mut output = BufWriter::new(File::create(output_path)?);
            lz_internal(&mut input, &mut output, endian, algorithm)
        }
    }
}

fn unlz_internal<R: Read, W: Write>(
    compressed: &mut R,
    uncompressed: &mut W,
    endian: Endian,
    algorithm: LzAlgorithm,
) -> BffCliResult<()> {
    let mut buf: Vec<u8> = Vec::new();
    compressed.read_to_end(&mut buf)?;

    let mut reader = BufReader::new(Cursor::new(buf));

    let decompressed = match algorithm {
        LzAlgorithm::Lzrs => lzrs_decompress_data_with_header_parser(&mut reader, endian)?,
        LzAlgorithm::Lzo => lzo_decompress(&mut reader, endian)?,
        LzAlgorithm::Lz4 => lz4_decompress_data_with_header_parser(&mut reader, endian)?,
    };

    uncompressed.write_all(&decompressed)?;
    Ok(())
}

pub fn unlz(
    compressed: &StdioOrPath,
    uncompressed: &StdioOrPath,
    endian: &LzEndian,
    algorithm: &LzAlgorithm,
) -> BffCliResult<()> {
    let endian = match endian {
        LzEndian::Big => Endian::Big,
        LzEndian::Little => Endian::Little,
    };
    let algorithm = *algorithm;

    match (compressed, uncompressed) {
        (StdioOrPath::Stdio, StdioOrPath::Stdio) => {
            let stdin = io::stdin();
            let stdout = io::stdout();
            unlz_internal(&mut stdin.lock(), &mut stdout.lock(), endian, algorithm)
        }
        (StdioOrPath::Stdio, StdioOrPath::Path(output_path)) => {
            let stdin = io::stdin();
            let mut output = BufWriter::new(File::create(output_path)?);
            unlz_internal(&mut stdin.lock(), &mut output, endian, algorithm)
        }
        (StdioOrPath::Path(input_path), StdioOrPath::Stdio) => {
            let mut input = BufReader::new(File::open(input_path)?);
            let stdout = io::stdout();
            unlz_internal(&mut input, &mut stdout.lock(), endian, algorithm)
        }
        (StdioOrPath::Path(input_path), StdioOrPath::Path(output_path)) => {
            let mut input = BufReader::new(File::open(input_path)?);
            let mut output = BufWriter::new(File::create(output_path)?);
            unlz_internal(&mut input, &mut output, endian, algorithm)
        }
    }
}
