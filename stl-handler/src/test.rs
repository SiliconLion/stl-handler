
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
