#![allow(dead_code)]
#[allow(non_snake_case)]

pub mod Ansi
{
    //Regular text
    pub const BLK: &str = "\x1b[30m";
    pub const RED: &str = "\x1b[31m";
    pub const GRN: &str = "\x1b[32m";
    pub const YEL: &str = "\x1b[33m";
    pub const BLU: &str = "\x1b[34m";
    pub const MAG: &str = "\x1b[35m";
    pub const CYN: &str = "\x1b[36m";
    pub const WHT: &str = "\x1b[37m";

    //Regular bold text
    pub const BBLK: &str = "\x1b[30m";
    pub const BRED: &str = "\x1b[31m";
    pub const BGRN: &str = "\x1b[32m";
    pub const BYEL: &str = "\x1b[33m";
    pub const BBLU: &str = "\x1b[34m";
    pub const BMAG: &str = "\x1b[35m";
    pub const BCYN: &str = "\x1b[36m";
    pub const BWHT: &str = "\x1b[37m";

    //Regular underline text
    pub const UBLK: &str = "\x1b[4;30m";
    pub const URED: &str = "\x1b[4;31m";
    pub const UGRN: &str = "\x1b[4;32m";
    pub const UYEL: &str = "\x1b[4;33m";
    pub const UBLU: &str = "\x1b[4;34m";
    pub const UMAG: &str = "\x1b[4;35m";
    pub const UCYN: &str = "\x1b[4;36m";
    pub const UWHT: &str = "\x1b[4;37m;";

    //Regular background
    pub const BLKB: &str = "\x1b[40m";
    pub const REDB: &str = "\x1b[41m";
    pub const GRNB: &str = "\x1b[42m";
    pub const YELB: &str = "\x1b[43m";
    pub const BLUB: &str = "\x1b[44m";
    pub const MAGB: &str = "\x1b[45m";
    pub const CYNB: &str = "\x1b[46m";
    pub const WHTB: &str = "\x1b[47m";

    //High intensty background
    pub const BLKHB: &str = "\x1b[100m";
    pub const REDHB: &str = "\x1b[101m";
    pub const GRNHB: &str = "\x1b[102m";
    pub const YELHB: &str = "\x1b[103m";
    pub const BLUHB: &str = "\x1b[104m";
    pub const MAGHB: &str = "\x1b[105m";
    pub const CYNHB: &str = "\x1b[106m";
    pub const WHTHB: &str = "\x1b[107m";

    //High intensty text
    pub const HBLK: &str = "\x1b[90m";
    pub const HRED: &str = "\x1b[91m";
    pub const HGRN: &str = "\x1b[92m";
    pub const HYEL: &str = "\x1b[93m";
    pub const HBLU: &str = "\x1b[94m";
    pub const HMAG: &str = "\x1b[95m";
    pub const HCYN: &str = "\x1b[96m";
    pub const HWHT: &str = "\x1b[97m";

    //Bold high intensity text
    pub const BHBLK: &str = "\x1b[90m";
    pub const BHRED: &str = "\x1b[91m";
    pub const BHGRN: &str = "\x1b[92m";
    pub const BHYEL: &str = "\x1b[93m";
    pub const BHBLU: &str = "\x1b[94m";
    pub const BHMAG: &str = "\x1b[95m";
    pub const BHCYN: &str = "\x1b[96m";
    pub const BHWHT: &str = "\x1b[97m";

    //Reset
    pub const COLOR_END: &str = "\x1b[0m";
    pub const NONE: &str = "";
}
