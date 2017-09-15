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

fn find_char(buf: &Vec<u8>, target: u8, len: i32) -> i32 {
    for i in 0..len {
        if buf[i as usize] == target {
            return i;
        }
    }
    return -1;
}

fn strncpy(line: &mut Vec<u8>, buf: &Vec<u8>, pos: i32) {
    for i in 0..pos {
        line[i as usize] = buf[i as usize];
    }
}

fn read_line_(buf: &Vec<u8>, line: &mut Vec<u8>, len: i32) -> i32 {
    let pos = find_char(&buf, '\n' as u8, len);
    if pos < 0 {
        return -1;
    }
    strncpy(line, buf, pos);
    line[pos as usize] = '\0' as u8;
    // dmprint(line);
    return pos + 1;
}

fn read_line(buf: &mut Vec<u8>, line: &mut Vec<u8>, len: i32) -> i32 {
    let tmpbuf = buf;
    let mut n: i32 = 0;

    loop {
        let nn = read_line_(tmpbuf, line, 1024);
        if nn < 0 {
            return -1;
        }
        n += nn;
        for _ in 0..nn {
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
    let mut tmp_str: Vec<u8> = Vec::with_capacity(256);
    for _ in 0..255 {
        tmp_str.push(' ' as u8);
    }
    let pos: i32 = find_char(&buf, ' ' as u8, buf.len() as i32);
    let ptr = &mut tmp_str;
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
    let mut line: Vec<u8> = Vec::with_capacity(len as usize);
    for _ in 0..len {
        line.push(' ' as u8);
    }
    let tmpbuf = &mut *ppmbuf;
    let mut header_length: i32 = 0;

    // PNM TYPE
    let lineptr = &mut line;
    let mut n = read_line(tmpbuf, lineptr, 1024);
    header_length += n;
    n = read_line(tmpbuf, lineptr, 1024);
    header_length += n;
    let (width, height) = get_image_size(lineptr);

    n = read_line(tmpbuf, lineptr, 1024);

    header_length += n;
    return (header_length, width, height);
}

fn parse_ppm_p6(buf: &mut Vec<u8>, len: i32) -> Ppm {
    let ptr = &mut *buf;
    let (pos, width, height) = parse_ppm_header(ptr, len);
    Ppm {
        width: width,
        height: height,
        index: pos,
    }
}

fn main() {
    imgtypers::term_init();
    let capture = webcamrs::webcam::create_camera_capture(0);

    loop {
        let frame = webcamrs::webcam::query_frame(&capture);
        let params = vec![];
        let mut mat = webcamrs::webcam::encode_image(".ppm", &frame, params);
        let matptr = &mut mat.buf;
        let ppminfo = parse_ppm_p6(matptr, mat.cols);
        for _ in 0..(ppminfo.index) {
            (*matptr).remove(0);
        }
        imgtypers::term_put_image(matptr, ppminfo.width, ppminfo.height);
        imgtypers::term_flush();

        if imgtypers::term_get_esc_key() {
            break;
        }
    }
    webcamrs::webcam::release_capture(&capture);
    imgtypers::term_close();
}
