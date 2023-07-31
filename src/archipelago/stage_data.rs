pub const ALL_LOCATIONS: &[i32] = &[
    101 + 0xD00000,
    102 + 0xD00000,
    140 + 0xD00000,
    200 + 0xD00000,
    218 + 0xD00000,
    220 + 0xD00000,
    270 + 0xD00000,
    271 + 0xD00000,
    304 + 0xD00000,
    322 + 0xD00000,
    390 + 0xD00000,
    412 + 0xD00000,
    540 + 0xD00000,
    642 + 0xD00000,
    705 + 0xD00000,
    834 + 0xD00000,
    880 + 0xD00000,
    920 + 0xD00000,
    1043 + 0xD00000,
    1530 + 0xD00000,
    1640 + 0xD00000,
    1700 + 0xD00000,
];

pub enum Container {
    Chest,
    // LifeCapsule,
    // //Fireplace,
    // Sparkle,
    // //Bucket,
}

pub fn generate_tsc(
    buf: &mut Vec<u8>,
    flag_id: u16,
    event_id: u16,
    container: Container,
    item_id: i32,
    item_name: String,
    player_name: Option<String>,
) {
    let disp_id: i32;
    let disp_item: String = match player_name {
        Some(name) => {
            disp_id = 5;
            format!("{}'s ={}=", name, item_name)
        }
        None => {
            disp_id = item_id - 0xD00000;
            format!("={}=", item_name)
        }
    };
    let root: String = match container {
        Container::Chest => format!(
            "<PRI<FLJ{:#04}:0001<FL+{:#04}<SOU0022<CNP{:#04}:0021:0000\r\n\
            <MSGOpened the treasure chest.<NOD\
            <GIT10{:#02}<IT+{:#04}<CLR\r\n\
            <CMU0010Got {}.<WAI0160<NOD<RMU",
            flag_id, flag_id, event_id, disp_id, disp_id, disp_item
        ),
        // Container::LifeCapsule => format!(""),
        // Container::Sparkle => format!(""),
    };
    *buf = match flag_id {
        _ => format!("{:#04}\r\n{}<END\r\n\r\n", event_id, root).as_bytes().to_vec(),
    }
}
