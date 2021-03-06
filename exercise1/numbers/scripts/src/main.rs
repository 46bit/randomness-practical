extern crate extprim;
extern crate image;

use std::env;
use std::io::Write;
//use extprim::u128::u128;
use std::fs::File;
use std::path::Path;
use std::f32;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();

    /*let a: u32 = 0;
    let m: u32 = 1;
    let c: u32 = 2;*/

    let a: u32 = args[1].parse().ok().expect("Wanted a number"); //3;
    let m: u32 = args[2].parse().ok().expect("Wanted a number"); //10;
    let c: u32 = args[3].parse().ok().expect("Wanted a number"); //2;
    println!("a={} m={} c={}", a, m, c);
    println!("f(x) = {}x + {} (mod {})", a, c, m);
    println!("");

    let sx: u32 = 600;
    let sy: u32 = 600;
    let capacity: u32 = sx * sy;

    let mut outputs = vec![0; 0];

    let mut s = 1;
    for _ in 0..(16*capacity) {
      s = ((a*s + c) % m) & 0xffff;
      outputs.push(s & 0xffff);
    }

    // Find cycle length
    // 17842 65539 121234`
    // cycle_length=19570 run_in=13230 <--- has run in and doesn't go to a single value
    let mut output_seen = HashMap::new();
    let mut output_index: usize = 0;
    let mut cycle_length: u32 = u32::max_value();
    let mut run_in: u32 = u32::max_value();
    while output_index < outputs.len() as usize {
        let output = outputs[output_index];
        if output_seen.contains_key(&output) {
            let output_seen_at: usize = *output_seen.get(&output).unwrap();
            // Loop found!
            cycle_length = (output_index - output_seen_at) as u32;
            run_in = output_seen_at as u32;
            break;
        }
        output_seen.insert(output, output_index as usize);
        output_index += 1;
    }
    println!("cycle_length={} run_in={}", cycle_length, run_in);

    // Low-order bits cycle length
    // maximal a=1009 m=1372 c=1 has low_order_bit_cycle_length=4 for mask 0x3
    // 2, 3, 0, 1, ...
    // as does maximal 85 65536 1 - plus with 0x7 mask, length 8 [6,7,4,5,2,3,0,1, | 6,7,...]
    // basically you can mine a=85 m=65536 c=1 (maximal) for short patterns. even up to 7 bits
    // or beyond, at which point the cycle is 512 long.
    // good demos:
    // * cargo run 85 65536 1 --> maximal. but LOB cycle lengths are 2**low_order_bit_count.
    // * cargo run 1001 2187 8 --> nonmaximal, cycle_length=162. but LOB cycle lengths are all 162.
    if u32::max_value() != cycle_length {
        for low_order_bit_count in 1..9 {
            let mask = (1<<low_order_bit_count)-1; // 0x127;
            let low_order_bit_cycle_length = find_mask_cycle(mask, a, m, c, cycle_length, run_in, outputs.clone());
            println!("low_order_bit_count={} low_order_bit_cycle_length={}", low_order_bit_count, low_order_bit_cycle_length);
        }
    }

    let path = format!("../a={}-m={}-c={}-intensity.png", a, m, c);
    let ref mut fout = File::create(&Path::new(&path)).unwrap();
    let mut imgbuf = image::ImageBuffer::new(sx, sy);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
      let pixel_output = outputs[(x + y * sx) as usize];
      let mut of = pixel_output as f32;
      of /= m as f32;
      of *= 0xff as f32;
      *pixel = image::Luma([of.round() as u8]);
    }
    image::ImageLuma8(imgbuf).save(fout, image::PNG).unwrap();

    let mut imgbuf2 = image::ImageBuffer::new(sx, sy);
    for i in 1..capacity {
      *imgbuf2.get_pixel_mut((outputs[i as usize] % sx) as u32, (outputs[(i-1) as usize] % sy) as u32) = image::Luma([255 as u8]);
    }
    let path = format!("../a={}-m={}-c={}-pairs.png", a, m, c);
    let ref mut fout2 = File::create(&Path::new(&path)).unwrap();
    image::ImageLuma8(imgbuf2).save(fout2, image::PNG).unwrap();

    let path = format!("../a={}-m={}-c={}-denary.txt", a, m, c);
    let ref mut f = File::create(&Path::new(&path)).unwrap();
    let mut sv = vec![];
    for i in 1..300 {
        sv.push(format!("{:6}", outputs[i]));
    }
    let mut s = String::from("");//format!("cycle_length={}\n", cycle_length));
    s.push_str(&*sv.join("\n"));
    f.write_all(s.as_bytes()).unwrap();

    let path = format!("../a={}-m={}-c={}-binary.txt", a, m, c);
    let ref mut f = File::create(&Path::new(&path)).unwrap();
    let mut sv = vec![];
    for i in 1..300 {
        sv.push(format!("…{:05b}", outputs[i] & 0x1f));
    }
    let mut s = String::from("");//format!("cycle_length={}\n", cycle_length));
    s.push_str(&*sv.join("\n"));
    f.write_all(s.as_bytes()).unwrap();

    let path = format!("../a={}-m={}-c={}-binary-b.txt", a, m, c);
    let ref mut f = File::create(&Path::new(&path)).unwrap();
    let mut sv = vec![];
    sv.push("bit0,bit1,bit2,bit3,bit4,bit5,bit6,bit7,bit8,bit9,bit10,bit11,bit12,bit13,bit14,bit15");
    for i in 1..24 {
        let output16 = outputs[i] as u16;
        sv.push(format!("{:01b},{:01b},{:01b},{:01b},{:01b},{:01b},{:01b},{:01b},{:01b},{:01b},{:01b},{:01b},{:01b},{:01b},{:01b},{:01b}", output16 & 0x1, output16 & 0x2, output16 & 0x4, output16 & 0x8, output16 & 0x10, output16 & 0x20, output16 & 0x40, output16 & 0x80, (output16 >> 1) & 0x80, (output16 >> 2) & 0x80, (output16 >> 3) & 0x80, (output16 >> 4) & 0x80, (output16 >> 5) & 0x80, (output16 >> 6) & 0x80, (output16 >> 7) & 0x80, (output16 >> 8) & 0x80));
    }
    let mut s = String::from("");//format!("cycle_length={}\n", cycle_length));
    s.push_str(&*sv.join("\n"));
    f.write_all(s.as_bytes()).unwrap();
}

fn find_mask_cycle(mask: u32, a: u32, m: u32, c: u32, cycle_length: u32, run_in: u32, outputs: Vec<u32>) -> u32 {
    let mut low_order_bit_cycle_length = 1;
    let mut explains = false;
    let first_output_of_full_cycle = outputs[run_in as usize];
    while !explains && low_order_bit_cycle_length < cycle_length {
        let new_output = outputs[(low_order_bit_cycle_length + run_in) as usize];
        //println!("{} {} {}", low_order_bit_cycle_length, new_output, first_output_of_full_cycle);
        if new_output & mask == first_output_of_full_cycle & mask {
            let mut s1 = first_output_of_full_cycle;
            let mut s2 = new_output;
            explains = true;
            for _ in 0..cycle_length {
                s1 = ((a*s1 + c) % m) & 0xffff;
                s2 = ((a*s2 + c) % m) & 0xffff;
                //println!("FF {} {}=?={} {}", s1, s1 & mask, s2 & mask, s2);
                if s1 & mask != s2 & mask {
                    //println!("FF {} {}!=!=!{} {}", s1, s1 & mask, s2 & mask, s2);
                    explains = false;
                    break;
                }
            }
        }
        if !explains {
            low_order_bit_cycle_length += 1;
        }
    }
    if !explains {
        low_order_bit_cycle_length = cycle_length;
    }
    low_order_bit_cycle_length
}

/*fn save_lcg_output_file(a: u32, m: u32, c: u32, name: str, content) {

}*/
