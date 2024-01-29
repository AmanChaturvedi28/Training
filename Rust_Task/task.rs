struct Area{
    length: f32,
    breath: f32,
    radius: f32
}

impl Area {
    fn area_circle(&self) {
        println!("Area of Circle: {}",3.14 * self.radius * self.radius)
    }

    fn area_square(len:f32) {
        println!("Area of Square: {}",len * len)
    }

    fn area_rectangle(&self) {
        println!("Area of Circle: {}",self.length * self.breath)
    }
}

fn main() {
    let values = Area {
        length: 10.0,
        breath: 12.0,
        radius: 6.0
    };
    values.area_circle();
    Area::area_square(values.length);
    values.area_rectangle();
}

//refer implementation.rs for self explanation