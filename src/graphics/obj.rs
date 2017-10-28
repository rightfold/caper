use std::collections::HashMap;
use std::num::{ParseFloatError, ParseIntError};
use std::str::FromStr;

use cgmath::{Matrix, Matrix4, Vector3};

#[derive(Debug)]
pub enum Error {
    MissingField,
    MissingSubfield,
    UnknownVertexPosition,
    UnknownVertexNormal,
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

pub type RawVertexPosition = Vector3<f32>;
pub type RawVertexNormal   = Vector3<f32>;
pub type RawFace           = [(u32, u32); 3];

#[derive(Debug)]
pub struct Obj<P, N> {
    pub vertex_positions: Vec<P>,
    pub vertex_normals: Vec<N>,
    pub vertex_indices: Vec<u32>,
    pub metas: HashMap<String, Meta>,
}

#[derive(Debug)]
pub struct Meta {
    pub transform: Matrix4<f32>,
}

impl<P, N> Obj<P, N> where P: From<Vector3<f32>>, N: From<Vector3<f32>> {
    pub fn new(raw_vertex_positions: &[RawVertexPosition],
               raw_vertex_normals: &[RawVertexNormal], raw_faces: &[RawFace],
               metas: HashMap<String, Meta>) -> Result<Self, Error> {
        // TODO(rightfold): Deduplicate the vertices.
        let mut vertex_positions = Vec::new();
        let mut vertex_normals = Vec::new();
        let mut vertex_indices = Vec::new();

        for raw_face in raw_faces {
            for &(vertex_position_index, vertex_normal_index) in raw_face {
                let vertex_position =
                    raw_vertex_positions.get(vertex_position_index as usize)
                    .map(|&v| P::from(v))
                    .ok_or(Error::UnknownVertexPosition)?;
                let vertex_normal =
                    raw_vertex_normals.get(vertex_normal_index as usize)
                    .map(|&v| N::from(v))
                    .ok_or(Error::UnknownVertexNormal)?;
                vertex_positions.push(vertex_position);
                vertex_normals.push(vertex_normal);
                vertex_indices.push(vertex_positions.len() as u32 - 1);
            }
        }

        Ok(Obj{vertex_positions, vertex_normals, vertex_indices, metas})
    }

    pub fn read(source: &str) -> Result<Self, Error> {
        let mut raw_vertex_positions = Vec::new();
        let mut raw_vertex_normals = Vec::new();
        let mut raw_faces = Vec::new();
        let mut metas = HashMap::new();

        for line in source.lines() {
            let mut fields = line.split(' ');
            let type_field = fields.next();
            match type_field {
                Some("v") => Self::read_vertex_position(&mut raw_vertex_positions, fields)?,
                Some("vn") => Self::read_vertex_normal(&mut raw_vertex_normals, fields)?,
                Some("f") => Self::read_face(&mut raw_faces, fields)?,
                Some("meta") => {
                    let mut fields = line.splitn(18, ' ');
                    fields.next();
                    Self::read_meta(&mut metas, fields)?
                },
                _ => (),
            }
        }

        Self::new(&raw_vertex_positions, &raw_vertex_normals, &raw_faces, metas)
    }

    fn read_vertex_position<'a, I>(raw_vertex_positions: &mut Vec<RawVertexPosition>,
                                   mut fields: I) -> Result<(), Error>
        where I: Iterator<Item=&'a str> {
        let mut read_field = || -> Result<f32, Error> {
            let field = fields.next().ok_or(Error::MissingField)?;
            f32::from_str(field).map_err(Error::from)
        };
        let x = read_field()?;
        let y = read_field()?;
        let z = read_field()?;
        raw_vertex_positions.push(Vector3::new(x, y, z));
        Ok(())
    }

    fn read_vertex_normal<'a, I>(raw_vertex_normals: &mut Vec<RawVertexNormal>,
                                 fields: I) -> Result<(), Error>
        where I: Iterator<Item=&'a str> {
        // Vertex positions and vertex normals have the same syntax.
        Self::read_vertex_position(raw_vertex_normals, fields)
    }

    fn read_face<'a, I>(raw_faces: &mut Vec<RawFace>,
                        mut fields: I) -> Result<(), Error>
        where I: Iterator<Item=&'a str> {
        let mut read_field = || -> Result<(u32, u32), Error> {
            let field = fields.next().ok_or(Error::MissingField)?;
            let mut subfields = field.split("/");
            let mut read_subfield = || {
                let subfield = subfields.next().ok_or(Error::MissingSubfield)?;
                u32::from_str(subfield).map(|x| x - 1).map_err(Error::from)
            };
            let position = read_subfield()?;
            let _        = read_subfield();
            let normal   = read_subfield()?;
            Ok((position, normal))
        };
        let a = read_field()?;
        let b = read_field()?;
        let c = read_field()?;
        raw_faces.push([a, b, c]);
        Ok(())
    }

    fn read_meta<'a, I>(metas: &mut HashMap<String, Meta>,
                        mut fields: I) -> Result<(), Error>
        where I: Iterator<Item=&'a str> {
        let transform = {
            let mut read_field = || -> Result<f32, Error> {
                let field = fields.next().ok_or(Error::MissingField)?;
                f32::from_str(field).map_err(Error::from)
            };
            let mut transform_array = [0.0; 16];
            for transform_component in transform_array.iter_mut() {
                *transform_component = read_field()?;
            }
            Matrix4::new(
                transform_array[ 0], transform_array[ 1], transform_array[ 2], transform_array[ 3],
                transform_array[ 4], transform_array[ 5], transform_array[ 6], transform_array[ 7],
                transform_array[ 8], transform_array[ 9], transform_array[10], transform_array[11],
                transform_array[12], transform_array[13], transform_array[14], transform_array[15],
            ).transpose()
        };

        let name = fields.next().ok_or(Error::MissingField)?.to_string();
        let meta = Meta{transform};
        metas.insert(name, meta);

        Ok(())
    }
}
