mod penger;
mod types;

fn main() {
    let l: types::Line;
    let ll: types::Lines;
    let p: types::Point3D;

    println!("point3d[300]  = {:?}", crate::penger::VS[300]);
    println!("line[300]  = {:?}", crate::penger::FS[300]);
}
