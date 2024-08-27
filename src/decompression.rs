pub mod decompress {
    use flate2::read::GzDecoder;
    use std::{fs::File, io::copy, io::BufReader};

    pub fn decode(input_path: &String, output_path: &String) {
        let file = File::open(input_path).expect("Error opening input file");
        let mut decoder = GzDecoder::new(file);

        let mut output_file = File::create(output_path.split("_fileziler").nth(0).unwrap())
            .expect("Error creating output file");
        copy(&mut decoder, &mut output_file).expect("Error during decompression");
    }
}
