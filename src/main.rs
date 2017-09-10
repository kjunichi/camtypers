extern crate webcamrs;
extern crate imgtypers;

#[derive(Debug)]
struct Ppm {
    pub width: i32,
    pub height: i32,
    pub index: i32,
}

fn dmprint(str: &Vec<u8>) {
    let mut i = 0;
    loop {
        if str[i] == '\0' as u8 {
            break;
        }
        print!("{}", str[i] as char);
        i = i + 1;
    }
    print!("\n");
}
fn atoi(num: String) -> i32 {
    // println!("num = {}, {}", num, num.len());
    num.parse().expect("not a number")
}

fn findChar(buf: &Vec<u8>, target: u8, len: i32) -> i32 {
    for i in 0..len {
        if buf[i as usize] == target {
            return i;
        }
    }
    return -1;
}

fn strncpy(mut line: &mut Vec<u8>, buf: &Vec<u8>, pos: i32) {
    for i in 0..pos {
        line[i as usize] = buf[i as usize];
    }
}

fn readLine_(buf: &Vec<u8>, line: &mut Vec<u8>, len: i32) -> i32 {
    let pos = findChar(&buf, '\n' as u8, len);
    if pos < 0 {
        return -1;
    }
    strncpy(line, buf, pos);
    line[pos as usize] = '\0' as u8;
    // dmprint(line);
    return pos + 1;
}

fn readLine(buf: &mut Vec<u8>, line: &mut Vec<u8>, len: i32) -> i32 {
    let mut tmpbuf = buf;
    let mut n: i32 = 0;

    loop {
        let nn = readLine_(tmpbuf, line, 1024);
        if nn < 0 {
            return -1;
        }
        n += nn;
        for idx in 0..nn {
            tmpbuf.remove(0);
        }
        // print!("line = ");
        // dmprint(line);
        if !(line[0] == '#' as u8 && n < len) {
            break;
        }
    }
    return n;
}

fn get_image_size(buf: &mut Vec<u8>) -> (i32, i32) {
    let mut tmpStr: Vec<u8> = Vec::with_capacity(256);
    for i in 0..255 {
        tmpStr.push(' ' as u8);
    }
    let pos: i32 = findChar(&buf, ' ' as u8, buf.len() as i32);
    // println!("pos = {}", pos);
    let mut ptr = &mut tmpStr;
    strncpy(ptr, buf, pos);
    (*ptr)[pos as usize] = '\0' as u8;
    // dmprint(ptr);
    let mut wv = vec![];
    for i in 0..pos {
        wv.push((*ptr)[i as usize]);
    }
    let wstr = unsafe { String::from_utf8_unchecked(wv) };
    let width = atoi(wstr);
    let mut hv = vec![];
    for i in pos + 1..buf.len() as i32 {
        if buf[i as usize] == '\0' as u8 {
            break;
        }
        hv.push(buf[i as usize]);
    }
    let hstr = unsafe { String::from_utf8_unchecked(hv) };
    let height = atoi(hstr);
    return (width, height);
}

fn parse_ppm_header(ppmbuf: &mut Vec<u8>, len: i32) -> (i32, i32, i32) {
    // print!("ppmbuf = [");
    // for i in 0..12 {
    // print!("{}", (*ppmbuf)[i as usize] as char);
    // }
    // println!("]");
    let mut line: Vec<u8> = Vec::with_capacity(1024);
    for i in 0..1023 {
        line.push(' ' as u8);
    }
    // println!("ppmbuf.len = {}", (*ppmbuf).len());
    let mut tmpbuf = &mut *ppmbuf;
    // print!("tmpbuf = [");
    // for i in 0..12 {
    // print!("{}", (*tmpbuf)[i as usize] as char);
    // }
    // println!("]");
    //
    let mut headerLength: i32 = 0;

    // PNM TYPE
    let mut lineptr = &mut line;
    let mut n = readLine(tmpbuf, lineptr, 1024);
    //     println!("n = {}", n);
    // print!("lineptr = [");
    // for i in 0..n {
    // print!("{}", (*lineptr)[i as usize] as char);
    // }
    // println!("]");
    // print!("tmpbuf = [");
    // for i in 0..12 {
    // print!("{}", (*tmpbuf)[i as usize] as char);
    // }
    // println!("]");
    //
    headerLength += n;
    n = readLine(tmpbuf, lineptr, 1024);
    //     println!("n = {}", n);
    // print!("tmpbuf = [");
    // for i in 0..12 {
    // print!("{}", (*tmpbuf)[i as usize] as char);
    // }
    // println!("]");
    //
    headerLength += n;
    //     print!("lineptr = [");
    // for i in 0..n - 1 {
    // print!("{}", (*lineptr)[i as usize] as char);
    // }
    // println!("]");
    //
    // lineptr = &mut line;
    let (width, height) = get_image_size(lineptr);

    n = readLine(tmpbuf, lineptr, 1024);

    // depth = atoi(line);
    headerLength += n;
    return (headerLength, width, height);
}

fn parse_ppm_p6(buf: &mut Vec<u8>, len: i32) -> Ppm {
    let mut ptr = &mut *buf;
    let (pos, width, height) = parse_ppm_header(ptr, len);
    Ppm {
        width: width,
        height: height,
        index: pos,
    }
}

fn main() {
    imgtypers::term_init();
    let name = "WebCam test";
    let capture = webcamrs::webcam::create_camera_capture(0);
    webcamrs::webcam::named_window(name);
    loop {
        let frame = webcamrs::webcam::query_frame(&capture);
        webcamrs::webcam::show_image(name, &frame);

        let params = vec![];
        let mut mat = webcamrs::webcam::encode_image(".ppm", &frame, params);
        let mut matptr = &mut mat.buf;
        let ppminfo = parse_ppm_p6(matptr, mat.cols);
        for i in 0..(ppminfo.index) {
            (*matptr).remove(0);
        }
        imgtypers::term_put_image(matptr, ppminfo.width, ppminfo.height);
        imgtypers::term_flush();

        let c = webcamrs::webcam::wait_key(2);
        if c == 0x1b {
            break;
        }
        if c == 0x20 {
            webcamrs::webcam::save_image("snap.jpg", &frame);
            // break;
            let params = vec![];
            let mut mat = webcamrs::webcam::encode_image(".ppm", &frame, params);
            let mut matptr = &mut mat.buf;
            let ppminfo = parse_ppm_p6(matptr, mat.cols);
            for i in 0..(ppminfo.index) {
                (*matptr).remove(0);
            }
            imgtypers::term_put_image(matptr, ppminfo.width, ppminfo.height);
            imgtypers::term_flush();
        }
    }
    webcamrs::webcam::release_capture(&capture);
    webcamrs::webcam::destroy_all_windows();
    imgtypers::term_close();
}