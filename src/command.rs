pub struct Fill {

    coords1: (i32, u8, i32),
    coords2: (i32, u8, i32),
    block_name: String

}

impl Fill {

    pub fn new(coords1: (i32, u8, i32),
           coords2: (i32, u8, i32),
           block_name: &str) -> Fill {

        Fill {
            coords1, coords2,
            block_name: String::from(block_name)
        }

    }

    pub fn to_string(&self) -> String {

        format!("/fill {} {} {} {} {} {} {}",
            self.coords1.0, self.coords1.1, self.coords1.2,
            self.coords2.0, self.coords2.1, self.coords2.2,
            self.block_name
        )

    }

}


pub struct SetBlock {

    coords: (i32, u8, i32),
    block_name: String

}

impl SetBlock {

    pub fn new(coords: (i32, u8, i32), block_name: &str) -> SetBlock {

        SetBlock {
            coords,
            block_name: String::from(block_name)
        }

    }

    pub fn to_string(&self) -> String {

        format!("/setblock {} {} {} {}",
            self.coords.0, self.coords.1, self.coords.2,
            self.block_name
        )

    }

}
