use std::num::{ParseFloatError, ParseIntError};

use cgmath::Vector3;

#[derive(Debug)]
pub struct Obj<V> {
    pub vertices: Vec<V>,
    pub indices: Vec<u32>,
}

#[derive(Debug)]
pub enum Error {
    MissingField,
    MissingSubfield,
    ParseIntError(ParseIntError),
    ParseFloatError(ParseFloatError),
}

impl From<ParseIntError> for Error {
    fn from(other: ParseIntError) -> Self {
        Error::ParseIntError(other)
    }
}

impl From<ParseFloatError> for Error {
    fn from(other: ParseFloatError) -> Self {
        Error::ParseFloatError(other)
    }
}

impl<V> Obj<V> where V: From<Vector3<f32>> {
    pub fn read(source: &str) -> Result<Self, Error> {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        for line in source.lines() {
            let mut fields = line.split_whitespace();
            match fields.next() {
                Some("v") => Self::read_vertex(&mut vertices, fields)?,
                Some("f") => Self::read_face(&mut indices, fields)?,
                _ => (),
            }
        }
        Ok(Obj{vertices, indices})
    }

    fn read_vertex<'a, I>(vertices: &mut Vec<V>, mut fields: I) -> Result<(), Error>
        where I: Iterator<Item=&'a str> {
        let x = fields.next().ok_or(Error::MissingField)?.parse::<f32>()?;
        let y = fields.next().ok_or(Error::MissingField)?.parse::<f32>()?;
        let z = fields.next().ok_or(Error::MissingField)?.parse::<f32>()?;
        let vertex = Vector3::new(x, y, z);
        vertices.push(V::from(vertex));
        Ok(())
    }

    fn read_face<'a, I>(indices: &mut Vec<u32>, fields: I) -> Result<(), Error>
        where I: Iterator<Item=&'a str> {
        for field in fields {
            let mut subfields = field.split("/");
            let index_subfield = subfields.next().ok_or(Error::MissingSubfield)?;
            let index = index_subfield.parse::<u32>()?;
            indices.push(index - 1);
        }
        Ok(())
    }
}
