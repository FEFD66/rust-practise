static MAP: [char; 64] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'];

pub fn encode(data: &[u8]) -> String {
    let mut res = String::with_capacity((data.len() + 2) / 3 * 4);
    let mut i =0;
    while i < data.len() {
        let (mut trip,mut quad)=(false,false);
        let mut val:usize= data[i] as usize;
        val=val<<8;
        if (i+1)<data.len() {
            val =val + data[i+1] as usize;
            trip=true;
        }
        val=val<<8;
        if (i+2)<data.len() {
            val =val + data[i+2] as usize;
            quad=true;
        }
        let mut buf = ['=','=','=','='];

        if quad {buf[3]=MAP[val&0x3F]};
        val=val>>6;

        if trip{buf[2]=MAP[val&0x3F]};
        val=val>>6;

        buf[1]=MAP[val&0x3F];
        val=val>>6;

        buf[0]=MAP[val&0x3F];
        val=val>>6;

        for i in buf{
            res.push(i);
        }
        i=i+3;
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{Read, Write};

    #[test]
    fn encode() {
        let mut file =File::open(r#"C:\Users\Kyanc\Pictures\mhw.png"#).unwrap();
        let mut data = vec![];
        file.read_to_end(&mut data).unwrap();
        let s = r#"Running unittests (target\debug\deps\practise-1da89ba00bb79a0b.exe)"#;
        let encoded = super::encode(data.as_slice());
        let mut out_file=File::create(r#"D:\out.txt"#).unwrap();
        out_file.write_all("data:image/png;base64,".as_bytes()).unwrap();
        out_file.write_all(encoded.as_bytes()).unwrap();
        out_file.sync_all().unwrap();
        println!("{}", encoded);
    }
}
