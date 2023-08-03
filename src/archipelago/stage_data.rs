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

// Barr:
//      0290\r\n<ITJ0004:0291<END\r\n
//      0291\r\n<FLJ0360:0000<FL+0362<PRI<MSG<FAC0008Why, you!!<NOD<DNP0290<CNP1000:0063:0002<END
//

pub enum Container {
    Chest,
    LifeCapsule,
    //Fireplace,
    Sparkle,
    //Bucket,
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
    let disp_item: String = match player_name {
        Some(name) => {
            let disp_id = 5;
            format!(
                "<GIT10{:#02}<IT+{:#04}\r\n\
                <CMU0010Got {}'s\r\n ={}=.",
                disp_id, disp_id, name, item_name
            )
        }
        None => {
            let disp_id = (item_id - 0xD00000) % 100;
            match (item_id - 0xD00000) / 100 {
                1 => match item_id - 0xD00000 {
                    105 => "<EVE0030".to_string(),
                    _ => format!(
                        "<GIT00{:#02}<AM+{:#04}:0000\r\n\
                    <CMU0010Got ={}=.",
                        disp_id, disp_id, item_name
                    ),
                },
                2 => format!(
                    "<GIT1006Got a =Life Capsule=!<WAI0160<NOD<RMU<ML+000{}\r\n\
                    Max health increased by {}!",
                    disp_id, disp_id
                ),
                _ => format!(
                    "<GIT10{:#02}<IT+{:#04}\r\n\
                <CMU0010Got ={}=.",
                    disp_id, disp_id, item_name
                ),
            }
        }
    };
    let root: String = match container {
        Container::Chest => format!(
            "<FLJ{:#04}:0001<FL+{:#04}<SOU0022<CNP{:#04}:0021:0000\r\n\
            <MSGOpened the treasure chest.<NOD<CLR{}<WAI0160<NOD",
            flag_id, flag_id, event_id, disp_item
        ),
        Container::LifeCapsule => format!(
            "<SOU0022<DNP{:#04}<CMU0016\r\n\
            <MSG{}<NOD",
            event_id, disp_item
        ),
        Container::Sparkle => format!(
            "<FL+{:#04}<DNP{:#04}\r\n\
            <MSG<CLR{}<WAI0160<NOD",
            flag_id, event_id, disp_item
        ),
    };
    let mut prefix = "<PRI";
    let mut postfix = "<GIT0000<CLO<RMU<END\r\n";
    match flag_id {
        370 => {
            prefix = "<PRI<FL-0321<FL+0326<FL+0371\r\n";
        }
        390 => {
            prefix = "<PRI<DNP0300\r\n\
            <MSGArthur's grave.<NOD\r\n\
            There's writing on the\r\n\
            tombstone:<NOD<CLR<MSG<TURHere sleeps the noble Arthur,\r\n\
            true hero to the Mimigas.<NOD<CLR\r\n";
        }
        501 => {
            prefix = "<KEY";
            postfix = "\
            <CMU0000<GIT0000<CLO\r\n\
            <CNP0500:0067:0002<WAI0010<MYD0000<WAI0090\r\n\
            <MSG<FAC0000<FAC0015Oh?<NOD<FAC0000<CLO\r\n\
            <ANP0500:0013:0002<WAI0050<ANP1000:0008:0000\r\n\
            <MSG<FAC0015We meet again.<NOD\r\n\
            Do you remember me?<NOD<CLRIndeed, in the Mimiga\r\n\
            village...<NOD<CLRI hadn't noticed before,\r\n\
            but...<NOD<CLRAren't you a soldier\r\n\
            from the surface?<NOD<CLRI wasn't aware there\r\n\
            were any left.<NOD<FAC0000<CLO\r\n\
            <CNP0201:0009:0002<WAI0088<MYD0000<WAI0050<CMU0011\r\n\
            <CNP0201:0012:0002<ANP0201:0000:0002\r\n\
            <MSG<FAC0005Misery!<NOD<CLRWatch out, that one's\r\n\
            a fighter!<NOD<CLRStronger than a Mimiga,\r\n\
            even!<NOD<CLR<FAC0015You don't say.<NOD<FAC0000<CLO\r\n\
            <WAI0020<ANP0500:0025:0000<WAI0050<ANP0201:0040:0002<MSG\r\n\
            <FAC0023!!!<NOD<CLR<FAC0015Soldiers are YOUR duty.<NOD\r\n\
            <ANP0500:0020:0000<WAI0030Come back when this\r\n\
            one's in pieces.<NOD<FAC0000<CLO\r\n\
            <ANP0201:0042:0002<BOA0020<WAI0100\r\n\
            <DNP0201<BOA0010\r\n\
            <MSGGeghrooakk!<NOD<BOA0100<BSL0000<CMU0007<FL+0500<END\r\n"
        }
        550 => {
            prefix = "<KEY";
            postfix = "<CNP0302:0046:0000<GIT0000<CLO<END\r\n";
        },
        711 => {
            prefix = "<PRI<FL+0710";
            postfix = "<CLO\r\n\
            <GIT0000<RMU<CMU0004<CNP0300:0160:0000<BSL0300<END";
        }
        834 => {
            prefix = "<PRI<FLJ0828:0241<MSGThe broken robot's arm\r\n\
            clenches something.<NOD<CLRWill you take it?<YNJ0000<CLR";
            postfix = "<FL+0839<CLO<RMU<END\r\n";
        }
        920 => {
            prefix = "<KEY";
            postfix = "<GIT0000<CLO<RMU<CNP0304:0046:0000<END\r\n";
        },
        1640 => {
            postfix = "<GIT0000<CLO<RMU<MSG\r\n\
            From somewhere, a transmission...<FAO0004<NOD<TRA0018:0501:0002:0000\r\n";
        }
        _ => (),
    }
    *buf = format!("{:#04}\r\n{}{}{}\r\n", event_id, prefix, root, postfix).as_bytes().to_vec();
}
