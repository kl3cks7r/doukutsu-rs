use std::collections::HashMap;

// struct TSCMod {
//     event: u16,
//     flag: u16,
// }

// impl TSCMod {
//     pub fn vector(
//         op: Vec<[u16]>
//     ) -> TSCMod {
//         Self {
//             event,
//             flag,
//         }
//     }  
// }

// pub fn get_event_list() -> HashMap<String, Vec<TSCMod>> {
//     let mut event_list = HashMap::<String, Vec<TSCMod>>::new();
//     event_list = [
//         ("Almond", "apple"),
//         ("Cave", "bear"),
//         ("Cemet", "cat"),
//     ].iter().copied().collect();
// }

pub const ALL_LOCATIONS: &[i32] = &[
    101 + 0xD00000,
    102 + 0xD00000,
    140 + 0xD00000,
    200 + 0xD00000,
    218 + 0xD00000,
    220 + 0xD00000,
    270 + 0xD00000,
    271 + 0xD00000,
    //279 + 0xD00000,
    304 + 0xD00000,
    322 + 0xD00000,
    370 + 0xD00000,
    390 + 0xD00000,
    412 + 0xD00000,
    501 + 0xD00000,
    540 + 0xD00000,
    550 + 0xD00000,
    581 + 0xD00000,
    642 + 0xD00000,
    705 + 0xD00000,
    711 + 0xD00000,
    834 + 0xD00000,
    880 + 0xD00000,
    920 + 0xD00000,
    1043 + 0xD00000,
    1530 + 0xD00000,
    1640 + 0xD00000,
    1700 + 0xD00000,
];

pub fn generate_tsc(
    buf: &mut Vec<u8>,
    flag_id: u16,
    event_id: u16,
    item_id: i32,
    item_name: String,
    player_name: Option<String>,
) {
    let command: String = match player_name {
        Some(name) => {
            let disp_id = 5;
            format!(
                "<MSG<CLR<GIT1{:#03}\r\n\
                <CMU0010Got {}'s\r\n ={}=.\
                <WAI0160<NOD",
                disp_id, name, item_name
            )
        }
        None => {
            format!("<EVE{:#04}", item_id)
        }
    };
    *buf = format!("{:#04}\r\n<FL+{:#04}\r\n{}\r\n", event_id, flag_id, command).as_bytes().to_vec();
}
