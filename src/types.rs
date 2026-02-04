// Copyright (C) 2026  Antonio-M. Corbi Bellot
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

pub type Line = [u32; 3];

// pub type Lines = Vec<Line>;
// Cambiamos el tipo a una referencia de array o array fijo
pub type Lines = &'static [Line];

#[derive(Debug, Copy, Clone)]
pub struct Point2D {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Copy, Clone)]
pub struct Point3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub type Points = &'static [Point3D];

// Definimos los ejes posibles
pub enum Axe {
    X,
    Y,
    Z,
}

impl Point3D {
    pub fn translate_z(&self, dz: f32) -> Self {
        Point3D {
            x: self.x,
            y: self.y,
            z: self.z + dz,
        }
    }

    pub fn project(&self) -> Point2D {
        Point2D {
            x: self.x / self.z,
            y: self.y / self.z,
        }
    }

    // Función que recibe el punto, el ángulo y el eje de rotación
    pub fn rotate(&self, angle: f32, axe: Axe) -> Point3D {
        let angle = angle.to_radians();
        let Point3D { x, y, z } = *self;
        let cos_a = angle.cos();
        let sin_a = angle.sin();

        match axe {
            Axe::X => {
                // X se queda igual
                let y = y * cos_a - z * sin_a;
                let z = y * sin_a + z * cos_a;
                Point3D { x, y, z }
            }
            Axe::Y => {
                // Y se queda igual
                let x = x * cos_a + z * sin_a;
                let z = -x * sin_a + z * cos_a;
                Point3D { x, y, z }
            }
            Axe::Z => {
                // Z se queda igual
                let x = x * cos_a - y * sin_a;
                let y = x * sin_a + y * cos_a;
                Point3D { x, y, z }
            }
        }
    }
}
