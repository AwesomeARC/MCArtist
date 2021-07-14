pub mod Command {

    struct Fill {

        coords1: (i32, u8, i32),
        coords2: (i32, u8, i32),
        block_name: str

    }

    impl Fill {

        fn new(coords1: (u32, u8, u32), coords2: (u32, u8, u32), block_name: str) {

            Fill { coords1, coords2, block_name }

        }

        fn to_string(&self) {

            format!("/fill {} {} {} {} {} {} {}",
                coords1.0, coords1.1, coords1.2,
                coords2.0, coords2.1, coords2.2,
                block_name
            )

        }

    }


    struct SetBlock {

        coords: (i32, u8, i32),
        block_name: str

    }

    impl SetBlock {

        fn new(coords: (u32, u8, u32), block_name: str) {

            SetBlock { coords, block_name }

        }

        fn to_string(&self) {

            format!("/setblock {} {} {} {}",
                coords.0, coords.1, coords.2,
                block_name
            )

        }

    }

}
