pub trait CalcVolume {
    fn volume(self) -> i32;
}

fn main() {
    // === V1 ===
    // let area = Cuboid::builder().x(1).y(2).build().volume();  // COMPILATION FAILS DUE TO MISSING "Z" ARG
    let volume1 = v1::Cuboid::builder().x(1).y(2).z(3).build().volume();

    println!("[v1] Volume: {volume1}");

    // === V2 ===
    // let volume2 = v2::CuboidBuilder::new().x(v2::XArg(1)).y(v2::YArg(2)).build().volume(); // COMPILATION FAILS DUE TO MISSING "Z" ARG
    let volume2 = v2::CuboidBuilder::new()
        .x(v2::XArg(1))
        .y(v2::YArg(2))
        .z(v2::ZArg(3))
        .build()
        .volume();

    println!("[v2] Volume: {volume2}");
}

mod v1 {

    use typed_builder::TypedBuilder;

    use crate::CalcVolume;

    #[derive(TypedBuilder)]
    pub struct Cuboid {
        x: i32,
        y: i32,
        z: i32,
    }

    impl CalcVolume for Cuboid {
        fn volume(self) -> i32 {
            self.x * self.y * self.z
        }
    }
}

// === V2 ===
mod v2 {
    use crate::CalcVolume;

    pub struct XArg(pub i32);

    pub struct YArg(pub i32);

    pub struct ZArg(pub i32);

    pub struct Incomplete;

    pub struct Cuboid {
        x: XArg,
        y: YArg,
        z: ZArg,
    }

    pub struct CuboidBuilder<X = Incomplete, Y = Incomplete, Z = Incomplete> {
        x: X,
        y: Y,
        z: Z,
    }
    impl CuboidBuilder<Incomplete, Incomplete, Incomplete> {
        pub fn new() -> Self {
            Self {
                x: Incomplete,
                y: Incomplete,
                z: Incomplete,
            }
        }
    }
    impl<Y, Z> CuboidBuilder<Incomplete, Y, Z> {
        pub fn x(self, x: XArg) -> CuboidBuilder<XArg, Y, Z> {
            CuboidBuilder::<XArg, Y, Z> {
                x: x,
                y: self.y,
                z: self.z,
            }
        }
    }
    impl<X, Z> CuboidBuilder<X, Incomplete, Z> {
        pub fn y(self, y: YArg) -> CuboidBuilder<X, YArg, Z> {
            CuboidBuilder::<X, YArg, Z> {
                x: self.x,
                y: y,
                z: self.z,
            }
        }
    }
    impl<X, Y> CuboidBuilder<X, Y, Incomplete> {
        pub fn z(self, z: ZArg) -> CuboidBuilder<X, Y, ZArg> {
            CuboidBuilder::<X, Y, ZArg> {
                x: self.x,
                y: self.y,
                z: z,
            }
        }
    }
    impl CuboidBuilder<XArg, YArg, ZArg> {
        pub fn build(self) -> Cuboid {
            Cuboid {
                x: self.x,
                y: self.y,
                z: self.z,
            }
        }
    }
    impl CalcVolume for Cuboid {
        fn volume(self) -> i32 {
            self.x.0 * self.y.0 * self.z.0
        }
    }
}
