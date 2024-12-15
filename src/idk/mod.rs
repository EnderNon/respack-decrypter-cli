use std::{
    fmt::Debug,
    io::{Read, Write},
    usize,
};
use std::io::Cursor;
use adler::Adler32;
use crc32fast::Hasher;
use miniz_oxide::inflate::core::{
    decompress,
    inflate_flags::{
        TINFL_FLAG_IGNORE_ADLER32, TINFL_FLAG_PARSE_ZLIB_HEADER,
        TINFL_FLAG_USING_NON_WRAPPING_OUTPUT_BUF,
    },
    DecompressorOxide,
};

struct Chunk {
    kind: [u8; 4],
    data: Vec<u8>,
    crc: [u8; 4],
}

impl Debug for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Chunk")
            .field("kind", &self.kind)
            .field("data len", &self.data.len())
            .field("crc", &self.crc)
            .finish()
    }
}

impl Chunk {
    fn kind_to_string(&self) -> String {
        String::from_utf8_lossy(&self.kind).to_string()
    }
}

pub fn fix(mut bytes: Vec<u8>) -> Vec<u8> {
    let mut bufread = Cursor::new(bytes.clone());

    // read the png header
    let mut header = [0; 8];
    bufread.read_exact(&mut header).unwrap();

    let mut chunks = Vec::new();

    let mut lenbuf = [0; 4];
    while let Ok(_) = bufread.read_exact(&mut lenbuf) {
        let len = u32::from_be_bytes(lenbuf);

        let mut kind = [0; 4];
        bufread.read_exact(&mut kind).unwrap();

        let data = {
            let mut data = vec![0; len as usize];
            bufread.read_exact(&mut data).unwrap();
            data
        };

        let mut crc = [0; 4];
        bufread.read_exact(&mut crc).unwrap();

        let mut chunk = Chunk { kind, data, crc };
        // println!("{:?}", chunk);

        // recode the compressed image data
        if chunk.kind == *b"IDAT" {
            // println!("Decompressing IDAT chunk");
            let mut decompressor = DecompressorOxide::new();
            decompressor.init();
            let mut buf = vec![0; 1024 * 1024 * 1024]; // this could probably be smaller
            let data = decompress(
                &mut decompressor,
                &chunk.data,
                &mut buf,
                0,
                TINFL_FLAG_IGNORE_ADLER32
                    | TINFL_FLAG_PARSE_ZLIB_HEADER
                    | TINFL_FLAG_USING_NON_WRAPPING_OUTPUT_BUF,
            );

            // println!(
            //     "Decompressed IDAT chunk status {:?}, bytes read {}, bytes outputted {}",
            //     data.0, data.1, data.2
            // );

            let _ = buf.split_off(data.2);

            let mut adler = Adler32::new();
            adler.write_slice(&buf);

            let csum = adler.checksum().to_be_bytes();

            // replace the last 4 bytes of the data with the new checksum
            let data_len = chunk.data.len();
            chunk.data[data_len - 4..].copy_from_slice(&csum);
            // println!("Corrected Adler32 checksum");
        }

        let mut hasher = Hasher::new();
        hasher.update(&chunk.kind);
        hasher.update(&chunk.data);
        let checksum = hasher.finalize();

        if checksum != u32::from_be_bytes(chunk.crc) {
            // println!("CRC error in chunk {:?}", chunk.kind_to_string());
            chunk.crc = checksum.to_be_bytes();
            // println!("Corrected CRC");
        }

        chunks.push(chunk);
    }

    // new cursor
    let mut newcursor = Cursor::new(Vec::new());
    // write a new header
    newcursor
        .write_all(&[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A])
        .unwrap();

    for chunk in chunks {
        newcursor
            .write_all(&u32::to_be_bytes(chunk.data.len() as u32))
            .unwrap();
        newcursor.write_all(&chunk.kind).unwrap();
        newcursor.write_all(&chunk.data).unwrap();
        newcursor.write_all(&chunk.crc).unwrap();
    }
    let finalres: Vec<u8> = newcursor.into_inner();
    finalres
}
