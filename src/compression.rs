pub mod compress {
    use flate2::{write::GzEncoder, Compression};
    use std::{fs::File, io::BufReader};

    pub fn encode(input_path: &String, output_path: &String) {
        let file = File::open(input_path).expect("Error opening input file");
        let mut reader = BufReader::new(file);

        let output_file =
            File::create(output_path.split(".").nth(0).unwrap().trim().to_owned() + "_fileziler")
                .expect("Error creating output file");
        let mut encoder = GzEncoder::new(output_file, Compression::default());

        std::io::copy(&mut reader, &mut encoder).expect("Error during compression");

        encoder.finish().expect("Error finalizing compression");
    }
}
