mod stl {

    use byteorder::{ByteOrder, LittleEndian};
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;

    #[derive(Debug)]
    pub struct Triangle {
        pub normal: [f32; 3],
        pub points: [[f32; 3]; 3],
    }
    pub struct StlFormat {
        pub header: [u8; 80],
        pub triangles: Vec<Triangle>,
    }

    fn read_triangle<R: Read>(freader: &mut BufReader<R>) -> Triangle {
        let mut normal: [f32; 3] = [0.0; 3];
        let mut points: [[f32; 3]; 3] = [[0.0; 3]; 3];

        //read normal vector
        let mut buf: [u8; 12] = [0; 12];
        freader.read_exact(&mut buf).unwrap();
        LittleEndian::read_f32_into(&buf, &mut normal);

        //read 3 points
        for i in 0..3 {
            freader.read_exact(&mut buf).unwrap();

            LittleEndian::read_f32_into(&buf, &mut points[i]);
        }

        //read attributed 16 bits
        let mut attr: [u8; 2] = [0; 2];
        freader.read_exact(&mut attr).unwrap();
        Triangle { normal, points }
    }

    fn is_binary_file<R: Read>(reader: &mut BufReader<R>) -> bool {
        use std::str;
        let mut solid_buffer: [u8; 5] = [0; 5];
        reader.read_exact(&mut solid_buffer).unwrap();
        match str::from_utf8(&solid_buffer) {
            Ok(s) => s != "solid",
            Err(_) => true,
        }
    }

    fn read_binary<R: Read>(reader: &mut BufReader<R>) -> Result<StlFormat, std::io::Error> {
        let mut header: [u8; 80] = [0; 80];
        match reader.read_exact(&mut header) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }

        let mut tri_count: [u8; 4] = [0, 0, 0, 0];
        reader
            .read_exact(&mut tri_count)
            .expect("error reading number of triangles");

        let number_tri = u32::from_le_bytes(tri_count);

        let mut triangles: Vec<Triangle> = vec![];
        for _ in 0..number_tri {
            triangles.push(read_triangle(reader));
        }
        Ok(StlFormat { header, triangles })
    }
    pub fn read_file(filename: &str) -> Result<StlFormat, std::io::Error> {
        let f = File::open(filename);
        let f = match f {
            Ok(file) => file,
            Err(e) => return Err(e),
        };
        let mut reader = BufReader::new(f);
        if is_binary_file(&mut reader) {
            return read_binary(&mut reader);
        } else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "text format not supported",
            ));
        }
    }
}

mod test {
    use crate::stl::read_file;
    #[test]
    fn test_bin_stl() {
        let cube = read_file("cube.stl").unwrap();
        for tri in cube.triangles {
            println!("{:?}", tri);
        }
    }
    #[test]
    fn test_text_stl() {
        let cube = read_file("cube.stl");
    }
}
