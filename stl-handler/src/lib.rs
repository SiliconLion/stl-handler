use std::io;
use std::fs::File;
use std::io::prelude::*;



///bytes should be exactly 4 in length
fn bytes_to_f32(bytes: &[u8]) -> f32 {
    let mut buffer = [0; 4];
    //copies each element from bytes, and shifts it such that it
    //will line up into a u32 when all added together
    for i in 0..4 {
        buffer[i] = bytes[i] as u32;
        let amount_to_shift = (4 - (i + 1)) * 8;
        buffer[i] <<= amount_to_shift;
    }

    let sum = buffer.into_iter().sum();
    f32::from_bits(sum)
} 

fn bytes_to_u16(bytes: &[u8]) -> u16 {
    let mut first = bytes[0] as u16;
    first <<= 8; 
    let sum = first + bytes[1] as u16;
    sum
}

///bytes should be exactly 4 in length
fn bytes_to_u32(bytes: &[u8]) -> u32 {
    let mut buffer = [0; 4];
    //copies each element from bytes, and shifts it such that it
    //will line up into a u32 when all added together
    for i in 0..4 {
        buffer[i] = bytes[i] as u32;
        let amount_to_shift = (4 - (i + 1)) * 8;
        buffer[i] <<= amount_to_shift;
    }

    let sum = buffer.into_iter().sum();
    sum
} 




fn f32_to_bytes(num: f32) -> [u8; 4] {
    //num is shadowed into a u32 in such a way as to preserve the bit information 
    //this alows bitwise operations
    let num = num.to_bits();
    let mut buffer = [0; 4];
    //this will be used with the '&' bitwise operator to select 8 bits a time
    let mut selector: u32 = 0x000000FF;
    print!("{}", selector);
    for i in 0..4 {
        //this is used to know how many bits to shift in order to seperate
        //num into u8's , and how many bit to shit the buffer as a u32 to be able to cast it
        //to a u8 
        let amount_to_shift = (3 - i) * 8;
        selector <<= amount_to_shift;
        buffer[i] = num & selector;
        buffer[i] >>= amount_to_shift; 
        //resets selector so that the above shifting is offset from the initial value
        selector = 0x000000FF;
    }
    let mut output = [0; 4];
    for (i, n) in buffer.iter().enumerate() {
        output[i] = *n as u8;
    }
    output
}

fn bytes_to_string(bytes: &[u8]) -> String {
    let mut chars = Vec::new();
    for b in bytes.iter() {
        chars.push(*b as char);
    }
    let output: String = chars.iter().collect();
    output
}

//represents any point in space, or any vector centered at the orign
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vertex {
    //x,y,z
    coords: [f32; 3]
}


impl Vertex {
    ///bytes should be exactly 12 in length
    fn from_bytes(bytes: &[u8]) -> Vertex {
        let mut floats = [0 as f32; 3];
        //to create 3 floats
        for i in 0..3 {
            floats[i] =  bytes_to_f32( &bytes[(i*4)..=( (i*4) + 3) ] ); 
        }
        Vertex {coords: floats}
    }

    fn empty_vert() -> Vertex {
        Vertex { coords: [0.0, 0.0, 0.0]}
    }

    pub fn x(&self) -> f32 {
        self.coords[0]
    }

    pub fn y(&self) -> f32 {
        self.coords[1]
    }

    pub fn z(&self) -> f32 {
        self.coords[2]
    }
}


#[derive(Debug, Clone)]
pub struct Triangle {
    normal: Vertex,
    verts: [Vertex; 3],
    attribute: u16
}

impl Triangle {
    //bytes should be a slice 50 in length
    fn from_bytes(bytes: &[u8] ) -> Triangle {
        if bytes.len() != 50 { print!("triangle::from_bytes error. bytes.len() != 50"); }
        let normal = Vertex::from_bytes(&bytes[0..12]);
        let attribute = bytes_to_u16(&bytes[48..50]);
        let mut verts = [Vertex::empty_vert(); 3];
        for i in 0..3 {
            //12 bytes at a time, skipping the first 12, as those are the normal
            verts[i] = Vertex::from_bytes( &bytes[ ((i + 1) * 12)..((i + 2) * 12)] );
        }

        
        Triangle {normal, verts, attribute}
    }
    
}

pub fn decode_stl(path: String) -> Result< (String, u32, Vec<Triangle>), io::Error > {
    let mut file = File::open(path)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;

    //both defined in the STL standard
    let header = bytes_to_string( &contents[0..80] ); 
    println!("bytes 0..85: {:?}", &contents[0..85]);
    // println!("bytes 80..84: {:?}", &contents[80..84]);
   // let num_of_tris = bytes_to_u32( &contents[80..84] );
    
    println!("header: {}", header);
    

    //the remaining bytes consist of just triangles
    let remaining_bytes = &contents[84..];
    let num_of_tris = remaining_bytes.len() as u32/ 50;
    println!("num of tris: {}", num_of_tris);
    let mut triangles = Vec::new();
    

    //each triangle is 50 bytes. the len() % 50 != 0, then somewhere the data is mangled
    if remaining_bytes.len() % 50 != 0 {
        print!("remaining bytes % 50 != 0");
        let e = io::Error::new(io::ErrorKind::InvalidData, "Triangle data is mangled");
        return Err(e);
    }
    
    //takes slices of 50 bytes at a time, and turns them into triangles
    for i in 0..(num_of_tris  as usize) {
        triangles.push( 
            Triangle::from_bytes( &remaining_bytes[ (i * 50)..((i + 1) * 50) ] )
        );
    }

    Ok( (header, num_of_tris, triangles) )
}






#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bytes_to_f32_conversion() {
        //the number 0.15625 (as f32) split into bytes
        let stream = [0b00111110, 0b00100000, 0b00000000, 0b00000000];
                        
        let test_f32 = bytes_to_f32(&stream);
        assert_eq!(test_f32, 0.15625);
    }

     #[test]
    fn bytes_to_u16_conversion() {
        //the number 5000 in binary, split into bytes
        let stream = [0b0010011,0b10001000];
                        
        let test_u16 = bytes_to_u16(&stream);
        assert_eq!(test_u16, 5000);
    }

    #[test]
    fn bytes_to_u32_conversion() {
        //the number 5000 in binary, split into bytes
        let stream = [0b00000000,0b00000000,0b0010011,0b10001000];
                        
        let test_u32 = bytes_to_u32(&stream);
        assert_eq!(test_u32, 5000);
    }

    #[test]
    fn f32_to_bytes_conversion() {
        //the numbers they are named, split into bytes
        let ten = [0b01000001, 0b00100000, 0b00000000, 0b00000000];
        let hundred  = [0b01000010, 0b11001000, 0b00000000, 0b00000000];
        let twenty  = [0b01000001, 0b10100000, 0b00000000, 0b00000000]; 

        assert_eq!( ten, f32_to_bytes(10.0) );
        assert_eq!( hundred, f32_to_bytes(100.0) );
        assert_eq!( twenty, f32_to_bytes(20.0) );
    }

    #[test]
    fn bytes_to_string_conversion() {
        //ascci code for "hello"
        let hello = [104, 101, 108, 108, 111];
        let test = bytes_to_string(&hello);
        assert_eq!(String::from("hello"), test);
    }

    #[test]
    fn bytes_to_vert_conversion() {
        //the number 0.15625 split into bytes, repeated 3 times
        let stream = [0b00111110, 0b00100000, 0b00000000, 0b00000000,
                      0b00111110, 0b00100000, 0b00000000, 0b00000000,
                      0b00111110, 0b00100000, 0b00000000, 0b00000000];
        let vert = Vertex::from_bytes(&stream);
        assert_eq!(vert.coords, [0.15625,0.15625,0.15625]);
    }

    #[test]
    fn bytes_to_tri_conversion() {
        let ten = f32_to_bytes(10.0);
        let hundred  = f32_to_bytes(100.0);
        let twenty  = f32_to_bytes(20.0);

        //the normal and vertex bytes, such that each will be [10.0, 100.0, 20.0]
        let mut stream = Vec::new();
        for i in 0..4 {
            for t in ten.iter() {
                stream.push(*t);
            }
            for h in hundred.iter() {
                stream.push(*h);
            }
            for t in twenty.iter() {
                stream.push(*t);
            }
        }
        //the last two bytes (the attribute data)
        stream.push(0);
        stream.push(0);

        print!("{:?}", stream);
        let tri = Triangle::from_bytes( &stream[..] );

        //checks normal
        assert_eq!(tri.normal.coords, [10.0, 100.0, 20.0]);
        //checks each vertex
        assert_eq!(tri.verts[0].coords,[10.0, 100.0, 20.0] );
        assert_eq!(tri.verts[1].coords,[10.0, 100.0, 20.0] );
        assert_eq!(tri.verts[2].coords,[10.0, 100.0, 20.0] );
        //checks attribute 
        assert_eq!(tri.attribute, 0);
    }

    #[test]
    fn decodeing_stl() {
       let path = String::from("assets/meshmixed dragon bust.STL");
       let output = decode_stl(path);

       //bad way of doing this, and i should defentetly change it, but kinda fine for a test. 
       let (header, num_of_tris, triangles) = match output {
           Err(e) => (String::from("error"), 0, vec![]), 
           Ok( (h, n, tris) )=> (h, n, tris)
       };

       assert_eq!(num_of_tris, 411488);
       assert_eq!(num_of_tris as usize, triangles.len());
    }


}