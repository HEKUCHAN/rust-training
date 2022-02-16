use image::{self, imageops, GenericImageView};

fn main() {
    let size = 128;

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("[USAGE] image_thumb imagefile");
        return;
    }

    let infile = String::from(&args[1]);
    let outfile = format!("{}-thimb.png", infile);
    println!("input: {}", infile);
    println!("output: {}", outfile);

    let mut img = image::open(infile)
        .expect("画像ファイルが読めません");
    let dim = img.dimensions();
    let w = if dim.0 > dim.1 {dim.1} else {dim.0};
    let mut img2 = imageops::crop(&mut img,
        (dim.0-w)/2, (dim.1-w)/2, w, w).to_image();
    let img3 = imageops::resize(&mut img2, size, size,
        imageops::Lanczos3);
    img3.save(outfile).unwrap();

}
