use glium;
use std::ops::*;

/*
    Vertex
*/

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: Vector3<f32>,
    pub texture_coordinate: Vector2<f32>,
}

implement_vertex!(Vertex, position, texture_coordinate);

/*
    Vector2
*/

#[derive(Copy, Clone, Debug)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

unsafe impl glium::vertex::Attribute for Vector2<f32> {
    fn get_type() -> glium::vertex::AttributeType {
        glium::vertex::AttributeType::F32F32
    }
}

unsafe impl glium::vertex::Attribute for Vector2<f64> {
    fn get_type() -> glium::vertex::AttributeType {
        glium::vertex::AttributeType::F64F64
    }
}

unsafe impl glium::vertex::Attribute for Vector2<i32> {
    fn get_type() -> glium::vertex::AttributeType {
        glium::vertex::AttributeType::I32I32
    }
}

impl<T: Add> Add for Vector2<T> {
    type Output = Vector2<T::Output>;

    fn add(self, other: Self) -> Vector2<T::Output> {
        Vector2{
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: Sub> Sub for Vector2<T> {
    type Output = Vector2<T::Output>;

    fn sub(self, other: Self) -> Vector2<T::Output> {
        Vector2{
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

/*
    Vector3
*/

#[derive(Copy, Clone, Debug)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

unsafe impl glium::vertex::Attribute for Vector3<f32> {
    fn get_type() -> glium::vertex::AttributeType {
        glium::vertex::AttributeType::F32F32F32
    }
}

unsafe impl glium::vertex::Attribute for Vector3<f64> {
    fn get_type() -> glium::vertex::AttributeType {
        glium::vertex::AttributeType::F64F64F64
    }
}

unsafe impl glium::vertex::Attribute for Vector3<i32> {
    fn get_type() -> glium::vertex::AttributeType {
        glium::vertex::AttributeType::I32I32I32
    }
}

impl glium::uniforms::AsUniformValue for Vector3<f32> {
    fn as_uniform_value(&self) -> glium::uniforms::UniformValue {
        glium::uniforms::UniformValue::Vec3([self.x, self.y, self.z])
    }
}

impl<T: Add> Add for Vector3<T> {
    type Output = Vector3<T::Output>;

    fn add(self, other: Self) -> Vector3<T::Output> {
        Vector3{
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T: Sub> Sub for Vector3<T> {
    type Output = Vector3<T::Output>;

    fn sub(self, other: Self) -> Vector3<T::Output> {
        Vector3{
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}