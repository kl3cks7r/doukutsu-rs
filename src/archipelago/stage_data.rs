use std::collections::HashMap;

struct TSCMod {
    event: u16,
    flag: u16,
}

impl TSCMod {
    pub fn vector(
        op: Vec<[u16]>
    ) -> TSCMod {
        Self {
            event,
            flag,
        }
    }  
}

pub fn get_event_list() -> HashMap<String, Vec<TSCMod>> {
    let mut event_list = HashMap::<String, Vec<TSCMod>>::new();
    event_list = [
        ("Almond", vec[TSCMod()]!),
        ("Cave", "bear"),
        ("Cemet", "cat"),
    ].iter().copied().collect();
}

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
