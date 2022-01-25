use std::env;
use std::fs;
use std::str;

use std::path::Path;
use std::process::Command;

extern crate byteorder;
use byteorder::{LittleEndian, WriteBytesExt};

struct SampleParser {
    samp_data: Vec<i16>,
    rshift: u32,
    spike_count: u32,
    samp_done: bool,
}

impl SampleParser {
    // process a single data byte
    // return true if a new sample is ready
    fn process(&mut self, b: i8) -> bool {
        let mut ret = false;
        if b == -128 {
            if self.samp_done {
                // nothing to do
            } else {
                self.rshift += 1;
                self.spike_count += 1;
                if self.spike_count > 2 {
                    self.samp_done = true;
                    ret = true;
                }
            }
        } else {
            if self.samp_done {
                self.rshift = 0;
                self.samp_done = false;
            }
            let c = (b as i16) << 8 >> self.rshift;
            self.samp_data.push(c);
        }
        return ret;
    }

    // get a copy of the current sample data, reset internal state
    fn get_sample(&mut self) -> Vec<i16> {
        let samp_data_copy = self.samp_data.to_vec();
        self.spike_count = 0;
        self.samp_data.clear();
        self.rshift = 0;
        return samp_data_copy;
    }
}

// given an iterator over raw binary data,
// return a vector of audio samples
fn parse_bin(bin: Vec<u8>) -> Vec<Vec<i16>> {
    let mut samps: Vec<Vec<i16>> = Vec::new();
    let mut smp = SampleParser {
        samp_data: Vec::new(),
        rshift: 0,
        spike_count: 0,
        samp_done: false,
    };
    for b in bin {
        if smp.process(b as i8) {
            samps.push(smp.get_sample());
        }
    }
    return samps;
}

fn export_sample(samp: &Vec<i16>, path: &str) {
    let mut file = fs::File::create(path).expect("couldn't open file for write");
    for frame in samp {
        file.write_i16::<LittleEndian>(*frame).unwrap();
    }
}

fn convert_raw(path: &str) {
    let out_path = path.replace(".raw", ".wav");
    Command::new("sox")
            .args(["-r48000", "-b16", "-c1", "-L", "-esigned-integer", path, &out_path])
            .output()
            .expect("failed to execute sox command");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let b = fs::read(&args[1]).unwrap();

    println!("parsing...");
    let samps = parse_bin(b);

    let nsamps =samps.len(); 
    println!("found {} samples", nsamps);

    let base_out_path = Path::new(&args[2]);
    let base_out_name = &args[3];
    fs::create_dir_all(base_out_path).expect("failed to create output directory");
    let mut i = 1;
    println!("exporting...");
    for samp in samps {    
        println!("{}/{}", i, nsamps);
        let out_path = base_out_path.join(Path::new(&format!("{}_{:03}.raw", base_out_name, i))).display().to_string();      
        i += 1;
        export_sample(&samp, &out_path);
        convert_raw(&out_path);
    }
    println!("...done.");

}
