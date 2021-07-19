pub type Coords = (i32, u8, i32);
pub type CommandList = Vec<SetBlock>;

pub struct SetBlock {

    coords: Coords,
    block_name: String

}

impl SetBlock {

    pub fn new(coords: Coords, block_name: &str) -> SetBlock {

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
