#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Graphic MMU configuration register"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x04 - Graphic MMU status register"]
    pub sr: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x08 - Graphic MMU flag clear register"]
    pub fcr: crate::Reg<fcr::FCR_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - Graphic MMU default value register"]
    pub dvr: crate::Reg<dvr::DVR_SPEC>,
    _reserved4: [u8; 0x0c],
    #[doc = "0x20 - Graphic MMU buffer 0 configuration register"]
    pub b0cr: crate::Reg<b0cr::B0CR_SPEC>,
    #[doc = "0x24 - Graphic MMU buffer 1 configuration register"]
    pub b1cr: crate::Reg<b1cr::B1CR_SPEC>,
    #[doc = "0x28 - Graphic MMU buffer 2 configuration register"]
    pub b2cr: crate::Reg<b2cr::B2CR_SPEC>,
    #[doc = "0x2c - Graphic MMU buffer 3 configuration register"]
    pub b3cr: crate::Reg<b3cr::B3CR_SPEC>,
    _reserved8: [u8; 0x0fc4],
    #[doc = "0xff4 - Graphic MMU version register"]
    pub verr: crate::Reg<verr::VERR_SPEC>,
    #[doc = "0xff8 - Graphic MMU identification register"]
    pub ipidr: crate::Reg<ipidr::IPIDR_SPEC>,
    #[doc = "0xffc - Graphic MMU size identification register"]
    pub sidr: crate::Reg<sidr::SIDR_SPEC>,
    #[doc = "0x1000 - Graphic MMU LUT entry 0 low"]
    pub lut0l: crate::Reg<lut0l::LUT0L_SPEC>,
    #[doc = "0x1004 - Graphic MMU LUT entry 0 high"]
    pub lut0h: crate::Reg<lut0h::LUT0H_SPEC>,
    #[doc = "0x1008 - Graphic MMU LUT entry 1 low"]
    pub lut1l: crate::Reg<lut1l::LUT1L_SPEC>,
    #[doc = "0x100c - Graphic MMU LUT entry 1 high"]
    pub lut1h: crate::Reg<lut1h::LUT1H_SPEC>,
    #[doc = "0x1010 - Graphic MMU LUT entry 2 low"]
    pub lut2l: crate::Reg<lut2l::LUT2L_SPEC>,
    #[doc = "0x1014 - Graphic MMU LUT entry 2 high"]
    pub lut2h: crate::Reg<lut2h::LUT2H_SPEC>,
    #[doc = "0x1018 - Graphic MMU LUT entry 3 low"]
    pub lut3l: crate::Reg<lut3l::LUT3L_SPEC>,
    #[doc = "0x101c - Graphic MMU LUT entry 3 high"]
    pub lut3h: crate::Reg<lut3h::LUT3H_SPEC>,
    #[doc = "0x1020 - Graphic MMU LUT entry 4 low"]
    pub lut4l: crate::Reg<lut4l::LUT4L_SPEC>,
    #[doc = "0x1024 - Graphic MMU LUT entry 4 high"]
    pub lut4h: crate::Reg<lut4h::LUT4H_SPEC>,
    #[doc = "0x1028 - Graphic MMU LUT entry 5 low"]
    pub lut5l: crate::Reg<lut5l::LUT5L_SPEC>,
    #[doc = "0x102c - Graphic MMU LUT entry 5 high"]
    pub lut5h: crate::Reg<lut5h::LUT5H_SPEC>,
    #[doc = "0x1030 - Graphic MMU LUT entry 6 low"]
    pub lut6l: crate::Reg<lut6l::LUT6L_SPEC>,
    #[doc = "0x1034 - Graphic MMU LUT entry 6 high"]
    pub lut6h: crate::Reg<lut6h::LUT6H_SPEC>,
    #[doc = "0x1038 - Graphic MMU LUT entry 7 low"]
    pub lut7l: crate::Reg<lut7l::LUT7L_SPEC>,
    #[doc = "0x103c - Graphic MMU LUT entry 7 high"]
    pub lut7h: crate::Reg<lut7h::LUT7H_SPEC>,
    #[doc = "0x1040 - Graphic MMU LUT entry 8 low"]
    pub lut8l: crate::Reg<lut8l::LUT8L_SPEC>,
    #[doc = "0x1044 - Graphic MMU LUT entry 8 high"]
    pub lut8h: crate::Reg<lut8h::LUT8H_SPEC>,
    #[doc = "0x1048 - Graphic MMU LUT entry 9 low"]
    pub lut9l: crate::Reg<lut9l::LUT9L_SPEC>,
    #[doc = "0x104c - Graphic MMU LUT entry 9 high"]
    pub lut9h: crate::Reg<lut9h::LUT9H_SPEC>,
    #[doc = "0x1050 - Graphic MMU LUT entry 10 low"]
    pub lut10l: crate::Reg<lut10l::LUT10L_SPEC>,
    #[doc = "0x1054 - Graphic MMU LUT entry 10 high"]
    pub lut10h: crate::Reg<lut10h::LUT10H_SPEC>,
    #[doc = "0x1058 - Graphic MMU LUT entry 11 low"]
    pub lut11l: crate::Reg<lut11l::LUT11L_SPEC>,
    #[doc = "0x105c - Graphic MMU LUT entry 11 high"]
    pub lut11h: crate::Reg<lut11h::LUT11H_SPEC>,
    #[doc = "0x1060 - Graphic MMU LUT entry 12 low"]
    pub lut12l: crate::Reg<lut12l::LUT12L_SPEC>,
    #[doc = "0x1064 - Graphic MMU LUT entry 12 high"]
    pub lut12h: crate::Reg<lut12h::LUT12H_SPEC>,
    #[doc = "0x1068 - Graphic MMU LUT entry 13 low"]
    pub lut13l: crate::Reg<lut13l::LUT13L_SPEC>,
    #[doc = "0x106c - Graphic MMU LUT entry 13 high"]
    pub lut13h: crate::Reg<lut13h::LUT13H_SPEC>,
    #[doc = "0x1070 - Graphic MMU LUT entry 14 low"]
    pub lut14l: crate::Reg<lut14l::LUT14L_SPEC>,
    #[doc = "0x1074 - Graphic MMU LUT entry 14 high"]
    pub lut14h: crate::Reg<lut14h::LUT14H_SPEC>,
    #[doc = "0x1078 - Graphic MMU LUT entry 15 low"]
    pub lut15l: crate::Reg<lut15l::LUT15L_SPEC>,
    #[doc = "0x107c - Graphic MMU LUT entry 15 high"]
    pub lut15h: crate::Reg<lut15h::LUT15H_SPEC>,
    #[doc = "0x1080 - Graphic MMU LUT entry 16 low"]
    pub lut16l: crate::Reg<lut16l::LUT16L_SPEC>,
    #[doc = "0x1084 - Graphic MMU LUT entry 16 high"]
    pub lut16h: crate::Reg<lut16h::LUT16H_SPEC>,
    #[doc = "0x1088 - Graphic MMU LUT entry 17 low"]
    pub lut17l: crate::Reg<lut17l::LUT17L_SPEC>,
    #[doc = "0x108c - Graphic MMU LUT entry 17 high"]
    pub lut17h: crate::Reg<lut17h::LUT17H_SPEC>,
    #[doc = "0x1090 - Graphic MMU LUT entry 18 low"]
    pub lut18l: crate::Reg<lut18l::LUT18L_SPEC>,
    #[doc = "0x1094 - Graphic MMU LUT entry 18 high"]
    pub lut18h: crate::Reg<lut18h::LUT18H_SPEC>,
    #[doc = "0x1098 - Graphic MMU LUT entry 19 low"]
    pub lut19l: crate::Reg<lut19l::LUT19L_SPEC>,
    #[doc = "0x109c - Graphic MMU LUT entry 19 high"]
    pub lut19h: crate::Reg<lut19h::LUT19H_SPEC>,
    #[doc = "0x10a0 - Graphic MMU LUT entry 20 low"]
    pub lut20l: crate::Reg<lut20l::LUT20L_SPEC>,
    #[doc = "0x10a4 - Graphic MMU LUT entry 20 high"]
    pub lut20h: crate::Reg<lut20h::LUT20H_SPEC>,
    #[doc = "0x10a8 - Graphic MMU LUT entry 21 low"]
    pub lut21l: crate::Reg<lut21l::LUT21L_SPEC>,
    #[doc = "0x10ac - Graphic MMU LUT entry 21 high"]
    pub lut21h: crate::Reg<lut21h::LUT21H_SPEC>,
    #[doc = "0x10b0 - Graphic MMU LUT entry 22 low"]
    pub lut22l: crate::Reg<lut22l::LUT22L_SPEC>,
    #[doc = "0x10b4 - Graphic MMU LUT entry 22 high"]
    pub lut22h: crate::Reg<lut22h::LUT22H_SPEC>,
    #[doc = "0x10b8 - Graphic MMU LUT entry 23 low"]
    pub lut23l: crate::Reg<lut23l::LUT23L_SPEC>,
    #[doc = "0x10bc - Graphic MMU LUT entry 23 high"]
    pub lut23h: crate::Reg<lut23h::LUT23H_SPEC>,
    #[doc = "0x10c0 - Graphic MMU LUT entry 24 low"]
    pub lut24l: crate::Reg<lut24l::LUT24L_SPEC>,
    #[doc = "0x10c4 - Graphic MMU LUT entry 24 high"]
    pub lut24h: crate::Reg<lut24h::LUT24H_SPEC>,
    #[doc = "0x10c8 - Graphic MMU LUT entry 25 low"]
    pub lut25l: crate::Reg<lut25l::LUT25L_SPEC>,
    #[doc = "0x10cc - Graphic MMU LUT entry 25 high"]
    pub lut25h: crate::Reg<lut25h::LUT25H_SPEC>,
    #[doc = "0x10d0 - Graphic MMU LUT entry 26 low"]
    pub lut26l: crate::Reg<lut26l::LUT26L_SPEC>,
    #[doc = "0x10d4 - Graphic MMU LUT entry 26 high"]
    pub lut26h: crate::Reg<lut26h::LUT26H_SPEC>,
    #[doc = "0x10d8 - Graphic MMU LUT entry 27 low"]
    pub lut27l: crate::Reg<lut27l::LUT27L_SPEC>,
    #[doc = "0x10dc - Graphic MMU LUT entry 27 high"]
    pub lut27h: crate::Reg<lut27h::LUT27H_SPEC>,
    #[doc = "0x10e0 - Graphic MMU LUT entry 28 low"]
    pub lut28l: crate::Reg<lut28l::LUT28L_SPEC>,
    #[doc = "0x10e4 - Graphic MMU LUT entry 28 high"]
    pub lut28h: crate::Reg<lut28h::LUT28H_SPEC>,
    #[doc = "0x10e8 - Graphic MMU LUT entry 29 low"]
    pub lut29l: crate::Reg<lut29l::LUT29L_SPEC>,
    #[doc = "0x10ec - Graphic MMU LUT entry 29 high"]
    pub lut29h: crate::Reg<lut29h::LUT29H_SPEC>,
    #[doc = "0x10f0 - Graphic MMU LUT entry 30 low"]
    pub lut30l: crate::Reg<lut30l::LUT30L_SPEC>,
    #[doc = "0x10f4 - Graphic MMU LUT entry 30 high"]
    pub lut30h: crate::Reg<lut30h::LUT30H_SPEC>,
    #[doc = "0x10f8 - Graphic MMU LUT entry 31 low"]
    pub lut31l: crate::Reg<lut31l::LUT31L_SPEC>,
    #[doc = "0x10fc - Graphic MMU LUT entry 31 high"]
    pub lut31h: crate::Reg<lut31h::LUT31H_SPEC>,
    #[doc = "0x1100 - Graphic MMU LUT entry 32 low"]
    pub lut32l: crate::Reg<lut32l::LUT32L_SPEC>,
    #[doc = "0x1104 - Graphic MMU LUT entry 32 high"]
    pub lut32h: crate::Reg<lut32h::LUT32H_SPEC>,
    #[doc = "0x1108 - Graphic MMU LUT entry 33 low"]
    pub lut33l: crate::Reg<lut33l::LUT33L_SPEC>,
    #[doc = "0x110c - Graphic MMU LUT entry 33 high"]
    pub lut33h: crate::Reg<lut33h::LUT33H_SPEC>,
    #[doc = "0x1110 - Graphic MMU LUT entry 34 low"]
    pub lut34l: crate::Reg<lut34l::LUT34L_SPEC>,
    #[doc = "0x1114 - Graphic MMU LUT entry 34 high"]
    pub lut34h: crate::Reg<lut34h::LUT34H_SPEC>,
    #[doc = "0x1118 - Graphic MMU LUT entry 35 low"]
    pub lut35l: crate::Reg<lut35l::LUT35L_SPEC>,
    #[doc = "0x111c - Graphic MMU LUT entry 35 high"]
    pub lut35h: crate::Reg<lut35h::LUT35H_SPEC>,
    #[doc = "0x1120 - Graphic MMU LUT entry 36 low"]
    pub lut36l: crate::Reg<lut36l::LUT36L_SPEC>,
    #[doc = "0x1124 - Graphic MMU LUT entry 36 high"]
    pub lut36h: crate::Reg<lut36h::LUT36H_SPEC>,
    #[doc = "0x1128 - Graphic MMU LUT entry 37 low"]
    pub lut37l: crate::Reg<lut37l::LUT37L_SPEC>,
    #[doc = "0x112c - Graphic MMU LUT entry 37 high"]
    pub lut37h: crate::Reg<lut37h::LUT37H_SPEC>,
    #[doc = "0x1130 - Graphic MMU LUT entry 38 low"]
    pub lut38l: crate::Reg<lut38l::LUT38L_SPEC>,
    #[doc = "0x1134 - Graphic MMU LUT entry 38 high"]
    pub lut38h: crate::Reg<lut38h::LUT38H_SPEC>,
    #[doc = "0x1138 - Graphic MMU LUT entry 39 low"]
    pub lut39l: crate::Reg<lut39l::LUT39L_SPEC>,
    #[doc = "0x113c - Graphic MMU LUT entry 39 high"]
    pub lut39h: crate::Reg<lut39h::LUT39H_SPEC>,
    #[doc = "0x1140 - Graphic MMU LUT entry 40 low"]
    pub lut40l: crate::Reg<lut40l::LUT40L_SPEC>,
    #[doc = "0x1144 - Graphic MMU LUT entry 40 high"]
    pub lut40h: crate::Reg<lut40h::LUT40H_SPEC>,
    #[doc = "0x1148 - Graphic MMU LUT entry 41 low"]
    pub lut41l: crate::Reg<lut41l::LUT41L_SPEC>,
    #[doc = "0x114c - Graphic MMU LUT entry 41 high"]
    pub lut41h: crate::Reg<lut41h::LUT41H_SPEC>,
    #[doc = "0x1150 - Graphic MMU LUT entry 42 low"]
    pub lut42l: crate::Reg<lut42l::LUT42L_SPEC>,
    #[doc = "0x1154 - Graphic MMU LUT entry 42 high"]
    pub lut42h: crate::Reg<lut42h::LUT42H_SPEC>,
    #[doc = "0x1158 - Graphic MMU LUT entry 43 low"]
    pub lut43l: crate::Reg<lut43l::LUT43L_SPEC>,
    #[doc = "0x115c - Graphic MMU LUT entry 43 high"]
    pub lut43h: crate::Reg<lut43h::LUT43H_SPEC>,
    #[doc = "0x1160 - Graphic MMU LUT entry 44 low"]
    pub lut44l: crate::Reg<lut44l::LUT44L_SPEC>,
    #[doc = "0x1164 - Graphic MMU LUT entry 44 high"]
    pub lut44h: crate::Reg<lut44h::LUT44H_SPEC>,
    #[doc = "0x1168 - Graphic MMU LUT entry 45 low"]
    pub lut45l: crate::Reg<lut45l::LUT45L_SPEC>,
    #[doc = "0x116c - Graphic MMU LUT entry 45 high"]
    pub lut45h: crate::Reg<lut45h::LUT45H_SPEC>,
    #[doc = "0x1170 - Graphic MMU LUT entry 46 low"]
    pub lut46l: crate::Reg<lut46l::LUT46L_SPEC>,
    #[doc = "0x1174 - Graphic MMU LUT entry 46 high"]
    pub lut46h: crate::Reg<lut46h::LUT46H_SPEC>,
    #[doc = "0x1178 - Graphic MMU LUT entry 47 low"]
    pub lut47l: crate::Reg<lut47l::LUT47L_SPEC>,
    #[doc = "0x117c - Graphic MMU LUT entry 47 high"]
    pub lut47h: crate::Reg<lut47h::LUT47H_SPEC>,
    #[doc = "0x1180 - Graphic MMU LUT entry 48 low"]
    pub lut48l: crate::Reg<lut48l::LUT48L_SPEC>,
    #[doc = "0x1184 - Graphic MMU LUT entry 48 high"]
    pub lut48h: crate::Reg<lut48h::LUT48H_SPEC>,
    #[doc = "0x1188 - Graphic MMU LUT entry 49 low"]
    pub lut49l: crate::Reg<lut49l::LUT49L_SPEC>,
    #[doc = "0x118c - Graphic MMU LUT entry 49 high"]
    pub lut49h: crate::Reg<lut49h::LUT49H_SPEC>,
    #[doc = "0x1190 - Graphic MMU LUT entry 50 low"]
    pub lut50l: crate::Reg<lut50l::LUT50L_SPEC>,
    #[doc = "0x1194 - Graphic MMU LUT entry 50 high"]
    pub lut50h: crate::Reg<lut50h::LUT50H_SPEC>,
    #[doc = "0x1198 - Graphic MMU LUT entry 51 low"]
    pub lut51l: crate::Reg<lut51l::LUT51L_SPEC>,
    #[doc = "0x119c - Graphic MMU LUT entry 51 high"]
    pub lut51h: crate::Reg<lut51h::LUT51H_SPEC>,
    #[doc = "0x11a0 - Graphic MMU LUT entry 52 low"]
    pub lut52l: crate::Reg<lut52l::LUT52L_SPEC>,
    #[doc = "0x11a4 - Graphic MMU LUT entry 52 high"]
    pub lut52h: crate::Reg<lut52h::LUT52H_SPEC>,
    #[doc = "0x11a8 - Graphic MMU LUT entry 53 low"]
    pub lut53l: crate::Reg<lut53l::LUT53L_SPEC>,
    #[doc = "0x11ac - Graphic MMU LUT entry 53 high"]
    pub lut53h: crate::Reg<lut53h::LUT53H_SPEC>,
    #[doc = "0x11b0 - Graphic MMU LUT entry 54 low"]
    pub lut54l: crate::Reg<lut54l::LUT54L_SPEC>,
    #[doc = "0x11b4 - Graphic MMU LUT entry 54 high"]
    pub lut54h: crate::Reg<lut54h::LUT54H_SPEC>,
    #[doc = "0x11b8 - Graphic MMU LUT entry 55 low"]
    pub lut55l: crate::Reg<lut55l::LUT55L_SPEC>,
    #[doc = "0x11bc - Graphic MMU LUT entry 55 high"]
    pub lut55h: crate::Reg<lut55h::LUT55H_SPEC>,
    #[doc = "0x11c0 - Graphic MMU LUT entry 56 low"]
    pub lut56l: crate::Reg<lut56l::LUT56L_SPEC>,
    #[doc = "0x11c4 - Graphic MMU LUT entry 56 high"]
    pub lut56h: crate::Reg<lut56h::LUT56H_SPEC>,
    #[doc = "0x11c8 - Graphic MMU LUT entry 57 low"]
    pub lut57l: crate::Reg<lut57l::LUT57L_SPEC>,
    #[doc = "0x11cc - Graphic MMU LUT entry 57 high"]
    pub lut57h: crate::Reg<lut57h::LUT57H_SPEC>,
    #[doc = "0x11d0 - Graphic MMU LUT entry 58 low"]
    pub lut58l: crate::Reg<lut58l::LUT58L_SPEC>,
    #[doc = "0x11d4 - Graphic MMU LUT entry 58 high"]
    pub lut58h: crate::Reg<lut58h::LUT58H_SPEC>,
    #[doc = "0x11d8 - Graphic MMU LUT entry 59 low"]
    pub lut59l: crate::Reg<lut59l::LUT59L_SPEC>,
    #[doc = "0x11dc - Graphic MMU LUT entry 59 high"]
    pub lut59h: crate::Reg<lut59h::LUT59H_SPEC>,
    #[doc = "0x11e0 - Graphic MMU LUT entry 60 low"]
    pub lut60l: crate::Reg<lut60l::LUT60L_SPEC>,
    #[doc = "0x11e4 - Graphic MMU LUT entry 60 high"]
    pub lut60h: crate::Reg<lut60h::LUT60H_SPEC>,
    #[doc = "0x11e8 - Graphic MMU LUT entry 61 low"]
    pub lut61l: crate::Reg<lut61l::LUT61L_SPEC>,
    #[doc = "0x11ec - Graphic MMU LUT entry 61 high"]
    pub lut61h: crate::Reg<lut61h::LUT61H_SPEC>,
    #[doc = "0x11f0 - Graphic MMU LUT entry 62 low"]
    pub lut62l: crate::Reg<lut62l::LUT62L_SPEC>,
    #[doc = "0x11f4 - Graphic MMU LUT entry 62 high"]
    pub lut62h: crate::Reg<lut62h::LUT62H_SPEC>,
    #[doc = "0x11f8 - Graphic MMU LUT entry 63 low"]
    pub lut63l: crate::Reg<lut63l::LUT63L_SPEC>,
    #[doc = "0x11fc - Graphic MMU LUT entry 63 high"]
    pub lut63h: crate::Reg<lut63h::LUT63H_SPEC>,
    #[doc = "0x1200 - Graphic MMU LUT entry 64 low"]
    pub lut64l: crate::Reg<lut64l::LUT64L_SPEC>,
    #[doc = "0x1204 - Graphic MMU LUT entry 64 high"]
    pub lut64h: crate::Reg<lut64h::LUT64H_SPEC>,
    #[doc = "0x1208 - Graphic MMU LUT entry 65 low"]
    pub lut65l: crate::Reg<lut65l::LUT65L_SPEC>,
    #[doc = "0x120c - Graphic MMU LUT entry 65 high"]
    pub lut65h: crate::Reg<lut65h::LUT65H_SPEC>,
    #[doc = "0x1210 - Graphic MMU LUT entry 66 low"]
    pub lut66l: crate::Reg<lut66l::LUT66L_SPEC>,
    #[doc = "0x1214 - Graphic MMU LUT entry 66 high"]
    pub lut66h: crate::Reg<lut66h::LUT66H_SPEC>,
    #[doc = "0x1218 - Graphic MMU LUT entry 67 low"]
    pub lut67l: crate::Reg<lut67l::LUT67L_SPEC>,
    #[doc = "0x121c - Graphic MMU LUT entry 67 high"]
    pub lut67h: crate::Reg<lut67h::LUT67H_SPEC>,
    #[doc = "0x1220 - Graphic MMU LUT entry 68 low"]
    pub lut68l: crate::Reg<lut68l::LUT68L_SPEC>,
    #[doc = "0x1224 - Graphic MMU LUT entry 68 high"]
    pub lut68h: crate::Reg<lut68h::LUT68H_SPEC>,
    #[doc = "0x1228 - Graphic MMU LUT entry 69 low"]
    pub lut69l: crate::Reg<lut69l::LUT69L_SPEC>,
    #[doc = "0x122c - Graphic MMU LUT entry 69 high"]
    pub lut69h: crate::Reg<lut69h::LUT69H_SPEC>,
    #[doc = "0x1230 - Graphic MMU LUT entry 70 low"]
    pub lut70l: crate::Reg<lut70l::LUT70L_SPEC>,
    #[doc = "0x1234 - Graphic MMU LUT entry 70 high"]
    pub lut70h: crate::Reg<lut70h::LUT70H_SPEC>,
    #[doc = "0x1238 - Graphic MMU LUT entry 71 low"]
    pub lut71l: crate::Reg<lut71l::LUT71L_SPEC>,
    #[doc = "0x123c - Graphic MMU LUT entry 71 high"]
    pub lut71h: crate::Reg<lut71h::LUT71H_SPEC>,
    #[doc = "0x1240 - Graphic MMU LUT entry 72 low"]
    pub lut72l: crate::Reg<lut72l::LUT72L_SPEC>,
    #[doc = "0x1244 - Graphic MMU LUT entry 72 high"]
    pub lut72h: crate::Reg<lut72h::LUT72H_SPEC>,
    #[doc = "0x1248 - Graphic MMU LUT entry 73 low"]
    pub lut73l: crate::Reg<lut73l::LUT73L_SPEC>,
    #[doc = "0x124c - Graphic MMU LUT entry 73 high"]
    pub lut73h: crate::Reg<lut73h::LUT73H_SPEC>,
    #[doc = "0x1250 - Graphic MMU LUT entry 74 low"]
    pub lut74l: crate::Reg<lut74l::LUT74L_SPEC>,
    #[doc = "0x1254 - Graphic MMU LUT entry 74 high"]
    pub lut74h: crate::Reg<lut74h::LUT74H_SPEC>,
    #[doc = "0x1258 - Graphic MMU LUT entry 75 low"]
    pub lut75l: crate::Reg<lut75l::LUT75L_SPEC>,
    #[doc = "0x125c - Graphic MMU LUT entry 75 high"]
    pub lut75h: crate::Reg<lut75h::LUT75H_SPEC>,
    #[doc = "0x1260 - Graphic MMU LUT entry 76 low"]
    pub lut76l: crate::Reg<lut76l::LUT76L_SPEC>,
    #[doc = "0x1264 - Graphic MMU LUT entry 76 high"]
    pub lut76h: crate::Reg<lut76h::LUT76H_SPEC>,
    #[doc = "0x1268 - Graphic MMU LUT entry 77 low"]
    pub lut77l: crate::Reg<lut77l::LUT77L_SPEC>,
    #[doc = "0x126c - Graphic MMU LUT entry 77 high"]
    pub lut77h: crate::Reg<lut77h::LUT77H_SPEC>,
    #[doc = "0x1270 - Graphic MMU LUT entry 78 low"]
    pub lut78l: crate::Reg<lut78l::LUT78L_SPEC>,
    #[doc = "0x1274 - Graphic MMU LUT entry 78 high"]
    pub lut78h: crate::Reg<lut78h::LUT78H_SPEC>,
    #[doc = "0x1278 - Graphic MMU LUT entry 79 low"]
    pub lut79l: crate::Reg<lut79l::LUT79L_SPEC>,
    #[doc = "0x127c - Graphic MMU LUT entry 79 high"]
    pub lut79h: crate::Reg<lut79h::LUT79H_SPEC>,
    #[doc = "0x1280 - Graphic MMU LUT entry 80 low"]
    pub lut80l: crate::Reg<lut80l::LUT80L_SPEC>,
    #[doc = "0x1284 - Graphic MMU LUT entry 80 high"]
    pub lut80h: crate::Reg<lut80h::LUT80H_SPEC>,
    #[doc = "0x1288 - Graphic MMU LUT entry 81 low"]
    pub lut81l: crate::Reg<lut81l::LUT81L_SPEC>,
    #[doc = "0x128c - Graphic MMU LUT entry 81 high"]
    pub lut81h: crate::Reg<lut81h::LUT81H_SPEC>,
    #[doc = "0x1290 - Graphic MMU LUT entry 82 low"]
    pub lut82l: crate::Reg<lut82l::LUT82L_SPEC>,
    #[doc = "0x1294 - Graphic MMU LUT entry 82 high"]
    pub lut82h: crate::Reg<lut82h::LUT82H_SPEC>,
    #[doc = "0x1298 - Graphic MMU LUT entry 83 low"]
    pub lut83l: crate::Reg<lut83l::LUT83L_SPEC>,
    #[doc = "0x129c - Graphic MMU LUT entry 83 high"]
    pub lut83h: crate::Reg<lut83h::LUT83H_SPEC>,
    #[doc = "0x12a0 - Graphic MMU LUT entry 84 low"]
    pub lut84l: crate::Reg<lut84l::LUT84L_SPEC>,
    #[doc = "0x12a4 - Graphic MMU LUT entry 84 high"]
    pub lut84h: crate::Reg<lut84h::LUT84H_SPEC>,
    #[doc = "0x12a8 - Graphic MMU LUT entry 85 low"]
    pub lut85l: crate::Reg<lut85l::LUT85L_SPEC>,
    #[doc = "0x12ac - Graphic MMU LUT entry 85 high"]
    pub lut85h: crate::Reg<lut85h::LUT85H_SPEC>,
    #[doc = "0x12b0 - Graphic MMU LUT entry 86 low"]
    pub lut86l: crate::Reg<lut86l::LUT86L_SPEC>,
    #[doc = "0x12b4 - Graphic MMU LUT entry 86 high"]
    pub lut86h: crate::Reg<lut86h::LUT86H_SPEC>,
    #[doc = "0x12b8 - Graphic MMU LUT entry 87 low"]
    pub lut87l: crate::Reg<lut87l::LUT87L_SPEC>,
    #[doc = "0x12bc - Graphic MMU LUT entry 87 high"]
    pub lut87h: crate::Reg<lut87h::LUT87H_SPEC>,
    #[doc = "0x12c0 - Graphic MMU LUT entry 88 low"]
    pub lut88l: crate::Reg<lut88l::LUT88L_SPEC>,
    #[doc = "0x12c4 - Graphic MMU LUT entry 88 high"]
    pub lut88h: crate::Reg<lut88h::LUT88H_SPEC>,
    #[doc = "0x12c8 - Graphic MMU LUT entry 89 low"]
    pub lut89l: crate::Reg<lut89l::LUT89L_SPEC>,
    #[doc = "0x12cc - Graphic MMU LUT entry 89 high"]
    pub lut89h: crate::Reg<lut89h::LUT89H_SPEC>,
    #[doc = "0x12d0 - Graphic MMU LUT entry 90 low"]
    pub lut90l: crate::Reg<lut90l::LUT90L_SPEC>,
    #[doc = "0x12d4 - Graphic MMU LUT entry 90 high"]
    pub lut90h: crate::Reg<lut90h::LUT90H_SPEC>,
    #[doc = "0x12d8 - Graphic MMU LUT entry 91 low"]
    pub lut91l: crate::Reg<lut91l::LUT91L_SPEC>,
    #[doc = "0x12dc - Graphic MMU LUT entry 91 high"]
    pub lut91h: crate::Reg<lut91h::LUT91H_SPEC>,
    #[doc = "0x12e0 - Graphic MMU LUT entry 92 low"]
    pub lut92l: crate::Reg<lut92l::LUT92L_SPEC>,
    #[doc = "0x12e4 - Graphic MMU LUT entry 92 high"]
    pub lut92h: crate::Reg<lut92h::LUT92H_SPEC>,
    #[doc = "0x12e8 - Graphic MMU LUT entry 93 low"]
    pub lut93l: crate::Reg<lut93l::LUT93L_SPEC>,
    #[doc = "0x12ec - Graphic MMU LUT entry 93 high"]
    pub lut93h: crate::Reg<lut93h::LUT93H_SPEC>,
    #[doc = "0x12f0 - Graphic MMU LUT entry 94 low"]
    pub lut94l: crate::Reg<lut94l::LUT94L_SPEC>,
    #[doc = "0x12f4 - Graphic MMU LUT entry 94 high"]
    pub lut94h: crate::Reg<lut94h::LUT94H_SPEC>,
    #[doc = "0x12f8 - Graphic MMU LUT entry 95 low"]
    pub lut95l: crate::Reg<lut95l::LUT95L_SPEC>,
    #[doc = "0x12fc - Graphic MMU LUT entry 95 high"]
    pub lut95h: crate::Reg<lut95h::LUT95H_SPEC>,
    #[doc = "0x1300 - Graphic MMU LUT entry 96 low"]
    pub lut96l: crate::Reg<lut96l::LUT96L_SPEC>,
    #[doc = "0x1304 - Graphic MMU LUT entry 96 high"]
    pub lut96h: crate::Reg<lut96h::LUT96H_SPEC>,
    #[doc = "0x1308 - Graphic MMU LUT entry 97 low"]
    pub lut97l: crate::Reg<lut97l::LUT97L_SPEC>,
    #[doc = "0x130c - Graphic MMU LUT entry 97 high"]
    pub lut97h: crate::Reg<lut97h::LUT97H_SPEC>,
    #[doc = "0x1310 - Graphic MMU LUT entry 98 low"]
    pub lut98l: crate::Reg<lut98l::LUT98L_SPEC>,
    #[doc = "0x1314 - Graphic MMU LUT entry 98 high"]
    pub lut98h: crate::Reg<lut98h::LUT98H_SPEC>,
    #[doc = "0x1318 - Graphic MMU LUT entry 99 low"]
    pub lut99l: crate::Reg<lut99l::LUT99L_SPEC>,
    #[doc = "0x131c - Graphic MMU LUT entry 99 high"]
    pub lut99h: crate::Reg<lut99h::LUT99H_SPEC>,
    #[doc = "0x1320 - Graphic MMU LUT entry 100 low"]
    pub lut100l: crate::Reg<lut100l::LUT100L_SPEC>,
    #[doc = "0x1324 - Graphic MMU LUT entry 100 high"]
    pub lut100h: crate::Reg<lut100h::LUT100H_SPEC>,
    #[doc = "0x1328 - Graphic MMU LUT entry 101 low"]
    pub lut101l: crate::Reg<lut101l::LUT101L_SPEC>,
    #[doc = "0x132c - Graphic MMU LUT entry 101 high"]
    pub lut101h: crate::Reg<lut101h::LUT101H_SPEC>,
    #[doc = "0x1330 - Graphic MMU LUT entry 102 low"]
    pub lut102l: crate::Reg<lut102l::LUT102L_SPEC>,
    #[doc = "0x1334 - Graphic MMU LUT entry 102 high"]
    pub lut102h: crate::Reg<lut102h::LUT102H_SPEC>,
    #[doc = "0x1338 - Graphic MMU LUT entry 103 low"]
    pub lut103l: crate::Reg<lut103l::LUT103L_SPEC>,
    #[doc = "0x133c - Graphic MMU LUT entry 103 high"]
    pub lut103h: crate::Reg<lut103h::LUT103H_SPEC>,
    #[doc = "0x1340 - Graphic MMU LUT entry 104 low"]
    pub lut104l: crate::Reg<lut104l::LUT104L_SPEC>,
    #[doc = "0x1344 - Graphic MMU LUT entry 104 high"]
    pub lut104h: crate::Reg<lut104h::LUT104H_SPEC>,
    #[doc = "0x1348 - Graphic MMU LUT entry 105 low"]
    pub lut105l: crate::Reg<lut105l::LUT105L_SPEC>,
    #[doc = "0x134c - Graphic MMU LUT entry 105 high"]
    pub lut105h: crate::Reg<lut105h::LUT105H_SPEC>,
    #[doc = "0x1350 - Graphic MMU LUT entry 106 low"]
    pub lut106l: crate::Reg<lut106l::LUT106L_SPEC>,
    #[doc = "0x1354 - Graphic MMU LUT entry 106 high"]
    pub lut106h: crate::Reg<lut106h::LUT106H_SPEC>,
    #[doc = "0x1358 - Graphic MMU LUT entry 107 low"]
    pub lut107l: crate::Reg<lut107l::LUT107L_SPEC>,
    #[doc = "0x135c - Graphic MMU LUT entry 107 high"]
    pub lut107h: crate::Reg<lut107h::LUT107H_SPEC>,
    #[doc = "0x1360 - Graphic MMU LUT entry 108 low"]
    pub lut108l: crate::Reg<lut108l::LUT108L_SPEC>,
    #[doc = "0x1364 - Graphic MMU LUT entry 108 high"]
    pub lut108h: crate::Reg<lut108h::LUT108H_SPEC>,
    #[doc = "0x1368 - Graphic MMU LUT entry 109 low"]
    pub lut109l: crate::Reg<lut109l::LUT109L_SPEC>,
    #[doc = "0x136c - Graphic MMU LUT entry 109 high"]
    pub lut109h: crate::Reg<lut109h::LUT109H_SPEC>,
    #[doc = "0x1370 - Graphic MMU LUT entry 110 low"]
    pub lut110l: crate::Reg<lut110l::LUT110L_SPEC>,
    #[doc = "0x1374 - Graphic MMU LUT entry 110 high"]
    pub lut110h: crate::Reg<lut110h::LUT110H_SPEC>,
    #[doc = "0x1378 - Graphic MMU LUT entry 111 low"]
    pub lut111l: crate::Reg<lut111l::LUT111L_SPEC>,
    #[doc = "0x137c - Graphic MMU LUT entry 111 high"]
    pub lut111h: crate::Reg<lut111h::LUT111H_SPEC>,
    #[doc = "0x1380 - Graphic MMU LUT entry 112 low"]
    pub lut112l: crate::Reg<lut112l::LUT112L_SPEC>,
    #[doc = "0x1384 - Graphic MMU LUT entry 112 high"]
    pub lut112h: crate::Reg<lut112h::LUT112H_SPEC>,
    #[doc = "0x1388 - Graphic MMU LUT entry 113 low"]
    pub lut113l: crate::Reg<lut113l::LUT113L_SPEC>,
    #[doc = "0x138c - Graphic MMU LUT entry 113 high"]
    pub lut113h: crate::Reg<lut113h::LUT113H_SPEC>,
    #[doc = "0x1390 - Graphic MMU LUT entry 114 low"]
    pub lut114l: crate::Reg<lut114l::LUT114L_SPEC>,
    #[doc = "0x1394 - Graphic MMU LUT entry 114 high"]
    pub lut114h: crate::Reg<lut114h::LUT114H_SPEC>,
    #[doc = "0x1398 - Graphic MMU LUT entry 115 low"]
    pub lut115l: crate::Reg<lut115l::LUT115L_SPEC>,
    #[doc = "0x139c - Graphic MMU LUT entry 115 high"]
    pub lut115h: crate::Reg<lut115h::LUT115H_SPEC>,
    #[doc = "0x13a0 - Graphic MMU LUT entry 116 low"]
    pub lut116l: crate::Reg<lut116l::LUT116L_SPEC>,
    #[doc = "0x13a4 - Graphic MMU LUT entry 116 high"]
    pub lut116h: crate::Reg<lut116h::LUT116H_SPEC>,
    #[doc = "0x13a8 - Graphic MMU LUT entry 117 low"]
    pub lut117l: crate::Reg<lut117l::LUT117L_SPEC>,
    #[doc = "0x13ac - Graphic MMU LUT entry 117 high"]
    pub lut117h: crate::Reg<lut117h::LUT117H_SPEC>,
    #[doc = "0x13b0 - Graphic MMU LUT entry 118 low"]
    pub lut118l: crate::Reg<lut118l::LUT118L_SPEC>,
    #[doc = "0x13b4 - Graphic MMU LUT entry 118 high"]
    pub lut118h: crate::Reg<lut118h::LUT118H_SPEC>,
    #[doc = "0x13b8 - Graphic MMU LUT entry 119 low"]
    pub lut119l: crate::Reg<lut119l::LUT119L_SPEC>,
    #[doc = "0x13bc - Graphic MMU LUT entry 119 high"]
    pub lut119h: crate::Reg<lut119h::LUT119H_SPEC>,
    #[doc = "0x13c0 - Graphic MMU LUT entry 120 low"]
    pub lut120l: crate::Reg<lut120l::LUT120L_SPEC>,
    #[doc = "0x13c4 - Graphic MMU LUT entry 120 high"]
    pub lut120h: crate::Reg<lut120h::LUT120H_SPEC>,
    #[doc = "0x13c8 - Graphic MMU LUT entry 121 low"]
    pub lut121l: crate::Reg<lut121l::LUT121L_SPEC>,
    #[doc = "0x13cc - Graphic MMU LUT entry 121 high"]
    pub lut121h: crate::Reg<lut121h::LUT121H_SPEC>,
    #[doc = "0x13d0 - Graphic MMU LUT entry 122 low"]
    pub lut122l: crate::Reg<lut122l::LUT122L_SPEC>,
    #[doc = "0x13d4 - Graphic MMU LUT entry 122 high"]
    pub lut122h: crate::Reg<lut122h::LUT122H_SPEC>,
    #[doc = "0x13d8 - Graphic MMU LUT entry 123 low"]
    pub lut123l: crate::Reg<lut123l::LUT123L_SPEC>,
    #[doc = "0x13dc - Graphic MMU LUT entry 123 high"]
    pub lut123h: crate::Reg<lut123h::LUT123H_SPEC>,
    #[doc = "0x13e0 - Graphic MMU LUT entry 124 low"]
    pub lut124l: crate::Reg<lut124l::LUT124L_SPEC>,
    #[doc = "0x13e4 - Graphic MMU LUT entry 124 high"]
    pub lut124h: crate::Reg<lut124h::LUT124H_SPEC>,
    #[doc = "0x13e8 - Graphic MMU LUT entry 125 low"]
    pub lut125l: crate::Reg<lut125l::LUT125L_SPEC>,
    #[doc = "0x13ec - Graphic MMU LUT entry 125 high"]
    pub lut125h: crate::Reg<lut125h::LUT125H_SPEC>,
    #[doc = "0x13f0 - Graphic MMU LUT entry 126 low"]
    pub lut126l: crate::Reg<lut126l::LUT126L_SPEC>,
    #[doc = "0x13f4 - Graphic MMU LUT entry 126 high"]
    pub lut126h: crate::Reg<lut126h::LUT126H_SPEC>,
    #[doc = "0x13f8 - Graphic MMU LUT entry 127 low"]
    pub lut127l: crate::Reg<lut127l::LUT127L_SPEC>,
    #[doc = "0x13fc - Graphic MMU LUT entry 127 high"]
    pub lut127h: crate::Reg<lut127h::LUT127H_SPEC>,
    #[doc = "0x1400 - Graphic MMU LUT entry 128 low"]
    pub lut128l: crate::Reg<lut128l::LUT128L_SPEC>,
    #[doc = "0x1404 - Graphic MMU LUT entry 128 high"]
    pub lut128h: crate::Reg<lut128h::LUT128H_SPEC>,
    #[doc = "0x1408 - Graphic MMU LUT entry 129 low"]
    pub lut129l: crate::Reg<lut129l::LUT129L_SPEC>,
    #[doc = "0x140c - Graphic MMU LUT entry 129 high"]
    pub lut129h: crate::Reg<lut129h::LUT129H_SPEC>,
    #[doc = "0x1410 - Graphic MMU LUT entry 130 low"]
    pub lut130l: crate::Reg<lut130l::LUT130L_SPEC>,
    #[doc = "0x1414 - Graphic MMU LUT entry 130 high"]
    pub lut130h: crate::Reg<lut130h::LUT130H_SPEC>,
    #[doc = "0x1418 - Graphic MMU LUT entry 131 low"]
    pub lut131l: crate::Reg<lut131l::LUT131L_SPEC>,
    #[doc = "0x141c - Graphic MMU LUT entry 131 high"]
    pub lut131h: crate::Reg<lut131h::LUT131H_SPEC>,
    #[doc = "0x1420 - Graphic MMU LUT entry 132 low"]
    pub lut132l: crate::Reg<lut132l::LUT132L_SPEC>,
    #[doc = "0x1424 - Graphic MMU LUT entry 132 high"]
    pub lut132h: crate::Reg<lut132h::LUT132H_SPEC>,
    #[doc = "0x1428 - Graphic MMU LUT entry 133 low"]
    pub lut133l: crate::Reg<lut133l::LUT133L_SPEC>,
    #[doc = "0x142c - Graphic MMU LUT entry 133 high"]
    pub lut133h: crate::Reg<lut133h::LUT133H_SPEC>,
    #[doc = "0x1430 - Graphic MMU LUT entry 134 low"]
    pub lut134l: crate::Reg<lut134l::LUT134L_SPEC>,
    #[doc = "0x1434 - Graphic MMU LUT entry 134 high"]
    pub lut134h: crate::Reg<lut134h::LUT134H_SPEC>,
    #[doc = "0x1438 - Graphic MMU LUT entry 135 low"]
    pub lut135l: crate::Reg<lut135l::LUT135L_SPEC>,
    #[doc = "0x143c - Graphic MMU LUT entry 135 high"]
    pub lut135h: crate::Reg<lut135h::LUT135H_SPEC>,
    #[doc = "0x1440 - Graphic MMU LUT entry 136 low"]
    pub lut136l: crate::Reg<lut136l::LUT136L_SPEC>,
    #[doc = "0x1444 - Graphic MMU LUT entry 136 high"]
    pub lut136h: crate::Reg<lut136h::LUT136H_SPEC>,
    #[doc = "0x1448 - Graphic MMU LUT entry 137 low"]
    pub lut137l: crate::Reg<lut137l::LUT137L_SPEC>,
    #[doc = "0x144c - Graphic MMU LUT entry 137 high"]
    pub lut137h: crate::Reg<lut137h::LUT137H_SPEC>,
    #[doc = "0x1450 - Graphic MMU LUT entry 138 low"]
    pub lut138l: crate::Reg<lut138l::LUT138L_SPEC>,
    #[doc = "0x1454 - Graphic MMU LUT entry 138 high"]
    pub lut138h: crate::Reg<lut138h::LUT138H_SPEC>,
    #[doc = "0x1458 - Graphic MMU LUT entry 139 low"]
    pub lut139l: crate::Reg<lut139l::LUT139L_SPEC>,
    #[doc = "0x145c - Graphic MMU LUT entry 139 high"]
    pub lut139h: crate::Reg<lut139h::LUT139H_SPEC>,
    #[doc = "0x1460 - Graphic MMU LUT entry 140 low"]
    pub lut140l: crate::Reg<lut140l::LUT140L_SPEC>,
    #[doc = "0x1464 - Graphic MMU LUT entry 140 high"]
    pub lut140h: crate::Reg<lut140h::LUT140H_SPEC>,
    #[doc = "0x1468 - Graphic MMU LUT entry 141 low"]
    pub lut141l: crate::Reg<lut141l::LUT141L_SPEC>,
    #[doc = "0x146c - Graphic MMU LUT entry 141 high"]
    pub lut141h: crate::Reg<lut141h::LUT141H_SPEC>,
    #[doc = "0x1470 - Graphic MMU LUT entry 142 low"]
    pub lut142l: crate::Reg<lut142l::LUT142L_SPEC>,
    #[doc = "0x1474 - Graphic MMU LUT entry 142 high"]
    pub lut142h: crate::Reg<lut142h::LUT142H_SPEC>,
    #[doc = "0x1478 - Graphic MMU LUT entry 143 low"]
    pub lut143l: crate::Reg<lut143l::LUT143L_SPEC>,
    #[doc = "0x147c - Graphic MMU LUT entry 143 high"]
    pub lut143h: crate::Reg<lut143h::LUT143H_SPEC>,
    #[doc = "0x1480 - Graphic MMU LUT entry 144 low"]
    pub lut144l: crate::Reg<lut144l::LUT144L_SPEC>,
    #[doc = "0x1484 - Graphic MMU LUT entry 144 high"]
    pub lut144h: crate::Reg<lut144h::LUT144H_SPEC>,
    #[doc = "0x1488 - Graphic MMU LUT entry 145 low"]
    pub lut145l: crate::Reg<lut145l::LUT145L_SPEC>,
    #[doc = "0x148c - Graphic MMU LUT entry 145 high"]
    pub lut145h: crate::Reg<lut145h::LUT145H_SPEC>,
    #[doc = "0x1490 - Graphic MMU LUT entry 146 low"]
    pub lut146l: crate::Reg<lut146l::LUT146L_SPEC>,
    #[doc = "0x1494 - Graphic MMU LUT entry 146 high"]
    pub lut146h: crate::Reg<lut146h::LUT146H_SPEC>,
    #[doc = "0x1498 - Graphic MMU LUT entry 147 low"]
    pub lut147l: crate::Reg<lut147l::LUT147L_SPEC>,
    #[doc = "0x149c - Graphic MMU LUT entry 147 high"]
    pub lut147h: crate::Reg<lut147h::LUT147H_SPEC>,
    #[doc = "0x14a0 - Graphic MMU LUT entry 148 low"]
    pub lut148l: crate::Reg<lut148l::LUT148L_SPEC>,
    #[doc = "0x14a4 - Graphic MMU LUT entry 148 high"]
    pub lut148h: crate::Reg<lut148h::LUT148H_SPEC>,
    #[doc = "0x14a8 - Graphic MMU LUT entry 149 low"]
    pub lut149l: crate::Reg<lut149l::LUT149L_SPEC>,
    #[doc = "0x14ac - Graphic MMU LUT entry 149 high"]
    pub lut149h: crate::Reg<lut149h::LUT149H_SPEC>,
    #[doc = "0x14b0 - Graphic MMU LUT entry 150 low"]
    pub lut150l: crate::Reg<lut150l::LUT150L_SPEC>,
    #[doc = "0x14b4 - Graphic MMU LUT entry 150 high"]
    pub lut150h: crate::Reg<lut150h::LUT150H_SPEC>,
    #[doc = "0x14b8 - Graphic MMU LUT entry 151 low"]
    pub lut151l: crate::Reg<lut151l::LUT151L_SPEC>,
    #[doc = "0x14bc - Graphic MMU LUT entry 151 high"]
    pub lut151h: crate::Reg<lut151h::LUT151H_SPEC>,
    #[doc = "0x14c0 - Graphic MMU LUT entry 152 low"]
    pub lut152l: crate::Reg<lut152l::LUT152L_SPEC>,
    #[doc = "0x14c4 - Graphic MMU LUT entry 152 high"]
    pub lut152h: crate::Reg<lut152h::LUT152H_SPEC>,
    #[doc = "0x14c8 - Graphic MMU LUT entry 153 low"]
    pub lut153l: crate::Reg<lut153l::LUT153L_SPEC>,
    #[doc = "0x14cc - Graphic MMU LUT entry 153 high"]
    pub lut153h: crate::Reg<lut153h::LUT153H_SPEC>,
    #[doc = "0x14d0 - Graphic MMU LUT entry 154 low"]
    pub lut154l: crate::Reg<lut154l::LUT154L_SPEC>,
    #[doc = "0x14d4 - Graphic MMU LUT entry 154 high"]
    pub lut154h: crate::Reg<lut154h::LUT154H_SPEC>,
    #[doc = "0x14d8 - Graphic MMU LUT entry 155 low"]
    pub lut155l: crate::Reg<lut155l::LUT155L_SPEC>,
    #[doc = "0x14dc - Graphic MMU LUT entry 155 high"]
    pub lut155h: crate::Reg<lut155h::LUT155H_SPEC>,
    #[doc = "0x14e0 - Graphic MMU LUT entry 156 low"]
    pub lut156l: crate::Reg<lut156l::LUT156L_SPEC>,
    #[doc = "0x14e4 - Graphic MMU LUT entry 156 high"]
    pub lut156h: crate::Reg<lut156h::LUT156H_SPEC>,
    #[doc = "0x14e8 - Graphic MMU LUT entry 157 low"]
    pub lut157l: crate::Reg<lut157l::LUT157L_SPEC>,
    #[doc = "0x14ec - Graphic MMU LUT entry 157 high"]
    pub lut157h: crate::Reg<lut157h::LUT157H_SPEC>,
    #[doc = "0x14f0 - Graphic MMU LUT entry 158 low"]
    pub lut158l: crate::Reg<lut158l::LUT158L_SPEC>,
    #[doc = "0x14f4 - Graphic MMU LUT entry 158 high"]
    pub lut158h: crate::Reg<lut158h::LUT158H_SPEC>,
    #[doc = "0x14f8 - Graphic MMU LUT entry 159 low"]
    pub lut159l: crate::Reg<lut159l::LUT159L_SPEC>,
    #[doc = "0x14fc - Graphic MMU LUT entry 159 high"]
    pub lut159h: crate::Reg<lut159h::LUT159H_SPEC>,
    #[doc = "0x1500 - Graphic MMU LUT entry 160 low"]
    pub lut160l: crate::Reg<lut160l::LUT160L_SPEC>,
    #[doc = "0x1504 - Graphic MMU LUT entry 160 high"]
    pub lut160h: crate::Reg<lut160h::LUT160H_SPEC>,
    #[doc = "0x1508 - Graphic MMU LUT entry 161 low"]
    pub lut161l: crate::Reg<lut161l::LUT161L_SPEC>,
    #[doc = "0x150c - Graphic MMU LUT entry 161 high"]
    pub lut161h: crate::Reg<lut161h::LUT161H_SPEC>,
    #[doc = "0x1510 - Graphic MMU LUT entry 162 low"]
    pub lut162l: crate::Reg<lut162l::LUT162L_SPEC>,
    #[doc = "0x1514 - Graphic MMU LUT entry 162 high"]
    pub lut162h: crate::Reg<lut162h::LUT162H_SPEC>,
    #[doc = "0x1518 - Graphic MMU LUT entry 163 low"]
    pub lut163l: crate::Reg<lut163l::LUT163L_SPEC>,
    #[doc = "0x151c - Graphic MMU LUT entry 163 high"]
    pub lut163h: crate::Reg<lut163h::LUT163H_SPEC>,
    #[doc = "0x1520 - Graphic MMU LUT entry 164 low"]
    pub lut164l: crate::Reg<lut164l::LUT164L_SPEC>,
    #[doc = "0x1524 - Graphic MMU LUT entry 164 high"]
    pub lut164h: crate::Reg<lut164h::LUT164H_SPEC>,
    #[doc = "0x1528 - Graphic MMU LUT entry 165 low"]
    pub lut165l: crate::Reg<lut165l::LUT165L_SPEC>,
    #[doc = "0x152c - Graphic MMU LUT entry 165 high"]
    pub lut165h: crate::Reg<lut165h::LUT165H_SPEC>,
    #[doc = "0x1530 - Graphic MMU LUT entry 166 low"]
    pub lut166l: crate::Reg<lut166l::LUT166L_SPEC>,
    #[doc = "0x1534 - Graphic MMU LUT entry 166 high"]
    pub lut166h: crate::Reg<lut166h::LUT166H_SPEC>,
    #[doc = "0x1538 - Graphic MMU LUT entry 167 low"]
    pub lut167l: crate::Reg<lut167l::LUT167L_SPEC>,
    #[doc = "0x153c - Graphic MMU LUT entry 167 high"]
    pub lut167h: crate::Reg<lut167h::LUT167H_SPEC>,
    #[doc = "0x1540 - Graphic MMU LUT entry 168 low"]
    pub lut168l: crate::Reg<lut168l::LUT168L_SPEC>,
    #[doc = "0x1544 - Graphic MMU LUT entry 168 high"]
    pub lut168h: crate::Reg<lut168h::LUT168H_SPEC>,
    #[doc = "0x1548 - Graphic MMU LUT entry 169 low"]
    pub lut169l: crate::Reg<lut169l::LUT169L_SPEC>,
    #[doc = "0x154c - Graphic MMU LUT entry 169 high"]
    pub lut169h: crate::Reg<lut169h::LUT169H_SPEC>,
    #[doc = "0x1550 - Graphic MMU LUT entry 170 low"]
    pub lut170l: crate::Reg<lut170l::LUT170L_SPEC>,
    #[doc = "0x1554 - Graphic MMU LUT entry 170 high"]
    pub lut170h: crate::Reg<lut170h::LUT170H_SPEC>,
    #[doc = "0x1558 - Graphic MMU LUT entry 171 low"]
    pub lut171l: crate::Reg<lut171l::LUT171L_SPEC>,
    #[doc = "0x155c - Graphic MMU LUT entry 171 high"]
    pub lut171h: crate::Reg<lut171h::LUT171H_SPEC>,
    #[doc = "0x1560 - Graphic MMU LUT entry 172 low"]
    pub lut172l: crate::Reg<lut172l::LUT172L_SPEC>,
    #[doc = "0x1564 - Graphic MMU LUT entry 172 high"]
    pub lut172h: crate::Reg<lut172h::LUT172H_SPEC>,
    #[doc = "0x1568 - Graphic MMU LUT entry 173 low"]
    pub lut173l: crate::Reg<lut173l::LUT173L_SPEC>,
    #[doc = "0x156c - Graphic MMU LUT entry 173 high"]
    pub lut173h: crate::Reg<lut173h::LUT173H_SPEC>,
    #[doc = "0x1570 - Graphic MMU LUT entry 174 low"]
    pub lut174l: crate::Reg<lut174l::LUT174L_SPEC>,
    #[doc = "0x1574 - Graphic MMU LUT entry 174 high"]
    pub lut174h: crate::Reg<lut174h::LUT174H_SPEC>,
    #[doc = "0x1578 - Graphic MMU LUT entry 175 low"]
    pub lut175l: crate::Reg<lut175l::LUT175L_SPEC>,
    #[doc = "0x157c - Graphic MMU LUT entry 175 high"]
    pub lut175h: crate::Reg<lut175h::LUT175H_SPEC>,
    #[doc = "0x1580 - Graphic MMU LUT entry 176 low"]
    pub lut176l: crate::Reg<lut176l::LUT176L_SPEC>,
    #[doc = "0x1584 - Graphic MMU LUT entry 176 high"]
    pub lut176h: crate::Reg<lut176h::LUT176H_SPEC>,
    #[doc = "0x1588 - Graphic MMU LUT entry 177 low"]
    pub lut177l: crate::Reg<lut177l::LUT177L_SPEC>,
    #[doc = "0x158c - Graphic MMU LUT entry 177 high"]
    pub lut177h: crate::Reg<lut177h::LUT177H_SPEC>,
    #[doc = "0x1590 - Graphic MMU LUT entry 178 low"]
    pub lut178l: crate::Reg<lut178l::LUT178L_SPEC>,
    #[doc = "0x1594 - Graphic MMU LUT entry 178 high"]
    pub lut178h: crate::Reg<lut178h::LUT178H_SPEC>,
    #[doc = "0x1598 - Graphic MMU LUT entry 179 low"]
    pub lut179l: crate::Reg<lut179l::LUT179L_SPEC>,
    #[doc = "0x159c - Graphic MMU LUT entry 179 high"]
    pub lut179h: crate::Reg<lut179h::LUT179H_SPEC>,
    #[doc = "0x15a0 - Graphic MMU LUT entry 180 low"]
    pub lut180l: crate::Reg<lut180l::LUT180L_SPEC>,
    #[doc = "0x15a4 - Graphic MMU LUT entry 180 high"]
    pub lut180h: crate::Reg<lut180h::LUT180H_SPEC>,
    #[doc = "0x15a8 - Graphic MMU LUT entry 181 low"]
    pub lut181l: crate::Reg<lut181l::LUT181L_SPEC>,
    #[doc = "0x15ac - Graphic MMU LUT entry 181 high"]
    pub lut181h: crate::Reg<lut181h::LUT181H_SPEC>,
    #[doc = "0x15b0 - Graphic MMU LUT entry 182 low"]
    pub lut182l: crate::Reg<lut182l::LUT182L_SPEC>,
    #[doc = "0x15b4 - Graphic MMU LUT entry 182 high"]
    pub lut182h: crate::Reg<lut182h::LUT182H_SPEC>,
    #[doc = "0x15b8 - Graphic MMU LUT entry 183 low"]
    pub lut183l: crate::Reg<lut183l::LUT183L_SPEC>,
    #[doc = "0x15bc - Graphic MMU LUT entry 183 high"]
    pub lut183h: crate::Reg<lut183h::LUT183H_SPEC>,
    #[doc = "0x15c0 - Graphic MMU LUT entry 184 low"]
    pub lut184l: crate::Reg<lut184l::LUT184L_SPEC>,
    #[doc = "0x15c4 - Graphic MMU LUT entry 184 high"]
    pub lut184h: crate::Reg<lut184h::LUT184H_SPEC>,
    #[doc = "0x15c8 - Graphic MMU LUT entry 185 low"]
    pub lut185l: crate::Reg<lut185l::LUT185L_SPEC>,
    #[doc = "0x15cc - Graphic MMU LUT entry 185 high"]
    pub lut185h: crate::Reg<lut185h::LUT185H_SPEC>,
    #[doc = "0x15d0 - Graphic MMU LUT entry 186 low"]
    pub lut186l: crate::Reg<lut186l::LUT186L_SPEC>,
    #[doc = "0x15d4 - Graphic MMU LUT entry 186 high"]
    pub lut186h: crate::Reg<lut186h::LUT186H_SPEC>,
    #[doc = "0x15d8 - Graphic MMU LUT entry 187 low"]
    pub lut187l: crate::Reg<lut187l::LUT187L_SPEC>,
    #[doc = "0x15dc - Graphic MMU LUT entry 187 high"]
    pub lut187h: crate::Reg<lut187h::LUT187H_SPEC>,
    #[doc = "0x15e0 - Graphic MMU LUT entry 188 low"]
    pub lut188l: crate::Reg<lut188l::LUT188L_SPEC>,
    #[doc = "0x15e4 - Graphic MMU LUT entry 188 high"]
    pub lut188h: crate::Reg<lut188h::LUT188H_SPEC>,
    #[doc = "0x15e8 - Graphic MMU LUT entry 189 low"]
    pub lut189l: crate::Reg<lut189l::LUT189L_SPEC>,
    #[doc = "0x15ec - Graphic MMU LUT entry 189 high"]
    pub lut189h: crate::Reg<lut189h::LUT189H_SPEC>,
    #[doc = "0x15f0 - Graphic MMU LUT entry 190 low"]
    pub lut190l: crate::Reg<lut190l::LUT190L_SPEC>,
    #[doc = "0x15f4 - Graphic MMU LUT entry 190 high"]
    pub lut190h: crate::Reg<lut190h::LUT190H_SPEC>,
    #[doc = "0x15f8 - Graphic MMU LUT entry 191 low"]
    pub lut191l: crate::Reg<lut191l::LUT191L_SPEC>,
    #[doc = "0x15fc - Graphic MMU LUT entry 191 high"]
    pub lut191h: crate::Reg<lut191h::LUT191H_SPEC>,
    #[doc = "0x1600 - Graphic MMU LUT entry 192 low"]
    pub lut192l: crate::Reg<lut192l::LUT192L_SPEC>,
    #[doc = "0x1604 - Graphic MMU LUT entry 192 high"]
    pub lut192h: crate::Reg<lut192h::LUT192H_SPEC>,
    #[doc = "0x1608 - Graphic MMU LUT entry 193 low"]
    pub lut193l: crate::Reg<lut193l::LUT193L_SPEC>,
    #[doc = "0x160c - Graphic MMU LUT entry 193 high"]
    pub lut193h: crate::Reg<lut193h::LUT193H_SPEC>,
    #[doc = "0x1610 - Graphic MMU LUT entry 194 low"]
    pub lut194l: crate::Reg<lut194l::LUT194L_SPEC>,
    #[doc = "0x1614 - Graphic MMU LUT entry 194 high"]
    pub lut194h: crate::Reg<lut194h::LUT194H_SPEC>,
    #[doc = "0x1618 - Graphic MMU LUT entry 195 low"]
    pub lut195l: crate::Reg<lut195l::LUT195L_SPEC>,
    #[doc = "0x161c - Graphic MMU LUT entry 195 high"]
    pub lut195h: crate::Reg<lut195h::LUT195H_SPEC>,
    #[doc = "0x1620 - Graphic MMU LUT entry 196 low"]
    pub lut196l: crate::Reg<lut196l::LUT196L_SPEC>,
    #[doc = "0x1624 - Graphic MMU LUT entry 196 high"]
    pub lut196h: crate::Reg<lut196h::LUT196H_SPEC>,
    #[doc = "0x1628 - Graphic MMU LUT entry 197 low"]
    pub lut197l: crate::Reg<lut197l::LUT197L_SPEC>,
    #[doc = "0x162c - Graphic MMU LUT entry 197 high"]
    pub lut197h: crate::Reg<lut197h::LUT197H_SPEC>,
    #[doc = "0x1630 - Graphic MMU LUT entry 198 low"]
    pub lut198l: crate::Reg<lut198l::LUT198L_SPEC>,
    #[doc = "0x1634 - Graphic MMU LUT entry 198 high"]
    pub lut198h: crate::Reg<lut198h::LUT198H_SPEC>,
    #[doc = "0x1638 - Graphic MMU LUT entry 199 low"]
    pub lut199l: crate::Reg<lut199l::LUT199L_SPEC>,
    #[doc = "0x163c - Graphic MMU LUT entry 199 high"]
    pub lut199h: crate::Reg<lut199h::LUT199H_SPEC>,
    #[doc = "0x1640 - Graphic MMU LUT entry 200 low"]
    pub lut200l: crate::Reg<lut200l::LUT200L_SPEC>,
    #[doc = "0x1644 - Graphic MMU LUT entry 200 high"]
    pub lut200h: crate::Reg<lut200h::LUT200H_SPEC>,
    #[doc = "0x1648 - Graphic MMU LUT entry 201 low"]
    pub lut201l: crate::Reg<lut201l::LUT201L_SPEC>,
    #[doc = "0x164c - Graphic MMU LUT entry 201 high"]
    pub lut201h: crate::Reg<lut201h::LUT201H_SPEC>,
    #[doc = "0x1650 - Graphic MMU LUT entry 202 low"]
    pub lut202l: crate::Reg<lut202l::LUT202L_SPEC>,
    #[doc = "0x1654 - Graphic MMU LUT entry 202 high"]
    pub lut202h: crate::Reg<lut202h::LUT202H_SPEC>,
    #[doc = "0x1658 - Graphic MMU LUT entry 203 low"]
    pub lut203l: crate::Reg<lut203l::LUT203L_SPEC>,
    #[doc = "0x165c - Graphic MMU LUT entry 203 high"]
    pub lut203h: crate::Reg<lut203h::LUT203H_SPEC>,
    #[doc = "0x1660 - Graphic MMU LUT entry 204 low"]
    pub lut204l: crate::Reg<lut204l::LUT204L_SPEC>,
    #[doc = "0x1664 - Graphic MMU LUT entry 204 high"]
    pub lut204h: crate::Reg<lut204h::LUT204H_SPEC>,
    #[doc = "0x1668 - Graphic MMU LUT entry 205 low"]
    pub lut205l: crate::Reg<lut205l::LUT205L_SPEC>,
    #[doc = "0x166c - Graphic MMU LUT entry 205 high"]
    pub lut205h: crate::Reg<lut205h::LUT205H_SPEC>,
    #[doc = "0x1670 - Graphic MMU LUT entry 206 low"]
    pub lut206l: crate::Reg<lut206l::LUT206L_SPEC>,
    #[doc = "0x1674 - Graphic MMU LUT entry 206 high"]
    pub lut206h: crate::Reg<lut206h::LUT206H_SPEC>,
    #[doc = "0x1678 - Graphic MMU LUT entry 207 low"]
    pub lut207l: crate::Reg<lut207l::LUT207L_SPEC>,
    #[doc = "0x167c - Graphic MMU LUT entry 207 high"]
    pub lut207h: crate::Reg<lut207h::LUT207H_SPEC>,
    #[doc = "0x1680 - Graphic MMU LUT entry 208 low"]
    pub lut208l: crate::Reg<lut208l::LUT208L_SPEC>,
    #[doc = "0x1684 - Graphic MMU LUT entry 208 high"]
    pub lut208h: crate::Reg<lut208h::LUT208H_SPEC>,
    #[doc = "0x1688 - Graphic MMU LUT entry 209 low"]
    pub lut209l: crate::Reg<lut209l::LUT209L_SPEC>,
    #[doc = "0x168c - Graphic MMU LUT entry 209 high"]
    pub lut209h: crate::Reg<lut209h::LUT209H_SPEC>,
    #[doc = "0x1690 - Graphic MMU LUT entry 210 low"]
    pub lut210l: crate::Reg<lut210l::LUT210L_SPEC>,
    #[doc = "0x1694 - Graphic MMU LUT entry 210 high"]
    pub lut210h: crate::Reg<lut210h::LUT210H_SPEC>,
    #[doc = "0x1698 - Graphic MMU LUT entry 211 low"]
    pub lut211l: crate::Reg<lut211l::LUT211L_SPEC>,
    #[doc = "0x169c - Graphic MMU LUT entry 211 high"]
    pub lut211h: crate::Reg<lut211h::LUT211H_SPEC>,
    #[doc = "0x16a0 - Graphic MMU LUT entry 212 low"]
    pub lut212l: crate::Reg<lut212l::LUT212L_SPEC>,
    #[doc = "0x16a4 - Graphic MMU LUT entry 212 high"]
    pub lut212h: crate::Reg<lut212h::LUT212H_SPEC>,
    #[doc = "0x16a8 - Graphic MMU LUT entry 213 low"]
    pub lut213l: crate::Reg<lut213l::LUT213L_SPEC>,
    #[doc = "0x16ac - Graphic MMU LUT entry 213 high"]
    pub lut213h: crate::Reg<lut213h::LUT213H_SPEC>,
    #[doc = "0x16b0 - Graphic MMU LUT entry 214 low"]
    pub lut214l: crate::Reg<lut214l::LUT214L_SPEC>,
    #[doc = "0x16b4 - Graphic MMU LUT entry 214 high"]
    pub lut214h: crate::Reg<lut214h::LUT214H_SPEC>,
    #[doc = "0x16b8 - Graphic MMU LUT entry 215 low"]
    pub lut215l: crate::Reg<lut215l::LUT215L_SPEC>,
    #[doc = "0x16bc - Graphic MMU LUT entry 215 high"]
    pub lut215h: crate::Reg<lut215h::LUT215H_SPEC>,
    #[doc = "0x16c0 - Graphic MMU LUT entry 216 low"]
    pub lut216l: crate::Reg<lut216l::LUT216L_SPEC>,
    #[doc = "0x16c4 - Graphic MMU LUT entry 216 high"]
    pub lut216h: crate::Reg<lut216h::LUT216H_SPEC>,
    #[doc = "0x16c8 - Graphic MMU LUT entry 217 low"]
    pub lut217l: crate::Reg<lut217l::LUT217L_SPEC>,
    #[doc = "0x16cc - Graphic MMU LUT entry 217 high"]
    pub lut217h: crate::Reg<lut217h::LUT217H_SPEC>,
    #[doc = "0x16d0 - Graphic MMU LUT entry 218 low"]
    pub lut218l: crate::Reg<lut218l::LUT218L_SPEC>,
    #[doc = "0x16d4 - Graphic MMU LUT entry 218 high"]
    pub lut218h: crate::Reg<lut218h::LUT218H_SPEC>,
    #[doc = "0x16d8 - Graphic MMU LUT entry 219 low"]
    pub lut219l: crate::Reg<lut219l::LUT219L_SPEC>,
    #[doc = "0x16dc - Graphic MMU LUT entry 219 high"]
    pub lut219h: crate::Reg<lut219h::LUT219H_SPEC>,
    #[doc = "0x16e0 - Graphic MMU LUT entry 220 low"]
    pub lut220l: crate::Reg<lut220l::LUT220L_SPEC>,
    #[doc = "0x16e4 - Graphic MMU LUT entry 220 high"]
    pub lut220h: crate::Reg<lut220h::LUT220H_SPEC>,
    #[doc = "0x16e8 - Graphic MMU LUT entry 221 low"]
    pub lut221l: crate::Reg<lut221l::LUT221L_SPEC>,
    #[doc = "0x16ec - Graphic MMU LUT entry 221 high"]
    pub lut221h: crate::Reg<lut221h::LUT221H_SPEC>,
    #[doc = "0x16f0 - Graphic MMU LUT entry 222 low"]
    pub lut222l: crate::Reg<lut222l::LUT222L_SPEC>,
    #[doc = "0x16f4 - Graphic MMU LUT entry 222 high"]
    pub lut222h: crate::Reg<lut222h::LUT222H_SPEC>,
    #[doc = "0x16f8 - Graphic MMU LUT entry 223 low"]
    pub lut223l: crate::Reg<lut223l::LUT223L_SPEC>,
    #[doc = "0x16fc - Graphic MMU LUT entry 223 high"]
    pub lut223h: crate::Reg<lut223h::LUT223H_SPEC>,
    #[doc = "0x1700 - Graphic MMU LUT entry 224 low"]
    pub lut224l: crate::Reg<lut224l::LUT224L_SPEC>,
    #[doc = "0x1704 - Graphic MMU LUT entry 224 high"]
    pub lut224h: crate::Reg<lut224h::LUT224H_SPEC>,
    #[doc = "0x1708 - Graphic MMU LUT entry 225 low"]
    pub lut225l: crate::Reg<lut225l::LUT225L_SPEC>,
    #[doc = "0x170c - Graphic MMU LUT entry 225 high"]
    pub lut225h: crate::Reg<lut225h::LUT225H_SPEC>,
    #[doc = "0x1710 - Graphic MMU LUT entry 226 low"]
    pub lut226l: crate::Reg<lut226l::LUT226L_SPEC>,
    #[doc = "0x1714 - Graphic MMU LUT entry 226 high"]
    pub lut226h: crate::Reg<lut226h::LUT226H_SPEC>,
    #[doc = "0x1718 - Graphic MMU LUT entry 227 low"]
    pub lut227l: crate::Reg<lut227l::LUT227L_SPEC>,
    #[doc = "0x171c - Graphic MMU LUT entry 227 high"]
    pub lut227h: crate::Reg<lut227h::LUT227H_SPEC>,
    #[doc = "0x1720 - Graphic MMU LUT entry 228 low"]
    pub lut228l: crate::Reg<lut228l::LUT228L_SPEC>,
    #[doc = "0x1724 - Graphic MMU LUT entry 228 high"]
    pub lut228h: crate::Reg<lut228h::LUT228H_SPEC>,
    #[doc = "0x1728 - Graphic MMU LUT entry 229 low"]
    pub lut229l: crate::Reg<lut229l::LUT229L_SPEC>,
    #[doc = "0x172c - Graphic MMU LUT entry 229 high"]
    pub lut229h: crate::Reg<lut229h::LUT229H_SPEC>,
    #[doc = "0x1730 - Graphic MMU LUT entry 230 low"]
    pub lut230l: crate::Reg<lut230l::LUT230L_SPEC>,
    #[doc = "0x1734 - Graphic MMU LUT entry 230 high"]
    pub lut230h: crate::Reg<lut230h::LUT230H_SPEC>,
    #[doc = "0x1738 - Graphic MMU LUT entry 231 low"]
    pub lut231l: crate::Reg<lut231l::LUT231L_SPEC>,
    #[doc = "0x173c - Graphic MMU LUT entry 231 high"]
    pub lut231h: crate::Reg<lut231h::LUT231H_SPEC>,
    #[doc = "0x1740 - Graphic MMU LUT entry 232 low"]
    pub lut232l: crate::Reg<lut232l::LUT232L_SPEC>,
    #[doc = "0x1744 - Graphic MMU LUT entry 232 high"]
    pub lut232h: crate::Reg<lut232h::LUT232H_SPEC>,
    #[doc = "0x1748 - Graphic MMU LUT entry 233 low"]
    pub lut233l: crate::Reg<lut233l::LUT233L_SPEC>,
    #[doc = "0x174c - Graphic MMU LUT entry 233 high"]
    pub lut233h: crate::Reg<lut233h::LUT233H_SPEC>,
    #[doc = "0x1750 - Graphic MMU LUT entry 234 low"]
    pub lut234l: crate::Reg<lut234l::LUT234L_SPEC>,
    #[doc = "0x1754 - Graphic MMU LUT entry 234 high"]
    pub lut234h: crate::Reg<lut234h::LUT234H_SPEC>,
    #[doc = "0x1758 - Graphic MMU LUT entry 235 low"]
    pub lut235l: crate::Reg<lut235l::LUT235L_SPEC>,
    #[doc = "0x175c - Graphic MMU LUT entry 235 high"]
    pub lut235h: crate::Reg<lut235h::LUT235H_SPEC>,
    #[doc = "0x1760 - Graphic MMU LUT entry 236 low"]
    pub lut236l: crate::Reg<lut236l::LUT236L_SPEC>,
    #[doc = "0x1764 - Graphic MMU LUT entry 236 high"]
    pub lut236h: crate::Reg<lut236h::LUT236H_SPEC>,
    #[doc = "0x1768 - Graphic MMU LUT entry 237 low"]
    pub lut237l: crate::Reg<lut237l::LUT237L_SPEC>,
    #[doc = "0x176c - Graphic MMU LUT entry 237 high"]
    pub lut237h: crate::Reg<lut237h::LUT237H_SPEC>,
    #[doc = "0x1770 - Graphic MMU LUT entry 238 low"]
    pub lut238l: crate::Reg<lut238l::LUT238L_SPEC>,
    #[doc = "0x1774 - Graphic MMU LUT entry 238 high"]
    pub lut238h: crate::Reg<lut238h::LUT238H_SPEC>,
    #[doc = "0x1778 - Graphic MMU LUT entry 239 low"]
    pub lut239l: crate::Reg<lut239l::LUT239L_SPEC>,
    #[doc = "0x177c - Graphic MMU LUT entry 239 high"]
    pub lut239h: crate::Reg<lut239h::LUT239H_SPEC>,
    #[doc = "0x1780 - Graphic MMU LUT entry 240 low"]
    pub lut240l: crate::Reg<lut240l::LUT240L_SPEC>,
    #[doc = "0x1784 - Graphic MMU LUT entry 240 high"]
    pub lut240h: crate::Reg<lut240h::LUT240H_SPEC>,
    #[doc = "0x1788 - Graphic MMU LUT entry 241 low"]
    pub lut241l: crate::Reg<lut241l::LUT241L_SPEC>,
    #[doc = "0x178c - Graphic MMU LUT entry 241 high"]
    pub lut241h: crate::Reg<lut241h::LUT241H_SPEC>,
    #[doc = "0x1790 - Graphic MMU LUT entry 242 low"]
    pub lut242l: crate::Reg<lut242l::LUT242L_SPEC>,
    #[doc = "0x1794 - Graphic MMU LUT entry 242 high"]
    pub lut242h: crate::Reg<lut242h::LUT242H_SPEC>,
    #[doc = "0x1798 - Graphic MMU LUT entry 243 low"]
    pub lut243l: crate::Reg<lut243l::LUT243L_SPEC>,
    #[doc = "0x179c - Graphic MMU LUT entry 243 high"]
    pub lut243h: crate::Reg<lut243h::LUT243H_SPEC>,
    #[doc = "0x17a0 - Graphic MMU LUT entry 244 low"]
    pub lut244l: crate::Reg<lut244l::LUT244L_SPEC>,
    #[doc = "0x17a4 - Graphic MMU LUT entry 244 high"]
    pub lut244h: crate::Reg<lut244h::LUT244H_SPEC>,
    #[doc = "0x17a8 - Graphic MMU LUT entry 245 low"]
    pub lut245l: crate::Reg<lut245l::LUT245L_SPEC>,
    #[doc = "0x17ac - Graphic MMU LUT entry 245 high"]
    pub lut245h: crate::Reg<lut245h::LUT245H_SPEC>,
    #[doc = "0x17b0 - Graphic MMU LUT entry 246 low"]
    pub lut246l: crate::Reg<lut246l::LUT246L_SPEC>,
    #[doc = "0x17b4 - Graphic MMU LUT entry 246 high"]
    pub lut246h: crate::Reg<lut246h::LUT246H_SPEC>,
    #[doc = "0x17b8 - Graphic MMU LUT entry 247 low"]
    pub lut247l: crate::Reg<lut247l::LUT247L_SPEC>,
    #[doc = "0x17bc - Graphic MMU LUT entry 247 high"]
    pub lut247h: crate::Reg<lut247h::LUT247H_SPEC>,
    #[doc = "0x17c0 - Graphic MMU LUT entry 248 low"]
    pub lut248l: crate::Reg<lut248l::LUT248L_SPEC>,
    #[doc = "0x17c4 - Graphic MMU LUT entry 248 high"]
    pub lut248h: crate::Reg<lut248h::LUT248H_SPEC>,
    #[doc = "0x17c8 - Graphic MMU LUT entry 249 low"]
    pub lut249l: crate::Reg<lut249l::LUT249L_SPEC>,
    #[doc = "0x17cc - Graphic MMU LUT entry 249 high"]
    pub lut249h: crate::Reg<lut249h::LUT249H_SPEC>,
    #[doc = "0x17d0 - Graphic MMU LUT entry 250 low"]
    pub lut250l: crate::Reg<lut250l::LUT250L_SPEC>,
    #[doc = "0x17d4 - Graphic MMU LUT entry 250 high"]
    pub lut250h: crate::Reg<lut250h::LUT250H_SPEC>,
    #[doc = "0x17d8 - Graphic MMU LUT entry 251 low"]
    pub lut251l: crate::Reg<lut251l::LUT251L_SPEC>,
    #[doc = "0x17dc - Graphic MMU LUT entry 251 high"]
    pub lut251h: crate::Reg<lut251h::LUT251H_SPEC>,
    #[doc = "0x17e0 - Graphic MMU LUT entry 252 low"]
    pub lut252l: crate::Reg<lut252l::LUT252L_SPEC>,
    #[doc = "0x17e4 - Graphic MMU LUT entry 252 high"]
    pub lut252h: crate::Reg<lut252h::LUT252H_SPEC>,
    #[doc = "0x17e8 - Graphic MMU LUT entry 253 low"]
    pub lut253l: crate::Reg<lut253l::LUT253L_SPEC>,
    #[doc = "0x17ec - Graphic MMU LUT entry 253 high"]
    pub lut253h: crate::Reg<lut253h::LUT253H_SPEC>,
    #[doc = "0x17f0 - Graphic MMU LUT entry 254 low"]
    pub lut254l: crate::Reg<lut254l::LUT254L_SPEC>,
    #[doc = "0x17f4 - Graphic MMU LUT entry 254 high"]
    pub lut254h: crate::Reg<lut254h::LUT254H_SPEC>,
    #[doc = "0x17f8 - Graphic MMU LUT entry 255 low"]
    pub lut255l: crate::Reg<lut255l::LUT255L_SPEC>,
    #[doc = "0x17fc - Graphic MMU LUT entry 255 high"]
    pub lut255h: crate::Reg<lut255h::LUT255H_SPEC>,
    #[doc = "0x1800 - Graphic MMU LUT entry 256 low"]
    pub lut256l: crate::Reg<lut256l::LUT256L_SPEC>,
    #[doc = "0x1804 - Graphic MMU LUT entry 256 high"]
    pub lut256h: crate::Reg<lut256h::LUT256H_SPEC>,
    #[doc = "0x1808 - Graphic MMU LUT entry 257 low"]
    pub lut257l: crate::Reg<lut257l::LUT257L_SPEC>,
    #[doc = "0x180c - Graphic MMU LUT entry 257 high"]
    pub lut257h: crate::Reg<lut257h::LUT257H_SPEC>,
    #[doc = "0x1810 - Graphic MMU LUT entry 258 low"]
    pub lut258l: crate::Reg<lut258l::LUT258L_SPEC>,
    #[doc = "0x1814 - Graphic MMU LUT entry 258 high"]
    pub lut258h: crate::Reg<lut258h::LUT258H_SPEC>,
    #[doc = "0x1818 - Graphic MMU LUT entry 259 low"]
    pub lut259l: crate::Reg<lut259l::LUT259L_SPEC>,
    #[doc = "0x181c - Graphic MMU LUT entry 259 high"]
    pub lut259h: crate::Reg<lut259h::LUT259H_SPEC>,
    #[doc = "0x1820 - Graphic MMU LUT entry 260 low"]
    pub lut260l: crate::Reg<lut260l::LUT260L_SPEC>,
    #[doc = "0x1824 - Graphic MMU LUT entry 260 high"]
    pub lut260h: crate::Reg<lut260h::LUT260H_SPEC>,
    #[doc = "0x1828 - Graphic MMU LUT entry 261 low"]
    pub lut261l: crate::Reg<lut261l::LUT261L_SPEC>,
    #[doc = "0x182c - Graphic MMU LUT entry 261 high"]
    pub lut261h: crate::Reg<lut261h::LUT261H_SPEC>,
    #[doc = "0x1830 - Graphic MMU LUT entry 262 low"]
    pub lut262l: crate::Reg<lut262l::LUT262L_SPEC>,
    #[doc = "0x1834 - Graphic MMU LUT entry 262 high"]
    pub lut262h: crate::Reg<lut262h::LUT262H_SPEC>,
    #[doc = "0x1838 - Graphic MMU LUT entry 263 low"]
    pub lut263l: crate::Reg<lut263l::LUT263L_SPEC>,
    #[doc = "0x183c - Graphic MMU LUT entry 263 high"]
    pub lut263h: crate::Reg<lut263h::LUT263H_SPEC>,
    #[doc = "0x1840 - Graphic MMU LUT entry 264 low"]
    pub lut264l: crate::Reg<lut264l::LUT264L_SPEC>,
    #[doc = "0x1844 - Graphic MMU LUT entry 264 high"]
    pub lut264h: crate::Reg<lut264h::LUT264H_SPEC>,
    #[doc = "0x1848 - Graphic MMU LUT entry 265 low"]
    pub lut265l: crate::Reg<lut265l::LUT265L_SPEC>,
    #[doc = "0x184c - Graphic MMU LUT entry 265 high"]
    pub lut265h: crate::Reg<lut265h::LUT265H_SPEC>,
    #[doc = "0x1850 - Graphic MMU LUT entry 266 low"]
    pub lut266l: crate::Reg<lut266l::LUT266L_SPEC>,
    #[doc = "0x1854 - Graphic MMU LUT entry 266 high"]
    pub lut266h: crate::Reg<lut266h::LUT266H_SPEC>,
    #[doc = "0x1858 - Graphic MMU LUT entry 267 low"]
    pub lut267l: crate::Reg<lut267l::LUT267L_SPEC>,
    #[doc = "0x185c - Graphic MMU LUT entry 267 high"]
    pub lut267h: crate::Reg<lut267h::LUT267H_SPEC>,
    #[doc = "0x1860 - Graphic MMU LUT entry 268 low"]
    pub lut268l: crate::Reg<lut268l::LUT268L_SPEC>,
    #[doc = "0x1864 - Graphic MMU LUT entry 268 high"]
    pub lut268h: crate::Reg<lut268h::LUT268H_SPEC>,
    #[doc = "0x1868 - Graphic MMU LUT entry 269 low"]
    pub lut269l: crate::Reg<lut269l::LUT269L_SPEC>,
    #[doc = "0x186c - Graphic MMU LUT entry 269 high"]
    pub lut269h: crate::Reg<lut269h::LUT269H_SPEC>,
    #[doc = "0x1870 - Graphic MMU LUT entry 270 low"]
    pub lut270l: crate::Reg<lut270l::LUT270L_SPEC>,
    #[doc = "0x1874 - Graphic MMU LUT entry 270 high"]
    pub lut270h: crate::Reg<lut270h::LUT270H_SPEC>,
    #[doc = "0x1878 - Graphic MMU LUT entry 271 low"]
    pub lut271l: crate::Reg<lut271l::LUT271L_SPEC>,
    #[doc = "0x187c - Graphic MMU LUT entry 271 high"]
    pub lut271h: crate::Reg<lut271h::LUT271H_SPEC>,
    #[doc = "0x1880 - Graphic MMU LUT entry 272 low"]
    pub lut272l: crate::Reg<lut272l::LUT272L_SPEC>,
    #[doc = "0x1884 - Graphic MMU LUT entry 272 high"]
    pub lut272h: crate::Reg<lut272h::LUT272H_SPEC>,
    #[doc = "0x1888 - Graphic MMU LUT entry 273 low"]
    pub lut273l: crate::Reg<lut273l::LUT273L_SPEC>,
    #[doc = "0x188c - Graphic MMU LUT entry 273 high"]
    pub lut273h: crate::Reg<lut273h::LUT273H_SPEC>,
    #[doc = "0x1890 - Graphic MMU LUT entry 274 low"]
    pub lut274l: crate::Reg<lut274l::LUT274L_SPEC>,
    #[doc = "0x1894 - Graphic MMU LUT entry 274 high"]
    pub lut274h: crate::Reg<lut274h::LUT274H_SPEC>,
    #[doc = "0x1898 - Graphic MMU LUT entry 275 low"]
    pub lut275l: crate::Reg<lut275l::LUT275L_SPEC>,
    #[doc = "0x189c - Graphic MMU LUT entry 275 high"]
    pub lut275h: crate::Reg<lut275h::LUT275H_SPEC>,
    #[doc = "0x18a0 - Graphic MMU LUT entry 276 low"]
    pub lut276l: crate::Reg<lut276l::LUT276L_SPEC>,
    #[doc = "0x18a4 - Graphic MMU LUT entry 276 high"]
    pub lut276h: crate::Reg<lut276h::LUT276H_SPEC>,
    #[doc = "0x18a8 - Graphic MMU LUT entry 277 low"]
    pub lut277l: crate::Reg<lut277l::LUT277L_SPEC>,
    #[doc = "0x18ac - Graphic MMU LUT entry 277 high"]
    pub lut277h: crate::Reg<lut277h::LUT277H_SPEC>,
    #[doc = "0x18b0 - Graphic MMU LUT entry 278 low"]
    pub lut278l: crate::Reg<lut278l::LUT278L_SPEC>,
    #[doc = "0x18b4 - Graphic MMU LUT entry 278 high"]
    pub lut278h: crate::Reg<lut278h::LUT278H_SPEC>,
    #[doc = "0x18b8 - Graphic MMU LUT entry 279 low"]
    pub lut279l: crate::Reg<lut279l::LUT279L_SPEC>,
    #[doc = "0x18bc - Graphic MMU LUT entry 279 high"]
    pub lut279h: crate::Reg<lut279h::LUT279H_SPEC>,
    #[doc = "0x18c0 - Graphic MMU LUT entry 280 low"]
    pub lut280l: crate::Reg<lut280l::LUT280L_SPEC>,
    #[doc = "0x18c4 - Graphic MMU LUT entry 280 high"]
    pub lut280h: crate::Reg<lut280h::LUT280H_SPEC>,
    #[doc = "0x18c8 - Graphic MMU LUT entry 281 low"]
    pub lut281l: crate::Reg<lut281l::LUT281L_SPEC>,
    #[doc = "0x18cc - Graphic MMU LUT entry 281 high"]
    pub lut281h: crate::Reg<lut281h::LUT281H_SPEC>,
    #[doc = "0x18d0 - Graphic MMU LUT entry 282 low"]
    pub lut282l: crate::Reg<lut282l::LUT282L_SPEC>,
    #[doc = "0x18d4 - Graphic MMU LUT entry 282 high"]
    pub lut282h: crate::Reg<lut282h::LUT282H_SPEC>,
    #[doc = "0x18d8 - Graphic MMU LUT entry 283 low"]
    pub lut283l: crate::Reg<lut283l::LUT283L_SPEC>,
    #[doc = "0x18dc - Graphic MMU LUT entry 283 high"]
    pub lut283h: crate::Reg<lut283h::LUT283H_SPEC>,
    #[doc = "0x18e0 - Graphic MMU LUT entry 284 low"]
    pub lut284l: crate::Reg<lut284l::LUT284L_SPEC>,
    #[doc = "0x18e4 - Graphic MMU LUT entry 284 high"]
    pub lut284h: crate::Reg<lut284h::LUT284H_SPEC>,
    #[doc = "0x18e8 - Graphic MMU LUT entry 285 low"]
    pub lut285l: crate::Reg<lut285l::LUT285L_SPEC>,
    #[doc = "0x18ec - Graphic MMU LUT entry 285 high"]
    pub lut285h: crate::Reg<lut285h::LUT285H_SPEC>,
    #[doc = "0x18f0 - Graphic MMU LUT entry 286 low"]
    pub lut286l: crate::Reg<lut286l::LUT286L_SPEC>,
    #[doc = "0x18f4 - Graphic MMU LUT entry 286 high"]
    pub lut286h: crate::Reg<lut286h::LUT286H_SPEC>,
    #[doc = "0x18f8 - Graphic MMU LUT entry 287 low"]
    pub lut287l: crate::Reg<lut287l::LUT287L_SPEC>,
    #[doc = "0x18fc - Graphic MMU LUT entry 287 high"]
    pub lut287h: crate::Reg<lut287h::LUT287H_SPEC>,
    #[doc = "0x1900 - Graphic MMU LUT entry 288 low"]
    pub lut288l: crate::Reg<lut288l::LUT288L_SPEC>,
    #[doc = "0x1904 - Graphic MMU LUT entry 288 high"]
    pub lut288h: crate::Reg<lut288h::LUT288H_SPEC>,
    #[doc = "0x1908 - Graphic MMU LUT entry 289 low"]
    pub lut289l: crate::Reg<lut289l::LUT289L_SPEC>,
    #[doc = "0x190c - Graphic MMU LUT entry 289 high"]
    pub lut289h: crate::Reg<lut289h::LUT289H_SPEC>,
    #[doc = "0x1910 - Graphic MMU LUT entry 290 low"]
    pub lut290l: crate::Reg<lut290l::LUT290L_SPEC>,
    #[doc = "0x1914 - Graphic MMU LUT entry 290 high"]
    pub lut290h: crate::Reg<lut290h::LUT290H_SPEC>,
    #[doc = "0x1918 - Graphic MMU LUT entry 291 low"]
    pub lut291l: crate::Reg<lut291l::LUT291L_SPEC>,
    #[doc = "0x191c - Graphic MMU LUT entry 291 high"]
    pub lut291h: crate::Reg<lut291h::LUT291H_SPEC>,
    #[doc = "0x1920 - Graphic MMU LUT entry 292 low"]
    pub lut292l: crate::Reg<lut292l::LUT292L_SPEC>,
    #[doc = "0x1924 - Graphic MMU LUT entry 292 high"]
    pub lut292h: crate::Reg<lut292h::LUT292H_SPEC>,
    #[doc = "0x1928 - Graphic MMU LUT entry 293 low"]
    pub lut293l: crate::Reg<lut293l::LUT293L_SPEC>,
    #[doc = "0x192c - Graphic MMU LUT entry 293 high"]
    pub lut293h: crate::Reg<lut293h::LUT293H_SPEC>,
    #[doc = "0x1930 - Graphic MMU LUT entry 294 low"]
    pub lut294l: crate::Reg<lut294l::LUT294L_SPEC>,
    #[doc = "0x1934 - Graphic MMU LUT entry 294 high"]
    pub lut294h: crate::Reg<lut294h::LUT294H_SPEC>,
    #[doc = "0x1938 - Graphic MMU LUT entry 295 low"]
    pub lut295l: crate::Reg<lut295l::LUT295L_SPEC>,
    #[doc = "0x193c - Graphic MMU LUT entry 295 high"]
    pub lut295h: crate::Reg<lut295h::LUT295H_SPEC>,
    #[doc = "0x1940 - Graphic MMU LUT entry 296 low"]
    pub lut296l: crate::Reg<lut296l::LUT296L_SPEC>,
    #[doc = "0x1944 - Graphic MMU LUT entry 296 high"]
    pub lut296h: crate::Reg<lut296h::LUT296H_SPEC>,
    #[doc = "0x1948 - Graphic MMU LUT entry 297 low"]
    pub lut297l: crate::Reg<lut297l::LUT297L_SPEC>,
    #[doc = "0x194c - Graphic MMU LUT entry 297 high"]
    pub lut297h: crate::Reg<lut297h::LUT297H_SPEC>,
    #[doc = "0x1950 - Graphic MMU LUT entry 298 low"]
    pub lut298l: crate::Reg<lut298l::LUT298L_SPEC>,
    #[doc = "0x1954 - Graphic MMU LUT entry 298 high"]
    pub lut298h: crate::Reg<lut298h::LUT298H_SPEC>,
    #[doc = "0x1958 - Graphic MMU LUT entry 299 low"]
    pub lut299l: crate::Reg<lut299l::LUT299L_SPEC>,
    #[doc = "0x195c - Graphic MMU LUT entry 299 high"]
    pub lut299h: crate::Reg<lut299h::LUT299H_SPEC>,
    #[doc = "0x1960 - Graphic MMU LUT entry 300 low"]
    pub lut300l: crate::Reg<lut300l::LUT300L_SPEC>,
    #[doc = "0x1964 - Graphic MMU LUT entry 300 high"]
    pub lut300h: crate::Reg<lut300h::LUT300H_SPEC>,
    #[doc = "0x1968 - Graphic MMU LUT entry 301 low"]
    pub lut301l: crate::Reg<lut301l::LUT301L_SPEC>,
    #[doc = "0x196c - Graphic MMU LUT entry 301 high"]
    pub lut301h: crate::Reg<lut301h::LUT301H_SPEC>,
    #[doc = "0x1970 - Graphic MMU LUT entry 302 low"]
    pub lut302l: crate::Reg<lut302l::LUT302L_SPEC>,
    #[doc = "0x1974 - Graphic MMU LUT entry 302 high"]
    pub lut302h: crate::Reg<lut302h::LUT302H_SPEC>,
    #[doc = "0x1978 - Graphic MMU LUT entry 303 low"]
    pub lut303l: crate::Reg<lut303l::LUT303L_SPEC>,
    #[doc = "0x197c - Graphic MMU LUT entry 303 high"]
    pub lut303h: crate::Reg<lut303h::LUT303H_SPEC>,
    #[doc = "0x1980 - Graphic MMU LUT entry 304 low"]
    pub lut304l: crate::Reg<lut304l::LUT304L_SPEC>,
    #[doc = "0x1984 - Graphic MMU LUT entry 304 high"]
    pub lut304h: crate::Reg<lut304h::LUT304H_SPEC>,
    #[doc = "0x1988 - Graphic MMU LUT entry 305 low"]
    pub lut305l: crate::Reg<lut305l::LUT305L_SPEC>,
    #[doc = "0x198c - Graphic MMU LUT entry 305 high"]
    pub lut305h: crate::Reg<lut305h::LUT305H_SPEC>,
    #[doc = "0x1990 - Graphic MMU LUT entry 306 low"]
    pub lut306l: crate::Reg<lut306l::LUT306L_SPEC>,
    #[doc = "0x1994 - Graphic MMU LUT entry 306 high"]
    pub lut306h: crate::Reg<lut306h::LUT306H_SPEC>,
    #[doc = "0x1998 - Graphic MMU LUT entry 307 low"]
    pub lut307l: crate::Reg<lut307l::LUT307L_SPEC>,
    #[doc = "0x199c - Graphic MMU LUT entry 307 high"]
    pub lut307h: crate::Reg<lut307h::LUT307H_SPEC>,
    #[doc = "0x19a0 - Graphic MMU LUT entry 308 low"]
    pub lut308l: crate::Reg<lut308l::LUT308L_SPEC>,
    #[doc = "0x19a4 - Graphic MMU LUT entry 308 high"]
    pub lut308h: crate::Reg<lut308h::LUT308H_SPEC>,
    #[doc = "0x19a8 - Graphic MMU LUT entry 309 low"]
    pub lut309l: crate::Reg<lut309l::LUT309L_SPEC>,
    #[doc = "0x19ac - Graphic MMU LUT entry 309 high"]
    pub lut309h: crate::Reg<lut309h::LUT309H_SPEC>,
    #[doc = "0x19b0 - Graphic MMU LUT entry 310 low"]
    pub lut310l: crate::Reg<lut310l::LUT310L_SPEC>,
    #[doc = "0x19b4 - Graphic MMU LUT entry 310 high"]
    pub lut310h: crate::Reg<lut310h::LUT310H_SPEC>,
    #[doc = "0x19b8 - Graphic MMU LUT entry 311 low"]
    pub lut311l: crate::Reg<lut311l::LUT311L_SPEC>,
    #[doc = "0x19bc - Graphic MMU LUT entry 311 high"]
    pub lut311h: crate::Reg<lut311h::LUT311H_SPEC>,
    #[doc = "0x19c0 - Graphic MMU LUT entry 312 low"]
    pub lut312l: crate::Reg<lut312l::LUT312L_SPEC>,
    #[doc = "0x19c4 - Graphic MMU LUT entry 312 high"]
    pub lut312h: crate::Reg<lut312h::LUT312H_SPEC>,
    #[doc = "0x19c8 - Graphic MMU LUT entry 313 low"]
    pub lut313l: crate::Reg<lut313l::LUT313L_SPEC>,
    #[doc = "0x19cc - Graphic MMU LUT entry 313 high"]
    pub lut313h: crate::Reg<lut313h::LUT313H_SPEC>,
    #[doc = "0x19d0 - Graphic MMU LUT entry 314 low"]
    pub lut314l: crate::Reg<lut314l::LUT314L_SPEC>,
    #[doc = "0x19d4 - Graphic MMU LUT entry 314 high"]
    pub lut314h: crate::Reg<lut314h::LUT314H_SPEC>,
    #[doc = "0x19d8 - Graphic MMU LUT entry 315 low"]
    pub lut315l: crate::Reg<lut315l::LUT315L_SPEC>,
    #[doc = "0x19dc - Graphic MMU LUT entry 315 high"]
    pub lut315h: crate::Reg<lut315h::LUT315H_SPEC>,
    #[doc = "0x19e0 - Graphic MMU LUT entry 316 low"]
    pub lut316l: crate::Reg<lut316l::LUT316L_SPEC>,
    #[doc = "0x19e4 - Graphic MMU LUT entry 316 high"]
    pub lut316h: crate::Reg<lut316h::LUT316H_SPEC>,
    #[doc = "0x19e8 - Graphic MMU LUT entry 317 low"]
    pub lut317l: crate::Reg<lut317l::LUT317L_SPEC>,
    #[doc = "0x19ec - Graphic MMU LUT entry 317 high"]
    pub lut317h: crate::Reg<lut317h::LUT317H_SPEC>,
    #[doc = "0x19f0 - Graphic MMU LUT entry 318 low"]
    pub lut318l: crate::Reg<lut318l::LUT318L_SPEC>,
    #[doc = "0x19f4 - Graphic MMU LUT entry 318 high"]
    pub lut318h: crate::Reg<lut318h::LUT318H_SPEC>,
    #[doc = "0x19f8 - Graphic MMU LUT entry 319 low"]
    pub lut319l: crate::Reg<lut319l::LUT319L_SPEC>,
    #[doc = "0x19fc - Graphic MMU LUT entry 319 high"]
    pub lut319h: crate::Reg<lut319h::LUT319H_SPEC>,
    #[doc = "0x1a00 - Graphic MMU LUT entry 320 low"]
    pub lut320l: crate::Reg<lut320l::LUT320L_SPEC>,
    #[doc = "0x1a04 - Graphic MMU LUT entry 320 high"]
    pub lut320h: crate::Reg<lut320h::LUT320H_SPEC>,
    #[doc = "0x1a08 - Graphic MMU LUT entry 321 low"]
    pub lut321l: crate::Reg<lut321l::LUT321L_SPEC>,
    #[doc = "0x1a0c - Graphic MMU LUT entry 321 high"]
    pub lut321h: crate::Reg<lut321h::LUT321H_SPEC>,
    #[doc = "0x1a10 - Graphic MMU LUT entry 322 low"]
    pub lut322l: crate::Reg<lut322l::LUT322L_SPEC>,
    #[doc = "0x1a14 - Graphic MMU LUT entry 322 high"]
    pub lut322h: crate::Reg<lut322h::LUT322H_SPEC>,
    #[doc = "0x1a18 - Graphic MMU LUT entry 323 low"]
    pub lut323l: crate::Reg<lut323l::LUT323L_SPEC>,
    #[doc = "0x1a1c - Graphic MMU LUT entry 323 high"]
    pub lut323h: crate::Reg<lut323h::LUT323H_SPEC>,
    #[doc = "0x1a20 - Graphic MMU LUT entry 324 low"]
    pub lut324l: crate::Reg<lut324l::LUT324L_SPEC>,
    #[doc = "0x1a24 - Graphic MMU LUT entry 324 high"]
    pub lut324h: crate::Reg<lut324h::LUT324H_SPEC>,
    #[doc = "0x1a28 - Graphic MMU LUT entry 325 low"]
    pub lut325l: crate::Reg<lut325l::LUT325L_SPEC>,
    #[doc = "0x1a2c - Graphic MMU LUT entry 325 high"]
    pub lut325h: crate::Reg<lut325h::LUT325H_SPEC>,
    #[doc = "0x1a30 - Graphic MMU LUT entry 326 low"]
    pub lut326l: crate::Reg<lut326l::LUT326L_SPEC>,
    #[doc = "0x1a34 - Graphic MMU LUT entry 326 high"]
    pub lut326h: crate::Reg<lut326h::LUT326H_SPEC>,
    #[doc = "0x1a38 - Graphic MMU LUT entry 327 low"]
    pub lut327l: crate::Reg<lut327l::LUT327L_SPEC>,
    #[doc = "0x1a3c - Graphic MMU LUT entry 327 high"]
    pub lut327h: crate::Reg<lut327h::LUT327H_SPEC>,
    #[doc = "0x1a40 - Graphic MMU LUT entry 328 low"]
    pub lut328l: crate::Reg<lut328l::LUT328L_SPEC>,
    #[doc = "0x1a44 - Graphic MMU LUT entry 328 high"]
    pub lut328h: crate::Reg<lut328h::LUT328H_SPEC>,
    #[doc = "0x1a48 - Graphic MMU LUT entry 329 low"]
    pub lut329l: crate::Reg<lut329l::LUT329L_SPEC>,
    #[doc = "0x1a4c - Graphic MMU LUT entry 329 high"]
    pub lut329h: crate::Reg<lut329h::LUT329H_SPEC>,
    #[doc = "0x1a50 - Graphic MMU LUT entry 330 low"]
    pub lut330l: crate::Reg<lut330l::LUT330L_SPEC>,
    #[doc = "0x1a54 - Graphic MMU LUT entry 330 high"]
    pub lut330h: crate::Reg<lut330h::LUT330H_SPEC>,
    #[doc = "0x1a58 - Graphic MMU LUT entry 331 low"]
    pub lut331l: crate::Reg<lut331l::LUT331L_SPEC>,
    #[doc = "0x1a5c - Graphic MMU LUT entry 331 high"]
    pub lut331h: crate::Reg<lut331h::LUT331H_SPEC>,
    #[doc = "0x1a60 - Graphic MMU LUT entry 332 low"]
    pub lut332l: crate::Reg<lut332l::LUT332L_SPEC>,
    #[doc = "0x1a64 - Graphic MMU LUT entry 332 high"]
    pub lut332h: crate::Reg<lut332h::LUT332H_SPEC>,
    #[doc = "0x1a68 - Graphic MMU LUT entry 333 low"]
    pub lut333l: crate::Reg<lut333l::LUT333L_SPEC>,
    #[doc = "0x1a6c - Graphic MMU LUT entry 333 high"]
    pub lut333h: crate::Reg<lut333h::LUT333H_SPEC>,
    #[doc = "0x1a70 - Graphic MMU LUT entry 334 low"]
    pub lut334l: crate::Reg<lut334l::LUT334L_SPEC>,
    #[doc = "0x1a74 - Graphic MMU LUT entry 334 high"]
    pub lut334h: crate::Reg<lut334h::LUT334H_SPEC>,
    #[doc = "0x1a78 - Graphic MMU LUT entry 335 low"]
    pub lut335l: crate::Reg<lut335l::LUT335L_SPEC>,
    #[doc = "0x1a7c - Graphic MMU LUT entry 335 high"]
    pub lut335h: crate::Reg<lut335h::LUT335H_SPEC>,
    #[doc = "0x1a80 - Graphic MMU LUT entry 336 low"]
    pub lut336l: crate::Reg<lut336l::LUT336L_SPEC>,
    #[doc = "0x1a84 - Graphic MMU LUT entry 336 high"]
    pub lut336h: crate::Reg<lut336h::LUT336H_SPEC>,
    #[doc = "0x1a88 - Graphic MMU LUT entry 337 low"]
    pub lut337l: crate::Reg<lut337l::LUT337L_SPEC>,
    #[doc = "0x1a8c - Graphic MMU LUT entry 337 high"]
    pub lut337h: crate::Reg<lut337h::LUT337H_SPEC>,
    #[doc = "0x1a90 - Graphic MMU LUT entry 338 low"]
    pub lut338l: crate::Reg<lut338l::LUT338L_SPEC>,
    #[doc = "0x1a94 - Graphic MMU LUT entry 338 high"]
    pub lut338h: crate::Reg<lut338h::LUT338H_SPEC>,
    #[doc = "0x1a98 - Graphic MMU LUT entry 339 low"]
    pub lut339l: crate::Reg<lut339l::LUT339L_SPEC>,
    #[doc = "0x1a9c - Graphic MMU LUT entry 339 high"]
    pub lut339h: crate::Reg<lut339h::LUT339H_SPEC>,
    #[doc = "0x1aa0 - Graphic MMU LUT entry 340 low"]
    pub lut340l: crate::Reg<lut340l::LUT340L_SPEC>,
    #[doc = "0x1aa4 - Graphic MMU LUT entry 340 high"]
    pub lut340h: crate::Reg<lut340h::LUT340H_SPEC>,
    #[doc = "0x1aa8 - Graphic MMU LUT entry 341 low"]
    pub lut341l: crate::Reg<lut341l::LUT341L_SPEC>,
    #[doc = "0x1aac - Graphic MMU LUT entry 341 high"]
    pub lut341h: crate::Reg<lut341h::LUT341H_SPEC>,
    #[doc = "0x1ab0 - Graphic MMU LUT entry 342 low"]
    pub lut342l: crate::Reg<lut342l::LUT342L_SPEC>,
    #[doc = "0x1ab4 - Graphic MMU LUT entry 342 high"]
    pub lut342h: crate::Reg<lut342h::LUT342H_SPEC>,
    #[doc = "0x1ab8 - Graphic MMU LUT entry 343 low"]
    pub lut343l: crate::Reg<lut343l::LUT343L_SPEC>,
    #[doc = "0x1abc - Graphic MMU LUT entry 343 high"]
    pub lut343h: crate::Reg<lut343h::LUT343H_SPEC>,
    #[doc = "0x1ac0 - Graphic MMU LUT entry 344 low"]
    pub lut344l: crate::Reg<lut344l::LUT344L_SPEC>,
    #[doc = "0x1ac4 - Graphic MMU LUT entry 344 high"]
    pub lut344h: crate::Reg<lut344h::LUT344H_SPEC>,
    #[doc = "0x1ac8 - Graphic MMU LUT entry 345 low"]
    pub lut345l: crate::Reg<lut345l::LUT345L_SPEC>,
    #[doc = "0x1acc - Graphic MMU LUT entry 345 high"]
    pub lut345h: crate::Reg<lut345h::LUT345H_SPEC>,
    #[doc = "0x1ad0 - Graphic MMU LUT entry 346 low"]
    pub lut346l: crate::Reg<lut346l::LUT346L_SPEC>,
    #[doc = "0x1ad4 - Graphic MMU LUT entry 346 high"]
    pub lut346h: crate::Reg<lut346h::LUT346H_SPEC>,
    #[doc = "0x1ad8 - Graphic MMU LUT entry 347 low"]
    pub lut347l: crate::Reg<lut347l::LUT347L_SPEC>,
    #[doc = "0x1adc - Graphic MMU LUT entry 347 high"]
    pub lut347h: crate::Reg<lut347h::LUT347H_SPEC>,
    #[doc = "0x1ae0 - Graphic MMU LUT entry 348 low"]
    pub lut348l: crate::Reg<lut348l::LUT348L_SPEC>,
    #[doc = "0x1ae4 - Graphic MMU LUT entry 348 high"]
    pub lut348h: crate::Reg<lut348h::LUT348H_SPEC>,
    #[doc = "0x1ae8 - Graphic MMU LUT entry 349 low"]
    pub lut349l: crate::Reg<lut349l::LUT349L_SPEC>,
    #[doc = "0x1aec - Graphic MMU LUT entry 349 high"]
    pub lut349h: crate::Reg<lut349h::LUT349H_SPEC>,
    #[doc = "0x1af0 - Graphic MMU LUT entry 350 low"]
    pub lut350l: crate::Reg<lut350l::LUT350L_SPEC>,
    #[doc = "0x1af4 - Graphic MMU LUT entry 350 high"]
    pub lut350h: crate::Reg<lut350h::LUT350H_SPEC>,
    #[doc = "0x1af8 - Graphic MMU LUT entry 351 low"]
    pub lut351l: crate::Reg<lut351l::LUT351L_SPEC>,
    #[doc = "0x1afc - Graphic MMU LUT entry 351 high"]
    pub lut351h: crate::Reg<lut351h::LUT351H_SPEC>,
    #[doc = "0x1b00 - Graphic MMU LUT entry 352 low"]
    pub lut352l: crate::Reg<lut352l::LUT352L_SPEC>,
    #[doc = "0x1b04 - Graphic MMU LUT entry 352 high"]
    pub lut352h: crate::Reg<lut352h::LUT352H_SPEC>,
    #[doc = "0x1b08 - Graphic MMU LUT entry 353 low"]
    pub lut353l: crate::Reg<lut353l::LUT353L_SPEC>,
    #[doc = "0x1b0c - Graphic MMU LUT entry 353 high"]
    pub lut353h: crate::Reg<lut353h::LUT353H_SPEC>,
    #[doc = "0x1b10 - Graphic MMU LUT entry 354 low"]
    pub lut354l: crate::Reg<lut354l::LUT354L_SPEC>,
    #[doc = "0x1b14 - Graphic MMU LUT entry 354 high"]
    pub lut354h: crate::Reg<lut354h::LUT354H_SPEC>,
    #[doc = "0x1b18 - Graphic MMU LUT entry 355 low"]
    pub lut355l: crate::Reg<lut355l::LUT355L_SPEC>,
    #[doc = "0x1b1c - Graphic MMU LUT entry 355 high"]
    pub lut355h: crate::Reg<lut355h::LUT355H_SPEC>,
    #[doc = "0x1b20 - Graphic MMU LUT entry 356 low"]
    pub lut356l: crate::Reg<lut356l::LUT356L_SPEC>,
    #[doc = "0x1b24 - Graphic MMU LUT entry 356 high"]
    pub lut356h: crate::Reg<lut356h::LUT356H_SPEC>,
    #[doc = "0x1b28 - Graphic MMU LUT entry 357 low"]
    pub lut357l: crate::Reg<lut357l::LUT357L_SPEC>,
    #[doc = "0x1b2c - Graphic MMU LUT entry 357 high"]
    pub lut357h: crate::Reg<lut357h::LUT357H_SPEC>,
    #[doc = "0x1b30 - Graphic MMU LUT entry 358 low"]
    pub lut358l: crate::Reg<lut358l::LUT358L_SPEC>,
    #[doc = "0x1b34 - Graphic MMU LUT entry 358 high"]
    pub lut358h: crate::Reg<lut358h::LUT358H_SPEC>,
    #[doc = "0x1b38 - Graphic MMU LUT entry 359 low"]
    pub lut359l: crate::Reg<lut359l::LUT359L_SPEC>,
    #[doc = "0x1b3c - Graphic MMU LUT entry 359 high"]
    pub lut359h: crate::Reg<lut359h::LUT359H_SPEC>,
    #[doc = "0x1b40 - Graphic MMU LUT entry 360 low"]
    pub lut360l: crate::Reg<lut360l::LUT360L_SPEC>,
    #[doc = "0x1b44 - Graphic MMU LUT entry 360 high"]
    pub lut360h: crate::Reg<lut360h::LUT360H_SPEC>,
    #[doc = "0x1b48 - Graphic MMU LUT entry 361 low"]
    pub lut361l: crate::Reg<lut361l::LUT361L_SPEC>,
    #[doc = "0x1b4c - Graphic MMU LUT entry 361 high"]
    pub lut361h: crate::Reg<lut361h::LUT361H_SPEC>,
    #[doc = "0x1b50 - Graphic MMU LUT entry 362 low"]
    pub lut362l: crate::Reg<lut362l::LUT362L_SPEC>,
    #[doc = "0x1b54 - Graphic MMU LUT entry 362 high"]
    pub lut362h: crate::Reg<lut362h::LUT362H_SPEC>,
    #[doc = "0x1b58 - Graphic MMU LUT entry 363 low"]
    pub lut363l: crate::Reg<lut363l::LUT363L_SPEC>,
    #[doc = "0x1b5c - Graphic MMU LUT entry 363 high"]
    pub lut363h: crate::Reg<lut363h::LUT363H_SPEC>,
    #[doc = "0x1b60 - Graphic MMU LUT entry 364 low"]
    pub lut364l: crate::Reg<lut364l::LUT364L_SPEC>,
    #[doc = "0x1b64 - Graphic MMU LUT entry 364 high"]
    pub lut364h: crate::Reg<lut364h::LUT364H_SPEC>,
    #[doc = "0x1b68 - Graphic MMU LUT entry 365 low"]
    pub lut365l: crate::Reg<lut365l::LUT365L_SPEC>,
    #[doc = "0x1b6c - Graphic MMU LUT entry 365 high"]
    pub lut365h: crate::Reg<lut365h::LUT365H_SPEC>,
    #[doc = "0x1b70 - Graphic MMU LUT entry 366 low"]
    pub lut366l: crate::Reg<lut366l::LUT366L_SPEC>,
    #[doc = "0x1b74 - Graphic MMU LUT entry 366 high"]
    pub lut366h: crate::Reg<lut366h::LUT366H_SPEC>,
    #[doc = "0x1b78 - Graphic MMU LUT entry 367 low"]
    pub lut367l: crate::Reg<lut367l::LUT367L_SPEC>,
    #[doc = "0x1b7c - Graphic MMU LUT entry 367 high"]
    pub lut367h: crate::Reg<lut367h::LUT367H_SPEC>,
    #[doc = "0x1b80 - Graphic MMU LUT entry 368 low"]
    pub lut368l: crate::Reg<lut368l::LUT368L_SPEC>,
    #[doc = "0x1b84 - Graphic MMU LUT entry 368 high"]
    pub lut368h: crate::Reg<lut368h::LUT368H_SPEC>,
    #[doc = "0x1b88 - Graphic MMU LUT entry 369 low"]
    pub lut369l: crate::Reg<lut369l::LUT369L_SPEC>,
    #[doc = "0x1b8c - Graphic MMU LUT entry 369 high"]
    pub lut369h: crate::Reg<lut369h::LUT369H_SPEC>,
    #[doc = "0x1b90 - Graphic MMU LUT entry 370 low"]
    pub lut370l: crate::Reg<lut370l::LUT370L_SPEC>,
    #[doc = "0x1b94 - Graphic MMU LUT entry 370 high"]
    pub lut370h: crate::Reg<lut370h::LUT370H_SPEC>,
    #[doc = "0x1b98 - Graphic MMU LUT entry 371 low"]
    pub lut371l: crate::Reg<lut371l::LUT371L_SPEC>,
    #[doc = "0x1b9c - Graphic MMU LUT entry 371 high"]
    pub lut371h: crate::Reg<lut371h::LUT371H_SPEC>,
    #[doc = "0x1ba0 - Graphic MMU LUT entry 372 low"]
    pub lut372l: crate::Reg<lut372l::LUT372L_SPEC>,
    #[doc = "0x1ba4 - Graphic MMU LUT entry 372 high"]
    pub lut372h: crate::Reg<lut372h::LUT372H_SPEC>,
    #[doc = "0x1ba8 - Graphic MMU LUT entry 373 low"]
    pub lut373l: crate::Reg<lut373l::LUT373L_SPEC>,
    #[doc = "0x1bac - Graphic MMU LUT entry 373 high"]
    pub lut373h: crate::Reg<lut373h::LUT373H_SPEC>,
    #[doc = "0x1bb0 - Graphic MMU LUT entry 374 low"]
    pub lut374l: crate::Reg<lut374l::LUT374L_SPEC>,
    #[doc = "0x1bb4 - Graphic MMU LUT entry 374 high"]
    pub lut374h: crate::Reg<lut374h::LUT374H_SPEC>,
    #[doc = "0x1bb8 - Graphic MMU LUT entry 375 low"]
    pub lut375l: crate::Reg<lut375l::LUT375L_SPEC>,
    #[doc = "0x1bbc - Graphic MMU LUT entry 375 high"]
    pub lut375h: crate::Reg<lut375h::LUT375H_SPEC>,
    #[doc = "0x1bc0 - Graphic MMU LUT entry 376 low"]
    pub lut376l: crate::Reg<lut376l::LUT376L_SPEC>,
    #[doc = "0x1bc4 - Graphic MMU LUT entry 376 high"]
    pub lut376h: crate::Reg<lut376h::LUT376H_SPEC>,
    #[doc = "0x1bc8 - Graphic MMU LUT entry 377 low"]
    pub lut377l: crate::Reg<lut377l::LUT377L_SPEC>,
    #[doc = "0x1bcc - Graphic MMU LUT entry 377 high"]
    pub lut377h: crate::Reg<lut377h::LUT377H_SPEC>,
    #[doc = "0x1bd0 - Graphic MMU LUT entry 378 low"]
    pub lut378l: crate::Reg<lut378l::LUT378L_SPEC>,
    #[doc = "0x1bd4 - Graphic MMU LUT entry 378 high"]
    pub lut378h: crate::Reg<lut378h::LUT378H_SPEC>,
    #[doc = "0x1bd8 - Graphic MMU LUT entry 379 low"]
    pub lut379l: crate::Reg<lut379l::LUT379L_SPEC>,
    #[doc = "0x1bdc - Graphic MMU LUT entry 379 high"]
    pub lut379h: crate::Reg<lut379h::LUT379H_SPEC>,
    #[doc = "0x1be0 - Graphic MMU LUT entry 380 low"]
    pub lut380l: crate::Reg<lut380l::LUT380L_SPEC>,
    #[doc = "0x1be4 - Graphic MMU LUT entry 380 high"]
    pub lut380h: crate::Reg<lut380h::LUT380H_SPEC>,
    #[doc = "0x1be8 - Graphic MMU LUT entry 381 low"]
    pub lut381l: crate::Reg<lut381l::LUT381L_SPEC>,
    #[doc = "0x1bec - Graphic MMU LUT entry 381 high"]
    pub lut381h: crate::Reg<lut381h::LUT381H_SPEC>,
    #[doc = "0x1bf0 - Graphic MMU LUT entry 382 low"]
    pub lut382l: crate::Reg<lut382l::LUT382L_SPEC>,
    #[doc = "0x1bf4 - Graphic MMU LUT entry 382 high"]
    pub lut382h: crate::Reg<lut382h::LUT382H_SPEC>,
    #[doc = "0x1bf8 - Graphic MMU LUT entry 383 low"]
    pub lut383l: crate::Reg<lut383l::LUT383L_SPEC>,
    #[doc = "0x1bfc - Graphic MMU LUT entry 383 high"]
    pub lut383h: crate::Reg<lut383h::LUT383H_SPEC>,
    #[doc = "0x1c00 - Graphic MMU LUT entry 384 low"]
    pub lut384l: crate::Reg<lut384l::LUT384L_SPEC>,
    #[doc = "0x1c04 - Graphic MMU LUT entry 384 high"]
    pub lut384h: crate::Reg<lut384h::LUT384H_SPEC>,
    #[doc = "0x1c08 - Graphic MMU LUT entry 385 low"]
    pub lut385l: crate::Reg<lut385l::LUT385L_SPEC>,
    #[doc = "0x1c0c - Graphic MMU LUT entry 385 high"]
    pub lut385h: crate::Reg<lut385h::LUT385H_SPEC>,
    #[doc = "0x1c10 - Graphic MMU LUT entry 386 low"]
    pub lut386l: crate::Reg<lut386l::LUT386L_SPEC>,
    #[doc = "0x1c14 - Graphic MMU LUT entry 386 high"]
    pub lut386h: crate::Reg<lut386h::LUT386H_SPEC>,
    #[doc = "0x1c18 - Graphic MMU LUT entry 387 low"]
    pub lut387l: crate::Reg<lut387l::LUT387L_SPEC>,
    #[doc = "0x1c1c - Graphic MMU LUT entry 387 high"]
    pub lut387h: crate::Reg<lut387h::LUT387H_SPEC>,
    #[doc = "0x1c20 - Graphic MMU LUT entry 388 low"]
    pub lut388l: crate::Reg<lut388l::LUT388L_SPEC>,
    #[doc = "0x1c24 - Graphic MMU LUT entry 388 high"]
    pub lut388h: crate::Reg<lut388h::LUT388H_SPEC>,
    #[doc = "0x1c28 - Graphic MMU LUT entry 389 low"]
    pub lut389l: crate::Reg<lut389l::LUT389L_SPEC>,
    #[doc = "0x1c2c - Graphic MMU LUT entry 389 high"]
    pub lut389h: crate::Reg<lut389h::LUT389H_SPEC>,
    #[doc = "0x1c30 - Graphic MMU LUT entry 390 low"]
    pub lut390l: crate::Reg<lut390l::LUT390L_SPEC>,
    #[doc = "0x1c34 - Graphic MMU LUT entry 390 high"]
    pub lut390h: crate::Reg<lut390h::LUT390H_SPEC>,
    #[doc = "0x1c38 - Graphic MMU LUT entry 391 low"]
    pub lut391l: crate::Reg<lut391l::LUT391L_SPEC>,
    #[doc = "0x1c3c - Graphic MMU LUT entry 391 high"]
    pub lut391h: crate::Reg<lut391h::LUT391H_SPEC>,
    #[doc = "0x1c40 - Graphic MMU LUT entry 392 low"]
    pub lut392l: crate::Reg<lut392l::LUT392L_SPEC>,
    #[doc = "0x1c44 - Graphic MMU LUT entry 392 high"]
    pub lut392h: crate::Reg<lut392h::LUT392H_SPEC>,
    #[doc = "0x1c48 - Graphic MMU LUT entry 393 low"]
    pub lut393l: crate::Reg<lut393l::LUT393L_SPEC>,
    #[doc = "0x1c4c - Graphic MMU LUT entry 393 high"]
    pub lut393h: crate::Reg<lut393h::LUT393H_SPEC>,
    #[doc = "0x1c50 - Graphic MMU LUT entry 394 low"]
    pub lut394l: crate::Reg<lut394l::LUT394L_SPEC>,
    #[doc = "0x1c54 - Graphic MMU LUT entry 394 high"]
    pub lut394h: crate::Reg<lut394h::LUT394H_SPEC>,
    #[doc = "0x1c58 - Graphic MMU LUT entry 395 low"]
    pub lut395l: crate::Reg<lut395l::LUT395L_SPEC>,
    #[doc = "0x1c5c - Graphic MMU LUT entry 395 high"]
    pub lut395h: crate::Reg<lut395h::LUT395H_SPEC>,
    #[doc = "0x1c60 - Graphic MMU LUT entry 396 low"]
    pub lut396l: crate::Reg<lut396l::LUT396L_SPEC>,
    #[doc = "0x1c64 - Graphic MMU LUT entry 396 high"]
    pub lut396h: crate::Reg<lut396h::LUT396H_SPEC>,
    #[doc = "0x1c68 - Graphic MMU LUT entry 397 low"]
    pub lut397l: crate::Reg<lut397l::LUT397L_SPEC>,
    #[doc = "0x1c6c - Graphic MMU LUT entry 397 high"]
    pub lut397h: crate::Reg<lut397h::LUT397H_SPEC>,
    #[doc = "0x1c70 - Graphic MMU LUT entry 398 low"]
    pub lut398l: crate::Reg<lut398l::LUT398L_SPEC>,
    #[doc = "0x1c74 - Graphic MMU LUT entry 398 high"]
    pub lut398h: crate::Reg<lut398h::LUT398H_SPEC>,
    #[doc = "0x1c78 - Graphic MMU LUT entry 399 low"]
    pub lut399l: crate::Reg<lut399l::LUT399L_SPEC>,
    #[doc = "0x1c7c - Graphic MMU LUT entry 399 high"]
    pub lut399h: crate::Reg<lut399h::LUT399H_SPEC>,
    #[doc = "0x1c80 - Graphic MMU LUT entry 400 low"]
    pub lut400l: crate::Reg<lut400l::LUT400L_SPEC>,
    #[doc = "0x1c84 - Graphic MMU LUT entry 400 high"]
    pub lut400h: crate::Reg<lut400h::LUT400H_SPEC>,
    #[doc = "0x1c88 - Graphic MMU LUT entry 401 low"]
    pub lut401l: crate::Reg<lut401l::LUT401L_SPEC>,
    #[doc = "0x1c8c - Graphic MMU LUT entry 401 high"]
    pub lut401h: crate::Reg<lut401h::LUT401H_SPEC>,
    #[doc = "0x1c90 - Graphic MMU LUT entry 402 low"]
    pub lut402l: crate::Reg<lut402l::LUT402L_SPEC>,
    #[doc = "0x1c94 - Graphic MMU LUT entry 402 high"]
    pub lut402h: crate::Reg<lut402h::LUT402H_SPEC>,
    #[doc = "0x1c98 - Graphic MMU LUT entry 403 low"]
    pub lut403l: crate::Reg<lut403l::LUT403L_SPEC>,
    #[doc = "0x1c9c - Graphic MMU LUT entry 403 high"]
    pub lut403h: crate::Reg<lut403h::LUT403H_SPEC>,
    #[doc = "0x1ca0 - Graphic MMU LUT entry 404 low"]
    pub lut404l: crate::Reg<lut404l::LUT404L_SPEC>,
    #[doc = "0x1ca4 - Graphic MMU LUT entry 404 high"]
    pub lut404h: crate::Reg<lut404h::LUT404H_SPEC>,
    #[doc = "0x1ca8 - Graphic MMU LUT entry 405 low"]
    pub lut405l: crate::Reg<lut405l::LUT405L_SPEC>,
    #[doc = "0x1cac - Graphic MMU LUT entry 405 high"]
    pub lut405h: crate::Reg<lut405h::LUT405H_SPEC>,
    #[doc = "0x1cb0 - Graphic MMU LUT entry 406 low"]
    pub lut406l: crate::Reg<lut406l::LUT406L_SPEC>,
    #[doc = "0x1cb4 - Graphic MMU LUT entry 406 high"]
    pub lut406h: crate::Reg<lut406h::LUT406H_SPEC>,
    #[doc = "0x1cb8 - Graphic MMU LUT entry 407 low"]
    pub lut407l: crate::Reg<lut407l::LUT407L_SPEC>,
    #[doc = "0x1cbc - Graphic MMU LUT entry 407 high"]
    pub lut407h: crate::Reg<lut407h::LUT407H_SPEC>,
    #[doc = "0x1cc0 - Graphic MMU LUT entry 408 low"]
    pub lut408l: crate::Reg<lut408l::LUT408L_SPEC>,
    #[doc = "0x1cc4 - Graphic MMU LUT entry 408 high"]
    pub lut408h: crate::Reg<lut408h::LUT408H_SPEC>,
    #[doc = "0x1cc8 - Graphic MMU LUT entry 409 low"]
    pub lut409l: crate::Reg<lut409l::LUT409L_SPEC>,
    #[doc = "0x1ccc - Graphic MMU LUT entry 409 high"]
    pub lut409h: crate::Reg<lut409h::LUT409H_SPEC>,
    #[doc = "0x1cd0 - Graphic MMU LUT entry 410 low"]
    pub lut410l: crate::Reg<lut410l::LUT410L_SPEC>,
    #[doc = "0x1cd4 - Graphic MMU LUT entry 410 high"]
    pub lut410h: crate::Reg<lut410h::LUT410H_SPEC>,
    #[doc = "0x1cd8 - Graphic MMU LUT entry 411 low"]
    pub lut411l: crate::Reg<lut411l::LUT411L_SPEC>,
    #[doc = "0x1cdc - Graphic MMU LUT entry 411 high"]
    pub lut411h: crate::Reg<lut411h::LUT411H_SPEC>,
    #[doc = "0x1ce0 - Graphic MMU LUT entry 412 low"]
    pub lut412l: crate::Reg<lut412l::LUT412L_SPEC>,
    #[doc = "0x1ce4 - Graphic MMU LUT entry 412 high"]
    pub lut412h: crate::Reg<lut412h::LUT412H_SPEC>,
    #[doc = "0x1ce8 - Graphic MMU LUT entry 413 low"]
    pub lut413l: crate::Reg<lut413l::LUT413L_SPEC>,
    #[doc = "0x1cec - Graphic MMU LUT entry 413 high"]
    pub lut413h: crate::Reg<lut413h::LUT413H_SPEC>,
    #[doc = "0x1cf0 - Graphic MMU LUT entry 414 low"]
    pub lut414l: crate::Reg<lut414l::LUT414L_SPEC>,
    #[doc = "0x1cf4 - Graphic MMU LUT entry 414 high"]
    pub lut414h: crate::Reg<lut414h::LUT414H_SPEC>,
    #[doc = "0x1cf8 - Graphic MMU LUT entry 415 low"]
    pub lut415l: crate::Reg<lut415l::LUT415L_SPEC>,
    #[doc = "0x1cfc - Graphic MMU LUT entry 415 high"]
    pub lut415h: crate::Reg<lut415h::LUT415H_SPEC>,
    #[doc = "0x1d00 - Graphic MMU LUT entry 416 low"]
    pub lut416l: crate::Reg<lut416l::LUT416L_SPEC>,
    #[doc = "0x1d04 - Graphic MMU LUT entry 416 high"]
    pub lut416h: crate::Reg<lut416h::LUT416H_SPEC>,
    #[doc = "0x1d08 - Graphic MMU LUT entry 417 low"]
    pub lut417l: crate::Reg<lut417l::LUT417L_SPEC>,
    #[doc = "0x1d0c - Graphic MMU LUT entry 417 high"]
    pub lut417h: crate::Reg<lut417h::LUT417H_SPEC>,
    #[doc = "0x1d10 - Graphic MMU LUT entry 418 low"]
    pub lut418l: crate::Reg<lut418l::LUT418L_SPEC>,
    #[doc = "0x1d14 - Graphic MMU LUT entry 418 high"]
    pub lut418h: crate::Reg<lut418h::LUT418H_SPEC>,
    #[doc = "0x1d18 - Graphic MMU LUT entry 419 low"]
    pub lut419l: crate::Reg<lut419l::LUT419L_SPEC>,
    #[doc = "0x1d1c - Graphic MMU LUT entry 419 high"]
    pub lut419h: crate::Reg<lut419h::LUT419H_SPEC>,
    #[doc = "0x1d20 - Graphic MMU LUT entry 420 low"]
    pub lut420l: crate::Reg<lut420l::LUT420L_SPEC>,
    #[doc = "0x1d24 - Graphic MMU LUT entry 420 high"]
    pub lut420h: crate::Reg<lut420h::LUT420H_SPEC>,
    #[doc = "0x1d28 - Graphic MMU LUT entry 421 low"]
    pub lut421l: crate::Reg<lut421l::LUT421L_SPEC>,
    #[doc = "0x1d2c - Graphic MMU LUT entry 421 high"]
    pub lut421h: crate::Reg<lut421h::LUT421H_SPEC>,
    #[doc = "0x1d30 - Graphic MMU LUT entry 422 low"]
    pub lut422l: crate::Reg<lut422l::LUT422L_SPEC>,
    #[doc = "0x1d34 - Graphic MMU LUT entry 422 high"]
    pub lut422h: crate::Reg<lut422h::LUT422H_SPEC>,
    #[doc = "0x1d38 - Graphic MMU LUT entry 423 low"]
    pub lut423l: crate::Reg<lut423l::LUT423L_SPEC>,
    #[doc = "0x1d3c - Graphic MMU LUT entry 423 high"]
    pub lut423h: crate::Reg<lut423h::LUT423H_SPEC>,
    #[doc = "0x1d40 - Graphic MMU LUT entry 424 low"]
    pub lut424l: crate::Reg<lut424l::LUT424L_SPEC>,
    #[doc = "0x1d44 - Graphic MMU LUT entry 424 high"]
    pub lut424h: crate::Reg<lut424h::LUT424H_SPEC>,
    #[doc = "0x1d48 - Graphic MMU LUT entry 425 low"]
    pub lut425l: crate::Reg<lut425l::LUT425L_SPEC>,
    #[doc = "0x1d4c - Graphic MMU LUT entry 425 high"]
    pub lut425h: crate::Reg<lut425h::LUT425H_SPEC>,
    #[doc = "0x1d50 - Graphic MMU LUT entry 426 low"]
    pub lut426l: crate::Reg<lut426l::LUT426L_SPEC>,
    #[doc = "0x1d54 - Graphic MMU LUT entry 426 high"]
    pub lut426h: crate::Reg<lut426h::LUT426H_SPEC>,
    #[doc = "0x1d58 - Graphic MMU LUT entry 427 low"]
    pub lut427l: crate::Reg<lut427l::LUT427L_SPEC>,
    #[doc = "0x1d5c - Graphic MMU LUT entry 427 high"]
    pub lut427h: crate::Reg<lut427h::LUT427H_SPEC>,
    #[doc = "0x1d60 - Graphic MMU LUT entry 428 low"]
    pub lut428l: crate::Reg<lut428l::LUT428L_SPEC>,
    #[doc = "0x1d64 - Graphic MMU LUT entry 428 high"]
    pub lut428h: crate::Reg<lut428h::LUT428H_SPEC>,
    #[doc = "0x1d68 - Graphic MMU LUT entry 429 low"]
    pub lut429l: crate::Reg<lut429l::LUT429L_SPEC>,
    #[doc = "0x1d6c - Graphic MMU LUT entry 429 high"]
    pub lut429h: crate::Reg<lut429h::LUT429H_SPEC>,
    #[doc = "0x1d70 - Graphic MMU LUT entry 430 low"]
    pub lut430l: crate::Reg<lut430l::LUT430L_SPEC>,
    #[doc = "0x1d74 - Graphic MMU LUT entry 430 high"]
    pub lut430h: crate::Reg<lut430h::LUT430H_SPEC>,
    #[doc = "0x1d78 - Graphic MMU LUT entry 431 low"]
    pub lut431l: crate::Reg<lut431l::LUT431L_SPEC>,
    #[doc = "0x1d7c - Graphic MMU LUT entry 431 high"]
    pub lut431h: crate::Reg<lut431h::LUT431H_SPEC>,
    #[doc = "0x1d80 - Graphic MMU LUT entry 432 low"]
    pub lut432l: crate::Reg<lut432l::LUT432L_SPEC>,
    #[doc = "0x1d84 - Graphic MMU LUT entry 432 high"]
    pub lut432h: crate::Reg<lut432h::LUT432H_SPEC>,
    #[doc = "0x1d88 - Graphic MMU LUT entry 433 low"]
    pub lut433l: crate::Reg<lut433l::LUT433L_SPEC>,
    #[doc = "0x1d8c - Graphic MMU LUT entry 433 high"]
    pub lut433h: crate::Reg<lut433h::LUT433H_SPEC>,
    #[doc = "0x1d90 - Graphic MMU LUT entry 434 low"]
    pub lut434l: crate::Reg<lut434l::LUT434L_SPEC>,
    #[doc = "0x1d94 - Graphic MMU LUT entry 434 high"]
    pub lut434h: crate::Reg<lut434h::LUT434H_SPEC>,
    #[doc = "0x1d98 - Graphic MMU LUT entry 435 low"]
    pub lut435l: crate::Reg<lut435l::LUT435L_SPEC>,
    #[doc = "0x1d9c - Graphic MMU LUT entry 435 high"]
    pub lut435h: crate::Reg<lut435h::LUT435H_SPEC>,
    #[doc = "0x1da0 - Graphic MMU LUT entry 436 low"]
    pub lut436l: crate::Reg<lut436l::LUT436L_SPEC>,
    #[doc = "0x1da4 - Graphic MMU LUT entry 436 high"]
    pub lut436h: crate::Reg<lut436h::LUT436H_SPEC>,
    #[doc = "0x1da8 - Graphic MMU LUT entry 437 low"]
    pub lut437l: crate::Reg<lut437l::LUT437L_SPEC>,
    #[doc = "0x1dac - Graphic MMU LUT entry 437 high"]
    pub lut437h: crate::Reg<lut437h::LUT437H_SPEC>,
    #[doc = "0x1db0 - Graphic MMU LUT entry 438 low"]
    pub lut438l: crate::Reg<lut438l::LUT438L_SPEC>,
    #[doc = "0x1db4 - Graphic MMU LUT entry 438 high"]
    pub lut438h: crate::Reg<lut438h::LUT438H_SPEC>,
    #[doc = "0x1db8 - Graphic MMU LUT entry 439 low"]
    pub lut439l: crate::Reg<lut439l::LUT439L_SPEC>,
    #[doc = "0x1dbc - Graphic MMU LUT entry 439 high"]
    pub lut439h: crate::Reg<lut439h::LUT439H_SPEC>,
    #[doc = "0x1dc0 - Graphic MMU LUT entry 440 low"]
    pub lut440l: crate::Reg<lut440l::LUT440L_SPEC>,
    #[doc = "0x1dc4 - Graphic MMU LUT entry 440 high"]
    pub lut440h: crate::Reg<lut440h::LUT440H_SPEC>,
    #[doc = "0x1dc8 - Graphic MMU LUT entry 441 low"]
    pub lut441l: crate::Reg<lut441l::LUT441L_SPEC>,
    #[doc = "0x1dcc - Graphic MMU LUT entry 441 high"]
    pub lut441h: crate::Reg<lut441h::LUT441H_SPEC>,
    #[doc = "0x1dd0 - Graphic MMU LUT entry 442 low"]
    pub lut442l: crate::Reg<lut442l::LUT442L_SPEC>,
    #[doc = "0x1dd4 - Graphic MMU LUT entry 442 high"]
    pub lut442h: crate::Reg<lut442h::LUT442H_SPEC>,
    #[doc = "0x1dd8 - Graphic MMU LUT entry 443 low"]
    pub lut443l: crate::Reg<lut443l::LUT443L_SPEC>,
    #[doc = "0x1ddc - Graphic MMU LUT entry 443 high"]
    pub lut443h: crate::Reg<lut443h::LUT443H_SPEC>,
    #[doc = "0x1de0 - Graphic MMU LUT entry 444 low"]
    pub lut444l: crate::Reg<lut444l::LUT444L_SPEC>,
    #[doc = "0x1de4 - Graphic MMU LUT entry 444 high"]
    pub lut444h: crate::Reg<lut444h::LUT444H_SPEC>,
    #[doc = "0x1de8 - Graphic MMU LUT entry 445 low"]
    pub lut445l: crate::Reg<lut445l::LUT445L_SPEC>,
    #[doc = "0x1dec - Graphic MMU LUT entry 445 high"]
    pub lut445h: crate::Reg<lut445h::LUT445H_SPEC>,
    #[doc = "0x1df0 - Graphic MMU LUT entry 446 low"]
    pub lut446l: crate::Reg<lut446l::LUT446L_SPEC>,
    #[doc = "0x1df4 - Graphic MMU LUT entry 446 high"]
    pub lut446h: crate::Reg<lut446h::LUT446H_SPEC>,
    #[doc = "0x1df8 - Graphic MMU LUT entry 447 low"]
    pub lut447l: crate::Reg<lut447l::LUT447L_SPEC>,
    #[doc = "0x1dfc - Graphic MMU LUT entry 447 high"]
    pub lut447h: crate::Reg<lut447h::LUT447H_SPEC>,
    #[doc = "0x1e00 - Graphic MMU LUT entry 448 low"]
    pub lut448l: crate::Reg<lut448l::LUT448L_SPEC>,
    #[doc = "0x1e04 - Graphic MMU LUT entry 448 high"]
    pub lut448h: crate::Reg<lut448h::LUT448H_SPEC>,
    #[doc = "0x1e08 - Graphic MMU LUT entry 449 low"]
    pub lut449l: crate::Reg<lut449l::LUT449L_SPEC>,
    #[doc = "0x1e0c - Graphic MMU LUT entry 449 high"]
    pub lut449h: crate::Reg<lut449h::LUT449H_SPEC>,
    #[doc = "0x1e10 - Graphic MMU LUT entry 450 low"]
    pub lut450l: crate::Reg<lut450l::LUT450L_SPEC>,
    #[doc = "0x1e14 - Graphic MMU LUT entry 450 high"]
    pub lut450h: crate::Reg<lut450h::LUT450H_SPEC>,
    #[doc = "0x1e18 - Graphic MMU LUT entry 451 low"]
    pub lut451l: crate::Reg<lut451l::LUT451L_SPEC>,
    #[doc = "0x1e1c - Graphic MMU LUT entry 451 high"]
    pub lut451h: crate::Reg<lut451h::LUT451H_SPEC>,
    #[doc = "0x1e20 - Graphic MMU LUT entry 452 low"]
    pub lut452l: crate::Reg<lut452l::LUT452L_SPEC>,
    #[doc = "0x1e24 - Graphic MMU LUT entry 452 high"]
    pub lut452h: crate::Reg<lut452h::LUT452H_SPEC>,
    #[doc = "0x1e28 - Graphic MMU LUT entry 453 low"]
    pub lut453l: crate::Reg<lut453l::LUT453L_SPEC>,
    #[doc = "0x1e2c - Graphic MMU LUT entry 453 high"]
    pub lut453h: crate::Reg<lut453h::LUT453H_SPEC>,
    #[doc = "0x1e30 - Graphic MMU LUT entry 454 low"]
    pub lut454l: crate::Reg<lut454l::LUT454L_SPEC>,
    #[doc = "0x1e34 - Graphic MMU LUT entry 454 high"]
    pub lut454h: crate::Reg<lut454h::LUT454H_SPEC>,
    #[doc = "0x1e38 - Graphic MMU LUT entry 455 low"]
    pub lut455l: crate::Reg<lut455l::LUT455L_SPEC>,
    #[doc = "0x1e3c - Graphic MMU LUT entry 455 high"]
    pub lut455h: crate::Reg<lut455h::LUT455H_SPEC>,
    #[doc = "0x1e40 - Graphic MMU LUT entry 456 low"]
    pub lut456l: crate::Reg<lut456l::LUT456L_SPEC>,
    #[doc = "0x1e44 - Graphic MMU LUT entry 456 high"]
    pub lut456h: crate::Reg<lut456h::LUT456H_SPEC>,
    #[doc = "0x1e48 - Graphic MMU LUT entry 457 low"]
    pub lut457l: crate::Reg<lut457l::LUT457L_SPEC>,
    #[doc = "0x1e4c - Graphic MMU LUT entry 457 high"]
    pub lut457h: crate::Reg<lut457h::LUT457H_SPEC>,
    #[doc = "0x1e50 - Graphic MMU LUT entry 458 low"]
    pub lut458l: crate::Reg<lut458l::LUT458L_SPEC>,
    #[doc = "0x1e54 - Graphic MMU LUT entry 458 high"]
    pub lut458h: crate::Reg<lut458h::LUT458H_SPEC>,
    #[doc = "0x1e58 - Graphic MMU LUT entry 459 low"]
    pub lut459l: crate::Reg<lut459l::LUT459L_SPEC>,
    #[doc = "0x1e5c - Graphic MMU LUT entry 459 high"]
    pub lut459h: crate::Reg<lut459h::LUT459H_SPEC>,
    #[doc = "0x1e60 - Graphic MMU LUT entry 460 low"]
    pub lut460l: crate::Reg<lut460l::LUT460L_SPEC>,
    #[doc = "0x1e64 - Graphic MMU LUT entry 460 high"]
    pub lut460h: crate::Reg<lut460h::LUT460H_SPEC>,
    #[doc = "0x1e68 - Graphic MMU LUT entry 461 low"]
    pub lut461l: crate::Reg<lut461l::LUT461L_SPEC>,
    #[doc = "0x1e6c - Graphic MMU LUT entry 461 high"]
    pub lut461h: crate::Reg<lut461h::LUT461H_SPEC>,
    #[doc = "0x1e70 - Graphic MMU LUT entry 462 low"]
    pub lut462l: crate::Reg<lut462l::LUT462L_SPEC>,
    #[doc = "0x1e74 - Graphic MMU LUT entry 462 high"]
    pub lut462h: crate::Reg<lut462h::LUT462H_SPEC>,
    #[doc = "0x1e78 - Graphic MMU LUT entry 463 low"]
    pub lut463l: crate::Reg<lut463l::LUT463L_SPEC>,
    #[doc = "0x1e7c - Graphic MMU LUT entry 463 high"]
    pub lut463h: crate::Reg<lut463h::LUT463H_SPEC>,
    #[doc = "0x1e80 - Graphic MMU LUT entry 464 low"]
    pub lut464l: crate::Reg<lut464l::LUT464L_SPEC>,
    #[doc = "0x1e84 - Graphic MMU LUT entry 464 high"]
    pub lut464h: crate::Reg<lut464h::LUT464H_SPEC>,
    #[doc = "0x1e88 - Graphic MMU LUT entry 465 low"]
    pub lut465l: crate::Reg<lut465l::LUT465L_SPEC>,
    #[doc = "0x1e8c - Graphic MMU LUT entry 465 high"]
    pub lut465h: crate::Reg<lut465h::LUT465H_SPEC>,
    #[doc = "0x1e90 - Graphic MMU LUT entry 466 low"]
    pub lut466l: crate::Reg<lut466l::LUT466L_SPEC>,
    #[doc = "0x1e94 - Graphic MMU LUT entry 466 high"]
    pub lut466h: crate::Reg<lut466h::LUT466H_SPEC>,
    #[doc = "0x1e98 - Graphic MMU LUT entry 467 low"]
    pub lut467l: crate::Reg<lut467l::LUT467L_SPEC>,
    #[doc = "0x1e9c - Graphic MMU LUT entry 467 high"]
    pub lut467h: crate::Reg<lut467h::LUT467H_SPEC>,
    #[doc = "0x1ea0 - Graphic MMU LUT entry 468 low"]
    pub lut468l: crate::Reg<lut468l::LUT468L_SPEC>,
    #[doc = "0x1ea4 - Graphic MMU LUT entry 468 high"]
    pub lut468h: crate::Reg<lut468h::LUT468H_SPEC>,
    #[doc = "0x1ea8 - Graphic MMU LUT entry 469 low"]
    pub lut469l: crate::Reg<lut469l::LUT469L_SPEC>,
    #[doc = "0x1eac - Graphic MMU LUT entry 469 high"]
    pub lut469h: crate::Reg<lut469h::LUT469H_SPEC>,
    #[doc = "0x1eb0 - Graphic MMU LUT entry 470 low"]
    pub lut470l: crate::Reg<lut470l::LUT470L_SPEC>,
    #[doc = "0x1eb4 - Graphic MMU LUT entry 470 high"]
    pub lut470h: crate::Reg<lut470h::LUT470H_SPEC>,
    #[doc = "0x1eb8 - Graphic MMU LUT entry 471 low"]
    pub lut471l: crate::Reg<lut471l::LUT471L_SPEC>,
    #[doc = "0x1ebc - Graphic MMU LUT entry 471 high"]
    pub lut471h: crate::Reg<lut471h::LUT471H_SPEC>,
    #[doc = "0x1ec0 - Graphic MMU LUT entry 472 low"]
    pub lut472l: crate::Reg<lut472l::LUT472L_SPEC>,
    #[doc = "0x1ec4 - Graphic MMU LUT entry 472 high"]
    pub lut472h: crate::Reg<lut472h::LUT472H_SPEC>,
    #[doc = "0x1ec8 - Graphic MMU LUT entry 473 low"]
    pub lut473l: crate::Reg<lut473l::LUT473L_SPEC>,
    #[doc = "0x1ecc - Graphic MMU LUT entry 473 high"]
    pub lut473h: crate::Reg<lut473h::LUT473H_SPEC>,
    #[doc = "0x1ed0 - Graphic MMU LUT entry 474 low"]
    pub lut474l: crate::Reg<lut474l::LUT474L_SPEC>,
    #[doc = "0x1ed4 - Graphic MMU LUT entry 474 high"]
    pub lut474h: crate::Reg<lut474h::LUT474H_SPEC>,
    #[doc = "0x1ed8 - Graphic MMU LUT entry 475 low"]
    pub lut475l: crate::Reg<lut475l::LUT475L_SPEC>,
    #[doc = "0x1edc - Graphic MMU LUT entry 475 high"]
    pub lut475h: crate::Reg<lut475h::LUT475H_SPEC>,
    #[doc = "0x1ee0 - Graphic MMU LUT entry 476 low"]
    pub lut476l: crate::Reg<lut476l::LUT476L_SPEC>,
    #[doc = "0x1ee4 - Graphic MMU LUT entry 476 high"]
    pub lut476h: crate::Reg<lut476h::LUT476H_SPEC>,
    #[doc = "0x1ee8 - Graphic MMU LUT entry 477 low"]
    pub lut477l: crate::Reg<lut477l::LUT477L_SPEC>,
    #[doc = "0x1eec - Graphic MMU LUT entry 477 high"]
    pub lut477h: crate::Reg<lut477h::LUT477H_SPEC>,
    #[doc = "0x1ef0 - Graphic MMU LUT entry 478 low"]
    pub lut478l: crate::Reg<lut478l::LUT478L_SPEC>,
    #[doc = "0x1ef4 - Graphic MMU LUT entry 478 high"]
    pub lut478h: crate::Reg<lut478h::LUT478H_SPEC>,
    #[doc = "0x1ef8 - Graphic MMU LUT entry 479 low"]
    pub lut479l: crate::Reg<lut479l::LUT479L_SPEC>,
    #[doc = "0x1efc - Graphic MMU LUT entry 479 high"]
    pub lut479h: crate::Reg<lut479h::LUT479H_SPEC>,
    #[doc = "0x1f00 - Graphic MMU LUT entry 480 low"]
    pub lut480l: crate::Reg<lut480l::LUT480L_SPEC>,
    #[doc = "0x1f04 - Graphic MMU LUT entry 480 high"]
    pub lut480h: crate::Reg<lut480h::LUT480H_SPEC>,
    #[doc = "0x1f08 - Graphic MMU LUT entry 481 low"]
    pub lut481l: crate::Reg<lut481l::LUT481L_SPEC>,
    #[doc = "0x1f0c - Graphic MMU LUT entry 481 high"]
    pub lut481h: crate::Reg<lut481h::LUT481H_SPEC>,
    #[doc = "0x1f10 - Graphic MMU LUT entry 482 low"]
    pub lut482l: crate::Reg<lut482l::LUT482L_SPEC>,
    #[doc = "0x1f14 - Graphic MMU LUT entry 482 high"]
    pub lut482h: crate::Reg<lut482h::LUT482H_SPEC>,
    #[doc = "0x1f18 - Graphic MMU LUT entry 483 low"]
    pub lut483l: crate::Reg<lut483l::LUT483L_SPEC>,
    #[doc = "0x1f1c - Graphic MMU LUT entry 483 high"]
    pub lut483h: crate::Reg<lut483h::LUT483H_SPEC>,
    #[doc = "0x1f20 - Graphic MMU LUT entry 484 low"]
    pub lut484l: crate::Reg<lut484l::LUT484L_SPEC>,
    #[doc = "0x1f24 - Graphic MMU LUT entry 484 high"]
    pub lut484h: crate::Reg<lut484h::LUT484H_SPEC>,
    #[doc = "0x1f28 - Graphic MMU LUT entry 485 low"]
    pub lut485l: crate::Reg<lut485l::LUT485L_SPEC>,
    #[doc = "0x1f2c - Graphic MMU LUT entry 485 high"]
    pub lut485h: crate::Reg<lut485h::LUT485H_SPEC>,
    #[doc = "0x1f30 - Graphic MMU LUT entry 486 low"]
    pub lut486l: crate::Reg<lut486l::LUT486L_SPEC>,
    #[doc = "0x1f34 - Graphic MMU LUT entry 486 high"]
    pub lut486h: crate::Reg<lut486h::LUT486H_SPEC>,
    #[doc = "0x1f38 - Graphic MMU LUT entry 487 low"]
    pub lut487l: crate::Reg<lut487l::LUT487L_SPEC>,
    #[doc = "0x1f3c - Graphic MMU LUT entry 487 high"]
    pub lut487h: crate::Reg<lut487h::LUT487H_SPEC>,
    #[doc = "0x1f40 - Graphic MMU LUT entry 488 low"]
    pub lut488l: crate::Reg<lut488l::LUT488L_SPEC>,
    #[doc = "0x1f44 - Graphic MMU LUT entry 488 high"]
    pub lut488h: crate::Reg<lut488h::LUT488H_SPEC>,
    #[doc = "0x1f48 - Graphic MMU LUT entry 489 low"]
    pub lut489l: crate::Reg<lut489l::LUT489L_SPEC>,
    #[doc = "0x1f4c - Graphic MMU LUT entry 489 high"]
    pub lut489h: crate::Reg<lut489h::LUT489H_SPEC>,
    #[doc = "0x1f50 - Graphic MMU LUT entry 490 low"]
    pub lut490l: crate::Reg<lut490l::LUT490L_SPEC>,
    #[doc = "0x1f54 - Graphic MMU LUT entry 490 high"]
    pub lut490h: crate::Reg<lut490h::LUT490H_SPEC>,
    #[doc = "0x1f58 - Graphic MMU LUT entry 491 low"]
    pub lut491l: crate::Reg<lut491l::LUT491L_SPEC>,
    #[doc = "0x1f5c - Graphic MMU LUT entry 491 high"]
    pub lut491h: crate::Reg<lut491h::LUT491H_SPEC>,
    #[doc = "0x1f60 - Graphic MMU LUT entry 492 low"]
    pub lut492l: crate::Reg<lut492l::LUT492L_SPEC>,
    #[doc = "0x1f64 - Graphic MMU LUT entry 492 high"]
    pub lut492h: crate::Reg<lut492h::LUT492H_SPEC>,
    #[doc = "0x1f68 - Graphic MMU LUT entry 493 low"]
    pub lut493l: crate::Reg<lut493l::LUT493L_SPEC>,
    #[doc = "0x1f6c - Graphic MMU LUT entry 493 high"]
    pub lut493h: crate::Reg<lut493h::LUT493H_SPEC>,
    #[doc = "0x1f70 - Graphic MMU LUT entry 494 low"]
    pub lut494l: crate::Reg<lut494l::LUT494L_SPEC>,
    #[doc = "0x1f74 - Graphic MMU LUT entry 494 high"]
    pub lut494h: crate::Reg<lut494h::LUT494H_SPEC>,
    #[doc = "0x1f78 - Graphic MMU LUT entry 495 low"]
    pub lut495l: crate::Reg<lut495l::LUT495L_SPEC>,
    #[doc = "0x1f7c - Graphic MMU LUT entry 495 high"]
    pub lut495h: crate::Reg<lut495h::LUT495H_SPEC>,
    #[doc = "0x1f80 - Graphic MMU LUT entry 496 low"]
    pub lut496l: crate::Reg<lut496l::LUT496L_SPEC>,
    #[doc = "0x1f84 - Graphic MMU LUT entry 496 high"]
    pub lut496h: crate::Reg<lut496h::LUT496H_SPEC>,
    #[doc = "0x1f88 - Graphic MMU LUT entry 497 low"]
    pub lut497l: crate::Reg<lut497l::LUT497L_SPEC>,
    #[doc = "0x1f8c - Graphic MMU LUT entry 497 high"]
    pub lut497h: crate::Reg<lut497h::LUT497H_SPEC>,
    #[doc = "0x1f90 - Graphic MMU LUT entry 498 low"]
    pub lut498l: crate::Reg<lut498l::LUT498L_SPEC>,
    #[doc = "0x1f94 - Graphic MMU LUT entry 498 high"]
    pub lut498h: crate::Reg<lut498h::LUT498H_SPEC>,
    #[doc = "0x1f98 - Graphic MMU LUT entry 499 low"]
    pub lut499l: crate::Reg<lut499l::LUT499L_SPEC>,
    #[doc = "0x1f9c - Graphic MMU LUT entry 499 high"]
    pub lut499h: crate::Reg<lut499h::LUT499H_SPEC>,
    #[doc = "0x1fa0 - Graphic MMU LUT entry 500 low"]
    pub lut500l: crate::Reg<lut500l::LUT500L_SPEC>,
    #[doc = "0x1fa4 - Graphic MMU LUT entry 500 high"]
    pub lut500h: crate::Reg<lut500h::LUT500H_SPEC>,
    #[doc = "0x1fa8 - Graphic MMU LUT entry 501 low"]
    pub lut501l: crate::Reg<lut501l::LUT501L_SPEC>,
    #[doc = "0x1fac - Graphic MMU LUT entry 501 high"]
    pub lut501h: crate::Reg<lut501h::LUT501H_SPEC>,
    #[doc = "0x1fb0 - Graphic MMU LUT entry 502 low"]
    pub lut502l: crate::Reg<lut502l::LUT502L_SPEC>,
    #[doc = "0x1fb4 - Graphic MMU LUT entry 502 high"]
    pub lut502h: crate::Reg<lut502h::LUT502H_SPEC>,
    #[doc = "0x1fb8 - Graphic MMU LUT entry 503 low"]
    pub lut503l: crate::Reg<lut503l::LUT503L_SPEC>,
    #[doc = "0x1fbc - Graphic MMU LUT entry 503 high"]
    pub lut503h: crate::Reg<lut503h::LUT503H_SPEC>,
    #[doc = "0x1fc0 - Graphic MMU LUT entry 504 low"]
    pub lut504l: crate::Reg<lut504l::LUT504L_SPEC>,
    #[doc = "0x1fc4 - Graphic MMU LUT entry 504 high"]
    pub lut504h: crate::Reg<lut504h::LUT504H_SPEC>,
    #[doc = "0x1fc8 - Graphic MMU LUT entry 505 low"]
    pub lut505l: crate::Reg<lut505l::LUT505L_SPEC>,
    #[doc = "0x1fcc - Graphic MMU LUT entry 505 high"]
    pub lut505h: crate::Reg<lut505h::LUT505H_SPEC>,
    #[doc = "0x1fd0 - Graphic MMU LUT entry 506 low"]
    pub lut506l: crate::Reg<lut506l::LUT506L_SPEC>,
    #[doc = "0x1fd4 - Graphic MMU LUT entry 506 high"]
    pub lut506h: crate::Reg<lut506h::LUT506H_SPEC>,
    #[doc = "0x1fd8 - Graphic MMU LUT entry 507 low"]
    pub lut507l: crate::Reg<lut507l::LUT507L_SPEC>,
    #[doc = "0x1fdc - Graphic MMU LUT entry 507 high"]
    pub lut507h: crate::Reg<lut507h::LUT507H_SPEC>,
    #[doc = "0x1fe0 - Graphic MMU LUT entry 508 low"]
    pub lut508l: crate::Reg<lut508l::LUT508L_SPEC>,
    #[doc = "0x1fe4 - Graphic MMU LUT entry 508 high"]
    pub lut508h: crate::Reg<lut508h::LUT508H_SPEC>,
    #[doc = "0x1fe8 - Graphic MMU LUT entry 509 low"]
    pub lut509l: crate::Reg<lut509l::LUT509L_SPEC>,
    #[doc = "0x1fec - Graphic MMU LUT entry 509 high"]
    pub lut509h: crate::Reg<lut509h::LUT509H_SPEC>,
    #[doc = "0x1ff0 - Graphic MMU LUT entry 510 low"]
    pub lut510l: crate::Reg<lut510l::LUT510L_SPEC>,
    #[doc = "0x1ff4 - Graphic MMU LUT entry 510 high"]
    pub lut510h: crate::Reg<lut510h::LUT510H_SPEC>,
    #[doc = "0x1ff8 - Graphic MMU LUT entry 511 low"]
    pub lut511l: crate::Reg<lut511l::LUT511L_SPEC>,
    #[doc = "0x1ffc - Graphic MMU LUT entry 511 high"]
    pub lut511h: crate::Reg<lut511h::LUT511H_SPEC>,
    #[doc = "0x2000 - Graphic MMU LUT entry 512 low"]
    pub lut512l: crate::Reg<lut512l::LUT512L_SPEC>,
    #[doc = "0x2004 - Graphic MMU LUT entry 512 high"]
    pub lut512h: crate::Reg<lut512h::LUT512H_SPEC>,
    #[doc = "0x2008 - Graphic MMU LUT entry 513 low"]
    pub lut513l: crate::Reg<lut513l::LUT513L_SPEC>,
    #[doc = "0x200c - Graphic MMU LUT entry 513 high"]
    pub lut513h: crate::Reg<lut513h::LUT513H_SPEC>,
    #[doc = "0x2010 - Graphic MMU LUT entry 514 low"]
    pub lut514l: crate::Reg<lut514l::LUT514L_SPEC>,
    #[doc = "0x2014 - Graphic MMU LUT entry 514 high"]
    pub lut514h: crate::Reg<lut514h::LUT514H_SPEC>,
    #[doc = "0x2018 - Graphic MMU LUT entry 515 low"]
    pub lut515l: crate::Reg<lut515l::LUT515L_SPEC>,
    #[doc = "0x201c - Graphic MMU LUT entry 515 high"]
    pub lut515h: crate::Reg<lut515h::LUT515H_SPEC>,
    #[doc = "0x2020 - Graphic MMU LUT entry 516 low"]
    pub lut516l: crate::Reg<lut516l::LUT516L_SPEC>,
    #[doc = "0x2024 - Graphic MMU LUT entry 516 high"]
    pub lut516h: crate::Reg<lut516h::LUT516H_SPEC>,
    #[doc = "0x2028 - Graphic MMU LUT entry 517 low"]
    pub lut517l: crate::Reg<lut517l::LUT517L_SPEC>,
    #[doc = "0x202c - Graphic MMU LUT entry 517 high"]
    pub lut517h: crate::Reg<lut517h::LUT517H_SPEC>,
    #[doc = "0x2030 - Graphic MMU LUT entry 518 low"]
    pub lut518l: crate::Reg<lut518l::LUT518L_SPEC>,
    #[doc = "0x2034 - Graphic MMU LUT entry 518 high"]
    pub lut518h: crate::Reg<lut518h::LUT518H_SPEC>,
    #[doc = "0x2038 - Graphic MMU LUT entry 519 low"]
    pub lut519l: crate::Reg<lut519l::LUT519L_SPEC>,
    #[doc = "0x203c - Graphic MMU LUT entry 519 high"]
    pub lut519h: crate::Reg<lut519h::LUT519H_SPEC>,
    #[doc = "0x2040 - Graphic MMU LUT entry 520 low"]
    pub lut520l: crate::Reg<lut520l::LUT520L_SPEC>,
    #[doc = "0x2044 - Graphic MMU LUT entry 520 high"]
    pub lut520h: crate::Reg<lut520h::LUT520H_SPEC>,
    #[doc = "0x2048 - Graphic MMU LUT entry 521 low"]
    pub lut521l: crate::Reg<lut521l::LUT521L_SPEC>,
    #[doc = "0x204c - Graphic MMU LUT entry 521 high"]
    pub lut521h: crate::Reg<lut521h::LUT521H_SPEC>,
    #[doc = "0x2050 - Graphic MMU LUT entry 522 low"]
    pub lut522l: crate::Reg<lut522l::LUT522L_SPEC>,
    #[doc = "0x2054 - Graphic MMU LUT entry 522 high"]
    pub lut522h: crate::Reg<lut522h::LUT522H_SPEC>,
    #[doc = "0x2058 - Graphic MMU LUT entry 523 low"]
    pub lut523l: crate::Reg<lut523l::LUT523L_SPEC>,
    #[doc = "0x205c - Graphic MMU LUT entry 523 high"]
    pub lut523h: crate::Reg<lut523h::LUT523H_SPEC>,
    #[doc = "0x2060 - Graphic MMU LUT entry 524 low"]
    pub lut524l: crate::Reg<lut524l::LUT524L_SPEC>,
    #[doc = "0x2064 - Graphic MMU LUT entry 524 high"]
    pub lut524h: crate::Reg<lut524h::LUT524H_SPEC>,
    #[doc = "0x2068 - Graphic MMU LUT entry 525 low"]
    pub lut525l: crate::Reg<lut525l::LUT525L_SPEC>,
    #[doc = "0x206c - Graphic MMU LUT entry 525 high"]
    pub lut525h: crate::Reg<lut525h::LUT525H_SPEC>,
    #[doc = "0x2070 - Graphic MMU LUT entry 526 low"]
    pub lut526l: crate::Reg<lut526l::LUT526L_SPEC>,
    #[doc = "0x2074 - Graphic MMU LUT entry 526 high"]
    pub lut526h: crate::Reg<lut526h::LUT526H_SPEC>,
    #[doc = "0x2078 - Graphic MMU LUT entry 527 low"]
    pub lut527l: crate::Reg<lut527l::LUT527L_SPEC>,
    #[doc = "0x207c - Graphic MMU LUT entry 527 high"]
    pub lut527h: crate::Reg<lut527h::LUT527H_SPEC>,
    #[doc = "0x2080 - Graphic MMU LUT entry 528 low"]
    pub lut528l: crate::Reg<lut528l::LUT528L_SPEC>,
    #[doc = "0x2084 - Graphic MMU LUT entry 528 high"]
    pub lut528h: crate::Reg<lut528h::LUT528H_SPEC>,
    #[doc = "0x2088 - Graphic MMU LUT entry 529 low"]
    pub lut529l: crate::Reg<lut529l::LUT529L_SPEC>,
    #[doc = "0x208c - Graphic MMU LUT entry 529 high"]
    pub lut529h: crate::Reg<lut529h::LUT529H_SPEC>,
    #[doc = "0x2090 - Graphic MMU LUT entry 530 low"]
    pub lut530l: crate::Reg<lut530l::LUT530L_SPEC>,
    #[doc = "0x2094 - Graphic MMU LUT entry 530 high"]
    pub lut530h: crate::Reg<lut530h::LUT530H_SPEC>,
    #[doc = "0x2098 - Graphic MMU LUT entry 531 low"]
    pub lut531l: crate::Reg<lut531l::LUT531L_SPEC>,
    #[doc = "0x209c - Graphic MMU LUT entry 531 high"]
    pub lut531h: crate::Reg<lut531h::LUT531H_SPEC>,
    #[doc = "0x20a0 - Graphic MMU LUT entry 532 low"]
    pub lut532l: crate::Reg<lut532l::LUT532L_SPEC>,
    #[doc = "0x20a4 - Graphic MMU LUT entry 532 high"]
    pub lut532h: crate::Reg<lut532h::LUT532H_SPEC>,
    #[doc = "0x20a8 - Graphic MMU LUT entry 533 low"]
    pub lut533l: crate::Reg<lut533l::LUT533L_SPEC>,
    #[doc = "0x20ac - Graphic MMU LUT entry 533 high"]
    pub lut533h: crate::Reg<lut533h::LUT533H_SPEC>,
    #[doc = "0x20b0 - Graphic MMU LUT entry 534 low"]
    pub lut534l: crate::Reg<lut534l::LUT534L_SPEC>,
    #[doc = "0x20b4 - Graphic MMU LUT entry 534 high"]
    pub lut534h: crate::Reg<lut534h::LUT534H_SPEC>,
    #[doc = "0x20b8 - Graphic MMU LUT entry 535 low"]
    pub lut535l: crate::Reg<lut535l::LUT535L_SPEC>,
    #[doc = "0x20bc - Graphic MMU LUT entry 535 high"]
    pub lut535h: crate::Reg<lut535h::LUT535H_SPEC>,
    #[doc = "0x20c0 - Graphic MMU LUT entry 536 low"]
    pub lut536l: crate::Reg<lut536l::LUT536L_SPEC>,
    #[doc = "0x20c4 - Graphic MMU LUT entry 536 high"]
    pub lut536h: crate::Reg<lut536h::LUT536H_SPEC>,
    #[doc = "0x20c8 - Graphic MMU LUT entry 537 low"]
    pub lut537l: crate::Reg<lut537l::LUT537L_SPEC>,
    #[doc = "0x20cc - Graphic MMU LUT entry 537 high"]
    pub lut537h: crate::Reg<lut537h::LUT537H_SPEC>,
    #[doc = "0x20d0 - Graphic MMU LUT entry 538 low"]
    pub lut538l: crate::Reg<lut538l::LUT538L_SPEC>,
    #[doc = "0x20d4 - Graphic MMU LUT entry 538 high"]
    pub lut538h: crate::Reg<lut538h::LUT538H_SPEC>,
    #[doc = "0x20d8 - Graphic MMU LUT entry 539 low"]
    pub lut539l: crate::Reg<lut539l::LUT539L_SPEC>,
    #[doc = "0x20dc - Graphic MMU LUT entry 539 high"]
    pub lut539h: crate::Reg<lut539h::LUT539H_SPEC>,
    #[doc = "0x20e0 - Graphic MMU LUT entry 540 low"]
    pub lut540l: crate::Reg<lut540l::LUT540L_SPEC>,
    #[doc = "0x20e4 - Graphic MMU LUT entry 540 high"]
    pub lut540h: crate::Reg<lut540h::LUT540H_SPEC>,
    #[doc = "0x20e8 - Graphic MMU LUT entry 541 low"]
    pub lut541l: crate::Reg<lut541l::LUT541L_SPEC>,
    #[doc = "0x20ec - Graphic MMU LUT entry 541 high"]
    pub lut541h: crate::Reg<lut541h::LUT541H_SPEC>,
    #[doc = "0x20f0 - Graphic MMU LUT entry 542 low"]
    pub lut542l: crate::Reg<lut542l::LUT542L_SPEC>,
    #[doc = "0x20f4 - Graphic MMU LUT entry 542 high"]
    pub lut542h: crate::Reg<lut542h::LUT542H_SPEC>,
    #[doc = "0x20f8 - Graphic MMU LUT entry 543 low"]
    pub lut543l: crate::Reg<lut543l::LUT543L_SPEC>,
    #[doc = "0x20fc - Graphic MMU LUT entry 543 high"]
    pub lut543h: crate::Reg<lut543h::LUT543H_SPEC>,
    #[doc = "0x2100 - Graphic MMU LUT entry 544 low"]
    pub lut544l: crate::Reg<lut544l::LUT544L_SPEC>,
    #[doc = "0x2104 - Graphic MMU LUT entry 544 high"]
    pub lut544h: crate::Reg<lut544h::LUT544H_SPEC>,
    #[doc = "0x2108 - Graphic MMU LUT entry 545 low"]
    pub lut545l: crate::Reg<lut545l::LUT545L_SPEC>,
    #[doc = "0x210c - Graphic MMU LUT entry 545 high"]
    pub lut545h: crate::Reg<lut545h::LUT545H_SPEC>,
    #[doc = "0x2110 - Graphic MMU LUT entry 546 low"]
    pub lut546l: crate::Reg<lut546l::LUT546L_SPEC>,
    #[doc = "0x2114 - Graphic MMU LUT entry 546 high"]
    pub lut546h: crate::Reg<lut546h::LUT546H_SPEC>,
    #[doc = "0x2118 - Graphic MMU LUT entry 547 low"]
    pub lut547l: crate::Reg<lut547l::LUT547L_SPEC>,
    #[doc = "0x211c - Graphic MMU LUT entry 547 high"]
    pub lut547h: crate::Reg<lut547h::LUT547H_SPEC>,
    #[doc = "0x2120 - Graphic MMU LUT entry 548 low"]
    pub lut548l: crate::Reg<lut548l::LUT548L_SPEC>,
    #[doc = "0x2124 - Graphic MMU LUT entry 548 high"]
    pub lut548h: crate::Reg<lut548h::LUT548H_SPEC>,
    #[doc = "0x2128 - Graphic MMU LUT entry 549 low"]
    pub lut549l: crate::Reg<lut549l::LUT549L_SPEC>,
    #[doc = "0x212c - Graphic MMU LUT entry 549 high"]
    pub lut549h: crate::Reg<lut549h::LUT549H_SPEC>,
    #[doc = "0x2130 - Graphic MMU LUT entry 550 low"]
    pub lut550l: crate::Reg<lut550l::LUT550L_SPEC>,
    #[doc = "0x2134 - Graphic MMU LUT entry 550 high"]
    pub lut550h: crate::Reg<lut550h::LUT550H_SPEC>,
    #[doc = "0x2138 - Graphic MMU LUT entry 551 low"]
    pub lut551l: crate::Reg<lut551l::LUT551L_SPEC>,
    #[doc = "0x213c - Graphic MMU LUT entry 551 high"]
    pub lut551h: crate::Reg<lut551h::LUT551H_SPEC>,
    #[doc = "0x2140 - Graphic MMU LUT entry 552 low"]
    pub lut552l: crate::Reg<lut552l::LUT552L_SPEC>,
    #[doc = "0x2144 - Graphic MMU LUT entry 552 high"]
    pub lut552h: crate::Reg<lut552h::LUT552H_SPEC>,
    #[doc = "0x2148 - Graphic MMU LUT entry 553 low"]
    pub lut553l: crate::Reg<lut553l::LUT553L_SPEC>,
    #[doc = "0x214c - Graphic MMU LUT entry 553 high"]
    pub lut553h: crate::Reg<lut553h::LUT553H_SPEC>,
    #[doc = "0x2150 - Graphic MMU LUT entry 554 low"]
    pub lut554l: crate::Reg<lut554l::LUT554L_SPEC>,
    #[doc = "0x2154 - Graphic MMU LUT entry 554 high"]
    pub lut554h: crate::Reg<lut554h::LUT554H_SPEC>,
    #[doc = "0x2158 - Graphic MMU LUT entry 555 low"]
    pub lut555l: crate::Reg<lut555l::LUT555L_SPEC>,
    #[doc = "0x215c - Graphic MMU LUT entry 555 high"]
    pub lut555h: crate::Reg<lut555h::LUT555H_SPEC>,
    #[doc = "0x2160 - Graphic MMU LUT entry 556 low"]
    pub lut556l: crate::Reg<lut556l::LUT556L_SPEC>,
    #[doc = "0x2164 - Graphic MMU LUT entry 556 high"]
    pub lut556h: crate::Reg<lut556h::LUT556H_SPEC>,
    #[doc = "0x2168 - Graphic MMU LUT entry 557 low"]
    pub lut557l: crate::Reg<lut557l::LUT557L_SPEC>,
    #[doc = "0x216c - Graphic MMU LUT entry 557 high"]
    pub lut557h: crate::Reg<lut557h::LUT557H_SPEC>,
    #[doc = "0x2170 - Graphic MMU LUT entry 558 low"]
    pub lut558l: crate::Reg<lut558l::LUT558L_SPEC>,
    #[doc = "0x2174 - Graphic MMU LUT entry 558 high"]
    pub lut558h: crate::Reg<lut558h::LUT558H_SPEC>,
    #[doc = "0x2178 - Graphic MMU LUT entry 559 low"]
    pub lut559l: crate::Reg<lut559l::LUT559L_SPEC>,
    #[doc = "0x217c - Graphic MMU LUT entry 559 high"]
    pub lut559h: crate::Reg<lut559h::LUT559H_SPEC>,
    #[doc = "0x2180 - Graphic MMU LUT entry 560 low"]
    pub lut560l: crate::Reg<lut560l::LUT560L_SPEC>,
    #[doc = "0x2184 - Graphic MMU LUT entry 560 high"]
    pub lut560h: crate::Reg<lut560h::LUT560H_SPEC>,
    #[doc = "0x2188 - Graphic MMU LUT entry 561 low"]
    pub lut561l: crate::Reg<lut561l::LUT561L_SPEC>,
    #[doc = "0x218c - Graphic MMU LUT entry 561 high"]
    pub lut561h: crate::Reg<lut561h::LUT561H_SPEC>,
    #[doc = "0x2190 - Graphic MMU LUT entry 562 low"]
    pub lut562l: crate::Reg<lut562l::LUT562L_SPEC>,
    #[doc = "0x2194 - Graphic MMU LUT entry 562 high"]
    pub lut562h: crate::Reg<lut562h::LUT562H_SPEC>,
    #[doc = "0x2198 - Graphic MMU LUT entry 563 low"]
    pub lut563l: crate::Reg<lut563l::LUT563L_SPEC>,
    #[doc = "0x219c - Graphic MMU LUT entry 563 high"]
    pub lut563h: crate::Reg<lut563h::LUT563H_SPEC>,
    #[doc = "0x21a0 - Graphic MMU LUT entry 564 low"]
    pub lut564l: crate::Reg<lut564l::LUT564L_SPEC>,
    #[doc = "0x21a4 - Graphic MMU LUT entry 564 high"]
    pub lut564h: crate::Reg<lut564h::LUT564H_SPEC>,
    #[doc = "0x21a8 - Graphic MMU LUT entry 565 low"]
    pub lut565l: crate::Reg<lut565l::LUT565L_SPEC>,
    #[doc = "0x21ac - Graphic MMU LUT entry 565 high"]
    pub lut565h: crate::Reg<lut565h::LUT565H_SPEC>,
    #[doc = "0x21b0 - Graphic MMU LUT entry 566 low"]
    pub lut566l: crate::Reg<lut566l::LUT566L_SPEC>,
    #[doc = "0x21b4 - Graphic MMU LUT entry 566 high"]
    pub lut566h: crate::Reg<lut566h::LUT566H_SPEC>,
    #[doc = "0x21b8 - Graphic MMU LUT entry 567 low"]
    pub lut567l: crate::Reg<lut567l::LUT567L_SPEC>,
    #[doc = "0x21bc - Graphic MMU LUT entry 567 high"]
    pub lut567h: crate::Reg<lut567h::LUT567H_SPEC>,
    #[doc = "0x21c0 - Graphic MMU LUT entry 568 low"]
    pub lut568l: crate::Reg<lut568l::LUT568L_SPEC>,
    #[doc = "0x21c4 - Graphic MMU LUT entry 568 high"]
    pub lut568h: crate::Reg<lut568h::LUT568H_SPEC>,
    #[doc = "0x21c8 - Graphic MMU LUT entry 569 low"]
    pub lut569l: crate::Reg<lut569l::LUT569L_SPEC>,
    #[doc = "0x21cc - Graphic MMU LUT entry 569 high"]
    pub lut569h: crate::Reg<lut569h::LUT569H_SPEC>,
    #[doc = "0x21d0 - Graphic MMU LUT entry 570 low"]
    pub lut570l: crate::Reg<lut570l::LUT570L_SPEC>,
    #[doc = "0x21d4 - Graphic MMU LUT entry 570 high"]
    pub lut570h: crate::Reg<lut570h::LUT570H_SPEC>,
    #[doc = "0x21d8 - Graphic MMU LUT entry 571 low"]
    pub lut571l: crate::Reg<lut571l::LUT571L_SPEC>,
    #[doc = "0x21dc - Graphic MMU LUT entry 571 high"]
    pub lut571h: crate::Reg<lut571h::LUT571H_SPEC>,
    #[doc = "0x21e0 - Graphic MMU LUT entry 572 low"]
    pub lut572l: crate::Reg<lut572l::LUT572L_SPEC>,
    #[doc = "0x21e4 - Graphic MMU LUT entry 572 high"]
    pub lut572h: crate::Reg<lut572h::LUT572H_SPEC>,
    #[doc = "0x21e8 - Graphic MMU LUT entry 573 low"]
    pub lut573l: crate::Reg<lut573l::LUT573L_SPEC>,
    #[doc = "0x21ec - Graphic MMU LUT entry 573 high"]
    pub lut573h: crate::Reg<lut573h::LUT573H_SPEC>,
    #[doc = "0x21f0 - Graphic MMU LUT entry 574 low"]
    pub lut574l: crate::Reg<lut574l::LUT574L_SPEC>,
    #[doc = "0x21f4 - Graphic MMU LUT entry 574 high"]
    pub lut574h: crate::Reg<lut574h::LUT574H_SPEC>,
    #[doc = "0x21f8 - Graphic MMU LUT entry 575 low"]
    pub lut575l: crate::Reg<lut575l::LUT575L_SPEC>,
    #[doc = "0x21fc - Graphic MMU LUT entry 575 high"]
    pub lut575h: crate::Reg<lut575h::LUT575H_SPEC>,
    #[doc = "0x2200 - Graphic MMU LUT entry 576 low"]
    pub lut576l: crate::Reg<lut576l::LUT576L_SPEC>,
    #[doc = "0x2204 - Graphic MMU LUT entry 576 high"]
    pub lut576h: crate::Reg<lut576h::LUT576H_SPEC>,
    #[doc = "0x2208 - Graphic MMU LUT entry 577 low"]
    pub lut577l: crate::Reg<lut577l::LUT577L_SPEC>,
    #[doc = "0x220c - Graphic MMU LUT entry 577 high"]
    pub lut577h: crate::Reg<lut577h::LUT577H_SPEC>,
    #[doc = "0x2210 - Graphic MMU LUT entry 578 low"]
    pub lut578l: crate::Reg<lut578l::LUT578L_SPEC>,
    #[doc = "0x2214 - Graphic MMU LUT entry 578 high"]
    pub lut578h: crate::Reg<lut578h::LUT578H_SPEC>,
    #[doc = "0x2218 - Graphic MMU LUT entry 579 low"]
    pub lut579l: crate::Reg<lut579l::LUT579L_SPEC>,
    #[doc = "0x221c - Graphic MMU LUT entry 579 high"]
    pub lut579h: crate::Reg<lut579h::LUT579H_SPEC>,
    #[doc = "0x2220 - Graphic MMU LUT entry 580 low"]
    pub lut580l: crate::Reg<lut580l::LUT580L_SPEC>,
    #[doc = "0x2224 - Graphic MMU LUT entry 580 high"]
    pub lut580h: crate::Reg<lut580h::LUT580H_SPEC>,
    #[doc = "0x2228 - Graphic MMU LUT entry 581 low"]
    pub lut581l: crate::Reg<lut581l::LUT581L_SPEC>,
    #[doc = "0x222c - Graphic MMU LUT entry 581 high"]
    pub lut581h: crate::Reg<lut581h::LUT581H_SPEC>,
    #[doc = "0x2230 - Graphic MMU LUT entry 582 low"]
    pub lut582l: crate::Reg<lut582l::LUT582L_SPEC>,
    #[doc = "0x2234 - Graphic MMU LUT entry 582 high"]
    pub lut582h: crate::Reg<lut582h::LUT582H_SPEC>,
    #[doc = "0x2238 - Graphic MMU LUT entry 583 low"]
    pub lut583l: crate::Reg<lut583l::LUT583L_SPEC>,
    #[doc = "0x223c - Graphic MMU LUT entry 583 high"]
    pub lut583h: crate::Reg<lut583h::LUT583H_SPEC>,
    #[doc = "0x2240 - Graphic MMU LUT entry 584 low"]
    pub lut584l: crate::Reg<lut584l::LUT584L_SPEC>,
    #[doc = "0x2244 - Graphic MMU LUT entry 584 high"]
    pub lut584h: crate::Reg<lut584h::LUT584H_SPEC>,
    #[doc = "0x2248 - Graphic MMU LUT entry 585 low"]
    pub lut585l: crate::Reg<lut585l::LUT585L_SPEC>,
    #[doc = "0x224c - Graphic MMU LUT entry 585 high"]
    pub lut585h: crate::Reg<lut585h::LUT585H_SPEC>,
    #[doc = "0x2250 - Graphic MMU LUT entry 586 low"]
    pub lut586l: crate::Reg<lut586l::LUT586L_SPEC>,
    #[doc = "0x2254 - Graphic MMU LUT entry 586 high"]
    pub lut586h: crate::Reg<lut586h::LUT586H_SPEC>,
    #[doc = "0x2258 - Graphic MMU LUT entry 587 low"]
    pub lut587l: crate::Reg<lut587l::LUT587L_SPEC>,
    #[doc = "0x225c - Graphic MMU LUT entry 587 high"]
    pub lut587h: crate::Reg<lut587h::LUT587H_SPEC>,
    #[doc = "0x2260 - Graphic MMU LUT entry 588 low"]
    pub lut588l: crate::Reg<lut588l::LUT588L_SPEC>,
    #[doc = "0x2264 - Graphic MMU LUT entry 588 high"]
    pub lut588h: crate::Reg<lut588h::LUT588H_SPEC>,
    #[doc = "0x2268 - Graphic MMU LUT entry 589 low"]
    pub lut589l: crate::Reg<lut589l::LUT589L_SPEC>,
    #[doc = "0x226c - Graphic MMU LUT entry 589 high"]
    pub lut589h: crate::Reg<lut589h::LUT589H_SPEC>,
    #[doc = "0x2270 - Graphic MMU LUT entry 590 low"]
    pub lut590l: crate::Reg<lut590l::LUT590L_SPEC>,
    #[doc = "0x2274 - Graphic MMU LUT entry 590 high"]
    pub lut590h: crate::Reg<lut590h::LUT590H_SPEC>,
    #[doc = "0x2278 - Graphic MMU LUT entry 591 low"]
    pub lut591l: crate::Reg<lut591l::LUT591L_SPEC>,
    #[doc = "0x227c - Graphic MMU LUT entry 591 high"]
    pub lut591h: crate::Reg<lut591h::LUT591H_SPEC>,
    #[doc = "0x2280 - Graphic MMU LUT entry 592 low"]
    pub lut592l: crate::Reg<lut592l::LUT592L_SPEC>,
    #[doc = "0x2284 - Graphic MMU LUT entry 592 high"]
    pub lut592h: crate::Reg<lut592h::LUT592H_SPEC>,
    #[doc = "0x2288 - Graphic MMU LUT entry 593 low"]
    pub lut593l: crate::Reg<lut593l::LUT593L_SPEC>,
    #[doc = "0x228c - Graphic MMU LUT entry 593 high"]
    pub lut593h: crate::Reg<lut593h::LUT593H_SPEC>,
    #[doc = "0x2290 - Graphic MMU LUT entry 594 low"]
    pub lut594l: crate::Reg<lut594l::LUT594L_SPEC>,
    #[doc = "0x2294 - Graphic MMU LUT entry 594 high"]
    pub lut594h: crate::Reg<lut594h::LUT594H_SPEC>,
    #[doc = "0x2298 - Graphic MMU LUT entry 595 low"]
    pub lut595l: crate::Reg<lut595l::LUT595L_SPEC>,
    #[doc = "0x229c - Graphic MMU LUT entry 595 high"]
    pub lut595h: crate::Reg<lut595h::LUT595H_SPEC>,
    #[doc = "0x22a0 - Graphic MMU LUT entry 596 low"]
    pub lut596l: crate::Reg<lut596l::LUT596L_SPEC>,
    #[doc = "0x22a4 - Graphic MMU LUT entry 596 high"]
    pub lut596h: crate::Reg<lut596h::LUT596H_SPEC>,
    #[doc = "0x22a8 - Graphic MMU LUT entry 597 low"]
    pub lut597l: crate::Reg<lut597l::LUT597L_SPEC>,
    #[doc = "0x22ac - Graphic MMU LUT entry 597 high"]
    pub lut597h: crate::Reg<lut597h::LUT597H_SPEC>,
    #[doc = "0x22b0 - Graphic MMU LUT entry 598 low"]
    pub lut598l: crate::Reg<lut598l::LUT598L_SPEC>,
    #[doc = "0x22b4 - Graphic MMU LUT entry 598 high"]
    pub lut598h: crate::Reg<lut598h::LUT598H_SPEC>,
    #[doc = "0x22b8 - Graphic MMU LUT entry 599 low"]
    pub lut599l: crate::Reg<lut599l::LUT599L_SPEC>,
    #[doc = "0x22bc - Graphic MMU LUT entry 599 high"]
    pub lut599h: crate::Reg<lut599h::LUT599H_SPEC>,
    #[doc = "0x22c0 - Graphic MMU LUT entry 600 low"]
    pub lut600l: crate::Reg<lut600l::LUT600L_SPEC>,
    #[doc = "0x22c4 - Graphic MMU LUT entry 600 high"]
    pub lut600h: crate::Reg<lut600h::LUT600H_SPEC>,
    #[doc = "0x22c8 - Graphic MMU LUT entry 601 low"]
    pub lut601l: crate::Reg<lut601l::LUT601L_SPEC>,
    #[doc = "0x22cc - Graphic MMU LUT entry 601 high"]
    pub lut601h: crate::Reg<lut601h::LUT601H_SPEC>,
    #[doc = "0x22d0 - Graphic MMU LUT entry 602 low"]
    pub lut602l: crate::Reg<lut602l::LUT602L_SPEC>,
    #[doc = "0x22d4 - Graphic MMU LUT entry 602 high"]
    pub lut602h: crate::Reg<lut602h::LUT602H_SPEC>,
    #[doc = "0x22d8 - Graphic MMU LUT entry 603 low"]
    pub lut603l: crate::Reg<lut603l::LUT603L_SPEC>,
    #[doc = "0x22dc - Graphic MMU LUT entry 603 high"]
    pub lut603h: crate::Reg<lut603h::LUT603H_SPEC>,
    #[doc = "0x22e0 - Graphic MMU LUT entry 604 low"]
    pub lut604l: crate::Reg<lut604l::LUT604L_SPEC>,
    #[doc = "0x22e4 - Graphic MMU LUT entry 604 high"]
    pub lut604h: crate::Reg<lut604h::LUT604H_SPEC>,
    #[doc = "0x22e8 - Graphic MMU LUT entry 605 low"]
    pub lut605l: crate::Reg<lut605l::LUT605L_SPEC>,
    #[doc = "0x22ec - Graphic MMU LUT entry 605 high"]
    pub lut605h: crate::Reg<lut605h::LUT605H_SPEC>,
    #[doc = "0x22f0 - Graphic MMU LUT entry 606 low"]
    pub lut606l: crate::Reg<lut606l::LUT606L_SPEC>,
    #[doc = "0x22f4 - Graphic MMU LUT entry 606 high"]
    pub lut606h: crate::Reg<lut606h::LUT606H_SPEC>,
    #[doc = "0x22f8 - Graphic MMU LUT entry 607 low"]
    pub lut607l: crate::Reg<lut607l::LUT607L_SPEC>,
    #[doc = "0x22fc - Graphic MMU LUT entry 607 high"]
    pub lut607h: crate::Reg<lut607h::LUT607H_SPEC>,
    #[doc = "0x2300 - Graphic MMU LUT entry 608 low"]
    pub lut608l: crate::Reg<lut608l::LUT608L_SPEC>,
    #[doc = "0x2304 - Graphic MMU LUT entry 608 high"]
    pub lut608h: crate::Reg<lut608h::LUT608H_SPEC>,
    #[doc = "0x2308 - Graphic MMU LUT entry 609 low"]
    pub lut609l: crate::Reg<lut609l::LUT609L_SPEC>,
    #[doc = "0x230c - Graphic MMU LUT entry 609 high"]
    pub lut609h: crate::Reg<lut609h::LUT609H_SPEC>,
    #[doc = "0x2310 - Graphic MMU LUT entry 610 low"]
    pub lut610l: crate::Reg<lut610l::LUT610L_SPEC>,
    #[doc = "0x2314 - Graphic MMU LUT entry 610 high"]
    pub lut610h: crate::Reg<lut610h::LUT610H_SPEC>,
    #[doc = "0x2318 - Graphic MMU LUT entry 611 low"]
    pub lut611l: crate::Reg<lut611l::LUT611L_SPEC>,
    #[doc = "0x231c - Graphic MMU LUT entry 611 high"]
    pub lut611h: crate::Reg<lut611h::LUT611H_SPEC>,
    #[doc = "0x2320 - Graphic MMU LUT entry 612 low"]
    pub lut612l: crate::Reg<lut612l::LUT612L_SPEC>,
    #[doc = "0x2324 - Graphic MMU LUT entry 612 high"]
    pub lut612h: crate::Reg<lut612h::LUT612H_SPEC>,
    #[doc = "0x2328 - Graphic MMU LUT entry 613 low"]
    pub lut613l: crate::Reg<lut613l::LUT613L_SPEC>,
    #[doc = "0x232c - Graphic MMU LUT entry 613 high"]
    pub lut613h: crate::Reg<lut613h::LUT613H_SPEC>,
    #[doc = "0x2330 - Graphic MMU LUT entry 614 low"]
    pub lut614l: crate::Reg<lut614l::LUT614L_SPEC>,
    #[doc = "0x2334 - Graphic MMU LUT entry 614 high"]
    pub lut614h: crate::Reg<lut614h::LUT614H_SPEC>,
    #[doc = "0x2338 - Graphic MMU LUT entry 615 low"]
    pub lut615l: crate::Reg<lut615l::LUT615L_SPEC>,
    #[doc = "0x233c - Graphic MMU LUT entry 615 high"]
    pub lut615h: crate::Reg<lut615h::LUT615H_SPEC>,
    #[doc = "0x2340 - Graphic MMU LUT entry 616 low"]
    pub lut616l: crate::Reg<lut616l::LUT616L_SPEC>,
    #[doc = "0x2344 - Graphic MMU LUT entry 616 high"]
    pub lut616h: crate::Reg<lut616h::LUT616H_SPEC>,
    #[doc = "0x2348 - Graphic MMU LUT entry 617 low"]
    pub lut617l: crate::Reg<lut617l::LUT617L_SPEC>,
    #[doc = "0x234c - Graphic MMU LUT entry 617 high"]
    pub lut617h: crate::Reg<lut617h::LUT617H_SPEC>,
    #[doc = "0x2350 - Graphic MMU LUT entry 618 low"]
    pub lut618l: crate::Reg<lut618l::LUT618L_SPEC>,
    #[doc = "0x2354 - Graphic MMU LUT entry 618 high"]
    pub lut618h: crate::Reg<lut618h::LUT618H_SPEC>,
    #[doc = "0x2358 - Graphic MMU LUT entry 619 low"]
    pub lut619l: crate::Reg<lut619l::LUT619L_SPEC>,
    #[doc = "0x235c - Graphic MMU LUT entry 619 high"]
    pub lut619h: crate::Reg<lut619h::LUT619H_SPEC>,
    #[doc = "0x2360 - Graphic MMU LUT entry 620 low"]
    pub lut620l: crate::Reg<lut620l::LUT620L_SPEC>,
    #[doc = "0x2364 - Graphic MMU LUT entry 620 high"]
    pub lut620h: crate::Reg<lut620h::LUT620H_SPEC>,
    #[doc = "0x2368 - Graphic MMU LUT entry 621 low"]
    pub lut621l: crate::Reg<lut621l::LUT621L_SPEC>,
    #[doc = "0x236c - Graphic MMU LUT entry 621 high"]
    pub lut621h: crate::Reg<lut621h::LUT621H_SPEC>,
    #[doc = "0x2370 - Graphic MMU LUT entry 622 low"]
    pub lut622l: crate::Reg<lut622l::LUT622L_SPEC>,
    #[doc = "0x2374 - Graphic MMU LUT entry 622 high"]
    pub lut622h: crate::Reg<lut622h::LUT622H_SPEC>,
    #[doc = "0x2378 - Graphic MMU LUT entry 623 low"]
    pub lut623l: crate::Reg<lut623l::LUT623L_SPEC>,
    #[doc = "0x237c - Graphic MMU LUT entry 623 high"]
    pub lut623h: crate::Reg<lut623h::LUT623H_SPEC>,
    #[doc = "0x2380 - Graphic MMU LUT entry 624 low"]
    pub lut624l: crate::Reg<lut624l::LUT624L_SPEC>,
    #[doc = "0x2384 - Graphic MMU LUT entry 624 high"]
    pub lut624h: crate::Reg<lut624h::LUT624H_SPEC>,
    #[doc = "0x2388 - Graphic MMU LUT entry 625 low"]
    pub lut625l: crate::Reg<lut625l::LUT625L_SPEC>,
    #[doc = "0x238c - Graphic MMU LUT entry 625 high"]
    pub lut625h: crate::Reg<lut625h::LUT625H_SPEC>,
    #[doc = "0x2390 - Graphic MMU LUT entry 626 low"]
    pub lut626l: crate::Reg<lut626l::LUT626L_SPEC>,
    #[doc = "0x2394 - Graphic MMU LUT entry 626 high"]
    pub lut626h: crate::Reg<lut626h::LUT626H_SPEC>,
    #[doc = "0x2398 - Graphic MMU LUT entry 627 low"]
    pub lut627l: crate::Reg<lut627l::LUT627L_SPEC>,
    #[doc = "0x239c - Graphic MMU LUT entry 627 high"]
    pub lut627h: crate::Reg<lut627h::LUT627H_SPEC>,
    #[doc = "0x23a0 - Graphic MMU LUT entry 628 low"]
    pub lut628l: crate::Reg<lut628l::LUT628L_SPEC>,
    #[doc = "0x23a4 - Graphic MMU LUT entry 628 high"]
    pub lut628h: crate::Reg<lut628h::LUT628H_SPEC>,
    #[doc = "0x23a8 - Graphic MMU LUT entry 629 low"]
    pub lut629l: crate::Reg<lut629l::LUT629L_SPEC>,
    #[doc = "0x23ac - Graphic MMU LUT entry 629 high"]
    pub lut629h: crate::Reg<lut629h::LUT629H_SPEC>,
    #[doc = "0x23b0 - Graphic MMU LUT entry 630 low"]
    pub lut630l: crate::Reg<lut630l::LUT630L_SPEC>,
    #[doc = "0x23b4 - Graphic MMU LUT entry 630 high"]
    pub lut630h: crate::Reg<lut630h::LUT630H_SPEC>,
    #[doc = "0x23b8 - Graphic MMU LUT entry 631 low"]
    pub lut631l: crate::Reg<lut631l::LUT631L_SPEC>,
    #[doc = "0x23bc - Graphic MMU LUT entry 631 high"]
    pub lut631h: crate::Reg<lut631h::LUT631H_SPEC>,
    #[doc = "0x23c0 - Graphic MMU LUT entry 632 low"]
    pub lut632l: crate::Reg<lut632l::LUT632L_SPEC>,
    #[doc = "0x23c4 - Graphic MMU LUT entry 632 high"]
    pub lut632h: crate::Reg<lut632h::LUT632H_SPEC>,
    #[doc = "0x23c8 - Graphic MMU LUT entry 633 low"]
    pub lut633l: crate::Reg<lut633l::LUT633L_SPEC>,
    #[doc = "0x23cc - Graphic MMU LUT entry 633 high"]
    pub lut633h: crate::Reg<lut633h::LUT633H_SPEC>,
    #[doc = "0x23d0 - Graphic MMU LUT entry 634 low"]
    pub lut634l: crate::Reg<lut634l::LUT634L_SPEC>,
    #[doc = "0x23d4 - Graphic MMU LUT entry 634 high"]
    pub lut634h: crate::Reg<lut634h::LUT634H_SPEC>,
    #[doc = "0x23d8 - Graphic MMU LUT entry 635 low"]
    pub lut635l: crate::Reg<lut635l::LUT635L_SPEC>,
    #[doc = "0x23dc - Graphic MMU LUT entry 635 high"]
    pub lut635h: crate::Reg<lut635h::LUT635H_SPEC>,
    #[doc = "0x23e0 - Graphic MMU LUT entry 636 low"]
    pub lut636l: crate::Reg<lut636l::LUT636L_SPEC>,
    #[doc = "0x23e4 - Graphic MMU LUT entry 636 high"]
    pub lut636h: crate::Reg<lut636h::LUT636H_SPEC>,
    #[doc = "0x23e8 - Graphic MMU LUT entry 637 low"]
    pub lut637l: crate::Reg<lut637l::LUT637L_SPEC>,
    #[doc = "0x23ec - Graphic MMU LUT entry 637 high"]
    pub lut637h: crate::Reg<lut637h::LUT637H_SPEC>,
    #[doc = "0x23f0 - Graphic MMU LUT entry 638 low"]
    pub lut638l: crate::Reg<lut638l::LUT638L_SPEC>,
    #[doc = "0x23f4 - Graphic MMU LUT entry 638 high"]
    pub lut638h: crate::Reg<lut638h::LUT638H_SPEC>,
    #[doc = "0x23f8 - Graphic MMU LUT entry 639 low"]
    pub lut639l: crate::Reg<lut639l::LUT639L_SPEC>,
    #[doc = "0x23fc - Graphic MMU LUT entry 639 high"]
    pub lut639h: crate::Reg<lut639h::LUT639H_SPEC>,
    #[doc = "0x2400 - Graphic MMU LUT entry 640 low"]
    pub lut640l: crate::Reg<lut640l::LUT640L_SPEC>,
    #[doc = "0x2404 - Graphic MMU LUT entry 640 high"]
    pub lut640h: crate::Reg<lut640h::LUT640H_SPEC>,
    #[doc = "0x2408 - Graphic MMU LUT entry 641 low"]
    pub lut641l: crate::Reg<lut641l::LUT641L_SPEC>,
    #[doc = "0x240c - Graphic MMU LUT entry 641 high"]
    pub lut641h: crate::Reg<lut641h::LUT641H_SPEC>,
    #[doc = "0x2410 - Graphic MMU LUT entry 642 low"]
    pub lut642l: crate::Reg<lut642l::LUT642L_SPEC>,
    #[doc = "0x2414 - Graphic MMU LUT entry 642 high"]
    pub lut642h: crate::Reg<lut642h::LUT642H_SPEC>,
    #[doc = "0x2418 - Graphic MMU LUT entry 643 low"]
    pub lut643l: crate::Reg<lut643l::LUT643L_SPEC>,
    #[doc = "0x241c - Graphic MMU LUT entry 643 high"]
    pub lut643h: crate::Reg<lut643h::LUT643H_SPEC>,
    #[doc = "0x2420 - Graphic MMU LUT entry 644 low"]
    pub lut644l: crate::Reg<lut644l::LUT644L_SPEC>,
    #[doc = "0x2424 - Graphic MMU LUT entry 644 high"]
    pub lut644h: crate::Reg<lut644h::LUT644H_SPEC>,
    #[doc = "0x2428 - Graphic MMU LUT entry 645 low"]
    pub lut645l: crate::Reg<lut645l::LUT645L_SPEC>,
    #[doc = "0x242c - Graphic MMU LUT entry 645 high"]
    pub lut645h: crate::Reg<lut645h::LUT645H_SPEC>,
    #[doc = "0x2430 - Graphic MMU LUT entry 646 low"]
    pub lut646l: crate::Reg<lut646l::LUT646L_SPEC>,
    #[doc = "0x2434 - Graphic MMU LUT entry 646 high"]
    pub lut646h: crate::Reg<lut646h::LUT646H_SPEC>,
    #[doc = "0x2438 - Graphic MMU LUT entry 647 low"]
    pub lut647l: crate::Reg<lut647l::LUT647L_SPEC>,
    #[doc = "0x243c - Graphic MMU LUT entry 647 high"]
    pub lut647h: crate::Reg<lut647h::LUT647H_SPEC>,
    #[doc = "0x2440 - Graphic MMU LUT entry 648 low"]
    pub lut648l: crate::Reg<lut648l::LUT648L_SPEC>,
    #[doc = "0x2444 - Graphic MMU LUT entry 648 high"]
    pub lut648h: crate::Reg<lut648h::LUT648H_SPEC>,
    #[doc = "0x2448 - Graphic MMU LUT entry 649 low"]
    pub lut649l: crate::Reg<lut649l::LUT649L_SPEC>,
    #[doc = "0x244c - Graphic MMU LUT entry 649 high"]
    pub lut649h: crate::Reg<lut649h::LUT649H_SPEC>,
    #[doc = "0x2450 - Graphic MMU LUT entry 650 low"]
    pub lut650l: crate::Reg<lut650l::LUT650L_SPEC>,
    #[doc = "0x2454 - Graphic MMU LUT entry 650 high"]
    pub lut650h: crate::Reg<lut650h::LUT650H_SPEC>,
    #[doc = "0x2458 - Graphic MMU LUT entry 651 low"]
    pub lut651l: crate::Reg<lut651l::LUT651L_SPEC>,
    #[doc = "0x245c - Graphic MMU LUT entry 651 high"]
    pub lut651h: crate::Reg<lut651h::LUT651H_SPEC>,
    #[doc = "0x2460 - Graphic MMU LUT entry 652 low"]
    pub lut652l: crate::Reg<lut652l::LUT652L_SPEC>,
    #[doc = "0x2464 - Graphic MMU LUT entry 652 high"]
    pub lut652h: crate::Reg<lut652h::LUT652H_SPEC>,
    #[doc = "0x2468 - Graphic MMU LUT entry 653 low"]
    pub lut653l: crate::Reg<lut653l::LUT653L_SPEC>,
    #[doc = "0x246c - Graphic MMU LUT entry 653 high"]
    pub lut653h: crate::Reg<lut653h::LUT653H_SPEC>,
    #[doc = "0x2470 - Graphic MMU LUT entry 654 low"]
    pub lut654l: crate::Reg<lut654l::LUT654L_SPEC>,
    #[doc = "0x2474 - Graphic MMU LUT entry 654 high"]
    pub lut654h: crate::Reg<lut654h::LUT654H_SPEC>,
    #[doc = "0x2478 - Graphic MMU LUT entry 655 low"]
    pub lut655l: crate::Reg<lut655l::LUT655L_SPEC>,
    #[doc = "0x247c - Graphic MMU LUT entry 655 high"]
    pub lut655h: crate::Reg<lut655h::LUT655H_SPEC>,
    #[doc = "0x2480 - Graphic MMU LUT entry 656 low"]
    pub lut656l: crate::Reg<lut656l::LUT656L_SPEC>,
    #[doc = "0x2484 - Graphic MMU LUT entry 656 high"]
    pub lut656h: crate::Reg<lut656h::LUT656H_SPEC>,
    #[doc = "0x2488 - Graphic MMU LUT entry 657 low"]
    pub lut657l: crate::Reg<lut657l::LUT657L_SPEC>,
    #[doc = "0x248c - Graphic MMU LUT entry 657 high"]
    pub lut657h: crate::Reg<lut657h::LUT657H_SPEC>,
    #[doc = "0x2490 - Graphic MMU LUT entry 658 low"]
    pub lut658l: crate::Reg<lut658l::LUT658L_SPEC>,
    #[doc = "0x2494 - Graphic MMU LUT entry 658 high"]
    pub lut658h: crate::Reg<lut658h::LUT658H_SPEC>,
    #[doc = "0x2498 - Graphic MMU LUT entry 659 low"]
    pub lut659l: crate::Reg<lut659l::LUT659L_SPEC>,
    #[doc = "0x249c - Graphic MMU LUT entry 659 high"]
    pub lut659h: crate::Reg<lut659h::LUT659H_SPEC>,
    #[doc = "0x24a0 - Graphic MMU LUT entry 660 low"]
    pub lut660l: crate::Reg<lut660l::LUT660L_SPEC>,
    #[doc = "0x24a4 - Graphic MMU LUT entry 660 high"]
    pub lut660h: crate::Reg<lut660h::LUT660H_SPEC>,
    #[doc = "0x24a8 - Graphic MMU LUT entry 661 low"]
    pub lut661l: crate::Reg<lut661l::LUT661L_SPEC>,
    #[doc = "0x24ac - Graphic MMU LUT entry 661 high"]
    pub lut661h: crate::Reg<lut661h::LUT661H_SPEC>,
    #[doc = "0x24b0 - Graphic MMU LUT entry 662 low"]
    pub lut662l: crate::Reg<lut662l::LUT662L_SPEC>,
    #[doc = "0x24b4 - Graphic MMU LUT entry 662 high"]
    pub lut662h: crate::Reg<lut662h::LUT662H_SPEC>,
    #[doc = "0x24b8 - Graphic MMU LUT entry 663 low"]
    pub lut663l: crate::Reg<lut663l::LUT663L_SPEC>,
    #[doc = "0x24bc - Graphic MMU LUT entry 663 high"]
    pub lut663h: crate::Reg<lut663h::LUT663H_SPEC>,
    #[doc = "0x24c0 - Graphic MMU LUT entry 664 low"]
    pub lut664l: crate::Reg<lut664l::LUT664L_SPEC>,
    #[doc = "0x24c4 - Graphic MMU LUT entry 664 high"]
    pub lut664h: crate::Reg<lut664h::LUT664H_SPEC>,
    #[doc = "0x24c8 - Graphic MMU LUT entry 665 low"]
    pub lut665l: crate::Reg<lut665l::LUT665L_SPEC>,
    #[doc = "0x24cc - Graphic MMU LUT entry 665 high"]
    pub lut665h: crate::Reg<lut665h::LUT665H_SPEC>,
    #[doc = "0x24d0 - Graphic MMU LUT entry 666 low"]
    pub lut666l: crate::Reg<lut666l::LUT666L_SPEC>,
    #[doc = "0x24d4 - Graphic MMU LUT entry 666 high"]
    pub lut666h: crate::Reg<lut666h::LUT666H_SPEC>,
    #[doc = "0x24d8 - Graphic MMU LUT entry 667 low"]
    pub lut667l: crate::Reg<lut667l::LUT667L_SPEC>,
    #[doc = "0x24dc - Graphic MMU LUT entry 667 high"]
    pub lut667h: crate::Reg<lut667h::LUT667H_SPEC>,
    #[doc = "0x24e0 - Graphic MMU LUT entry 668 low"]
    pub lut668l: crate::Reg<lut668l::LUT668L_SPEC>,
    #[doc = "0x24e4 - Graphic MMU LUT entry 668 high"]
    pub lut668h: crate::Reg<lut668h::LUT668H_SPEC>,
    #[doc = "0x24e8 - Graphic MMU LUT entry 669 low"]
    pub lut669l: crate::Reg<lut669l::LUT669L_SPEC>,
    #[doc = "0x24ec - Graphic MMU LUT entry 669 high"]
    pub lut669h: crate::Reg<lut669h::LUT669H_SPEC>,
    #[doc = "0x24f0 - Graphic MMU LUT entry 670 low"]
    pub lut670l: crate::Reg<lut670l::LUT670L_SPEC>,
    #[doc = "0x24f4 - Graphic MMU LUT entry 670 high"]
    pub lut670h: crate::Reg<lut670h::LUT670H_SPEC>,
    #[doc = "0x24f8 - Graphic MMU LUT entry 671 low"]
    pub lut671l: crate::Reg<lut671l::LUT671L_SPEC>,
    #[doc = "0x24fc - Graphic MMU LUT entry 671 high"]
    pub lut671h: crate::Reg<lut671h::LUT671H_SPEC>,
    #[doc = "0x2500 - Graphic MMU LUT entry 672 low"]
    pub lut672l: crate::Reg<lut672l::LUT672L_SPEC>,
    #[doc = "0x2504 - Graphic MMU LUT entry 672 high"]
    pub lut672h: crate::Reg<lut672h::LUT672H_SPEC>,
    #[doc = "0x2508 - Graphic MMU LUT entry 673 low"]
    pub lut673l: crate::Reg<lut673l::LUT673L_SPEC>,
    #[doc = "0x250c - Graphic MMU LUT entry 673 high"]
    pub lut673h: crate::Reg<lut673h::LUT673H_SPEC>,
    #[doc = "0x2510 - Graphic MMU LUT entry 674 low"]
    pub lut674l: crate::Reg<lut674l::LUT674L_SPEC>,
    #[doc = "0x2514 - Graphic MMU LUT entry 674 high"]
    pub lut674h: crate::Reg<lut674h::LUT674H_SPEC>,
    #[doc = "0x2518 - Graphic MMU LUT entry 675 low"]
    pub lut675l: crate::Reg<lut675l::LUT675L_SPEC>,
    #[doc = "0x251c - Graphic MMU LUT entry 675 high"]
    pub lut675h: crate::Reg<lut675h::LUT675H_SPEC>,
    #[doc = "0x2520 - Graphic MMU LUT entry 676 low"]
    pub lut676l: crate::Reg<lut676l::LUT676L_SPEC>,
    #[doc = "0x2524 - Graphic MMU LUT entry 676 high"]
    pub lut676h: crate::Reg<lut676h::LUT676H_SPEC>,
    #[doc = "0x2528 - Graphic MMU LUT entry 677 low"]
    pub lut677l: crate::Reg<lut677l::LUT677L_SPEC>,
    #[doc = "0x252c - Graphic MMU LUT entry 677 high"]
    pub lut677h: crate::Reg<lut677h::LUT677H_SPEC>,
    #[doc = "0x2530 - Graphic MMU LUT entry 678 low"]
    pub lut678l: crate::Reg<lut678l::LUT678L_SPEC>,
    #[doc = "0x2534 - Graphic MMU LUT entry 678 high"]
    pub lut678h: crate::Reg<lut678h::LUT678H_SPEC>,
    #[doc = "0x2538 - Graphic MMU LUT entry 679 low"]
    pub lut679l: crate::Reg<lut679l::LUT679L_SPEC>,
    #[doc = "0x253c - Graphic MMU LUT entry 679 high"]
    pub lut679h: crate::Reg<lut679h::LUT679H_SPEC>,
    #[doc = "0x2540 - Graphic MMU LUT entry 680 low"]
    pub lut680l: crate::Reg<lut680l::LUT680L_SPEC>,
    #[doc = "0x2544 - Graphic MMU LUT entry 680 high"]
    pub lut680h: crate::Reg<lut680h::LUT680H_SPEC>,
    #[doc = "0x2548 - Graphic MMU LUT entry 681 low"]
    pub lut681l: crate::Reg<lut681l::LUT681L_SPEC>,
    #[doc = "0x254c - Graphic MMU LUT entry 681 high"]
    pub lut681h: crate::Reg<lut681h::LUT681H_SPEC>,
    #[doc = "0x2550 - Graphic MMU LUT entry 682 low"]
    pub lut682l: crate::Reg<lut682l::LUT682L_SPEC>,
    #[doc = "0x2554 - Graphic MMU LUT entry 682 high"]
    pub lut682h: crate::Reg<lut682h::LUT682H_SPEC>,
    #[doc = "0x2558 - Graphic MMU LUT entry 683 low"]
    pub lut683l: crate::Reg<lut683l::LUT683L_SPEC>,
    #[doc = "0x255c - Graphic MMU LUT entry 683 high"]
    pub lut683h: crate::Reg<lut683h::LUT683H_SPEC>,
    #[doc = "0x2560 - Graphic MMU LUT entry 684 low"]
    pub lut684l: crate::Reg<lut684l::LUT684L_SPEC>,
    #[doc = "0x2564 - Graphic MMU LUT entry 684 high"]
    pub lut684h: crate::Reg<lut684h::LUT684H_SPEC>,
    #[doc = "0x2568 - Graphic MMU LUT entry 685 low"]
    pub lut685l: crate::Reg<lut685l::LUT685L_SPEC>,
    #[doc = "0x256c - Graphic MMU LUT entry 685 high"]
    pub lut685h: crate::Reg<lut685h::LUT685H_SPEC>,
    #[doc = "0x2570 - Graphic MMU LUT entry 686 low"]
    pub lut686l: crate::Reg<lut686l::LUT686L_SPEC>,
    #[doc = "0x2574 - Graphic MMU LUT entry 686 high"]
    pub lut686h: crate::Reg<lut686h::LUT686H_SPEC>,
    #[doc = "0x2578 - Graphic MMU LUT entry 687 low"]
    pub lut687l: crate::Reg<lut687l::LUT687L_SPEC>,
    #[doc = "0x257c - Graphic MMU LUT entry 687 high"]
    pub lut687h: crate::Reg<lut687h::LUT687H_SPEC>,
    #[doc = "0x2580 - Graphic MMU LUT entry 688 low"]
    pub lut688l: crate::Reg<lut688l::LUT688L_SPEC>,
    #[doc = "0x2584 - Graphic MMU LUT entry 688 high"]
    pub lut688h: crate::Reg<lut688h::LUT688H_SPEC>,
    #[doc = "0x2588 - Graphic MMU LUT entry 689 low"]
    pub lut689l: crate::Reg<lut689l::LUT689L_SPEC>,
    #[doc = "0x258c - Graphic MMU LUT entry 689 high"]
    pub lut689h: crate::Reg<lut689h::LUT689H_SPEC>,
    #[doc = "0x2590 - Graphic MMU LUT entry 690 low"]
    pub lut690l: crate::Reg<lut690l::LUT690L_SPEC>,
    #[doc = "0x2594 - Graphic MMU LUT entry 690 high"]
    pub lut690h: crate::Reg<lut690h::LUT690H_SPEC>,
    #[doc = "0x2598 - Graphic MMU LUT entry 691 low"]
    pub lut691l: crate::Reg<lut691l::LUT691L_SPEC>,
    #[doc = "0x259c - Graphic MMU LUT entry 691 high"]
    pub lut691h: crate::Reg<lut691h::LUT691H_SPEC>,
    #[doc = "0x25a0 - Graphic MMU LUT entry 692 low"]
    pub lut692l: crate::Reg<lut692l::LUT692L_SPEC>,
    #[doc = "0x25a4 - Graphic MMU LUT entry 692 high"]
    pub lut692h: crate::Reg<lut692h::LUT692H_SPEC>,
    #[doc = "0x25a8 - Graphic MMU LUT entry 693 low"]
    pub lut693l: crate::Reg<lut693l::LUT693L_SPEC>,
    #[doc = "0x25ac - Graphic MMU LUT entry 693 high"]
    pub lut693h: crate::Reg<lut693h::LUT693H_SPEC>,
    #[doc = "0x25b0 - Graphic MMU LUT entry 694 low"]
    pub lut694l: crate::Reg<lut694l::LUT694L_SPEC>,
    #[doc = "0x25b4 - Graphic MMU LUT entry 694 high"]
    pub lut694h: crate::Reg<lut694h::LUT694H_SPEC>,
    #[doc = "0x25b8 - Graphic MMU LUT entry 695 low"]
    pub lut695l: crate::Reg<lut695l::LUT695L_SPEC>,
    #[doc = "0x25bc - Graphic MMU LUT entry 695 high"]
    pub lut695h: crate::Reg<lut695h::LUT695H_SPEC>,
    #[doc = "0x25c0 - Graphic MMU LUT entry 696 low"]
    pub lut696l: crate::Reg<lut696l::LUT696L_SPEC>,
    #[doc = "0x25c4 - Graphic MMU LUT entry 696 high"]
    pub lut696h: crate::Reg<lut696h::LUT696H_SPEC>,
    #[doc = "0x25c8 - Graphic MMU LUT entry 697 low"]
    pub lut697l: crate::Reg<lut697l::LUT697L_SPEC>,
    #[doc = "0x25cc - Graphic MMU LUT entry 697 high"]
    pub lut697h: crate::Reg<lut697h::LUT697H_SPEC>,
    #[doc = "0x25d0 - Graphic MMU LUT entry 698 low"]
    pub lut698l: crate::Reg<lut698l::LUT698L_SPEC>,
    #[doc = "0x25d4 - Graphic MMU LUT entry 698 high"]
    pub lut698h: crate::Reg<lut698h::LUT698H_SPEC>,
    #[doc = "0x25d8 - Graphic MMU LUT entry 699 low"]
    pub lut699l: crate::Reg<lut699l::LUT699L_SPEC>,
    #[doc = "0x25dc - Graphic MMU LUT entry 699 high"]
    pub lut699h: crate::Reg<lut699h::LUT699H_SPEC>,
    #[doc = "0x25e0 - Graphic MMU LUT entry 700 low"]
    pub lut700l: crate::Reg<lut700l::LUT700L_SPEC>,
    #[doc = "0x25e4 - Graphic MMU LUT entry 700 high"]
    pub lut700h: crate::Reg<lut700h::LUT700H_SPEC>,
    #[doc = "0x25e8 - Graphic MMU LUT entry 701 low"]
    pub lut701l: crate::Reg<lut701l::LUT701L_SPEC>,
    #[doc = "0x25ec - Graphic MMU LUT entry 701 high"]
    pub lut701h: crate::Reg<lut701h::LUT701H_SPEC>,
    #[doc = "0x25f0 - Graphic MMU LUT entry 702 low"]
    pub lut702l: crate::Reg<lut702l::LUT702L_SPEC>,
    #[doc = "0x25f4 - Graphic MMU LUT entry 702 high"]
    pub lut702h: crate::Reg<lut702h::LUT702H_SPEC>,
    #[doc = "0x25f8 - Graphic MMU LUT entry 703 low"]
    pub lut703l: crate::Reg<lut703l::LUT703L_SPEC>,
    #[doc = "0x25fc - Graphic MMU LUT entry 703 high"]
    pub lut703h: crate::Reg<lut703h::LUT703H_SPEC>,
    #[doc = "0x2600 - Graphic MMU LUT entry 704 low"]
    pub lut704l: crate::Reg<lut704l::LUT704L_SPEC>,
    #[doc = "0x2604 - Graphic MMU LUT entry 704 high"]
    pub lut704h: crate::Reg<lut704h::LUT704H_SPEC>,
    #[doc = "0x2608 - Graphic MMU LUT entry 705 low"]
    pub lut705l: crate::Reg<lut705l::LUT705L_SPEC>,
    #[doc = "0x260c - Graphic MMU LUT entry 705 high"]
    pub lut705h: crate::Reg<lut705h::LUT705H_SPEC>,
    #[doc = "0x2610 - Graphic MMU LUT entry 706 low"]
    pub lut706l: crate::Reg<lut706l::LUT706L_SPEC>,
    #[doc = "0x2614 - Graphic MMU LUT entry 706 high"]
    pub lut706h: crate::Reg<lut706h::LUT706H_SPEC>,
    #[doc = "0x2618 - Graphic MMU LUT entry 707 low"]
    pub lut707l: crate::Reg<lut707l::LUT707L_SPEC>,
    #[doc = "0x261c - Graphic MMU LUT entry 707 high"]
    pub lut707h: crate::Reg<lut707h::LUT707H_SPEC>,
    #[doc = "0x2620 - Graphic MMU LUT entry 708 low"]
    pub lut708l: crate::Reg<lut708l::LUT708L_SPEC>,
    #[doc = "0x2624 - Graphic MMU LUT entry 708 high"]
    pub lut708h: crate::Reg<lut708h::LUT708H_SPEC>,
    #[doc = "0x2628 - Graphic MMU LUT entry 709 low"]
    pub lut709l: crate::Reg<lut709l::LUT709L_SPEC>,
    #[doc = "0x262c - Graphic MMU LUT entry 709 high"]
    pub lut709h: crate::Reg<lut709h::LUT709H_SPEC>,
    #[doc = "0x2630 - Graphic MMU LUT entry 710 low"]
    pub lut710l: crate::Reg<lut710l::LUT710L_SPEC>,
    #[doc = "0x2634 - Graphic MMU LUT entry 710 high"]
    pub lut710h: crate::Reg<lut710h::LUT710H_SPEC>,
    #[doc = "0x2638 - Graphic MMU LUT entry 711 low"]
    pub lut711l: crate::Reg<lut711l::LUT711L_SPEC>,
    #[doc = "0x263c - Graphic MMU LUT entry 711 high"]
    pub lut711h: crate::Reg<lut711h::LUT711H_SPEC>,
    #[doc = "0x2640 - Graphic MMU LUT entry 712 low"]
    pub lut712l: crate::Reg<lut712l::LUT712L_SPEC>,
    #[doc = "0x2644 - Graphic MMU LUT entry 712 high"]
    pub lut712h: crate::Reg<lut712h::LUT712H_SPEC>,
    #[doc = "0x2648 - Graphic MMU LUT entry 713 low"]
    pub lut713l: crate::Reg<lut713l::LUT713L_SPEC>,
    #[doc = "0x264c - Graphic MMU LUT entry 713 high"]
    pub lut713h: crate::Reg<lut713h::LUT713H_SPEC>,
    #[doc = "0x2650 - Graphic MMU LUT entry 714 low"]
    pub lut714l: crate::Reg<lut714l::LUT714L_SPEC>,
    #[doc = "0x2654 - Graphic MMU LUT entry 714 high"]
    pub lut714h: crate::Reg<lut714h::LUT714H_SPEC>,
    #[doc = "0x2658 - Graphic MMU LUT entry 715 low"]
    pub lut715l: crate::Reg<lut715l::LUT715L_SPEC>,
    #[doc = "0x265c - Graphic MMU LUT entry 715 high"]
    pub lut715h: crate::Reg<lut715h::LUT715H_SPEC>,
    #[doc = "0x2660 - Graphic MMU LUT entry 716 low"]
    pub lut716l: crate::Reg<lut716l::LUT716L_SPEC>,
    #[doc = "0x2664 - Graphic MMU LUT entry 716 high"]
    pub lut716h: crate::Reg<lut716h::LUT716H_SPEC>,
    #[doc = "0x2668 - Graphic MMU LUT entry 717 low"]
    pub lut717l: crate::Reg<lut717l::LUT717L_SPEC>,
    #[doc = "0x266c - Graphic MMU LUT entry 717 high"]
    pub lut717h: crate::Reg<lut717h::LUT717H_SPEC>,
    #[doc = "0x2670 - Graphic MMU LUT entry 718 low"]
    pub lut718l: crate::Reg<lut718l::LUT718L_SPEC>,
    #[doc = "0x2674 - Graphic MMU LUT entry 718 high"]
    pub lut718h: crate::Reg<lut718h::LUT718H_SPEC>,
    #[doc = "0x2678 - Graphic MMU LUT entry 719 low"]
    pub lut719l: crate::Reg<lut719l::LUT719L_SPEC>,
    #[doc = "0x267c - Graphic MMU LUT entry 719 high"]
    pub lut719h: crate::Reg<lut719h::LUT719H_SPEC>,
    #[doc = "0x2680 - Graphic MMU LUT entry 720 low"]
    pub lut720l: crate::Reg<lut720l::LUT720L_SPEC>,
    #[doc = "0x2684 - Graphic MMU LUT entry 720 high"]
    pub lut720h: crate::Reg<lut720h::LUT720H_SPEC>,
    #[doc = "0x2688 - Graphic MMU LUT entry 721 low"]
    pub lut721l: crate::Reg<lut721l::LUT721L_SPEC>,
    #[doc = "0x268c - Graphic MMU LUT entry 721 high"]
    pub lut721h: crate::Reg<lut721h::LUT721H_SPEC>,
    #[doc = "0x2690 - Graphic MMU LUT entry 722 low"]
    pub lut722l: crate::Reg<lut722l::LUT722L_SPEC>,
    #[doc = "0x2694 - Graphic MMU LUT entry 722 high"]
    pub lut722h: crate::Reg<lut722h::LUT722H_SPEC>,
    #[doc = "0x2698 - Graphic MMU LUT entry 723 low"]
    pub lut723l: crate::Reg<lut723l::LUT723L_SPEC>,
    #[doc = "0x269c - Graphic MMU LUT entry 723 high"]
    pub lut723h: crate::Reg<lut723h::LUT723H_SPEC>,
    #[doc = "0x26a0 - Graphic MMU LUT entry 724 low"]
    pub lut724l: crate::Reg<lut724l::LUT724L_SPEC>,
    #[doc = "0x26a4 - Graphic MMU LUT entry 724 high"]
    pub lut724h: crate::Reg<lut724h::LUT724H_SPEC>,
    #[doc = "0x26a8 - Graphic MMU LUT entry 725 low"]
    pub lut725l: crate::Reg<lut725l::LUT725L_SPEC>,
    #[doc = "0x26ac - Graphic MMU LUT entry 725 high"]
    pub lut725h: crate::Reg<lut725h::LUT725H_SPEC>,
    #[doc = "0x26b0 - Graphic MMU LUT entry 726 low"]
    pub lut726l: crate::Reg<lut726l::LUT726L_SPEC>,
    #[doc = "0x26b4 - Graphic MMU LUT entry 726 high"]
    pub lut726h: crate::Reg<lut726h::LUT726H_SPEC>,
    #[doc = "0x26b8 - Graphic MMU LUT entry 727 low"]
    pub lut727l: crate::Reg<lut727l::LUT727L_SPEC>,
    #[doc = "0x26bc - Graphic MMU LUT entry 727 high"]
    pub lut727h: crate::Reg<lut727h::LUT727H_SPEC>,
    #[doc = "0x26c0 - Graphic MMU LUT entry 728 low"]
    pub lut728l: crate::Reg<lut728l::LUT728L_SPEC>,
    #[doc = "0x26c4 - Graphic MMU LUT entry 728 high"]
    pub lut728h: crate::Reg<lut728h::LUT728H_SPEC>,
    #[doc = "0x26c8 - Graphic MMU LUT entry 729 low"]
    pub lut729l: crate::Reg<lut729l::LUT729L_SPEC>,
    #[doc = "0x26cc - Graphic MMU LUT entry 729 high"]
    pub lut729h: crate::Reg<lut729h::LUT729H_SPEC>,
    #[doc = "0x26d0 - Graphic MMU LUT entry 730 low"]
    pub lut730l: crate::Reg<lut730l::LUT730L_SPEC>,
    #[doc = "0x26d4 - Graphic MMU LUT entry 730 high"]
    pub lut730h: crate::Reg<lut730h::LUT730H_SPEC>,
    #[doc = "0x26d8 - Graphic MMU LUT entry 731 low"]
    pub lut731l: crate::Reg<lut731l::LUT731L_SPEC>,
    #[doc = "0x26dc - Graphic MMU LUT entry 731 high"]
    pub lut731h: crate::Reg<lut731h::LUT731H_SPEC>,
    #[doc = "0x26e0 - Graphic MMU LUT entry 732 low"]
    pub lut732l: crate::Reg<lut732l::LUT732L_SPEC>,
    #[doc = "0x26e4 - Graphic MMU LUT entry 732 high"]
    pub lut732h: crate::Reg<lut732h::LUT732H_SPEC>,
    #[doc = "0x26e8 - Graphic MMU LUT entry 733 low"]
    pub lut733l: crate::Reg<lut733l::LUT733L_SPEC>,
    #[doc = "0x26ec - Graphic MMU LUT entry 733 high"]
    pub lut733h: crate::Reg<lut733h::LUT733H_SPEC>,
    #[doc = "0x26f0 - Graphic MMU LUT entry 734 low"]
    pub lut734l: crate::Reg<lut734l::LUT734L_SPEC>,
    #[doc = "0x26f4 - Graphic MMU LUT entry 734 high"]
    pub lut734h: crate::Reg<lut734h::LUT734H_SPEC>,
    #[doc = "0x26f8 - Graphic MMU LUT entry 735 low"]
    pub lut735l: crate::Reg<lut735l::LUT735L_SPEC>,
    #[doc = "0x26fc - Graphic MMU LUT entry 735 high"]
    pub lut735h: crate::Reg<lut735h::LUT735H_SPEC>,
    #[doc = "0x2700 - Graphic MMU LUT entry 736 low"]
    pub lut736l: crate::Reg<lut736l::LUT736L_SPEC>,
    #[doc = "0x2704 - Graphic MMU LUT entry 736 high"]
    pub lut736h: crate::Reg<lut736h::LUT736H_SPEC>,
    #[doc = "0x2708 - Graphic MMU LUT entry 737 low"]
    pub lut737l: crate::Reg<lut737l::LUT737L_SPEC>,
    #[doc = "0x270c - Graphic MMU LUT entry 737 high"]
    pub lut737h: crate::Reg<lut737h::LUT737H_SPEC>,
    #[doc = "0x2710 - Graphic MMU LUT entry 738 low"]
    pub lut738l: crate::Reg<lut738l::LUT738L_SPEC>,
    #[doc = "0x2714 - Graphic MMU LUT entry 738 high"]
    pub lut738h: crate::Reg<lut738h::LUT738H_SPEC>,
    #[doc = "0x2718 - Graphic MMU LUT entry 739 low"]
    pub lut739l: crate::Reg<lut739l::LUT739L_SPEC>,
    #[doc = "0x271c - Graphic MMU LUT entry 739 high"]
    pub lut739h: crate::Reg<lut739h::LUT739H_SPEC>,
    #[doc = "0x2720 - Graphic MMU LUT entry 740 low"]
    pub lut740l: crate::Reg<lut740l::LUT740L_SPEC>,
    #[doc = "0x2724 - Graphic MMU LUT entry 740 high"]
    pub lut740h: crate::Reg<lut740h::LUT740H_SPEC>,
    #[doc = "0x2728 - Graphic MMU LUT entry 741 low"]
    pub lut741l: crate::Reg<lut741l::LUT741L_SPEC>,
    #[doc = "0x272c - Graphic MMU LUT entry 741 high"]
    pub lut741h: crate::Reg<lut741h::LUT741H_SPEC>,
    #[doc = "0x2730 - Graphic MMU LUT entry 742 low"]
    pub lut742l: crate::Reg<lut742l::LUT742L_SPEC>,
    #[doc = "0x2734 - Graphic MMU LUT entry 742 high"]
    pub lut742h: crate::Reg<lut742h::LUT742H_SPEC>,
    #[doc = "0x2738 - Graphic MMU LUT entry 743 low"]
    pub lut743l: crate::Reg<lut743l::LUT743L_SPEC>,
    #[doc = "0x273c - Graphic MMU LUT entry 743 high"]
    pub lut743h: crate::Reg<lut743h::LUT743H_SPEC>,
    #[doc = "0x2740 - Graphic MMU LUT entry 744 low"]
    pub lut744l: crate::Reg<lut744l::LUT744L_SPEC>,
    #[doc = "0x2744 - Graphic MMU LUT entry 744 high"]
    pub lut744h: crate::Reg<lut744h::LUT744H_SPEC>,
    #[doc = "0x2748 - Graphic MMU LUT entry 745 low"]
    pub lut745l: crate::Reg<lut745l::LUT745L_SPEC>,
    #[doc = "0x274c - Graphic MMU LUT entry 745 high"]
    pub lut745h: crate::Reg<lut745h::LUT745H_SPEC>,
    #[doc = "0x2750 - Graphic MMU LUT entry 746 low"]
    pub lut746l: crate::Reg<lut746l::LUT746L_SPEC>,
    #[doc = "0x2754 - Graphic MMU LUT entry 746 high"]
    pub lut746h: crate::Reg<lut746h::LUT746H_SPEC>,
    #[doc = "0x2758 - Graphic MMU LUT entry 747 low"]
    pub lut747l: crate::Reg<lut747l::LUT747L_SPEC>,
    #[doc = "0x275c - Graphic MMU LUT entry 747 high"]
    pub lut747h: crate::Reg<lut747h::LUT747H_SPEC>,
    #[doc = "0x2760 - Graphic MMU LUT entry 748 low"]
    pub lut748l: crate::Reg<lut748l::LUT748L_SPEC>,
    #[doc = "0x2764 - Graphic MMU LUT entry 748 high"]
    pub lut748h: crate::Reg<lut748h::LUT748H_SPEC>,
    #[doc = "0x2768 - Graphic MMU LUT entry 749 low"]
    pub lut749l: crate::Reg<lut749l::LUT749L_SPEC>,
    #[doc = "0x276c - Graphic MMU LUT entry 749 high"]
    pub lut749h: crate::Reg<lut749h::LUT749H_SPEC>,
    #[doc = "0x2770 - Graphic MMU LUT entry 750 low"]
    pub lut750l: crate::Reg<lut750l::LUT750L_SPEC>,
    #[doc = "0x2774 - Graphic MMU LUT entry 750 high"]
    pub lut750h: crate::Reg<lut750h::LUT750H_SPEC>,
    #[doc = "0x2778 - Graphic MMU LUT entry 751 low"]
    pub lut751l: crate::Reg<lut751l::LUT751L_SPEC>,
    #[doc = "0x277c - Graphic MMU LUT entry 751 high"]
    pub lut751h: crate::Reg<lut751h::LUT751H_SPEC>,
    #[doc = "0x2780 - Graphic MMU LUT entry 752 low"]
    pub lut752l: crate::Reg<lut752l::LUT752L_SPEC>,
    #[doc = "0x2784 - Graphic MMU LUT entry 752 high"]
    pub lut752h: crate::Reg<lut752h::LUT752H_SPEC>,
    #[doc = "0x2788 - Graphic MMU LUT entry 753 low"]
    pub lut753l: crate::Reg<lut753l::LUT753L_SPEC>,
    #[doc = "0x278c - Graphic MMU LUT entry 753 high"]
    pub lut753h: crate::Reg<lut753h::LUT753H_SPEC>,
    #[doc = "0x2790 - Graphic MMU LUT entry 754 low"]
    pub lut754l: crate::Reg<lut754l::LUT754L_SPEC>,
    #[doc = "0x2794 - Graphic MMU LUT entry 754 high"]
    pub lut754h: crate::Reg<lut754h::LUT754H_SPEC>,
    #[doc = "0x2798 - Graphic MMU LUT entry 755 low"]
    pub lut755l: crate::Reg<lut755l::LUT755L_SPEC>,
    #[doc = "0x279c - Graphic MMU LUT entry 755 high"]
    pub lut755h: crate::Reg<lut755h::LUT755H_SPEC>,
    #[doc = "0x27a0 - Graphic MMU LUT entry 756 low"]
    pub lut756l: crate::Reg<lut756l::LUT756L_SPEC>,
    #[doc = "0x27a4 - Graphic MMU LUT entry 756 high"]
    pub lut756h: crate::Reg<lut756h::LUT756H_SPEC>,
    #[doc = "0x27a8 - Graphic MMU LUT entry 757 low"]
    pub lut757l: crate::Reg<lut757l::LUT757L_SPEC>,
    #[doc = "0x27ac - Graphic MMU LUT entry 757 high"]
    pub lut757h: crate::Reg<lut757h::LUT757H_SPEC>,
    #[doc = "0x27b0 - Graphic MMU LUT entry 758 low"]
    pub lut758l: crate::Reg<lut758l::LUT758L_SPEC>,
    #[doc = "0x27b4 - Graphic MMU LUT entry 758 high"]
    pub lut758h: crate::Reg<lut758h::LUT758H_SPEC>,
    #[doc = "0x27b8 - Graphic MMU LUT entry 759 low"]
    pub lut759l: crate::Reg<lut759l::LUT759L_SPEC>,
    #[doc = "0x27bc - Graphic MMU LUT entry 759 high"]
    pub lut759h: crate::Reg<lut759h::LUT759H_SPEC>,
    #[doc = "0x27c0 - Graphic MMU LUT entry 760 low"]
    pub lut760l: crate::Reg<lut760l::LUT760L_SPEC>,
    #[doc = "0x27c4 - Graphic MMU LUT entry 760 high"]
    pub lut760h: crate::Reg<lut760h::LUT760H_SPEC>,
    #[doc = "0x27c8 - Graphic MMU LUT entry 761 low"]
    pub lut761l: crate::Reg<lut761l::LUT761L_SPEC>,
    #[doc = "0x27cc - Graphic MMU LUT entry 761 high"]
    pub lut761h: crate::Reg<lut761h::LUT761H_SPEC>,
    #[doc = "0x27d0 - Graphic MMU LUT entry 762 low"]
    pub lut762l: crate::Reg<lut762l::LUT762L_SPEC>,
    #[doc = "0x27d4 - Graphic MMU LUT entry 762 high"]
    pub lut762h: crate::Reg<lut762h::LUT762H_SPEC>,
    #[doc = "0x27d8 - Graphic MMU LUT entry 763 low"]
    pub lut763l: crate::Reg<lut763l::LUT763L_SPEC>,
    #[doc = "0x27dc - Graphic MMU LUT entry 763 high"]
    pub lut763h: crate::Reg<lut763h::LUT763H_SPEC>,
    #[doc = "0x27e0 - Graphic MMU LUT entry 764 low"]
    pub lut764l: crate::Reg<lut764l::LUT764L_SPEC>,
    #[doc = "0x27e4 - Graphic MMU LUT entry 764 high"]
    pub lut764h: crate::Reg<lut764h::LUT764H_SPEC>,
    #[doc = "0x27e8 - Graphic MMU LUT entry 765 low"]
    pub lut765l: crate::Reg<lut765l::LUT765L_SPEC>,
    #[doc = "0x27ec - Graphic MMU LUT entry 765 high"]
    pub lut765h: crate::Reg<lut765h::LUT765H_SPEC>,
    #[doc = "0x27f0 - Graphic MMU LUT entry 766 low"]
    pub lut766l: crate::Reg<lut766l::LUT766L_SPEC>,
    #[doc = "0x27f4 - Graphic MMU LUT entry 766 high"]
    pub lut766h: crate::Reg<lut766h::LUT766H_SPEC>,
    #[doc = "0x27f8 - Graphic MMU LUT entry 767 low"]
    pub lut767l: crate::Reg<lut767l::LUT767L_SPEC>,
    #[doc = "0x27fc - Graphic MMU LUT entry 767 high"]
    pub lut767h: crate::Reg<lut767h::LUT767H_SPEC>,
    #[doc = "0x2800 - Graphic MMU LUT entry 768 low"]
    pub lut768l: crate::Reg<lut768l::LUT768L_SPEC>,
    #[doc = "0x2804 - Graphic MMU LUT entry 768 high"]
    pub lut768h: crate::Reg<lut768h::LUT768H_SPEC>,
    #[doc = "0x2808 - Graphic MMU LUT entry 769 low"]
    pub lut769l: crate::Reg<lut769l::LUT769L_SPEC>,
    #[doc = "0x280c - Graphic MMU LUT entry 769 high"]
    pub lut769h: crate::Reg<lut769h::LUT769H_SPEC>,
    #[doc = "0x2810 - Graphic MMU LUT entry 770 low"]
    pub lut770l: crate::Reg<lut770l::LUT770L_SPEC>,
    #[doc = "0x2814 - Graphic MMU LUT entry 770 high"]
    pub lut770h: crate::Reg<lut770h::LUT770H_SPEC>,
    #[doc = "0x2818 - Graphic MMU LUT entry 771 low"]
    pub lut771l: crate::Reg<lut771l::LUT771L_SPEC>,
    #[doc = "0x281c - Graphic MMU LUT entry 771 high"]
    pub lut771h: crate::Reg<lut771h::LUT771H_SPEC>,
    #[doc = "0x2820 - Graphic MMU LUT entry 772 low"]
    pub lut772l: crate::Reg<lut772l::LUT772L_SPEC>,
    #[doc = "0x2824 - Graphic MMU LUT entry 772 high"]
    pub lut772h: crate::Reg<lut772h::LUT772H_SPEC>,
    #[doc = "0x2828 - Graphic MMU LUT entry 773 low"]
    pub lut773l: crate::Reg<lut773l::LUT773L_SPEC>,
    #[doc = "0x282c - Graphic MMU LUT entry 773 high"]
    pub lut773h: crate::Reg<lut773h::LUT773H_SPEC>,
    #[doc = "0x2830 - Graphic MMU LUT entry 774 low"]
    pub lut774l: crate::Reg<lut774l::LUT774L_SPEC>,
    #[doc = "0x2834 - Graphic MMU LUT entry 774 high"]
    pub lut774h: crate::Reg<lut774h::LUT774H_SPEC>,
    #[doc = "0x2838 - Graphic MMU LUT entry 775 low"]
    pub lut775l: crate::Reg<lut775l::LUT775L_SPEC>,
    #[doc = "0x283c - Graphic MMU LUT entry 775 high"]
    pub lut775h: crate::Reg<lut775h::LUT775H_SPEC>,
    #[doc = "0x2840 - Graphic MMU LUT entry 776 low"]
    pub lut776l: crate::Reg<lut776l::LUT776L_SPEC>,
    #[doc = "0x2844 - Graphic MMU LUT entry 776 high"]
    pub lut776h: crate::Reg<lut776h::LUT776H_SPEC>,
    #[doc = "0x2848 - Graphic MMU LUT entry 777 low"]
    pub lut777l: crate::Reg<lut777l::LUT777L_SPEC>,
    #[doc = "0x284c - Graphic MMU LUT entry 777 high"]
    pub lut777h: crate::Reg<lut777h::LUT777H_SPEC>,
    #[doc = "0x2850 - Graphic MMU LUT entry 778 low"]
    pub lut778l: crate::Reg<lut778l::LUT778L_SPEC>,
    #[doc = "0x2854 - Graphic MMU LUT entry 778 high"]
    pub lut778h: crate::Reg<lut778h::LUT778H_SPEC>,
    #[doc = "0x2858 - Graphic MMU LUT entry 779 low"]
    pub lut779l: crate::Reg<lut779l::LUT779L_SPEC>,
    #[doc = "0x285c - Graphic MMU LUT entry 779 high"]
    pub lut779h: crate::Reg<lut779h::LUT779H_SPEC>,
    #[doc = "0x2860 - Graphic MMU LUT entry 780 low"]
    pub lut780l: crate::Reg<lut780l::LUT780L_SPEC>,
    #[doc = "0x2864 - Graphic MMU LUT entry 780 high"]
    pub lut780h: crate::Reg<lut780h::LUT780H_SPEC>,
    #[doc = "0x2868 - Graphic MMU LUT entry 781 low"]
    pub lut781l: crate::Reg<lut781l::LUT781L_SPEC>,
    #[doc = "0x286c - Graphic MMU LUT entry 781 high"]
    pub lut781h: crate::Reg<lut781h::LUT781H_SPEC>,
    #[doc = "0x2870 - Graphic MMU LUT entry 782 low"]
    pub lut782l: crate::Reg<lut782l::LUT782L_SPEC>,
    #[doc = "0x2874 - Graphic MMU LUT entry 782 high"]
    pub lut782h: crate::Reg<lut782h::LUT782H_SPEC>,
    #[doc = "0x2878 - Graphic MMU LUT entry 783 low"]
    pub lut783l: crate::Reg<lut783l::LUT783L_SPEC>,
    #[doc = "0x287c - Graphic MMU LUT entry 783 high"]
    pub lut783h: crate::Reg<lut783h::LUT783H_SPEC>,
    #[doc = "0x2880 - Graphic MMU LUT entry 784 low"]
    pub lut784l: crate::Reg<lut784l::LUT784L_SPEC>,
    #[doc = "0x2884 - Graphic MMU LUT entry 784 high"]
    pub lut784h: crate::Reg<lut784h::LUT784H_SPEC>,
    #[doc = "0x2888 - Graphic MMU LUT entry 785 low"]
    pub lut785l: crate::Reg<lut785l::LUT785L_SPEC>,
    #[doc = "0x288c - Graphic MMU LUT entry 785 high"]
    pub lut785h: crate::Reg<lut785h::LUT785H_SPEC>,
    #[doc = "0x2890 - Graphic MMU LUT entry 786 low"]
    pub lut786l: crate::Reg<lut786l::LUT786L_SPEC>,
    #[doc = "0x2894 - Graphic MMU LUT entry 786 high"]
    pub lut786h: crate::Reg<lut786h::LUT786H_SPEC>,
    #[doc = "0x2898 - Graphic MMU LUT entry 787 low"]
    pub lut787l: crate::Reg<lut787l::LUT787L_SPEC>,
    #[doc = "0x289c - Graphic MMU LUT entry 787 high"]
    pub lut787h: crate::Reg<lut787h::LUT787H_SPEC>,
    #[doc = "0x28a0 - Graphic MMU LUT entry 788 low"]
    pub lut788l: crate::Reg<lut788l::LUT788L_SPEC>,
    #[doc = "0x28a4 - Graphic MMU LUT entry 788 high"]
    pub lut788h: crate::Reg<lut788h::LUT788H_SPEC>,
    #[doc = "0x28a8 - Graphic MMU LUT entry 789 low"]
    pub lut789l: crate::Reg<lut789l::LUT789L_SPEC>,
    #[doc = "0x28ac - Graphic MMU LUT entry 789 high"]
    pub lut789h: crate::Reg<lut789h::LUT789H_SPEC>,
    #[doc = "0x28b0 - Graphic MMU LUT entry 790 low"]
    pub lut790l: crate::Reg<lut790l::LUT790L_SPEC>,
    #[doc = "0x28b4 - Graphic MMU LUT entry 790 high"]
    pub lut790h: crate::Reg<lut790h::LUT790H_SPEC>,
    #[doc = "0x28b8 - Graphic MMU LUT entry 791 low"]
    pub lut791l: crate::Reg<lut791l::LUT791L_SPEC>,
    #[doc = "0x28bc - Graphic MMU LUT entry 791 high"]
    pub lut791h: crate::Reg<lut791h::LUT791H_SPEC>,
    #[doc = "0x28c0 - Graphic MMU LUT entry 792 low"]
    pub lut792l: crate::Reg<lut792l::LUT792L_SPEC>,
    #[doc = "0x28c4 - Graphic MMU LUT entry 792 high"]
    pub lut792h: crate::Reg<lut792h::LUT792H_SPEC>,
    #[doc = "0x28c8 - Graphic MMU LUT entry 793 low"]
    pub lut793l: crate::Reg<lut793l::LUT793L_SPEC>,
    #[doc = "0x28cc - Graphic MMU LUT entry 793 high"]
    pub lut793h: crate::Reg<lut793h::LUT793H_SPEC>,
    #[doc = "0x28d0 - Graphic MMU LUT entry 794 low"]
    pub lut794l: crate::Reg<lut794l::LUT794L_SPEC>,
    #[doc = "0x28d4 - Graphic MMU LUT entry 794 high"]
    pub lut794h: crate::Reg<lut794h::LUT794H_SPEC>,
    #[doc = "0x28d8 - Graphic MMU LUT entry 795 low"]
    pub lut795l: crate::Reg<lut795l::LUT795L_SPEC>,
    #[doc = "0x28dc - Graphic MMU LUT entry 795 high"]
    pub lut795h: crate::Reg<lut795h::LUT795H_SPEC>,
    #[doc = "0x28e0 - Graphic MMU LUT entry 796 low"]
    pub lut796l: crate::Reg<lut796l::LUT796L_SPEC>,
    #[doc = "0x28e4 - Graphic MMU LUT entry 796 high"]
    pub lut796h: crate::Reg<lut796h::LUT796H_SPEC>,
    #[doc = "0x28e8 - Graphic MMU LUT entry 797 low"]
    pub lut797l: crate::Reg<lut797l::LUT797L_SPEC>,
    #[doc = "0x28ec - Graphic MMU LUT entry 797 high"]
    pub lut797h: crate::Reg<lut797h::LUT797H_SPEC>,
    #[doc = "0x28f0 - Graphic MMU LUT entry 798 low"]
    pub lut798l: crate::Reg<lut798l::LUT798L_SPEC>,
    #[doc = "0x28f4 - Graphic MMU LUT entry 798 high"]
    pub lut798h: crate::Reg<lut798h::LUT798H_SPEC>,
    #[doc = "0x28f8 - Graphic MMU LUT entry 799 low"]
    pub lut799l: crate::Reg<lut799l::LUT799L_SPEC>,
    #[doc = "0x28fc - Graphic MMU LUT entry 799 high"]
    pub lut799h: crate::Reg<lut799h::LUT799H_SPEC>,
    #[doc = "0x2900 - Graphic MMU LUT entry 800 low"]
    pub lut800l: crate::Reg<lut800l::LUT800L_SPEC>,
    #[doc = "0x2904 - Graphic MMU LUT entry 800 high"]
    pub lut800h: crate::Reg<lut800h::LUT800H_SPEC>,
    #[doc = "0x2908 - Graphic MMU LUT entry 801 low"]
    pub lut801l: crate::Reg<lut801l::LUT801L_SPEC>,
    #[doc = "0x290c - Graphic MMU LUT entry 801 high"]
    pub lut801h: crate::Reg<lut801h::LUT801H_SPEC>,
    #[doc = "0x2910 - Graphic MMU LUT entry 802 low"]
    pub lut802l: crate::Reg<lut802l::LUT802L_SPEC>,
    #[doc = "0x2914 - Graphic MMU LUT entry 802 high"]
    pub lut802h: crate::Reg<lut802h::LUT802H_SPEC>,
    #[doc = "0x2918 - Graphic MMU LUT entry 803 low"]
    pub lut803l: crate::Reg<lut803l::LUT803L_SPEC>,
    #[doc = "0x291c - Graphic MMU LUT entry 803 high"]
    pub lut803h: crate::Reg<lut803h::LUT803H_SPEC>,
    #[doc = "0x2920 - Graphic MMU LUT entry 804 low"]
    pub lut804l: crate::Reg<lut804l::LUT804L_SPEC>,
    #[doc = "0x2924 - Graphic MMU LUT entry 804 high"]
    pub lut804h: crate::Reg<lut804h::LUT804H_SPEC>,
    #[doc = "0x2928 - Graphic MMU LUT entry 805 low"]
    pub lut805l: crate::Reg<lut805l::LUT805L_SPEC>,
    #[doc = "0x292c - Graphic MMU LUT entry 805 high"]
    pub lut805h: crate::Reg<lut805h::LUT805H_SPEC>,
    #[doc = "0x2930 - Graphic MMU LUT entry 806 low"]
    pub lut806l: crate::Reg<lut806l::LUT806L_SPEC>,
    #[doc = "0x2934 - Graphic MMU LUT entry 806 high"]
    pub lut806h: crate::Reg<lut806h::LUT806H_SPEC>,
    #[doc = "0x2938 - Graphic MMU LUT entry 807 low"]
    pub lut807l: crate::Reg<lut807l::LUT807L_SPEC>,
    #[doc = "0x293c - Graphic MMU LUT entry 807 high"]
    pub lut807h: crate::Reg<lut807h::LUT807H_SPEC>,
    #[doc = "0x2940 - Graphic MMU LUT entry 808 low"]
    pub lut808l: crate::Reg<lut808l::LUT808L_SPEC>,
    #[doc = "0x2944 - Graphic MMU LUT entry 808 high"]
    pub lut808h: crate::Reg<lut808h::LUT808H_SPEC>,
    #[doc = "0x2948 - Graphic MMU LUT entry 809 low"]
    pub lut809l: crate::Reg<lut809l::LUT809L_SPEC>,
    #[doc = "0x294c - Graphic MMU LUT entry 809 high"]
    pub lut809h: crate::Reg<lut809h::LUT809H_SPEC>,
    #[doc = "0x2950 - Graphic MMU LUT entry 810 low"]
    pub lut810l: crate::Reg<lut810l::LUT810L_SPEC>,
    #[doc = "0x2954 - Graphic MMU LUT entry 810 high"]
    pub lut810h: crate::Reg<lut810h::LUT810H_SPEC>,
    #[doc = "0x2958 - Graphic MMU LUT entry 811 low"]
    pub lut811l: crate::Reg<lut811l::LUT811L_SPEC>,
    #[doc = "0x295c - Graphic MMU LUT entry 811 high"]
    pub lut811h: crate::Reg<lut811h::LUT811H_SPEC>,
    #[doc = "0x2960 - Graphic MMU LUT entry 812 low"]
    pub lut812l: crate::Reg<lut812l::LUT812L_SPEC>,
    #[doc = "0x2964 - Graphic MMU LUT entry 812 high"]
    pub lut812h: crate::Reg<lut812h::LUT812H_SPEC>,
    #[doc = "0x2968 - Graphic MMU LUT entry 813 low"]
    pub lut813l: crate::Reg<lut813l::LUT813L_SPEC>,
    #[doc = "0x296c - Graphic MMU LUT entry 813 high"]
    pub lut813h: crate::Reg<lut813h::LUT813H_SPEC>,
    #[doc = "0x2970 - Graphic MMU LUT entry 814 low"]
    pub lut814l: crate::Reg<lut814l::LUT814L_SPEC>,
    #[doc = "0x2974 - Graphic MMU LUT entry 814 high"]
    pub lut814h: crate::Reg<lut814h::LUT814H_SPEC>,
    #[doc = "0x2978 - Graphic MMU LUT entry 815 low"]
    pub lut815l: crate::Reg<lut815l::LUT815L_SPEC>,
    #[doc = "0x297c - Graphic MMU LUT entry 815 high"]
    pub lut815h: crate::Reg<lut815h::LUT815H_SPEC>,
    #[doc = "0x2980 - Graphic MMU LUT entry 816 low"]
    pub lut816l: crate::Reg<lut816l::LUT816L_SPEC>,
    #[doc = "0x2984 - Graphic MMU LUT entry 816 high"]
    pub lut816h: crate::Reg<lut816h::LUT816H_SPEC>,
    #[doc = "0x2988 - Graphic MMU LUT entry 817 low"]
    pub lut817l: crate::Reg<lut817l::LUT817L_SPEC>,
    #[doc = "0x298c - Graphic MMU LUT entry 817 high"]
    pub lut817h: crate::Reg<lut817h::LUT817H_SPEC>,
    #[doc = "0x2990 - Graphic MMU LUT entry 818 low"]
    pub lut818l: crate::Reg<lut818l::LUT818L_SPEC>,
    #[doc = "0x2994 - Graphic MMU LUT entry 818 high"]
    pub lut818h: crate::Reg<lut818h::LUT818H_SPEC>,
    #[doc = "0x2998 - Graphic MMU LUT entry 819 low"]
    pub lut819l: crate::Reg<lut819l::LUT819L_SPEC>,
    #[doc = "0x299c - Graphic MMU LUT entry 819 high"]
    pub lut819h: crate::Reg<lut819h::LUT819H_SPEC>,
    #[doc = "0x29a0 - Graphic MMU LUT entry 820 low"]
    pub lut820l: crate::Reg<lut820l::LUT820L_SPEC>,
    #[doc = "0x29a4 - Graphic MMU LUT entry 820 high"]
    pub lut820h: crate::Reg<lut820h::LUT820H_SPEC>,
    #[doc = "0x29a8 - Graphic MMU LUT entry 821 low"]
    pub lut821l: crate::Reg<lut821l::LUT821L_SPEC>,
    #[doc = "0x29ac - Graphic MMU LUT entry 821 high"]
    pub lut821h: crate::Reg<lut821h::LUT821H_SPEC>,
    #[doc = "0x29b0 - Graphic MMU LUT entry 822 low"]
    pub lut822l: crate::Reg<lut822l::LUT822L_SPEC>,
    #[doc = "0x29b4 - Graphic MMU LUT entry 822 high"]
    pub lut822h: crate::Reg<lut822h::LUT822H_SPEC>,
    #[doc = "0x29b8 - Graphic MMU LUT entry 823 low"]
    pub lut823l: crate::Reg<lut823l::LUT823L_SPEC>,
    #[doc = "0x29bc - Graphic MMU LUT entry 823 high"]
    pub lut823h: crate::Reg<lut823h::LUT823H_SPEC>,
    #[doc = "0x29c0 - Graphic MMU LUT entry 824 low"]
    pub lut824l: crate::Reg<lut824l::LUT824L_SPEC>,
    #[doc = "0x29c4 - Graphic MMU LUT entry 824 high"]
    pub lut824h: crate::Reg<lut824h::LUT824H_SPEC>,
    #[doc = "0x29c8 - Graphic MMU LUT entry 825 low"]
    pub lut825l: crate::Reg<lut825l::LUT825L_SPEC>,
    #[doc = "0x29cc - Graphic MMU LUT entry 825 high"]
    pub lut825h: crate::Reg<lut825h::LUT825H_SPEC>,
    #[doc = "0x29d0 - Graphic MMU LUT entry 826 low"]
    pub lut826l: crate::Reg<lut826l::LUT826L_SPEC>,
    #[doc = "0x29d4 - Graphic MMU LUT entry 826 high"]
    pub lut826h: crate::Reg<lut826h::LUT826H_SPEC>,
    #[doc = "0x29d8 - Graphic MMU LUT entry 827 low"]
    pub lut827l: crate::Reg<lut827l::LUT827L_SPEC>,
    #[doc = "0x29dc - Graphic MMU LUT entry 827 high"]
    pub lut827h: crate::Reg<lut827h::LUT827H_SPEC>,
    #[doc = "0x29e0 - Graphic MMU LUT entry 828 low"]
    pub lut828l: crate::Reg<lut828l::LUT828L_SPEC>,
    #[doc = "0x29e4 - Graphic MMU LUT entry 828 high"]
    pub lut828h: crate::Reg<lut828h::LUT828H_SPEC>,
    #[doc = "0x29e8 - Graphic MMU LUT entry 829 low"]
    pub lut829l: crate::Reg<lut829l::LUT829L_SPEC>,
    #[doc = "0x29ec - Graphic MMU LUT entry 829 high"]
    pub lut829h: crate::Reg<lut829h::LUT829H_SPEC>,
    #[doc = "0x29f0 - Graphic MMU LUT entry 830 low"]
    pub lut830l: crate::Reg<lut830l::LUT830L_SPEC>,
    #[doc = "0x29f4 - Graphic MMU LUT entry 830 high"]
    pub lut830h: crate::Reg<lut830h::LUT830H_SPEC>,
    #[doc = "0x29f8 - Graphic MMU LUT entry 831 low"]
    pub lut831l: crate::Reg<lut831l::LUT831L_SPEC>,
    #[doc = "0x29fc - Graphic MMU LUT entry 831 high"]
    pub lut831h: crate::Reg<lut831h::LUT831H_SPEC>,
    #[doc = "0x2a00 - Graphic MMU LUT entry 832 low"]
    pub lut832l: crate::Reg<lut832l::LUT832L_SPEC>,
    #[doc = "0x2a04 - Graphic MMU LUT entry 832 high"]
    pub lut832h: crate::Reg<lut832h::LUT832H_SPEC>,
    #[doc = "0x2a08 - Graphic MMU LUT entry 833 low"]
    pub lut833l: crate::Reg<lut833l::LUT833L_SPEC>,
    #[doc = "0x2a0c - Graphic MMU LUT entry 833 high"]
    pub lut833h: crate::Reg<lut833h::LUT833H_SPEC>,
    #[doc = "0x2a10 - Graphic MMU LUT entry 834 low"]
    pub lut834l: crate::Reg<lut834l::LUT834L_SPEC>,
    #[doc = "0x2a14 - Graphic MMU LUT entry 834 high"]
    pub lut834h: crate::Reg<lut834h::LUT834H_SPEC>,
    #[doc = "0x2a18 - Graphic MMU LUT entry 835 low"]
    pub lut835l: crate::Reg<lut835l::LUT835L_SPEC>,
    #[doc = "0x2a1c - Graphic MMU LUT entry 835 high"]
    pub lut835h: crate::Reg<lut835h::LUT835H_SPEC>,
    #[doc = "0x2a20 - Graphic MMU LUT entry 836 low"]
    pub lut836l: crate::Reg<lut836l::LUT836L_SPEC>,
    #[doc = "0x2a24 - Graphic MMU LUT entry 836 high"]
    pub lut836h: crate::Reg<lut836h::LUT836H_SPEC>,
    #[doc = "0x2a28 - Graphic MMU LUT entry 837 low"]
    pub lut837l: crate::Reg<lut837l::LUT837L_SPEC>,
    #[doc = "0x2a2c - Graphic MMU LUT entry 837 high"]
    pub lut837h: crate::Reg<lut837h::LUT837H_SPEC>,
    #[doc = "0x2a30 - Graphic MMU LUT entry 838 low"]
    pub lut838l: crate::Reg<lut838l::LUT838L_SPEC>,
    #[doc = "0x2a34 - Graphic MMU LUT entry 838 high"]
    pub lut838h: crate::Reg<lut838h::LUT838H_SPEC>,
    #[doc = "0x2a38 - Graphic MMU LUT entry 839 low"]
    pub lut839l: crate::Reg<lut839l::LUT839L_SPEC>,
    #[doc = "0x2a3c - Graphic MMU LUT entry 839 high"]
    pub lut839h: crate::Reg<lut839h::LUT839H_SPEC>,
    #[doc = "0x2a40 - Graphic MMU LUT entry 840 low"]
    pub lut840l: crate::Reg<lut840l::LUT840L_SPEC>,
    #[doc = "0x2a44 - Graphic MMU LUT entry 840 high"]
    pub lut840h: crate::Reg<lut840h::LUT840H_SPEC>,
    #[doc = "0x2a48 - Graphic MMU LUT entry 841 low"]
    pub lut841l: crate::Reg<lut841l::LUT841L_SPEC>,
    #[doc = "0x2a4c - Graphic MMU LUT entry 841 high"]
    pub lut841h: crate::Reg<lut841h::LUT841H_SPEC>,
    #[doc = "0x2a50 - Graphic MMU LUT entry 842 low"]
    pub lut842l: crate::Reg<lut842l::LUT842L_SPEC>,
    #[doc = "0x2a54 - Graphic MMU LUT entry 842 high"]
    pub lut842h: crate::Reg<lut842h::LUT842H_SPEC>,
    #[doc = "0x2a58 - Graphic MMU LUT entry 843 low"]
    pub lut843l: crate::Reg<lut843l::LUT843L_SPEC>,
    #[doc = "0x2a5c - Graphic MMU LUT entry 843 high"]
    pub lut843h: crate::Reg<lut843h::LUT843H_SPEC>,
    #[doc = "0x2a60 - Graphic MMU LUT entry 844 low"]
    pub lut844l: crate::Reg<lut844l::LUT844L_SPEC>,
    #[doc = "0x2a64 - Graphic MMU LUT entry 844 high"]
    pub lut844h: crate::Reg<lut844h::LUT844H_SPEC>,
    #[doc = "0x2a68 - Graphic MMU LUT entry 845 low"]
    pub lut845l: crate::Reg<lut845l::LUT845L_SPEC>,
    #[doc = "0x2a6c - Graphic MMU LUT entry 845 high"]
    pub lut845h: crate::Reg<lut845h::LUT845H_SPEC>,
    #[doc = "0x2a70 - Graphic MMU LUT entry 846 low"]
    pub lut846l: crate::Reg<lut846l::LUT846L_SPEC>,
    #[doc = "0x2a74 - Graphic MMU LUT entry 846 high"]
    pub lut846h: crate::Reg<lut846h::LUT846H_SPEC>,
    #[doc = "0x2a78 - Graphic MMU LUT entry 847 low"]
    pub lut847l: crate::Reg<lut847l::LUT847L_SPEC>,
    #[doc = "0x2a7c - Graphic MMU LUT entry 847 high"]
    pub lut847h: crate::Reg<lut847h::LUT847H_SPEC>,
    #[doc = "0x2a80 - Graphic MMU LUT entry 848 low"]
    pub lut848l: crate::Reg<lut848l::LUT848L_SPEC>,
    #[doc = "0x2a84 - Graphic MMU LUT entry 848 high"]
    pub lut848h: crate::Reg<lut848h::LUT848H_SPEC>,
    #[doc = "0x2a88 - Graphic MMU LUT entry 849 low"]
    pub lut849l: crate::Reg<lut849l::LUT849L_SPEC>,
    #[doc = "0x2a8c - Graphic MMU LUT entry 849 high"]
    pub lut849h: crate::Reg<lut849h::LUT849H_SPEC>,
    #[doc = "0x2a90 - Graphic MMU LUT entry 850 low"]
    pub lut850l: crate::Reg<lut850l::LUT850L_SPEC>,
    #[doc = "0x2a94 - Graphic MMU LUT entry 850 high"]
    pub lut850h: crate::Reg<lut850h::LUT850H_SPEC>,
    #[doc = "0x2a98 - Graphic MMU LUT entry 851 low"]
    pub lut851l: crate::Reg<lut851l::LUT851L_SPEC>,
    #[doc = "0x2a9c - Graphic MMU LUT entry 851 high"]
    pub lut851h: crate::Reg<lut851h::LUT851H_SPEC>,
    #[doc = "0x2aa0 - Graphic MMU LUT entry 852 low"]
    pub lut852l: crate::Reg<lut852l::LUT852L_SPEC>,
    #[doc = "0x2aa4 - Graphic MMU LUT entry 852 high"]
    pub lut852h: crate::Reg<lut852h::LUT852H_SPEC>,
    #[doc = "0x2aa8 - Graphic MMU LUT entry 853 low"]
    pub lut853l: crate::Reg<lut853l::LUT853L_SPEC>,
    #[doc = "0x2aac - Graphic MMU LUT entry 853 high"]
    pub lut853h: crate::Reg<lut853h::LUT853H_SPEC>,
    #[doc = "0x2ab0 - Graphic MMU LUT entry 854 low"]
    pub lut854l: crate::Reg<lut854l::LUT854L_SPEC>,
    #[doc = "0x2ab4 - Graphic MMU LUT entry 854 high"]
    pub lut854h: crate::Reg<lut854h::LUT854H_SPEC>,
    #[doc = "0x2ab8 - Graphic MMU LUT entry 855 low"]
    pub lut855l: crate::Reg<lut855l::LUT855L_SPEC>,
    #[doc = "0x2abc - Graphic MMU LUT entry 855 high"]
    pub lut855h: crate::Reg<lut855h::LUT855H_SPEC>,
    #[doc = "0x2ac0 - Graphic MMU LUT entry 856 low"]
    pub lut856l: crate::Reg<lut856l::LUT856L_SPEC>,
    #[doc = "0x2ac4 - Graphic MMU LUT entry 856 high"]
    pub lut856h: crate::Reg<lut856h::LUT856H_SPEC>,
    #[doc = "0x2ac8 - Graphic MMU LUT entry 857 low"]
    pub lut857l: crate::Reg<lut857l::LUT857L_SPEC>,
    #[doc = "0x2acc - Graphic MMU LUT entry 857 high"]
    pub lut857h: crate::Reg<lut857h::LUT857H_SPEC>,
    #[doc = "0x2ad0 - Graphic MMU LUT entry 858 low"]
    pub lut858l: crate::Reg<lut858l::LUT858L_SPEC>,
    #[doc = "0x2ad4 - Graphic MMU LUT entry 858 high"]
    pub lut858h: crate::Reg<lut858h::LUT858H_SPEC>,
    #[doc = "0x2ad8 - Graphic MMU LUT entry 859 low"]
    pub lut859l: crate::Reg<lut859l::LUT859L_SPEC>,
    #[doc = "0x2adc - Graphic MMU LUT entry 859 high"]
    pub lut859h: crate::Reg<lut859h::LUT859H_SPEC>,
    #[doc = "0x2ae0 - Graphic MMU LUT entry 860 low"]
    pub lut860l: crate::Reg<lut860l::LUT860L_SPEC>,
    #[doc = "0x2ae4 - Graphic MMU LUT entry 860 high"]
    pub lut860h: crate::Reg<lut860h::LUT860H_SPEC>,
    #[doc = "0x2ae8 - Graphic MMU LUT entry 861 low"]
    pub lut861l: crate::Reg<lut861l::LUT861L_SPEC>,
    #[doc = "0x2aec - Graphic MMU LUT entry 861 high"]
    pub lut861h: crate::Reg<lut861h::LUT861H_SPEC>,
    #[doc = "0x2af0 - Graphic MMU LUT entry 862 low"]
    pub lut862l: crate::Reg<lut862l::LUT862L_SPEC>,
    #[doc = "0x2af4 - Graphic MMU LUT entry 862 high"]
    pub lut862h: crate::Reg<lut862h::LUT862H_SPEC>,
    #[doc = "0x2af8 - Graphic MMU LUT entry 863 low"]
    pub lut863l: crate::Reg<lut863l::LUT863L_SPEC>,
    #[doc = "0x2afc - Graphic MMU LUT entry 863 high"]
    pub lut863h: crate::Reg<lut863h::LUT863H_SPEC>,
    #[doc = "0x2b00 - Graphic MMU LUT entry 864 low"]
    pub lut864l: crate::Reg<lut864l::LUT864L_SPEC>,
    #[doc = "0x2b04 - Graphic MMU LUT entry 864 high"]
    pub lut864h: crate::Reg<lut864h::LUT864H_SPEC>,
    #[doc = "0x2b08 - Graphic MMU LUT entry 865 low"]
    pub lut865l: crate::Reg<lut865l::LUT865L_SPEC>,
    #[doc = "0x2b0c - Graphic MMU LUT entry 865 high"]
    pub lut865h: crate::Reg<lut865h::LUT865H_SPEC>,
    #[doc = "0x2b10 - Graphic MMU LUT entry 866 low"]
    pub lut866l: crate::Reg<lut866l::LUT866L_SPEC>,
    #[doc = "0x2b14 - Graphic MMU LUT entry 866 high"]
    pub lut866h: crate::Reg<lut866h::LUT866H_SPEC>,
    #[doc = "0x2b18 - Graphic MMU LUT entry 867 low"]
    pub lut867l: crate::Reg<lut867l::LUT867L_SPEC>,
    #[doc = "0x2b1c - Graphic MMU LUT entry 867 high"]
    pub lut867h: crate::Reg<lut867h::LUT867H_SPEC>,
    #[doc = "0x2b20 - Graphic MMU LUT entry 868 low"]
    pub lut868l: crate::Reg<lut868l::LUT868L_SPEC>,
    #[doc = "0x2b24 - Graphic MMU LUT entry 868 high"]
    pub lut868h: crate::Reg<lut868h::LUT868H_SPEC>,
    #[doc = "0x2b28 - Graphic MMU LUT entry 869 low"]
    pub lut869l: crate::Reg<lut869l::LUT869L_SPEC>,
    #[doc = "0x2b2c - Graphic MMU LUT entry 869 high"]
    pub lut869h: crate::Reg<lut869h::LUT869H_SPEC>,
    #[doc = "0x2b30 - Graphic MMU LUT entry 870 low"]
    pub lut870l: crate::Reg<lut870l::LUT870L_SPEC>,
    #[doc = "0x2b34 - Graphic MMU LUT entry 870 high"]
    pub lut870h: crate::Reg<lut870h::LUT870H_SPEC>,
    #[doc = "0x2b38 - Graphic MMU LUT entry 871 low"]
    pub lut871l: crate::Reg<lut871l::LUT871L_SPEC>,
    #[doc = "0x2b3c - Graphic MMU LUT entry 871 high"]
    pub lut871h: crate::Reg<lut871h::LUT871H_SPEC>,
    #[doc = "0x2b40 - Graphic MMU LUT entry 872 low"]
    pub lut872l: crate::Reg<lut872l::LUT872L_SPEC>,
    #[doc = "0x2b44 - Graphic MMU LUT entry 872 high"]
    pub lut872h: crate::Reg<lut872h::LUT872H_SPEC>,
    #[doc = "0x2b48 - Graphic MMU LUT entry 873 low"]
    pub lut873l: crate::Reg<lut873l::LUT873L_SPEC>,
    #[doc = "0x2b4c - Graphic MMU LUT entry 873 high"]
    pub lut873h: crate::Reg<lut873h::LUT873H_SPEC>,
    #[doc = "0x2b50 - Graphic MMU LUT entry 874 low"]
    pub lut874l: crate::Reg<lut874l::LUT874L_SPEC>,
    #[doc = "0x2b54 - Graphic MMU LUT entry 874 high"]
    pub lut874h: crate::Reg<lut874h::LUT874H_SPEC>,
    #[doc = "0x2b58 - Graphic MMU LUT entry 875 low"]
    pub lut875l: crate::Reg<lut875l::LUT875L_SPEC>,
    #[doc = "0x2b5c - Graphic MMU LUT entry 875 high"]
    pub lut875h: crate::Reg<lut875h::LUT875H_SPEC>,
    #[doc = "0x2b60 - Graphic MMU LUT entry 876 low"]
    pub lut876l: crate::Reg<lut876l::LUT876L_SPEC>,
    #[doc = "0x2b64 - Graphic MMU LUT entry 876 high"]
    pub lut876h: crate::Reg<lut876h::LUT876H_SPEC>,
    #[doc = "0x2b68 - Graphic MMU LUT entry 877 low"]
    pub lut877l: crate::Reg<lut877l::LUT877L_SPEC>,
    #[doc = "0x2b6c - Graphic MMU LUT entry 877 high"]
    pub lut877h: crate::Reg<lut877h::LUT877H_SPEC>,
    #[doc = "0x2b70 - Graphic MMU LUT entry 878 low"]
    pub lut878l: crate::Reg<lut878l::LUT878L_SPEC>,
    #[doc = "0x2b74 - Graphic MMU LUT entry 878 high"]
    pub lut878h: crate::Reg<lut878h::LUT878H_SPEC>,
    #[doc = "0x2b78 - Graphic MMU LUT entry 879 low"]
    pub lut879l: crate::Reg<lut879l::LUT879L_SPEC>,
    #[doc = "0x2b7c - Graphic MMU LUT entry 879 high"]
    pub lut879h: crate::Reg<lut879h::LUT879H_SPEC>,
    #[doc = "0x2b80 - Graphic MMU LUT entry 880 low"]
    pub lut880l: crate::Reg<lut880l::LUT880L_SPEC>,
    #[doc = "0x2b84 - Graphic MMU LUT entry 880 high"]
    pub lut880h: crate::Reg<lut880h::LUT880H_SPEC>,
    #[doc = "0x2b88 - Graphic MMU LUT entry 881 low"]
    pub lut881l: crate::Reg<lut881l::LUT881L_SPEC>,
    #[doc = "0x2b8c - Graphic MMU LUT entry 881 high"]
    pub lut881h: crate::Reg<lut881h::LUT881H_SPEC>,
    #[doc = "0x2b90 - Graphic MMU LUT entry 882 low"]
    pub lut882l: crate::Reg<lut882l::LUT882L_SPEC>,
    #[doc = "0x2b94 - Graphic MMU LUT entry 882 high"]
    pub lut882h: crate::Reg<lut882h::LUT882H_SPEC>,
    #[doc = "0x2b98 - Graphic MMU LUT entry 883 low"]
    pub lut883l: crate::Reg<lut883l::LUT883L_SPEC>,
    #[doc = "0x2b9c - Graphic MMU LUT entry 883 high"]
    pub lut883h: crate::Reg<lut883h::LUT883H_SPEC>,
    #[doc = "0x2ba0 - Graphic MMU LUT entry 884 low"]
    pub lut884l: crate::Reg<lut884l::LUT884L_SPEC>,
    #[doc = "0x2ba4 - Graphic MMU LUT entry 884 high"]
    pub lut884h: crate::Reg<lut884h::LUT884H_SPEC>,
    #[doc = "0x2ba8 - Graphic MMU LUT entry 885 low"]
    pub lut885l: crate::Reg<lut885l::LUT885L_SPEC>,
    #[doc = "0x2bac - Graphic MMU LUT entry 885 high"]
    pub lut885h: crate::Reg<lut885h::LUT885H_SPEC>,
    #[doc = "0x2bb0 - Graphic MMU LUT entry 886 low"]
    pub lut886l: crate::Reg<lut886l::LUT886L_SPEC>,
    #[doc = "0x2bb4 - Graphic MMU LUT entry 886 high"]
    pub lut886h: crate::Reg<lut886h::LUT886H_SPEC>,
    #[doc = "0x2bb8 - Graphic MMU LUT entry 887 low"]
    pub lut887l: crate::Reg<lut887l::LUT887L_SPEC>,
    #[doc = "0x2bbc - Graphic MMU LUT entry 887 high"]
    pub lut887h: crate::Reg<lut887h::LUT887H_SPEC>,
    #[doc = "0x2bc0 - Graphic MMU LUT entry 888 low"]
    pub lut888l: crate::Reg<lut888l::LUT888L_SPEC>,
    #[doc = "0x2bc4 - Graphic MMU LUT entry 888 high"]
    pub lut888h: crate::Reg<lut888h::LUT888H_SPEC>,
    #[doc = "0x2bc8 - Graphic MMU LUT entry 889 low"]
    pub lut889l: crate::Reg<lut889l::LUT889L_SPEC>,
    #[doc = "0x2bcc - Graphic MMU LUT entry 889 high"]
    pub lut889h: crate::Reg<lut889h::LUT889H_SPEC>,
    #[doc = "0x2bd0 - Graphic MMU LUT entry 890 low"]
    pub lut890l: crate::Reg<lut890l::LUT890L_SPEC>,
    #[doc = "0x2bd4 - Graphic MMU LUT entry 890 high"]
    pub lut890h: crate::Reg<lut890h::LUT890H_SPEC>,
    #[doc = "0x2bd8 - Graphic MMU LUT entry 891 low"]
    pub lut891l: crate::Reg<lut891l::LUT891L_SPEC>,
    #[doc = "0x2bdc - Graphic MMU LUT entry 891 high"]
    pub lut891h: crate::Reg<lut891h::LUT891H_SPEC>,
    #[doc = "0x2be0 - Graphic MMU LUT entry 892 low"]
    pub lut892l: crate::Reg<lut892l::LUT892L_SPEC>,
    #[doc = "0x2be4 - Graphic MMU LUT entry 892 high"]
    pub lut892h: crate::Reg<lut892h::LUT892H_SPEC>,
    #[doc = "0x2be8 - Graphic MMU LUT entry 893 low"]
    pub lut893l: crate::Reg<lut893l::LUT893L_SPEC>,
    #[doc = "0x2bec - Graphic MMU LUT entry 893 high"]
    pub lut893h: crate::Reg<lut893h::LUT893H_SPEC>,
    #[doc = "0x2bf0 - Graphic MMU LUT entry 894 low"]
    pub lut894l: crate::Reg<lut894l::LUT894L_SPEC>,
    #[doc = "0x2bf4 - Graphic MMU LUT entry 894 high"]
    pub lut894h: crate::Reg<lut894h::LUT894H_SPEC>,
    #[doc = "0x2bf8 - Graphic MMU LUT entry 895 low"]
    pub lut895l: crate::Reg<lut895l::LUT895L_SPEC>,
    #[doc = "0x2bfc - Graphic MMU LUT entry 895 high"]
    pub lut895h: crate::Reg<lut895h::LUT895H_SPEC>,
    #[doc = "0x2c00 - Graphic MMU LUT entry 896 low"]
    pub lut896l: crate::Reg<lut896l::LUT896L_SPEC>,
    #[doc = "0x2c04 - Graphic MMU LUT entry 896 high"]
    pub lut896h: crate::Reg<lut896h::LUT896H_SPEC>,
    #[doc = "0x2c08 - Graphic MMU LUT entry 897 low"]
    pub lut897l: crate::Reg<lut897l::LUT897L_SPEC>,
    #[doc = "0x2c0c - Graphic MMU LUT entry 897 high"]
    pub lut897h: crate::Reg<lut897h::LUT897H_SPEC>,
    #[doc = "0x2c10 - Graphic MMU LUT entry 898 low"]
    pub lut898l: crate::Reg<lut898l::LUT898L_SPEC>,
    #[doc = "0x2c14 - Graphic MMU LUT entry 898 high"]
    pub lut898h: crate::Reg<lut898h::LUT898H_SPEC>,
    #[doc = "0x2c18 - Graphic MMU LUT entry 899 low"]
    pub lut899l: crate::Reg<lut899l::LUT899L_SPEC>,
    #[doc = "0x2c1c - Graphic MMU LUT entry 899 high"]
    pub lut899h: crate::Reg<lut899h::LUT899H_SPEC>,
    #[doc = "0x2c20 - Graphic MMU LUT entry 900 low"]
    pub lut900l: crate::Reg<lut900l::LUT900L_SPEC>,
    #[doc = "0x2c24 - Graphic MMU LUT entry 900 high"]
    pub lut900h: crate::Reg<lut900h::LUT900H_SPEC>,
    #[doc = "0x2c28 - Graphic MMU LUT entry 901 low"]
    pub lut901l: crate::Reg<lut901l::LUT901L_SPEC>,
    #[doc = "0x2c2c - Graphic MMU LUT entry 901 high"]
    pub lut901h: crate::Reg<lut901h::LUT901H_SPEC>,
    #[doc = "0x2c30 - Graphic MMU LUT entry 902 low"]
    pub lut902l: crate::Reg<lut902l::LUT902L_SPEC>,
    #[doc = "0x2c34 - Graphic MMU LUT entry 902 high"]
    pub lut902h: crate::Reg<lut902h::LUT902H_SPEC>,
    #[doc = "0x2c38 - Graphic MMU LUT entry 903 low"]
    pub lut903l: crate::Reg<lut903l::LUT903L_SPEC>,
    #[doc = "0x2c3c - Graphic MMU LUT entry 903 high"]
    pub lut903h: crate::Reg<lut903h::LUT903H_SPEC>,
    #[doc = "0x2c40 - Graphic MMU LUT entry 904 low"]
    pub lut904l: crate::Reg<lut904l::LUT904L_SPEC>,
    #[doc = "0x2c44 - Graphic MMU LUT entry 904 high"]
    pub lut904h: crate::Reg<lut904h::LUT904H_SPEC>,
    #[doc = "0x2c48 - Graphic MMU LUT entry 905 low"]
    pub lut905l: crate::Reg<lut905l::LUT905L_SPEC>,
    #[doc = "0x2c4c - Graphic MMU LUT entry 905 high"]
    pub lut905h: crate::Reg<lut905h::LUT905H_SPEC>,
    #[doc = "0x2c50 - Graphic MMU LUT entry 906 low"]
    pub lut906l: crate::Reg<lut906l::LUT906L_SPEC>,
    #[doc = "0x2c54 - Graphic MMU LUT entry 906 high"]
    pub lut906h: crate::Reg<lut906h::LUT906H_SPEC>,
    #[doc = "0x2c58 - Graphic MMU LUT entry 907 low"]
    pub lut907l: crate::Reg<lut907l::LUT907L_SPEC>,
    #[doc = "0x2c5c - Graphic MMU LUT entry 907 high"]
    pub lut907h: crate::Reg<lut907h::LUT907H_SPEC>,
    #[doc = "0x2c60 - Graphic MMU LUT entry 908 low"]
    pub lut908l: crate::Reg<lut908l::LUT908L_SPEC>,
    #[doc = "0x2c64 - Graphic MMU LUT entry 908 high"]
    pub lut908h: crate::Reg<lut908h::LUT908H_SPEC>,
    #[doc = "0x2c68 - Graphic MMU LUT entry 909 low"]
    pub lut909l: crate::Reg<lut909l::LUT909L_SPEC>,
    #[doc = "0x2c6c - Graphic MMU LUT entry 909 high"]
    pub lut909h: crate::Reg<lut909h::LUT909H_SPEC>,
    #[doc = "0x2c70 - Graphic MMU LUT entry 910 low"]
    pub lut910l: crate::Reg<lut910l::LUT910L_SPEC>,
    #[doc = "0x2c74 - Graphic MMU LUT entry 910 high"]
    pub lut910h: crate::Reg<lut910h::LUT910H_SPEC>,
    #[doc = "0x2c78 - Graphic MMU LUT entry 911 low"]
    pub lut911l: crate::Reg<lut911l::LUT911L_SPEC>,
    #[doc = "0x2c7c - Graphic MMU LUT entry 911 high"]
    pub lut911h: crate::Reg<lut911h::LUT911H_SPEC>,
    #[doc = "0x2c80 - Graphic MMU LUT entry 912 low"]
    pub lut912l: crate::Reg<lut912l::LUT912L_SPEC>,
    #[doc = "0x2c84 - Graphic MMU LUT entry 912 high"]
    pub lut912h: crate::Reg<lut912h::LUT912H_SPEC>,
    #[doc = "0x2c88 - Graphic MMU LUT entry 913 low"]
    pub lut913l: crate::Reg<lut913l::LUT913L_SPEC>,
    #[doc = "0x2c8c - Graphic MMU LUT entry 913 high"]
    pub lut913h: crate::Reg<lut913h::LUT913H_SPEC>,
    #[doc = "0x2c90 - Graphic MMU LUT entry 914 low"]
    pub lut914l: crate::Reg<lut914l::LUT914L_SPEC>,
    #[doc = "0x2c94 - Graphic MMU LUT entry 914 high"]
    pub lut914h: crate::Reg<lut914h::LUT914H_SPEC>,
    #[doc = "0x2c98 - Graphic MMU LUT entry 915 low"]
    pub lut915l: crate::Reg<lut915l::LUT915L_SPEC>,
    #[doc = "0x2c9c - Graphic MMU LUT entry 915 high"]
    pub lut915h: crate::Reg<lut915h::LUT915H_SPEC>,
    #[doc = "0x2ca0 - Graphic MMU LUT entry 916 low"]
    pub lut916l: crate::Reg<lut916l::LUT916L_SPEC>,
    #[doc = "0x2ca4 - Graphic MMU LUT entry 916 high"]
    pub lut916h: crate::Reg<lut916h::LUT916H_SPEC>,
    #[doc = "0x2ca8 - Graphic MMU LUT entry 917 low"]
    pub lut917l: crate::Reg<lut917l::LUT917L_SPEC>,
    #[doc = "0x2cac - Graphic MMU LUT entry 917 high"]
    pub lut917h: crate::Reg<lut917h::LUT917H_SPEC>,
    #[doc = "0x2cb0 - Graphic MMU LUT entry 918 low"]
    pub lut918l: crate::Reg<lut918l::LUT918L_SPEC>,
    #[doc = "0x2cb4 - Graphic MMU LUT entry 918 high"]
    pub lut918h: crate::Reg<lut918h::LUT918H_SPEC>,
    #[doc = "0x2cb8 - Graphic MMU LUT entry 919 low"]
    pub lut919l: crate::Reg<lut919l::LUT919L_SPEC>,
    #[doc = "0x2cbc - Graphic MMU LUT entry 919 high"]
    pub lut919h: crate::Reg<lut919h::LUT919H_SPEC>,
    #[doc = "0x2cc0 - Graphic MMU LUT entry 920 low"]
    pub lut920l: crate::Reg<lut920l::LUT920L_SPEC>,
    #[doc = "0x2cc4 - Graphic MMU LUT entry 920 high"]
    pub lut920h: crate::Reg<lut920h::LUT920H_SPEC>,
    #[doc = "0x2cc8 - Graphic MMU LUT entry 921 low"]
    pub lut921l: crate::Reg<lut921l::LUT921L_SPEC>,
    #[doc = "0x2ccc - Graphic MMU LUT entry 921 high"]
    pub lut921h: crate::Reg<lut921h::LUT921H_SPEC>,
    #[doc = "0x2cd0 - Graphic MMU LUT entry 922 low"]
    pub lut922l: crate::Reg<lut922l::LUT922L_SPEC>,
    #[doc = "0x2cd4 - Graphic MMU LUT entry 922 high"]
    pub lut922h: crate::Reg<lut922h::LUT922H_SPEC>,
    #[doc = "0x2cd8 - Graphic MMU LUT entry 923 low"]
    pub lut923l: crate::Reg<lut923l::LUT923L_SPEC>,
    #[doc = "0x2cdc - Graphic MMU LUT entry 923 high"]
    pub lut923h: crate::Reg<lut923h::LUT923H_SPEC>,
    #[doc = "0x2ce0 - Graphic MMU LUT entry 924 low"]
    pub lut924l: crate::Reg<lut924l::LUT924L_SPEC>,
    #[doc = "0x2ce4 - Graphic MMU LUT entry 924 high"]
    pub lut924h: crate::Reg<lut924h::LUT924H_SPEC>,
    #[doc = "0x2ce8 - Graphic MMU LUT entry 925 low"]
    pub lut925l: crate::Reg<lut925l::LUT925L_SPEC>,
    #[doc = "0x2cec - Graphic MMU LUT entry 925 high"]
    pub lut925h: crate::Reg<lut925h::LUT925H_SPEC>,
    #[doc = "0x2cf0 - Graphic MMU LUT entry 926 low"]
    pub lut926l: crate::Reg<lut926l::LUT926L_SPEC>,
    #[doc = "0x2cf4 - Graphic MMU LUT entry 926 high"]
    pub lut926h: crate::Reg<lut926h::LUT926H_SPEC>,
    #[doc = "0x2cf8 - Graphic MMU LUT entry 927 low"]
    pub lut927l: crate::Reg<lut927l::LUT927L_SPEC>,
    #[doc = "0x2cfc - Graphic MMU LUT entry 927 high"]
    pub lut927h: crate::Reg<lut927h::LUT927H_SPEC>,
    #[doc = "0x2d00 - Graphic MMU LUT entry 928 low"]
    pub lut928l: crate::Reg<lut928l::LUT928L_SPEC>,
    #[doc = "0x2d04 - Graphic MMU LUT entry 928 high"]
    pub lut928h: crate::Reg<lut928h::LUT928H_SPEC>,
    #[doc = "0x2d08 - Graphic MMU LUT entry 929 low"]
    pub lut929l: crate::Reg<lut929l::LUT929L_SPEC>,
    #[doc = "0x2d0c - Graphic MMU LUT entry 929 high"]
    pub lut929h: crate::Reg<lut929h::LUT929H_SPEC>,
    #[doc = "0x2d10 - Graphic MMU LUT entry 930 low"]
    pub lut930l: crate::Reg<lut930l::LUT930L_SPEC>,
    #[doc = "0x2d14 - Graphic MMU LUT entry 930 high"]
    pub lut930h: crate::Reg<lut930h::LUT930H_SPEC>,
    #[doc = "0x2d18 - Graphic MMU LUT entry 931 low"]
    pub lut931l: crate::Reg<lut931l::LUT931L_SPEC>,
    #[doc = "0x2d1c - Graphic MMU LUT entry 931 high"]
    pub lut931h: crate::Reg<lut931h::LUT931H_SPEC>,
    #[doc = "0x2d20 - Graphic MMU LUT entry 932 low"]
    pub lut932l: crate::Reg<lut932l::LUT932L_SPEC>,
    #[doc = "0x2d24 - Graphic MMU LUT entry 932 high"]
    pub lut932h: crate::Reg<lut932h::LUT932H_SPEC>,
    #[doc = "0x2d28 - Graphic MMU LUT entry 933 low"]
    pub lut933l: crate::Reg<lut933l::LUT933L_SPEC>,
    #[doc = "0x2d2c - Graphic MMU LUT entry 933 high"]
    pub lut933h: crate::Reg<lut933h::LUT933H_SPEC>,
    #[doc = "0x2d30 - Graphic MMU LUT entry 934 low"]
    pub lut934l: crate::Reg<lut934l::LUT934L_SPEC>,
    #[doc = "0x2d34 - Graphic MMU LUT entry 934 high"]
    pub lut934h: crate::Reg<lut934h::LUT934H_SPEC>,
    #[doc = "0x2d38 - Graphic MMU LUT entry 935 low"]
    pub lut935l: crate::Reg<lut935l::LUT935L_SPEC>,
    #[doc = "0x2d3c - Graphic MMU LUT entry 935 high"]
    pub lut935h: crate::Reg<lut935h::LUT935H_SPEC>,
    #[doc = "0x2d40 - Graphic MMU LUT entry 936 low"]
    pub lut936l: crate::Reg<lut936l::LUT936L_SPEC>,
    #[doc = "0x2d44 - Graphic MMU LUT entry 936 high"]
    pub lut936h: crate::Reg<lut936h::LUT936H_SPEC>,
    #[doc = "0x2d48 - Graphic MMU LUT entry 937 low"]
    pub lut937l: crate::Reg<lut937l::LUT937L_SPEC>,
    #[doc = "0x2d4c - Graphic MMU LUT entry 937 high"]
    pub lut937h: crate::Reg<lut937h::LUT937H_SPEC>,
    #[doc = "0x2d50 - Graphic MMU LUT entry 938 low"]
    pub lut938l: crate::Reg<lut938l::LUT938L_SPEC>,
    #[doc = "0x2d54 - Graphic MMU LUT entry 938 high"]
    pub lut938h: crate::Reg<lut938h::LUT938H_SPEC>,
    #[doc = "0x2d58 - Graphic MMU LUT entry 939 low"]
    pub lut939l: crate::Reg<lut939l::LUT939L_SPEC>,
    #[doc = "0x2d5c - Graphic MMU LUT entry 939 high"]
    pub lut939h: crate::Reg<lut939h::LUT939H_SPEC>,
    #[doc = "0x2d60 - Graphic MMU LUT entry 940 low"]
    pub lut940l: crate::Reg<lut940l::LUT940L_SPEC>,
    #[doc = "0x2d64 - Graphic MMU LUT entry 940 high"]
    pub lut940h: crate::Reg<lut940h::LUT940H_SPEC>,
    #[doc = "0x2d68 - Graphic MMU LUT entry 941 low"]
    pub lut941l: crate::Reg<lut941l::LUT941L_SPEC>,
    #[doc = "0x2d6c - Graphic MMU LUT entry 941 high"]
    pub lut941h: crate::Reg<lut941h::LUT941H_SPEC>,
    #[doc = "0x2d70 - Graphic MMU LUT entry 942 low"]
    pub lut942l: crate::Reg<lut942l::LUT942L_SPEC>,
    #[doc = "0x2d74 - Graphic MMU LUT entry 942 high"]
    pub lut942h: crate::Reg<lut942h::LUT942H_SPEC>,
    #[doc = "0x2d78 - Graphic MMU LUT entry 943 low"]
    pub lut943l: crate::Reg<lut943l::LUT943L_SPEC>,
    #[doc = "0x2d7c - Graphic MMU LUT entry 943 high"]
    pub lut943h: crate::Reg<lut943h::LUT943H_SPEC>,
    #[doc = "0x2d80 - Graphic MMU LUT entry 944 low"]
    pub lut944l: crate::Reg<lut944l::LUT944L_SPEC>,
    #[doc = "0x2d84 - Graphic MMU LUT entry 944 high"]
    pub lut944h: crate::Reg<lut944h::LUT944H_SPEC>,
    #[doc = "0x2d88 - Graphic MMU LUT entry 945 low"]
    pub lut945l: crate::Reg<lut945l::LUT945L_SPEC>,
    #[doc = "0x2d8c - Graphic MMU LUT entry 945 high"]
    pub lut945h: crate::Reg<lut945h::LUT945H_SPEC>,
    #[doc = "0x2d90 - Graphic MMU LUT entry 946 low"]
    pub lut946l: crate::Reg<lut946l::LUT946L_SPEC>,
    #[doc = "0x2d94 - Graphic MMU LUT entry 946 high"]
    pub lut946h: crate::Reg<lut946h::LUT946H_SPEC>,
    #[doc = "0x2d98 - Graphic MMU LUT entry 947 low"]
    pub lut947l: crate::Reg<lut947l::LUT947L_SPEC>,
    #[doc = "0x2d9c - Graphic MMU LUT entry 947 high"]
    pub lut947h: crate::Reg<lut947h::LUT947H_SPEC>,
    #[doc = "0x2da0 - Graphic MMU LUT entry 948 low"]
    pub lut948l: crate::Reg<lut948l::LUT948L_SPEC>,
    #[doc = "0x2da4 - Graphic MMU LUT entry 948 high"]
    pub lut948h: crate::Reg<lut948h::LUT948H_SPEC>,
    #[doc = "0x2da8 - Graphic MMU LUT entry 949 low"]
    pub lut949l: crate::Reg<lut949l::LUT949L_SPEC>,
    #[doc = "0x2dac - Graphic MMU LUT entry 949 high"]
    pub lut949h: crate::Reg<lut949h::LUT949H_SPEC>,
    #[doc = "0x2db0 - Graphic MMU LUT entry 950 low"]
    pub lut950l: crate::Reg<lut950l::LUT950L_SPEC>,
    #[doc = "0x2db4 - Graphic MMU LUT entry 950 high"]
    pub lut950h: crate::Reg<lut950h::LUT950H_SPEC>,
    #[doc = "0x2db8 - Graphic MMU LUT entry 951 low"]
    pub lut951l: crate::Reg<lut951l::LUT951L_SPEC>,
    #[doc = "0x2dbc - Graphic MMU LUT entry 951 high"]
    pub lut951h: crate::Reg<lut951h::LUT951H_SPEC>,
    #[doc = "0x2dc0 - Graphic MMU LUT entry 952 low"]
    pub lut952l: crate::Reg<lut952l::LUT952L_SPEC>,
    #[doc = "0x2dc4 - Graphic MMU LUT entry 952 high"]
    pub lut952h: crate::Reg<lut952h::LUT952H_SPEC>,
    #[doc = "0x2dc8 - Graphic MMU LUT entry 953 low"]
    pub lut953l: crate::Reg<lut953l::LUT953L_SPEC>,
    #[doc = "0x2dcc - Graphic MMU LUT entry 953 high"]
    pub lut953h: crate::Reg<lut953h::LUT953H_SPEC>,
    #[doc = "0x2dd0 - Graphic MMU LUT entry 954 low"]
    pub lut954l: crate::Reg<lut954l::LUT954L_SPEC>,
    #[doc = "0x2dd4 - Graphic MMU LUT entry 954 high"]
    pub lut954h: crate::Reg<lut954h::LUT954H_SPEC>,
    #[doc = "0x2dd8 - Graphic MMU LUT entry 955 low"]
    pub lut955l: crate::Reg<lut955l::LUT955L_SPEC>,
    #[doc = "0x2ddc - Graphic MMU LUT entry 955 high"]
    pub lut955h: crate::Reg<lut955h::LUT955H_SPEC>,
    #[doc = "0x2de0 - Graphic MMU LUT entry 956 low"]
    pub lut956l: crate::Reg<lut956l::LUT956L_SPEC>,
    #[doc = "0x2de4 - Graphic MMU LUT entry 956 high"]
    pub lut956h: crate::Reg<lut956h::LUT956H_SPEC>,
    #[doc = "0x2de8 - Graphic MMU LUT entry 957 low"]
    pub lut957l: crate::Reg<lut957l::LUT957L_SPEC>,
    #[doc = "0x2dec - Graphic MMU LUT entry 957 high"]
    pub lut957h: crate::Reg<lut957h::LUT957H_SPEC>,
    #[doc = "0x2df0 - Graphic MMU LUT entry 958 low"]
    pub lut958l: crate::Reg<lut958l::LUT958L_SPEC>,
    #[doc = "0x2df4 - Graphic MMU LUT entry 958 high"]
    pub lut958h: crate::Reg<lut958h::LUT958H_SPEC>,
    #[doc = "0x2df8 - Graphic MMU LUT entry 959 low"]
    pub lut959l: crate::Reg<lut959l::LUT959L_SPEC>,
    #[doc = "0x2dfc - Graphic MMU LUT entry 959 high"]
    pub lut959h: crate::Reg<lut959h::LUT959H_SPEC>,
    #[doc = "0x2e00 - Graphic MMU LUT entry 960 low"]
    pub lut960l: crate::Reg<lut960l::LUT960L_SPEC>,
    #[doc = "0x2e04 - Graphic MMU LUT entry 960 high"]
    pub lut960h: crate::Reg<lut960h::LUT960H_SPEC>,
    #[doc = "0x2e08 - Graphic MMU LUT entry 961 low"]
    pub lut961l: crate::Reg<lut961l::LUT961L_SPEC>,
    #[doc = "0x2e0c - Graphic MMU LUT entry 961 high"]
    pub lut961h: crate::Reg<lut961h::LUT961H_SPEC>,
    #[doc = "0x2e10 - Graphic MMU LUT entry 962 low"]
    pub lut962l: crate::Reg<lut962l::LUT962L_SPEC>,
    #[doc = "0x2e14 - Graphic MMU LUT entry 962 high"]
    pub lut962h: crate::Reg<lut962h::LUT962H_SPEC>,
    #[doc = "0x2e18 - Graphic MMU LUT entry 963 low"]
    pub lut963l: crate::Reg<lut963l::LUT963L_SPEC>,
    #[doc = "0x2e1c - Graphic MMU LUT entry 963 high"]
    pub lut963h: crate::Reg<lut963h::LUT963H_SPEC>,
    #[doc = "0x2e20 - Graphic MMU LUT entry 964 low"]
    pub lut964l: crate::Reg<lut964l::LUT964L_SPEC>,
    #[doc = "0x2e24 - Graphic MMU LUT entry 964 high"]
    pub lut964h: crate::Reg<lut964h::LUT964H_SPEC>,
    #[doc = "0x2e28 - Graphic MMU LUT entry 965 low"]
    pub lut965l: crate::Reg<lut965l::LUT965L_SPEC>,
    #[doc = "0x2e2c - Graphic MMU LUT entry 965 high"]
    pub lut965h: crate::Reg<lut965h::LUT965H_SPEC>,
    #[doc = "0x2e30 - Graphic MMU LUT entry 966 low"]
    pub lut966l: crate::Reg<lut966l::LUT966L_SPEC>,
    #[doc = "0x2e34 - Graphic MMU LUT entry 966 high"]
    pub lut966h: crate::Reg<lut966h::LUT966H_SPEC>,
    #[doc = "0x2e38 - Graphic MMU LUT entry 967 low"]
    pub lut967l: crate::Reg<lut967l::LUT967L_SPEC>,
    #[doc = "0x2e3c - Graphic MMU LUT entry 967 high"]
    pub lut967h: crate::Reg<lut967h::LUT967H_SPEC>,
    #[doc = "0x2e40 - Graphic MMU LUT entry 968 low"]
    pub lut968l: crate::Reg<lut968l::LUT968L_SPEC>,
    #[doc = "0x2e44 - Graphic MMU LUT entry 968 high"]
    pub lut968h: crate::Reg<lut968h::LUT968H_SPEC>,
    #[doc = "0x2e48 - Graphic MMU LUT entry 969 low"]
    pub lut969l: crate::Reg<lut969l::LUT969L_SPEC>,
    #[doc = "0x2e4c - Graphic MMU LUT entry 969 high"]
    pub lut969h: crate::Reg<lut969h::LUT969H_SPEC>,
    #[doc = "0x2e50 - Graphic MMU LUT entry 970 low"]
    pub lut970l: crate::Reg<lut970l::LUT970L_SPEC>,
    #[doc = "0x2e54 - Graphic MMU LUT entry 970 high"]
    pub lut970h: crate::Reg<lut970h::LUT970H_SPEC>,
    #[doc = "0x2e58 - Graphic MMU LUT entry 971 low"]
    pub lut971l: crate::Reg<lut971l::LUT971L_SPEC>,
    #[doc = "0x2e5c - Graphic MMU LUT entry 971 high"]
    pub lut971h: crate::Reg<lut971h::LUT971H_SPEC>,
    #[doc = "0x2e60 - Graphic MMU LUT entry 972 low"]
    pub lut972l: crate::Reg<lut972l::LUT972L_SPEC>,
    #[doc = "0x2e64 - Graphic MMU LUT entry 972 high"]
    pub lut972h: crate::Reg<lut972h::LUT972H_SPEC>,
    #[doc = "0x2e68 - Graphic MMU LUT entry 973 low"]
    pub lut973l: crate::Reg<lut973l::LUT973L_SPEC>,
    #[doc = "0x2e6c - Graphic MMU LUT entry 973 high"]
    pub lut973h: crate::Reg<lut973h::LUT973H_SPEC>,
    #[doc = "0x2e70 - Graphic MMU LUT entry 974 low"]
    pub lut974l: crate::Reg<lut974l::LUT974L_SPEC>,
    #[doc = "0x2e74 - Graphic MMU LUT entry 974 high"]
    pub lut974h: crate::Reg<lut974h::LUT974H_SPEC>,
    #[doc = "0x2e78 - Graphic MMU LUT entry 975 low"]
    pub lut975l: crate::Reg<lut975l::LUT975L_SPEC>,
    #[doc = "0x2e7c - Graphic MMU LUT entry 975 high"]
    pub lut975h: crate::Reg<lut975h::LUT975H_SPEC>,
    #[doc = "0x2e80 - Graphic MMU LUT entry 976 low"]
    pub lut976l: crate::Reg<lut976l::LUT976L_SPEC>,
    #[doc = "0x2e84 - Graphic MMU LUT entry 976 high"]
    pub lut976h: crate::Reg<lut976h::LUT976H_SPEC>,
    #[doc = "0x2e88 - Graphic MMU LUT entry 977 low"]
    pub lut977l: crate::Reg<lut977l::LUT977L_SPEC>,
    #[doc = "0x2e8c - Graphic MMU LUT entry 977 high"]
    pub lut977h: crate::Reg<lut977h::LUT977H_SPEC>,
    #[doc = "0x2e90 - Graphic MMU LUT entry 978 low"]
    pub lut978l: crate::Reg<lut978l::LUT978L_SPEC>,
    #[doc = "0x2e94 - Graphic MMU LUT entry 978 high"]
    pub lut978h: crate::Reg<lut978h::LUT978H_SPEC>,
    #[doc = "0x2e98 - Graphic MMU LUT entry 979 low"]
    pub lut979l: crate::Reg<lut979l::LUT979L_SPEC>,
    #[doc = "0x2e9c - Graphic MMU LUT entry 979 high"]
    pub lut979h: crate::Reg<lut979h::LUT979H_SPEC>,
    #[doc = "0x2ea0 - Graphic MMU LUT entry 980 low"]
    pub lut980l: crate::Reg<lut980l::LUT980L_SPEC>,
    #[doc = "0x2ea4 - Graphic MMU LUT entry 980 high"]
    pub lut980h: crate::Reg<lut980h::LUT980H_SPEC>,
    #[doc = "0x2ea8 - Graphic MMU LUT entry 981 low"]
    pub lut981l: crate::Reg<lut981l::LUT981L_SPEC>,
    #[doc = "0x2eac - Graphic MMU LUT entry 981 high"]
    pub lut981h: crate::Reg<lut981h::LUT981H_SPEC>,
    #[doc = "0x2eb0 - Graphic MMU LUT entry 982 low"]
    pub lut982l: crate::Reg<lut982l::LUT982L_SPEC>,
    #[doc = "0x2eb4 - Graphic MMU LUT entry 982 high"]
    pub lut982h: crate::Reg<lut982h::LUT982H_SPEC>,
    #[doc = "0x2eb8 - Graphic MMU LUT entry 983 low"]
    pub lut983l: crate::Reg<lut983l::LUT983L_SPEC>,
    #[doc = "0x2ebc - Graphic MMU LUT entry 983 high"]
    pub lut983h: crate::Reg<lut983h::LUT983H_SPEC>,
    #[doc = "0x2ec0 - Graphic MMU LUT entry 984 low"]
    pub lut984l: crate::Reg<lut984l::LUT984L_SPEC>,
    #[doc = "0x2ec4 - Graphic MMU LUT entry 984 high"]
    pub lut984h: crate::Reg<lut984h::LUT984H_SPEC>,
    #[doc = "0x2ec8 - Graphic MMU LUT entry 985 low"]
    pub lut985l: crate::Reg<lut985l::LUT985L_SPEC>,
    #[doc = "0x2ecc - Graphic MMU LUT entry 985 high"]
    pub lut985h: crate::Reg<lut985h::LUT985H_SPEC>,
    #[doc = "0x2ed0 - Graphic MMU LUT entry 986 low"]
    pub lut986l: crate::Reg<lut986l::LUT986L_SPEC>,
    #[doc = "0x2ed4 - Graphic MMU LUT entry 986 high"]
    pub lut986h: crate::Reg<lut986h::LUT986H_SPEC>,
    #[doc = "0x2ed8 - Graphic MMU LUT entry 987 low"]
    pub lut987l: crate::Reg<lut987l::LUT987L_SPEC>,
    #[doc = "0x2edc - Graphic MMU LUT entry 987 high"]
    pub lut987h: crate::Reg<lut987h::LUT987H_SPEC>,
    #[doc = "0x2ee0 - Graphic MMU LUT entry 988 low"]
    pub lut988l: crate::Reg<lut988l::LUT988L_SPEC>,
    #[doc = "0x2ee4 - Graphic MMU LUT entry 988 high"]
    pub lut988h: crate::Reg<lut988h::LUT988H_SPEC>,
    #[doc = "0x2ee8 - Graphic MMU LUT entry 989 low"]
    pub lut989l: crate::Reg<lut989l::LUT989L_SPEC>,
    #[doc = "0x2eec - Graphic MMU LUT entry 989 high"]
    pub lut989h: crate::Reg<lut989h::LUT989H_SPEC>,
    #[doc = "0x2ef0 - Graphic MMU LUT entry 990 low"]
    pub lut990l: crate::Reg<lut990l::LUT990L_SPEC>,
    #[doc = "0x2ef4 - Graphic MMU LUT entry 990 high"]
    pub lut990h: crate::Reg<lut990h::LUT990H_SPEC>,
    #[doc = "0x2ef8 - Graphic MMU LUT entry 991 low"]
    pub lut991l: crate::Reg<lut991l::LUT991L_SPEC>,
    #[doc = "0x2efc - Graphic MMU LUT entry 991 high"]
    pub lut991h: crate::Reg<lut991h::LUT991H_SPEC>,
    #[doc = "0x2f00 - Graphic MMU LUT entry 992 low"]
    pub lut992l: crate::Reg<lut992l::LUT992L_SPEC>,
    #[doc = "0x2f04 - Graphic MMU LUT entry 992 high"]
    pub lut992h: crate::Reg<lut992h::LUT992H_SPEC>,
    #[doc = "0x2f08 - Graphic MMU LUT entry 993 low"]
    pub lut993l: crate::Reg<lut993l::LUT993L_SPEC>,
    #[doc = "0x2f0c - Graphic MMU LUT entry 993 high"]
    pub lut993h: crate::Reg<lut993h::LUT993H_SPEC>,
    #[doc = "0x2f10 - Graphic MMU LUT entry 994 low"]
    pub lut994l: crate::Reg<lut994l::LUT994L_SPEC>,
    #[doc = "0x2f14 - Graphic MMU LUT entry 994 high"]
    pub lut994h: crate::Reg<lut994h::LUT994H_SPEC>,
    #[doc = "0x2f18 - Graphic MMU LUT entry 995 low"]
    pub lut995l: crate::Reg<lut995l::LUT995L_SPEC>,
    #[doc = "0x2f1c - Graphic MMU LUT entry 995 high"]
    pub lut995h: crate::Reg<lut995h::LUT995H_SPEC>,
    #[doc = "0x2f20 - Graphic MMU LUT entry 996 low"]
    pub lut996l: crate::Reg<lut996l::LUT996L_SPEC>,
    #[doc = "0x2f24 - Graphic MMU LUT entry 996 high"]
    pub lut996h: crate::Reg<lut996h::LUT996H_SPEC>,
    #[doc = "0x2f28 - Graphic MMU LUT entry 997 low"]
    pub lut997l: crate::Reg<lut997l::LUT997L_SPEC>,
    #[doc = "0x2f2c - Graphic MMU LUT entry 997 high"]
    pub lut997h: crate::Reg<lut997h::LUT997H_SPEC>,
    #[doc = "0x2f30 - Graphic MMU LUT entry 998 low"]
    pub lut998l: crate::Reg<lut998l::LUT998L_SPEC>,
    #[doc = "0x2f34 - Graphic MMU LUT entry 998 high"]
    pub lut998h: crate::Reg<lut998h::LUT998H_SPEC>,
    #[doc = "0x2f38 - Graphic MMU LUT entry 999 low"]
    pub lut999l: crate::Reg<lut999l::LUT999L_SPEC>,
    #[doc = "0x2f3c - Graphic MMU LUT entry 999 high"]
    pub lut999h: crate::Reg<lut999h::LUT999H_SPEC>,
    #[doc = "0x2f40 - Graphic MMU LUT entry 1000 low"]
    pub lut1000l: crate::Reg<lut1000l::LUT1000L_SPEC>,
    #[doc = "0x2f44 - Graphic MMU LUT entry 1000 high"]
    pub lut1000h: crate::Reg<lut1000h::LUT1000H_SPEC>,
    #[doc = "0x2f48 - Graphic MMU LUT entry 1001 low"]
    pub lut1001l: crate::Reg<lut1001l::LUT1001L_SPEC>,
    #[doc = "0x2f4c - Graphic MMU LUT entry 1001 high"]
    pub lut1001h: crate::Reg<lut1001h::LUT1001H_SPEC>,
    #[doc = "0x2f50 - Graphic MMU LUT entry 1002 low"]
    pub lut1002l: crate::Reg<lut1002l::LUT1002L_SPEC>,
    #[doc = "0x2f54 - Graphic MMU LUT entry 1002 high"]
    pub lut1002h: crate::Reg<lut1002h::LUT1002H_SPEC>,
    #[doc = "0x2f58 - Graphic MMU LUT entry 1003 low"]
    pub lut1003l: crate::Reg<lut1003l::LUT1003L_SPEC>,
    #[doc = "0x2f5c - Graphic MMU LUT entry 1003 high"]
    pub lut1003h: crate::Reg<lut1003h::LUT1003H_SPEC>,
    #[doc = "0x2f60 - Graphic MMU LUT entry 1004 low"]
    pub lut1004l: crate::Reg<lut1004l::LUT1004L_SPEC>,
    #[doc = "0x2f64 - Graphic MMU LUT entry 1004 high"]
    pub lut1004h: crate::Reg<lut1004h::LUT1004H_SPEC>,
    #[doc = "0x2f68 - Graphic MMU LUT entry 1005 low"]
    pub lut1005l: crate::Reg<lut1005l::LUT1005L_SPEC>,
    #[doc = "0x2f6c - Graphic MMU LUT entry 1005 high"]
    pub lut1005h: crate::Reg<lut1005h::LUT1005H_SPEC>,
    #[doc = "0x2f70 - Graphic MMU LUT entry 1006 low"]
    pub lut1006l: crate::Reg<lut1006l::LUT1006L_SPEC>,
    #[doc = "0x2f74 - Graphic MMU LUT entry 1006 high"]
    pub lut1006h: crate::Reg<lut1006h::LUT1006H_SPEC>,
    #[doc = "0x2f78 - Graphic MMU LUT entry 1007 low"]
    pub lut1007l: crate::Reg<lut1007l::LUT1007L_SPEC>,
    #[doc = "0x2f7c - Graphic MMU LUT entry 1007 high"]
    pub lut1007h: crate::Reg<lut1007h::LUT1007H_SPEC>,
    #[doc = "0x2f80 - Graphic MMU LUT entry 1008 low"]
    pub lut1008l: crate::Reg<lut1008l::LUT1008L_SPEC>,
    #[doc = "0x2f84 - Graphic MMU LUT entry 1008 high"]
    pub lut1008h: crate::Reg<lut1008h::LUT1008H_SPEC>,
    #[doc = "0x2f88 - Graphic MMU LUT entry 1009 low"]
    pub lut1009l: crate::Reg<lut1009l::LUT1009L_SPEC>,
    #[doc = "0x2f8c - Graphic MMU LUT entry 1009 high"]
    pub lut1009h: crate::Reg<lut1009h::LUT1009H_SPEC>,
    #[doc = "0x2f90 - Graphic MMU LUT entry 1010 low"]
    pub lut1010l: crate::Reg<lut1010l::LUT1010L_SPEC>,
    #[doc = "0x2f94 - Graphic MMU LUT entry 1010 high"]
    pub lut1010h: crate::Reg<lut1010h::LUT1010H_SPEC>,
    #[doc = "0x2f98 - Graphic MMU LUT entry 1011 low"]
    pub lut1011l: crate::Reg<lut1011l::LUT1011L_SPEC>,
    #[doc = "0x2f9c - Graphic MMU LUT entry 1011 high"]
    pub lut1011h: crate::Reg<lut1011h::LUT1011H_SPEC>,
    #[doc = "0x2fa0 - Graphic MMU LUT entry 1012 low"]
    pub lut1012l: crate::Reg<lut1012l::LUT1012L_SPEC>,
    #[doc = "0x2fa4 - Graphic MMU LUT entry 1012 high"]
    pub lut1012h: crate::Reg<lut1012h::LUT1012H_SPEC>,
    #[doc = "0x2fa8 - Graphic MMU LUT entry 1013 low"]
    pub lut1013l: crate::Reg<lut1013l::LUT1013L_SPEC>,
    #[doc = "0x2fac - Graphic MMU LUT entry 1013 high"]
    pub lut1013h: crate::Reg<lut1013h::LUT1013H_SPEC>,
    #[doc = "0x2fb0 - Graphic MMU LUT entry 1014 low"]
    pub lut1014l: crate::Reg<lut1014l::LUT1014L_SPEC>,
    #[doc = "0x2fb4 - Graphic MMU LUT entry 1014 high"]
    pub lut1014h: crate::Reg<lut1014h::LUT1014H_SPEC>,
    #[doc = "0x2fb8 - Graphic MMU LUT entry 1015 low"]
    pub lut1015l: crate::Reg<lut1015l::LUT1015L_SPEC>,
    #[doc = "0x2fbc - Graphic MMU LUT entry 1015 high"]
    pub lut1015h: crate::Reg<lut1015h::LUT1015H_SPEC>,
    #[doc = "0x2fc0 - Graphic MMU LUT entry 1016 low"]
    pub lut1016l: crate::Reg<lut1016l::LUT1016L_SPEC>,
    #[doc = "0x2fc4 - Graphic MMU LUT entry 1016 high"]
    pub lut1016h: crate::Reg<lut1016h::LUT1016H_SPEC>,
    #[doc = "0x2fc8 - Graphic MMU LUT entry 1017 low"]
    pub lut1017l: crate::Reg<lut1017l::LUT1017L_SPEC>,
    #[doc = "0x2fcc - Graphic MMU LUT entry 1017 high"]
    pub lut1017h: crate::Reg<lut1017h::LUT1017H_SPEC>,
    #[doc = "0x2fd0 - Graphic MMU LUT entry 1018 low"]
    pub lut1018l: crate::Reg<lut1018l::LUT1018L_SPEC>,
    #[doc = "0x2fd4 - Graphic MMU LUT entry 1018 high"]
    pub lut1018h: crate::Reg<lut1018h::LUT1018H_SPEC>,
    #[doc = "0x2fd8 - Graphic MMU LUT entry 1019 low"]
    pub lut1019l: crate::Reg<lut1019l::LUT1019L_SPEC>,
    #[doc = "0x2fdc - Graphic MMU LUT entry 1019 high"]
    pub lut1019h: crate::Reg<lut1019h::LUT1019H_SPEC>,
    #[doc = "0x2fe0 - Graphic MMU LUT entry 1020 low"]
    pub lut1020l: crate::Reg<lut1020l::LUT1020L_SPEC>,
    #[doc = "0x2fe4 - Graphic MMU LUT entry 1020 high"]
    pub lut1020h: crate::Reg<lut1020h::LUT1020H_SPEC>,
    #[doc = "0x2fe8 - Graphic MMU LUT entry 1021 low"]
    pub lut1021l: crate::Reg<lut1021l::LUT1021L_SPEC>,
    #[doc = "0x2fec - Graphic MMU LUT entry 1021 high"]
    pub lut1021h: crate::Reg<lut1021h::LUT1021H_SPEC>,
    #[doc = "0x2ff0 - Graphic MMU LUT entry 1022 low"]
    pub lut1022l: crate::Reg<lut1022l::LUT1022L_SPEC>,
    #[doc = "0x2ff4 - Graphic MMU LUT entry 1022 high"]
    pub lut1022h: crate::Reg<lut1022h::LUT1022H_SPEC>,
    #[doc = "0x2ff8 - Graphic MMU LUT entry 1023 low"]
    pub lut1023l: crate::Reg<lut1023l::LUT1023L_SPEC>,
    #[doc = "0x2ffc - Graphic MMU LUT entry 1023 high"]
    pub lut1023h: crate::Reg<lut1023h::LUT1023H_SPEC>,
}
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Graphic MMU configuration register"]
pub mod cr;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Graphic MMU status register"]
pub mod sr;
#[doc = "FCR register accessor: an alias for `Reg<FCR_SPEC>`"]
pub type FCR = crate::Reg<fcr::FCR_SPEC>;
#[doc = "Graphic MMU flag clear register"]
pub mod fcr;
#[doc = "DVR register accessor: an alias for `Reg<DVR_SPEC>`"]
pub type DVR = crate::Reg<dvr::DVR_SPEC>;
#[doc = "Graphic MMU default value register"]
pub mod dvr;
#[doc = "B0CR register accessor: an alias for `Reg<B0CR_SPEC>`"]
pub type B0CR = crate::Reg<b0cr::B0CR_SPEC>;
#[doc = "Graphic MMU buffer 0 configuration register"]
pub mod b0cr;
#[doc = "B1CR register accessor: an alias for `Reg<B1CR_SPEC>`"]
pub type B1CR = crate::Reg<b1cr::B1CR_SPEC>;
#[doc = "Graphic MMU buffer 1 configuration register"]
pub mod b1cr;
#[doc = "B2CR register accessor: an alias for `Reg<B2CR_SPEC>`"]
pub type B2CR = crate::Reg<b2cr::B2CR_SPEC>;
#[doc = "Graphic MMU buffer 2 configuration register"]
pub mod b2cr;
#[doc = "B3CR register accessor: an alias for `Reg<B3CR_SPEC>`"]
pub type B3CR = crate::Reg<b3cr::B3CR_SPEC>;
#[doc = "Graphic MMU buffer 3 configuration register"]
pub mod b3cr;
#[doc = "VERR register accessor: an alias for `Reg<VERR_SPEC>`"]
pub type VERR = crate::Reg<verr::VERR_SPEC>;
#[doc = "Graphic MMU version register"]
pub mod verr;
#[doc = "IPIDR register accessor: an alias for `Reg<IPIDR_SPEC>`"]
pub type IPIDR = crate::Reg<ipidr::IPIDR_SPEC>;
#[doc = "Graphic MMU identification register"]
pub mod ipidr;
#[doc = "SIDR register accessor: an alias for `Reg<SIDR_SPEC>`"]
pub type SIDR = crate::Reg<sidr::SIDR_SPEC>;
#[doc = "Graphic MMU size identification register"]
pub mod sidr;
#[doc = "LUT0L register accessor: an alias for `Reg<LUT0L_SPEC>`"]
pub type LUT0L = crate::Reg<lut0l::LUT0L_SPEC>;
#[doc = "Graphic MMU LUT entry 0 low"]
pub mod lut0l;
#[doc = "LUT1L register accessor: an alias for `Reg<LUT1L_SPEC>`"]
pub type LUT1L = crate::Reg<lut1l::LUT1L_SPEC>;
#[doc = "Graphic MMU LUT entry 1 low"]
pub mod lut1l;
#[doc = "LUT2L register accessor: an alias for `Reg<LUT2L_SPEC>`"]
pub type LUT2L = crate::Reg<lut2l::LUT2L_SPEC>;
#[doc = "Graphic MMU LUT entry 2 low"]
pub mod lut2l;
#[doc = "LUT3L register accessor: an alias for `Reg<LUT3L_SPEC>`"]
pub type LUT3L = crate::Reg<lut3l::LUT3L_SPEC>;
#[doc = "Graphic MMU LUT entry 3 low"]
pub mod lut3l;
#[doc = "LUT4L register accessor: an alias for `Reg<LUT4L_SPEC>`"]
pub type LUT4L = crate::Reg<lut4l::LUT4L_SPEC>;
#[doc = "Graphic MMU LUT entry 4 low"]
pub mod lut4l;
#[doc = "LUT5L register accessor: an alias for `Reg<LUT5L_SPEC>`"]
pub type LUT5L = crate::Reg<lut5l::LUT5L_SPEC>;
#[doc = "Graphic MMU LUT entry 5 low"]
pub mod lut5l;
#[doc = "LUT6L register accessor: an alias for `Reg<LUT6L_SPEC>`"]
pub type LUT6L = crate::Reg<lut6l::LUT6L_SPEC>;
#[doc = "Graphic MMU LUT entry 6 low"]
pub mod lut6l;
#[doc = "LUT7L register accessor: an alias for `Reg<LUT7L_SPEC>`"]
pub type LUT7L = crate::Reg<lut7l::LUT7L_SPEC>;
#[doc = "Graphic MMU LUT entry 7 low"]
pub mod lut7l;
#[doc = "LUT8L register accessor: an alias for `Reg<LUT8L_SPEC>`"]
pub type LUT8L = crate::Reg<lut8l::LUT8L_SPEC>;
#[doc = "Graphic MMU LUT entry 8 low"]
pub mod lut8l;
#[doc = "LUT9L register accessor: an alias for `Reg<LUT9L_SPEC>`"]
pub type LUT9L = crate::Reg<lut9l::LUT9L_SPEC>;
#[doc = "Graphic MMU LUT entry 9 low"]
pub mod lut9l;
#[doc = "LUT10L register accessor: an alias for `Reg<LUT10L_SPEC>`"]
pub type LUT10L = crate::Reg<lut10l::LUT10L_SPEC>;
#[doc = "Graphic MMU LUT entry 10 low"]
pub mod lut10l;
#[doc = "LUT11L register accessor: an alias for `Reg<LUT11L_SPEC>`"]
pub type LUT11L = crate::Reg<lut11l::LUT11L_SPEC>;
#[doc = "Graphic MMU LUT entry 11 low"]
pub mod lut11l;
#[doc = "LUT12L register accessor: an alias for `Reg<LUT12L_SPEC>`"]
pub type LUT12L = crate::Reg<lut12l::LUT12L_SPEC>;
#[doc = "Graphic MMU LUT entry 12 low"]
pub mod lut12l;
#[doc = "LUT13L register accessor: an alias for `Reg<LUT13L_SPEC>`"]
pub type LUT13L = crate::Reg<lut13l::LUT13L_SPEC>;
#[doc = "Graphic MMU LUT entry 13 low"]
pub mod lut13l;
#[doc = "LUT14L register accessor: an alias for `Reg<LUT14L_SPEC>`"]
pub type LUT14L = crate::Reg<lut14l::LUT14L_SPEC>;
#[doc = "Graphic MMU LUT entry 14 low"]
pub mod lut14l;
#[doc = "LUT15L register accessor: an alias for `Reg<LUT15L_SPEC>`"]
pub type LUT15L = crate::Reg<lut15l::LUT15L_SPEC>;
#[doc = "Graphic MMU LUT entry 15 low"]
pub mod lut15l;
#[doc = "LUT16L register accessor: an alias for `Reg<LUT16L_SPEC>`"]
pub type LUT16L = crate::Reg<lut16l::LUT16L_SPEC>;
#[doc = "Graphic MMU LUT entry 16 low"]
pub mod lut16l;
#[doc = "LUT17L register accessor: an alias for `Reg<LUT17L_SPEC>`"]
pub type LUT17L = crate::Reg<lut17l::LUT17L_SPEC>;
#[doc = "Graphic MMU LUT entry 17 low"]
pub mod lut17l;
#[doc = "LUT18L register accessor: an alias for `Reg<LUT18L_SPEC>`"]
pub type LUT18L = crate::Reg<lut18l::LUT18L_SPEC>;
#[doc = "Graphic MMU LUT entry 18 low"]
pub mod lut18l;
#[doc = "LUT19L register accessor: an alias for `Reg<LUT19L_SPEC>`"]
pub type LUT19L = crate::Reg<lut19l::LUT19L_SPEC>;
#[doc = "Graphic MMU LUT entry 19 low"]
pub mod lut19l;
#[doc = "LUT20L register accessor: an alias for `Reg<LUT20L_SPEC>`"]
pub type LUT20L = crate::Reg<lut20l::LUT20L_SPEC>;
#[doc = "Graphic MMU LUT entry 20 low"]
pub mod lut20l;
#[doc = "LUT21L register accessor: an alias for `Reg<LUT21L_SPEC>`"]
pub type LUT21L = crate::Reg<lut21l::LUT21L_SPEC>;
#[doc = "Graphic MMU LUT entry 21 low"]
pub mod lut21l;
#[doc = "LUT22L register accessor: an alias for `Reg<LUT22L_SPEC>`"]
pub type LUT22L = crate::Reg<lut22l::LUT22L_SPEC>;
#[doc = "Graphic MMU LUT entry 22 low"]
pub mod lut22l;
#[doc = "LUT23L register accessor: an alias for `Reg<LUT23L_SPEC>`"]
pub type LUT23L = crate::Reg<lut23l::LUT23L_SPEC>;
#[doc = "Graphic MMU LUT entry 23 low"]
pub mod lut23l;
#[doc = "LUT24L register accessor: an alias for `Reg<LUT24L_SPEC>`"]
pub type LUT24L = crate::Reg<lut24l::LUT24L_SPEC>;
#[doc = "Graphic MMU LUT entry 24 low"]
pub mod lut24l;
#[doc = "LUT25L register accessor: an alias for `Reg<LUT25L_SPEC>`"]
pub type LUT25L = crate::Reg<lut25l::LUT25L_SPEC>;
#[doc = "Graphic MMU LUT entry 25 low"]
pub mod lut25l;
#[doc = "LUT26L register accessor: an alias for `Reg<LUT26L_SPEC>`"]
pub type LUT26L = crate::Reg<lut26l::LUT26L_SPEC>;
#[doc = "Graphic MMU LUT entry 26 low"]
pub mod lut26l;
#[doc = "LUT27L register accessor: an alias for `Reg<LUT27L_SPEC>`"]
pub type LUT27L = crate::Reg<lut27l::LUT27L_SPEC>;
#[doc = "Graphic MMU LUT entry 27 low"]
pub mod lut27l;
#[doc = "LUT28L register accessor: an alias for `Reg<LUT28L_SPEC>`"]
pub type LUT28L = crate::Reg<lut28l::LUT28L_SPEC>;
#[doc = "Graphic MMU LUT entry 28 low"]
pub mod lut28l;
#[doc = "LUT29L register accessor: an alias for `Reg<LUT29L_SPEC>`"]
pub type LUT29L = crate::Reg<lut29l::LUT29L_SPEC>;
#[doc = "Graphic MMU LUT entry 29 low"]
pub mod lut29l;
#[doc = "LUT30L register accessor: an alias for `Reg<LUT30L_SPEC>`"]
pub type LUT30L = crate::Reg<lut30l::LUT30L_SPEC>;
#[doc = "Graphic MMU LUT entry 30 low"]
pub mod lut30l;
#[doc = "LUT31L register accessor: an alias for `Reg<LUT31L_SPEC>`"]
pub type LUT31L = crate::Reg<lut31l::LUT31L_SPEC>;
#[doc = "Graphic MMU LUT entry 31 low"]
pub mod lut31l;
#[doc = "LUT32L register accessor: an alias for `Reg<LUT32L_SPEC>`"]
pub type LUT32L = crate::Reg<lut32l::LUT32L_SPEC>;
#[doc = "Graphic MMU LUT entry 32 low"]
pub mod lut32l;
#[doc = "LUT33L register accessor: an alias for `Reg<LUT33L_SPEC>`"]
pub type LUT33L = crate::Reg<lut33l::LUT33L_SPEC>;
#[doc = "Graphic MMU LUT entry 33 low"]
pub mod lut33l;
#[doc = "LUT34L register accessor: an alias for `Reg<LUT34L_SPEC>`"]
pub type LUT34L = crate::Reg<lut34l::LUT34L_SPEC>;
#[doc = "Graphic MMU LUT entry 34 low"]
pub mod lut34l;
#[doc = "LUT35L register accessor: an alias for `Reg<LUT35L_SPEC>`"]
pub type LUT35L = crate::Reg<lut35l::LUT35L_SPEC>;
#[doc = "Graphic MMU LUT entry 35 low"]
pub mod lut35l;
#[doc = "LUT36L register accessor: an alias for `Reg<LUT36L_SPEC>`"]
pub type LUT36L = crate::Reg<lut36l::LUT36L_SPEC>;
#[doc = "Graphic MMU LUT entry 36 low"]
pub mod lut36l;
#[doc = "LUT37L register accessor: an alias for `Reg<LUT37L_SPEC>`"]
pub type LUT37L = crate::Reg<lut37l::LUT37L_SPEC>;
#[doc = "Graphic MMU LUT entry 37 low"]
pub mod lut37l;
#[doc = "LUT38L register accessor: an alias for `Reg<LUT38L_SPEC>`"]
pub type LUT38L = crate::Reg<lut38l::LUT38L_SPEC>;
#[doc = "Graphic MMU LUT entry 38 low"]
pub mod lut38l;
#[doc = "LUT39L register accessor: an alias for `Reg<LUT39L_SPEC>`"]
pub type LUT39L = crate::Reg<lut39l::LUT39L_SPEC>;
#[doc = "Graphic MMU LUT entry 39 low"]
pub mod lut39l;
#[doc = "LUT40L register accessor: an alias for `Reg<LUT40L_SPEC>`"]
pub type LUT40L = crate::Reg<lut40l::LUT40L_SPEC>;
#[doc = "Graphic MMU LUT entry 40 low"]
pub mod lut40l;
#[doc = "LUT41L register accessor: an alias for `Reg<LUT41L_SPEC>`"]
pub type LUT41L = crate::Reg<lut41l::LUT41L_SPEC>;
#[doc = "Graphic MMU LUT entry 41 low"]
pub mod lut41l;
#[doc = "LUT42L register accessor: an alias for `Reg<LUT42L_SPEC>`"]
pub type LUT42L = crate::Reg<lut42l::LUT42L_SPEC>;
#[doc = "Graphic MMU LUT entry 42 low"]
pub mod lut42l;
#[doc = "LUT43L register accessor: an alias for `Reg<LUT43L_SPEC>`"]
pub type LUT43L = crate::Reg<lut43l::LUT43L_SPEC>;
#[doc = "Graphic MMU LUT entry 43 low"]
pub mod lut43l;
#[doc = "LUT44L register accessor: an alias for `Reg<LUT44L_SPEC>`"]
pub type LUT44L = crate::Reg<lut44l::LUT44L_SPEC>;
#[doc = "Graphic MMU LUT entry 44 low"]
pub mod lut44l;
#[doc = "LUT45L register accessor: an alias for `Reg<LUT45L_SPEC>`"]
pub type LUT45L = crate::Reg<lut45l::LUT45L_SPEC>;
#[doc = "Graphic MMU LUT entry 45 low"]
pub mod lut45l;
#[doc = "LUT46L register accessor: an alias for `Reg<LUT46L_SPEC>`"]
pub type LUT46L = crate::Reg<lut46l::LUT46L_SPEC>;
#[doc = "Graphic MMU LUT entry 46 low"]
pub mod lut46l;
#[doc = "LUT47L register accessor: an alias for `Reg<LUT47L_SPEC>`"]
pub type LUT47L = crate::Reg<lut47l::LUT47L_SPEC>;
#[doc = "Graphic MMU LUT entry 47 low"]
pub mod lut47l;
#[doc = "LUT48L register accessor: an alias for `Reg<LUT48L_SPEC>`"]
pub type LUT48L = crate::Reg<lut48l::LUT48L_SPEC>;
#[doc = "Graphic MMU LUT entry 48 low"]
pub mod lut48l;
#[doc = "LUT49L register accessor: an alias for `Reg<LUT49L_SPEC>`"]
pub type LUT49L = crate::Reg<lut49l::LUT49L_SPEC>;
#[doc = "Graphic MMU LUT entry 49 low"]
pub mod lut49l;
#[doc = "LUT50L register accessor: an alias for `Reg<LUT50L_SPEC>`"]
pub type LUT50L = crate::Reg<lut50l::LUT50L_SPEC>;
#[doc = "Graphic MMU LUT entry 50 low"]
pub mod lut50l;
#[doc = "LUT51L register accessor: an alias for `Reg<LUT51L_SPEC>`"]
pub type LUT51L = crate::Reg<lut51l::LUT51L_SPEC>;
#[doc = "Graphic MMU LUT entry 51 low"]
pub mod lut51l;
#[doc = "LUT52L register accessor: an alias for `Reg<LUT52L_SPEC>`"]
pub type LUT52L = crate::Reg<lut52l::LUT52L_SPEC>;
#[doc = "Graphic MMU LUT entry 52 low"]
pub mod lut52l;
#[doc = "LUT53L register accessor: an alias for `Reg<LUT53L_SPEC>`"]
pub type LUT53L = crate::Reg<lut53l::LUT53L_SPEC>;
#[doc = "Graphic MMU LUT entry 53 low"]
pub mod lut53l;
#[doc = "LUT54L register accessor: an alias for `Reg<LUT54L_SPEC>`"]
pub type LUT54L = crate::Reg<lut54l::LUT54L_SPEC>;
#[doc = "Graphic MMU LUT entry 54 low"]
pub mod lut54l;
#[doc = "LUT55L register accessor: an alias for `Reg<LUT55L_SPEC>`"]
pub type LUT55L = crate::Reg<lut55l::LUT55L_SPEC>;
#[doc = "Graphic MMU LUT entry 55 low"]
pub mod lut55l;
#[doc = "LUT56L register accessor: an alias for `Reg<LUT56L_SPEC>`"]
pub type LUT56L = crate::Reg<lut56l::LUT56L_SPEC>;
#[doc = "Graphic MMU LUT entry 56 low"]
pub mod lut56l;
#[doc = "LUT57L register accessor: an alias for `Reg<LUT57L_SPEC>`"]
pub type LUT57L = crate::Reg<lut57l::LUT57L_SPEC>;
#[doc = "Graphic MMU LUT entry 57 low"]
pub mod lut57l;
#[doc = "LUT58L register accessor: an alias for `Reg<LUT58L_SPEC>`"]
pub type LUT58L = crate::Reg<lut58l::LUT58L_SPEC>;
#[doc = "Graphic MMU LUT entry 58 low"]
pub mod lut58l;
#[doc = "LUT59L register accessor: an alias for `Reg<LUT59L_SPEC>`"]
pub type LUT59L = crate::Reg<lut59l::LUT59L_SPEC>;
#[doc = "Graphic MMU LUT entry 59 low"]
pub mod lut59l;
#[doc = "LUT60L register accessor: an alias for `Reg<LUT60L_SPEC>`"]
pub type LUT60L = crate::Reg<lut60l::LUT60L_SPEC>;
#[doc = "Graphic MMU LUT entry 60 low"]
pub mod lut60l;
#[doc = "LUT61L register accessor: an alias for `Reg<LUT61L_SPEC>`"]
pub type LUT61L = crate::Reg<lut61l::LUT61L_SPEC>;
#[doc = "Graphic MMU LUT entry 61 low"]
pub mod lut61l;
#[doc = "LUT62L register accessor: an alias for `Reg<LUT62L_SPEC>`"]
pub type LUT62L = crate::Reg<lut62l::LUT62L_SPEC>;
#[doc = "Graphic MMU LUT entry 62 low"]
pub mod lut62l;
#[doc = "LUT63L register accessor: an alias for `Reg<LUT63L_SPEC>`"]
pub type LUT63L = crate::Reg<lut63l::LUT63L_SPEC>;
#[doc = "Graphic MMU LUT entry 63 low"]
pub mod lut63l;
#[doc = "LUT64L register accessor: an alias for `Reg<LUT64L_SPEC>`"]
pub type LUT64L = crate::Reg<lut64l::LUT64L_SPEC>;
#[doc = "Graphic MMU LUT entry 64 low"]
pub mod lut64l;
#[doc = "LUT65L register accessor: an alias for `Reg<LUT65L_SPEC>`"]
pub type LUT65L = crate::Reg<lut65l::LUT65L_SPEC>;
#[doc = "Graphic MMU LUT entry 65 low"]
pub mod lut65l;
#[doc = "LUT66L register accessor: an alias for `Reg<LUT66L_SPEC>`"]
pub type LUT66L = crate::Reg<lut66l::LUT66L_SPEC>;
#[doc = "Graphic MMU LUT entry 66 low"]
pub mod lut66l;
#[doc = "LUT67L register accessor: an alias for `Reg<LUT67L_SPEC>`"]
pub type LUT67L = crate::Reg<lut67l::LUT67L_SPEC>;
#[doc = "Graphic MMU LUT entry 67 low"]
pub mod lut67l;
#[doc = "LUT68L register accessor: an alias for `Reg<LUT68L_SPEC>`"]
pub type LUT68L = crate::Reg<lut68l::LUT68L_SPEC>;
#[doc = "Graphic MMU LUT entry 68 low"]
pub mod lut68l;
#[doc = "LUT69L register accessor: an alias for `Reg<LUT69L_SPEC>`"]
pub type LUT69L = crate::Reg<lut69l::LUT69L_SPEC>;
#[doc = "Graphic MMU LUT entry 69 low"]
pub mod lut69l;
#[doc = "LUT70L register accessor: an alias for `Reg<LUT70L_SPEC>`"]
pub type LUT70L = crate::Reg<lut70l::LUT70L_SPEC>;
#[doc = "Graphic MMU LUT entry 70 low"]
pub mod lut70l;
#[doc = "LUT71L register accessor: an alias for `Reg<LUT71L_SPEC>`"]
pub type LUT71L = crate::Reg<lut71l::LUT71L_SPEC>;
#[doc = "Graphic MMU LUT entry 71 low"]
pub mod lut71l;
#[doc = "LUT72L register accessor: an alias for `Reg<LUT72L_SPEC>`"]
pub type LUT72L = crate::Reg<lut72l::LUT72L_SPEC>;
#[doc = "Graphic MMU LUT entry 72 low"]
pub mod lut72l;
#[doc = "LUT73L register accessor: an alias for `Reg<LUT73L_SPEC>`"]
pub type LUT73L = crate::Reg<lut73l::LUT73L_SPEC>;
#[doc = "Graphic MMU LUT entry 73 low"]
pub mod lut73l;
#[doc = "LUT74L register accessor: an alias for `Reg<LUT74L_SPEC>`"]
pub type LUT74L = crate::Reg<lut74l::LUT74L_SPEC>;
#[doc = "Graphic MMU LUT entry 74 low"]
pub mod lut74l;
#[doc = "LUT75L register accessor: an alias for `Reg<LUT75L_SPEC>`"]
pub type LUT75L = crate::Reg<lut75l::LUT75L_SPEC>;
#[doc = "Graphic MMU LUT entry 75 low"]
pub mod lut75l;
#[doc = "LUT76L register accessor: an alias for `Reg<LUT76L_SPEC>`"]
pub type LUT76L = crate::Reg<lut76l::LUT76L_SPEC>;
#[doc = "Graphic MMU LUT entry 76 low"]
pub mod lut76l;
#[doc = "LUT77L register accessor: an alias for `Reg<LUT77L_SPEC>`"]
pub type LUT77L = crate::Reg<lut77l::LUT77L_SPEC>;
#[doc = "Graphic MMU LUT entry 77 low"]
pub mod lut77l;
#[doc = "LUT78L register accessor: an alias for `Reg<LUT78L_SPEC>`"]
pub type LUT78L = crate::Reg<lut78l::LUT78L_SPEC>;
#[doc = "Graphic MMU LUT entry 78 low"]
pub mod lut78l;
#[doc = "LUT79L register accessor: an alias for `Reg<LUT79L_SPEC>`"]
pub type LUT79L = crate::Reg<lut79l::LUT79L_SPEC>;
#[doc = "Graphic MMU LUT entry 79 low"]
pub mod lut79l;
#[doc = "LUT80L register accessor: an alias for `Reg<LUT80L_SPEC>`"]
pub type LUT80L = crate::Reg<lut80l::LUT80L_SPEC>;
#[doc = "Graphic MMU LUT entry 80 low"]
pub mod lut80l;
#[doc = "LUT81L register accessor: an alias for `Reg<LUT81L_SPEC>`"]
pub type LUT81L = crate::Reg<lut81l::LUT81L_SPEC>;
#[doc = "Graphic MMU LUT entry 81 low"]
pub mod lut81l;
#[doc = "LUT82L register accessor: an alias for `Reg<LUT82L_SPEC>`"]
pub type LUT82L = crate::Reg<lut82l::LUT82L_SPEC>;
#[doc = "Graphic MMU LUT entry 82 low"]
pub mod lut82l;
#[doc = "LUT83L register accessor: an alias for `Reg<LUT83L_SPEC>`"]
pub type LUT83L = crate::Reg<lut83l::LUT83L_SPEC>;
#[doc = "Graphic MMU LUT entry 83 low"]
pub mod lut83l;
#[doc = "LUT84L register accessor: an alias for `Reg<LUT84L_SPEC>`"]
pub type LUT84L = crate::Reg<lut84l::LUT84L_SPEC>;
#[doc = "Graphic MMU LUT entry 84 low"]
pub mod lut84l;
#[doc = "LUT85L register accessor: an alias for `Reg<LUT85L_SPEC>`"]
pub type LUT85L = crate::Reg<lut85l::LUT85L_SPEC>;
#[doc = "Graphic MMU LUT entry 85 low"]
pub mod lut85l;
#[doc = "LUT86L register accessor: an alias for `Reg<LUT86L_SPEC>`"]
pub type LUT86L = crate::Reg<lut86l::LUT86L_SPEC>;
#[doc = "Graphic MMU LUT entry 86 low"]
pub mod lut86l;
#[doc = "LUT87L register accessor: an alias for `Reg<LUT87L_SPEC>`"]
pub type LUT87L = crate::Reg<lut87l::LUT87L_SPEC>;
#[doc = "Graphic MMU LUT entry 87 low"]
pub mod lut87l;
#[doc = "LUT88L register accessor: an alias for `Reg<LUT88L_SPEC>`"]
pub type LUT88L = crate::Reg<lut88l::LUT88L_SPEC>;
#[doc = "Graphic MMU LUT entry 88 low"]
pub mod lut88l;
#[doc = "LUT89L register accessor: an alias for `Reg<LUT89L_SPEC>`"]
pub type LUT89L = crate::Reg<lut89l::LUT89L_SPEC>;
#[doc = "Graphic MMU LUT entry 89 low"]
pub mod lut89l;
#[doc = "LUT90L register accessor: an alias for `Reg<LUT90L_SPEC>`"]
pub type LUT90L = crate::Reg<lut90l::LUT90L_SPEC>;
#[doc = "Graphic MMU LUT entry 90 low"]
pub mod lut90l;
#[doc = "LUT91L register accessor: an alias for `Reg<LUT91L_SPEC>`"]
pub type LUT91L = crate::Reg<lut91l::LUT91L_SPEC>;
#[doc = "Graphic MMU LUT entry 91 low"]
pub mod lut91l;
#[doc = "LUT92L register accessor: an alias for `Reg<LUT92L_SPEC>`"]
pub type LUT92L = crate::Reg<lut92l::LUT92L_SPEC>;
#[doc = "Graphic MMU LUT entry 92 low"]
pub mod lut92l;
#[doc = "LUT93L register accessor: an alias for `Reg<LUT93L_SPEC>`"]
pub type LUT93L = crate::Reg<lut93l::LUT93L_SPEC>;
#[doc = "Graphic MMU LUT entry 93 low"]
pub mod lut93l;
#[doc = "LUT94L register accessor: an alias for `Reg<LUT94L_SPEC>`"]
pub type LUT94L = crate::Reg<lut94l::LUT94L_SPEC>;
#[doc = "Graphic MMU LUT entry 94 low"]
pub mod lut94l;
#[doc = "LUT95L register accessor: an alias for `Reg<LUT95L_SPEC>`"]
pub type LUT95L = crate::Reg<lut95l::LUT95L_SPEC>;
#[doc = "Graphic MMU LUT entry 95 low"]
pub mod lut95l;
#[doc = "LUT96L register accessor: an alias for `Reg<LUT96L_SPEC>`"]
pub type LUT96L = crate::Reg<lut96l::LUT96L_SPEC>;
#[doc = "Graphic MMU LUT entry 96 low"]
pub mod lut96l;
#[doc = "LUT97L register accessor: an alias for `Reg<LUT97L_SPEC>`"]
pub type LUT97L = crate::Reg<lut97l::LUT97L_SPEC>;
#[doc = "Graphic MMU LUT entry 97 low"]
pub mod lut97l;
#[doc = "LUT98L register accessor: an alias for `Reg<LUT98L_SPEC>`"]
pub type LUT98L = crate::Reg<lut98l::LUT98L_SPEC>;
#[doc = "Graphic MMU LUT entry 98 low"]
pub mod lut98l;
#[doc = "LUT99L register accessor: an alias for `Reg<LUT99L_SPEC>`"]
pub type LUT99L = crate::Reg<lut99l::LUT99L_SPEC>;
#[doc = "Graphic MMU LUT entry 99 low"]
pub mod lut99l;
#[doc = "LUT100L register accessor: an alias for `Reg<LUT100L_SPEC>`"]
pub type LUT100L = crate::Reg<lut100l::LUT100L_SPEC>;
#[doc = "Graphic MMU LUT entry 100 low"]
pub mod lut100l;
#[doc = "LUT101L register accessor: an alias for `Reg<LUT101L_SPEC>`"]
pub type LUT101L = crate::Reg<lut101l::LUT101L_SPEC>;
#[doc = "Graphic MMU LUT entry 101 low"]
pub mod lut101l;
#[doc = "LUT102L register accessor: an alias for `Reg<LUT102L_SPEC>`"]
pub type LUT102L = crate::Reg<lut102l::LUT102L_SPEC>;
#[doc = "Graphic MMU LUT entry 102 low"]
pub mod lut102l;
#[doc = "LUT103L register accessor: an alias for `Reg<LUT103L_SPEC>`"]
pub type LUT103L = crate::Reg<lut103l::LUT103L_SPEC>;
#[doc = "Graphic MMU LUT entry 103 low"]
pub mod lut103l;
#[doc = "LUT104L register accessor: an alias for `Reg<LUT104L_SPEC>`"]
pub type LUT104L = crate::Reg<lut104l::LUT104L_SPEC>;
#[doc = "Graphic MMU LUT entry 104 low"]
pub mod lut104l;
#[doc = "LUT105L register accessor: an alias for `Reg<LUT105L_SPEC>`"]
pub type LUT105L = crate::Reg<lut105l::LUT105L_SPEC>;
#[doc = "Graphic MMU LUT entry 105 low"]
pub mod lut105l;
#[doc = "LUT106L register accessor: an alias for `Reg<LUT106L_SPEC>`"]
pub type LUT106L = crate::Reg<lut106l::LUT106L_SPEC>;
#[doc = "Graphic MMU LUT entry 106 low"]
pub mod lut106l;
#[doc = "LUT107L register accessor: an alias for `Reg<LUT107L_SPEC>`"]
pub type LUT107L = crate::Reg<lut107l::LUT107L_SPEC>;
#[doc = "Graphic MMU LUT entry 107 low"]
pub mod lut107l;
#[doc = "LUT108L register accessor: an alias for `Reg<LUT108L_SPEC>`"]
pub type LUT108L = crate::Reg<lut108l::LUT108L_SPEC>;
#[doc = "Graphic MMU LUT entry 108 low"]
pub mod lut108l;
#[doc = "LUT109L register accessor: an alias for `Reg<LUT109L_SPEC>`"]
pub type LUT109L = crate::Reg<lut109l::LUT109L_SPEC>;
#[doc = "Graphic MMU LUT entry 109 low"]
pub mod lut109l;
#[doc = "LUT110L register accessor: an alias for `Reg<LUT110L_SPEC>`"]
pub type LUT110L = crate::Reg<lut110l::LUT110L_SPEC>;
#[doc = "Graphic MMU LUT entry 110 low"]
pub mod lut110l;
#[doc = "LUT111L register accessor: an alias for `Reg<LUT111L_SPEC>`"]
pub type LUT111L = crate::Reg<lut111l::LUT111L_SPEC>;
#[doc = "Graphic MMU LUT entry 111 low"]
pub mod lut111l;
#[doc = "LUT112L register accessor: an alias for `Reg<LUT112L_SPEC>`"]
pub type LUT112L = crate::Reg<lut112l::LUT112L_SPEC>;
#[doc = "Graphic MMU LUT entry 112 low"]
pub mod lut112l;
#[doc = "LUT113L register accessor: an alias for `Reg<LUT113L_SPEC>`"]
pub type LUT113L = crate::Reg<lut113l::LUT113L_SPEC>;
#[doc = "Graphic MMU LUT entry 113 low"]
pub mod lut113l;
#[doc = "LUT114L register accessor: an alias for `Reg<LUT114L_SPEC>`"]
pub type LUT114L = crate::Reg<lut114l::LUT114L_SPEC>;
#[doc = "Graphic MMU LUT entry 114 low"]
pub mod lut114l;
#[doc = "LUT115L register accessor: an alias for `Reg<LUT115L_SPEC>`"]
pub type LUT115L = crate::Reg<lut115l::LUT115L_SPEC>;
#[doc = "Graphic MMU LUT entry 115 low"]
pub mod lut115l;
#[doc = "LUT116L register accessor: an alias for `Reg<LUT116L_SPEC>`"]
pub type LUT116L = crate::Reg<lut116l::LUT116L_SPEC>;
#[doc = "Graphic MMU LUT entry 116 low"]
pub mod lut116l;
#[doc = "LUT117L register accessor: an alias for `Reg<LUT117L_SPEC>`"]
pub type LUT117L = crate::Reg<lut117l::LUT117L_SPEC>;
#[doc = "Graphic MMU LUT entry 117 low"]
pub mod lut117l;
#[doc = "LUT118L register accessor: an alias for `Reg<LUT118L_SPEC>`"]
pub type LUT118L = crate::Reg<lut118l::LUT118L_SPEC>;
#[doc = "Graphic MMU LUT entry 118 low"]
pub mod lut118l;
#[doc = "LUT119L register accessor: an alias for `Reg<LUT119L_SPEC>`"]
pub type LUT119L = crate::Reg<lut119l::LUT119L_SPEC>;
#[doc = "Graphic MMU LUT entry 119 low"]
pub mod lut119l;
#[doc = "LUT120L register accessor: an alias for `Reg<LUT120L_SPEC>`"]
pub type LUT120L = crate::Reg<lut120l::LUT120L_SPEC>;
#[doc = "Graphic MMU LUT entry 120 low"]
pub mod lut120l;
#[doc = "LUT121L register accessor: an alias for `Reg<LUT121L_SPEC>`"]
pub type LUT121L = crate::Reg<lut121l::LUT121L_SPEC>;
#[doc = "Graphic MMU LUT entry 121 low"]
pub mod lut121l;
#[doc = "LUT122L register accessor: an alias for `Reg<LUT122L_SPEC>`"]
pub type LUT122L = crate::Reg<lut122l::LUT122L_SPEC>;
#[doc = "Graphic MMU LUT entry 122 low"]
pub mod lut122l;
#[doc = "LUT123L register accessor: an alias for `Reg<LUT123L_SPEC>`"]
pub type LUT123L = crate::Reg<lut123l::LUT123L_SPEC>;
#[doc = "Graphic MMU LUT entry 123 low"]
pub mod lut123l;
#[doc = "LUT124L register accessor: an alias for `Reg<LUT124L_SPEC>`"]
pub type LUT124L = crate::Reg<lut124l::LUT124L_SPEC>;
#[doc = "Graphic MMU LUT entry 124 low"]
pub mod lut124l;
#[doc = "LUT125L register accessor: an alias for `Reg<LUT125L_SPEC>`"]
pub type LUT125L = crate::Reg<lut125l::LUT125L_SPEC>;
#[doc = "Graphic MMU LUT entry 125 low"]
pub mod lut125l;
#[doc = "LUT126L register accessor: an alias for `Reg<LUT126L_SPEC>`"]
pub type LUT126L = crate::Reg<lut126l::LUT126L_SPEC>;
#[doc = "Graphic MMU LUT entry 126 low"]
pub mod lut126l;
#[doc = "LUT127L register accessor: an alias for `Reg<LUT127L_SPEC>`"]
pub type LUT127L = crate::Reg<lut127l::LUT127L_SPEC>;
#[doc = "Graphic MMU LUT entry 127 low"]
pub mod lut127l;
#[doc = "LUT128L register accessor: an alias for `Reg<LUT128L_SPEC>`"]
pub type LUT128L = crate::Reg<lut128l::LUT128L_SPEC>;
#[doc = "Graphic MMU LUT entry 128 low"]
pub mod lut128l;
#[doc = "LUT129L register accessor: an alias for `Reg<LUT129L_SPEC>`"]
pub type LUT129L = crate::Reg<lut129l::LUT129L_SPEC>;
#[doc = "Graphic MMU LUT entry 129 low"]
pub mod lut129l;
#[doc = "LUT130L register accessor: an alias for `Reg<LUT130L_SPEC>`"]
pub type LUT130L = crate::Reg<lut130l::LUT130L_SPEC>;
#[doc = "Graphic MMU LUT entry 130 low"]
pub mod lut130l;
#[doc = "LUT131L register accessor: an alias for `Reg<LUT131L_SPEC>`"]
pub type LUT131L = crate::Reg<lut131l::LUT131L_SPEC>;
#[doc = "Graphic MMU LUT entry 131 low"]
pub mod lut131l;
#[doc = "LUT132L register accessor: an alias for `Reg<LUT132L_SPEC>`"]
pub type LUT132L = crate::Reg<lut132l::LUT132L_SPEC>;
#[doc = "Graphic MMU LUT entry 132 low"]
pub mod lut132l;
#[doc = "LUT133L register accessor: an alias for `Reg<LUT133L_SPEC>`"]
pub type LUT133L = crate::Reg<lut133l::LUT133L_SPEC>;
#[doc = "Graphic MMU LUT entry 133 low"]
pub mod lut133l;
#[doc = "LUT134L register accessor: an alias for `Reg<LUT134L_SPEC>`"]
pub type LUT134L = crate::Reg<lut134l::LUT134L_SPEC>;
#[doc = "Graphic MMU LUT entry 134 low"]
pub mod lut134l;
#[doc = "LUT135L register accessor: an alias for `Reg<LUT135L_SPEC>`"]
pub type LUT135L = crate::Reg<lut135l::LUT135L_SPEC>;
#[doc = "Graphic MMU LUT entry 135 low"]
pub mod lut135l;
#[doc = "LUT136L register accessor: an alias for `Reg<LUT136L_SPEC>`"]
pub type LUT136L = crate::Reg<lut136l::LUT136L_SPEC>;
#[doc = "Graphic MMU LUT entry 136 low"]
pub mod lut136l;
#[doc = "LUT137L register accessor: an alias for `Reg<LUT137L_SPEC>`"]
pub type LUT137L = crate::Reg<lut137l::LUT137L_SPEC>;
#[doc = "Graphic MMU LUT entry 137 low"]
pub mod lut137l;
#[doc = "LUT138L register accessor: an alias for `Reg<LUT138L_SPEC>`"]
pub type LUT138L = crate::Reg<lut138l::LUT138L_SPEC>;
#[doc = "Graphic MMU LUT entry 138 low"]
pub mod lut138l;
#[doc = "LUT139L register accessor: an alias for `Reg<LUT139L_SPEC>`"]
pub type LUT139L = crate::Reg<lut139l::LUT139L_SPEC>;
#[doc = "Graphic MMU LUT entry 139 low"]
pub mod lut139l;
#[doc = "LUT140L register accessor: an alias for `Reg<LUT140L_SPEC>`"]
pub type LUT140L = crate::Reg<lut140l::LUT140L_SPEC>;
#[doc = "Graphic MMU LUT entry 140 low"]
pub mod lut140l;
#[doc = "LUT141L register accessor: an alias for `Reg<LUT141L_SPEC>`"]
pub type LUT141L = crate::Reg<lut141l::LUT141L_SPEC>;
#[doc = "Graphic MMU LUT entry 141 low"]
pub mod lut141l;
#[doc = "LUT142L register accessor: an alias for `Reg<LUT142L_SPEC>`"]
pub type LUT142L = crate::Reg<lut142l::LUT142L_SPEC>;
#[doc = "Graphic MMU LUT entry 142 low"]
pub mod lut142l;
#[doc = "LUT143L register accessor: an alias for `Reg<LUT143L_SPEC>`"]
pub type LUT143L = crate::Reg<lut143l::LUT143L_SPEC>;
#[doc = "Graphic MMU LUT entry 143 low"]
pub mod lut143l;
#[doc = "LUT144L register accessor: an alias for `Reg<LUT144L_SPEC>`"]
pub type LUT144L = crate::Reg<lut144l::LUT144L_SPEC>;
#[doc = "Graphic MMU LUT entry 144 low"]
pub mod lut144l;
#[doc = "LUT145L register accessor: an alias for `Reg<LUT145L_SPEC>`"]
pub type LUT145L = crate::Reg<lut145l::LUT145L_SPEC>;
#[doc = "Graphic MMU LUT entry 145 low"]
pub mod lut145l;
#[doc = "LUT146L register accessor: an alias for `Reg<LUT146L_SPEC>`"]
pub type LUT146L = crate::Reg<lut146l::LUT146L_SPEC>;
#[doc = "Graphic MMU LUT entry 146 low"]
pub mod lut146l;
#[doc = "LUT147L register accessor: an alias for `Reg<LUT147L_SPEC>`"]
pub type LUT147L = crate::Reg<lut147l::LUT147L_SPEC>;
#[doc = "Graphic MMU LUT entry 147 low"]
pub mod lut147l;
#[doc = "LUT148L register accessor: an alias for `Reg<LUT148L_SPEC>`"]
pub type LUT148L = crate::Reg<lut148l::LUT148L_SPEC>;
#[doc = "Graphic MMU LUT entry 148 low"]
pub mod lut148l;
#[doc = "LUT149L register accessor: an alias for `Reg<LUT149L_SPEC>`"]
pub type LUT149L = crate::Reg<lut149l::LUT149L_SPEC>;
#[doc = "Graphic MMU LUT entry 149 low"]
pub mod lut149l;
#[doc = "LUT150L register accessor: an alias for `Reg<LUT150L_SPEC>`"]
pub type LUT150L = crate::Reg<lut150l::LUT150L_SPEC>;
#[doc = "Graphic MMU LUT entry 150 low"]
pub mod lut150l;
#[doc = "LUT151L register accessor: an alias for `Reg<LUT151L_SPEC>`"]
pub type LUT151L = crate::Reg<lut151l::LUT151L_SPEC>;
#[doc = "Graphic MMU LUT entry 151 low"]
pub mod lut151l;
#[doc = "LUT152L register accessor: an alias for `Reg<LUT152L_SPEC>`"]
pub type LUT152L = crate::Reg<lut152l::LUT152L_SPEC>;
#[doc = "Graphic MMU LUT entry 152 low"]
pub mod lut152l;
#[doc = "LUT153L register accessor: an alias for `Reg<LUT153L_SPEC>`"]
pub type LUT153L = crate::Reg<lut153l::LUT153L_SPEC>;
#[doc = "Graphic MMU LUT entry 153 low"]
pub mod lut153l;
#[doc = "LUT154L register accessor: an alias for `Reg<LUT154L_SPEC>`"]
pub type LUT154L = crate::Reg<lut154l::LUT154L_SPEC>;
#[doc = "Graphic MMU LUT entry 154 low"]
pub mod lut154l;
#[doc = "LUT155L register accessor: an alias for `Reg<LUT155L_SPEC>`"]
pub type LUT155L = crate::Reg<lut155l::LUT155L_SPEC>;
#[doc = "Graphic MMU LUT entry 155 low"]
pub mod lut155l;
#[doc = "LUT156L register accessor: an alias for `Reg<LUT156L_SPEC>`"]
pub type LUT156L = crate::Reg<lut156l::LUT156L_SPEC>;
#[doc = "Graphic MMU LUT entry 156 low"]
pub mod lut156l;
#[doc = "LUT157L register accessor: an alias for `Reg<LUT157L_SPEC>`"]
pub type LUT157L = crate::Reg<lut157l::LUT157L_SPEC>;
#[doc = "Graphic MMU LUT entry 157 low"]
pub mod lut157l;
#[doc = "LUT158L register accessor: an alias for `Reg<LUT158L_SPEC>`"]
pub type LUT158L = crate::Reg<lut158l::LUT158L_SPEC>;
#[doc = "Graphic MMU LUT entry 158 low"]
pub mod lut158l;
#[doc = "LUT159L register accessor: an alias for `Reg<LUT159L_SPEC>`"]
pub type LUT159L = crate::Reg<lut159l::LUT159L_SPEC>;
#[doc = "Graphic MMU LUT entry 159 low"]
pub mod lut159l;
#[doc = "LUT160L register accessor: an alias for `Reg<LUT160L_SPEC>`"]
pub type LUT160L = crate::Reg<lut160l::LUT160L_SPEC>;
#[doc = "Graphic MMU LUT entry 160 low"]
pub mod lut160l;
#[doc = "LUT161L register accessor: an alias for `Reg<LUT161L_SPEC>`"]
pub type LUT161L = crate::Reg<lut161l::LUT161L_SPEC>;
#[doc = "Graphic MMU LUT entry 161 low"]
pub mod lut161l;
#[doc = "LUT162L register accessor: an alias for `Reg<LUT162L_SPEC>`"]
pub type LUT162L = crate::Reg<lut162l::LUT162L_SPEC>;
#[doc = "Graphic MMU LUT entry 162 low"]
pub mod lut162l;
#[doc = "LUT163L register accessor: an alias for `Reg<LUT163L_SPEC>`"]
pub type LUT163L = crate::Reg<lut163l::LUT163L_SPEC>;
#[doc = "Graphic MMU LUT entry 163 low"]
pub mod lut163l;
#[doc = "LUT164L register accessor: an alias for `Reg<LUT164L_SPEC>`"]
pub type LUT164L = crate::Reg<lut164l::LUT164L_SPEC>;
#[doc = "Graphic MMU LUT entry 164 low"]
pub mod lut164l;
#[doc = "LUT165L register accessor: an alias for `Reg<LUT165L_SPEC>`"]
pub type LUT165L = crate::Reg<lut165l::LUT165L_SPEC>;
#[doc = "Graphic MMU LUT entry 165 low"]
pub mod lut165l;
#[doc = "LUT166L register accessor: an alias for `Reg<LUT166L_SPEC>`"]
pub type LUT166L = crate::Reg<lut166l::LUT166L_SPEC>;
#[doc = "Graphic MMU LUT entry 166 low"]
pub mod lut166l;
#[doc = "LUT167L register accessor: an alias for `Reg<LUT167L_SPEC>`"]
pub type LUT167L = crate::Reg<lut167l::LUT167L_SPEC>;
#[doc = "Graphic MMU LUT entry 167 low"]
pub mod lut167l;
#[doc = "LUT168L register accessor: an alias for `Reg<LUT168L_SPEC>`"]
pub type LUT168L = crate::Reg<lut168l::LUT168L_SPEC>;
#[doc = "Graphic MMU LUT entry 168 low"]
pub mod lut168l;
#[doc = "LUT169L register accessor: an alias for `Reg<LUT169L_SPEC>`"]
pub type LUT169L = crate::Reg<lut169l::LUT169L_SPEC>;
#[doc = "Graphic MMU LUT entry 169 low"]
pub mod lut169l;
#[doc = "LUT170L register accessor: an alias for `Reg<LUT170L_SPEC>`"]
pub type LUT170L = crate::Reg<lut170l::LUT170L_SPEC>;
#[doc = "Graphic MMU LUT entry 170 low"]
pub mod lut170l;
#[doc = "LUT171L register accessor: an alias for `Reg<LUT171L_SPEC>`"]
pub type LUT171L = crate::Reg<lut171l::LUT171L_SPEC>;
#[doc = "Graphic MMU LUT entry 171 low"]
pub mod lut171l;
#[doc = "LUT172L register accessor: an alias for `Reg<LUT172L_SPEC>`"]
pub type LUT172L = crate::Reg<lut172l::LUT172L_SPEC>;
#[doc = "Graphic MMU LUT entry 172 low"]
pub mod lut172l;
#[doc = "LUT173L register accessor: an alias for `Reg<LUT173L_SPEC>`"]
pub type LUT173L = crate::Reg<lut173l::LUT173L_SPEC>;
#[doc = "Graphic MMU LUT entry 173 low"]
pub mod lut173l;
#[doc = "LUT174L register accessor: an alias for `Reg<LUT174L_SPEC>`"]
pub type LUT174L = crate::Reg<lut174l::LUT174L_SPEC>;
#[doc = "Graphic MMU LUT entry 174 low"]
pub mod lut174l;
#[doc = "LUT175L register accessor: an alias for `Reg<LUT175L_SPEC>`"]
pub type LUT175L = crate::Reg<lut175l::LUT175L_SPEC>;
#[doc = "Graphic MMU LUT entry 175 low"]
pub mod lut175l;
#[doc = "LUT176L register accessor: an alias for `Reg<LUT176L_SPEC>`"]
pub type LUT176L = crate::Reg<lut176l::LUT176L_SPEC>;
#[doc = "Graphic MMU LUT entry 176 low"]
pub mod lut176l;
#[doc = "LUT177L register accessor: an alias for `Reg<LUT177L_SPEC>`"]
pub type LUT177L = crate::Reg<lut177l::LUT177L_SPEC>;
#[doc = "Graphic MMU LUT entry 177 low"]
pub mod lut177l;
#[doc = "LUT178L register accessor: an alias for `Reg<LUT178L_SPEC>`"]
pub type LUT178L = crate::Reg<lut178l::LUT178L_SPEC>;
#[doc = "Graphic MMU LUT entry 178 low"]
pub mod lut178l;
#[doc = "LUT179L register accessor: an alias for `Reg<LUT179L_SPEC>`"]
pub type LUT179L = crate::Reg<lut179l::LUT179L_SPEC>;
#[doc = "Graphic MMU LUT entry 179 low"]
pub mod lut179l;
#[doc = "LUT180L register accessor: an alias for `Reg<LUT180L_SPEC>`"]
pub type LUT180L = crate::Reg<lut180l::LUT180L_SPEC>;
#[doc = "Graphic MMU LUT entry 180 low"]
pub mod lut180l;
#[doc = "LUT181L register accessor: an alias for `Reg<LUT181L_SPEC>`"]
pub type LUT181L = crate::Reg<lut181l::LUT181L_SPEC>;
#[doc = "Graphic MMU LUT entry 181 low"]
pub mod lut181l;
#[doc = "LUT182L register accessor: an alias for `Reg<LUT182L_SPEC>`"]
pub type LUT182L = crate::Reg<lut182l::LUT182L_SPEC>;
#[doc = "Graphic MMU LUT entry 182 low"]
pub mod lut182l;
#[doc = "LUT183L register accessor: an alias for `Reg<LUT183L_SPEC>`"]
pub type LUT183L = crate::Reg<lut183l::LUT183L_SPEC>;
#[doc = "Graphic MMU LUT entry 183 low"]
pub mod lut183l;
#[doc = "LUT184L register accessor: an alias for `Reg<LUT184L_SPEC>`"]
pub type LUT184L = crate::Reg<lut184l::LUT184L_SPEC>;
#[doc = "Graphic MMU LUT entry 184 low"]
pub mod lut184l;
#[doc = "LUT185L register accessor: an alias for `Reg<LUT185L_SPEC>`"]
pub type LUT185L = crate::Reg<lut185l::LUT185L_SPEC>;
#[doc = "Graphic MMU LUT entry 185 low"]
pub mod lut185l;
#[doc = "LUT186L register accessor: an alias for `Reg<LUT186L_SPEC>`"]
pub type LUT186L = crate::Reg<lut186l::LUT186L_SPEC>;
#[doc = "Graphic MMU LUT entry 186 low"]
pub mod lut186l;
#[doc = "LUT187L register accessor: an alias for `Reg<LUT187L_SPEC>`"]
pub type LUT187L = crate::Reg<lut187l::LUT187L_SPEC>;
#[doc = "Graphic MMU LUT entry 187 low"]
pub mod lut187l;
#[doc = "LUT188L register accessor: an alias for `Reg<LUT188L_SPEC>`"]
pub type LUT188L = crate::Reg<lut188l::LUT188L_SPEC>;
#[doc = "Graphic MMU LUT entry 188 low"]
pub mod lut188l;
#[doc = "LUT189L register accessor: an alias for `Reg<LUT189L_SPEC>`"]
pub type LUT189L = crate::Reg<lut189l::LUT189L_SPEC>;
#[doc = "Graphic MMU LUT entry 189 low"]
pub mod lut189l;
#[doc = "LUT190L register accessor: an alias for `Reg<LUT190L_SPEC>`"]
pub type LUT190L = crate::Reg<lut190l::LUT190L_SPEC>;
#[doc = "Graphic MMU LUT entry 190 low"]
pub mod lut190l;
#[doc = "LUT191L register accessor: an alias for `Reg<LUT191L_SPEC>`"]
pub type LUT191L = crate::Reg<lut191l::LUT191L_SPEC>;
#[doc = "Graphic MMU LUT entry 191 low"]
pub mod lut191l;
#[doc = "LUT192L register accessor: an alias for `Reg<LUT192L_SPEC>`"]
pub type LUT192L = crate::Reg<lut192l::LUT192L_SPEC>;
#[doc = "Graphic MMU LUT entry 192 low"]
pub mod lut192l;
#[doc = "LUT193L register accessor: an alias for `Reg<LUT193L_SPEC>`"]
pub type LUT193L = crate::Reg<lut193l::LUT193L_SPEC>;
#[doc = "Graphic MMU LUT entry 193 low"]
pub mod lut193l;
#[doc = "LUT194L register accessor: an alias for `Reg<LUT194L_SPEC>`"]
pub type LUT194L = crate::Reg<lut194l::LUT194L_SPEC>;
#[doc = "Graphic MMU LUT entry 194 low"]
pub mod lut194l;
#[doc = "LUT195L register accessor: an alias for `Reg<LUT195L_SPEC>`"]
pub type LUT195L = crate::Reg<lut195l::LUT195L_SPEC>;
#[doc = "Graphic MMU LUT entry 195 low"]
pub mod lut195l;
#[doc = "LUT196L register accessor: an alias for `Reg<LUT196L_SPEC>`"]
pub type LUT196L = crate::Reg<lut196l::LUT196L_SPEC>;
#[doc = "Graphic MMU LUT entry 196 low"]
pub mod lut196l;
#[doc = "LUT197L register accessor: an alias for `Reg<LUT197L_SPEC>`"]
pub type LUT197L = crate::Reg<lut197l::LUT197L_SPEC>;
#[doc = "Graphic MMU LUT entry 197 low"]
pub mod lut197l;
#[doc = "LUT198L register accessor: an alias for `Reg<LUT198L_SPEC>`"]
pub type LUT198L = crate::Reg<lut198l::LUT198L_SPEC>;
#[doc = "Graphic MMU LUT entry 198 low"]
pub mod lut198l;
#[doc = "LUT199L register accessor: an alias for `Reg<LUT199L_SPEC>`"]
pub type LUT199L = crate::Reg<lut199l::LUT199L_SPEC>;
#[doc = "Graphic MMU LUT entry 199 low"]
pub mod lut199l;
#[doc = "LUT200L register accessor: an alias for `Reg<LUT200L_SPEC>`"]
pub type LUT200L = crate::Reg<lut200l::LUT200L_SPEC>;
#[doc = "Graphic MMU LUT entry 200 low"]
pub mod lut200l;
#[doc = "LUT201L register accessor: an alias for `Reg<LUT201L_SPEC>`"]
pub type LUT201L = crate::Reg<lut201l::LUT201L_SPEC>;
#[doc = "Graphic MMU LUT entry 201 low"]
pub mod lut201l;
#[doc = "LUT202L register accessor: an alias for `Reg<LUT202L_SPEC>`"]
pub type LUT202L = crate::Reg<lut202l::LUT202L_SPEC>;
#[doc = "Graphic MMU LUT entry 202 low"]
pub mod lut202l;
#[doc = "LUT203L register accessor: an alias for `Reg<LUT203L_SPEC>`"]
pub type LUT203L = crate::Reg<lut203l::LUT203L_SPEC>;
#[doc = "Graphic MMU LUT entry 203 low"]
pub mod lut203l;
#[doc = "LUT204L register accessor: an alias for `Reg<LUT204L_SPEC>`"]
pub type LUT204L = crate::Reg<lut204l::LUT204L_SPEC>;
#[doc = "Graphic MMU LUT entry 204 low"]
pub mod lut204l;
#[doc = "LUT205L register accessor: an alias for `Reg<LUT205L_SPEC>`"]
pub type LUT205L = crate::Reg<lut205l::LUT205L_SPEC>;
#[doc = "Graphic MMU LUT entry 205 low"]
pub mod lut205l;
#[doc = "LUT206L register accessor: an alias for `Reg<LUT206L_SPEC>`"]
pub type LUT206L = crate::Reg<lut206l::LUT206L_SPEC>;
#[doc = "Graphic MMU LUT entry 206 low"]
pub mod lut206l;
#[doc = "LUT207L register accessor: an alias for `Reg<LUT207L_SPEC>`"]
pub type LUT207L = crate::Reg<lut207l::LUT207L_SPEC>;
#[doc = "Graphic MMU LUT entry 207 low"]
pub mod lut207l;
#[doc = "LUT208L register accessor: an alias for `Reg<LUT208L_SPEC>`"]
pub type LUT208L = crate::Reg<lut208l::LUT208L_SPEC>;
#[doc = "Graphic MMU LUT entry 208 low"]
pub mod lut208l;
#[doc = "LUT209L register accessor: an alias for `Reg<LUT209L_SPEC>`"]
pub type LUT209L = crate::Reg<lut209l::LUT209L_SPEC>;
#[doc = "Graphic MMU LUT entry 209 low"]
pub mod lut209l;
#[doc = "LUT210L register accessor: an alias for `Reg<LUT210L_SPEC>`"]
pub type LUT210L = crate::Reg<lut210l::LUT210L_SPEC>;
#[doc = "Graphic MMU LUT entry 210 low"]
pub mod lut210l;
#[doc = "LUT211L register accessor: an alias for `Reg<LUT211L_SPEC>`"]
pub type LUT211L = crate::Reg<lut211l::LUT211L_SPEC>;
#[doc = "Graphic MMU LUT entry 211 low"]
pub mod lut211l;
#[doc = "LUT212L register accessor: an alias for `Reg<LUT212L_SPEC>`"]
pub type LUT212L = crate::Reg<lut212l::LUT212L_SPEC>;
#[doc = "Graphic MMU LUT entry 212 low"]
pub mod lut212l;
#[doc = "LUT213L register accessor: an alias for `Reg<LUT213L_SPEC>`"]
pub type LUT213L = crate::Reg<lut213l::LUT213L_SPEC>;
#[doc = "Graphic MMU LUT entry 213 low"]
pub mod lut213l;
#[doc = "LUT214L register accessor: an alias for `Reg<LUT214L_SPEC>`"]
pub type LUT214L = crate::Reg<lut214l::LUT214L_SPEC>;
#[doc = "Graphic MMU LUT entry 214 low"]
pub mod lut214l;
#[doc = "LUT215L register accessor: an alias for `Reg<LUT215L_SPEC>`"]
pub type LUT215L = crate::Reg<lut215l::LUT215L_SPEC>;
#[doc = "Graphic MMU LUT entry 215 low"]
pub mod lut215l;
#[doc = "LUT216L register accessor: an alias for `Reg<LUT216L_SPEC>`"]
pub type LUT216L = crate::Reg<lut216l::LUT216L_SPEC>;
#[doc = "Graphic MMU LUT entry 216 low"]
pub mod lut216l;
#[doc = "LUT217L register accessor: an alias for `Reg<LUT217L_SPEC>`"]
pub type LUT217L = crate::Reg<lut217l::LUT217L_SPEC>;
#[doc = "Graphic MMU LUT entry 217 low"]
pub mod lut217l;
#[doc = "LUT218L register accessor: an alias for `Reg<LUT218L_SPEC>`"]
pub type LUT218L = crate::Reg<lut218l::LUT218L_SPEC>;
#[doc = "Graphic MMU LUT entry 218 low"]
pub mod lut218l;
#[doc = "LUT219L register accessor: an alias for `Reg<LUT219L_SPEC>`"]
pub type LUT219L = crate::Reg<lut219l::LUT219L_SPEC>;
#[doc = "Graphic MMU LUT entry 219 low"]
pub mod lut219l;
#[doc = "LUT220L register accessor: an alias for `Reg<LUT220L_SPEC>`"]
pub type LUT220L = crate::Reg<lut220l::LUT220L_SPEC>;
#[doc = "Graphic MMU LUT entry 220 low"]
pub mod lut220l;
#[doc = "LUT221L register accessor: an alias for `Reg<LUT221L_SPEC>`"]
pub type LUT221L = crate::Reg<lut221l::LUT221L_SPEC>;
#[doc = "Graphic MMU LUT entry 221 low"]
pub mod lut221l;
#[doc = "LUT222L register accessor: an alias for `Reg<LUT222L_SPEC>`"]
pub type LUT222L = crate::Reg<lut222l::LUT222L_SPEC>;
#[doc = "Graphic MMU LUT entry 222 low"]
pub mod lut222l;
#[doc = "LUT223L register accessor: an alias for `Reg<LUT223L_SPEC>`"]
pub type LUT223L = crate::Reg<lut223l::LUT223L_SPEC>;
#[doc = "Graphic MMU LUT entry 223 low"]
pub mod lut223l;
#[doc = "LUT224L register accessor: an alias for `Reg<LUT224L_SPEC>`"]
pub type LUT224L = crate::Reg<lut224l::LUT224L_SPEC>;
#[doc = "Graphic MMU LUT entry 224 low"]
pub mod lut224l;
#[doc = "LUT225L register accessor: an alias for `Reg<LUT225L_SPEC>`"]
pub type LUT225L = crate::Reg<lut225l::LUT225L_SPEC>;
#[doc = "Graphic MMU LUT entry 225 low"]
pub mod lut225l;
#[doc = "LUT226L register accessor: an alias for `Reg<LUT226L_SPEC>`"]
pub type LUT226L = crate::Reg<lut226l::LUT226L_SPEC>;
#[doc = "Graphic MMU LUT entry 226 low"]
pub mod lut226l;
#[doc = "LUT227L register accessor: an alias for `Reg<LUT227L_SPEC>`"]
pub type LUT227L = crate::Reg<lut227l::LUT227L_SPEC>;
#[doc = "Graphic MMU LUT entry 227 low"]
pub mod lut227l;
#[doc = "LUT228L register accessor: an alias for `Reg<LUT228L_SPEC>`"]
pub type LUT228L = crate::Reg<lut228l::LUT228L_SPEC>;
#[doc = "Graphic MMU LUT entry 228 low"]
pub mod lut228l;
#[doc = "LUT229L register accessor: an alias for `Reg<LUT229L_SPEC>`"]
pub type LUT229L = crate::Reg<lut229l::LUT229L_SPEC>;
#[doc = "Graphic MMU LUT entry 229 low"]
pub mod lut229l;
#[doc = "LUT230L register accessor: an alias for `Reg<LUT230L_SPEC>`"]
pub type LUT230L = crate::Reg<lut230l::LUT230L_SPEC>;
#[doc = "Graphic MMU LUT entry 230 low"]
pub mod lut230l;
#[doc = "LUT231L register accessor: an alias for `Reg<LUT231L_SPEC>`"]
pub type LUT231L = crate::Reg<lut231l::LUT231L_SPEC>;
#[doc = "Graphic MMU LUT entry 231 low"]
pub mod lut231l;
#[doc = "LUT232L register accessor: an alias for `Reg<LUT232L_SPEC>`"]
pub type LUT232L = crate::Reg<lut232l::LUT232L_SPEC>;
#[doc = "Graphic MMU LUT entry 232 low"]
pub mod lut232l;
#[doc = "LUT233L register accessor: an alias for `Reg<LUT233L_SPEC>`"]
pub type LUT233L = crate::Reg<lut233l::LUT233L_SPEC>;
#[doc = "Graphic MMU LUT entry 233 low"]
pub mod lut233l;
#[doc = "LUT234L register accessor: an alias for `Reg<LUT234L_SPEC>`"]
pub type LUT234L = crate::Reg<lut234l::LUT234L_SPEC>;
#[doc = "Graphic MMU LUT entry 234 low"]
pub mod lut234l;
#[doc = "LUT235L register accessor: an alias for `Reg<LUT235L_SPEC>`"]
pub type LUT235L = crate::Reg<lut235l::LUT235L_SPEC>;
#[doc = "Graphic MMU LUT entry 235 low"]
pub mod lut235l;
#[doc = "LUT236L register accessor: an alias for `Reg<LUT236L_SPEC>`"]
pub type LUT236L = crate::Reg<lut236l::LUT236L_SPEC>;
#[doc = "Graphic MMU LUT entry 236 low"]
pub mod lut236l;
#[doc = "LUT237L register accessor: an alias for `Reg<LUT237L_SPEC>`"]
pub type LUT237L = crate::Reg<lut237l::LUT237L_SPEC>;
#[doc = "Graphic MMU LUT entry 237 low"]
pub mod lut237l;
#[doc = "LUT238L register accessor: an alias for `Reg<LUT238L_SPEC>`"]
pub type LUT238L = crate::Reg<lut238l::LUT238L_SPEC>;
#[doc = "Graphic MMU LUT entry 238 low"]
pub mod lut238l;
#[doc = "LUT239L register accessor: an alias for `Reg<LUT239L_SPEC>`"]
pub type LUT239L = crate::Reg<lut239l::LUT239L_SPEC>;
#[doc = "Graphic MMU LUT entry 239 low"]
pub mod lut239l;
#[doc = "LUT240L register accessor: an alias for `Reg<LUT240L_SPEC>`"]
pub type LUT240L = crate::Reg<lut240l::LUT240L_SPEC>;
#[doc = "Graphic MMU LUT entry 240 low"]
pub mod lut240l;
#[doc = "LUT241L register accessor: an alias for `Reg<LUT241L_SPEC>`"]
pub type LUT241L = crate::Reg<lut241l::LUT241L_SPEC>;
#[doc = "Graphic MMU LUT entry 241 low"]
pub mod lut241l;
#[doc = "LUT242L register accessor: an alias for `Reg<LUT242L_SPEC>`"]
pub type LUT242L = crate::Reg<lut242l::LUT242L_SPEC>;
#[doc = "Graphic MMU LUT entry 242 low"]
pub mod lut242l;
#[doc = "LUT243L register accessor: an alias for `Reg<LUT243L_SPEC>`"]
pub type LUT243L = crate::Reg<lut243l::LUT243L_SPEC>;
#[doc = "Graphic MMU LUT entry 243 low"]
pub mod lut243l;
#[doc = "LUT244L register accessor: an alias for `Reg<LUT244L_SPEC>`"]
pub type LUT244L = crate::Reg<lut244l::LUT244L_SPEC>;
#[doc = "Graphic MMU LUT entry 244 low"]
pub mod lut244l;
#[doc = "LUT245L register accessor: an alias for `Reg<LUT245L_SPEC>`"]
pub type LUT245L = crate::Reg<lut245l::LUT245L_SPEC>;
#[doc = "Graphic MMU LUT entry 245 low"]
pub mod lut245l;
#[doc = "LUT246L register accessor: an alias for `Reg<LUT246L_SPEC>`"]
pub type LUT246L = crate::Reg<lut246l::LUT246L_SPEC>;
#[doc = "Graphic MMU LUT entry 246 low"]
pub mod lut246l;
#[doc = "LUT247L register accessor: an alias for `Reg<LUT247L_SPEC>`"]
pub type LUT247L = crate::Reg<lut247l::LUT247L_SPEC>;
#[doc = "Graphic MMU LUT entry 247 low"]
pub mod lut247l;
#[doc = "LUT248L register accessor: an alias for `Reg<LUT248L_SPEC>`"]
pub type LUT248L = crate::Reg<lut248l::LUT248L_SPEC>;
#[doc = "Graphic MMU LUT entry 248 low"]
pub mod lut248l;
#[doc = "LUT249L register accessor: an alias for `Reg<LUT249L_SPEC>`"]
pub type LUT249L = crate::Reg<lut249l::LUT249L_SPEC>;
#[doc = "Graphic MMU LUT entry 249 low"]
pub mod lut249l;
#[doc = "LUT250L register accessor: an alias for `Reg<LUT250L_SPEC>`"]
pub type LUT250L = crate::Reg<lut250l::LUT250L_SPEC>;
#[doc = "Graphic MMU LUT entry 250 low"]
pub mod lut250l;
#[doc = "LUT251L register accessor: an alias for `Reg<LUT251L_SPEC>`"]
pub type LUT251L = crate::Reg<lut251l::LUT251L_SPEC>;
#[doc = "Graphic MMU LUT entry 251 low"]
pub mod lut251l;
#[doc = "LUT252L register accessor: an alias for `Reg<LUT252L_SPEC>`"]
pub type LUT252L = crate::Reg<lut252l::LUT252L_SPEC>;
#[doc = "Graphic MMU LUT entry 252 low"]
pub mod lut252l;
#[doc = "LUT253L register accessor: an alias for `Reg<LUT253L_SPEC>`"]
pub type LUT253L = crate::Reg<lut253l::LUT253L_SPEC>;
#[doc = "Graphic MMU LUT entry 253 low"]
pub mod lut253l;
#[doc = "LUT254L register accessor: an alias for `Reg<LUT254L_SPEC>`"]
pub type LUT254L = crate::Reg<lut254l::LUT254L_SPEC>;
#[doc = "Graphic MMU LUT entry 254 low"]
pub mod lut254l;
#[doc = "LUT255L register accessor: an alias for `Reg<LUT255L_SPEC>`"]
pub type LUT255L = crate::Reg<lut255l::LUT255L_SPEC>;
#[doc = "Graphic MMU LUT entry 255 low"]
pub mod lut255l;
#[doc = "LUT256L register accessor: an alias for `Reg<LUT256L_SPEC>`"]
pub type LUT256L = crate::Reg<lut256l::LUT256L_SPEC>;
#[doc = "Graphic MMU LUT entry 256 low"]
pub mod lut256l;
#[doc = "LUT257L register accessor: an alias for `Reg<LUT257L_SPEC>`"]
pub type LUT257L = crate::Reg<lut257l::LUT257L_SPEC>;
#[doc = "Graphic MMU LUT entry 257 low"]
pub mod lut257l;
#[doc = "LUT258L register accessor: an alias for `Reg<LUT258L_SPEC>`"]
pub type LUT258L = crate::Reg<lut258l::LUT258L_SPEC>;
#[doc = "Graphic MMU LUT entry 258 low"]
pub mod lut258l;
#[doc = "LUT259L register accessor: an alias for `Reg<LUT259L_SPEC>`"]
pub type LUT259L = crate::Reg<lut259l::LUT259L_SPEC>;
#[doc = "Graphic MMU LUT entry 259 low"]
pub mod lut259l;
#[doc = "LUT260L register accessor: an alias for `Reg<LUT260L_SPEC>`"]
pub type LUT260L = crate::Reg<lut260l::LUT260L_SPEC>;
#[doc = "Graphic MMU LUT entry 260 low"]
pub mod lut260l;
#[doc = "LUT261L register accessor: an alias for `Reg<LUT261L_SPEC>`"]
pub type LUT261L = crate::Reg<lut261l::LUT261L_SPEC>;
#[doc = "Graphic MMU LUT entry 261 low"]
pub mod lut261l;
#[doc = "LUT262L register accessor: an alias for `Reg<LUT262L_SPEC>`"]
pub type LUT262L = crate::Reg<lut262l::LUT262L_SPEC>;
#[doc = "Graphic MMU LUT entry 262 low"]
pub mod lut262l;
#[doc = "LUT263L register accessor: an alias for `Reg<LUT263L_SPEC>`"]
pub type LUT263L = crate::Reg<lut263l::LUT263L_SPEC>;
#[doc = "Graphic MMU LUT entry 263 low"]
pub mod lut263l;
#[doc = "LUT264L register accessor: an alias for `Reg<LUT264L_SPEC>`"]
pub type LUT264L = crate::Reg<lut264l::LUT264L_SPEC>;
#[doc = "Graphic MMU LUT entry 264 low"]
pub mod lut264l;
#[doc = "LUT265L register accessor: an alias for `Reg<LUT265L_SPEC>`"]
pub type LUT265L = crate::Reg<lut265l::LUT265L_SPEC>;
#[doc = "Graphic MMU LUT entry 265 low"]
pub mod lut265l;
#[doc = "LUT266L register accessor: an alias for `Reg<LUT266L_SPEC>`"]
pub type LUT266L = crate::Reg<lut266l::LUT266L_SPEC>;
#[doc = "Graphic MMU LUT entry 266 low"]
pub mod lut266l;
#[doc = "LUT267L register accessor: an alias for `Reg<LUT267L_SPEC>`"]
pub type LUT267L = crate::Reg<lut267l::LUT267L_SPEC>;
#[doc = "Graphic MMU LUT entry 267 low"]
pub mod lut267l;
#[doc = "LUT268L register accessor: an alias for `Reg<LUT268L_SPEC>`"]
pub type LUT268L = crate::Reg<lut268l::LUT268L_SPEC>;
#[doc = "Graphic MMU LUT entry 268 low"]
pub mod lut268l;
#[doc = "LUT269L register accessor: an alias for `Reg<LUT269L_SPEC>`"]
pub type LUT269L = crate::Reg<lut269l::LUT269L_SPEC>;
#[doc = "Graphic MMU LUT entry 269 low"]
pub mod lut269l;
#[doc = "LUT270L register accessor: an alias for `Reg<LUT270L_SPEC>`"]
pub type LUT270L = crate::Reg<lut270l::LUT270L_SPEC>;
#[doc = "Graphic MMU LUT entry 270 low"]
pub mod lut270l;
#[doc = "LUT271L register accessor: an alias for `Reg<LUT271L_SPEC>`"]
pub type LUT271L = crate::Reg<lut271l::LUT271L_SPEC>;
#[doc = "Graphic MMU LUT entry 271 low"]
pub mod lut271l;
#[doc = "LUT272L register accessor: an alias for `Reg<LUT272L_SPEC>`"]
pub type LUT272L = crate::Reg<lut272l::LUT272L_SPEC>;
#[doc = "Graphic MMU LUT entry 272 low"]
pub mod lut272l;
#[doc = "LUT273L register accessor: an alias for `Reg<LUT273L_SPEC>`"]
pub type LUT273L = crate::Reg<lut273l::LUT273L_SPEC>;
#[doc = "Graphic MMU LUT entry 273 low"]
pub mod lut273l;
#[doc = "LUT274L register accessor: an alias for `Reg<LUT274L_SPEC>`"]
pub type LUT274L = crate::Reg<lut274l::LUT274L_SPEC>;
#[doc = "Graphic MMU LUT entry 274 low"]
pub mod lut274l;
#[doc = "LUT275L register accessor: an alias for `Reg<LUT275L_SPEC>`"]
pub type LUT275L = crate::Reg<lut275l::LUT275L_SPEC>;
#[doc = "Graphic MMU LUT entry 275 low"]
pub mod lut275l;
#[doc = "LUT276L register accessor: an alias for `Reg<LUT276L_SPEC>`"]
pub type LUT276L = crate::Reg<lut276l::LUT276L_SPEC>;
#[doc = "Graphic MMU LUT entry 276 low"]
pub mod lut276l;
#[doc = "LUT277L register accessor: an alias for `Reg<LUT277L_SPEC>`"]
pub type LUT277L = crate::Reg<lut277l::LUT277L_SPEC>;
#[doc = "Graphic MMU LUT entry 277 low"]
pub mod lut277l;
#[doc = "LUT278L register accessor: an alias for `Reg<LUT278L_SPEC>`"]
pub type LUT278L = crate::Reg<lut278l::LUT278L_SPEC>;
#[doc = "Graphic MMU LUT entry 278 low"]
pub mod lut278l;
#[doc = "LUT279L register accessor: an alias for `Reg<LUT279L_SPEC>`"]
pub type LUT279L = crate::Reg<lut279l::LUT279L_SPEC>;
#[doc = "Graphic MMU LUT entry 279 low"]
pub mod lut279l;
#[doc = "LUT280L register accessor: an alias for `Reg<LUT280L_SPEC>`"]
pub type LUT280L = crate::Reg<lut280l::LUT280L_SPEC>;
#[doc = "Graphic MMU LUT entry 280 low"]
pub mod lut280l;
#[doc = "LUT281L register accessor: an alias for `Reg<LUT281L_SPEC>`"]
pub type LUT281L = crate::Reg<lut281l::LUT281L_SPEC>;
#[doc = "Graphic MMU LUT entry 281 low"]
pub mod lut281l;
#[doc = "LUT282L register accessor: an alias for `Reg<LUT282L_SPEC>`"]
pub type LUT282L = crate::Reg<lut282l::LUT282L_SPEC>;
#[doc = "Graphic MMU LUT entry 282 low"]
pub mod lut282l;
#[doc = "LUT283L register accessor: an alias for `Reg<LUT283L_SPEC>`"]
pub type LUT283L = crate::Reg<lut283l::LUT283L_SPEC>;
#[doc = "Graphic MMU LUT entry 283 low"]
pub mod lut283l;
#[doc = "LUT284L register accessor: an alias for `Reg<LUT284L_SPEC>`"]
pub type LUT284L = crate::Reg<lut284l::LUT284L_SPEC>;
#[doc = "Graphic MMU LUT entry 284 low"]
pub mod lut284l;
#[doc = "LUT285L register accessor: an alias for `Reg<LUT285L_SPEC>`"]
pub type LUT285L = crate::Reg<lut285l::LUT285L_SPEC>;
#[doc = "Graphic MMU LUT entry 285 low"]
pub mod lut285l;
#[doc = "LUT286L register accessor: an alias for `Reg<LUT286L_SPEC>`"]
pub type LUT286L = crate::Reg<lut286l::LUT286L_SPEC>;
#[doc = "Graphic MMU LUT entry 286 low"]
pub mod lut286l;
#[doc = "LUT287L register accessor: an alias for `Reg<LUT287L_SPEC>`"]
pub type LUT287L = crate::Reg<lut287l::LUT287L_SPEC>;
#[doc = "Graphic MMU LUT entry 287 low"]
pub mod lut287l;
#[doc = "LUT288L register accessor: an alias for `Reg<LUT288L_SPEC>`"]
pub type LUT288L = crate::Reg<lut288l::LUT288L_SPEC>;
#[doc = "Graphic MMU LUT entry 288 low"]
pub mod lut288l;
#[doc = "LUT289L register accessor: an alias for `Reg<LUT289L_SPEC>`"]
pub type LUT289L = crate::Reg<lut289l::LUT289L_SPEC>;
#[doc = "Graphic MMU LUT entry 289 low"]
pub mod lut289l;
#[doc = "LUT290L register accessor: an alias for `Reg<LUT290L_SPEC>`"]
pub type LUT290L = crate::Reg<lut290l::LUT290L_SPEC>;
#[doc = "Graphic MMU LUT entry 290 low"]
pub mod lut290l;
#[doc = "LUT291L register accessor: an alias for `Reg<LUT291L_SPEC>`"]
pub type LUT291L = crate::Reg<lut291l::LUT291L_SPEC>;
#[doc = "Graphic MMU LUT entry 291 low"]
pub mod lut291l;
#[doc = "LUT292L register accessor: an alias for `Reg<LUT292L_SPEC>`"]
pub type LUT292L = crate::Reg<lut292l::LUT292L_SPEC>;
#[doc = "Graphic MMU LUT entry 292 low"]
pub mod lut292l;
#[doc = "LUT293L register accessor: an alias for `Reg<LUT293L_SPEC>`"]
pub type LUT293L = crate::Reg<lut293l::LUT293L_SPEC>;
#[doc = "Graphic MMU LUT entry 293 low"]
pub mod lut293l;
#[doc = "LUT294L register accessor: an alias for `Reg<LUT294L_SPEC>`"]
pub type LUT294L = crate::Reg<lut294l::LUT294L_SPEC>;
#[doc = "Graphic MMU LUT entry 294 low"]
pub mod lut294l;
#[doc = "LUT295L register accessor: an alias for `Reg<LUT295L_SPEC>`"]
pub type LUT295L = crate::Reg<lut295l::LUT295L_SPEC>;
#[doc = "Graphic MMU LUT entry 295 low"]
pub mod lut295l;
#[doc = "LUT296L register accessor: an alias for `Reg<LUT296L_SPEC>`"]
pub type LUT296L = crate::Reg<lut296l::LUT296L_SPEC>;
#[doc = "Graphic MMU LUT entry 296 low"]
pub mod lut296l;
#[doc = "LUT297L register accessor: an alias for `Reg<LUT297L_SPEC>`"]
pub type LUT297L = crate::Reg<lut297l::LUT297L_SPEC>;
#[doc = "Graphic MMU LUT entry 297 low"]
pub mod lut297l;
#[doc = "LUT298L register accessor: an alias for `Reg<LUT298L_SPEC>`"]
pub type LUT298L = crate::Reg<lut298l::LUT298L_SPEC>;
#[doc = "Graphic MMU LUT entry 298 low"]
pub mod lut298l;
#[doc = "LUT299L register accessor: an alias for `Reg<LUT299L_SPEC>`"]
pub type LUT299L = crate::Reg<lut299l::LUT299L_SPEC>;
#[doc = "Graphic MMU LUT entry 299 low"]
pub mod lut299l;
#[doc = "LUT300L register accessor: an alias for `Reg<LUT300L_SPEC>`"]
pub type LUT300L = crate::Reg<lut300l::LUT300L_SPEC>;
#[doc = "Graphic MMU LUT entry 300 low"]
pub mod lut300l;
#[doc = "LUT301L register accessor: an alias for `Reg<LUT301L_SPEC>`"]
pub type LUT301L = crate::Reg<lut301l::LUT301L_SPEC>;
#[doc = "Graphic MMU LUT entry 301 low"]
pub mod lut301l;
#[doc = "LUT302L register accessor: an alias for `Reg<LUT302L_SPEC>`"]
pub type LUT302L = crate::Reg<lut302l::LUT302L_SPEC>;
#[doc = "Graphic MMU LUT entry 302 low"]
pub mod lut302l;
#[doc = "LUT303L register accessor: an alias for `Reg<LUT303L_SPEC>`"]
pub type LUT303L = crate::Reg<lut303l::LUT303L_SPEC>;
#[doc = "Graphic MMU LUT entry 303 low"]
pub mod lut303l;
#[doc = "LUT304L register accessor: an alias for `Reg<LUT304L_SPEC>`"]
pub type LUT304L = crate::Reg<lut304l::LUT304L_SPEC>;
#[doc = "Graphic MMU LUT entry 304 low"]
pub mod lut304l;
#[doc = "LUT305L register accessor: an alias for `Reg<LUT305L_SPEC>`"]
pub type LUT305L = crate::Reg<lut305l::LUT305L_SPEC>;
#[doc = "Graphic MMU LUT entry 305 low"]
pub mod lut305l;
#[doc = "LUT306L register accessor: an alias for `Reg<LUT306L_SPEC>`"]
pub type LUT306L = crate::Reg<lut306l::LUT306L_SPEC>;
#[doc = "Graphic MMU LUT entry 306 low"]
pub mod lut306l;
#[doc = "LUT307L register accessor: an alias for `Reg<LUT307L_SPEC>`"]
pub type LUT307L = crate::Reg<lut307l::LUT307L_SPEC>;
#[doc = "Graphic MMU LUT entry 307 low"]
pub mod lut307l;
#[doc = "LUT308L register accessor: an alias for `Reg<LUT308L_SPEC>`"]
pub type LUT308L = crate::Reg<lut308l::LUT308L_SPEC>;
#[doc = "Graphic MMU LUT entry 308 low"]
pub mod lut308l;
#[doc = "LUT309L register accessor: an alias for `Reg<LUT309L_SPEC>`"]
pub type LUT309L = crate::Reg<lut309l::LUT309L_SPEC>;
#[doc = "Graphic MMU LUT entry 309 low"]
pub mod lut309l;
#[doc = "LUT310L register accessor: an alias for `Reg<LUT310L_SPEC>`"]
pub type LUT310L = crate::Reg<lut310l::LUT310L_SPEC>;
#[doc = "Graphic MMU LUT entry 310 low"]
pub mod lut310l;
#[doc = "LUT311L register accessor: an alias for `Reg<LUT311L_SPEC>`"]
pub type LUT311L = crate::Reg<lut311l::LUT311L_SPEC>;
#[doc = "Graphic MMU LUT entry 311 low"]
pub mod lut311l;
#[doc = "LUT312L register accessor: an alias for `Reg<LUT312L_SPEC>`"]
pub type LUT312L = crate::Reg<lut312l::LUT312L_SPEC>;
#[doc = "Graphic MMU LUT entry 312 low"]
pub mod lut312l;
#[doc = "LUT313L register accessor: an alias for `Reg<LUT313L_SPEC>`"]
pub type LUT313L = crate::Reg<lut313l::LUT313L_SPEC>;
#[doc = "Graphic MMU LUT entry 313 low"]
pub mod lut313l;
#[doc = "LUT314L register accessor: an alias for `Reg<LUT314L_SPEC>`"]
pub type LUT314L = crate::Reg<lut314l::LUT314L_SPEC>;
#[doc = "Graphic MMU LUT entry 314 low"]
pub mod lut314l;
#[doc = "LUT315L register accessor: an alias for `Reg<LUT315L_SPEC>`"]
pub type LUT315L = crate::Reg<lut315l::LUT315L_SPEC>;
#[doc = "Graphic MMU LUT entry 315 low"]
pub mod lut315l;
#[doc = "LUT316L register accessor: an alias for `Reg<LUT316L_SPEC>`"]
pub type LUT316L = crate::Reg<lut316l::LUT316L_SPEC>;
#[doc = "Graphic MMU LUT entry 316 low"]
pub mod lut316l;
#[doc = "LUT317L register accessor: an alias for `Reg<LUT317L_SPEC>`"]
pub type LUT317L = crate::Reg<lut317l::LUT317L_SPEC>;
#[doc = "Graphic MMU LUT entry 317 low"]
pub mod lut317l;
#[doc = "LUT318L register accessor: an alias for `Reg<LUT318L_SPEC>`"]
pub type LUT318L = crate::Reg<lut318l::LUT318L_SPEC>;
#[doc = "Graphic MMU LUT entry 318 low"]
pub mod lut318l;
#[doc = "LUT319L register accessor: an alias for `Reg<LUT319L_SPEC>`"]
pub type LUT319L = crate::Reg<lut319l::LUT319L_SPEC>;
#[doc = "Graphic MMU LUT entry 319 low"]
pub mod lut319l;
#[doc = "LUT320L register accessor: an alias for `Reg<LUT320L_SPEC>`"]
pub type LUT320L = crate::Reg<lut320l::LUT320L_SPEC>;
#[doc = "Graphic MMU LUT entry 320 low"]
pub mod lut320l;
#[doc = "LUT321L register accessor: an alias for `Reg<LUT321L_SPEC>`"]
pub type LUT321L = crate::Reg<lut321l::LUT321L_SPEC>;
#[doc = "Graphic MMU LUT entry 321 low"]
pub mod lut321l;
#[doc = "LUT322L register accessor: an alias for `Reg<LUT322L_SPEC>`"]
pub type LUT322L = crate::Reg<lut322l::LUT322L_SPEC>;
#[doc = "Graphic MMU LUT entry 322 low"]
pub mod lut322l;
#[doc = "LUT323L register accessor: an alias for `Reg<LUT323L_SPEC>`"]
pub type LUT323L = crate::Reg<lut323l::LUT323L_SPEC>;
#[doc = "Graphic MMU LUT entry 323 low"]
pub mod lut323l;
#[doc = "LUT324L register accessor: an alias for `Reg<LUT324L_SPEC>`"]
pub type LUT324L = crate::Reg<lut324l::LUT324L_SPEC>;
#[doc = "Graphic MMU LUT entry 324 low"]
pub mod lut324l;
#[doc = "LUT325L register accessor: an alias for `Reg<LUT325L_SPEC>`"]
pub type LUT325L = crate::Reg<lut325l::LUT325L_SPEC>;
#[doc = "Graphic MMU LUT entry 325 low"]
pub mod lut325l;
#[doc = "LUT326L register accessor: an alias for `Reg<LUT326L_SPEC>`"]
pub type LUT326L = crate::Reg<lut326l::LUT326L_SPEC>;
#[doc = "Graphic MMU LUT entry 326 low"]
pub mod lut326l;
#[doc = "LUT327L register accessor: an alias for `Reg<LUT327L_SPEC>`"]
pub type LUT327L = crate::Reg<lut327l::LUT327L_SPEC>;
#[doc = "Graphic MMU LUT entry 327 low"]
pub mod lut327l;
#[doc = "LUT328L register accessor: an alias for `Reg<LUT328L_SPEC>`"]
pub type LUT328L = crate::Reg<lut328l::LUT328L_SPEC>;
#[doc = "Graphic MMU LUT entry 328 low"]
pub mod lut328l;
#[doc = "LUT329L register accessor: an alias for `Reg<LUT329L_SPEC>`"]
pub type LUT329L = crate::Reg<lut329l::LUT329L_SPEC>;
#[doc = "Graphic MMU LUT entry 329 low"]
pub mod lut329l;
#[doc = "LUT330L register accessor: an alias for `Reg<LUT330L_SPEC>`"]
pub type LUT330L = crate::Reg<lut330l::LUT330L_SPEC>;
#[doc = "Graphic MMU LUT entry 330 low"]
pub mod lut330l;
#[doc = "LUT331L register accessor: an alias for `Reg<LUT331L_SPEC>`"]
pub type LUT331L = crate::Reg<lut331l::LUT331L_SPEC>;
#[doc = "Graphic MMU LUT entry 331 low"]
pub mod lut331l;
#[doc = "LUT332L register accessor: an alias for `Reg<LUT332L_SPEC>`"]
pub type LUT332L = crate::Reg<lut332l::LUT332L_SPEC>;
#[doc = "Graphic MMU LUT entry 332 low"]
pub mod lut332l;
#[doc = "LUT333L register accessor: an alias for `Reg<LUT333L_SPEC>`"]
pub type LUT333L = crate::Reg<lut333l::LUT333L_SPEC>;
#[doc = "Graphic MMU LUT entry 333 low"]
pub mod lut333l;
#[doc = "LUT334L register accessor: an alias for `Reg<LUT334L_SPEC>`"]
pub type LUT334L = crate::Reg<lut334l::LUT334L_SPEC>;
#[doc = "Graphic MMU LUT entry 334 low"]
pub mod lut334l;
#[doc = "LUT335L register accessor: an alias for `Reg<LUT335L_SPEC>`"]
pub type LUT335L = crate::Reg<lut335l::LUT335L_SPEC>;
#[doc = "Graphic MMU LUT entry 335 low"]
pub mod lut335l;
#[doc = "LUT336L register accessor: an alias for `Reg<LUT336L_SPEC>`"]
pub type LUT336L = crate::Reg<lut336l::LUT336L_SPEC>;
#[doc = "Graphic MMU LUT entry 336 low"]
pub mod lut336l;
#[doc = "LUT337L register accessor: an alias for `Reg<LUT337L_SPEC>`"]
pub type LUT337L = crate::Reg<lut337l::LUT337L_SPEC>;
#[doc = "Graphic MMU LUT entry 337 low"]
pub mod lut337l;
#[doc = "LUT338L register accessor: an alias for `Reg<LUT338L_SPEC>`"]
pub type LUT338L = crate::Reg<lut338l::LUT338L_SPEC>;
#[doc = "Graphic MMU LUT entry 338 low"]
pub mod lut338l;
#[doc = "LUT339L register accessor: an alias for `Reg<LUT339L_SPEC>`"]
pub type LUT339L = crate::Reg<lut339l::LUT339L_SPEC>;
#[doc = "Graphic MMU LUT entry 339 low"]
pub mod lut339l;
#[doc = "LUT340L register accessor: an alias for `Reg<LUT340L_SPEC>`"]
pub type LUT340L = crate::Reg<lut340l::LUT340L_SPEC>;
#[doc = "Graphic MMU LUT entry 340 low"]
pub mod lut340l;
#[doc = "LUT341L register accessor: an alias for `Reg<LUT341L_SPEC>`"]
pub type LUT341L = crate::Reg<lut341l::LUT341L_SPEC>;
#[doc = "Graphic MMU LUT entry 341 low"]
pub mod lut341l;
#[doc = "LUT342L register accessor: an alias for `Reg<LUT342L_SPEC>`"]
pub type LUT342L = crate::Reg<lut342l::LUT342L_SPEC>;
#[doc = "Graphic MMU LUT entry 342 low"]
pub mod lut342l;
#[doc = "LUT343L register accessor: an alias for `Reg<LUT343L_SPEC>`"]
pub type LUT343L = crate::Reg<lut343l::LUT343L_SPEC>;
#[doc = "Graphic MMU LUT entry 343 low"]
pub mod lut343l;
#[doc = "LUT344L register accessor: an alias for `Reg<LUT344L_SPEC>`"]
pub type LUT344L = crate::Reg<lut344l::LUT344L_SPEC>;
#[doc = "Graphic MMU LUT entry 344 low"]
pub mod lut344l;
#[doc = "LUT345L register accessor: an alias for `Reg<LUT345L_SPEC>`"]
pub type LUT345L = crate::Reg<lut345l::LUT345L_SPEC>;
#[doc = "Graphic MMU LUT entry 345 low"]
pub mod lut345l;
#[doc = "LUT346L register accessor: an alias for `Reg<LUT346L_SPEC>`"]
pub type LUT346L = crate::Reg<lut346l::LUT346L_SPEC>;
#[doc = "Graphic MMU LUT entry 346 low"]
pub mod lut346l;
#[doc = "LUT347L register accessor: an alias for `Reg<LUT347L_SPEC>`"]
pub type LUT347L = crate::Reg<lut347l::LUT347L_SPEC>;
#[doc = "Graphic MMU LUT entry 347 low"]
pub mod lut347l;
#[doc = "LUT348L register accessor: an alias for `Reg<LUT348L_SPEC>`"]
pub type LUT348L = crate::Reg<lut348l::LUT348L_SPEC>;
#[doc = "Graphic MMU LUT entry 348 low"]
pub mod lut348l;
#[doc = "LUT349L register accessor: an alias for `Reg<LUT349L_SPEC>`"]
pub type LUT349L = crate::Reg<lut349l::LUT349L_SPEC>;
#[doc = "Graphic MMU LUT entry 349 low"]
pub mod lut349l;
#[doc = "LUT350L register accessor: an alias for `Reg<LUT350L_SPEC>`"]
pub type LUT350L = crate::Reg<lut350l::LUT350L_SPEC>;
#[doc = "Graphic MMU LUT entry 350 low"]
pub mod lut350l;
#[doc = "LUT351L register accessor: an alias for `Reg<LUT351L_SPEC>`"]
pub type LUT351L = crate::Reg<lut351l::LUT351L_SPEC>;
#[doc = "Graphic MMU LUT entry 351 low"]
pub mod lut351l;
#[doc = "LUT352L register accessor: an alias for `Reg<LUT352L_SPEC>`"]
pub type LUT352L = crate::Reg<lut352l::LUT352L_SPEC>;
#[doc = "Graphic MMU LUT entry 352 low"]
pub mod lut352l;
#[doc = "LUT353L register accessor: an alias for `Reg<LUT353L_SPEC>`"]
pub type LUT353L = crate::Reg<lut353l::LUT353L_SPEC>;
#[doc = "Graphic MMU LUT entry 353 low"]
pub mod lut353l;
#[doc = "LUT354L register accessor: an alias for `Reg<LUT354L_SPEC>`"]
pub type LUT354L = crate::Reg<lut354l::LUT354L_SPEC>;
#[doc = "Graphic MMU LUT entry 354 low"]
pub mod lut354l;
#[doc = "LUT355L register accessor: an alias for `Reg<LUT355L_SPEC>`"]
pub type LUT355L = crate::Reg<lut355l::LUT355L_SPEC>;
#[doc = "Graphic MMU LUT entry 355 low"]
pub mod lut355l;
#[doc = "LUT356L register accessor: an alias for `Reg<LUT356L_SPEC>`"]
pub type LUT356L = crate::Reg<lut356l::LUT356L_SPEC>;
#[doc = "Graphic MMU LUT entry 356 low"]
pub mod lut356l;
#[doc = "LUT357L register accessor: an alias for `Reg<LUT357L_SPEC>`"]
pub type LUT357L = crate::Reg<lut357l::LUT357L_SPEC>;
#[doc = "Graphic MMU LUT entry 357 low"]
pub mod lut357l;
#[doc = "LUT358L register accessor: an alias for `Reg<LUT358L_SPEC>`"]
pub type LUT358L = crate::Reg<lut358l::LUT358L_SPEC>;
#[doc = "Graphic MMU LUT entry 358 low"]
pub mod lut358l;
#[doc = "LUT359L register accessor: an alias for `Reg<LUT359L_SPEC>`"]
pub type LUT359L = crate::Reg<lut359l::LUT359L_SPEC>;
#[doc = "Graphic MMU LUT entry 359 low"]
pub mod lut359l;
#[doc = "LUT360L register accessor: an alias for `Reg<LUT360L_SPEC>`"]
pub type LUT360L = crate::Reg<lut360l::LUT360L_SPEC>;
#[doc = "Graphic MMU LUT entry 360 low"]
pub mod lut360l;
#[doc = "LUT361L register accessor: an alias for `Reg<LUT361L_SPEC>`"]
pub type LUT361L = crate::Reg<lut361l::LUT361L_SPEC>;
#[doc = "Graphic MMU LUT entry 361 low"]
pub mod lut361l;
#[doc = "LUT362L register accessor: an alias for `Reg<LUT362L_SPEC>`"]
pub type LUT362L = crate::Reg<lut362l::LUT362L_SPEC>;
#[doc = "Graphic MMU LUT entry 362 low"]
pub mod lut362l;
#[doc = "LUT363L register accessor: an alias for `Reg<LUT363L_SPEC>`"]
pub type LUT363L = crate::Reg<lut363l::LUT363L_SPEC>;
#[doc = "Graphic MMU LUT entry 363 low"]
pub mod lut363l;
#[doc = "LUT364L register accessor: an alias for `Reg<LUT364L_SPEC>`"]
pub type LUT364L = crate::Reg<lut364l::LUT364L_SPEC>;
#[doc = "Graphic MMU LUT entry 364 low"]
pub mod lut364l;
#[doc = "LUT365L register accessor: an alias for `Reg<LUT365L_SPEC>`"]
pub type LUT365L = crate::Reg<lut365l::LUT365L_SPEC>;
#[doc = "Graphic MMU LUT entry 365 low"]
pub mod lut365l;
#[doc = "LUT366L register accessor: an alias for `Reg<LUT366L_SPEC>`"]
pub type LUT366L = crate::Reg<lut366l::LUT366L_SPEC>;
#[doc = "Graphic MMU LUT entry 366 low"]
pub mod lut366l;
#[doc = "LUT367L register accessor: an alias for `Reg<LUT367L_SPEC>`"]
pub type LUT367L = crate::Reg<lut367l::LUT367L_SPEC>;
#[doc = "Graphic MMU LUT entry 367 low"]
pub mod lut367l;
#[doc = "LUT368L register accessor: an alias for `Reg<LUT368L_SPEC>`"]
pub type LUT368L = crate::Reg<lut368l::LUT368L_SPEC>;
#[doc = "Graphic MMU LUT entry 368 low"]
pub mod lut368l;
#[doc = "LUT369L register accessor: an alias for `Reg<LUT369L_SPEC>`"]
pub type LUT369L = crate::Reg<lut369l::LUT369L_SPEC>;
#[doc = "Graphic MMU LUT entry 369 low"]
pub mod lut369l;
#[doc = "LUT370L register accessor: an alias for `Reg<LUT370L_SPEC>`"]
pub type LUT370L = crate::Reg<lut370l::LUT370L_SPEC>;
#[doc = "Graphic MMU LUT entry 370 low"]
pub mod lut370l;
#[doc = "LUT371L register accessor: an alias for `Reg<LUT371L_SPEC>`"]
pub type LUT371L = crate::Reg<lut371l::LUT371L_SPEC>;
#[doc = "Graphic MMU LUT entry 371 low"]
pub mod lut371l;
#[doc = "LUT372L register accessor: an alias for `Reg<LUT372L_SPEC>`"]
pub type LUT372L = crate::Reg<lut372l::LUT372L_SPEC>;
#[doc = "Graphic MMU LUT entry 372 low"]
pub mod lut372l;
#[doc = "LUT373L register accessor: an alias for `Reg<LUT373L_SPEC>`"]
pub type LUT373L = crate::Reg<lut373l::LUT373L_SPEC>;
#[doc = "Graphic MMU LUT entry 373 low"]
pub mod lut373l;
#[doc = "LUT374L register accessor: an alias for `Reg<LUT374L_SPEC>`"]
pub type LUT374L = crate::Reg<lut374l::LUT374L_SPEC>;
#[doc = "Graphic MMU LUT entry 374 low"]
pub mod lut374l;
#[doc = "LUT375L register accessor: an alias for `Reg<LUT375L_SPEC>`"]
pub type LUT375L = crate::Reg<lut375l::LUT375L_SPEC>;
#[doc = "Graphic MMU LUT entry 375 low"]
pub mod lut375l;
#[doc = "LUT376L register accessor: an alias for `Reg<LUT376L_SPEC>`"]
pub type LUT376L = crate::Reg<lut376l::LUT376L_SPEC>;
#[doc = "Graphic MMU LUT entry 376 low"]
pub mod lut376l;
#[doc = "LUT377L register accessor: an alias for `Reg<LUT377L_SPEC>`"]
pub type LUT377L = crate::Reg<lut377l::LUT377L_SPEC>;
#[doc = "Graphic MMU LUT entry 377 low"]
pub mod lut377l;
#[doc = "LUT378L register accessor: an alias for `Reg<LUT378L_SPEC>`"]
pub type LUT378L = crate::Reg<lut378l::LUT378L_SPEC>;
#[doc = "Graphic MMU LUT entry 378 low"]
pub mod lut378l;
#[doc = "LUT379L register accessor: an alias for `Reg<LUT379L_SPEC>`"]
pub type LUT379L = crate::Reg<lut379l::LUT379L_SPEC>;
#[doc = "Graphic MMU LUT entry 379 low"]
pub mod lut379l;
#[doc = "LUT380L register accessor: an alias for `Reg<LUT380L_SPEC>`"]
pub type LUT380L = crate::Reg<lut380l::LUT380L_SPEC>;
#[doc = "Graphic MMU LUT entry 380 low"]
pub mod lut380l;
#[doc = "LUT381L register accessor: an alias for `Reg<LUT381L_SPEC>`"]
pub type LUT381L = crate::Reg<lut381l::LUT381L_SPEC>;
#[doc = "Graphic MMU LUT entry 381 low"]
pub mod lut381l;
#[doc = "LUT382L register accessor: an alias for `Reg<LUT382L_SPEC>`"]
pub type LUT382L = crate::Reg<lut382l::LUT382L_SPEC>;
#[doc = "Graphic MMU LUT entry 382 low"]
pub mod lut382l;
#[doc = "LUT383L register accessor: an alias for `Reg<LUT383L_SPEC>`"]
pub type LUT383L = crate::Reg<lut383l::LUT383L_SPEC>;
#[doc = "Graphic MMU LUT entry 383 low"]
pub mod lut383l;
#[doc = "LUT384L register accessor: an alias for `Reg<LUT384L_SPEC>`"]
pub type LUT384L = crate::Reg<lut384l::LUT384L_SPEC>;
#[doc = "Graphic MMU LUT entry 384 low"]
pub mod lut384l;
#[doc = "LUT385L register accessor: an alias for `Reg<LUT385L_SPEC>`"]
pub type LUT385L = crate::Reg<lut385l::LUT385L_SPEC>;
#[doc = "Graphic MMU LUT entry 385 low"]
pub mod lut385l;
#[doc = "LUT386L register accessor: an alias for `Reg<LUT386L_SPEC>`"]
pub type LUT386L = crate::Reg<lut386l::LUT386L_SPEC>;
#[doc = "Graphic MMU LUT entry 386 low"]
pub mod lut386l;
#[doc = "LUT387L register accessor: an alias for `Reg<LUT387L_SPEC>`"]
pub type LUT387L = crate::Reg<lut387l::LUT387L_SPEC>;
#[doc = "Graphic MMU LUT entry 387 low"]
pub mod lut387l;
#[doc = "LUT388L register accessor: an alias for `Reg<LUT388L_SPEC>`"]
pub type LUT388L = crate::Reg<lut388l::LUT388L_SPEC>;
#[doc = "Graphic MMU LUT entry 388 low"]
pub mod lut388l;
#[doc = "LUT389L register accessor: an alias for `Reg<LUT389L_SPEC>`"]
pub type LUT389L = crate::Reg<lut389l::LUT389L_SPEC>;
#[doc = "Graphic MMU LUT entry 389 low"]
pub mod lut389l;
#[doc = "LUT390L register accessor: an alias for `Reg<LUT390L_SPEC>`"]
pub type LUT390L = crate::Reg<lut390l::LUT390L_SPEC>;
#[doc = "Graphic MMU LUT entry 390 low"]
pub mod lut390l;
#[doc = "LUT391L register accessor: an alias for `Reg<LUT391L_SPEC>`"]
pub type LUT391L = crate::Reg<lut391l::LUT391L_SPEC>;
#[doc = "Graphic MMU LUT entry 391 low"]
pub mod lut391l;
#[doc = "LUT392L register accessor: an alias for `Reg<LUT392L_SPEC>`"]
pub type LUT392L = crate::Reg<lut392l::LUT392L_SPEC>;
#[doc = "Graphic MMU LUT entry 392 low"]
pub mod lut392l;
#[doc = "LUT393L register accessor: an alias for `Reg<LUT393L_SPEC>`"]
pub type LUT393L = crate::Reg<lut393l::LUT393L_SPEC>;
#[doc = "Graphic MMU LUT entry 393 low"]
pub mod lut393l;
#[doc = "LUT394L register accessor: an alias for `Reg<LUT394L_SPEC>`"]
pub type LUT394L = crate::Reg<lut394l::LUT394L_SPEC>;
#[doc = "Graphic MMU LUT entry 394 low"]
pub mod lut394l;
#[doc = "LUT395L register accessor: an alias for `Reg<LUT395L_SPEC>`"]
pub type LUT395L = crate::Reg<lut395l::LUT395L_SPEC>;
#[doc = "Graphic MMU LUT entry 395 low"]
pub mod lut395l;
#[doc = "LUT396L register accessor: an alias for `Reg<LUT396L_SPEC>`"]
pub type LUT396L = crate::Reg<lut396l::LUT396L_SPEC>;
#[doc = "Graphic MMU LUT entry 396 low"]
pub mod lut396l;
#[doc = "LUT397L register accessor: an alias for `Reg<LUT397L_SPEC>`"]
pub type LUT397L = crate::Reg<lut397l::LUT397L_SPEC>;
#[doc = "Graphic MMU LUT entry 397 low"]
pub mod lut397l;
#[doc = "LUT398L register accessor: an alias for `Reg<LUT398L_SPEC>`"]
pub type LUT398L = crate::Reg<lut398l::LUT398L_SPEC>;
#[doc = "Graphic MMU LUT entry 398 low"]
pub mod lut398l;
#[doc = "LUT399L register accessor: an alias for `Reg<LUT399L_SPEC>`"]
pub type LUT399L = crate::Reg<lut399l::LUT399L_SPEC>;
#[doc = "Graphic MMU LUT entry 399 low"]
pub mod lut399l;
#[doc = "LUT400L register accessor: an alias for `Reg<LUT400L_SPEC>`"]
pub type LUT400L = crate::Reg<lut400l::LUT400L_SPEC>;
#[doc = "Graphic MMU LUT entry 400 low"]
pub mod lut400l;
#[doc = "LUT401L register accessor: an alias for `Reg<LUT401L_SPEC>`"]
pub type LUT401L = crate::Reg<lut401l::LUT401L_SPEC>;
#[doc = "Graphic MMU LUT entry 401 low"]
pub mod lut401l;
#[doc = "LUT402L register accessor: an alias for `Reg<LUT402L_SPEC>`"]
pub type LUT402L = crate::Reg<lut402l::LUT402L_SPEC>;
#[doc = "Graphic MMU LUT entry 402 low"]
pub mod lut402l;
#[doc = "LUT403L register accessor: an alias for `Reg<LUT403L_SPEC>`"]
pub type LUT403L = crate::Reg<lut403l::LUT403L_SPEC>;
#[doc = "Graphic MMU LUT entry 403 low"]
pub mod lut403l;
#[doc = "LUT404L register accessor: an alias for `Reg<LUT404L_SPEC>`"]
pub type LUT404L = crate::Reg<lut404l::LUT404L_SPEC>;
#[doc = "Graphic MMU LUT entry 404 low"]
pub mod lut404l;
#[doc = "LUT405L register accessor: an alias for `Reg<LUT405L_SPEC>`"]
pub type LUT405L = crate::Reg<lut405l::LUT405L_SPEC>;
#[doc = "Graphic MMU LUT entry 405 low"]
pub mod lut405l;
#[doc = "LUT406L register accessor: an alias for `Reg<LUT406L_SPEC>`"]
pub type LUT406L = crate::Reg<lut406l::LUT406L_SPEC>;
#[doc = "Graphic MMU LUT entry 406 low"]
pub mod lut406l;
#[doc = "LUT407L register accessor: an alias for `Reg<LUT407L_SPEC>`"]
pub type LUT407L = crate::Reg<lut407l::LUT407L_SPEC>;
#[doc = "Graphic MMU LUT entry 407 low"]
pub mod lut407l;
#[doc = "LUT408L register accessor: an alias for `Reg<LUT408L_SPEC>`"]
pub type LUT408L = crate::Reg<lut408l::LUT408L_SPEC>;
#[doc = "Graphic MMU LUT entry 408 low"]
pub mod lut408l;
#[doc = "LUT409L register accessor: an alias for `Reg<LUT409L_SPEC>`"]
pub type LUT409L = crate::Reg<lut409l::LUT409L_SPEC>;
#[doc = "Graphic MMU LUT entry 409 low"]
pub mod lut409l;
#[doc = "LUT410L register accessor: an alias for `Reg<LUT410L_SPEC>`"]
pub type LUT410L = crate::Reg<lut410l::LUT410L_SPEC>;
#[doc = "Graphic MMU LUT entry 410 low"]
pub mod lut410l;
#[doc = "LUT411L register accessor: an alias for `Reg<LUT411L_SPEC>`"]
pub type LUT411L = crate::Reg<lut411l::LUT411L_SPEC>;
#[doc = "Graphic MMU LUT entry 411 low"]
pub mod lut411l;
#[doc = "LUT412L register accessor: an alias for `Reg<LUT412L_SPEC>`"]
pub type LUT412L = crate::Reg<lut412l::LUT412L_SPEC>;
#[doc = "Graphic MMU LUT entry 412 low"]
pub mod lut412l;
#[doc = "LUT413L register accessor: an alias for `Reg<LUT413L_SPEC>`"]
pub type LUT413L = crate::Reg<lut413l::LUT413L_SPEC>;
#[doc = "Graphic MMU LUT entry 413 low"]
pub mod lut413l;
#[doc = "LUT414L register accessor: an alias for `Reg<LUT414L_SPEC>`"]
pub type LUT414L = crate::Reg<lut414l::LUT414L_SPEC>;
#[doc = "Graphic MMU LUT entry 414 low"]
pub mod lut414l;
#[doc = "LUT415L register accessor: an alias for `Reg<LUT415L_SPEC>`"]
pub type LUT415L = crate::Reg<lut415l::LUT415L_SPEC>;
#[doc = "Graphic MMU LUT entry 415 low"]
pub mod lut415l;
#[doc = "LUT416L register accessor: an alias for `Reg<LUT416L_SPEC>`"]
pub type LUT416L = crate::Reg<lut416l::LUT416L_SPEC>;
#[doc = "Graphic MMU LUT entry 416 low"]
pub mod lut416l;
#[doc = "LUT417L register accessor: an alias for `Reg<LUT417L_SPEC>`"]
pub type LUT417L = crate::Reg<lut417l::LUT417L_SPEC>;
#[doc = "Graphic MMU LUT entry 417 low"]
pub mod lut417l;
#[doc = "LUT418L register accessor: an alias for `Reg<LUT418L_SPEC>`"]
pub type LUT418L = crate::Reg<lut418l::LUT418L_SPEC>;
#[doc = "Graphic MMU LUT entry 418 low"]
pub mod lut418l;
#[doc = "LUT419L register accessor: an alias for `Reg<LUT419L_SPEC>`"]
pub type LUT419L = crate::Reg<lut419l::LUT419L_SPEC>;
#[doc = "Graphic MMU LUT entry 419 low"]
pub mod lut419l;
#[doc = "LUT420L register accessor: an alias for `Reg<LUT420L_SPEC>`"]
pub type LUT420L = crate::Reg<lut420l::LUT420L_SPEC>;
#[doc = "Graphic MMU LUT entry 420 low"]
pub mod lut420l;
#[doc = "LUT421L register accessor: an alias for `Reg<LUT421L_SPEC>`"]
pub type LUT421L = crate::Reg<lut421l::LUT421L_SPEC>;
#[doc = "Graphic MMU LUT entry 421 low"]
pub mod lut421l;
#[doc = "LUT422L register accessor: an alias for `Reg<LUT422L_SPEC>`"]
pub type LUT422L = crate::Reg<lut422l::LUT422L_SPEC>;
#[doc = "Graphic MMU LUT entry 422 low"]
pub mod lut422l;
#[doc = "LUT423L register accessor: an alias for `Reg<LUT423L_SPEC>`"]
pub type LUT423L = crate::Reg<lut423l::LUT423L_SPEC>;
#[doc = "Graphic MMU LUT entry 423 low"]
pub mod lut423l;
#[doc = "LUT424L register accessor: an alias for `Reg<LUT424L_SPEC>`"]
pub type LUT424L = crate::Reg<lut424l::LUT424L_SPEC>;
#[doc = "Graphic MMU LUT entry 424 low"]
pub mod lut424l;
#[doc = "LUT425L register accessor: an alias for `Reg<LUT425L_SPEC>`"]
pub type LUT425L = crate::Reg<lut425l::LUT425L_SPEC>;
#[doc = "Graphic MMU LUT entry 425 low"]
pub mod lut425l;
#[doc = "LUT426L register accessor: an alias for `Reg<LUT426L_SPEC>`"]
pub type LUT426L = crate::Reg<lut426l::LUT426L_SPEC>;
#[doc = "Graphic MMU LUT entry 426 low"]
pub mod lut426l;
#[doc = "LUT427L register accessor: an alias for `Reg<LUT427L_SPEC>`"]
pub type LUT427L = crate::Reg<lut427l::LUT427L_SPEC>;
#[doc = "Graphic MMU LUT entry 427 low"]
pub mod lut427l;
#[doc = "LUT428L register accessor: an alias for `Reg<LUT428L_SPEC>`"]
pub type LUT428L = crate::Reg<lut428l::LUT428L_SPEC>;
#[doc = "Graphic MMU LUT entry 428 low"]
pub mod lut428l;
#[doc = "LUT429L register accessor: an alias for `Reg<LUT429L_SPEC>`"]
pub type LUT429L = crate::Reg<lut429l::LUT429L_SPEC>;
#[doc = "Graphic MMU LUT entry 429 low"]
pub mod lut429l;
#[doc = "LUT430L register accessor: an alias for `Reg<LUT430L_SPEC>`"]
pub type LUT430L = crate::Reg<lut430l::LUT430L_SPEC>;
#[doc = "Graphic MMU LUT entry 430 low"]
pub mod lut430l;
#[doc = "LUT431L register accessor: an alias for `Reg<LUT431L_SPEC>`"]
pub type LUT431L = crate::Reg<lut431l::LUT431L_SPEC>;
#[doc = "Graphic MMU LUT entry 431 low"]
pub mod lut431l;
#[doc = "LUT432L register accessor: an alias for `Reg<LUT432L_SPEC>`"]
pub type LUT432L = crate::Reg<lut432l::LUT432L_SPEC>;
#[doc = "Graphic MMU LUT entry 432 low"]
pub mod lut432l;
#[doc = "LUT433L register accessor: an alias for `Reg<LUT433L_SPEC>`"]
pub type LUT433L = crate::Reg<lut433l::LUT433L_SPEC>;
#[doc = "Graphic MMU LUT entry 433 low"]
pub mod lut433l;
#[doc = "LUT434L register accessor: an alias for `Reg<LUT434L_SPEC>`"]
pub type LUT434L = crate::Reg<lut434l::LUT434L_SPEC>;
#[doc = "Graphic MMU LUT entry 434 low"]
pub mod lut434l;
#[doc = "LUT435L register accessor: an alias for `Reg<LUT435L_SPEC>`"]
pub type LUT435L = crate::Reg<lut435l::LUT435L_SPEC>;
#[doc = "Graphic MMU LUT entry 435 low"]
pub mod lut435l;
#[doc = "LUT436L register accessor: an alias for `Reg<LUT436L_SPEC>`"]
pub type LUT436L = crate::Reg<lut436l::LUT436L_SPEC>;
#[doc = "Graphic MMU LUT entry 436 low"]
pub mod lut436l;
#[doc = "LUT437L register accessor: an alias for `Reg<LUT437L_SPEC>`"]
pub type LUT437L = crate::Reg<lut437l::LUT437L_SPEC>;
#[doc = "Graphic MMU LUT entry 437 low"]
pub mod lut437l;
#[doc = "LUT438L register accessor: an alias for `Reg<LUT438L_SPEC>`"]
pub type LUT438L = crate::Reg<lut438l::LUT438L_SPEC>;
#[doc = "Graphic MMU LUT entry 438 low"]
pub mod lut438l;
#[doc = "LUT439L register accessor: an alias for `Reg<LUT439L_SPEC>`"]
pub type LUT439L = crate::Reg<lut439l::LUT439L_SPEC>;
#[doc = "Graphic MMU LUT entry 439 low"]
pub mod lut439l;
#[doc = "LUT440L register accessor: an alias for `Reg<LUT440L_SPEC>`"]
pub type LUT440L = crate::Reg<lut440l::LUT440L_SPEC>;
#[doc = "Graphic MMU LUT entry 440 low"]
pub mod lut440l;
#[doc = "LUT441L register accessor: an alias for `Reg<LUT441L_SPEC>`"]
pub type LUT441L = crate::Reg<lut441l::LUT441L_SPEC>;
#[doc = "Graphic MMU LUT entry 441 low"]
pub mod lut441l;
#[doc = "LUT442L register accessor: an alias for `Reg<LUT442L_SPEC>`"]
pub type LUT442L = crate::Reg<lut442l::LUT442L_SPEC>;
#[doc = "Graphic MMU LUT entry 442 low"]
pub mod lut442l;
#[doc = "LUT443L register accessor: an alias for `Reg<LUT443L_SPEC>`"]
pub type LUT443L = crate::Reg<lut443l::LUT443L_SPEC>;
#[doc = "Graphic MMU LUT entry 443 low"]
pub mod lut443l;
#[doc = "LUT444L register accessor: an alias for `Reg<LUT444L_SPEC>`"]
pub type LUT444L = crate::Reg<lut444l::LUT444L_SPEC>;
#[doc = "Graphic MMU LUT entry 444 low"]
pub mod lut444l;
#[doc = "LUT445L register accessor: an alias for `Reg<LUT445L_SPEC>`"]
pub type LUT445L = crate::Reg<lut445l::LUT445L_SPEC>;
#[doc = "Graphic MMU LUT entry 445 low"]
pub mod lut445l;
#[doc = "LUT446L register accessor: an alias for `Reg<LUT446L_SPEC>`"]
pub type LUT446L = crate::Reg<lut446l::LUT446L_SPEC>;
#[doc = "Graphic MMU LUT entry 446 low"]
pub mod lut446l;
#[doc = "LUT447L register accessor: an alias for `Reg<LUT447L_SPEC>`"]
pub type LUT447L = crate::Reg<lut447l::LUT447L_SPEC>;
#[doc = "Graphic MMU LUT entry 447 low"]
pub mod lut447l;
#[doc = "LUT448L register accessor: an alias for `Reg<LUT448L_SPEC>`"]
pub type LUT448L = crate::Reg<lut448l::LUT448L_SPEC>;
#[doc = "Graphic MMU LUT entry 448 low"]
pub mod lut448l;
#[doc = "LUT449L register accessor: an alias for `Reg<LUT449L_SPEC>`"]
pub type LUT449L = crate::Reg<lut449l::LUT449L_SPEC>;
#[doc = "Graphic MMU LUT entry 449 low"]
pub mod lut449l;
#[doc = "LUT450L register accessor: an alias for `Reg<LUT450L_SPEC>`"]
pub type LUT450L = crate::Reg<lut450l::LUT450L_SPEC>;
#[doc = "Graphic MMU LUT entry 450 low"]
pub mod lut450l;
#[doc = "LUT451L register accessor: an alias for `Reg<LUT451L_SPEC>`"]
pub type LUT451L = crate::Reg<lut451l::LUT451L_SPEC>;
#[doc = "Graphic MMU LUT entry 451 low"]
pub mod lut451l;
#[doc = "LUT452L register accessor: an alias for `Reg<LUT452L_SPEC>`"]
pub type LUT452L = crate::Reg<lut452l::LUT452L_SPEC>;
#[doc = "Graphic MMU LUT entry 452 low"]
pub mod lut452l;
#[doc = "LUT453L register accessor: an alias for `Reg<LUT453L_SPEC>`"]
pub type LUT453L = crate::Reg<lut453l::LUT453L_SPEC>;
#[doc = "Graphic MMU LUT entry 453 low"]
pub mod lut453l;
#[doc = "LUT454L register accessor: an alias for `Reg<LUT454L_SPEC>`"]
pub type LUT454L = crate::Reg<lut454l::LUT454L_SPEC>;
#[doc = "Graphic MMU LUT entry 454 low"]
pub mod lut454l;
#[doc = "LUT455L register accessor: an alias for `Reg<LUT455L_SPEC>`"]
pub type LUT455L = crate::Reg<lut455l::LUT455L_SPEC>;
#[doc = "Graphic MMU LUT entry 455 low"]
pub mod lut455l;
#[doc = "LUT456L register accessor: an alias for `Reg<LUT456L_SPEC>`"]
pub type LUT456L = crate::Reg<lut456l::LUT456L_SPEC>;
#[doc = "Graphic MMU LUT entry 456 low"]
pub mod lut456l;
#[doc = "LUT457L register accessor: an alias for `Reg<LUT457L_SPEC>`"]
pub type LUT457L = crate::Reg<lut457l::LUT457L_SPEC>;
#[doc = "Graphic MMU LUT entry 457 low"]
pub mod lut457l;
#[doc = "LUT458L register accessor: an alias for `Reg<LUT458L_SPEC>`"]
pub type LUT458L = crate::Reg<lut458l::LUT458L_SPEC>;
#[doc = "Graphic MMU LUT entry 458 low"]
pub mod lut458l;
#[doc = "LUT459L register accessor: an alias for `Reg<LUT459L_SPEC>`"]
pub type LUT459L = crate::Reg<lut459l::LUT459L_SPEC>;
#[doc = "Graphic MMU LUT entry 459 low"]
pub mod lut459l;
#[doc = "LUT460L register accessor: an alias for `Reg<LUT460L_SPEC>`"]
pub type LUT460L = crate::Reg<lut460l::LUT460L_SPEC>;
#[doc = "Graphic MMU LUT entry 460 low"]
pub mod lut460l;
#[doc = "LUT461L register accessor: an alias for `Reg<LUT461L_SPEC>`"]
pub type LUT461L = crate::Reg<lut461l::LUT461L_SPEC>;
#[doc = "Graphic MMU LUT entry 461 low"]
pub mod lut461l;
#[doc = "LUT462L register accessor: an alias for `Reg<LUT462L_SPEC>`"]
pub type LUT462L = crate::Reg<lut462l::LUT462L_SPEC>;
#[doc = "Graphic MMU LUT entry 462 low"]
pub mod lut462l;
#[doc = "LUT463L register accessor: an alias for `Reg<LUT463L_SPEC>`"]
pub type LUT463L = crate::Reg<lut463l::LUT463L_SPEC>;
#[doc = "Graphic MMU LUT entry 463 low"]
pub mod lut463l;
#[doc = "LUT464L register accessor: an alias for `Reg<LUT464L_SPEC>`"]
pub type LUT464L = crate::Reg<lut464l::LUT464L_SPEC>;
#[doc = "Graphic MMU LUT entry 464 low"]
pub mod lut464l;
#[doc = "LUT465L register accessor: an alias for `Reg<LUT465L_SPEC>`"]
pub type LUT465L = crate::Reg<lut465l::LUT465L_SPEC>;
#[doc = "Graphic MMU LUT entry 465 low"]
pub mod lut465l;
#[doc = "LUT466L register accessor: an alias for `Reg<LUT466L_SPEC>`"]
pub type LUT466L = crate::Reg<lut466l::LUT466L_SPEC>;
#[doc = "Graphic MMU LUT entry 466 low"]
pub mod lut466l;
#[doc = "LUT467L register accessor: an alias for `Reg<LUT467L_SPEC>`"]
pub type LUT467L = crate::Reg<lut467l::LUT467L_SPEC>;
#[doc = "Graphic MMU LUT entry 467 low"]
pub mod lut467l;
#[doc = "LUT468L register accessor: an alias for `Reg<LUT468L_SPEC>`"]
pub type LUT468L = crate::Reg<lut468l::LUT468L_SPEC>;
#[doc = "Graphic MMU LUT entry 468 low"]
pub mod lut468l;
#[doc = "LUT469L register accessor: an alias for `Reg<LUT469L_SPEC>`"]
pub type LUT469L = crate::Reg<lut469l::LUT469L_SPEC>;
#[doc = "Graphic MMU LUT entry 469 low"]
pub mod lut469l;
#[doc = "LUT470L register accessor: an alias for `Reg<LUT470L_SPEC>`"]
pub type LUT470L = crate::Reg<lut470l::LUT470L_SPEC>;
#[doc = "Graphic MMU LUT entry 470 low"]
pub mod lut470l;
#[doc = "LUT471L register accessor: an alias for `Reg<LUT471L_SPEC>`"]
pub type LUT471L = crate::Reg<lut471l::LUT471L_SPEC>;
#[doc = "Graphic MMU LUT entry 471 low"]
pub mod lut471l;
#[doc = "LUT472L register accessor: an alias for `Reg<LUT472L_SPEC>`"]
pub type LUT472L = crate::Reg<lut472l::LUT472L_SPEC>;
#[doc = "Graphic MMU LUT entry 472 low"]
pub mod lut472l;
#[doc = "LUT473L register accessor: an alias for `Reg<LUT473L_SPEC>`"]
pub type LUT473L = crate::Reg<lut473l::LUT473L_SPEC>;
#[doc = "Graphic MMU LUT entry 473 low"]
pub mod lut473l;
#[doc = "LUT474L register accessor: an alias for `Reg<LUT474L_SPEC>`"]
pub type LUT474L = crate::Reg<lut474l::LUT474L_SPEC>;
#[doc = "Graphic MMU LUT entry 474 low"]
pub mod lut474l;
#[doc = "LUT475L register accessor: an alias for `Reg<LUT475L_SPEC>`"]
pub type LUT475L = crate::Reg<lut475l::LUT475L_SPEC>;
#[doc = "Graphic MMU LUT entry 475 low"]
pub mod lut475l;
#[doc = "LUT476L register accessor: an alias for `Reg<LUT476L_SPEC>`"]
pub type LUT476L = crate::Reg<lut476l::LUT476L_SPEC>;
#[doc = "Graphic MMU LUT entry 476 low"]
pub mod lut476l;
#[doc = "LUT477L register accessor: an alias for `Reg<LUT477L_SPEC>`"]
pub type LUT477L = crate::Reg<lut477l::LUT477L_SPEC>;
#[doc = "Graphic MMU LUT entry 477 low"]
pub mod lut477l;
#[doc = "LUT478L register accessor: an alias for `Reg<LUT478L_SPEC>`"]
pub type LUT478L = crate::Reg<lut478l::LUT478L_SPEC>;
#[doc = "Graphic MMU LUT entry 478 low"]
pub mod lut478l;
#[doc = "LUT479L register accessor: an alias for `Reg<LUT479L_SPEC>`"]
pub type LUT479L = crate::Reg<lut479l::LUT479L_SPEC>;
#[doc = "Graphic MMU LUT entry 479 low"]
pub mod lut479l;
#[doc = "LUT480L register accessor: an alias for `Reg<LUT480L_SPEC>`"]
pub type LUT480L = crate::Reg<lut480l::LUT480L_SPEC>;
#[doc = "Graphic MMU LUT entry 480 low"]
pub mod lut480l;
#[doc = "LUT481L register accessor: an alias for `Reg<LUT481L_SPEC>`"]
pub type LUT481L = crate::Reg<lut481l::LUT481L_SPEC>;
#[doc = "Graphic MMU LUT entry 481 low"]
pub mod lut481l;
#[doc = "LUT482L register accessor: an alias for `Reg<LUT482L_SPEC>`"]
pub type LUT482L = crate::Reg<lut482l::LUT482L_SPEC>;
#[doc = "Graphic MMU LUT entry 482 low"]
pub mod lut482l;
#[doc = "LUT483L register accessor: an alias for `Reg<LUT483L_SPEC>`"]
pub type LUT483L = crate::Reg<lut483l::LUT483L_SPEC>;
#[doc = "Graphic MMU LUT entry 483 low"]
pub mod lut483l;
#[doc = "LUT484L register accessor: an alias for `Reg<LUT484L_SPEC>`"]
pub type LUT484L = crate::Reg<lut484l::LUT484L_SPEC>;
#[doc = "Graphic MMU LUT entry 484 low"]
pub mod lut484l;
#[doc = "LUT485L register accessor: an alias for `Reg<LUT485L_SPEC>`"]
pub type LUT485L = crate::Reg<lut485l::LUT485L_SPEC>;
#[doc = "Graphic MMU LUT entry 485 low"]
pub mod lut485l;
#[doc = "LUT486L register accessor: an alias for `Reg<LUT486L_SPEC>`"]
pub type LUT486L = crate::Reg<lut486l::LUT486L_SPEC>;
#[doc = "Graphic MMU LUT entry 486 low"]
pub mod lut486l;
#[doc = "LUT487L register accessor: an alias for `Reg<LUT487L_SPEC>`"]
pub type LUT487L = crate::Reg<lut487l::LUT487L_SPEC>;
#[doc = "Graphic MMU LUT entry 487 low"]
pub mod lut487l;
#[doc = "LUT488L register accessor: an alias for `Reg<LUT488L_SPEC>`"]
pub type LUT488L = crate::Reg<lut488l::LUT488L_SPEC>;
#[doc = "Graphic MMU LUT entry 488 low"]
pub mod lut488l;
#[doc = "LUT489L register accessor: an alias for `Reg<LUT489L_SPEC>`"]
pub type LUT489L = crate::Reg<lut489l::LUT489L_SPEC>;
#[doc = "Graphic MMU LUT entry 489 low"]
pub mod lut489l;
#[doc = "LUT490L register accessor: an alias for `Reg<LUT490L_SPEC>`"]
pub type LUT490L = crate::Reg<lut490l::LUT490L_SPEC>;
#[doc = "Graphic MMU LUT entry 490 low"]
pub mod lut490l;
#[doc = "LUT491L register accessor: an alias for `Reg<LUT491L_SPEC>`"]
pub type LUT491L = crate::Reg<lut491l::LUT491L_SPEC>;
#[doc = "Graphic MMU LUT entry 491 low"]
pub mod lut491l;
#[doc = "LUT492L register accessor: an alias for `Reg<LUT492L_SPEC>`"]
pub type LUT492L = crate::Reg<lut492l::LUT492L_SPEC>;
#[doc = "Graphic MMU LUT entry 492 low"]
pub mod lut492l;
#[doc = "LUT493L register accessor: an alias for `Reg<LUT493L_SPEC>`"]
pub type LUT493L = crate::Reg<lut493l::LUT493L_SPEC>;
#[doc = "Graphic MMU LUT entry 493 low"]
pub mod lut493l;
#[doc = "LUT494L register accessor: an alias for `Reg<LUT494L_SPEC>`"]
pub type LUT494L = crate::Reg<lut494l::LUT494L_SPEC>;
#[doc = "Graphic MMU LUT entry 494 low"]
pub mod lut494l;
#[doc = "LUT495L register accessor: an alias for `Reg<LUT495L_SPEC>`"]
pub type LUT495L = crate::Reg<lut495l::LUT495L_SPEC>;
#[doc = "Graphic MMU LUT entry 495 low"]
pub mod lut495l;
#[doc = "LUT496L register accessor: an alias for `Reg<LUT496L_SPEC>`"]
pub type LUT496L = crate::Reg<lut496l::LUT496L_SPEC>;
#[doc = "Graphic MMU LUT entry 496 low"]
pub mod lut496l;
#[doc = "LUT497L register accessor: an alias for `Reg<LUT497L_SPEC>`"]
pub type LUT497L = crate::Reg<lut497l::LUT497L_SPEC>;
#[doc = "Graphic MMU LUT entry 497 low"]
pub mod lut497l;
#[doc = "LUT498L register accessor: an alias for `Reg<LUT498L_SPEC>`"]
pub type LUT498L = crate::Reg<lut498l::LUT498L_SPEC>;
#[doc = "Graphic MMU LUT entry 498 low"]
pub mod lut498l;
#[doc = "LUT499L register accessor: an alias for `Reg<LUT499L_SPEC>`"]
pub type LUT499L = crate::Reg<lut499l::LUT499L_SPEC>;
#[doc = "Graphic MMU LUT entry 499 low"]
pub mod lut499l;
#[doc = "LUT500L register accessor: an alias for `Reg<LUT500L_SPEC>`"]
pub type LUT500L = crate::Reg<lut500l::LUT500L_SPEC>;
#[doc = "Graphic MMU LUT entry 500 low"]
pub mod lut500l;
#[doc = "LUT501L register accessor: an alias for `Reg<LUT501L_SPEC>`"]
pub type LUT501L = crate::Reg<lut501l::LUT501L_SPEC>;
#[doc = "Graphic MMU LUT entry 501 low"]
pub mod lut501l;
#[doc = "LUT502L register accessor: an alias for `Reg<LUT502L_SPEC>`"]
pub type LUT502L = crate::Reg<lut502l::LUT502L_SPEC>;
#[doc = "Graphic MMU LUT entry 502 low"]
pub mod lut502l;
#[doc = "LUT503L register accessor: an alias for `Reg<LUT503L_SPEC>`"]
pub type LUT503L = crate::Reg<lut503l::LUT503L_SPEC>;
#[doc = "Graphic MMU LUT entry 503 low"]
pub mod lut503l;
#[doc = "LUT504L register accessor: an alias for `Reg<LUT504L_SPEC>`"]
pub type LUT504L = crate::Reg<lut504l::LUT504L_SPEC>;
#[doc = "Graphic MMU LUT entry 504 low"]
pub mod lut504l;
#[doc = "LUT505L register accessor: an alias for `Reg<LUT505L_SPEC>`"]
pub type LUT505L = crate::Reg<lut505l::LUT505L_SPEC>;
#[doc = "Graphic MMU LUT entry 505 low"]
pub mod lut505l;
#[doc = "LUT506L register accessor: an alias for `Reg<LUT506L_SPEC>`"]
pub type LUT506L = crate::Reg<lut506l::LUT506L_SPEC>;
#[doc = "Graphic MMU LUT entry 506 low"]
pub mod lut506l;
#[doc = "LUT507L register accessor: an alias for `Reg<LUT507L_SPEC>`"]
pub type LUT507L = crate::Reg<lut507l::LUT507L_SPEC>;
#[doc = "Graphic MMU LUT entry 507 low"]
pub mod lut507l;
#[doc = "LUT508L register accessor: an alias for `Reg<LUT508L_SPEC>`"]
pub type LUT508L = crate::Reg<lut508l::LUT508L_SPEC>;
#[doc = "Graphic MMU LUT entry 508 low"]
pub mod lut508l;
#[doc = "LUT509L register accessor: an alias for `Reg<LUT509L_SPEC>`"]
pub type LUT509L = crate::Reg<lut509l::LUT509L_SPEC>;
#[doc = "Graphic MMU LUT entry 509 low"]
pub mod lut509l;
#[doc = "LUT510L register accessor: an alias for `Reg<LUT510L_SPEC>`"]
pub type LUT510L = crate::Reg<lut510l::LUT510L_SPEC>;
#[doc = "Graphic MMU LUT entry 510 low"]
pub mod lut510l;
#[doc = "LUT511L register accessor: an alias for `Reg<LUT511L_SPEC>`"]
pub type LUT511L = crate::Reg<lut511l::LUT511L_SPEC>;
#[doc = "Graphic MMU LUT entry 511 low"]
pub mod lut511l;
#[doc = "LUT512L register accessor: an alias for `Reg<LUT512L_SPEC>`"]
pub type LUT512L = crate::Reg<lut512l::LUT512L_SPEC>;
#[doc = "Graphic MMU LUT entry 512 low"]
pub mod lut512l;
#[doc = "LUT513L register accessor: an alias for `Reg<LUT513L_SPEC>`"]
pub type LUT513L = crate::Reg<lut513l::LUT513L_SPEC>;
#[doc = "Graphic MMU LUT entry 513 low"]
pub mod lut513l;
#[doc = "LUT514L register accessor: an alias for `Reg<LUT514L_SPEC>`"]
pub type LUT514L = crate::Reg<lut514l::LUT514L_SPEC>;
#[doc = "Graphic MMU LUT entry 514 low"]
pub mod lut514l;
#[doc = "LUT515L register accessor: an alias for `Reg<LUT515L_SPEC>`"]
pub type LUT515L = crate::Reg<lut515l::LUT515L_SPEC>;
#[doc = "Graphic MMU LUT entry 515 low"]
pub mod lut515l;
#[doc = "LUT516L register accessor: an alias for `Reg<LUT516L_SPEC>`"]
pub type LUT516L = crate::Reg<lut516l::LUT516L_SPEC>;
#[doc = "Graphic MMU LUT entry 516 low"]
pub mod lut516l;
#[doc = "LUT517L register accessor: an alias for `Reg<LUT517L_SPEC>`"]
pub type LUT517L = crate::Reg<lut517l::LUT517L_SPEC>;
#[doc = "Graphic MMU LUT entry 517 low"]
pub mod lut517l;
#[doc = "LUT518L register accessor: an alias for `Reg<LUT518L_SPEC>`"]
pub type LUT518L = crate::Reg<lut518l::LUT518L_SPEC>;
#[doc = "Graphic MMU LUT entry 518 low"]
pub mod lut518l;
#[doc = "LUT519L register accessor: an alias for `Reg<LUT519L_SPEC>`"]
pub type LUT519L = crate::Reg<lut519l::LUT519L_SPEC>;
#[doc = "Graphic MMU LUT entry 519 low"]
pub mod lut519l;
#[doc = "LUT520L register accessor: an alias for `Reg<LUT520L_SPEC>`"]
pub type LUT520L = crate::Reg<lut520l::LUT520L_SPEC>;
#[doc = "Graphic MMU LUT entry 520 low"]
pub mod lut520l;
#[doc = "LUT521L register accessor: an alias for `Reg<LUT521L_SPEC>`"]
pub type LUT521L = crate::Reg<lut521l::LUT521L_SPEC>;
#[doc = "Graphic MMU LUT entry 521 low"]
pub mod lut521l;
#[doc = "LUT522L register accessor: an alias for `Reg<LUT522L_SPEC>`"]
pub type LUT522L = crate::Reg<lut522l::LUT522L_SPEC>;
#[doc = "Graphic MMU LUT entry 522 low"]
pub mod lut522l;
#[doc = "LUT523L register accessor: an alias for `Reg<LUT523L_SPEC>`"]
pub type LUT523L = crate::Reg<lut523l::LUT523L_SPEC>;
#[doc = "Graphic MMU LUT entry 523 low"]
pub mod lut523l;
#[doc = "LUT524L register accessor: an alias for `Reg<LUT524L_SPEC>`"]
pub type LUT524L = crate::Reg<lut524l::LUT524L_SPEC>;
#[doc = "Graphic MMU LUT entry 524 low"]
pub mod lut524l;
#[doc = "LUT525L register accessor: an alias for `Reg<LUT525L_SPEC>`"]
pub type LUT525L = crate::Reg<lut525l::LUT525L_SPEC>;
#[doc = "Graphic MMU LUT entry 525 low"]
pub mod lut525l;
#[doc = "LUT526L register accessor: an alias for `Reg<LUT526L_SPEC>`"]
pub type LUT526L = crate::Reg<lut526l::LUT526L_SPEC>;
#[doc = "Graphic MMU LUT entry 526 low"]
pub mod lut526l;
#[doc = "LUT527L register accessor: an alias for `Reg<LUT527L_SPEC>`"]
pub type LUT527L = crate::Reg<lut527l::LUT527L_SPEC>;
#[doc = "Graphic MMU LUT entry 527 low"]
pub mod lut527l;
#[doc = "LUT528L register accessor: an alias for `Reg<LUT528L_SPEC>`"]
pub type LUT528L = crate::Reg<lut528l::LUT528L_SPEC>;
#[doc = "Graphic MMU LUT entry 528 low"]
pub mod lut528l;
#[doc = "LUT529L register accessor: an alias for `Reg<LUT529L_SPEC>`"]
pub type LUT529L = crate::Reg<lut529l::LUT529L_SPEC>;
#[doc = "Graphic MMU LUT entry 529 low"]
pub mod lut529l;
#[doc = "LUT530L register accessor: an alias for `Reg<LUT530L_SPEC>`"]
pub type LUT530L = crate::Reg<lut530l::LUT530L_SPEC>;
#[doc = "Graphic MMU LUT entry 530 low"]
pub mod lut530l;
#[doc = "LUT531L register accessor: an alias for `Reg<LUT531L_SPEC>`"]
pub type LUT531L = crate::Reg<lut531l::LUT531L_SPEC>;
#[doc = "Graphic MMU LUT entry 531 low"]
pub mod lut531l;
#[doc = "LUT532L register accessor: an alias for `Reg<LUT532L_SPEC>`"]
pub type LUT532L = crate::Reg<lut532l::LUT532L_SPEC>;
#[doc = "Graphic MMU LUT entry 532 low"]
pub mod lut532l;
#[doc = "LUT533L register accessor: an alias for `Reg<LUT533L_SPEC>`"]
pub type LUT533L = crate::Reg<lut533l::LUT533L_SPEC>;
#[doc = "Graphic MMU LUT entry 533 low"]
pub mod lut533l;
#[doc = "LUT534L register accessor: an alias for `Reg<LUT534L_SPEC>`"]
pub type LUT534L = crate::Reg<lut534l::LUT534L_SPEC>;
#[doc = "Graphic MMU LUT entry 534 low"]
pub mod lut534l;
#[doc = "LUT535L register accessor: an alias for `Reg<LUT535L_SPEC>`"]
pub type LUT535L = crate::Reg<lut535l::LUT535L_SPEC>;
#[doc = "Graphic MMU LUT entry 535 low"]
pub mod lut535l;
#[doc = "LUT536L register accessor: an alias for `Reg<LUT536L_SPEC>`"]
pub type LUT536L = crate::Reg<lut536l::LUT536L_SPEC>;
#[doc = "Graphic MMU LUT entry 536 low"]
pub mod lut536l;
#[doc = "LUT537L register accessor: an alias for `Reg<LUT537L_SPEC>`"]
pub type LUT537L = crate::Reg<lut537l::LUT537L_SPEC>;
#[doc = "Graphic MMU LUT entry 537 low"]
pub mod lut537l;
#[doc = "LUT538L register accessor: an alias for `Reg<LUT538L_SPEC>`"]
pub type LUT538L = crate::Reg<lut538l::LUT538L_SPEC>;
#[doc = "Graphic MMU LUT entry 538 low"]
pub mod lut538l;
#[doc = "LUT539L register accessor: an alias for `Reg<LUT539L_SPEC>`"]
pub type LUT539L = crate::Reg<lut539l::LUT539L_SPEC>;
#[doc = "Graphic MMU LUT entry 539 low"]
pub mod lut539l;
#[doc = "LUT540L register accessor: an alias for `Reg<LUT540L_SPEC>`"]
pub type LUT540L = crate::Reg<lut540l::LUT540L_SPEC>;
#[doc = "Graphic MMU LUT entry 540 low"]
pub mod lut540l;
#[doc = "LUT541L register accessor: an alias for `Reg<LUT541L_SPEC>`"]
pub type LUT541L = crate::Reg<lut541l::LUT541L_SPEC>;
#[doc = "Graphic MMU LUT entry 541 low"]
pub mod lut541l;
#[doc = "LUT542L register accessor: an alias for `Reg<LUT542L_SPEC>`"]
pub type LUT542L = crate::Reg<lut542l::LUT542L_SPEC>;
#[doc = "Graphic MMU LUT entry 542 low"]
pub mod lut542l;
#[doc = "LUT543L register accessor: an alias for `Reg<LUT543L_SPEC>`"]
pub type LUT543L = crate::Reg<lut543l::LUT543L_SPEC>;
#[doc = "Graphic MMU LUT entry 543 low"]
pub mod lut543l;
#[doc = "LUT544L register accessor: an alias for `Reg<LUT544L_SPEC>`"]
pub type LUT544L = crate::Reg<lut544l::LUT544L_SPEC>;
#[doc = "Graphic MMU LUT entry 544 low"]
pub mod lut544l;
#[doc = "LUT545L register accessor: an alias for `Reg<LUT545L_SPEC>`"]
pub type LUT545L = crate::Reg<lut545l::LUT545L_SPEC>;
#[doc = "Graphic MMU LUT entry 545 low"]
pub mod lut545l;
#[doc = "LUT546L register accessor: an alias for `Reg<LUT546L_SPEC>`"]
pub type LUT546L = crate::Reg<lut546l::LUT546L_SPEC>;
#[doc = "Graphic MMU LUT entry 546 low"]
pub mod lut546l;
#[doc = "LUT547L register accessor: an alias for `Reg<LUT547L_SPEC>`"]
pub type LUT547L = crate::Reg<lut547l::LUT547L_SPEC>;
#[doc = "Graphic MMU LUT entry 547 low"]
pub mod lut547l;
#[doc = "LUT548L register accessor: an alias for `Reg<LUT548L_SPEC>`"]
pub type LUT548L = crate::Reg<lut548l::LUT548L_SPEC>;
#[doc = "Graphic MMU LUT entry 548 low"]
pub mod lut548l;
#[doc = "LUT549L register accessor: an alias for `Reg<LUT549L_SPEC>`"]
pub type LUT549L = crate::Reg<lut549l::LUT549L_SPEC>;
#[doc = "Graphic MMU LUT entry 549 low"]
pub mod lut549l;
#[doc = "LUT550L register accessor: an alias for `Reg<LUT550L_SPEC>`"]
pub type LUT550L = crate::Reg<lut550l::LUT550L_SPEC>;
#[doc = "Graphic MMU LUT entry 550 low"]
pub mod lut550l;
#[doc = "LUT551L register accessor: an alias for `Reg<LUT551L_SPEC>`"]
pub type LUT551L = crate::Reg<lut551l::LUT551L_SPEC>;
#[doc = "Graphic MMU LUT entry 551 low"]
pub mod lut551l;
#[doc = "LUT552L register accessor: an alias for `Reg<LUT552L_SPEC>`"]
pub type LUT552L = crate::Reg<lut552l::LUT552L_SPEC>;
#[doc = "Graphic MMU LUT entry 552 low"]
pub mod lut552l;
#[doc = "LUT553L register accessor: an alias for `Reg<LUT553L_SPEC>`"]
pub type LUT553L = crate::Reg<lut553l::LUT553L_SPEC>;
#[doc = "Graphic MMU LUT entry 553 low"]
pub mod lut553l;
#[doc = "LUT554L register accessor: an alias for `Reg<LUT554L_SPEC>`"]
pub type LUT554L = crate::Reg<lut554l::LUT554L_SPEC>;
#[doc = "Graphic MMU LUT entry 554 low"]
pub mod lut554l;
#[doc = "LUT555L register accessor: an alias for `Reg<LUT555L_SPEC>`"]
pub type LUT555L = crate::Reg<lut555l::LUT555L_SPEC>;
#[doc = "Graphic MMU LUT entry 555 low"]
pub mod lut555l;
#[doc = "LUT556L register accessor: an alias for `Reg<LUT556L_SPEC>`"]
pub type LUT556L = crate::Reg<lut556l::LUT556L_SPEC>;
#[doc = "Graphic MMU LUT entry 556 low"]
pub mod lut556l;
#[doc = "LUT557L register accessor: an alias for `Reg<LUT557L_SPEC>`"]
pub type LUT557L = crate::Reg<lut557l::LUT557L_SPEC>;
#[doc = "Graphic MMU LUT entry 557 low"]
pub mod lut557l;
#[doc = "LUT558L register accessor: an alias for `Reg<LUT558L_SPEC>`"]
pub type LUT558L = crate::Reg<lut558l::LUT558L_SPEC>;
#[doc = "Graphic MMU LUT entry 558 low"]
pub mod lut558l;
#[doc = "LUT559L register accessor: an alias for `Reg<LUT559L_SPEC>`"]
pub type LUT559L = crate::Reg<lut559l::LUT559L_SPEC>;
#[doc = "Graphic MMU LUT entry 559 low"]
pub mod lut559l;
#[doc = "LUT560L register accessor: an alias for `Reg<LUT560L_SPEC>`"]
pub type LUT560L = crate::Reg<lut560l::LUT560L_SPEC>;
#[doc = "Graphic MMU LUT entry 560 low"]
pub mod lut560l;
#[doc = "LUT561L register accessor: an alias for `Reg<LUT561L_SPEC>`"]
pub type LUT561L = crate::Reg<lut561l::LUT561L_SPEC>;
#[doc = "Graphic MMU LUT entry 561 low"]
pub mod lut561l;
#[doc = "LUT562L register accessor: an alias for `Reg<LUT562L_SPEC>`"]
pub type LUT562L = crate::Reg<lut562l::LUT562L_SPEC>;
#[doc = "Graphic MMU LUT entry 562 low"]
pub mod lut562l;
#[doc = "LUT563L register accessor: an alias for `Reg<LUT563L_SPEC>`"]
pub type LUT563L = crate::Reg<lut563l::LUT563L_SPEC>;
#[doc = "Graphic MMU LUT entry 563 low"]
pub mod lut563l;
#[doc = "LUT564L register accessor: an alias for `Reg<LUT564L_SPEC>`"]
pub type LUT564L = crate::Reg<lut564l::LUT564L_SPEC>;
#[doc = "Graphic MMU LUT entry 564 low"]
pub mod lut564l;
#[doc = "LUT565L register accessor: an alias for `Reg<LUT565L_SPEC>`"]
pub type LUT565L = crate::Reg<lut565l::LUT565L_SPEC>;
#[doc = "Graphic MMU LUT entry 565 low"]
pub mod lut565l;
#[doc = "LUT566L register accessor: an alias for `Reg<LUT566L_SPEC>`"]
pub type LUT566L = crate::Reg<lut566l::LUT566L_SPEC>;
#[doc = "Graphic MMU LUT entry 566 low"]
pub mod lut566l;
#[doc = "LUT567L register accessor: an alias for `Reg<LUT567L_SPEC>`"]
pub type LUT567L = crate::Reg<lut567l::LUT567L_SPEC>;
#[doc = "Graphic MMU LUT entry 567 low"]
pub mod lut567l;
#[doc = "LUT568L register accessor: an alias for `Reg<LUT568L_SPEC>`"]
pub type LUT568L = crate::Reg<lut568l::LUT568L_SPEC>;
#[doc = "Graphic MMU LUT entry 568 low"]
pub mod lut568l;
#[doc = "LUT569L register accessor: an alias for `Reg<LUT569L_SPEC>`"]
pub type LUT569L = crate::Reg<lut569l::LUT569L_SPEC>;
#[doc = "Graphic MMU LUT entry 569 low"]
pub mod lut569l;
#[doc = "LUT570L register accessor: an alias for `Reg<LUT570L_SPEC>`"]
pub type LUT570L = crate::Reg<lut570l::LUT570L_SPEC>;
#[doc = "Graphic MMU LUT entry 570 low"]
pub mod lut570l;
#[doc = "LUT571L register accessor: an alias for `Reg<LUT571L_SPEC>`"]
pub type LUT571L = crate::Reg<lut571l::LUT571L_SPEC>;
#[doc = "Graphic MMU LUT entry 571 low"]
pub mod lut571l;
#[doc = "LUT572L register accessor: an alias for `Reg<LUT572L_SPEC>`"]
pub type LUT572L = crate::Reg<lut572l::LUT572L_SPEC>;
#[doc = "Graphic MMU LUT entry 572 low"]
pub mod lut572l;
#[doc = "LUT573L register accessor: an alias for `Reg<LUT573L_SPEC>`"]
pub type LUT573L = crate::Reg<lut573l::LUT573L_SPEC>;
#[doc = "Graphic MMU LUT entry 573 low"]
pub mod lut573l;
#[doc = "LUT574L register accessor: an alias for `Reg<LUT574L_SPEC>`"]
pub type LUT574L = crate::Reg<lut574l::LUT574L_SPEC>;
#[doc = "Graphic MMU LUT entry 574 low"]
pub mod lut574l;
#[doc = "LUT575L register accessor: an alias for `Reg<LUT575L_SPEC>`"]
pub type LUT575L = crate::Reg<lut575l::LUT575L_SPEC>;
#[doc = "Graphic MMU LUT entry 575 low"]
pub mod lut575l;
#[doc = "LUT576L register accessor: an alias for `Reg<LUT576L_SPEC>`"]
pub type LUT576L = crate::Reg<lut576l::LUT576L_SPEC>;
#[doc = "Graphic MMU LUT entry 576 low"]
pub mod lut576l;
#[doc = "LUT577L register accessor: an alias for `Reg<LUT577L_SPEC>`"]
pub type LUT577L = crate::Reg<lut577l::LUT577L_SPEC>;
#[doc = "Graphic MMU LUT entry 577 low"]
pub mod lut577l;
#[doc = "LUT578L register accessor: an alias for `Reg<LUT578L_SPEC>`"]
pub type LUT578L = crate::Reg<lut578l::LUT578L_SPEC>;
#[doc = "Graphic MMU LUT entry 578 low"]
pub mod lut578l;
#[doc = "LUT579L register accessor: an alias for `Reg<LUT579L_SPEC>`"]
pub type LUT579L = crate::Reg<lut579l::LUT579L_SPEC>;
#[doc = "Graphic MMU LUT entry 579 low"]
pub mod lut579l;
#[doc = "LUT580L register accessor: an alias for `Reg<LUT580L_SPEC>`"]
pub type LUT580L = crate::Reg<lut580l::LUT580L_SPEC>;
#[doc = "Graphic MMU LUT entry 580 low"]
pub mod lut580l;
#[doc = "LUT581L register accessor: an alias for `Reg<LUT581L_SPEC>`"]
pub type LUT581L = crate::Reg<lut581l::LUT581L_SPEC>;
#[doc = "Graphic MMU LUT entry 581 low"]
pub mod lut581l;
#[doc = "LUT582L register accessor: an alias for `Reg<LUT582L_SPEC>`"]
pub type LUT582L = crate::Reg<lut582l::LUT582L_SPEC>;
#[doc = "Graphic MMU LUT entry 582 low"]
pub mod lut582l;
#[doc = "LUT583L register accessor: an alias for `Reg<LUT583L_SPEC>`"]
pub type LUT583L = crate::Reg<lut583l::LUT583L_SPEC>;
#[doc = "Graphic MMU LUT entry 583 low"]
pub mod lut583l;
#[doc = "LUT584L register accessor: an alias for `Reg<LUT584L_SPEC>`"]
pub type LUT584L = crate::Reg<lut584l::LUT584L_SPEC>;
#[doc = "Graphic MMU LUT entry 584 low"]
pub mod lut584l;
#[doc = "LUT585L register accessor: an alias for `Reg<LUT585L_SPEC>`"]
pub type LUT585L = crate::Reg<lut585l::LUT585L_SPEC>;
#[doc = "Graphic MMU LUT entry 585 low"]
pub mod lut585l;
#[doc = "LUT586L register accessor: an alias for `Reg<LUT586L_SPEC>`"]
pub type LUT586L = crate::Reg<lut586l::LUT586L_SPEC>;
#[doc = "Graphic MMU LUT entry 586 low"]
pub mod lut586l;
#[doc = "LUT587L register accessor: an alias for `Reg<LUT587L_SPEC>`"]
pub type LUT587L = crate::Reg<lut587l::LUT587L_SPEC>;
#[doc = "Graphic MMU LUT entry 587 low"]
pub mod lut587l;
#[doc = "LUT588L register accessor: an alias for `Reg<LUT588L_SPEC>`"]
pub type LUT588L = crate::Reg<lut588l::LUT588L_SPEC>;
#[doc = "Graphic MMU LUT entry 588 low"]
pub mod lut588l;
#[doc = "LUT589L register accessor: an alias for `Reg<LUT589L_SPEC>`"]
pub type LUT589L = crate::Reg<lut589l::LUT589L_SPEC>;
#[doc = "Graphic MMU LUT entry 589 low"]
pub mod lut589l;
#[doc = "LUT590L register accessor: an alias for `Reg<LUT590L_SPEC>`"]
pub type LUT590L = crate::Reg<lut590l::LUT590L_SPEC>;
#[doc = "Graphic MMU LUT entry 590 low"]
pub mod lut590l;
#[doc = "LUT591L register accessor: an alias for `Reg<LUT591L_SPEC>`"]
pub type LUT591L = crate::Reg<lut591l::LUT591L_SPEC>;
#[doc = "Graphic MMU LUT entry 591 low"]
pub mod lut591l;
#[doc = "LUT592L register accessor: an alias for `Reg<LUT592L_SPEC>`"]
pub type LUT592L = crate::Reg<lut592l::LUT592L_SPEC>;
#[doc = "Graphic MMU LUT entry 592 low"]
pub mod lut592l;
#[doc = "LUT593L register accessor: an alias for `Reg<LUT593L_SPEC>`"]
pub type LUT593L = crate::Reg<lut593l::LUT593L_SPEC>;
#[doc = "Graphic MMU LUT entry 593 low"]
pub mod lut593l;
#[doc = "LUT594L register accessor: an alias for `Reg<LUT594L_SPEC>`"]
pub type LUT594L = crate::Reg<lut594l::LUT594L_SPEC>;
#[doc = "Graphic MMU LUT entry 594 low"]
pub mod lut594l;
#[doc = "LUT595L register accessor: an alias for `Reg<LUT595L_SPEC>`"]
pub type LUT595L = crate::Reg<lut595l::LUT595L_SPEC>;
#[doc = "Graphic MMU LUT entry 595 low"]
pub mod lut595l;
#[doc = "LUT596L register accessor: an alias for `Reg<LUT596L_SPEC>`"]
pub type LUT596L = crate::Reg<lut596l::LUT596L_SPEC>;
#[doc = "Graphic MMU LUT entry 596 low"]
pub mod lut596l;
#[doc = "LUT597L register accessor: an alias for `Reg<LUT597L_SPEC>`"]
pub type LUT597L = crate::Reg<lut597l::LUT597L_SPEC>;
#[doc = "Graphic MMU LUT entry 597 low"]
pub mod lut597l;
#[doc = "LUT598L register accessor: an alias for `Reg<LUT598L_SPEC>`"]
pub type LUT598L = crate::Reg<lut598l::LUT598L_SPEC>;
#[doc = "Graphic MMU LUT entry 598 low"]
pub mod lut598l;
#[doc = "LUT599L register accessor: an alias for `Reg<LUT599L_SPEC>`"]
pub type LUT599L = crate::Reg<lut599l::LUT599L_SPEC>;
#[doc = "Graphic MMU LUT entry 599 low"]
pub mod lut599l;
#[doc = "LUT600L register accessor: an alias for `Reg<LUT600L_SPEC>`"]
pub type LUT600L = crate::Reg<lut600l::LUT600L_SPEC>;
#[doc = "Graphic MMU LUT entry 600 low"]
pub mod lut600l;
#[doc = "LUT601L register accessor: an alias for `Reg<LUT601L_SPEC>`"]
pub type LUT601L = crate::Reg<lut601l::LUT601L_SPEC>;
#[doc = "Graphic MMU LUT entry 601 low"]
pub mod lut601l;
#[doc = "LUT602L register accessor: an alias for `Reg<LUT602L_SPEC>`"]
pub type LUT602L = crate::Reg<lut602l::LUT602L_SPEC>;
#[doc = "Graphic MMU LUT entry 602 low"]
pub mod lut602l;
#[doc = "LUT603L register accessor: an alias for `Reg<LUT603L_SPEC>`"]
pub type LUT603L = crate::Reg<lut603l::LUT603L_SPEC>;
#[doc = "Graphic MMU LUT entry 603 low"]
pub mod lut603l;
#[doc = "LUT604L register accessor: an alias for `Reg<LUT604L_SPEC>`"]
pub type LUT604L = crate::Reg<lut604l::LUT604L_SPEC>;
#[doc = "Graphic MMU LUT entry 604 low"]
pub mod lut604l;
#[doc = "LUT605L register accessor: an alias for `Reg<LUT605L_SPEC>`"]
pub type LUT605L = crate::Reg<lut605l::LUT605L_SPEC>;
#[doc = "Graphic MMU LUT entry 605 low"]
pub mod lut605l;
#[doc = "LUT606L register accessor: an alias for `Reg<LUT606L_SPEC>`"]
pub type LUT606L = crate::Reg<lut606l::LUT606L_SPEC>;
#[doc = "Graphic MMU LUT entry 606 low"]
pub mod lut606l;
#[doc = "LUT607L register accessor: an alias for `Reg<LUT607L_SPEC>`"]
pub type LUT607L = crate::Reg<lut607l::LUT607L_SPEC>;
#[doc = "Graphic MMU LUT entry 607 low"]
pub mod lut607l;
#[doc = "LUT608L register accessor: an alias for `Reg<LUT608L_SPEC>`"]
pub type LUT608L = crate::Reg<lut608l::LUT608L_SPEC>;
#[doc = "Graphic MMU LUT entry 608 low"]
pub mod lut608l;
#[doc = "LUT609L register accessor: an alias for `Reg<LUT609L_SPEC>`"]
pub type LUT609L = crate::Reg<lut609l::LUT609L_SPEC>;
#[doc = "Graphic MMU LUT entry 609 low"]
pub mod lut609l;
#[doc = "LUT610L register accessor: an alias for `Reg<LUT610L_SPEC>`"]
pub type LUT610L = crate::Reg<lut610l::LUT610L_SPEC>;
#[doc = "Graphic MMU LUT entry 610 low"]
pub mod lut610l;
#[doc = "LUT611L register accessor: an alias for `Reg<LUT611L_SPEC>`"]
pub type LUT611L = crate::Reg<lut611l::LUT611L_SPEC>;
#[doc = "Graphic MMU LUT entry 611 low"]
pub mod lut611l;
#[doc = "LUT612L register accessor: an alias for `Reg<LUT612L_SPEC>`"]
pub type LUT612L = crate::Reg<lut612l::LUT612L_SPEC>;
#[doc = "Graphic MMU LUT entry 612 low"]
pub mod lut612l;
#[doc = "LUT613L register accessor: an alias for `Reg<LUT613L_SPEC>`"]
pub type LUT613L = crate::Reg<lut613l::LUT613L_SPEC>;
#[doc = "Graphic MMU LUT entry 613 low"]
pub mod lut613l;
#[doc = "LUT614L register accessor: an alias for `Reg<LUT614L_SPEC>`"]
pub type LUT614L = crate::Reg<lut614l::LUT614L_SPEC>;
#[doc = "Graphic MMU LUT entry 614 low"]
pub mod lut614l;
#[doc = "LUT615L register accessor: an alias for `Reg<LUT615L_SPEC>`"]
pub type LUT615L = crate::Reg<lut615l::LUT615L_SPEC>;
#[doc = "Graphic MMU LUT entry 615 low"]
pub mod lut615l;
#[doc = "LUT616L register accessor: an alias for `Reg<LUT616L_SPEC>`"]
pub type LUT616L = crate::Reg<lut616l::LUT616L_SPEC>;
#[doc = "Graphic MMU LUT entry 616 low"]
pub mod lut616l;
#[doc = "LUT617L register accessor: an alias for `Reg<LUT617L_SPEC>`"]
pub type LUT617L = crate::Reg<lut617l::LUT617L_SPEC>;
#[doc = "Graphic MMU LUT entry 617 low"]
pub mod lut617l;
#[doc = "LUT618L register accessor: an alias for `Reg<LUT618L_SPEC>`"]
pub type LUT618L = crate::Reg<lut618l::LUT618L_SPEC>;
#[doc = "Graphic MMU LUT entry 618 low"]
pub mod lut618l;
#[doc = "LUT619L register accessor: an alias for `Reg<LUT619L_SPEC>`"]
pub type LUT619L = crate::Reg<lut619l::LUT619L_SPEC>;
#[doc = "Graphic MMU LUT entry 619 low"]
pub mod lut619l;
#[doc = "LUT620L register accessor: an alias for `Reg<LUT620L_SPEC>`"]
pub type LUT620L = crate::Reg<lut620l::LUT620L_SPEC>;
#[doc = "Graphic MMU LUT entry 620 low"]
pub mod lut620l;
#[doc = "LUT621L register accessor: an alias for `Reg<LUT621L_SPEC>`"]
pub type LUT621L = crate::Reg<lut621l::LUT621L_SPEC>;
#[doc = "Graphic MMU LUT entry 621 low"]
pub mod lut621l;
#[doc = "LUT622L register accessor: an alias for `Reg<LUT622L_SPEC>`"]
pub type LUT622L = crate::Reg<lut622l::LUT622L_SPEC>;
#[doc = "Graphic MMU LUT entry 622 low"]
pub mod lut622l;
#[doc = "LUT623L register accessor: an alias for `Reg<LUT623L_SPEC>`"]
pub type LUT623L = crate::Reg<lut623l::LUT623L_SPEC>;
#[doc = "Graphic MMU LUT entry 623 low"]
pub mod lut623l;
#[doc = "LUT624L register accessor: an alias for `Reg<LUT624L_SPEC>`"]
pub type LUT624L = crate::Reg<lut624l::LUT624L_SPEC>;
#[doc = "Graphic MMU LUT entry 624 low"]
pub mod lut624l;
#[doc = "LUT625L register accessor: an alias for `Reg<LUT625L_SPEC>`"]
pub type LUT625L = crate::Reg<lut625l::LUT625L_SPEC>;
#[doc = "Graphic MMU LUT entry 625 low"]
pub mod lut625l;
#[doc = "LUT626L register accessor: an alias for `Reg<LUT626L_SPEC>`"]
pub type LUT626L = crate::Reg<lut626l::LUT626L_SPEC>;
#[doc = "Graphic MMU LUT entry 626 low"]
pub mod lut626l;
#[doc = "LUT627L register accessor: an alias for `Reg<LUT627L_SPEC>`"]
pub type LUT627L = crate::Reg<lut627l::LUT627L_SPEC>;
#[doc = "Graphic MMU LUT entry 627 low"]
pub mod lut627l;
#[doc = "LUT628L register accessor: an alias for `Reg<LUT628L_SPEC>`"]
pub type LUT628L = crate::Reg<lut628l::LUT628L_SPEC>;
#[doc = "Graphic MMU LUT entry 628 low"]
pub mod lut628l;
#[doc = "LUT629L register accessor: an alias for `Reg<LUT629L_SPEC>`"]
pub type LUT629L = crate::Reg<lut629l::LUT629L_SPEC>;
#[doc = "Graphic MMU LUT entry 629 low"]
pub mod lut629l;
#[doc = "LUT630L register accessor: an alias for `Reg<LUT630L_SPEC>`"]
pub type LUT630L = crate::Reg<lut630l::LUT630L_SPEC>;
#[doc = "Graphic MMU LUT entry 630 low"]
pub mod lut630l;
#[doc = "LUT631L register accessor: an alias for `Reg<LUT631L_SPEC>`"]
pub type LUT631L = crate::Reg<lut631l::LUT631L_SPEC>;
#[doc = "Graphic MMU LUT entry 631 low"]
pub mod lut631l;
#[doc = "LUT632L register accessor: an alias for `Reg<LUT632L_SPEC>`"]
pub type LUT632L = crate::Reg<lut632l::LUT632L_SPEC>;
#[doc = "Graphic MMU LUT entry 632 low"]
pub mod lut632l;
#[doc = "LUT633L register accessor: an alias for `Reg<LUT633L_SPEC>`"]
pub type LUT633L = crate::Reg<lut633l::LUT633L_SPEC>;
#[doc = "Graphic MMU LUT entry 633 low"]
pub mod lut633l;
#[doc = "LUT634L register accessor: an alias for `Reg<LUT634L_SPEC>`"]
pub type LUT634L = crate::Reg<lut634l::LUT634L_SPEC>;
#[doc = "Graphic MMU LUT entry 634 low"]
pub mod lut634l;
#[doc = "LUT635L register accessor: an alias for `Reg<LUT635L_SPEC>`"]
pub type LUT635L = crate::Reg<lut635l::LUT635L_SPEC>;
#[doc = "Graphic MMU LUT entry 635 low"]
pub mod lut635l;
#[doc = "LUT636L register accessor: an alias for `Reg<LUT636L_SPEC>`"]
pub type LUT636L = crate::Reg<lut636l::LUT636L_SPEC>;
#[doc = "Graphic MMU LUT entry 636 low"]
pub mod lut636l;
#[doc = "LUT637L register accessor: an alias for `Reg<LUT637L_SPEC>`"]
pub type LUT637L = crate::Reg<lut637l::LUT637L_SPEC>;
#[doc = "Graphic MMU LUT entry 637 low"]
pub mod lut637l;
#[doc = "LUT638L register accessor: an alias for `Reg<LUT638L_SPEC>`"]
pub type LUT638L = crate::Reg<lut638l::LUT638L_SPEC>;
#[doc = "Graphic MMU LUT entry 638 low"]
pub mod lut638l;
#[doc = "LUT639L register accessor: an alias for `Reg<LUT639L_SPEC>`"]
pub type LUT639L = crate::Reg<lut639l::LUT639L_SPEC>;
#[doc = "Graphic MMU LUT entry 639 low"]
pub mod lut639l;
#[doc = "LUT640L register accessor: an alias for `Reg<LUT640L_SPEC>`"]
pub type LUT640L = crate::Reg<lut640l::LUT640L_SPEC>;
#[doc = "Graphic MMU LUT entry 640 low"]
pub mod lut640l;
#[doc = "LUT641L register accessor: an alias for `Reg<LUT641L_SPEC>`"]
pub type LUT641L = crate::Reg<lut641l::LUT641L_SPEC>;
#[doc = "Graphic MMU LUT entry 641 low"]
pub mod lut641l;
#[doc = "LUT642L register accessor: an alias for `Reg<LUT642L_SPEC>`"]
pub type LUT642L = crate::Reg<lut642l::LUT642L_SPEC>;
#[doc = "Graphic MMU LUT entry 642 low"]
pub mod lut642l;
#[doc = "LUT643L register accessor: an alias for `Reg<LUT643L_SPEC>`"]
pub type LUT643L = crate::Reg<lut643l::LUT643L_SPEC>;
#[doc = "Graphic MMU LUT entry 643 low"]
pub mod lut643l;
#[doc = "LUT644L register accessor: an alias for `Reg<LUT644L_SPEC>`"]
pub type LUT644L = crate::Reg<lut644l::LUT644L_SPEC>;
#[doc = "Graphic MMU LUT entry 644 low"]
pub mod lut644l;
#[doc = "LUT645L register accessor: an alias for `Reg<LUT645L_SPEC>`"]
pub type LUT645L = crate::Reg<lut645l::LUT645L_SPEC>;
#[doc = "Graphic MMU LUT entry 645 low"]
pub mod lut645l;
#[doc = "LUT646L register accessor: an alias for `Reg<LUT646L_SPEC>`"]
pub type LUT646L = crate::Reg<lut646l::LUT646L_SPEC>;
#[doc = "Graphic MMU LUT entry 646 low"]
pub mod lut646l;
#[doc = "LUT647L register accessor: an alias for `Reg<LUT647L_SPEC>`"]
pub type LUT647L = crate::Reg<lut647l::LUT647L_SPEC>;
#[doc = "Graphic MMU LUT entry 647 low"]
pub mod lut647l;
#[doc = "LUT648L register accessor: an alias for `Reg<LUT648L_SPEC>`"]
pub type LUT648L = crate::Reg<lut648l::LUT648L_SPEC>;
#[doc = "Graphic MMU LUT entry 648 low"]
pub mod lut648l;
#[doc = "LUT649L register accessor: an alias for `Reg<LUT649L_SPEC>`"]
pub type LUT649L = crate::Reg<lut649l::LUT649L_SPEC>;
#[doc = "Graphic MMU LUT entry 649 low"]
pub mod lut649l;
#[doc = "LUT650L register accessor: an alias for `Reg<LUT650L_SPEC>`"]
pub type LUT650L = crate::Reg<lut650l::LUT650L_SPEC>;
#[doc = "Graphic MMU LUT entry 650 low"]
pub mod lut650l;
#[doc = "LUT651L register accessor: an alias for `Reg<LUT651L_SPEC>`"]
pub type LUT651L = crate::Reg<lut651l::LUT651L_SPEC>;
#[doc = "Graphic MMU LUT entry 651 low"]
pub mod lut651l;
#[doc = "LUT652L register accessor: an alias for `Reg<LUT652L_SPEC>`"]
pub type LUT652L = crate::Reg<lut652l::LUT652L_SPEC>;
#[doc = "Graphic MMU LUT entry 652 low"]
pub mod lut652l;
#[doc = "LUT653L register accessor: an alias for `Reg<LUT653L_SPEC>`"]
pub type LUT653L = crate::Reg<lut653l::LUT653L_SPEC>;
#[doc = "Graphic MMU LUT entry 653 low"]
pub mod lut653l;
#[doc = "LUT654L register accessor: an alias for `Reg<LUT654L_SPEC>`"]
pub type LUT654L = crate::Reg<lut654l::LUT654L_SPEC>;
#[doc = "Graphic MMU LUT entry 654 low"]
pub mod lut654l;
#[doc = "LUT655L register accessor: an alias for `Reg<LUT655L_SPEC>`"]
pub type LUT655L = crate::Reg<lut655l::LUT655L_SPEC>;
#[doc = "Graphic MMU LUT entry 655 low"]
pub mod lut655l;
#[doc = "LUT656L register accessor: an alias for `Reg<LUT656L_SPEC>`"]
pub type LUT656L = crate::Reg<lut656l::LUT656L_SPEC>;
#[doc = "Graphic MMU LUT entry 656 low"]
pub mod lut656l;
#[doc = "LUT657L register accessor: an alias for `Reg<LUT657L_SPEC>`"]
pub type LUT657L = crate::Reg<lut657l::LUT657L_SPEC>;
#[doc = "Graphic MMU LUT entry 657 low"]
pub mod lut657l;
#[doc = "LUT658L register accessor: an alias for `Reg<LUT658L_SPEC>`"]
pub type LUT658L = crate::Reg<lut658l::LUT658L_SPEC>;
#[doc = "Graphic MMU LUT entry 658 low"]
pub mod lut658l;
#[doc = "LUT659L register accessor: an alias for `Reg<LUT659L_SPEC>`"]
pub type LUT659L = crate::Reg<lut659l::LUT659L_SPEC>;
#[doc = "Graphic MMU LUT entry 659 low"]
pub mod lut659l;
#[doc = "LUT660L register accessor: an alias for `Reg<LUT660L_SPEC>`"]
pub type LUT660L = crate::Reg<lut660l::LUT660L_SPEC>;
#[doc = "Graphic MMU LUT entry 660 low"]
pub mod lut660l;
#[doc = "LUT661L register accessor: an alias for `Reg<LUT661L_SPEC>`"]
pub type LUT661L = crate::Reg<lut661l::LUT661L_SPEC>;
#[doc = "Graphic MMU LUT entry 661 low"]
pub mod lut661l;
#[doc = "LUT662L register accessor: an alias for `Reg<LUT662L_SPEC>`"]
pub type LUT662L = crate::Reg<lut662l::LUT662L_SPEC>;
#[doc = "Graphic MMU LUT entry 662 low"]
pub mod lut662l;
#[doc = "LUT663L register accessor: an alias for `Reg<LUT663L_SPEC>`"]
pub type LUT663L = crate::Reg<lut663l::LUT663L_SPEC>;
#[doc = "Graphic MMU LUT entry 663 low"]
pub mod lut663l;
#[doc = "LUT664L register accessor: an alias for `Reg<LUT664L_SPEC>`"]
pub type LUT664L = crate::Reg<lut664l::LUT664L_SPEC>;
#[doc = "Graphic MMU LUT entry 664 low"]
pub mod lut664l;
#[doc = "LUT665L register accessor: an alias for `Reg<LUT665L_SPEC>`"]
pub type LUT665L = crate::Reg<lut665l::LUT665L_SPEC>;
#[doc = "Graphic MMU LUT entry 665 low"]
pub mod lut665l;
#[doc = "LUT666L register accessor: an alias for `Reg<LUT666L_SPEC>`"]
pub type LUT666L = crate::Reg<lut666l::LUT666L_SPEC>;
#[doc = "Graphic MMU LUT entry 666 low"]
pub mod lut666l;
#[doc = "LUT667L register accessor: an alias for `Reg<LUT667L_SPEC>`"]
pub type LUT667L = crate::Reg<lut667l::LUT667L_SPEC>;
#[doc = "Graphic MMU LUT entry 667 low"]
pub mod lut667l;
#[doc = "LUT668L register accessor: an alias for `Reg<LUT668L_SPEC>`"]
pub type LUT668L = crate::Reg<lut668l::LUT668L_SPEC>;
#[doc = "Graphic MMU LUT entry 668 low"]
pub mod lut668l;
#[doc = "LUT669L register accessor: an alias for `Reg<LUT669L_SPEC>`"]
pub type LUT669L = crate::Reg<lut669l::LUT669L_SPEC>;
#[doc = "Graphic MMU LUT entry 669 low"]
pub mod lut669l;
#[doc = "LUT670L register accessor: an alias for `Reg<LUT670L_SPEC>`"]
pub type LUT670L = crate::Reg<lut670l::LUT670L_SPEC>;
#[doc = "Graphic MMU LUT entry 670 low"]
pub mod lut670l;
#[doc = "LUT671L register accessor: an alias for `Reg<LUT671L_SPEC>`"]
pub type LUT671L = crate::Reg<lut671l::LUT671L_SPEC>;
#[doc = "Graphic MMU LUT entry 671 low"]
pub mod lut671l;
#[doc = "LUT672L register accessor: an alias for `Reg<LUT672L_SPEC>`"]
pub type LUT672L = crate::Reg<lut672l::LUT672L_SPEC>;
#[doc = "Graphic MMU LUT entry 672 low"]
pub mod lut672l;
#[doc = "LUT673L register accessor: an alias for `Reg<LUT673L_SPEC>`"]
pub type LUT673L = crate::Reg<lut673l::LUT673L_SPEC>;
#[doc = "Graphic MMU LUT entry 673 low"]
pub mod lut673l;
#[doc = "LUT674L register accessor: an alias for `Reg<LUT674L_SPEC>`"]
pub type LUT674L = crate::Reg<lut674l::LUT674L_SPEC>;
#[doc = "Graphic MMU LUT entry 674 low"]
pub mod lut674l;
#[doc = "LUT675L register accessor: an alias for `Reg<LUT675L_SPEC>`"]
pub type LUT675L = crate::Reg<lut675l::LUT675L_SPEC>;
#[doc = "Graphic MMU LUT entry 675 low"]
pub mod lut675l;
#[doc = "LUT676L register accessor: an alias for `Reg<LUT676L_SPEC>`"]
pub type LUT676L = crate::Reg<lut676l::LUT676L_SPEC>;
#[doc = "Graphic MMU LUT entry 676 low"]
pub mod lut676l;
#[doc = "LUT677L register accessor: an alias for `Reg<LUT677L_SPEC>`"]
pub type LUT677L = crate::Reg<lut677l::LUT677L_SPEC>;
#[doc = "Graphic MMU LUT entry 677 low"]
pub mod lut677l;
#[doc = "LUT678L register accessor: an alias for `Reg<LUT678L_SPEC>`"]
pub type LUT678L = crate::Reg<lut678l::LUT678L_SPEC>;
#[doc = "Graphic MMU LUT entry 678 low"]
pub mod lut678l;
#[doc = "LUT679L register accessor: an alias for `Reg<LUT679L_SPEC>`"]
pub type LUT679L = crate::Reg<lut679l::LUT679L_SPEC>;
#[doc = "Graphic MMU LUT entry 679 low"]
pub mod lut679l;
#[doc = "LUT680L register accessor: an alias for `Reg<LUT680L_SPEC>`"]
pub type LUT680L = crate::Reg<lut680l::LUT680L_SPEC>;
#[doc = "Graphic MMU LUT entry 680 low"]
pub mod lut680l;
#[doc = "LUT681L register accessor: an alias for `Reg<LUT681L_SPEC>`"]
pub type LUT681L = crate::Reg<lut681l::LUT681L_SPEC>;
#[doc = "Graphic MMU LUT entry 681 low"]
pub mod lut681l;
#[doc = "LUT682L register accessor: an alias for `Reg<LUT682L_SPEC>`"]
pub type LUT682L = crate::Reg<lut682l::LUT682L_SPEC>;
#[doc = "Graphic MMU LUT entry 682 low"]
pub mod lut682l;
#[doc = "LUT683L register accessor: an alias for `Reg<LUT683L_SPEC>`"]
pub type LUT683L = crate::Reg<lut683l::LUT683L_SPEC>;
#[doc = "Graphic MMU LUT entry 683 low"]
pub mod lut683l;
#[doc = "LUT684L register accessor: an alias for `Reg<LUT684L_SPEC>`"]
pub type LUT684L = crate::Reg<lut684l::LUT684L_SPEC>;
#[doc = "Graphic MMU LUT entry 684 low"]
pub mod lut684l;
#[doc = "LUT685L register accessor: an alias for `Reg<LUT685L_SPEC>`"]
pub type LUT685L = crate::Reg<lut685l::LUT685L_SPEC>;
#[doc = "Graphic MMU LUT entry 685 low"]
pub mod lut685l;
#[doc = "LUT686L register accessor: an alias for `Reg<LUT686L_SPEC>`"]
pub type LUT686L = crate::Reg<lut686l::LUT686L_SPEC>;
#[doc = "Graphic MMU LUT entry 686 low"]
pub mod lut686l;
#[doc = "LUT687L register accessor: an alias for `Reg<LUT687L_SPEC>`"]
pub type LUT687L = crate::Reg<lut687l::LUT687L_SPEC>;
#[doc = "Graphic MMU LUT entry 687 low"]
pub mod lut687l;
#[doc = "LUT688L register accessor: an alias for `Reg<LUT688L_SPEC>`"]
pub type LUT688L = crate::Reg<lut688l::LUT688L_SPEC>;
#[doc = "Graphic MMU LUT entry 688 low"]
pub mod lut688l;
#[doc = "LUT689L register accessor: an alias for `Reg<LUT689L_SPEC>`"]
pub type LUT689L = crate::Reg<lut689l::LUT689L_SPEC>;
#[doc = "Graphic MMU LUT entry 689 low"]
pub mod lut689l;
#[doc = "LUT690L register accessor: an alias for `Reg<LUT690L_SPEC>`"]
pub type LUT690L = crate::Reg<lut690l::LUT690L_SPEC>;
#[doc = "Graphic MMU LUT entry 690 low"]
pub mod lut690l;
#[doc = "LUT691L register accessor: an alias for `Reg<LUT691L_SPEC>`"]
pub type LUT691L = crate::Reg<lut691l::LUT691L_SPEC>;
#[doc = "Graphic MMU LUT entry 691 low"]
pub mod lut691l;
#[doc = "LUT692L register accessor: an alias for `Reg<LUT692L_SPEC>`"]
pub type LUT692L = crate::Reg<lut692l::LUT692L_SPEC>;
#[doc = "Graphic MMU LUT entry 692 low"]
pub mod lut692l;
#[doc = "LUT693L register accessor: an alias for `Reg<LUT693L_SPEC>`"]
pub type LUT693L = crate::Reg<lut693l::LUT693L_SPEC>;
#[doc = "Graphic MMU LUT entry 693 low"]
pub mod lut693l;
#[doc = "LUT694L register accessor: an alias for `Reg<LUT694L_SPEC>`"]
pub type LUT694L = crate::Reg<lut694l::LUT694L_SPEC>;
#[doc = "Graphic MMU LUT entry 694 low"]
pub mod lut694l;
#[doc = "LUT695L register accessor: an alias for `Reg<LUT695L_SPEC>`"]
pub type LUT695L = crate::Reg<lut695l::LUT695L_SPEC>;
#[doc = "Graphic MMU LUT entry 695 low"]
pub mod lut695l;
#[doc = "LUT696L register accessor: an alias for `Reg<LUT696L_SPEC>`"]
pub type LUT696L = crate::Reg<lut696l::LUT696L_SPEC>;
#[doc = "Graphic MMU LUT entry 696 low"]
pub mod lut696l;
#[doc = "LUT697L register accessor: an alias for `Reg<LUT697L_SPEC>`"]
pub type LUT697L = crate::Reg<lut697l::LUT697L_SPEC>;
#[doc = "Graphic MMU LUT entry 697 low"]
pub mod lut697l;
#[doc = "LUT698L register accessor: an alias for `Reg<LUT698L_SPEC>`"]
pub type LUT698L = crate::Reg<lut698l::LUT698L_SPEC>;
#[doc = "Graphic MMU LUT entry 698 low"]
pub mod lut698l;
#[doc = "LUT699L register accessor: an alias for `Reg<LUT699L_SPEC>`"]
pub type LUT699L = crate::Reg<lut699l::LUT699L_SPEC>;
#[doc = "Graphic MMU LUT entry 699 low"]
pub mod lut699l;
#[doc = "LUT700L register accessor: an alias for `Reg<LUT700L_SPEC>`"]
pub type LUT700L = crate::Reg<lut700l::LUT700L_SPEC>;
#[doc = "Graphic MMU LUT entry 700 low"]
pub mod lut700l;
#[doc = "LUT701L register accessor: an alias for `Reg<LUT701L_SPEC>`"]
pub type LUT701L = crate::Reg<lut701l::LUT701L_SPEC>;
#[doc = "Graphic MMU LUT entry 701 low"]
pub mod lut701l;
#[doc = "LUT702L register accessor: an alias for `Reg<LUT702L_SPEC>`"]
pub type LUT702L = crate::Reg<lut702l::LUT702L_SPEC>;
#[doc = "Graphic MMU LUT entry 702 low"]
pub mod lut702l;
#[doc = "LUT703L register accessor: an alias for `Reg<LUT703L_SPEC>`"]
pub type LUT703L = crate::Reg<lut703l::LUT703L_SPEC>;
#[doc = "Graphic MMU LUT entry 703 low"]
pub mod lut703l;
#[doc = "LUT704L register accessor: an alias for `Reg<LUT704L_SPEC>`"]
pub type LUT704L = crate::Reg<lut704l::LUT704L_SPEC>;
#[doc = "Graphic MMU LUT entry 704 low"]
pub mod lut704l;
#[doc = "LUT705L register accessor: an alias for `Reg<LUT705L_SPEC>`"]
pub type LUT705L = crate::Reg<lut705l::LUT705L_SPEC>;
#[doc = "Graphic MMU LUT entry 705 low"]
pub mod lut705l;
#[doc = "LUT706L register accessor: an alias for `Reg<LUT706L_SPEC>`"]
pub type LUT706L = crate::Reg<lut706l::LUT706L_SPEC>;
#[doc = "Graphic MMU LUT entry 706 low"]
pub mod lut706l;
#[doc = "LUT707L register accessor: an alias for `Reg<LUT707L_SPEC>`"]
pub type LUT707L = crate::Reg<lut707l::LUT707L_SPEC>;
#[doc = "Graphic MMU LUT entry 707 low"]
pub mod lut707l;
#[doc = "LUT708L register accessor: an alias for `Reg<LUT708L_SPEC>`"]
pub type LUT708L = crate::Reg<lut708l::LUT708L_SPEC>;
#[doc = "Graphic MMU LUT entry 708 low"]
pub mod lut708l;
#[doc = "LUT709L register accessor: an alias for `Reg<LUT709L_SPEC>`"]
pub type LUT709L = crate::Reg<lut709l::LUT709L_SPEC>;
#[doc = "Graphic MMU LUT entry 709 low"]
pub mod lut709l;
#[doc = "LUT710L register accessor: an alias for `Reg<LUT710L_SPEC>`"]
pub type LUT710L = crate::Reg<lut710l::LUT710L_SPEC>;
#[doc = "Graphic MMU LUT entry 710 low"]
pub mod lut710l;
#[doc = "LUT711L register accessor: an alias for `Reg<LUT711L_SPEC>`"]
pub type LUT711L = crate::Reg<lut711l::LUT711L_SPEC>;
#[doc = "Graphic MMU LUT entry 711 low"]
pub mod lut711l;
#[doc = "LUT712L register accessor: an alias for `Reg<LUT712L_SPEC>`"]
pub type LUT712L = crate::Reg<lut712l::LUT712L_SPEC>;
#[doc = "Graphic MMU LUT entry 712 low"]
pub mod lut712l;
#[doc = "LUT713L register accessor: an alias for `Reg<LUT713L_SPEC>`"]
pub type LUT713L = crate::Reg<lut713l::LUT713L_SPEC>;
#[doc = "Graphic MMU LUT entry 713 low"]
pub mod lut713l;
#[doc = "LUT714L register accessor: an alias for `Reg<LUT714L_SPEC>`"]
pub type LUT714L = crate::Reg<lut714l::LUT714L_SPEC>;
#[doc = "Graphic MMU LUT entry 714 low"]
pub mod lut714l;
#[doc = "LUT715L register accessor: an alias for `Reg<LUT715L_SPEC>`"]
pub type LUT715L = crate::Reg<lut715l::LUT715L_SPEC>;
#[doc = "Graphic MMU LUT entry 715 low"]
pub mod lut715l;
#[doc = "LUT716L register accessor: an alias for `Reg<LUT716L_SPEC>`"]
pub type LUT716L = crate::Reg<lut716l::LUT716L_SPEC>;
#[doc = "Graphic MMU LUT entry 716 low"]
pub mod lut716l;
#[doc = "LUT717L register accessor: an alias for `Reg<LUT717L_SPEC>`"]
pub type LUT717L = crate::Reg<lut717l::LUT717L_SPEC>;
#[doc = "Graphic MMU LUT entry 717 low"]
pub mod lut717l;
#[doc = "LUT718L register accessor: an alias for `Reg<LUT718L_SPEC>`"]
pub type LUT718L = crate::Reg<lut718l::LUT718L_SPEC>;
#[doc = "Graphic MMU LUT entry 718 low"]
pub mod lut718l;
#[doc = "LUT719L register accessor: an alias for `Reg<LUT719L_SPEC>`"]
pub type LUT719L = crate::Reg<lut719l::LUT719L_SPEC>;
#[doc = "Graphic MMU LUT entry 719 low"]
pub mod lut719l;
#[doc = "LUT720L register accessor: an alias for `Reg<LUT720L_SPEC>`"]
pub type LUT720L = crate::Reg<lut720l::LUT720L_SPEC>;
#[doc = "Graphic MMU LUT entry 720 low"]
pub mod lut720l;
#[doc = "LUT721L register accessor: an alias for `Reg<LUT721L_SPEC>`"]
pub type LUT721L = crate::Reg<lut721l::LUT721L_SPEC>;
#[doc = "Graphic MMU LUT entry 721 low"]
pub mod lut721l;
#[doc = "LUT722L register accessor: an alias for `Reg<LUT722L_SPEC>`"]
pub type LUT722L = crate::Reg<lut722l::LUT722L_SPEC>;
#[doc = "Graphic MMU LUT entry 722 low"]
pub mod lut722l;
#[doc = "LUT723L register accessor: an alias for `Reg<LUT723L_SPEC>`"]
pub type LUT723L = crate::Reg<lut723l::LUT723L_SPEC>;
#[doc = "Graphic MMU LUT entry 723 low"]
pub mod lut723l;
#[doc = "LUT724L register accessor: an alias for `Reg<LUT724L_SPEC>`"]
pub type LUT724L = crate::Reg<lut724l::LUT724L_SPEC>;
#[doc = "Graphic MMU LUT entry 724 low"]
pub mod lut724l;
#[doc = "LUT725L register accessor: an alias for `Reg<LUT725L_SPEC>`"]
pub type LUT725L = crate::Reg<lut725l::LUT725L_SPEC>;
#[doc = "Graphic MMU LUT entry 725 low"]
pub mod lut725l;
#[doc = "LUT726L register accessor: an alias for `Reg<LUT726L_SPEC>`"]
pub type LUT726L = crate::Reg<lut726l::LUT726L_SPEC>;
#[doc = "Graphic MMU LUT entry 726 low"]
pub mod lut726l;
#[doc = "LUT727L register accessor: an alias for `Reg<LUT727L_SPEC>`"]
pub type LUT727L = crate::Reg<lut727l::LUT727L_SPEC>;
#[doc = "Graphic MMU LUT entry 727 low"]
pub mod lut727l;
#[doc = "LUT728L register accessor: an alias for `Reg<LUT728L_SPEC>`"]
pub type LUT728L = crate::Reg<lut728l::LUT728L_SPEC>;
#[doc = "Graphic MMU LUT entry 728 low"]
pub mod lut728l;
#[doc = "LUT729L register accessor: an alias for `Reg<LUT729L_SPEC>`"]
pub type LUT729L = crate::Reg<lut729l::LUT729L_SPEC>;
#[doc = "Graphic MMU LUT entry 729 low"]
pub mod lut729l;
#[doc = "LUT730L register accessor: an alias for `Reg<LUT730L_SPEC>`"]
pub type LUT730L = crate::Reg<lut730l::LUT730L_SPEC>;
#[doc = "Graphic MMU LUT entry 730 low"]
pub mod lut730l;
#[doc = "LUT731L register accessor: an alias for `Reg<LUT731L_SPEC>`"]
pub type LUT731L = crate::Reg<lut731l::LUT731L_SPEC>;
#[doc = "Graphic MMU LUT entry 731 low"]
pub mod lut731l;
#[doc = "LUT732L register accessor: an alias for `Reg<LUT732L_SPEC>`"]
pub type LUT732L = crate::Reg<lut732l::LUT732L_SPEC>;
#[doc = "Graphic MMU LUT entry 732 low"]
pub mod lut732l;
#[doc = "LUT733L register accessor: an alias for `Reg<LUT733L_SPEC>`"]
pub type LUT733L = crate::Reg<lut733l::LUT733L_SPEC>;
#[doc = "Graphic MMU LUT entry 733 low"]
pub mod lut733l;
#[doc = "LUT734L register accessor: an alias for `Reg<LUT734L_SPEC>`"]
pub type LUT734L = crate::Reg<lut734l::LUT734L_SPEC>;
#[doc = "Graphic MMU LUT entry 734 low"]
pub mod lut734l;
#[doc = "LUT735L register accessor: an alias for `Reg<LUT735L_SPEC>`"]
pub type LUT735L = crate::Reg<lut735l::LUT735L_SPEC>;
#[doc = "Graphic MMU LUT entry 735 low"]
pub mod lut735l;
#[doc = "LUT736L register accessor: an alias for `Reg<LUT736L_SPEC>`"]
pub type LUT736L = crate::Reg<lut736l::LUT736L_SPEC>;
#[doc = "Graphic MMU LUT entry 736 low"]
pub mod lut736l;
#[doc = "LUT737L register accessor: an alias for `Reg<LUT737L_SPEC>`"]
pub type LUT737L = crate::Reg<lut737l::LUT737L_SPEC>;
#[doc = "Graphic MMU LUT entry 737 low"]
pub mod lut737l;
#[doc = "LUT738L register accessor: an alias for `Reg<LUT738L_SPEC>`"]
pub type LUT738L = crate::Reg<lut738l::LUT738L_SPEC>;
#[doc = "Graphic MMU LUT entry 738 low"]
pub mod lut738l;
#[doc = "LUT739L register accessor: an alias for `Reg<LUT739L_SPEC>`"]
pub type LUT739L = crate::Reg<lut739l::LUT739L_SPEC>;
#[doc = "Graphic MMU LUT entry 739 low"]
pub mod lut739l;
#[doc = "LUT740L register accessor: an alias for `Reg<LUT740L_SPEC>`"]
pub type LUT740L = crate::Reg<lut740l::LUT740L_SPEC>;
#[doc = "Graphic MMU LUT entry 740 low"]
pub mod lut740l;
#[doc = "LUT741L register accessor: an alias for `Reg<LUT741L_SPEC>`"]
pub type LUT741L = crate::Reg<lut741l::LUT741L_SPEC>;
#[doc = "Graphic MMU LUT entry 741 low"]
pub mod lut741l;
#[doc = "LUT742L register accessor: an alias for `Reg<LUT742L_SPEC>`"]
pub type LUT742L = crate::Reg<lut742l::LUT742L_SPEC>;
#[doc = "Graphic MMU LUT entry 742 low"]
pub mod lut742l;
#[doc = "LUT743L register accessor: an alias for `Reg<LUT743L_SPEC>`"]
pub type LUT743L = crate::Reg<lut743l::LUT743L_SPEC>;
#[doc = "Graphic MMU LUT entry 743 low"]
pub mod lut743l;
#[doc = "LUT744L register accessor: an alias for `Reg<LUT744L_SPEC>`"]
pub type LUT744L = crate::Reg<lut744l::LUT744L_SPEC>;
#[doc = "Graphic MMU LUT entry 744 low"]
pub mod lut744l;
#[doc = "LUT745L register accessor: an alias for `Reg<LUT745L_SPEC>`"]
pub type LUT745L = crate::Reg<lut745l::LUT745L_SPEC>;
#[doc = "Graphic MMU LUT entry 745 low"]
pub mod lut745l;
#[doc = "LUT746L register accessor: an alias for `Reg<LUT746L_SPEC>`"]
pub type LUT746L = crate::Reg<lut746l::LUT746L_SPEC>;
#[doc = "Graphic MMU LUT entry 746 low"]
pub mod lut746l;
#[doc = "LUT747L register accessor: an alias for `Reg<LUT747L_SPEC>`"]
pub type LUT747L = crate::Reg<lut747l::LUT747L_SPEC>;
#[doc = "Graphic MMU LUT entry 747 low"]
pub mod lut747l;
#[doc = "LUT748L register accessor: an alias for `Reg<LUT748L_SPEC>`"]
pub type LUT748L = crate::Reg<lut748l::LUT748L_SPEC>;
#[doc = "Graphic MMU LUT entry 748 low"]
pub mod lut748l;
#[doc = "LUT749L register accessor: an alias for `Reg<LUT749L_SPEC>`"]
pub type LUT749L = crate::Reg<lut749l::LUT749L_SPEC>;
#[doc = "Graphic MMU LUT entry 749 low"]
pub mod lut749l;
#[doc = "LUT750L register accessor: an alias for `Reg<LUT750L_SPEC>`"]
pub type LUT750L = crate::Reg<lut750l::LUT750L_SPEC>;
#[doc = "Graphic MMU LUT entry 750 low"]
pub mod lut750l;
#[doc = "LUT751L register accessor: an alias for `Reg<LUT751L_SPEC>`"]
pub type LUT751L = crate::Reg<lut751l::LUT751L_SPEC>;
#[doc = "Graphic MMU LUT entry 751 low"]
pub mod lut751l;
#[doc = "LUT752L register accessor: an alias for `Reg<LUT752L_SPEC>`"]
pub type LUT752L = crate::Reg<lut752l::LUT752L_SPEC>;
#[doc = "Graphic MMU LUT entry 752 low"]
pub mod lut752l;
#[doc = "LUT753L register accessor: an alias for `Reg<LUT753L_SPEC>`"]
pub type LUT753L = crate::Reg<lut753l::LUT753L_SPEC>;
#[doc = "Graphic MMU LUT entry 753 low"]
pub mod lut753l;
#[doc = "LUT754L register accessor: an alias for `Reg<LUT754L_SPEC>`"]
pub type LUT754L = crate::Reg<lut754l::LUT754L_SPEC>;
#[doc = "Graphic MMU LUT entry 754 low"]
pub mod lut754l;
#[doc = "LUT755L register accessor: an alias for `Reg<LUT755L_SPEC>`"]
pub type LUT755L = crate::Reg<lut755l::LUT755L_SPEC>;
#[doc = "Graphic MMU LUT entry 755 low"]
pub mod lut755l;
#[doc = "LUT756L register accessor: an alias for `Reg<LUT756L_SPEC>`"]
pub type LUT756L = crate::Reg<lut756l::LUT756L_SPEC>;
#[doc = "Graphic MMU LUT entry 756 low"]
pub mod lut756l;
#[doc = "LUT757L register accessor: an alias for `Reg<LUT757L_SPEC>`"]
pub type LUT757L = crate::Reg<lut757l::LUT757L_SPEC>;
#[doc = "Graphic MMU LUT entry 757 low"]
pub mod lut757l;
#[doc = "LUT758L register accessor: an alias for `Reg<LUT758L_SPEC>`"]
pub type LUT758L = crate::Reg<lut758l::LUT758L_SPEC>;
#[doc = "Graphic MMU LUT entry 758 low"]
pub mod lut758l;
#[doc = "LUT759L register accessor: an alias for `Reg<LUT759L_SPEC>`"]
pub type LUT759L = crate::Reg<lut759l::LUT759L_SPEC>;
#[doc = "Graphic MMU LUT entry 759 low"]
pub mod lut759l;
#[doc = "LUT760L register accessor: an alias for `Reg<LUT760L_SPEC>`"]
pub type LUT760L = crate::Reg<lut760l::LUT760L_SPEC>;
#[doc = "Graphic MMU LUT entry 760 low"]
pub mod lut760l;
#[doc = "LUT761L register accessor: an alias for `Reg<LUT761L_SPEC>`"]
pub type LUT761L = crate::Reg<lut761l::LUT761L_SPEC>;
#[doc = "Graphic MMU LUT entry 761 low"]
pub mod lut761l;
#[doc = "LUT762L register accessor: an alias for `Reg<LUT762L_SPEC>`"]
pub type LUT762L = crate::Reg<lut762l::LUT762L_SPEC>;
#[doc = "Graphic MMU LUT entry 762 low"]
pub mod lut762l;
#[doc = "LUT763L register accessor: an alias for `Reg<LUT763L_SPEC>`"]
pub type LUT763L = crate::Reg<lut763l::LUT763L_SPEC>;
#[doc = "Graphic MMU LUT entry 763 low"]
pub mod lut763l;
#[doc = "LUT764L register accessor: an alias for `Reg<LUT764L_SPEC>`"]
pub type LUT764L = crate::Reg<lut764l::LUT764L_SPEC>;
#[doc = "Graphic MMU LUT entry 764 low"]
pub mod lut764l;
#[doc = "LUT765L register accessor: an alias for `Reg<LUT765L_SPEC>`"]
pub type LUT765L = crate::Reg<lut765l::LUT765L_SPEC>;
#[doc = "Graphic MMU LUT entry 765 low"]
pub mod lut765l;
#[doc = "LUT766L register accessor: an alias for `Reg<LUT766L_SPEC>`"]
pub type LUT766L = crate::Reg<lut766l::LUT766L_SPEC>;
#[doc = "Graphic MMU LUT entry 766 low"]
pub mod lut766l;
#[doc = "LUT767L register accessor: an alias for `Reg<LUT767L_SPEC>`"]
pub type LUT767L = crate::Reg<lut767l::LUT767L_SPEC>;
#[doc = "Graphic MMU LUT entry 767 low"]
pub mod lut767l;
#[doc = "LUT768L register accessor: an alias for `Reg<LUT768L_SPEC>`"]
pub type LUT768L = crate::Reg<lut768l::LUT768L_SPEC>;
#[doc = "Graphic MMU LUT entry 768 low"]
pub mod lut768l;
#[doc = "LUT769L register accessor: an alias for `Reg<LUT769L_SPEC>`"]
pub type LUT769L = crate::Reg<lut769l::LUT769L_SPEC>;
#[doc = "Graphic MMU LUT entry 769 low"]
pub mod lut769l;
#[doc = "LUT770L register accessor: an alias for `Reg<LUT770L_SPEC>`"]
pub type LUT770L = crate::Reg<lut770l::LUT770L_SPEC>;
#[doc = "Graphic MMU LUT entry 770 low"]
pub mod lut770l;
#[doc = "LUT771L register accessor: an alias for `Reg<LUT771L_SPEC>`"]
pub type LUT771L = crate::Reg<lut771l::LUT771L_SPEC>;
#[doc = "Graphic MMU LUT entry 771 low"]
pub mod lut771l;
#[doc = "LUT772L register accessor: an alias for `Reg<LUT772L_SPEC>`"]
pub type LUT772L = crate::Reg<lut772l::LUT772L_SPEC>;
#[doc = "Graphic MMU LUT entry 772 low"]
pub mod lut772l;
#[doc = "LUT773L register accessor: an alias for `Reg<LUT773L_SPEC>`"]
pub type LUT773L = crate::Reg<lut773l::LUT773L_SPEC>;
#[doc = "Graphic MMU LUT entry 773 low"]
pub mod lut773l;
#[doc = "LUT774L register accessor: an alias for `Reg<LUT774L_SPEC>`"]
pub type LUT774L = crate::Reg<lut774l::LUT774L_SPEC>;
#[doc = "Graphic MMU LUT entry 774 low"]
pub mod lut774l;
#[doc = "LUT775L register accessor: an alias for `Reg<LUT775L_SPEC>`"]
pub type LUT775L = crate::Reg<lut775l::LUT775L_SPEC>;
#[doc = "Graphic MMU LUT entry 775 low"]
pub mod lut775l;
#[doc = "LUT776L register accessor: an alias for `Reg<LUT776L_SPEC>`"]
pub type LUT776L = crate::Reg<lut776l::LUT776L_SPEC>;
#[doc = "Graphic MMU LUT entry 776 low"]
pub mod lut776l;
#[doc = "LUT777L register accessor: an alias for `Reg<LUT777L_SPEC>`"]
pub type LUT777L = crate::Reg<lut777l::LUT777L_SPEC>;
#[doc = "Graphic MMU LUT entry 777 low"]
pub mod lut777l;
#[doc = "LUT778L register accessor: an alias for `Reg<LUT778L_SPEC>`"]
pub type LUT778L = crate::Reg<lut778l::LUT778L_SPEC>;
#[doc = "Graphic MMU LUT entry 778 low"]
pub mod lut778l;
#[doc = "LUT779L register accessor: an alias for `Reg<LUT779L_SPEC>`"]
pub type LUT779L = crate::Reg<lut779l::LUT779L_SPEC>;
#[doc = "Graphic MMU LUT entry 779 low"]
pub mod lut779l;
#[doc = "LUT780L register accessor: an alias for `Reg<LUT780L_SPEC>`"]
pub type LUT780L = crate::Reg<lut780l::LUT780L_SPEC>;
#[doc = "Graphic MMU LUT entry 780 low"]
pub mod lut780l;
#[doc = "LUT781L register accessor: an alias for `Reg<LUT781L_SPEC>`"]
pub type LUT781L = crate::Reg<lut781l::LUT781L_SPEC>;
#[doc = "Graphic MMU LUT entry 781 low"]
pub mod lut781l;
#[doc = "LUT782L register accessor: an alias for `Reg<LUT782L_SPEC>`"]
pub type LUT782L = crate::Reg<lut782l::LUT782L_SPEC>;
#[doc = "Graphic MMU LUT entry 782 low"]
pub mod lut782l;
#[doc = "LUT783L register accessor: an alias for `Reg<LUT783L_SPEC>`"]
pub type LUT783L = crate::Reg<lut783l::LUT783L_SPEC>;
#[doc = "Graphic MMU LUT entry 783 low"]
pub mod lut783l;
#[doc = "LUT784L register accessor: an alias for `Reg<LUT784L_SPEC>`"]
pub type LUT784L = crate::Reg<lut784l::LUT784L_SPEC>;
#[doc = "Graphic MMU LUT entry 784 low"]
pub mod lut784l;
#[doc = "LUT785L register accessor: an alias for `Reg<LUT785L_SPEC>`"]
pub type LUT785L = crate::Reg<lut785l::LUT785L_SPEC>;
#[doc = "Graphic MMU LUT entry 785 low"]
pub mod lut785l;
#[doc = "LUT786L register accessor: an alias for `Reg<LUT786L_SPEC>`"]
pub type LUT786L = crate::Reg<lut786l::LUT786L_SPEC>;
#[doc = "Graphic MMU LUT entry 786 low"]
pub mod lut786l;
#[doc = "LUT787L register accessor: an alias for `Reg<LUT787L_SPEC>`"]
pub type LUT787L = crate::Reg<lut787l::LUT787L_SPEC>;
#[doc = "Graphic MMU LUT entry 787 low"]
pub mod lut787l;
#[doc = "LUT788L register accessor: an alias for `Reg<LUT788L_SPEC>`"]
pub type LUT788L = crate::Reg<lut788l::LUT788L_SPEC>;
#[doc = "Graphic MMU LUT entry 788 low"]
pub mod lut788l;
#[doc = "LUT789L register accessor: an alias for `Reg<LUT789L_SPEC>`"]
pub type LUT789L = crate::Reg<lut789l::LUT789L_SPEC>;
#[doc = "Graphic MMU LUT entry 789 low"]
pub mod lut789l;
#[doc = "LUT790L register accessor: an alias for `Reg<LUT790L_SPEC>`"]
pub type LUT790L = crate::Reg<lut790l::LUT790L_SPEC>;
#[doc = "Graphic MMU LUT entry 790 low"]
pub mod lut790l;
#[doc = "LUT791L register accessor: an alias for `Reg<LUT791L_SPEC>`"]
pub type LUT791L = crate::Reg<lut791l::LUT791L_SPEC>;
#[doc = "Graphic MMU LUT entry 791 low"]
pub mod lut791l;
#[doc = "LUT792L register accessor: an alias for `Reg<LUT792L_SPEC>`"]
pub type LUT792L = crate::Reg<lut792l::LUT792L_SPEC>;
#[doc = "Graphic MMU LUT entry 792 low"]
pub mod lut792l;
#[doc = "LUT793L register accessor: an alias for `Reg<LUT793L_SPEC>`"]
pub type LUT793L = crate::Reg<lut793l::LUT793L_SPEC>;
#[doc = "Graphic MMU LUT entry 793 low"]
pub mod lut793l;
#[doc = "LUT794L register accessor: an alias for `Reg<LUT794L_SPEC>`"]
pub type LUT794L = crate::Reg<lut794l::LUT794L_SPEC>;
#[doc = "Graphic MMU LUT entry 794 low"]
pub mod lut794l;
#[doc = "LUT795L register accessor: an alias for `Reg<LUT795L_SPEC>`"]
pub type LUT795L = crate::Reg<lut795l::LUT795L_SPEC>;
#[doc = "Graphic MMU LUT entry 795 low"]
pub mod lut795l;
#[doc = "LUT796L register accessor: an alias for `Reg<LUT796L_SPEC>`"]
pub type LUT796L = crate::Reg<lut796l::LUT796L_SPEC>;
#[doc = "Graphic MMU LUT entry 796 low"]
pub mod lut796l;
#[doc = "LUT797L register accessor: an alias for `Reg<LUT797L_SPEC>`"]
pub type LUT797L = crate::Reg<lut797l::LUT797L_SPEC>;
#[doc = "Graphic MMU LUT entry 797 low"]
pub mod lut797l;
#[doc = "LUT798L register accessor: an alias for `Reg<LUT798L_SPEC>`"]
pub type LUT798L = crate::Reg<lut798l::LUT798L_SPEC>;
#[doc = "Graphic MMU LUT entry 798 low"]
pub mod lut798l;
#[doc = "LUT799L register accessor: an alias for `Reg<LUT799L_SPEC>`"]
pub type LUT799L = crate::Reg<lut799l::LUT799L_SPEC>;
#[doc = "Graphic MMU LUT entry 799 low"]
pub mod lut799l;
#[doc = "LUT800L register accessor: an alias for `Reg<LUT800L_SPEC>`"]
pub type LUT800L = crate::Reg<lut800l::LUT800L_SPEC>;
#[doc = "Graphic MMU LUT entry 800 low"]
pub mod lut800l;
#[doc = "LUT801L register accessor: an alias for `Reg<LUT801L_SPEC>`"]
pub type LUT801L = crate::Reg<lut801l::LUT801L_SPEC>;
#[doc = "Graphic MMU LUT entry 801 low"]
pub mod lut801l;
#[doc = "LUT802L register accessor: an alias for `Reg<LUT802L_SPEC>`"]
pub type LUT802L = crate::Reg<lut802l::LUT802L_SPEC>;
#[doc = "Graphic MMU LUT entry 802 low"]
pub mod lut802l;
#[doc = "LUT803L register accessor: an alias for `Reg<LUT803L_SPEC>`"]
pub type LUT803L = crate::Reg<lut803l::LUT803L_SPEC>;
#[doc = "Graphic MMU LUT entry 803 low"]
pub mod lut803l;
#[doc = "LUT804L register accessor: an alias for `Reg<LUT804L_SPEC>`"]
pub type LUT804L = crate::Reg<lut804l::LUT804L_SPEC>;
#[doc = "Graphic MMU LUT entry 804 low"]
pub mod lut804l;
#[doc = "LUT805L register accessor: an alias for `Reg<LUT805L_SPEC>`"]
pub type LUT805L = crate::Reg<lut805l::LUT805L_SPEC>;
#[doc = "Graphic MMU LUT entry 805 low"]
pub mod lut805l;
#[doc = "LUT806L register accessor: an alias for `Reg<LUT806L_SPEC>`"]
pub type LUT806L = crate::Reg<lut806l::LUT806L_SPEC>;
#[doc = "Graphic MMU LUT entry 806 low"]
pub mod lut806l;
#[doc = "LUT807L register accessor: an alias for `Reg<LUT807L_SPEC>`"]
pub type LUT807L = crate::Reg<lut807l::LUT807L_SPEC>;
#[doc = "Graphic MMU LUT entry 807 low"]
pub mod lut807l;
#[doc = "LUT808L register accessor: an alias for `Reg<LUT808L_SPEC>`"]
pub type LUT808L = crate::Reg<lut808l::LUT808L_SPEC>;
#[doc = "Graphic MMU LUT entry 808 low"]
pub mod lut808l;
#[doc = "LUT809L register accessor: an alias for `Reg<LUT809L_SPEC>`"]
pub type LUT809L = crate::Reg<lut809l::LUT809L_SPEC>;
#[doc = "Graphic MMU LUT entry 809 low"]
pub mod lut809l;
#[doc = "LUT810L register accessor: an alias for `Reg<LUT810L_SPEC>`"]
pub type LUT810L = crate::Reg<lut810l::LUT810L_SPEC>;
#[doc = "Graphic MMU LUT entry 810 low"]
pub mod lut810l;
#[doc = "LUT811L register accessor: an alias for `Reg<LUT811L_SPEC>`"]
pub type LUT811L = crate::Reg<lut811l::LUT811L_SPEC>;
#[doc = "Graphic MMU LUT entry 811 low"]
pub mod lut811l;
#[doc = "LUT812L register accessor: an alias for `Reg<LUT812L_SPEC>`"]
pub type LUT812L = crate::Reg<lut812l::LUT812L_SPEC>;
#[doc = "Graphic MMU LUT entry 812 low"]
pub mod lut812l;
#[doc = "LUT813L register accessor: an alias for `Reg<LUT813L_SPEC>`"]
pub type LUT813L = crate::Reg<lut813l::LUT813L_SPEC>;
#[doc = "Graphic MMU LUT entry 813 low"]
pub mod lut813l;
#[doc = "LUT814L register accessor: an alias for `Reg<LUT814L_SPEC>`"]
pub type LUT814L = crate::Reg<lut814l::LUT814L_SPEC>;
#[doc = "Graphic MMU LUT entry 814 low"]
pub mod lut814l;
#[doc = "LUT815L register accessor: an alias for `Reg<LUT815L_SPEC>`"]
pub type LUT815L = crate::Reg<lut815l::LUT815L_SPEC>;
#[doc = "Graphic MMU LUT entry 815 low"]
pub mod lut815l;
#[doc = "LUT816L register accessor: an alias for `Reg<LUT816L_SPEC>`"]
pub type LUT816L = crate::Reg<lut816l::LUT816L_SPEC>;
#[doc = "Graphic MMU LUT entry 816 low"]
pub mod lut816l;
#[doc = "LUT817L register accessor: an alias for `Reg<LUT817L_SPEC>`"]
pub type LUT817L = crate::Reg<lut817l::LUT817L_SPEC>;
#[doc = "Graphic MMU LUT entry 817 low"]
pub mod lut817l;
#[doc = "LUT818L register accessor: an alias for `Reg<LUT818L_SPEC>`"]
pub type LUT818L = crate::Reg<lut818l::LUT818L_SPEC>;
#[doc = "Graphic MMU LUT entry 818 low"]
pub mod lut818l;
#[doc = "LUT819L register accessor: an alias for `Reg<LUT819L_SPEC>`"]
pub type LUT819L = crate::Reg<lut819l::LUT819L_SPEC>;
#[doc = "Graphic MMU LUT entry 819 low"]
pub mod lut819l;
#[doc = "LUT820L register accessor: an alias for `Reg<LUT820L_SPEC>`"]
pub type LUT820L = crate::Reg<lut820l::LUT820L_SPEC>;
#[doc = "Graphic MMU LUT entry 820 low"]
pub mod lut820l;
#[doc = "LUT821L register accessor: an alias for `Reg<LUT821L_SPEC>`"]
pub type LUT821L = crate::Reg<lut821l::LUT821L_SPEC>;
#[doc = "Graphic MMU LUT entry 821 low"]
pub mod lut821l;
#[doc = "LUT822L register accessor: an alias for `Reg<LUT822L_SPEC>`"]
pub type LUT822L = crate::Reg<lut822l::LUT822L_SPEC>;
#[doc = "Graphic MMU LUT entry 822 low"]
pub mod lut822l;
#[doc = "LUT823L register accessor: an alias for `Reg<LUT823L_SPEC>`"]
pub type LUT823L = crate::Reg<lut823l::LUT823L_SPEC>;
#[doc = "Graphic MMU LUT entry 823 low"]
pub mod lut823l;
#[doc = "LUT824L register accessor: an alias for `Reg<LUT824L_SPEC>`"]
pub type LUT824L = crate::Reg<lut824l::LUT824L_SPEC>;
#[doc = "Graphic MMU LUT entry 824 low"]
pub mod lut824l;
#[doc = "LUT825L register accessor: an alias for `Reg<LUT825L_SPEC>`"]
pub type LUT825L = crate::Reg<lut825l::LUT825L_SPEC>;
#[doc = "Graphic MMU LUT entry 825 low"]
pub mod lut825l;
#[doc = "LUT826L register accessor: an alias for `Reg<LUT826L_SPEC>`"]
pub type LUT826L = crate::Reg<lut826l::LUT826L_SPEC>;
#[doc = "Graphic MMU LUT entry 826 low"]
pub mod lut826l;
#[doc = "LUT827L register accessor: an alias for `Reg<LUT827L_SPEC>`"]
pub type LUT827L = crate::Reg<lut827l::LUT827L_SPEC>;
#[doc = "Graphic MMU LUT entry 827 low"]
pub mod lut827l;
#[doc = "LUT828L register accessor: an alias for `Reg<LUT828L_SPEC>`"]
pub type LUT828L = crate::Reg<lut828l::LUT828L_SPEC>;
#[doc = "Graphic MMU LUT entry 828 low"]
pub mod lut828l;
#[doc = "LUT829L register accessor: an alias for `Reg<LUT829L_SPEC>`"]
pub type LUT829L = crate::Reg<lut829l::LUT829L_SPEC>;
#[doc = "Graphic MMU LUT entry 829 low"]
pub mod lut829l;
#[doc = "LUT830L register accessor: an alias for `Reg<LUT830L_SPEC>`"]
pub type LUT830L = crate::Reg<lut830l::LUT830L_SPEC>;
#[doc = "Graphic MMU LUT entry 830 low"]
pub mod lut830l;
#[doc = "LUT831L register accessor: an alias for `Reg<LUT831L_SPEC>`"]
pub type LUT831L = crate::Reg<lut831l::LUT831L_SPEC>;
#[doc = "Graphic MMU LUT entry 831 low"]
pub mod lut831l;
#[doc = "LUT832L register accessor: an alias for `Reg<LUT832L_SPEC>`"]
pub type LUT832L = crate::Reg<lut832l::LUT832L_SPEC>;
#[doc = "Graphic MMU LUT entry 832 low"]
pub mod lut832l;
#[doc = "LUT833L register accessor: an alias for `Reg<LUT833L_SPEC>`"]
pub type LUT833L = crate::Reg<lut833l::LUT833L_SPEC>;
#[doc = "Graphic MMU LUT entry 833 low"]
pub mod lut833l;
#[doc = "LUT834L register accessor: an alias for `Reg<LUT834L_SPEC>`"]
pub type LUT834L = crate::Reg<lut834l::LUT834L_SPEC>;
#[doc = "Graphic MMU LUT entry 834 low"]
pub mod lut834l;
#[doc = "LUT835L register accessor: an alias for `Reg<LUT835L_SPEC>`"]
pub type LUT835L = crate::Reg<lut835l::LUT835L_SPEC>;
#[doc = "Graphic MMU LUT entry 835 low"]
pub mod lut835l;
#[doc = "LUT836L register accessor: an alias for `Reg<LUT836L_SPEC>`"]
pub type LUT836L = crate::Reg<lut836l::LUT836L_SPEC>;
#[doc = "Graphic MMU LUT entry 836 low"]
pub mod lut836l;
#[doc = "LUT837L register accessor: an alias for `Reg<LUT837L_SPEC>`"]
pub type LUT837L = crate::Reg<lut837l::LUT837L_SPEC>;
#[doc = "Graphic MMU LUT entry 837 low"]
pub mod lut837l;
#[doc = "LUT838L register accessor: an alias for `Reg<LUT838L_SPEC>`"]
pub type LUT838L = crate::Reg<lut838l::LUT838L_SPEC>;
#[doc = "Graphic MMU LUT entry 838 low"]
pub mod lut838l;
#[doc = "LUT839L register accessor: an alias for `Reg<LUT839L_SPEC>`"]
pub type LUT839L = crate::Reg<lut839l::LUT839L_SPEC>;
#[doc = "Graphic MMU LUT entry 839 low"]
pub mod lut839l;
#[doc = "LUT840L register accessor: an alias for `Reg<LUT840L_SPEC>`"]
pub type LUT840L = crate::Reg<lut840l::LUT840L_SPEC>;
#[doc = "Graphic MMU LUT entry 840 low"]
pub mod lut840l;
#[doc = "LUT841L register accessor: an alias for `Reg<LUT841L_SPEC>`"]
pub type LUT841L = crate::Reg<lut841l::LUT841L_SPEC>;
#[doc = "Graphic MMU LUT entry 841 low"]
pub mod lut841l;
#[doc = "LUT842L register accessor: an alias for `Reg<LUT842L_SPEC>`"]
pub type LUT842L = crate::Reg<lut842l::LUT842L_SPEC>;
#[doc = "Graphic MMU LUT entry 842 low"]
pub mod lut842l;
#[doc = "LUT843L register accessor: an alias for `Reg<LUT843L_SPEC>`"]
pub type LUT843L = crate::Reg<lut843l::LUT843L_SPEC>;
#[doc = "Graphic MMU LUT entry 843 low"]
pub mod lut843l;
#[doc = "LUT844L register accessor: an alias for `Reg<LUT844L_SPEC>`"]
pub type LUT844L = crate::Reg<lut844l::LUT844L_SPEC>;
#[doc = "Graphic MMU LUT entry 844 low"]
pub mod lut844l;
#[doc = "LUT845L register accessor: an alias for `Reg<LUT845L_SPEC>`"]
pub type LUT845L = crate::Reg<lut845l::LUT845L_SPEC>;
#[doc = "Graphic MMU LUT entry 845 low"]
pub mod lut845l;
#[doc = "LUT846L register accessor: an alias for `Reg<LUT846L_SPEC>`"]
pub type LUT846L = crate::Reg<lut846l::LUT846L_SPEC>;
#[doc = "Graphic MMU LUT entry 846 low"]
pub mod lut846l;
#[doc = "LUT847L register accessor: an alias for `Reg<LUT847L_SPEC>`"]
pub type LUT847L = crate::Reg<lut847l::LUT847L_SPEC>;
#[doc = "Graphic MMU LUT entry 847 low"]
pub mod lut847l;
#[doc = "LUT848L register accessor: an alias for `Reg<LUT848L_SPEC>`"]
pub type LUT848L = crate::Reg<lut848l::LUT848L_SPEC>;
#[doc = "Graphic MMU LUT entry 848 low"]
pub mod lut848l;
#[doc = "LUT849L register accessor: an alias for `Reg<LUT849L_SPEC>`"]
pub type LUT849L = crate::Reg<lut849l::LUT849L_SPEC>;
#[doc = "Graphic MMU LUT entry 849 low"]
pub mod lut849l;
#[doc = "LUT850L register accessor: an alias for `Reg<LUT850L_SPEC>`"]
pub type LUT850L = crate::Reg<lut850l::LUT850L_SPEC>;
#[doc = "Graphic MMU LUT entry 850 low"]
pub mod lut850l;
#[doc = "LUT851L register accessor: an alias for `Reg<LUT851L_SPEC>`"]
pub type LUT851L = crate::Reg<lut851l::LUT851L_SPEC>;
#[doc = "Graphic MMU LUT entry 851 low"]
pub mod lut851l;
#[doc = "LUT852L register accessor: an alias for `Reg<LUT852L_SPEC>`"]
pub type LUT852L = crate::Reg<lut852l::LUT852L_SPEC>;
#[doc = "Graphic MMU LUT entry 852 low"]
pub mod lut852l;
#[doc = "LUT853L register accessor: an alias for `Reg<LUT853L_SPEC>`"]
pub type LUT853L = crate::Reg<lut853l::LUT853L_SPEC>;
#[doc = "Graphic MMU LUT entry 853 low"]
pub mod lut853l;
#[doc = "LUT854L register accessor: an alias for `Reg<LUT854L_SPEC>`"]
pub type LUT854L = crate::Reg<lut854l::LUT854L_SPEC>;
#[doc = "Graphic MMU LUT entry 854 low"]
pub mod lut854l;
#[doc = "LUT855L register accessor: an alias for `Reg<LUT855L_SPEC>`"]
pub type LUT855L = crate::Reg<lut855l::LUT855L_SPEC>;
#[doc = "Graphic MMU LUT entry 855 low"]
pub mod lut855l;
#[doc = "LUT856L register accessor: an alias for `Reg<LUT856L_SPEC>`"]
pub type LUT856L = crate::Reg<lut856l::LUT856L_SPEC>;
#[doc = "Graphic MMU LUT entry 856 low"]
pub mod lut856l;
#[doc = "LUT857L register accessor: an alias for `Reg<LUT857L_SPEC>`"]
pub type LUT857L = crate::Reg<lut857l::LUT857L_SPEC>;
#[doc = "Graphic MMU LUT entry 857 low"]
pub mod lut857l;
#[doc = "LUT858L register accessor: an alias for `Reg<LUT858L_SPEC>`"]
pub type LUT858L = crate::Reg<lut858l::LUT858L_SPEC>;
#[doc = "Graphic MMU LUT entry 858 low"]
pub mod lut858l;
#[doc = "LUT859L register accessor: an alias for `Reg<LUT859L_SPEC>`"]
pub type LUT859L = crate::Reg<lut859l::LUT859L_SPEC>;
#[doc = "Graphic MMU LUT entry 859 low"]
pub mod lut859l;
#[doc = "LUT860L register accessor: an alias for `Reg<LUT860L_SPEC>`"]
pub type LUT860L = crate::Reg<lut860l::LUT860L_SPEC>;
#[doc = "Graphic MMU LUT entry 860 low"]
pub mod lut860l;
#[doc = "LUT861L register accessor: an alias for `Reg<LUT861L_SPEC>`"]
pub type LUT861L = crate::Reg<lut861l::LUT861L_SPEC>;
#[doc = "Graphic MMU LUT entry 861 low"]
pub mod lut861l;
#[doc = "LUT862L register accessor: an alias for `Reg<LUT862L_SPEC>`"]
pub type LUT862L = crate::Reg<lut862l::LUT862L_SPEC>;
#[doc = "Graphic MMU LUT entry 862 low"]
pub mod lut862l;
#[doc = "LUT863L register accessor: an alias for `Reg<LUT863L_SPEC>`"]
pub type LUT863L = crate::Reg<lut863l::LUT863L_SPEC>;
#[doc = "Graphic MMU LUT entry 863 low"]
pub mod lut863l;
#[doc = "LUT864L register accessor: an alias for `Reg<LUT864L_SPEC>`"]
pub type LUT864L = crate::Reg<lut864l::LUT864L_SPEC>;
#[doc = "Graphic MMU LUT entry 864 low"]
pub mod lut864l;
#[doc = "LUT865L register accessor: an alias for `Reg<LUT865L_SPEC>`"]
pub type LUT865L = crate::Reg<lut865l::LUT865L_SPEC>;
#[doc = "Graphic MMU LUT entry 865 low"]
pub mod lut865l;
#[doc = "LUT866L register accessor: an alias for `Reg<LUT866L_SPEC>`"]
pub type LUT866L = crate::Reg<lut866l::LUT866L_SPEC>;
#[doc = "Graphic MMU LUT entry 866 low"]
pub mod lut866l;
#[doc = "LUT867L register accessor: an alias for `Reg<LUT867L_SPEC>`"]
pub type LUT867L = crate::Reg<lut867l::LUT867L_SPEC>;
#[doc = "Graphic MMU LUT entry 867 low"]
pub mod lut867l;
#[doc = "LUT868L register accessor: an alias for `Reg<LUT868L_SPEC>`"]
pub type LUT868L = crate::Reg<lut868l::LUT868L_SPEC>;
#[doc = "Graphic MMU LUT entry 868 low"]
pub mod lut868l;
#[doc = "LUT869L register accessor: an alias for `Reg<LUT869L_SPEC>`"]
pub type LUT869L = crate::Reg<lut869l::LUT869L_SPEC>;
#[doc = "Graphic MMU LUT entry 869 low"]
pub mod lut869l;
#[doc = "LUT870L register accessor: an alias for `Reg<LUT870L_SPEC>`"]
pub type LUT870L = crate::Reg<lut870l::LUT870L_SPEC>;
#[doc = "Graphic MMU LUT entry 870 low"]
pub mod lut870l;
#[doc = "LUT871L register accessor: an alias for `Reg<LUT871L_SPEC>`"]
pub type LUT871L = crate::Reg<lut871l::LUT871L_SPEC>;
#[doc = "Graphic MMU LUT entry 871 low"]
pub mod lut871l;
#[doc = "LUT872L register accessor: an alias for `Reg<LUT872L_SPEC>`"]
pub type LUT872L = crate::Reg<lut872l::LUT872L_SPEC>;
#[doc = "Graphic MMU LUT entry 872 low"]
pub mod lut872l;
#[doc = "LUT873L register accessor: an alias for `Reg<LUT873L_SPEC>`"]
pub type LUT873L = crate::Reg<lut873l::LUT873L_SPEC>;
#[doc = "Graphic MMU LUT entry 873 low"]
pub mod lut873l;
#[doc = "LUT874L register accessor: an alias for `Reg<LUT874L_SPEC>`"]
pub type LUT874L = crate::Reg<lut874l::LUT874L_SPEC>;
#[doc = "Graphic MMU LUT entry 874 low"]
pub mod lut874l;
#[doc = "LUT875L register accessor: an alias for `Reg<LUT875L_SPEC>`"]
pub type LUT875L = crate::Reg<lut875l::LUT875L_SPEC>;
#[doc = "Graphic MMU LUT entry 875 low"]
pub mod lut875l;
#[doc = "LUT876L register accessor: an alias for `Reg<LUT876L_SPEC>`"]
pub type LUT876L = crate::Reg<lut876l::LUT876L_SPEC>;
#[doc = "Graphic MMU LUT entry 876 low"]
pub mod lut876l;
#[doc = "LUT877L register accessor: an alias for `Reg<LUT877L_SPEC>`"]
pub type LUT877L = crate::Reg<lut877l::LUT877L_SPEC>;
#[doc = "Graphic MMU LUT entry 877 low"]
pub mod lut877l;
#[doc = "LUT878L register accessor: an alias for `Reg<LUT878L_SPEC>`"]
pub type LUT878L = crate::Reg<lut878l::LUT878L_SPEC>;
#[doc = "Graphic MMU LUT entry 878 low"]
pub mod lut878l;
#[doc = "LUT879L register accessor: an alias for `Reg<LUT879L_SPEC>`"]
pub type LUT879L = crate::Reg<lut879l::LUT879L_SPEC>;
#[doc = "Graphic MMU LUT entry 879 low"]
pub mod lut879l;
#[doc = "LUT880L register accessor: an alias for `Reg<LUT880L_SPEC>`"]
pub type LUT880L = crate::Reg<lut880l::LUT880L_SPEC>;
#[doc = "Graphic MMU LUT entry 880 low"]
pub mod lut880l;
#[doc = "LUT881L register accessor: an alias for `Reg<LUT881L_SPEC>`"]
pub type LUT881L = crate::Reg<lut881l::LUT881L_SPEC>;
#[doc = "Graphic MMU LUT entry 881 low"]
pub mod lut881l;
#[doc = "LUT882L register accessor: an alias for `Reg<LUT882L_SPEC>`"]
pub type LUT882L = crate::Reg<lut882l::LUT882L_SPEC>;
#[doc = "Graphic MMU LUT entry 882 low"]
pub mod lut882l;
#[doc = "LUT883L register accessor: an alias for `Reg<LUT883L_SPEC>`"]
pub type LUT883L = crate::Reg<lut883l::LUT883L_SPEC>;
#[doc = "Graphic MMU LUT entry 883 low"]
pub mod lut883l;
#[doc = "LUT884L register accessor: an alias for `Reg<LUT884L_SPEC>`"]
pub type LUT884L = crate::Reg<lut884l::LUT884L_SPEC>;
#[doc = "Graphic MMU LUT entry 884 low"]
pub mod lut884l;
#[doc = "LUT885L register accessor: an alias for `Reg<LUT885L_SPEC>`"]
pub type LUT885L = crate::Reg<lut885l::LUT885L_SPEC>;
#[doc = "Graphic MMU LUT entry 885 low"]
pub mod lut885l;
#[doc = "LUT886L register accessor: an alias for `Reg<LUT886L_SPEC>`"]
pub type LUT886L = crate::Reg<lut886l::LUT886L_SPEC>;
#[doc = "Graphic MMU LUT entry 886 low"]
pub mod lut886l;
#[doc = "LUT887L register accessor: an alias for `Reg<LUT887L_SPEC>`"]
pub type LUT887L = crate::Reg<lut887l::LUT887L_SPEC>;
#[doc = "Graphic MMU LUT entry 887 low"]
pub mod lut887l;
#[doc = "LUT888L register accessor: an alias for `Reg<LUT888L_SPEC>`"]
pub type LUT888L = crate::Reg<lut888l::LUT888L_SPEC>;
#[doc = "Graphic MMU LUT entry 888 low"]
pub mod lut888l;
#[doc = "LUT889L register accessor: an alias for `Reg<LUT889L_SPEC>`"]
pub type LUT889L = crate::Reg<lut889l::LUT889L_SPEC>;
#[doc = "Graphic MMU LUT entry 889 low"]
pub mod lut889l;
#[doc = "LUT890L register accessor: an alias for `Reg<LUT890L_SPEC>`"]
pub type LUT890L = crate::Reg<lut890l::LUT890L_SPEC>;
#[doc = "Graphic MMU LUT entry 890 low"]
pub mod lut890l;
#[doc = "LUT891L register accessor: an alias for `Reg<LUT891L_SPEC>`"]
pub type LUT891L = crate::Reg<lut891l::LUT891L_SPEC>;
#[doc = "Graphic MMU LUT entry 891 low"]
pub mod lut891l;
#[doc = "LUT892L register accessor: an alias for `Reg<LUT892L_SPEC>`"]
pub type LUT892L = crate::Reg<lut892l::LUT892L_SPEC>;
#[doc = "Graphic MMU LUT entry 892 low"]
pub mod lut892l;
#[doc = "LUT893L register accessor: an alias for `Reg<LUT893L_SPEC>`"]
pub type LUT893L = crate::Reg<lut893l::LUT893L_SPEC>;
#[doc = "Graphic MMU LUT entry 893 low"]
pub mod lut893l;
#[doc = "LUT894L register accessor: an alias for `Reg<LUT894L_SPEC>`"]
pub type LUT894L = crate::Reg<lut894l::LUT894L_SPEC>;
#[doc = "Graphic MMU LUT entry 894 low"]
pub mod lut894l;
#[doc = "LUT895L register accessor: an alias for `Reg<LUT895L_SPEC>`"]
pub type LUT895L = crate::Reg<lut895l::LUT895L_SPEC>;
#[doc = "Graphic MMU LUT entry 895 low"]
pub mod lut895l;
#[doc = "LUT896L register accessor: an alias for `Reg<LUT896L_SPEC>`"]
pub type LUT896L = crate::Reg<lut896l::LUT896L_SPEC>;
#[doc = "Graphic MMU LUT entry 896 low"]
pub mod lut896l;
#[doc = "LUT897L register accessor: an alias for `Reg<LUT897L_SPEC>`"]
pub type LUT897L = crate::Reg<lut897l::LUT897L_SPEC>;
#[doc = "Graphic MMU LUT entry 897 low"]
pub mod lut897l;
#[doc = "LUT898L register accessor: an alias for `Reg<LUT898L_SPEC>`"]
pub type LUT898L = crate::Reg<lut898l::LUT898L_SPEC>;
#[doc = "Graphic MMU LUT entry 898 low"]
pub mod lut898l;
#[doc = "LUT899L register accessor: an alias for `Reg<LUT899L_SPEC>`"]
pub type LUT899L = crate::Reg<lut899l::LUT899L_SPEC>;
#[doc = "Graphic MMU LUT entry 899 low"]
pub mod lut899l;
#[doc = "LUT900L register accessor: an alias for `Reg<LUT900L_SPEC>`"]
pub type LUT900L = crate::Reg<lut900l::LUT900L_SPEC>;
#[doc = "Graphic MMU LUT entry 900 low"]
pub mod lut900l;
#[doc = "LUT901L register accessor: an alias for `Reg<LUT901L_SPEC>`"]
pub type LUT901L = crate::Reg<lut901l::LUT901L_SPEC>;
#[doc = "Graphic MMU LUT entry 901 low"]
pub mod lut901l;
#[doc = "LUT902L register accessor: an alias for `Reg<LUT902L_SPEC>`"]
pub type LUT902L = crate::Reg<lut902l::LUT902L_SPEC>;
#[doc = "Graphic MMU LUT entry 902 low"]
pub mod lut902l;
#[doc = "LUT903L register accessor: an alias for `Reg<LUT903L_SPEC>`"]
pub type LUT903L = crate::Reg<lut903l::LUT903L_SPEC>;
#[doc = "Graphic MMU LUT entry 903 low"]
pub mod lut903l;
#[doc = "LUT904L register accessor: an alias for `Reg<LUT904L_SPEC>`"]
pub type LUT904L = crate::Reg<lut904l::LUT904L_SPEC>;
#[doc = "Graphic MMU LUT entry 904 low"]
pub mod lut904l;
#[doc = "LUT905L register accessor: an alias for `Reg<LUT905L_SPEC>`"]
pub type LUT905L = crate::Reg<lut905l::LUT905L_SPEC>;
#[doc = "Graphic MMU LUT entry 905 low"]
pub mod lut905l;
#[doc = "LUT906L register accessor: an alias for `Reg<LUT906L_SPEC>`"]
pub type LUT906L = crate::Reg<lut906l::LUT906L_SPEC>;
#[doc = "Graphic MMU LUT entry 906 low"]
pub mod lut906l;
#[doc = "LUT907L register accessor: an alias for `Reg<LUT907L_SPEC>`"]
pub type LUT907L = crate::Reg<lut907l::LUT907L_SPEC>;
#[doc = "Graphic MMU LUT entry 907 low"]
pub mod lut907l;
#[doc = "LUT908L register accessor: an alias for `Reg<LUT908L_SPEC>`"]
pub type LUT908L = crate::Reg<lut908l::LUT908L_SPEC>;
#[doc = "Graphic MMU LUT entry 908 low"]
pub mod lut908l;
#[doc = "LUT909L register accessor: an alias for `Reg<LUT909L_SPEC>`"]
pub type LUT909L = crate::Reg<lut909l::LUT909L_SPEC>;
#[doc = "Graphic MMU LUT entry 909 low"]
pub mod lut909l;
#[doc = "LUT910L register accessor: an alias for `Reg<LUT910L_SPEC>`"]
pub type LUT910L = crate::Reg<lut910l::LUT910L_SPEC>;
#[doc = "Graphic MMU LUT entry 910 low"]
pub mod lut910l;
#[doc = "LUT911L register accessor: an alias for `Reg<LUT911L_SPEC>`"]
pub type LUT911L = crate::Reg<lut911l::LUT911L_SPEC>;
#[doc = "Graphic MMU LUT entry 911 low"]
pub mod lut911l;
#[doc = "LUT912L register accessor: an alias for `Reg<LUT912L_SPEC>`"]
pub type LUT912L = crate::Reg<lut912l::LUT912L_SPEC>;
#[doc = "Graphic MMU LUT entry 912 low"]
pub mod lut912l;
#[doc = "LUT913L register accessor: an alias for `Reg<LUT913L_SPEC>`"]
pub type LUT913L = crate::Reg<lut913l::LUT913L_SPEC>;
#[doc = "Graphic MMU LUT entry 913 low"]
pub mod lut913l;
#[doc = "LUT914L register accessor: an alias for `Reg<LUT914L_SPEC>`"]
pub type LUT914L = crate::Reg<lut914l::LUT914L_SPEC>;
#[doc = "Graphic MMU LUT entry 914 low"]
pub mod lut914l;
#[doc = "LUT915L register accessor: an alias for `Reg<LUT915L_SPEC>`"]
pub type LUT915L = crate::Reg<lut915l::LUT915L_SPEC>;
#[doc = "Graphic MMU LUT entry 915 low"]
pub mod lut915l;
#[doc = "LUT916L register accessor: an alias for `Reg<LUT916L_SPEC>`"]
pub type LUT916L = crate::Reg<lut916l::LUT916L_SPEC>;
#[doc = "Graphic MMU LUT entry 916 low"]
pub mod lut916l;
#[doc = "LUT917L register accessor: an alias for `Reg<LUT917L_SPEC>`"]
pub type LUT917L = crate::Reg<lut917l::LUT917L_SPEC>;
#[doc = "Graphic MMU LUT entry 917 low"]
pub mod lut917l;
#[doc = "LUT918L register accessor: an alias for `Reg<LUT918L_SPEC>`"]
pub type LUT918L = crate::Reg<lut918l::LUT918L_SPEC>;
#[doc = "Graphic MMU LUT entry 918 low"]
pub mod lut918l;
#[doc = "LUT919L register accessor: an alias for `Reg<LUT919L_SPEC>`"]
pub type LUT919L = crate::Reg<lut919l::LUT919L_SPEC>;
#[doc = "Graphic MMU LUT entry 919 low"]
pub mod lut919l;
#[doc = "LUT920L register accessor: an alias for `Reg<LUT920L_SPEC>`"]
pub type LUT920L = crate::Reg<lut920l::LUT920L_SPEC>;
#[doc = "Graphic MMU LUT entry 920 low"]
pub mod lut920l;
#[doc = "LUT921L register accessor: an alias for `Reg<LUT921L_SPEC>`"]
pub type LUT921L = crate::Reg<lut921l::LUT921L_SPEC>;
#[doc = "Graphic MMU LUT entry 921 low"]
pub mod lut921l;
#[doc = "LUT922L register accessor: an alias for `Reg<LUT922L_SPEC>`"]
pub type LUT922L = crate::Reg<lut922l::LUT922L_SPEC>;
#[doc = "Graphic MMU LUT entry 922 low"]
pub mod lut922l;
#[doc = "LUT923L register accessor: an alias for `Reg<LUT923L_SPEC>`"]
pub type LUT923L = crate::Reg<lut923l::LUT923L_SPEC>;
#[doc = "Graphic MMU LUT entry 923 low"]
pub mod lut923l;
#[doc = "LUT924L register accessor: an alias for `Reg<LUT924L_SPEC>`"]
pub type LUT924L = crate::Reg<lut924l::LUT924L_SPEC>;
#[doc = "Graphic MMU LUT entry 924 low"]
pub mod lut924l;
#[doc = "LUT925L register accessor: an alias for `Reg<LUT925L_SPEC>`"]
pub type LUT925L = crate::Reg<lut925l::LUT925L_SPEC>;
#[doc = "Graphic MMU LUT entry 925 low"]
pub mod lut925l;
#[doc = "LUT926L register accessor: an alias for `Reg<LUT926L_SPEC>`"]
pub type LUT926L = crate::Reg<lut926l::LUT926L_SPEC>;
#[doc = "Graphic MMU LUT entry 926 low"]
pub mod lut926l;
#[doc = "LUT927L register accessor: an alias for `Reg<LUT927L_SPEC>`"]
pub type LUT927L = crate::Reg<lut927l::LUT927L_SPEC>;
#[doc = "Graphic MMU LUT entry 927 low"]
pub mod lut927l;
#[doc = "LUT928L register accessor: an alias for `Reg<LUT928L_SPEC>`"]
pub type LUT928L = crate::Reg<lut928l::LUT928L_SPEC>;
#[doc = "Graphic MMU LUT entry 928 low"]
pub mod lut928l;
#[doc = "LUT929L register accessor: an alias for `Reg<LUT929L_SPEC>`"]
pub type LUT929L = crate::Reg<lut929l::LUT929L_SPEC>;
#[doc = "Graphic MMU LUT entry 929 low"]
pub mod lut929l;
#[doc = "LUT930L register accessor: an alias for `Reg<LUT930L_SPEC>`"]
pub type LUT930L = crate::Reg<lut930l::LUT930L_SPEC>;
#[doc = "Graphic MMU LUT entry 930 low"]
pub mod lut930l;
#[doc = "LUT931L register accessor: an alias for `Reg<LUT931L_SPEC>`"]
pub type LUT931L = crate::Reg<lut931l::LUT931L_SPEC>;
#[doc = "Graphic MMU LUT entry 931 low"]
pub mod lut931l;
#[doc = "LUT932L register accessor: an alias for `Reg<LUT932L_SPEC>`"]
pub type LUT932L = crate::Reg<lut932l::LUT932L_SPEC>;
#[doc = "Graphic MMU LUT entry 932 low"]
pub mod lut932l;
#[doc = "LUT933L register accessor: an alias for `Reg<LUT933L_SPEC>`"]
pub type LUT933L = crate::Reg<lut933l::LUT933L_SPEC>;
#[doc = "Graphic MMU LUT entry 933 low"]
pub mod lut933l;
#[doc = "LUT934L register accessor: an alias for `Reg<LUT934L_SPEC>`"]
pub type LUT934L = crate::Reg<lut934l::LUT934L_SPEC>;
#[doc = "Graphic MMU LUT entry 934 low"]
pub mod lut934l;
#[doc = "LUT935L register accessor: an alias for `Reg<LUT935L_SPEC>`"]
pub type LUT935L = crate::Reg<lut935l::LUT935L_SPEC>;
#[doc = "Graphic MMU LUT entry 935 low"]
pub mod lut935l;
#[doc = "LUT936L register accessor: an alias for `Reg<LUT936L_SPEC>`"]
pub type LUT936L = crate::Reg<lut936l::LUT936L_SPEC>;
#[doc = "Graphic MMU LUT entry 936 low"]
pub mod lut936l;
#[doc = "LUT937L register accessor: an alias for `Reg<LUT937L_SPEC>`"]
pub type LUT937L = crate::Reg<lut937l::LUT937L_SPEC>;
#[doc = "Graphic MMU LUT entry 937 low"]
pub mod lut937l;
#[doc = "LUT938L register accessor: an alias for `Reg<LUT938L_SPEC>`"]
pub type LUT938L = crate::Reg<lut938l::LUT938L_SPEC>;
#[doc = "Graphic MMU LUT entry 938 low"]
pub mod lut938l;
#[doc = "LUT939L register accessor: an alias for `Reg<LUT939L_SPEC>`"]
pub type LUT939L = crate::Reg<lut939l::LUT939L_SPEC>;
#[doc = "Graphic MMU LUT entry 939 low"]
pub mod lut939l;
#[doc = "LUT940L register accessor: an alias for `Reg<LUT940L_SPEC>`"]
pub type LUT940L = crate::Reg<lut940l::LUT940L_SPEC>;
#[doc = "Graphic MMU LUT entry 940 low"]
pub mod lut940l;
#[doc = "LUT941L register accessor: an alias for `Reg<LUT941L_SPEC>`"]
pub type LUT941L = crate::Reg<lut941l::LUT941L_SPEC>;
#[doc = "Graphic MMU LUT entry 941 low"]
pub mod lut941l;
#[doc = "LUT942L register accessor: an alias for `Reg<LUT942L_SPEC>`"]
pub type LUT942L = crate::Reg<lut942l::LUT942L_SPEC>;
#[doc = "Graphic MMU LUT entry 942 low"]
pub mod lut942l;
#[doc = "LUT943L register accessor: an alias for `Reg<LUT943L_SPEC>`"]
pub type LUT943L = crate::Reg<lut943l::LUT943L_SPEC>;
#[doc = "Graphic MMU LUT entry 943 low"]
pub mod lut943l;
#[doc = "LUT944L register accessor: an alias for `Reg<LUT944L_SPEC>`"]
pub type LUT944L = crate::Reg<lut944l::LUT944L_SPEC>;
#[doc = "Graphic MMU LUT entry 944 low"]
pub mod lut944l;
#[doc = "LUT945L register accessor: an alias for `Reg<LUT945L_SPEC>`"]
pub type LUT945L = crate::Reg<lut945l::LUT945L_SPEC>;
#[doc = "Graphic MMU LUT entry 945 low"]
pub mod lut945l;
#[doc = "LUT946L register accessor: an alias for `Reg<LUT946L_SPEC>`"]
pub type LUT946L = crate::Reg<lut946l::LUT946L_SPEC>;
#[doc = "Graphic MMU LUT entry 946 low"]
pub mod lut946l;
#[doc = "LUT947L register accessor: an alias for `Reg<LUT947L_SPEC>`"]
pub type LUT947L = crate::Reg<lut947l::LUT947L_SPEC>;
#[doc = "Graphic MMU LUT entry 947 low"]
pub mod lut947l;
#[doc = "LUT948L register accessor: an alias for `Reg<LUT948L_SPEC>`"]
pub type LUT948L = crate::Reg<lut948l::LUT948L_SPEC>;
#[doc = "Graphic MMU LUT entry 948 low"]
pub mod lut948l;
#[doc = "LUT949L register accessor: an alias for `Reg<LUT949L_SPEC>`"]
pub type LUT949L = crate::Reg<lut949l::LUT949L_SPEC>;
#[doc = "Graphic MMU LUT entry 949 low"]
pub mod lut949l;
#[doc = "LUT950L register accessor: an alias for `Reg<LUT950L_SPEC>`"]
pub type LUT950L = crate::Reg<lut950l::LUT950L_SPEC>;
#[doc = "Graphic MMU LUT entry 950 low"]
pub mod lut950l;
#[doc = "LUT951L register accessor: an alias for `Reg<LUT951L_SPEC>`"]
pub type LUT951L = crate::Reg<lut951l::LUT951L_SPEC>;
#[doc = "Graphic MMU LUT entry 951 low"]
pub mod lut951l;
#[doc = "LUT952L register accessor: an alias for `Reg<LUT952L_SPEC>`"]
pub type LUT952L = crate::Reg<lut952l::LUT952L_SPEC>;
#[doc = "Graphic MMU LUT entry 952 low"]
pub mod lut952l;
#[doc = "LUT953L register accessor: an alias for `Reg<LUT953L_SPEC>`"]
pub type LUT953L = crate::Reg<lut953l::LUT953L_SPEC>;
#[doc = "Graphic MMU LUT entry 953 low"]
pub mod lut953l;
#[doc = "LUT954L register accessor: an alias for `Reg<LUT954L_SPEC>`"]
pub type LUT954L = crate::Reg<lut954l::LUT954L_SPEC>;
#[doc = "Graphic MMU LUT entry 954 low"]
pub mod lut954l;
#[doc = "LUT955L register accessor: an alias for `Reg<LUT955L_SPEC>`"]
pub type LUT955L = crate::Reg<lut955l::LUT955L_SPEC>;
#[doc = "Graphic MMU LUT entry 955 low"]
pub mod lut955l;
#[doc = "LUT956L register accessor: an alias for `Reg<LUT956L_SPEC>`"]
pub type LUT956L = crate::Reg<lut956l::LUT956L_SPEC>;
#[doc = "Graphic MMU LUT entry 956 low"]
pub mod lut956l;
#[doc = "LUT957L register accessor: an alias for `Reg<LUT957L_SPEC>`"]
pub type LUT957L = crate::Reg<lut957l::LUT957L_SPEC>;
#[doc = "Graphic MMU LUT entry 957 low"]
pub mod lut957l;
#[doc = "LUT958L register accessor: an alias for `Reg<LUT958L_SPEC>`"]
pub type LUT958L = crate::Reg<lut958l::LUT958L_SPEC>;
#[doc = "Graphic MMU LUT entry 958 low"]
pub mod lut958l;
#[doc = "LUT959L register accessor: an alias for `Reg<LUT959L_SPEC>`"]
pub type LUT959L = crate::Reg<lut959l::LUT959L_SPEC>;
#[doc = "Graphic MMU LUT entry 959 low"]
pub mod lut959l;
#[doc = "LUT960L register accessor: an alias for `Reg<LUT960L_SPEC>`"]
pub type LUT960L = crate::Reg<lut960l::LUT960L_SPEC>;
#[doc = "Graphic MMU LUT entry 960 low"]
pub mod lut960l;
#[doc = "LUT961L register accessor: an alias for `Reg<LUT961L_SPEC>`"]
pub type LUT961L = crate::Reg<lut961l::LUT961L_SPEC>;
#[doc = "Graphic MMU LUT entry 961 low"]
pub mod lut961l;
#[doc = "LUT962L register accessor: an alias for `Reg<LUT962L_SPEC>`"]
pub type LUT962L = crate::Reg<lut962l::LUT962L_SPEC>;
#[doc = "Graphic MMU LUT entry 962 low"]
pub mod lut962l;
#[doc = "LUT963L register accessor: an alias for `Reg<LUT963L_SPEC>`"]
pub type LUT963L = crate::Reg<lut963l::LUT963L_SPEC>;
#[doc = "Graphic MMU LUT entry 963 low"]
pub mod lut963l;
#[doc = "LUT964L register accessor: an alias for `Reg<LUT964L_SPEC>`"]
pub type LUT964L = crate::Reg<lut964l::LUT964L_SPEC>;
#[doc = "Graphic MMU LUT entry 964 low"]
pub mod lut964l;
#[doc = "LUT965L register accessor: an alias for `Reg<LUT965L_SPEC>`"]
pub type LUT965L = crate::Reg<lut965l::LUT965L_SPEC>;
#[doc = "Graphic MMU LUT entry 965 low"]
pub mod lut965l;
#[doc = "LUT966L register accessor: an alias for `Reg<LUT966L_SPEC>`"]
pub type LUT966L = crate::Reg<lut966l::LUT966L_SPEC>;
#[doc = "Graphic MMU LUT entry 966 low"]
pub mod lut966l;
#[doc = "LUT967L register accessor: an alias for `Reg<LUT967L_SPEC>`"]
pub type LUT967L = crate::Reg<lut967l::LUT967L_SPEC>;
#[doc = "Graphic MMU LUT entry 967 low"]
pub mod lut967l;
#[doc = "LUT968L register accessor: an alias for `Reg<LUT968L_SPEC>`"]
pub type LUT968L = crate::Reg<lut968l::LUT968L_SPEC>;
#[doc = "Graphic MMU LUT entry 968 low"]
pub mod lut968l;
#[doc = "LUT969L register accessor: an alias for `Reg<LUT969L_SPEC>`"]
pub type LUT969L = crate::Reg<lut969l::LUT969L_SPEC>;
#[doc = "Graphic MMU LUT entry 969 low"]
pub mod lut969l;
#[doc = "LUT970L register accessor: an alias for `Reg<LUT970L_SPEC>`"]
pub type LUT970L = crate::Reg<lut970l::LUT970L_SPEC>;
#[doc = "Graphic MMU LUT entry 970 low"]
pub mod lut970l;
#[doc = "LUT971L register accessor: an alias for `Reg<LUT971L_SPEC>`"]
pub type LUT971L = crate::Reg<lut971l::LUT971L_SPEC>;
#[doc = "Graphic MMU LUT entry 971 low"]
pub mod lut971l;
#[doc = "LUT972L register accessor: an alias for `Reg<LUT972L_SPEC>`"]
pub type LUT972L = crate::Reg<lut972l::LUT972L_SPEC>;
#[doc = "Graphic MMU LUT entry 972 low"]
pub mod lut972l;
#[doc = "LUT973L register accessor: an alias for `Reg<LUT973L_SPEC>`"]
pub type LUT973L = crate::Reg<lut973l::LUT973L_SPEC>;
#[doc = "Graphic MMU LUT entry 973 low"]
pub mod lut973l;
#[doc = "LUT974L register accessor: an alias for `Reg<LUT974L_SPEC>`"]
pub type LUT974L = crate::Reg<lut974l::LUT974L_SPEC>;
#[doc = "Graphic MMU LUT entry 974 low"]
pub mod lut974l;
#[doc = "LUT975L register accessor: an alias for `Reg<LUT975L_SPEC>`"]
pub type LUT975L = crate::Reg<lut975l::LUT975L_SPEC>;
#[doc = "Graphic MMU LUT entry 975 low"]
pub mod lut975l;
#[doc = "LUT976L register accessor: an alias for `Reg<LUT976L_SPEC>`"]
pub type LUT976L = crate::Reg<lut976l::LUT976L_SPEC>;
#[doc = "Graphic MMU LUT entry 976 low"]
pub mod lut976l;
#[doc = "LUT977L register accessor: an alias for `Reg<LUT977L_SPEC>`"]
pub type LUT977L = crate::Reg<lut977l::LUT977L_SPEC>;
#[doc = "Graphic MMU LUT entry 977 low"]
pub mod lut977l;
#[doc = "LUT978L register accessor: an alias for `Reg<LUT978L_SPEC>`"]
pub type LUT978L = crate::Reg<lut978l::LUT978L_SPEC>;
#[doc = "Graphic MMU LUT entry 978 low"]
pub mod lut978l;
#[doc = "LUT979L register accessor: an alias for `Reg<LUT979L_SPEC>`"]
pub type LUT979L = crate::Reg<lut979l::LUT979L_SPEC>;
#[doc = "Graphic MMU LUT entry 979 low"]
pub mod lut979l;
#[doc = "LUT980L register accessor: an alias for `Reg<LUT980L_SPEC>`"]
pub type LUT980L = crate::Reg<lut980l::LUT980L_SPEC>;
#[doc = "Graphic MMU LUT entry 980 low"]
pub mod lut980l;
#[doc = "LUT981L register accessor: an alias for `Reg<LUT981L_SPEC>`"]
pub type LUT981L = crate::Reg<lut981l::LUT981L_SPEC>;
#[doc = "Graphic MMU LUT entry 981 low"]
pub mod lut981l;
#[doc = "LUT982L register accessor: an alias for `Reg<LUT982L_SPEC>`"]
pub type LUT982L = crate::Reg<lut982l::LUT982L_SPEC>;
#[doc = "Graphic MMU LUT entry 982 low"]
pub mod lut982l;
#[doc = "LUT983L register accessor: an alias for `Reg<LUT983L_SPEC>`"]
pub type LUT983L = crate::Reg<lut983l::LUT983L_SPEC>;
#[doc = "Graphic MMU LUT entry 983 low"]
pub mod lut983l;
#[doc = "LUT984L register accessor: an alias for `Reg<LUT984L_SPEC>`"]
pub type LUT984L = crate::Reg<lut984l::LUT984L_SPEC>;
#[doc = "Graphic MMU LUT entry 984 low"]
pub mod lut984l;
#[doc = "LUT985L register accessor: an alias for `Reg<LUT985L_SPEC>`"]
pub type LUT985L = crate::Reg<lut985l::LUT985L_SPEC>;
#[doc = "Graphic MMU LUT entry 985 low"]
pub mod lut985l;
#[doc = "LUT986L register accessor: an alias for `Reg<LUT986L_SPEC>`"]
pub type LUT986L = crate::Reg<lut986l::LUT986L_SPEC>;
#[doc = "Graphic MMU LUT entry 986 low"]
pub mod lut986l;
#[doc = "LUT987L register accessor: an alias for `Reg<LUT987L_SPEC>`"]
pub type LUT987L = crate::Reg<lut987l::LUT987L_SPEC>;
#[doc = "Graphic MMU LUT entry 987 low"]
pub mod lut987l;
#[doc = "LUT988L register accessor: an alias for `Reg<LUT988L_SPEC>`"]
pub type LUT988L = crate::Reg<lut988l::LUT988L_SPEC>;
#[doc = "Graphic MMU LUT entry 988 low"]
pub mod lut988l;
#[doc = "LUT989L register accessor: an alias for `Reg<LUT989L_SPEC>`"]
pub type LUT989L = crate::Reg<lut989l::LUT989L_SPEC>;
#[doc = "Graphic MMU LUT entry 989 low"]
pub mod lut989l;
#[doc = "LUT990L register accessor: an alias for `Reg<LUT990L_SPEC>`"]
pub type LUT990L = crate::Reg<lut990l::LUT990L_SPEC>;
#[doc = "Graphic MMU LUT entry 990 low"]
pub mod lut990l;
#[doc = "LUT991L register accessor: an alias for `Reg<LUT991L_SPEC>`"]
pub type LUT991L = crate::Reg<lut991l::LUT991L_SPEC>;
#[doc = "Graphic MMU LUT entry 991 low"]
pub mod lut991l;
#[doc = "LUT992L register accessor: an alias for `Reg<LUT992L_SPEC>`"]
pub type LUT992L = crate::Reg<lut992l::LUT992L_SPEC>;
#[doc = "Graphic MMU LUT entry 992 low"]
pub mod lut992l;
#[doc = "LUT993L register accessor: an alias for `Reg<LUT993L_SPEC>`"]
pub type LUT993L = crate::Reg<lut993l::LUT993L_SPEC>;
#[doc = "Graphic MMU LUT entry 993 low"]
pub mod lut993l;
#[doc = "LUT994L register accessor: an alias for `Reg<LUT994L_SPEC>`"]
pub type LUT994L = crate::Reg<lut994l::LUT994L_SPEC>;
#[doc = "Graphic MMU LUT entry 994 low"]
pub mod lut994l;
#[doc = "LUT995L register accessor: an alias for `Reg<LUT995L_SPEC>`"]
pub type LUT995L = crate::Reg<lut995l::LUT995L_SPEC>;
#[doc = "Graphic MMU LUT entry 995 low"]
pub mod lut995l;
#[doc = "LUT996L register accessor: an alias for `Reg<LUT996L_SPEC>`"]
pub type LUT996L = crate::Reg<lut996l::LUT996L_SPEC>;
#[doc = "Graphic MMU LUT entry 996 low"]
pub mod lut996l;
#[doc = "LUT997L register accessor: an alias for `Reg<LUT997L_SPEC>`"]
pub type LUT997L = crate::Reg<lut997l::LUT997L_SPEC>;
#[doc = "Graphic MMU LUT entry 997 low"]
pub mod lut997l;
#[doc = "LUT998L register accessor: an alias for `Reg<LUT998L_SPEC>`"]
pub type LUT998L = crate::Reg<lut998l::LUT998L_SPEC>;
#[doc = "Graphic MMU LUT entry 998 low"]
pub mod lut998l;
#[doc = "LUT999L register accessor: an alias for `Reg<LUT999L_SPEC>`"]
pub type LUT999L = crate::Reg<lut999l::LUT999L_SPEC>;
#[doc = "Graphic MMU LUT entry 999 low"]
pub mod lut999l;
#[doc = "LUT1000L register accessor: an alias for `Reg<LUT1000L_SPEC>`"]
pub type LUT1000L = crate::Reg<lut1000l::LUT1000L_SPEC>;
#[doc = "Graphic MMU LUT entry 1000 low"]
pub mod lut1000l;
#[doc = "LUT1001L register accessor: an alias for `Reg<LUT1001L_SPEC>`"]
pub type LUT1001L = crate::Reg<lut1001l::LUT1001L_SPEC>;
#[doc = "Graphic MMU LUT entry 1001 low"]
pub mod lut1001l;
#[doc = "LUT1002L register accessor: an alias for `Reg<LUT1002L_SPEC>`"]
pub type LUT1002L = crate::Reg<lut1002l::LUT1002L_SPEC>;
#[doc = "Graphic MMU LUT entry 1002 low"]
pub mod lut1002l;
#[doc = "LUT1003L register accessor: an alias for `Reg<LUT1003L_SPEC>`"]
pub type LUT1003L = crate::Reg<lut1003l::LUT1003L_SPEC>;
#[doc = "Graphic MMU LUT entry 1003 low"]
pub mod lut1003l;
#[doc = "LUT1004L register accessor: an alias for `Reg<LUT1004L_SPEC>`"]
pub type LUT1004L = crate::Reg<lut1004l::LUT1004L_SPEC>;
#[doc = "Graphic MMU LUT entry 1004 low"]
pub mod lut1004l;
#[doc = "LUT1005L register accessor: an alias for `Reg<LUT1005L_SPEC>`"]
pub type LUT1005L = crate::Reg<lut1005l::LUT1005L_SPEC>;
#[doc = "Graphic MMU LUT entry 1005 low"]
pub mod lut1005l;
#[doc = "LUT1006L register accessor: an alias for `Reg<LUT1006L_SPEC>`"]
pub type LUT1006L = crate::Reg<lut1006l::LUT1006L_SPEC>;
#[doc = "Graphic MMU LUT entry 1006 low"]
pub mod lut1006l;
#[doc = "LUT1007L register accessor: an alias for `Reg<LUT1007L_SPEC>`"]
pub type LUT1007L = crate::Reg<lut1007l::LUT1007L_SPEC>;
#[doc = "Graphic MMU LUT entry 1007 low"]
pub mod lut1007l;
#[doc = "LUT1008L register accessor: an alias for `Reg<LUT1008L_SPEC>`"]
pub type LUT1008L = crate::Reg<lut1008l::LUT1008L_SPEC>;
#[doc = "Graphic MMU LUT entry 1008 low"]
pub mod lut1008l;
#[doc = "LUT1009L register accessor: an alias for `Reg<LUT1009L_SPEC>`"]
pub type LUT1009L = crate::Reg<lut1009l::LUT1009L_SPEC>;
#[doc = "Graphic MMU LUT entry 1009 low"]
pub mod lut1009l;
#[doc = "LUT1010L register accessor: an alias for `Reg<LUT1010L_SPEC>`"]
pub type LUT1010L = crate::Reg<lut1010l::LUT1010L_SPEC>;
#[doc = "Graphic MMU LUT entry 1010 low"]
pub mod lut1010l;
#[doc = "LUT1011L register accessor: an alias for `Reg<LUT1011L_SPEC>`"]
pub type LUT1011L = crate::Reg<lut1011l::LUT1011L_SPEC>;
#[doc = "Graphic MMU LUT entry 1011 low"]
pub mod lut1011l;
#[doc = "LUT1012L register accessor: an alias for `Reg<LUT1012L_SPEC>`"]
pub type LUT1012L = crate::Reg<lut1012l::LUT1012L_SPEC>;
#[doc = "Graphic MMU LUT entry 1012 low"]
pub mod lut1012l;
#[doc = "LUT1013L register accessor: an alias for `Reg<LUT1013L_SPEC>`"]
pub type LUT1013L = crate::Reg<lut1013l::LUT1013L_SPEC>;
#[doc = "Graphic MMU LUT entry 1013 low"]
pub mod lut1013l;
#[doc = "LUT1014L register accessor: an alias for `Reg<LUT1014L_SPEC>`"]
pub type LUT1014L = crate::Reg<lut1014l::LUT1014L_SPEC>;
#[doc = "Graphic MMU LUT entry 1014 low"]
pub mod lut1014l;
#[doc = "LUT1015L register accessor: an alias for `Reg<LUT1015L_SPEC>`"]
pub type LUT1015L = crate::Reg<lut1015l::LUT1015L_SPEC>;
#[doc = "Graphic MMU LUT entry 1015 low"]
pub mod lut1015l;
#[doc = "LUT1016L register accessor: an alias for `Reg<LUT1016L_SPEC>`"]
pub type LUT1016L = crate::Reg<lut1016l::LUT1016L_SPEC>;
#[doc = "Graphic MMU LUT entry 1016 low"]
pub mod lut1016l;
#[doc = "LUT1017L register accessor: an alias for `Reg<LUT1017L_SPEC>`"]
pub type LUT1017L = crate::Reg<lut1017l::LUT1017L_SPEC>;
#[doc = "Graphic MMU LUT entry 1017 low"]
pub mod lut1017l;
#[doc = "LUT1018L register accessor: an alias for `Reg<LUT1018L_SPEC>`"]
pub type LUT1018L = crate::Reg<lut1018l::LUT1018L_SPEC>;
#[doc = "Graphic MMU LUT entry 1018 low"]
pub mod lut1018l;
#[doc = "LUT1019L register accessor: an alias for `Reg<LUT1019L_SPEC>`"]
pub type LUT1019L = crate::Reg<lut1019l::LUT1019L_SPEC>;
#[doc = "Graphic MMU LUT entry 1019 low"]
pub mod lut1019l;
#[doc = "LUT1020L register accessor: an alias for `Reg<LUT1020L_SPEC>`"]
pub type LUT1020L = crate::Reg<lut1020l::LUT1020L_SPEC>;
#[doc = "Graphic MMU LUT entry 1020 low"]
pub mod lut1020l;
#[doc = "LUT1021L register accessor: an alias for `Reg<LUT1021L_SPEC>`"]
pub type LUT1021L = crate::Reg<lut1021l::LUT1021L_SPEC>;
#[doc = "Graphic MMU LUT entry 1021 low"]
pub mod lut1021l;
#[doc = "LUT1022L register accessor: an alias for `Reg<LUT1022L_SPEC>`"]
pub type LUT1022L = crate::Reg<lut1022l::LUT1022L_SPEC>;
#[doc = "Graphic MMU LUT entry 1022 low"]
pub mod lut1022l;
#[doc = "LUT1023L register accessor: an alias for `Reg<LUT1023L_SPEC>`"]
pub type LUT1023L = crate::Reg<lut1023l::LUT1023L_SPEC>;
#[doc = "Graphic MMU LUT entry 1023 low"]
pub mod lut1023l;
#[doc = "LUT0H register accessor: an alias for `Reg<LUT0H_SPEC>`"]
pub type LUT0H = crate::Reg<lut0h::LUT0H_SPEC>;
#[doc = "Graphic MMU LUT entry 0 high"]
pub mod lut0h;
#[doc = "LUT1H register accessor: an alias for `Reg<LUT1H_SPEC>`"]
pub type LUT1H = crate::Reg<lut1h::LUT1H_SPEC>;
#[doc = "Graphic MMU LUT entry 1 high"]
pub mod lut1h;
#[doc = "LUT2H register accessor: an alias for `Reg<LUT2H_SPEC>`"]
pub type LUT2H = crate::Reg<lut2h::LUT2H_SPEC>;
#[doc = "Graphic MMU LUT entry 2 high"]
pub mod lut2h;
#[doc = "LUT3H register accessor: an alias for `Reg<LUT3H_SPEC>`"]
pub type LUT3H = crate::Reg<lut3h::LUT3H_SPEC>;
#[doc = "Graphic MMU LUT entry 3 high"]
pub mod lut3h;
#[doc = "LUT4H register accessor: an alias for `Reg<LUT4H_SPEC>`"]
pub type LUT4H = crate::Reg<lut4h::LUT4H_SPEC>;
#[doc = "Graphic MMU LUT entry 4 high"]
pub mod lut4h;
#[doc = "LUT5H register accessor: an alias for `Reg<LUT5H_SPEC>`"]
pub type LUT5H = crate::Reg<lut5h::LUT5H_SPEC>;
#[doc = "Graphic MMU LUT entry 5 high"]
pub mod lut5h;
#[doc = "LUT6H register accessor: an alias for `Reg<LUT6H_SPEC>`"]
pub type LUT6H = crate::Reg<lut6h::LUT6H_SPEC>;
#[doc = "Graphic MMU LUT entry 6 high"]
pub mod lut6h;
#[doc = "LUT7H register accessor: an alias for `Reg<LUT7H_SPEC>`"]
pub type LUT7H = crate::Reg<lut7h::LUT7H_SPEC>;
#[doc = "Graphic MMU LUT entry 7 high"]
pub mod lut7h;
#[doc = "LUT8H register accessor: an alias for `Reg<LUT8H_SPEC>`"]
pub type LUT8H = crate::Reg<lut8h::LUT8H_SPEC>;
#[doc = "Graphic MMU LUT entry 8 high"]
pub mod lut8h;
#[doc = "LUT9H register accessor: an alias for `Reg<LUT9H_SPEC>`"]
pub type LUT9H = crate::Reg<lut9h::LUT9H_SPEC>;
#[doc = "Graphic MMU LUT entry 9 high"]
pub mod lut9h;
#[doc = "LUT10H register accessor: an alias for `Reg<LUT10H_SPEC>`"]
pub type LUT10H = crate::Reg<lut10h::LUT10H_SPEC>;
#[doc = "Graphic MMU LUT entry 10 high"]
pub mod lut10h;
#[doc = "LUT11H register accessor: an alias for `Reg<LUT11H_SPEC>`"]
pub type LUT11H = crate::Reg<lut11h::LUT11H_SPEC>;
#[doc = "Graphic MMU LUT entry 11 high"]
pub mod lut11h;
#[doc = "LUT12H register accessor: an alias for `Reg<LUT12H_SPEC>`"]
pub type LUT12H = crate::Reg<lut12h::LUT12H_SPEC>;
#[doc = "Graphic MMU LUT entry 12 high"]
pub mod lut12h;
#[doc = "LUT13H register accessor: an alias for `Reg<LUT13H_SPEC>`"]
pub type LUT13H = crate::Reg<lut13h::LUT13H_SPEC>;
#[doc = "Graphic MMU LUT entry 13 high"]
pub mod lut13h;
#[doc = "LUT14H register accessor: an alias for `Reg<LUT14H_SPEC>`"]
pub type LUT14H = crate::Reg<lut14h::LUT14H_SPEC>;
#[doc = "Graphic MMU LUT entry 14 high"]
pub mod lut14h;
#[doc = "LUT15H register accessor: an alias for `Reg<LUT15H_SPEC>`"]
pub type LUT15H = crate::Reg<lut15h::LUT15H_SPEC>;
#[doc = "Graphic MMU LUT entry 15 high"]
pub mod lut15h;
#[doc = "LUT16H register accessor: an alias for `Reg<LUT16H_SPEC>`"]
pub type LUT16H = crate::Reg<lut16h::LUT16H_SPEC>;
#[doc = "Graphic MMU LUT entry 16 high"]
pub mod lut16h;
#[doc = "LUT17H register accessor: an alias for `Reg<LUT17H_SPEC>`"]
pub type LUT17H = crate::Reg<lut17h::LUT17H_SPEC>;
#[doc = "Graphic MMU LUT entry 17 high"]
pub mod lut17h;
#[doc = "LUT18H register accessor: an alias for `Reg<LUT18H_SPEC>`"]
pub type LUT18H = crate::Reg<lut18h::LUT18H_SPEC>;
#[doc = "Graphic MMU LUT entry 18 high"]
pub mod lut18h;
#[doc = "LUT19H register accessor: an alias for `Reg<LUT19H_SPEC>`"]
pub type LUT19H = crate::Reg<lut19h::LUT19H_SPEC>;
#[doc = "Graphic MMU LUT entry 19 high"]
pub mod lut19h;
#[doc = "LUT20H register accessor: an alias for `Reg<LUT20H_SPEC>`"]
pub type LUT20H = crate::Reg<lut20h::LUT20H_SPEC>;
#[doc = "Graphic MMU LUT entry 20 high"]
pub mod lut20h;
#[doc = "LUT21H register accessor: an alias for `Reg<LUT21H_SPEC>`"]
pub type LUT21H = crate::Reg<lut21h::LUT21H_SPEC>;
#[doc = "Graphic MMU LUT entry 21 high"]
pub mod lut21h;
#[doc = "LUT22H register accessor: an alias for `Reg<LUT22H_SPEC>`"]
pub type LUT22H = crate::Reg<lut22h::LUT22H_SPEC>;
#[doc = "Graphic MMU LUT entry 22 high"]
pub mod lut22h;
#[doc = "LUT23H register accessor: an alias for `Reg<LUT23H_SPEC>`"]
pub type LUT23H = crate::Reg<lut23h::LUT23H_SPEC>;
#[doc = "Graphic MMU LUT entry 23 high"]
pub mod lut23h;
#[doc = "LUT24H register accessor: an alias for `Reg<LUT24H_SPEC>`"]
pub type LUT24H = crate::Reg<lut24h::LUT24H_SPEC>;
#[doc = "Graphic MMU LUT entry 24 high"]
pub mod lut24h;
#[doc = "LUT25H register accessor: an alias for `Reg<LUT25H_SPEC>`"]
pub type LUT25H = crate::Reg<lut25h::LUT25H_SPEC>;
#[doc = "Graphic MMU LUT entry 25 high"]
pub mod lut25h;
#[doc = "LUT26H register accessor: an alias for `Reg<LUT26H_SPEC>`"]
pub type LUT26H = crate::Reg<lut26h::LUT26H_SPEC>;
#[doc = "Graphic MMU LUT entry 26 high"]
pub mod lut26h;
#[doc = "LUT27H register accessor: an alias for `Reg<LUT27H_SPEC>`"]
pub type LUT27H = crate::Reg<lut27h::LUT27H_SPEC>;
#[doc = "Graphic MMU LUT entry 27 high"]
pub mod lut27h;
#[doc = "LUT28H register accessor: an alias for `Reg<LUT28H_SPEC>`"]
pub type LUT28H = crate::Reg<lut28h::LUT28H_SPEC>;
#[doc = "Graphic MMU LUT entry 28 high"]
pub mod lut28h;
#[doc = "LUT29H register accessor: an alias for `Reg<LUT29H_SPEC>`"]
pub type LUT29H = crate::Reg<lut29h::LUT29H_SPEC>;
#[doc = "Graphic MMU LUT entry 29 high"]
pub mod lut29h;
#[doc = "LUT30H register accessor: an alias for `Reg<LUT30H_SPEC>`"]
pub type LUT30H = crate::Reg<lut30h::LUT30H_SPEC>;
#[doc = "Graphic MMU LUT entry 30 high"]
pub mod lut30h;
#[doc = "LUT31H register accessor: an alias for `Reg<LUT31H_SPEC>`"]
pub type LUT31H = crate::Reg<lut31h::LUT31H_SPEC>;
#[doc = "Graphic MMU LUT entry 31 high"]
pub mod lut31h;
#[doc = "LUT32H register accessor: an alias for `Reg<LUT32H_SPEC>`"]
pub type LUT32H = crate::Reg<lut32h::LUT32H_SPEC>;
#[doc = "Graphic MMU LUT entry 32 high"]
pub mod lut32h;
#[doc = "LUT33H register accessor: an alias for `Reg<LUT33H_SPEC>`"]
pub type LUT33H = crate::Reg<lut33h::LUT33H_SPEC>;
#[doc = "Graphic MMU LUT entry 33 high"]
pub mod lut33h;
#[doc = "LUT34H register accessor: an alias for `Reg<LUT34H_SPEC>`"]
pub type LUT34H = crate::Reg<lut34h::LUT34H_SPEC>;
#[doc = "Graphic MMU LUT entry 34 high"]
pub mod lut34h;
#[doc = "LUT35H register accessor: an alias for `Reg<LUT35H_SPEC>`"]
pub type LUT35H = crate::Reg<lut35h::LUT35H_SPEC>;
#[doc = "Graphic MMU LUT entry 35 high"]
pub mod lut35h;
#[doc = "LUT36H register accessor: an alias for `Reg<LUT36H_SPEC>`"]
pub type LUT36H = crate::Reg<lut36h::LUT36H_SPEC>;
#[doc = "Graphic MMU LUT entry 36 high"]
pub mod lut36h;
#[doc = "LUT37H register accessor: an alias for `Reg<LUT37H_SPEC>`"]
pub type LUT37H = crate::Reg<lut37h::LUT37H_SPEC>;
#[doc = "Graphic MMU LUT entry 37 high"]
pub mod lut37h;
#[doc = "LUT38H register accessor: an alias for `Reg<LUT38H_SPEC>`"]
pub type LUT38H = crate::Reg<lut38h::LUT38H_SPEC>;
#[doc = "Graphic MMU LUT entry 38 high"]
pub mod lut38h;
#[doc = "LUT39H register accessor: an alias for `Reg<LUT39H_SPEC>`"]
pub type LUT39H = crate::Reg<lut39h::LUT39H_SPEC>;
#[doc = "Graphic MMU LUT entry 39 high"]
pub mod lut39h;
#[doc = "LUT40H register accessor: an alias for `Reg<LUT40H_SPEC>`"]
pub type LUT40H = crate::Reg<lut40h::LUT40H_SPEC>;
#[doc = "Graphic MMU LUT entry 40 high"]
pub mod lut40h;
#[doc = "LUT41H register accessor: an alias for `Reg<LUT41H_SPEC>`"]
pub type LUT41H = crate::Reg<lut41h::LUT41H_SPEC>;
#[doc = "Graphic MMU LUT entry 41 high"]
pub mod lut41h;
#[doc = "LUT42H register accessor: an alias for `Reg<LUT42H_SPEC>`"]
pub type LUT42H = crate::Reg<lut42h::LUT42H_SPEC>;
#[doc = "Graphic MMU LUT entry 42 high"]
pub mod lut42h;
#[doc = "LUT43H register accessor: an alias for `Reg<LUT43H_SPEC>`"]
pub type LUT43H = crate::Reg<lut43h::LUT43H_SPEC>;
#[doc = "Graphic MMU LUT entry 43 high"]
pub mod lut43h;
#[doc = "LUT44H register accessor: an alias for `Reg<LUT44H_SPEC>`"]
pub type LUT44H = crate::Reg<lut44h::LUT44H_SPEC>;
#[doc = "Graphic MMU LUT entry 44 high"]
pub mod lut44h;
#[doc = "LUT45H register accessor: an alias for `Reg<LUT45H_SPEC>`"]
pub type LUT45H = crate::Reg<lut45h::LUT45H_SPEC>;
#[doc = "Graphic MMU LUT entry 45 high"]
pub mod lut45h;
#[doc = "LUT46H register accessor: an alias for `Reg<LUT46H_SPEC>`"]
pub type LUT46H = crate::Reg<lut46h::LUT46H_SPEC>;
#[doc = "Graphic MMU LUT entry 46 high"]
pub mod lut46h;
#[doc = "LUT47H register accessor: an alias for `Reg<LUT47H_SPEC>`"]
pub type LUT47H = crate::Reg<lut47h::LUT47H_SPEC>;
#[doc = "Graphic MMU LUT entry 47 high"]
pub mod lut47h;
#[doc = "LUT48H register accessor: an alias for `Reg<LUT48H_SPEC>`"]
pub type LUT48H = crate::Reg<lut48h::LUT48H_SPEC>;
#[doc = "Graphic MMU LUT entry 48 high"]
pub mod lut48h;
#[doc = "LUT49H register accessor: an alias for `Reg<LUT49H_SPEC>`"]
pub type LUT49H = crate::Reg<lut49h::LUT49H_SPEC>;
#[doc = "Graphic MMU LUT entry 49 high"]
pub mod lut49h;
#[doc = "LUT50H register accessor: an alias for `Reg<LUT50H_SPEC>`"]
pub type LUT50H = crate::Reg<lut50h::LUT50H_SPEC>;
#[doc = "Graphic MMU LUT entry 50 high"]
pub mod lut50h;
#[doc = "LUT51H register accessor: an alias for `Reg<LUT51H_SPEC>`"]
pub type LUT51H = crate::Reg<lut51h::LUT51H_SPEC>;
#[doc = "Graphic MMU LUT entry 51 high"]
pub mod lut51h;
#[doc = "LUT52H register accessor: an alias for `Reg<LUT52H_SPEC>`"]
pub type LUT52H = crate::Reg<lut52h::LUT52H_SPEC>;
#[doc = "Graphic MMU LUT entry 52 high"]
pub mod lut52h;
#[doc = "LUT53H register accessor: an alias for `Reg<LUT53H_SPEC>`"]
pub type LUT53H = crate::Reg<lut53h::LUT53H_SPEC>;
#[doc = "Graphic MMU LUT entry 53 high"]
pub mod lut53h;
#[doc = "LUT54H register accessor: an alias for `Reg<LUT54H_SPEC>`"]
pub type LUT54H = crate::Reg<lut54h::LUT54H_SPEC>;
#[doc = "Graphic MMU LUT entry 54 high"]
pub mod lut54h;
#[doc = "LUT55H register accessor: an alias for `Reg<LUT55H_SPEC>`"]
pub type LUT55H = crate::Reg<lut55h::LUT55H_SPEC>;
#[doc = "Graphic MMU LUT entry 55 high"]
pub mod lut55h;
#[doc = "LUT56H register accessor: an alias for `Reg<LUT56H_SPEC>`"]
pub type LUT56H = crate::Reg<lut56h::LUT56H_SPEC>;
#[doc = "Graphic MMU LUT entry 56 high"]
pub mod lut56h;
#[doc = "LUT57H register accessor: an alias for `Reg<LUT57H_SPEC>`"]
pub type LUT57H = crate::Reg<lut57h::LUT57H_SPEC>;
#[doc = "Graphic MMU LUT entry 57 high"]
pub mod lut57h;
#[doc = "LUT58H register accessor: an alias for `Reg<LUT58H_SPEC>`"]
pub type LUT58H = crate::Reg<lut58h::LUT58H_SPEC>;
#[doc = "Graphic MMU LUT entry 58 high"]
pub mod lut58h;
#[doc = "LUT59H register accessor: an alias for `Reg<LUT59H_SPEC>`"]
pub type LUT59H = crate::Reg<lut59h::LUT59H_SPEC>;
#[doc = "Graphic MMU LUT entry 59 high"]
pub mod lut59h;
#[doc = "LUT60H register accessor: an alias for `Reg<LUT60H_SPEC>`"]
pub type LUT60H = crate::Reg<lut60h::LUT60H_SPEC>;
#[doc = "Graphic MMU LUT entry 60 high"]
pub mod lut60h;
#[doc = "LUT61H register accessor: an alias for `Reg<LUT61H_SPEC>`"]
pub type LUT61H = crate::Reg<lut61h::LUT61H_SPEC>;
#[doc = "Graphic MMU LUT entry 61 high"]
pub mod lut61h;
#[doc = "LUT62H register accessor: an alias for `Reg<LUT62H_SPEC>`"]
pub type LUT62H = crate::Reg<lut62h::LUT62H_SPEC>;
#[doc = "Graphic MMU LUT entry 62 high"]
pub mod lut62h;
#[doc = "LUT63H register accessor: an alias for `Reg<LUT63H_SPEC>`"]
pub type LUT63H = crate::Reg<lut63h::LUT63H_SPEC>;
#[doc = "Graphic MMU LUT entry 63 high"]
pub mod lut63h;
#[doc = "LUT64H register accessor: an alias for `Reg<LUT64H_SPEC>`"]
pub type LUT64H = crate::Reg<lut64h::LUT64H_SPEC>;
#[doc = "Graphic MMU LUT entry 64 high"]
pub mod lut64h;
#[doc = "LUT65H register accessor: an alias for `Reg<LUT65H_SPEC>`"]
pub type LUT65H = crate::Reg<lut65h::LUT65H_SPEC>;
#[doc = "Graphic MMU LUT entry 65 high"]
pub mod lut65h;
#[doc = "LUT66H register accessor: an alias for `Reg<LUT66H_SPEC>`"]
pub type LUT66H = crate::Reg<lut66h::LUT66H_SPEC>;
#[doc = "Graphic MMU LUT entry 66 high"]
pub mod lut66h;
#[doc = "LUT67H register accessor: an alias for `Reg<LUT67H_SPEC>`"]
pub type LUT67H = crate::Reg<lut67h::LUT67H_SPEC>;
#[doc = "Graphic MMU LUT entry 67 high"]
pub mod lut67h;
#[doc = "LUT68H register accessor: an alias for `Reg<LUT68H_SPEC>`"]
pub type LUT68H = crate::Reg<lut68h::LUT68H_SPEC>;
#[doc = "Graphic MMU LUT entry 68 high"]
pub mod lut68h;
#[doc = "LUT69H register accessor: an alias for `Reg<LUT69H_SPEC>`"]
pub type LUT69H = crate::Reg<lut69h::LUT69H_SPEC>;
#[doc = "Graphic MMU LUT entry 69 high"]
pub mod lut69h;
#[doc = "LUT70H register accessor: an alias for `Reg<LUT70H_SPEC>`"]
pub type LUT70H = crate::Reg<lut70h::LUT70H_SPEC>;
#[doc = "Graphic MMU LUT entry 70 high"]
pub mod lut70h;
#[doc = "LUT71H register accessor: an alias for `Reg<LUT71H_SPEC>`"]
pub type LUT71H = crate::Reg<lut71h::LUT71H_SPEC>;
#[doc = "Graphic MMU LUT entry 71 high"]
pub mod lut71h;
#[doc = "LUT72H register accessor: an alias for `Reg<LUT72H_SPEC>`"]
pub type LUT72H = crate::Reg<lut72h::LUT72H_SPEC>;
#[doc = "Graphic MMU LUT entry 72 high"]
pub mod lut72h;
#[doc = "LUT73H register accessor: an alias for `Reg<LUT73H_SPEC>`"]
pub type LUT73H = crate::Reg<lut73h::LUT73H_SPEC>;
#[doc = "Graphic MMU LUT entry 73 high"]
pub mod lut73h;
#[doc = "LUT74H register accessor: an alias for `Reg<LUT74H_SPEC>`"]
pub type LUT74H = crate::Reg<lut74h::LUT74H_SPEC>;
#[doc = "Graphic MMU LUT entry 74 high"]
pub mod lut74h;
#[doc = "LUT75H register accessor: an alias for `Reg<LUT75H_SPEC>`"]
pub type LUT75H = crate::Reg<lut75h::LUT75H_SPEC>;
#[doc = "Graphic MMU LUT entry 75 high"]
pub mod lut75h;
#[doc = "LUT76H register accessor: an alias for `Reg<LUT76H_SPEC>`"]
pub type LUT76H = crate::Reg<lut76h::LUT76H_SPEC>;
#[doc = "Graphic MMU LUT entry 76 high"]
pub mod lut76h;
#[doc = "LUT77H register accessor: an alias for `Reg<LUT77H_SPEC>`"]
pub type LUT77H = crate::Reg<lut77h::LUT77H_SPEC>;
#[doc = "Graphic MMU LUT entry 77 high"]
pub mod lut77h;
#[doc = "LUT78H register accessor: an alias for `Reg<LUT78H_SPEC>`"]
pub type LUT78H = crate::Reg<lut78h::LUT78H_SPEC>;
#[doc = "Graphic MMU LUT entry 78 high"]
pub mod lut78h;
#[doc = "LUT79H register accessor: an alias for `Reg<LUT79H_SPEC>`"]
pub type LUT79H = crate::Reg<lut79h::LUT79H_SPEC>;
#[doc = "Graphic MMU LUT entry 79 high"]
pub mod lut79h;
#[doc = "LUT80H register accessor: an alias for `Reg<LUT80H_SPEC>`"]
pub type LUT80H = crate::Reg<lut80h::LUT80H_SPEC>;
#[doc = "Graphic MMU LUT entry 80 high"]
pub mod lut80h;
#[doc = "LUT81H register accessor: an alias for `Reg<LUT81H_SPEC>`"]
pub type LUT81H = crate::Reg<lut81h::LUT81H_SPEC>;
#[doc = "Graphic MMU LUT entry 81 high"]
pub mod lut81h;
#[doc = "LUT82H register accessor: an alias for `Reg<LUT82H_SPEC>`"]
pub type LUT82H = crate::Reg<lut82h::LUT82H_SPEC>;
#[doc = "Graphic MMU LUT entry 82 high"]
pub mod lut82h;
#[doc = "LUT83H register accessor: an alias for `Reg<LUT83H_SPEC>`"]
pub type LUT83H = crate::Reg<lut83h::LUT83H_SPEC>;
#[doc = "Graphic MMU LUT entry 83 high"]
pub mod lut83h;
#[doc = "LUT84H register accessor: an alias for `Reg<LUT84H_SPEC>`"]
pub type LUT84H = crate::Reg<lut84h::LUT84H_SPEC>;
#[doc = "Graphic MMU LUT entry 84 high"]
pub mod lut84h;
#[doc = "LUT85H register accessor: an alias for `Reg<LUT85H_SPEC>`"]
pub type LUT85H = crate::Reg<lut85h::LUT85H_SPEC>;
#[doc = "Graphic MMU LUT entry 85 high"]
pub mod lut85h;
#[doc = "LUT86H register accessor: an alias for `Reg<LUT86H_SPEC>`"]
pub type LUT86H = crate::Reg<lut86h::LUT86H_SPEC>;
#[doc = "Graphic MMU LUT entry 86 high"]
pub mod lut86h;
#[doc = "LUT87H register accessor: an alias for `Reg<LUT87H_SPEC>`"]
pub type LUT87H = crate::Reg<lut87h::LUT87H_SPEC>;
#[doc = "Graphic MMU LUT entry 87 high"]
pub mod lut87h;
#[doc = "LUT88H register accessor: an alias for `Reg<LUT88H_SPEC>`"]
pub type LUT88H = crate::Reg<lut88h::LUT88H_SPEC>;
#[doc = "Graphic MMU LUT entry 88 high"]
pub mod lut88h;
#[doc = "LUT89H register accessor: an alias for `Reg<LUT89H_SPEC>`"]
pub type LUT89H = crate::Reg<lut89h::LUT89H_SPEC>;
#[doc = "Graphic MMU LUT entry 89 high"]
pub mod lut89h;
#[doc = "LUT90H register accessor: an alias for `Reg<LUT90H_SPEC>`"]
pub type LUT90H = crate::Reg<lut90h::LUT90H_SPEC>;
#[doc = "Graphic MMU LUT entry 90 high"]
pub mod lut90h;
#[doc = "LUT91H register accessor: an alias for `Reg<LUT91H_SPEC>`"]
pub type LUT91H = crate::Reg<lut91h::LUT91H_SPEC>;
#[doc = "Graphic MMU LUT entry 91 high"]
pub mod lut91h;
#[doc = "LUT92H register accessor: an alias for `Reg<LUT92H_SPEC>`"]
pub type LUT92H = crate::Reg<lut92h::LUT92H_SPEC>;
#[doc = "Graphic MMU LUT entry 92 high"]
pub mod lut92h;
#[doc = "LUT93H register accessor: an alias for `Reg<LUT93H_SPEC>`"]
pub type LUT93H = crate::Reg<lut93h::LUT93H_SPEC>;
#[doc = "Graphic MMU LUT entry 93 high"]
pub mod lut93h;
#[doc = "LUT94H register accessor: an alias for `Reg<LUT94H_SPEC>`"]
pub type LUT94H = crate::Reg<lut94h::LUT94H_SPEC>;
#[doc = "Graphic MMU LUT entry 94 high"]
pub mod lut94h;
#[doc = "LUT95H register accessor: an alias for `Reg<LUT95H_SPEC>`"]
pub type LUT95H = crate::Reg<lut95h::LUT95H_SPEC>;
#[doc = "Graphic MMU LUT entry 95 high"]
pub mod lut95h;
#[doc = "LUT96H register accessor: an alias for `Reg<LUT96H_SPEC>`"]
pub type LUT96H = crate::Reg<lut96h::LUT96H_SPEC>;
#[doc = "Graphic MMU LUT entry 96 high"]
pub mod lut96h;
#[doc = "LUT97H register accessor: an alias for `Reg<LUT97H_SPEC>`"]
pub type LUT97H = crate::Reg<lut97h::LUT97H_SPEC>;
#[doc = "Graphic MMU LUT entry 97 high"]
pub mod lut97h;
#[doc = "LUT98H register accessor: an alias for `Reg<LUT98H_SPEC>`"]
pub type LUT98H = crate::Reg<lut98h::LUT98H_SPEC>;
#[doc = "Graphic MMU LUT entry 98 high"]
pub mod lut98h;
#[doc = "LUT99H register accessor: an alias for `Reg<LUT99H_SPEC>`"]
pub type LUT99H = crate::Reg<lut99h::LUT99H_SPEC>;
#[doc = "Graphic MMU LUT entry 99 high"]
pub mod lut99h;
#[doc = "LUT100H register accessor: an alias for `Reg<LUT100H_SPEC>`"]
pub type LUT100H = crate::Reg<lut100h::LUT100H_SPEC>;
#[doc = "Graphic MMU LUT entry 100 high"]
pub mod lut100h;
#[doc = "LUT101H register accessor: an alias for `Reg<LUT101H_SPEC>`"]
pub type LUT101H = crate::Reg<lut101h::LUT101H_SPEC>;
#[doc = "Graphic MMU LUT entry 101 high"]
pub mod lut101h;
#[doc = "LUT102H register accessor: an alias for `Reg<LUT102H_SPEC>`"]
pub type LUT102H = crate::Reg<lut102h::LUT102H_SPEC>;
#[doc = "Graphic MMU LUT entry 102 high"]
pub mod lut102h;
#[doc = "LUT103H register accessor: an alias for `Reg<LUT103H_SPEC>`"]
pub type LUT103H = crate::Reg<lut103h::LUT103H_SPEC>;
#[doc = "Graphic MMU LUT entry 103 high"]
pub mod lut103h;
#[doc = "LUT104H register accessor: an alias for `Reg<LUT104H_SPEC>`"]
pub type LUT104H = crate::Reg<lut104h::LUT104H_SPEC>;
#[doc = "Graphic MMU LUT entry 104 high"]
pub mod lut104h;
#[doc = "LUT105H register accessor: an alias for `Reg<LUT105H_SPEC>`"]
pub type LUT105H = crate::Reg<lut105h::LUT105H_SPEC>;
#[doc = "Graphic MMU LUT entry 105 high"]
pub mod lut105h;
#[doc = "LUT106H register accessor: an alias for `Reg<LUT106H_SPEC>`"]
pub type LUT106H = crate::Reg<lut106h::LUT106H_SPEC>;
#[doc = "Graphic MMU LUT entry 106 high"]
pub mod lut106h;
#[doc = "LUT107H register accessor: an alias for `Reg<LUT107H_SPEC>`"]
pub type LUT107H = crate::Reg<lut107h::LUT107H_SPEC>;
#[doc = "Graphic MMU LUT entry 107 high"]
pub mod lut107h;
#[doc = "LUT108H register accessor: an alias for `Reg<LUT108H_SPEC>`"]
pub type LUT108H = crate::Reg<lut108h::LUT108H_SPEC>;
#[doc = "Graphic MMU LUT entry 108 high"]
pub mod lut108h;
#[doc = "LUT109H register accessor: an alias for `Reg<LUT109H_SPEC>`"]
pub type LUT109H = crate::Reg<lut109h::LUT109H_SPEC>;
#[doc = "Graphic MMU LUT entry 109 high"]
pub mod lut109h;
#[doc = "LUT110H register accessor: an alias for `Reg<LUT110H_SPEC>`"]
pub type LUT110H = crate::Reg<lut110h::LUT110H_SPEC>;
#[doc = "Graphic MMU LUT entry 110 high"]
pub mod lut110h;
#[doc = "LUT111H register accessor: an alias for `Reg<LUT111H_SPEC>`"]
pub type LUT111H = crate::Reg<lut111h::LUT111H_SPEC>;
#[doc = "Graphic MMU LUT entry 111 high"]
pub mod lut111h;
#[doc = "LUT112H register accessor: an alias for `Reg<LUT112H_SPEC>`"]
pub type LUT112H = crate::Reg<lut112h::LUT112H_SPEC>;
#[doc = "Graphic MMU LUT entry 112 high"]
pub mod lut112h;
#[doc = "LUT113H register accessor: an alias for `Reg<LUT113H_SPEC>`"]
pub type LUT113H = crate::Reg<lut113h::LUT113H_SPEC>;
#[doc = "Graphic MMU LUT entry 113 high"]
pub mod lut113h;
#[doc = "LUT114H register accessor: an alias for `Reg<LUT114H_SPEC>`"]
pub type LUT114H = crate::Reg<lut114h::LUT114H_SPEC>;
#[doc = "Graphic MMU LUT entry 114 high"]
pub mod lut114h;
#[doc = "LUT115H register accessor: an alias for `Reg<LUT115H_SPEC>`"]
pub type LUT115H = crate::Reg<lut115h::LUT115H_SPEC>;
#[doc = "Graphic MMU LUT entry 115 high"]
pub mod lut115h;
#[doc = "LUT116H register accessor: an alias for `Reg<LUT116H_SPEC>`"]
pub type LUT116H = crate::Reg<lut116h::LUT116H_SPEC>;
#[doc = "Graphic MMU LUT entry 116 high"]
pub mod lut116h;
#[doc = "LUT117H register accessor: an alias for `Reg<LUT117H_SPEC>`"]
pub type LUT117H = crate::Reg<lut117h::LUT117H_SPEC>;
#[doc = "Graphic MMU LUT entry 117 high"]
pub mod lut117h;
#[doc = "LUT118H register accessor: an alias for `Reg<LUT118H_SPEC>`"]
pub type LUT118H = crate::Reg<lut118h::LUT118H_SPEC>;
#[doc = "Graphic MMU LUT entry 118 high"]
pub mod lut118h;
#[doc = "LUT119H register accessor: an alias for `Reg<LUT119H_SPEC>`"]
pub type LUT119H = crate::Reg<lut119h::LUT119H_SPEC>;
#[doc = "Graphic MMU LUT entry 119 high"]
pub mod lut119h;
#[doc = "LUT120H register accessor: an alias for `Reg<LUT120H_SPEC>`"]
pub type LUT120H = crate::Reg<lut120h::LUT120H_SPEC>;
#[doc = "Graphic MMU LUT entry 120 high"]
pub mod lut120h;
#[doc = "LUT121H register accessor: an alias for `Reg<LUT121H_SPEC>`"]
pub type LUT121H = crate::Reg<lut121h::LUT121H_SPEC>;
#[doc = "Graphic MMU LUT entry 121 high"]
pub mod lut121h;
#[doc = "LUT122H register accessor: an alias for `Reg<LUT122H_SPEC>`"]
pub type LUT122H = crate::Reg<lut122h::LUT122H_SPEC>;
#[doc = "Graphic MMU LUT entry 122 high"]
pub mod lut122h;
#[doc = "LUT123H register accessor: an alias for `Reg<LUT123H_SPEC>`"]
pub type LUT123H = crate::Reg<lut123h::LUT123H_SPEC>;
#[doc = "Graphic MMU LUT entry 123 high"]
pub mod lut123h;
#[doc = "LUT124H register accessor: an alias for `Reg<LUT124H_SPEC>`"]
pub type LUT124H = crate::Reg<lut124h::LUT124H_SPEC>;
#[doc = "Graphic MMU LUT entry 124 high"]
pub mod lut124h;
#[doc = "LUT125H register accessor: an alias for `Reg<LUT125H_SPEC>`"]
pub type LUT125H = crate::Reg<lut125h::LUT125H_SPEC>;
#[doc = "Graphic MMU LUT entry 125 high"]
pub mod lut125h;
#[doc = "LUT126H register accessor: an alias for `Reg<LUT126H_SPEC>`"]
pub type LUT126H = crate::Reg<lut126h::LUT126H_SPEC>;
#[doc = "Graphic MMU LUT entry 126 high"]
pub mod lut126h;
#[doc = "LUT127H register accessor: an alias for `Reg<LUT127H_SPEC>`"]
pub type LUT127H = crate::Reg<lut127h::LUT127H_SPEC>;
#[doc = "Graphic MMU LUT entry 127 high"]
pub mod lut127h;
#[doc = "LUT128H register accessor: an alias for `Reg<LUT128H_SPEC>`"]
pub type LUT128H = crate::Reg<lut128h::LUT128H_SPEC>;
#[doc = "Graphic MMU LUT entry 128 high"]
pub mod lut128h;
#[doc = "LUT129H register accessor: an alias for `Reg<LUT129H_SPEC>`"]
pub type LUT129H = crate::Reg<lut129h::LUT129H_SPEC>;
#[doc = "Graphic MMU LUT entry 129 high"]
pub mod lut129h;
#[doc = "LUT130H register accessor: an alias for `Reg<LUT130H_SPEC>`"]
pub type LUT130H = crate::Reg<lut130h::LUT130H_SPEC>;
#[doc = "Graphic MMU LUT entry 130 high"]
pub mod lut130h;
#[doc = "LUT131H register accessor: an alias for `Reg<LUT131H_SPEC>`"]
pub type LUT131H = crate::Reg<lut131h::LUT131H_SPEC>;
#[doc = "Graphic MMU LUT entry 131 high"]
pub mod lut131h;
#[doc = "LUT132H register accessor: an alias for `Reg<LUT132H_SPEC>`"]
pub type LUT132H = crate::Reg<lut132h::LUT132H_SPEC>;
#[doc = "Graphic MMU LUT entry 132 high"]
pub mod lut132h;
#[doc = "LUT133H register accessor: an alias for `Reg<LUT133H_SPEC>`"]
pub type LUT133H = crate::Reg<lut133h::LUT133H_SPEC>;
#[doc = "Graphic MMU LUT entry 133 high"]
pub mod lut133h;
#[doc = "LUT134H register accessor: an alias for `Reg<LUT134H_SPEC>`"]
pub type LUT134H = crate::Reg<lut134h::LUT134H_SPEC>;
#[doc = "Graphic MMU LUT entry 134 high"]
pub mod lut134h;
#[doc = "LUT135H register accessor: an alias for `Reg<LUT135H_SPEC>`"]
pub type LUT135H = crate::Reg<lut135h::LUT135H_SPEC>;
#[doc = "Graphic MMU LUT entry 135 high"]
pub mod lut135h;
#[doc = "LUT136H register accessor: an alias for `Reg<LUT136H_SPEC>`"]
pub type LUT136H = crate::Reg<lut136h::LUT136H_SPEC>;
#[doc = "Graphic MMU LUT entry 136 high"]
pub mod lut136h;
#[doc = "LUT137H register accessor: an alias for `Reg<LUT137H_SPEC>`"]
pub type LUT137H = crate::Reg<lut137h::LUT137H_SPEC>;
#[doc = "Graphic MMU LUT entry 137 high"]
pub mod lut137h;
#[doc = "LUT138H register accessor: an alias for `Reg<LUT138H_SPEC>`"]
pub type LUT138H = crate::Reg<lut138h::LUT138H_SPEC>;
#[doc = "Graphic MMU LUT entry 138 high"]
pub mod lut138h;
#[doc = "LUT139H register accessor: an alias for `Reg<LUT139H_SPEC>`"]
pub type LUT139H = crate::Reg<lut139h::LUT139H_SPEC>;
#[doc = "Graphic MMU LUT entry 139 high"]
pub mod lut139h;
#[doc = "LUT140H register accessor: an alias for `Reg<LUT140H_SPEC>`"]
pub type LUT140H = crate::Reg<lut140h::LUT140H_SPEC>;
#[doc = "Graphic MMU LUT entry 140 high"]
pub mod lut140h;
#[doc = "LUT141H register accessor: an alias for `Reg<LUT141H_SPEC>`"]
pub type LUT141H = crate::Reg<lut141h::LUT141H_SPEC>;
#[doc = "Graphic MMU LUT entry 141 high"]
pub mod lut141h;
#[doc = "LUT142H register accessor: an alias for `Reg<LUT142H_SPEC>`"]
pub type LUT142H = crate::Reg<lut142h::LUT142H_SPEC>;
#[doc = "Graphic MMU LUT entry 142 high"]
pub mod lut142h;
#[doc = "LUT143H register accessor: an alias for `Reg<LUT143H_SPEC>`"]
pub type LUT143H = crate::Reg<lut143h::LUT143H_SPEC>;
#[doc = "Graphic MMU LUT entry 143 high"]
pub mod lut143h;
#[doc = "LUT144H register accessor: an alias for `Reg<LUT144H_SPEC>`"]
pub type LUT144H = crate::Reg<lut144h::LUT144H_SPEC>;
#[doc = "Graphic MMU LUT entry 144 high"]
pub mod lut144h;
#[doc = "LUT145H register accessor: an alias for `Reg<LUT145H_SPEC>`"]
pub type LUT145H = crate::Reg<lut145h::LUT145H_SPEC>;
#[doc = "Graphic MMU LUT entry 145 high"]
pub mod lut145h;
#[doc = "LUT146H register accessor: an alias for `Reg<LUT146H_SPEC>`"]
pub type LUT146H = crate::Reg<lut146h::LUT146H_SPEC>;
#[doc = "Graphic MMU LUT entry 146 high"]
pub mod lut146h;
#[doc = "LUT147H register accessor: an alias for `Reg<LUT147H_SPEC>`"]
pub type LUT147H = crate::Reg<lut147h::LUT147H_SPEC>;
#[doc = "Graphic MMU LUT entry 147 high"]
pub mod lut147h;
#[doc = "LUT148H register accessor: an alias for `Reg<LUT148H_SPEC>`"]
pub type LUT148H = crate::Reg<lut148h::LUT148H_SPEC>;
#[doc = "Graphic MMU LUT entry 148 high"]
pub mod lut148h;
#[doc = "LUT149H register accessor: an alias for `Reg<LUT149H_SPEC>`"]
pub type LUT149H = crate::Reg<lut149h::LUT149H_SPEC>;
#[doc = "Graphic MMU LUT entry 149 high"]
pub mod lut149h;
#[doc = "LUT150H register accessor: an alias for `Reg<LUT150H_SPEC>`"]
pub type LUT150H = crate::Reg<lut150h::LUT150H_SPEC>;
#[doc = "Graphic MMU LUT entry 150 high"]
pub mod lut150h;
#[doc = "LUT151H register accessor: an alias for `Reg<LUT151H_SPEC>`"]
pub type LUT151H = crate::Reg<lut151h::LUT151H_SPEC>;
#[doc = "Graphic MMU LUT entry 151 high"]
pub mod lut151h;
#[doc = "LUT152H register accessor: an alias for `Reg<LUT152H_SPEC>`"]
pub type LUT152H = crate::Reg<lut152h::LUT152H_SPEC>;
#[doc = "Graphic MMU LUT entry 152 high"]
pub mod lut152h;
#[doc = "LUT153H register accessor: an alias for `Reg<LUT153H_SPEC>`"]
pub type LUT153H = crate::Reg<lut153h::LUT153H_SPEC>;
#[doc = "Graphic MMU LUT entry 153 high"]
pub mod lut153h;
#[doc = "LUT154H register accessor: an alias for `Reg<LUT154H_SPEC>`"]
pub type LUT154H = crate::Reg<lut154h::LUT154H_SPEC>;
#[doc = "Graphic MMU LUT entry 154 high"]
pub mod lut154h;
#[doc = "LUT155H register accessor: an alias for `Reg<LUT155H_SPEC>`"]
pub type LUT155H = crate::Reg<lut155h::LUT155H_SPEC>;
#[doc = "Graphic MMU LUT entry 155 high"]
pub mod lut155h;
#[doc = "LUT156H register accessor: an alias for `Reg<LUT156H_SPEC>`"]
pub type LUT156H = crate::Reg<lut156h::LUT156H_SPEC>;
#[doc = "Graphic MMU LUT entry 156 high"]
pub mod lut156h;
#[doc = "LUT157H register accessor: an alias for `Reg<LUT157H_SPEC>`"]
pub type LUT157H = crate::Reg<lut157h::LUT157H_SPEC>;
#[doc = "Graphic MMU LUT entry 157 high"]
pub mod lut157h;
#[doc = "LUT158H register accessor: an alias for `Reg<LUT158H_SPEC>`"]
pub type LUT158H = crate::Reg<lut158h::LUT158H_SPEC>;
#[doc = "Graphic MMU LUT entry 158 high"]
pub mod lut158h;
#[doc = "LUT159H register accessor: an alias for `Reg<LUT159H_SPEC>`"]
pub type LUT159H = crate::Reg<lut159h::LUT159H_SPEC>;
#[doc = "Graphic MMU LUT entry 159 high"]
pub mod lut159h;
#[doc = "LUT160H register accessor: an alias for `Reg<LUT160H_SPEC>`"]
pub type LUT160H = crate::Reg<lut160h::LUT160H_SPEC>;
#[doc = "Graphic MMU LUT entry 160 high"]
pub mod lut160h;
#[doc = "LUT161H register accessor: an alias for `Reg<LUT161H_SPEC>`"]
pub type LUT161H = crate::Reg<lut161h::LUT161H_SPEC>;
#[doc = "Graphic MMU LUT entry 161 high"]
pub mod lut161h;
#[doc = "LUT162H register accessor: an alias for `Reg<LUT162H_SPEC>`"]
pub type LUT162H = crate::Reg<lut162h::LUT162H_SPEC>;
#[doc = "Graphic MMU LUT entry 162 high"]
pub mod lut162h;
#[doc = "LUT163H register accessor: an alias for `Reg<LUT163H_SPEC>`"]
pub type LUT163H = crate::Reg<lut163h::LUT163H_SPEC>;
#[doc = "Graphic MMU LUT entry 163 high"]
pub mod lut163h;
#[doc = "LUT164H register accessor: an alias for `Reg<LUT164H_SPEC>`"]
pub type LUT164H = crate::Reg<lut164h::LUT164H_SPEC>;
#[doc = "Graphic MMU LUT entry 164 high"]
pub mod lut164h;
#[doc = "LUT165H register accessor: an alias for `Reg<LUT165H_SPEC>`"]
pub type LUT165H = crate::Reg<lut165h::LUT165H_SPEC>;
#[doc = "Graphic MMU LUT entry 165 high"]
pub mod lut165h;
#[doc = "LUT166H register accessor: an alias for `Reg<LUT166H_SPEC>`"]
pub type LUT166H = crate::Reg<lut166h::LUT166H_SPEC>;
#[doc = "Graphic MMU LUT entry 166 high"]
pub mod lut166h;
#[doc = "LUT167H register accessor: an alias for `Reg<LUT167H_SPEC>`"]
pub type LUT167H = crate::Reg<lut167h::LUT167H_SPEC>;
#[doc = "Graphic MMU LUT entry 167 high"]
pub mod lut167h;
#[doc = "LUT168H register accessor: an alias for `Reg<LUT168H_SPEC>`"]
pub type LUT168H = crate::Reg<lut168h::LUT168H_SPEC>;
#[doc = "Graphic MMU LUT entry 168 high"]
pub mod lut168h;
#[doc = "LUT169H register accessor: an alias for `Reg<LUT169H_SPEC>`"]
pub type LUT169H = crate::Reg<lut169h::LUT169H_SPEC>;
#[doc = "Graphic MMU LUT entry 169 high"]
pub mod lut169h;
#[doc = "LUT170H register accessor: an alias for `Reg<LUT170H_SPEC>`"]
pub type LUT170H = crate::Reg<lut170h::LUT170H_SPEC>;
#[doc = "Graphic MMU LUT entry 170 high"]
pub mod lut170h;
#[doc = "LUT171H register accessor: an alias for `Reg<LUT171H_SPEC>`"]
pub type LUT171H = crate::Reg<lut171h::LUT171H_SPEC>;
#[doc = "Graphic MMU LUT entry 171 high"]
pub mod lut171h;
#[doc = "LUT172H register accessor: an alias for `Reg<LUT172H_SPEC>`"]
pub type LUT172H = crate::Reg<lut172h::LUT172H_SPEC>;
#[doc = "Graphic MMU LUT entry 172 high"]
pub mod lut172h;
#[doc = "LUT173H register accessor: an alias for `Reg<LUT173H_SPEC>`"]
pub type LUT173H = crate::Reg<lut173h::LUT173H_SPEC>;
#[doc = "Graphic MMU LUT entry 173 high"]
pub mod lut173h;
#[doc = "LUT174H register accessor: an alias for `Reg<LUT174H_SPEC>`"]
pub type LUT174H = crate::Reg<lut174h::LUT174H_SPEC>;
#[doc = "Graphic MMU LUT entry 174 high"]
pub mod lut174h;
#[doc = "LUT175H register accessor: an alias for `Reg<LUT175H_SPEC>`"]
pub type LUT175H = crate::Reg<lut175h::LUT175H_SPEC>;
#[doc = "Graphic MMU LUT entry 175 high"]
pub mod lut175h;
#[doc = "LUT176H register accessor: an alias for `Reg<LUT176H_SPEC>`"]
pub type LUT176H = crate::Reg<lut176h::LUT176H_SPEC>;
#[doc = "Graphic MMU LUT entry 176 high"]
pub mod lut176h;
#[doc = "LUT177H register accessor: an alias for `Reg<LUT177H_SPEC>`"]
pub type LUT177H = crate::Reg<lut177h::LUT177H_SPEC>;
#[doc = "Graphic MMU LUT entry 177 high"]
pub mod lut177h;
#[doc = "LUT178H register accessor: an alias for `Reg<LUT178H_SPEC>`"]
pub type LUT178H = crate::Reg<lut178h::LUT178H_SPEC>;
#[doc = "Graphic MMU LUT entry 178 high"]
pub mod lut178h;
#[doc = "LUT179H register accessor: an alias for `Reg<LUT179H_SPEC>`"]
pub type LUT179H = crate::Reg<lut179h::LUT179H_SPEC>;
#[doc = "Graphic MMU LUT entry 179 high"]
pub mod lut179h;
#[doc = "LUT180H register accessor: an alias for `Reg<LUT180H_SPEC>`"]
pub type LUT180H = crate::Reg<lut180h::LUT180H_SPEC>;
#[doc = "Graphic MMU LUT entry 180 high"]
pub mod lut180h;
#[doc = "LUT181H register accessor: an alias for `Reg<LUT181H_SPEC>`"]
pub type LUT181H = crate::Reg<lut181h::LUT181H_SPEC>;
#[doc = "Graphic MMU LUT entry 181 high"]
pub mod lut181h;
#[doc = "LUT182H register accessor: an alias for `Reg<LUT182H_SPEC>`"]
pub type LUT182H = crate::Reg<lut182h::LUT182H_SPEC>;
#[doc = "Graphic MMU LUT entry 182 high"]
pub mod lut182h;
#[doc = "LUT183H register accessor: an alias for `Reg<LUT183H_SPEC>`"]
pub type LUT183H = crate::Reg<lut183h::LUT183H_SPEC>;
#[doc = "Graphic MMU LUT entry 183 high"]
pub mod lut183h;
#[doc = "LUT184H register accessor: an alias for `Reg<LUT184H_SPEC>`"]
pub type LUT184H = crate::Reg<lut184h::LUT184H_SPEC>;
#[doc = "Graphic MMU LUT entry 184 high"]
pub mod lut184h;
#[doc = "LUT185H register accessor: an alias for `Reg<LUT185H_SPEC>`"]
pub type LUT185H = crate::Reg<lut185h::LUT185H_SPEC>;
#[doc = "Graphic MMU LUT entry 185 high"]
pub mod lut185h;
#[doc = "LUT186H register accessor: an alias for `Reg<LUT186H_SPEC>`"]
pub type LUT186H = crate::Reg<lut186h::LUT186H_SPEC>;
#[doc = "Graphic MMU LUT entry 186 high"]
pub mod lut186h;
#[doc = "LUT187H register accessor: an alias for `Reg<LUT187H_SPEC>`"]
pub type LUT187H = crate::Reg<lut187h::LUT187H_SPEC>;
#[doc = "Graphic MMU LUT entry 187 high"]
pub mod lut187h;
#[doc = "LUT188H register accessor: an alias for `Reg<LUT188H_SPEC>`"]
pub type LUT188H = crate::Reg<lut188h::LUT188H_SPEC>;
#[doc = "Graphic MMU LUT entry 188 high"]
pub mod lut188h;
#[doc = "LUT189H register accessor: an alias for `Reg<LUT189H_SPEC>`"]
pub type LUT189H = crate::Reg<lut189h::LUT189H_SPEC>;
#[doc = "Graphic MMU LUT entry 189 high"]
pub mod lut189h;
#[doc = "LUT190H register accessor: an alias for `Reg<LUT190H_SPEC>`"]
pub type LUT190H = crate::Reg<lut190h::LUT190H_SPEC>;
#[doc = "Graphic MMU LUT entry 190 high"]
pub mod lut190h;
#[doc = "LUT191H register accessor: an alias for `Reg<LUT191H_SPEC>`"]
pub type LUT191H = crate::Reg<lut191h::LUT191H_SPEC>;
#[doc = "Graphic MMU LUT entry 191 high"]
pub mod lut191h;
#[doc = "LUT192H register accessor: an alias for `Reg<LUT192H_SPEC>`"]
pub type LUT192H = crate::Reg<lut192h::LUT192H_SPEC>;
#[doc = "Graphic MMU LUT entry 192 high"]
pub mod lut192h;
#[doc = "LUT193H register accessor: an alias for `Reg<LUT193H_SPEC>`"]
pub type LUT193H = crate::Reg<lut193h::LUT193H_SPEC>;
#[doc = "Graphic MMU LUT entry 193 high"]
pub mod lut193h;
#[doc = "LUT194H register accessor: an alias for `Reg<LUT194H_SPEC>`"]
pub type LUT194H = crate::Reg<lut194h::LUT194H_SPEC>;
#[doc = "Graphic MMU LUT entry 194 high"]
pub mod lut194h;
#[doc = "LUT195H register accessor: an alias for `Reg<LUT195H_SPEC>`"]
pub type LUT195H = crate::Reg<lut195h::LUT195H_SPEC>;
#[doc = "Graphic MMU LUT entry 195 high"]
pub mod lut195h;
#[doc = "LUT196H register accessor: an alias for `Reg<LUT196H_SPEC>`"]
pub type LUT196H = crate::Reg<lut196h::LUT196H_SPEC>;
#[doc = "Graphic MMU LUT entry 196 high"]
pub mod lut196h;
#[doc = "LUT197H register accessor: an alias for `Reg<LUT197H_SPEC>`"]
pub type LUT197H = crate::Reg<lut197h::LUT197H_SPEC>;
#[doc = "Graphic MMU LUT entry 197 high"]
pub mod lut197h;
#[doc = "LUT198H register accessor: an alias for `Reg<LUT198H_SPEC>`"]
pub type LUT198H = crate::Reg<lut198h::LUT198H_SPEC>;
#[doc = "Graphic MMU LUT entry 198 high"]
pub mod lut198h;
#[doc = "LUT199H register accessor: an alias for `Reg<LUT199H_SPEC>`"]
pub type LUT199H = crate::Reg<lut199h::LUT199H_SPEC>;
#[doc = "Graphic MMU LUT entry 199 high"]
pub mod lut199h;
#[doc = "LUT200H register accessor: an alias for `Reg<LUT200H_SPEC>`"]
pub type LUT200H = crate::Reg<lut200h::LUT200H_SPEC>;
#[doc = "Graphic MMU LUT entry 200 high"]
pub mod lut200h;
#[doc = "LUT201H register accessor: an alias for `Reg<LUT201H_SPEC>`"]
pub type LUT201H = crate::Reg<lut201h::LUT201H_SPEC>;
#[doc = "Graphic MMU LUT entry 201 high"]
pub mod lut201h;
#[doc = "LUT202H register accessor: an alias for `Reg<LUT202H_SPEC>`"]
pub type LUT202H = crate::Reg<lut202h::LUT202H_SPEC>;
#[doc = "Graphic MMU LUT entry 202 high"]
pub mod lut202h;
#[doc = "LUT203H register accessor: an alias for `Reg<LUT203H_SPEC>`"]
pub type LUT203H = crate::Reg<lut203h::LUT203H_SPEC>;
#[doc = "Graphic MMU LUT entry 203 high"]
pub mod lut203h;
#[doc = "LUT204H register accessor: an alias for `Reg<LUT204H_SPEC>`"]
pub type LUT204H = crate::Reg<lut204h::LUT204H_SPEC>;
#[doc = "Graphic MMU LUT entry 204 high"]
pub mod lut204h;
#[doc = "LUT205H register accessor: an alias for `Reg<LUT205H_SPEC>`"]
pub type LUT205H = crate::Reg<lut205h::LUT205H_SPEC>;
#[doc = "Graphic MMU LUT entry 205 high"]
pub mod lut205h;
#[doc = "LUT206H register accessor: an alias for `Reg<LUT206H_SPEC>`"]
pub type LUT206H = crate::Reg<lut206h::LUT206H_SPEC>;
#[doc = "Graphic MMU LUT entry 206 high"]
pub mod lut206h;
#[doc = "LUT207H register accessor: an alias for `Reg<LUT207H_SPEC>`"]
pub type LUT207H = crate::Reg<lut207h::LUT207H_SPEC>;
#[doc = "Graphic MMU LUT entry 207 high"]
pub mod lut207h;
#[doc = "LUT208H register accessor: an alias for `Reg<LUT208H_SPEC>`"]
pub type LUT208H = crate::Reg<lut208h::LUT208H_SPEC>;
#[doc = "Graphic MMU LUT entry 208 high"]
pub mod lut208h;
#[doc = "LUT209H register accessor: an alias for `Reg<LUT209H_SPEC>`"]
pub type LUT209H = crate::Reg<lut209h::LUT209H_SPEC>;
#[doc = "Graphic MMU LUT entry 209 high"]
pub mod lut209h;
#[doc = "LUT210H register accessor: an alias for `Reg<LUT210H_SPEC>`"]
pub type LUT210H = crate::Reg<lut210h::LUT210H_SPEC>;
#[doc = "Graphic MMU LUT entry 210 high"]
pub mod lut210h;
#[doc = "LUT211H register accessor: an alias for `Reg<LUT211H_SPEC>`"]
pub type LUT211H = crate::Reg<lut211h::LUT211H_SPEC>;
#[doc = "Graphic MMU LUT entry 211 high"]
pub mod lut211h;
#[doc = "LUT212H register accessor: an alias for `Reg<LUT212H_SPEC>`"]
pub type LUT212H = crate::Reg<lut212h::LUT212H_SPEC>;
#[doc = "Graphic MMU LUT entry 212 high"]
pub mod lut212h;
#[doc = "LUT213H register accessor: an alias for `Reg<LUT213H_SPEC>`"]
pub type LUT213H = crate::Reg<lut213h::LUT213H_SPEC>;
#[doc = "Graphic MMU LUT entry 213 high"]
pub mod lut213h;
#[doc = "LUT214H register accessor: an alias for `Reg<LUT214H_SPEC>`"]
pub type LUT214H = crate::Reg<lut214h::LUT214H_SPEC>;
#[doc = "Graphic MMU LUT entry 214 high"]
pub mod lut214h;
#[doc = "LUT215H register accessor: an alias for `Reg<LUT215H_SPEC>`"]
pub type LUT215H = crate::Reg<lut215h::LUT215H_SPEC>;
#[doc = "Graphic MMU LUT entry 215 high"]
pub mod lut215h;
#[doc = "LUT216H register accessor: an alias for `Reg<LUT216H_SPEC>`"]
pub type LUT216H = crate::Reg<lut216h::LUT216H_SPEC>;
#[doc = "Graphic MMU LUT entry 216 high"]
pub mod lut216h;
#[doc = "LUT217H register accessor: an alias for `Reg<LUT217H_SPEC>`"]
pub type LUT217H = crate::Reg<lut217h::LUT217H_SPEC>;
#[doc = "Graphic MMU LUT entry 217 high"]
pub mod lut217h;
#[doc = "LUT218H register accessor: an alias for `Reg<LUT218H_SPEC>`"]
pub type LUT218H = crate::Reg<lut218h::LUT218H_SPEC>;
#[doc = "Graphic MMU LUT entry 218 high"]
pub mod lut218h;
#[doc = "LUT219H register accessor: an alias for `Reg<LUT219H_SPEC>`"]
pub type LUT219H = crate::Reg<lut219h::LUT219H_SPEC>;
#[doc = "Graphic MMU LUT entry 219 high"]
pub mod lut219h;
#[doc = "LUT220H register accessor: an alias for `Reg<LUT220H_SPEC>`"]
pub type LUT220H = crate::Reg<lut220h::LUT220H_SPEC>;
#[doc = "Graphic MMU LUT entry 220 high"]
pub mod lut220h;
#[doc = "LUT221H register accessor: an alias for `Reg<LUT221H_SPEC>`"]
pub type LUT221H = crate::Reg<lut221h::LUT221H_SPEC>;
#[doc = "Graphic MMU LUT entry 221 high"]
pub mod lut221h;
#[doc = "LUT222H register accessor: an alias for `Reg<LUT222H_SPEC>`"]
pub type LUT222H = crate::Reg<lut222h::LUT222H_SPEC>;
#[doc = "Graphic MMU LUT entry 222 high"]
pub mod lut222h;
#[doc = "LUT223H register accessor: an alias for `Reg<LUT223H_SPEC>`"]
pub type LUT223H = crate::Reg<lut223h::LUT223H_SPEC>;
#[doc = "Graphic MMU LUT entry 223 high"]
pub mod lut223h;
#[doc = "LUT224H register accessor: an alias for `Reg<LUT224H_SPEC>`"]
pub type LUT224H = crate::Reg<lut224h::LUT224H_SPEC>;
#[doc = "Graphic MMU LUT entry 224 high"]
pub mod lut224h;
#[doc = "LUT225H register accessor: an alias for `Reg<LUT225H_SPEC>`"]
pub type LUT225H = crate::Reg<lut225h::LUT225H_SPEC>;
#[doc = "Graphic MMU LUT entry 225 high"]
pub mod lut225h;
#[doc = "LUT226H register accessor: an alias for `Reg<LUT226H_SPEC>`"]
pub type LUT226H = crate::Reg<lut226h::LUT226H_SPEC>;
#[doc = "Graphic MMU LUT entry 226 high"]
pub mod lut226h;
#[doc = "LUT227H register accessor: an alias for `Reg<LUT227H_SPEC>`"]
pub type LUT227H = crate::Reg<lut227h::LUT227H_SPEC>;
#[doc = "Graphic MMU LUT entry 227 high"]
pub mod lut227h;
#[doc = "LUT228H register accessor: an alias for `Reg<LUT228H_SPEC>`"]
pub type LUT228H = crate::Reg<lut228h::LUT228H_SPEC>;
#[doc = "Graphic MMU LUT entry 228 high"]
pub mod lut228h;
#[doc = "LUT229H register accessor: an alias for `Reg<LUT229H_SPEC>`"]
pub type LUT229H = crate::Reg<lut229h::LUT229H_SPEC>;
#[doc = "Graphic MMU LUT entry 229 high"]
pub mod lut229h;
#[doc = "LUT230H register accessor: an alias for `Reg<LUT230H_SPEC>`"]
pub type LUT230H = crate::Reg<lut230h::LUT230H_SPEC>;
#[doc = "Graphic MMU LUT entry 230 high"]
pub mod lut230h;
#[doc = "LUT231H register accessor: an alias for `Reg<LUT231H_SPEC>`"]
pub type LUT231H = crate::Reg<lut231h::LUT231H_SPEC>;
#[doc = "Graphic MMU LUT entry 231 high"]
pub mod lut231h;
#[doc = "LUT232H register accessor: an alias for `Reg<LUT232H_SPEC>`"]
pub type LUT232H = crate::Reg<lut232h::LUT232H_SPEC>;
#[doc = "Graphic MMU LUT entry 232 high"]
pub mod lut232h;
#[doc = "LUT233H register accessor: an alias for `Reg<LUT233H_SPEC>`"]
pub type LUT233H = crate::Reg<lut233h::LUT233H_SPEC>;
#[doc = "Graphic MMU LUT entry 233 high"]
pub mod lut233h;
#[doc = "LUT234H register accessor: an alias for `Reg<LUT234H_SPEC>`"]
pub type LUT234H = crate::Reg<lut234h::LUT234H_SPEC>;
#[doc = "Graphic MMU LUT entry 234 high"]
pub mod lut234h;
#[doc = "LUT235H register accessor: an alias for `Reg<LUT235H_SPEC>`"]
pub type LUT235H = crate::Reg<lut235h::LUT235H_SPEC>;
#[doc = "Graphic MMU LUT entry 235 high"]
pub mod lut235h;
#[doc = "LUT236H register accessor: an alias for `Reg<LUT236H_SPEC>`"]
pub type LUT236H = crate::Reg<lut236h::LUT236H_SPEC>;
#[doc = "Graphic MMU LUT entry 236 high"]
pub mod lut236h;
#[doc = "LUT237H register accessor: an alias for `Reg<LUT237H_SPEC>`"]
pub type LUT237H = crate::Reg<lut237h::LUT237H_SPEC>;
#[doc = "Graphic MMU LUT entry 237 high"]
pub mod lut237h;
#[doc = "LUT238H register accessor: an alias for `Reg<LUT238H_SPEC>`"]
pub type LUT238H = crate::Reg<lut238h::LUT238H_SPEC>;
#[doc = "Graphic MMU LUT entry 238 high"]
pub mod lut238h;
#[doc = "LUT239H register accessor: an alias for `Reg<LUT239H_SPEC>`"]
pub type LUT239H = crate::Reg<lut239h::LUT239H_SPEC>;
#[doc = "Graphic MMU LUT entry 239 high"]
pub mod lut239h;
#[doc = "LUT240H register accessor: an alias for `Reg<LUT240H_SPEC>`"]
pub type LUT240H = crate::Reg<lut240h::LUT240H_SPEC>;
#[doc = "Graphic MMU LUT entry 240 high"]
pub mod lut240h;
#[doc = "LUT241H register accessor: an alias for `Reg<LUT241H_SPEC>`"]
pub type LUT241H = crate::Reg<lut241h::LUT241H_SPEC>;
#[doc = "Graphic MMU LUT entry 241 high"]
pub mod lut241h;
#[doc = "LUT242H register accessor: an alias for `Reg<LUT242H_SPEC>`"]
pub type LUT242H = crate::Reg<lut242h::LUT242H_SPEC>;
#[doc = "Graphic MMU LUT entry 242 high"]
pub mod lut242h;
#[doc = "LUT243H register accessor: an alias for `Reg<LUT243H_SPEC>`"]
pub type LUT243H = crate::Reg<lut243h::LUT243H_SPEC>;
#[doc = "Graphic MMU LUT entry 243 high"]
pub mod lut243h;
#[doc = "LUT244H register accessor: an alias for `Reg<LUT244H_SPEC>`"]
pub type LUT244H = crate::Reg<lut244h::LUT244H_SPEC>;
#[doc = "Graphic MMU LUT entry 244 high"]
pub mod lut244h;
#[doc = "LUT245H register accessor: an alias for `Reg<LUT245H_SPEC>`"]
pub type LUT245H = crate::Reg<lut245h::LUT245H_SPEC>;
#[doc = "Graphic MMU LUT entry 245 high"]
pub mod lut245h;
#[doc = "LUT246H register accessor: an alias for `Reg<LUT246H_SPEC>`"]
pub type LUT246H = crate::Reg<lut246h::LUT246H_SPEC>;
#[doc = "Graphic MMU LUT entry 246 high"]
pub mod lut246h;
#[doc = "LUT247H register accessor: an alias for `Reg<LUT247H_SPEC>`"]
pub type LUT247H = crate::Reg<lut247h::LUT247H_SPEC>;
#[doc = "Graphic MMU LUT entry 247 high"]
pub mod lut247h;
#[doc = "LUT248H register accessor: an alias for `Reg<LUT248H_SPEC>`"]
pub type LUT248H = crate::Reg<lut248h::LUT248H_SPEC>;
#[doc = "Graphic MMU LUT entry 248 high"]
pub mod lut248h;
#[doc = "LUT249H register accessor: an alias for `Reg<LUT249H_SPEC>`"]
pub type LUT249H = crate::Reg<lut249h::LUT249H_SPEC>;
#[doc = "Graphic MMU LUT entry 249 high"]
pub mod lut249h;
#[doc = "LUT250H register accessor: an alias for `Reg<LUT250H_SPEC>`"]
pub type LUT250H = crate::Reg<lut250h::LUT250H_SPEC>;
#[doc = "Graphic MMU LUT entry 250 high"]
pub mod lut250h;
#[doc = "LUT251H register accessor: an alias for `Reg<LUT251H_SPEC>`"]
pub type LUT251H = crate::Reg<lut251h::LUT251H_SPEC>;
#[doc = "Graphic MMU LUT entry 251 high"]
pub mod lut251h;
#[doc = "LUT252H register accessor: an alias for `Reg<LUT252H_SPEC>`"]
pub type LUT252H = crate::Reg<lut252h::LUT252H_SPEC>;
#[doc = "Graphic MMU LUT entry 252 high"]
pub mod lut252h;
#[doc = "LUT253H register accessor: an alias for `Reg<LUT253H_SPEC>`"]
pub type LUT253H = crate::Reg<lut253h::LUT253H_SPEC>;
#[doc = "Graphic MMU LUT entry 253 high"]
pub mod lut253h;
#[doc = "LUT254H register accessor: an alias for `Reg<LUT254H_SPEC>`"]
pub type LUT254H = crate::Reg<lut254h::LUT254H_SPEC>;
#[doc = "Graphic MMU LUT entry 254 high"]
pub mod lut254h;
#[doc = "LUT255H register accessor: an alias for `Reg<LUT255H_SPEC>`"]
pub type LUT255H = crate::Reg<lut255h::LUT255H_SPEC>;
#[doc = "Graphic MMU LUT entry 255 high"]
pub mod lut255h;
#[doc = "LUT256H register accessor: an alias for `Reg<LUT256H_SPEC>`"]
pub type LUT256H = crate::Reg<lut256h::LUT256H_SPEC>;
#[doc = "Graphic MMU LUT entry 256 high"]
pub mod lut256h;
#[doc = "LUT257H register accessor: an alias for `Reg<LUT257H_SPEC>`"]
pub type LUT257H = crate::Reg<lut257h::LUT257H_SPEC>;
#[doc = "Graphic MMU LUT entry 257 high"]
pub mod lut257h;
#[doc = "LUT258H register accessor: an alias for `Reg<LUT258H_SPEC>`"]
pub type LUT258H = crate::Reg<lut258h::LUT258H_SPEC>;
#[doc = "Graphic MMU LUT entry 258 high"]
pub mod lut258h;
#[doc = "LUT259H register accessor: an alias for `Reg<LUT259H_SPEC>`"]
pub type LUT259H = crate::Reg<lut259h::LUT259H_SPEC>;
#[doc = "Graphic MMU LUT entry 259 high"]
pub mod lut259h;
#[doc = "LUT260H register accessor: an alias for `Reg<LUT260H_SPEC>`"]
pub type LUT260H = crate::Reg<lut260h::LUT260H_SPEC>;
#[doc = "Graphic MMU LUT entry 260 high"]
pub mod lut260h;
#[doc = "LUT261H register accessor: an alias for `Reg<LUT261H_SPEC>`"]
pub type LUT261H = crate::Reg<lut261h::LUT261H_SPEC>;
#[doc = "Graphic MMU LUT entry 261 high"]
pub mod lut261h;
#[doc = "LUT262H register accessor: an alias for `Reg<LUT262H_SPEC>`"]
pub type LUT262H = crate::Reg<lut262h::LUT262H_SPEC>;
#[doc = "Graphic MMU LUT entry 262 high"]
pub mod lut262h;
#[doc = "LUT263H register accessor: an alias for `Reg<LUT263H_SPEC>`"]
pub type LUT263H = crate::Reg<lut263h::LUT263H_SPEC>;
#[doc = "Graphic MMU LUT entry 263 high"]
pub mod lut263h;
#[doc = "LUT264H register accessor: an alias for `Reg<LUT264H_SPEC>`"]
pub type LUT264H = crate::Reg<lut264h::LUT264H_SPEC>;
#[doc = "Graphic MMU LUT entry 264 high"]
pub mod lut264h;
#[doc = "LUT265H register accessor: an alias for `Reg<LUT265H_SPEC>`"]
pub type LUT265H = crate::Reg<lut265h::LUT265H_SPEC>;
#[doc = "Graphic MMU LUT entry 265 high"]
pub mod lut265h;
#[doc = "LUT266H register accessor: an alias for `Reg<LUT266H_SPEC>`"]
pub type LUT266H = crate::Reg<lut266h::LUT266H_SPEC>;
#[doc = "Graphic MMU LUT entry 266 high"]
pub mod lut266h;
#[doc = "LUT267H register accessor: an alias for `Reg<LUT267H_SPEC>`"]
pub type LUT267H = crate::Reg<lut267h::LUT267H_SPEC>;
#[doc = "Graphic MMU LUT entry 267 high"]
pub mod lut267h;
#[doc = "LUT268H register accessor: an alias for `Reg<LUT268H_SPEC>`"]
pub type LUT268H = crate::Reg<lut268h::LUT268H_SPEC>;
#[doc = "Graphic MMU LUT entry 268 high"]
pub mod lut268h;
#[doc = "LUT269H register accessor: an alias for `Reg<LUT269H_SPEC>`"]
pub type LUT269H = crate::Reg<lut269h::LUT269H_SPEC>;
#[doc = "Graphic MMU LUT entry 269 high"]
pub mod lut269h;
#[doc = "LUT270H register accessor: an alias for `Reg<LUT270H_SPEC>`"]
pub type LUT270H = crate::Reg<lut270h::LUT270H_SPEC>;
#[doc = "Graphic MMU LUT entry 270 high"]
pub mod lut270h;
#[doc = "LUT271H register accessor: an alias for `Reg<LUT271H_SPEC>`"]
pub type LUT271H = crate::Reg<lut271h::LUT271H_SPEC>;
#[doc = "Graphic MMU LUT entry 271 high"]
pub mod lut271h;
#[doc = "LUT272H register accessor: an alias for `Reg<LUT272H_SPEC>`"]
pub type LUT272H = crate::Reg<lut272h::LUT272H_SPEC>;
#[doc = "Graphic MMU LUT entry 272 high"]
pub mod lut272h;
#[doc = "LUT273H register accessor: an alias for `Reg<LUT273H_SPEC>`"]
pub type LUT273H = crate::Reg<lut273h::LUT273H_SPEC>;
#[doc = "Graphic MMU LUT entry 273 high"]
pub mod lut273h;
#[doc = "LUT274H register accessor: an alias for `Reg<LUT274H_SPEC>`"]
pub type LUT274H = crate::Reg<lut274h::LUT274H_SPEC>;
#[doc = "Graphic MMU LUT entry 274 high"]
pub mod lut274h;
#[doc = "LUT275H register accessor: an alias for `Reg<LUT275H_SPEC>`"]
pub type LUT275H = crate::Reg<lut275h::LUT275H_SPEC>;
#[doc = "Graphic MMU LUT entry 275 high"]
pub mod lut275h;
#[doc = "LUT276H register accessor: an alias for `Reg<LUT276H_SPEC>`"]
pub type LUT276H = crate::Reg<lut276h::LUT276H_SPEC>;
#[doc = "Graphic MMU LUT entry 276 high"]
pub mod lut276h;
#[doc = "LUT277H register accessor: an alias for `Reg<LUT277H_SPEC>`"]
pub type LUT277H = crate::Reg<lut277h::LUT277H_SPEC>;
#[doc = "Graphic MMU LUT entry 277 high"]
pub mod lut277h;
#[doc = "LUT278H register accessor: an alias for `Reg<LUT278H_SPEC>`"]
pub type LUT278H = crate::Reg<lut278h::LUT278H_SPEC>;
#[doc = "Graphic MMU LUT entry 278 high"]
pub mod lut278h;
#[doc = "LUT279H register accessor: an alias for `Reg<LUT279H_SPEC>`"]
pub type LUT279H = crate::Reg<lut279h::LUT279H_SPEC>;
#[doc = "Graphic MMU LUT entry 279 high"]
pub mod lut279h;
#[doc = "LUT280H register accessor: an alias for `Reg<LUT280H_SPEC>`"]
pub type LUT280H = crate::Reg<lut280h::LUT280H_SPEC>;
#[doc = "Graphic MMU LUT entry 280 high"]
pub mod lut280h;
#[doc = "LUT281H register accessor: an alias for `Reg<LUT281H_SPEC>`"]
pub type LUT281H = crate::Reg<lut281h::LUT281H_SPEC>;
#[doc = "Graphic MMU LUT entry 281 high"]
pub mod lut281h;
#[doc = "LUT282H register accessor: an alias for `Reg<LUT282H_SPEC>`"]
pub type LUT282H = crate::Reg<lut282h::LUT282H_SPEC>;
#[doc = "Graphic MMU LUT entry 282 high"]
pub mod lut282h;
#[doc = "LUT283H register accessor: an alias for `Reg<LUT283H_SPEC>`"]
pub type LUT283H = crate::Reg<lut283h::LUT283H_SPEC>;
#[doc = "Graphic MMU LUT entry 283 high"]
pub mod lut283h;
#[doc = "LUT284H register accessor: an alias for `Reg<LUT284H_SPEC>`"]
pub type LUT284H = crate::Reg<lut284h::LUT284H_SPEC>;
#[doc = "Graphic MMU LUT entry 284 high"]
pub mod lut284h;
#[doc = "LUT285H register accessor: an alias for `Reg<LUT285H_SPEC>`"]
pub type LUT285H = crate::Reg<lut285h::LUT285H_SPEC>;
#[doc = "Graphic MMU LUT entry 285 high"]
pub mod lut285h;
#[doc = "LUT286H register accessor: an alias for `Reg<LUT286H_SPEC>`"]
pub type LUT286H = crate::Reg<lut286h::LUT286H_SPEC>;
#[doc = "Graphic MMU LUT entry 286 high"]
pub mod lut286h;
#[doc = "LUT287H register accessor: an alias for `Reg<LUT287H_SPEC>`"]
pub type LUT287H = crate::Reg<lut287h::LUT287H_SPEC>;
#[doc = "Graphic MMU LUT entry 287 high"]
pub mod lut287h;
#[doc = "LUT288H register accessor: an alias for `Reg<LUT288H_SPEC>`"]
pub type LUT288H = crate::Reg<lut288h::LUT288H_SPEC>;
#[doc = "Graphic MMU LUT entry 288 high"]
pub mod lut288h;
#[doc = "LUT289H register accessor: an alias for `Reg<LUT289H_SPEC>`"]
pub type LUT289H = crate::Reg<lut289h::LUT289H_SPEC>;
#[doc = "Graphic MMU LUT entry 289 high"]
pub mod lut289h;
#[doc = "LUT290H register accessor: an alias for `Reg<LUT290H_SPEC>`"]
pub type LUT290H = crate::Reg<lut290h::LUT290H_SPEC>;
#[doc = "Graphic MMU LUT entry 290 high"]
pub mod lut290h;
#[doc = "LUT291H register accessor: an alias for `Reg<LUT291H_SPEC>`"]
pub type LUT291H = crate::Reg<lut291h::LUT291H_SPEC>;
#[doc = "Graphic MMU LUT entry 291 high"]
pub mod lut291h;
#[doc = "LUT292H register accessor: an alias for `Reg<LUT292H_SPEC>`"]
pub type LUT292H = crate::Reg<lut292h::LUT292H_SPEC>;
#[doc = "Graphic MMU LUT entry 292 high"]
pub mod lut292h;
#[doc = "LUT293H register accessor: an alias for `Reg<LUT293H_SPEC>`"]
pub type LUT293H = crate::Reg<lut293h::LUT293H_SPEC>;
#[doc = "Graphic MMU LUT entry 293 high"]
pub mod lut293h;
#[doc = "LUT294H register accessor: an alias for `Reg<LUT294H_SPEC>`"]
pub type LUT294H = crate::Reg<lut294h::LUT294H_SPEC>;
#[doc = "Graphic MMU LUT entry 294 high"]
pub mod lut294h;
#[doc = "LUT295H register accessor: an alias for `Reg<LUT295H_SPEC>`"]
pub type LUT295H = crate::Reg<lut295h::LUT295H_SPEC>;
#[doc = "Graphic MMU LUT entry 295 high"]
pub mod lut295h;
#[doc = "LUT296H register accessor: an alias for `Reg<LUT296H_SPEC>`"]
pub type LUT296H = crate::Reg<lut296h::LUT296H_SPEC>;
#[doc = "Graphic MMU LUT entry 296 high"]
pub mod lut296h;
#[doc = "LUT297H register accessor: an alias for `Reg<LUT297H_SPEC>`"]
pub type LUT297H = crate::Reg<lut297h::LUT297H_SPEC>;
#[doc = "Graphic MMU LUT entry 297 high"]
pub mod lut297h;
#[doc = "LUT298H register accessor: an alias for `Reg<LUT298H_SPEC>`"]
pub type LUT298H = crate::Reg<lut298h::LUT298H_SPEC>;
#[doc = "Graphic MMU LUT entry 298 high"]
pub mod lut298h;
#[doc = "LUT299H register accessor: an alias for `Reg<LUT299H_SPEC>`"]
pub type LUT299H = crate::Reg<lut299h::LUT299H_SPEC>;
#[doc = "Graphic MMU LUT entry 299 high"]
pub mod lut299h;
#[doc = "LUT300H register accessor: an alias for `Reg<LUT300H_SPEC>`"]
pub type LUT300H = crate::Reg<lut300h::LUT300H_SPEC>;
#[doc = "Graphic MMU LUT entry 300 high"]
pub mod lut300h;
#[doc = "LUT301H register accessor: an alias for `Reg<LUT301H_SPEC>`"]
pub type LUT301H = crate::Reg<lut301h::LUT301H_SPEC>;
#[doc = "Graphic MMU LUT entry 301 high"]
pub mod lut301h;
#[doc = "LUT302H register accessor: an alias for `Reg<LUT302H_SPEC>`"]
pub type LUT302H = crate::Reg<lut302h::LUT302H_SPEC>;
#[doc = "Graphic MMU LUT entry 302 high"]
pub mod lut302h;
#[doc = "LUT303H register accessor: an alias for `Reg<LUT303H_SPEC>`"]
pub type LUT303H = crate::Reg<lut303h::LUT303H_SPEC>;
#[doc = "Graphic MMU LUT entry 303 high"]
pub mod lut303h;
#[doc = "LUT304H register accessor: an alias for `Reg<LUT304H_SPEC>`"]
pub type LUT304H = crate::Reg<lut304h::LUT304H_SPEC>;
#[doc = "Graphic MMU LUT entry 304 high"]
pub mod lut304h;
#[doc = "LUT305H register accessor: an alias for `Reg<LUT305H_SPEC>`"]
pub type LUT305H = crate::Reg<lut305h::LUT305H_SPEC>;
#[doc = "Graphic MMU LUT entry 305 high"]
pub mod lut305h;
#[doc = "LUT306H register accessor: an alias for `Reg<LUT306H_SPEC>`"]
pub type LUT306H = crate::Reg<lut306h::LUT306H_SPEC>;
#[doc = "Graphic MMU LUT entry 306 high"]
pub mod lut306h;
#[doc = "LUT307H register accessor: an alias for `Reg<LUT307H_SPEC>`"]
pub type LUT307H = crate::Reg<lut307h::LUT307H_SPEC>;
#[doc = "Graphic MMU LUT entry 307 high"]
pub mod lut307h;
#[doc = "LUT308H register accessor: an alias for `Reg<LUT308H_SPEC>`"]
pub type LUT308H = crate::Reg<lut308h::LUT308H_SPEC>;
#[doc = "Graphic MMU LUT entry 308 high"]
pub mod lut308h;
#[doc = "LUT309H register accessor: an alias for `Reg<LUT309H_SPEC>`"]
pub type LUT309H = crate::Reg<lut309h::LUT309H_SPEC>;
#[doc = "Graphic MMU LUT entry 309 high"]
pub mod lut309h;
#[doc = "LUT310H register accessor: an alias for `Reg<LUT310H_SPEC>`"]
pub type LUT310H = crate::Reg<lut310h::LUT310H_SPEC>;
#[doc = "Graphic MMU LUT entry 310 high"]
pub mod lut310h;
#[doc = "LUT311H register accessor: an alias for `Reg<LUT311H_SPEC>`"]
pub type LUT311H = crate::Reg<lut311h::LUT311H_SPEC>;
#[doc = "Graphic MMU LUT entry 311 high"]
pub mod lut311h;
#[doc = "LUT312H register accessor: an alias for `Reg<LUT312H_SPEC>`"]
pub type LUT312H = crate::Reg<lut312h::LUT312H_SPEC>;
#[doc = "Graphic MMU LUT entry 312 high"]
pub mod lut312h;
#[doc = "LUT313H register accessor: an alias for `Reg<LUT313H_SPEC>`"]
pub type LUT313H = crate::Reg<lut313h::LUT313H_SPEC>;
#[doc = "Graphic MMU LUT entry 313 high"]
pub mod lut313h;
#[doc = "LUT314H register accessor: an alias for `Reg<LUT314H_SPEC>`"]
pub type LUT314H = crate::Reg<lut314h::LUT314H_SPEC>;
#[doc = "Graphic MMU LUT entry 314 high"]
pub mod lut314h;
#[doc = "LUT315H register accessor: an alias for `Reg<LUT315H_SPEC>`"]
pub type LUT315H = crate::Reg<lut315h::LUT315H_SPEC>;
#[doc = "Graphic MMU LUT entry 315 high"]
pub mod lut315h;
#[doc = "LUT316H register accessor: an alias for `Reg<LUT316H_SPEC>`"]
pub type LUT316H = crate::Reg<lut316h::LUT316H_SPEC>;
#[doc = "Graphic MMU LUT entry 316 high"]
pub mod lut316h;
#[doc = "LUT317H register accessor: an alias for `Reg<LUT317H_SPEC>`"]
pub type LUT317H = crate::Reg<lut317h::LUT317H_SPEC>;
#[doc = "Graphic MMU LUT entry 317 high"]
pub mod lut317h;
#[doc = "LUT318H register accessor: an alias for `Reg<LUT318H_SPEC>`"]
pub type LUT318H = crate::Reg<lut318h::LUT318H_SPEC>;
#[doc = "Graphic MMU LUT entry 318 high"]
pub mod lut318h;
#[doc = "LUT319H register accessor: an alias for `Reg<LUT319H_SPEC>`"]
pub type LUT319H = crate::Reg<lut319h::LUT319H_SPEC>;
#[doc = "Graphic MMU LUT entry 319 high"]
pub mod lut319h;
#[doc = "LUT320H register accessor: an alias for `Reg<LUT320H_SPEC>`"]
pub type LUT320H = crate::Reg<lut320h::LUT320H_SPEC>;
#[doc = "Graphic MMU LUT entry 320 high"]
pub mod lut320h;
#[doc = "LUT321H register accessor: an alias for `Reg<LUT321H_SPEC>`"]
pub type LUT321H = crate::Reg<lut321h::LUT321H_SPEC>;
#[doc = "Graphic MMU LUT entry 321 high"]
pub mod lut321h;
#[doc = "LUT322H register accessor: an alias for `Reg<LUT322H_SPEC>`"]
pub type LUT322H = crate::Reg<lut322h::LUT322H_SPEC>;
#[doc = "Graphic MMU LUT entry 322 high"]
pub mod lut322h;
#[doc = "LUT323H register accessor: an alias for `Reg<LUT323H_SPEC>`"]
pub type LUT323H = crate::Reg<lut323h::LUT323H_SPEC>;
#[doc = "Graphic MMU LUT entry 323 high"]
pub mod lut323h;
#[doc = "LUT324H register accessor: an alias for `Reg<LUT324H_SPEC>`"]
pub type LUT324H = crate::Reg<lut324h::LUT324H_SPEC>;
#[doc = "Graphic MMU LUT entry 324 high"]
pub mod lut324h;
#[doc = "LUT325H register accessor: an alias for `Reg<LUT325H_SPEC>`"]
pub type LUT325H = crate::Reg<lut325h::LUT325H_SPEC>;
#[doc = "Graphic MMU LUT entry 325 high"]
pub mod lut325h;
#[doc = "LUT326H register accessor: an alias for `Reg<LUT326H_SPEC>`"]
pub type LUT326H = crate::Reg<lut326h::LUT326H_SPEC>;
#[doc = "Graphic MMU LUT entry 326 high"]
pub mod lut326h;
#[doc = "LUT327H register accessor: an alias for `Reg<LUT327H_SPEC>`"]
pub type LUT327H = crate::Reg<lut327h::LUT327H_SPEC>;
#[doc = "Graphic MMU LUT entry 327 high"]
pub mod lut327h;
#[doc = "LUT328H register accessor: an alias for `Reg<LUT328H_SPEC>`"]
pub type LUT328H = crate::Reg<lut328h::LUT328H_SPEC>;
#[doc = "Graphic MMU LUT entry 328 high"]
pub mod lut328h;
#[doc = "LUT329H register accessor: an alias for `Reg<LUT329H_SPEC>`"]
pub type LUT329H = crate::Reg<lut329h::LUT329H_SPEC>;
#[doc = "Graphic MMU LUT entry 329 high"]
pub mod lut329h;
#[doc = "LUT330H register accessor: an alias for `Reg<LUT330H_SPEC>`"]
pub type LUT330H = crate::Reg<lut330h::LUT330H_SPEC>;
#[doc = "Graphic MMU LUT entry 330 high"]
pub mod lut330h;
#[doc = "LUT331H register accessor: an alias for `Reg<LUT331H_SPEC>`"]
pub type LUT331H = crate::Reg<lut331h::LUT331H_SPEC>;
#[doc = "Graphic MMU LUT entry 331 high"]
pub mod lut331h;
#[doc = "LUT332H register accessor: an alias for `Reg<LUT332H_SPEC>`"]
pub type LUT332H = crate::Reg<lut332h::LUT332H_SPEC>;
#[doc = "Graphic MMU LUT entry 332 high"]
pub mod lut332h;
#[doc = "LUT333H register accessor: an alias for `Reg<LUT333H_SPEC>`"]
pub type LUT333H = crate::Reg<lut333h::LUT333H_SPEC>;
#[doc = "Graphic MMU LUT entry 333 high"]
pub mod lut333h;
#[doc = "LUT334H register accessor: an alias for `Reg<LUT334H_SPEC>`"]
pub type LUT334H = crate::Reg<lut334h::LUT334H_SPEC>;
#[doc = "Graphic MMU LUT entry 334 high"]
pub mod lut334h;
#[doc = "LUT335H register accessor: an alias for `Reg<LUT335H_SPEC>`"]
pub type LUT335H = crate::Reg<lut335h::LUT335H_SPEC>;
#[doc = "Graphic MMU LUT entry 335 high"]
pub mod lut335h;
#[doc = "LUT336H register accessor: an alias for `Reg<LUT336H_SPEC>`"]
pub type LUT336H = crate::Reg<lut336h::LUT336H_SPEC>;
#[doc = "Graphic MMU LUT entry 336 high"]
pub mod lut336h;
#[doc = "LUT337H register accessor: an alias for `Reg<LUT337H_SPEC>`"]
pub type LUT337H = crate::Reg<lut337h::LUT337H_SPEC>;
#[doc = "Graphic MMU LUT entry 337 high"]
pub mod lut337h;
#[doc = "LUT338H register accessor: an alias for `Reg<LUT338H_SPEC>`"]
pub type LUT338H = crate::Reg<lut338h::LUT338H_SPEC>;
#[doc = "Graphic MMU LUT entry 338 high"]
pub mod lut338h;
#[doc = "LUT339H register accessor: an alias for `Reg<LUT339H_SPEC>`"]
pub type LUT339H = crate::Reg<lut339h::LUT339H_SPEC>;
#[doc = "Graphic MMU LUT entry 339 high"]
pub mod lut339h;
#[doc = "LUT340H register accessor: an alias for `Reg<LUT340H_SPEC>`"]
pub type LUT340H = crate::Reg<lut340h::LUT340H_SPEC>;
#[doc = "Graphic MMU LUT entry 340 high"]
pub mod lut340h;
#[doc = "LUT341H register accessor: an alias for `Reg<LUT341H_SPEC>`"]
pub type LUT341H = crate::Reg<lut341h::LUT341H_SPEC>;
#[doc = "Graphic MMU LUT entry 341 high"]
pub mod lut341h;
#[doc = "LUT342H register accessor: an alias for `Reg<LUT342H_SPEC>`"]
pub type LUT342H = crate::Reg<lut342h::LUT342H_SPEC>;
#[doc = "Graphic MMU LUT entry 342 high"]
pub mod lut342h;
#[doc = "LUT343H register accessor: an alias for `Reg<LUT343H_SPEC>`"]
pub type LUT343H = crate::Reg<lut343h::LUT343H_SPEC>;
#[doc = "Graphic MMU LUT entry 343 high"]
pub mod lut343h;
#[doc = "LUT344H register accessor: an alias for `Reg<LUT344H_SPEC>`"]
pub type LUT344H = crate::Reg<lut344h::LUT344H_SPEC>;
#[doc = "Graphic MMU LUT entry 344 high"]
pub mod lut344h;
#[doc = "LUT345H register accessor: an alias for `Reg<LUT345H_SPEC>`"]
pub type LUT345H = crate::Reg<lut345h::LUT345H_SPEC>;
#[doc = "Graphic MMU LUT entry 345 high"]
pub mod lut345h;
#[doc = "LUT346H register accessor: an alias for `Reg<LUT346H_SPEC>`"]
pub type LUT346H = crate::Reg<lut346h::LUT346H_SPEC>;
#[doc = "Graphic MMU LUT entry 346 high"]
pub mod lut346h;
#[doc = "LUT347H register accessor: an alias for `Reg<LUT347H_SPEC>`"]
pub type LUT347H = crate::Reg<lut347h::LUT347H_SPEC>;
#[doc = "Graphic MMU LUT entry 347 high"]
pub mod lut347h;
#[doc = "LUT348H register accessor: an alias for `Reg<LUT348H_SPEC>`"]
pub type LUT348H = crate::Reg<lut348h::LUT348H_SPEC>;
#[doc = "Graphic MMU LUT entry 348 high"]
pub mod lut348h;
#[doc = "LUT349H register accessor: an alias for `Reg<LUT349H_SPEC>`"]
pub type LUT349H = crate::Reg<lut349h::LUT349H_SPEC>;
#[doc = "Graphic MMU LUT entry 349 high"]
pub mod lut349h;
#[doc = "LUT350H register accessor: an alias for `Reg<LUT350H_SPEC>`"]
pub type LUT350H = crate::Reg<lut350h::LUT350H_SPEC>;
#[doc = "Graphic MMU LUT entry 350 high"]
pub mod lut350h;
#[doc = "LUT351H register accessor: an alias for `Reg<LUT351H_SPEC>`"]
pub type LUT351H = crate::Reg<lut351h::LUT351H_SPEC>;
#[doc = "Graphic MMU LUT entry 351 high"]
pub mod lut351h;
#[doc = "LUT352H register accessor: an alias for `Reg<LUT352H_SPEC>`"]
pub type LUT352H = crate::Reg<lut352h::LUT352H_SPEC>;
#[doc = "Graphic MMU LUT entry 352 high"]
pub mod lut352h;
#[doc = "LUT353H register accessor: an alias for `Reg<LUT353H_SPEC>`"]
pub type LUT353H = crate::Reg<lut353h::LUT353H_SPEC>;
#[doc = "Graphic MMU LUT entry 353 high"]
pub mod lut353h;
#[doc = "LUT354H register accessor: an alias for `Reg<LUT354H_SPEC>`"]
pub type LUT354H = crate::Reg<lut354h::LUT354H_SPEC>;
#[doc = "Graphic MMU LUT entry 354 high"]
pub mod lut354h;
#[doc = "LUT355H register accessor: an alias for `Reg<LUT355H_SPEC>`"]
pub type LUT355H = crate::Reg<lut355h::LUT355H_SPEC>;
#[doc = "Graphic MMU LUT entry 355 high"]
pub mod lut355h;
#[doc = "LUT356H register accessor: an alias for `Reg<LUT356H_SPEC>`"]
pub type LUT356H = crate::Reg<lut356h::LUT356H_SPEC>;
#[doc = "Graphic MMU LUT entry 356 high"]
pub mod lut356h;
#[doc = "LUT357H register accessor: an alias for `Reg<LUT357H_SPEC>`"]
pub type LUT357H = crate::Reg<lut357h::LUT357H_SPEC>;
#[doc = "Graphic MMU LUT entry 357 high"]
pub mod lut357h;
#[doc = "LUT358H register accessor: an alias for `Reg<LUT358H_SPEC>`"]
pub type LUT358H = crate::Reg<lut358h::LUT358H_SPEC>;
#[doc = "Graphic MMU LUT entry 358 high"]
pub mod lut358h;
#[doc = "LUT359H register accessor: an alias for `Reg<LUT359H_SPEC>`"]
pub type LUT359H = crate::Reg<lut359h::LUT359H_SPEC>;
#[doc = "Graphic MMU LUT entry 359 high"]
pub mod lut359h;
#[doc = "LUT360H register accessor: an alias for `Reg<LUT360H_SPEC>`"]
pub type LUT360H = crate::Reg<lut360h::LUT360H_SPEC>;
#[doc = "Graphic MMU LUT entry 360 high"]
pub mod lut360h;
#[doc = "LUT361H register accessor: an alias for `Reg<LUT361H_SPEC>`"]
pub type LUT361H = crate::Reg<lut361h::LUT361H_SPEC>;
#[doc = "Graphic MMU LUT entry 361 high"]
pub mod lut361h;
#[doc = "LUT362H register accessor: an alias for `Reg<LUT362H_SPEC>`"]
pub type LUT362H = crate::Reg<lut362h::LUT362H_SPEC>;
#[doc = "Graphic MMU LUT entry 362 high"]
pub mod lut362h;
#[doc = "LUT363H register accessor: an alias for `Reg<LUT363H_SPEC>`"]
pub type LUT363H = crate::Reg<lut363h::LUT363H_SPEC>;
#[doc = "Graphic MMU LUT entry 363 high"]
pub mod lut363h;
#[doc = "LUT364H register accessor: an alias for `Reg<LUT364H_SPEC>`"]
pub type LUT364H = crate::Reg<lut364h::LUT364H_SPEC>;
#[doc = "Graphic MMU LUT entry 364 high"]
pub mod lut364h;
#[doc = "LUT365H register accessor: an alias for `Reg<LUT365H_SPEC>`"]
pub type LUT365H = crate::Reg<lut365h::LUT365H_SPEC>;
#[doc = "Graphic MMU LUT entry 365 high"]
pub mod lut365h;
#[doc = "LUT366H register accessor: an alias for `Reg<LUT366H_SPEC>`"]
pub type LUT366H = crate::Reg<lut366h::LUT366H_SPEC>;
#[doc = "Graphic MMU LUT entry 366 high"]
pub mod lut366h;
#[doc = "LUT367H register accessor: an alias for `Reg<LUT367H_SPEC>`"]
pub type LUT367H = crate::Reg<lut367h::LUT367H_SPEC>;
#[doc = "Graphic MMU LUT entry 367 high"]
pub mod lut367h;
#[doc = "LUT368H register accessor: an alias for `Reg<LUT368H_SPEC>`"]
pub type LUT368H = crate::Reg<lut368h::LUT368H_SPEC>;
#[doc = "Graphic MMU LUT entry 368 high"]
pub mod lut368h;
#[doc = "LUT369H register accessor: an alias for `Reg<LUT369H_SPEC>`"]
pub type LUT369H = crate::Reg<lut369h::LUT369H_SPEC>;
#[doc = "Graphic MMU LUT entry 369 high"]
pub mod lut369h;
#[doc = "LUT370H register accessor: an alias for `Reg<LUT370H_SPEC>`"]
pub type LUT370H = crate::Reg<lut370h::LUT370H_SPEC>;
#[doc = "Graphic MMU LUT entry 370 high"]
pub mod lut370h;
#[doc = "LUT371H register accessor: an alias for `Reg<LUT371H_SPEC>`"]
pub type LUT371H = crate::Reg<lut371h::LUT371H_SPEC>;
#[doc = "Graphic MMU LUT entry 371 high"]
pub mod lut371h;
#[doc = "LUT372H register accessor: an alias for `Reg<LUT372H_SPEC>`"]
pub type LUT372H = crate::Reg<lut372h::LUT372H_SPEC>;
#[doc = "Graphic MMU LUT entry 372 high"]
pub mod lut372h;
#[doc = "LUT373H register accessor: an alias for `Reg<LUT373H_SPEC>`"]
pub type LUT373H = crate::Reg<lut373h::LUT373H_SPEC>;
#[doc = "Graphic MMU LUT entry 373 high"]
pub mod lut373h;
#[doc = "LUT374H register accessor: an alias for `Reg<LUT374H_SPEC>`"]
pub type LUT374H = crate::Reg<lut374h::LUT374H_SPEC>;
#[doc = "Graphic MMU LUT entry 374 high"]
pub mod lut374h;
#[doc = "LUT375H register accessor: an alias for `Reg<LUT375H_SPEC>`"]
pub type LUT375H = crate::Reg<lut375h::LUT375H_SPEC>;
#[doc = "Graphic MMU LUT entry 375 high"]
pub mod lut375h;
#[doc = "LUT376H register accessor: an alias for `Reg<LUT376H_SPEC>`"]
pub type LUT376H = crate::Reg<lut376h::LUT376H_SPEC>;
#[doc = "Graphic MMU LUT entry 376 high"]
pub mod lut376h;
#[doc = "LUT377H register accessor: an alias for `Reg<LUT377H_SPEC>`"]
pub type LUT377H = crate::Reg<lut377h::LUT377H_SPEC>;
#[doc = "Graphic MMU LUT entry 377 high"]
pub mod lut377h;
#[doc = "LUT378H register accessor: an alias for `Reg<LUT378H_SPEC>`"]
pub type LUT378H = crate::Reg<lut378h::LUT378H_SPEC>;
#[doc = "Graphic MMU LUT entry 378 high"]
pub mod lut378h;
#[doc = "LUT379H register accessor: an alias for `Reg<LUT379H_SPEC>`"]
pub type LUT379H = crate::Reg<lut379h::LUT379H_SPEC>;
#[doc = "Graphic MMU LUT entry 379 high"]
pub mod lut379h;
#[doc = "LUT380H register accessor: an alias for `Reg<LUT380H_SPEC>`"]
pub type LUT380H = crate::Reg<lut380h::LUT380H_SPEC>;
#[doc = "Graphic MMU LUT entry 380 high"]
pub mod lut380h;
#[doc = "LUT381H register accessor: an alias for `Reg<LUT381H_SPEC>`"]
pub type LUT381H = crate::Reg<lut381h::LUT381H_SPEC>;
#[doc = "Graphic MMU LUT entry 381 high"]
pub mod lut381h;
#[doc = "LUT382H register accessor: an alias for `Reg<LUT382H_SPEC>`"]
pub type LUT382H = crate::Reg<lut382h::LUT382H_SPEC>;
#[doc = "Graphic MMU LUT entry 382 high"]
pub mod lut382h;
#[doc = "LUT383H register accessor: an alias for `Reg<LUT383H_SPEC>`"]
pub type LUT383H = crate::Reg<lut383h::LUT383H_SPEC>;
#[doc = "Graphic MMU LUT entry 383 high"]
pub mod lut383h;
#[doc = "LUT384H register accessor: an alias for `Reg<LUT384H_SPEC>`"]
pub type LUT384H = crate::Reg<lut384h::LUT384H_SPEC>;
#[doc = "Graphic MMU LUT entry 384 high"]
pub mod lut384h;
#[doc = "LUT385H register accessor: an alias for `Reg<LUT385H_SPEC>`"]
pub type LUT385H = crate::Reg<lut385h::LUT385H_SPEC>;
#[doc = "Graphic MMU LUT entry 385 high"]
pub mod lut385h;
#[doc = "LUT386H register accessor: an alias for `Reg<LUT386H_SPEC>`"]
pub type LUT386H = crate::Reg<lut386h::LUT386H_SPEC>;
#[doc = "Graphic MMU LUT entry 386 high"]
pub mod lut386h;
#[doc = "LUT387H register accessor: an alias for `Reg<LUT387H_SPEC>`"]
pub type LUT387H = crate::Reg<lut387h::LUT387H_SPEC>;
#[doc = "Graphic MMU LUT entry 387 high"]
pub mod lut387h;
#[doc = "LUT388H register accessor: an alias for `Reg<LUT388H_SPEC>`"]
pub type LUT388H = crate::Reg<lut388h::LUT388H_SPEC>;
#[doc = "Graphic MMU LUT entry 388 high"]
pub mod lut388h;
#[doc = "LUT389H register accessor: an alias for `Reg<LUT389H_SPEC>`"]
pub type LUT389H = crate::Reg<lut389h::LUT389H_SPEC>;
#[doc = "Graphic MMU LUT entry 389 high"]
pub mod lut389h;
#[doc = "LUT390H register accessor: an alias for `Reg<LUT390H_SPEC>`"]
pub type LUT390H = crate::Reg<lut390h::LUT390H_SPEC>;
#[doc = "Graphic MMU LUT entry 390 high"]
pub mod lut390h;
#[doc = "LUT391H register accessor: an alias for `Reg<LUT391H_SPEC>`"]
pub type LUT391H = crate::Reg<lut391h::LUT391H_SPEC>;
#[doc = "Graphic MMU LUT entry 391 high"]
pub mod lut391h;
#[doc = "LUT392H register accessor: an alias for `Reg<LUT392H_SPEC>`"]
pub type LUT392H = crate::Reg<lut392h::LUT392H_SPEC>;
#[doc = "Graphic MMU LUT entry 392 high"]
pub mod lut392h;
#[doc = "LUT393H register accessor: an alias for `Reg<LUT393H_SPEC>`"]
pub type LUT393H = crate::Reg<lut393h::LUT393H_SPEC>;
#[doc = "Graphic MMU LUT entry 393 high"]
pub mod lut393h;
#[doc = "LUT394H register accessor: an alias for `Reg<LUT394H_SPEC>`"]
pub type LUT394H = crate::Reg<lut394h::LUT394H_SPEC>;
#[doc = "Graphic MMU LUT entry 394 high"]
pub mod lut394h;
#[doc = "LUT395H register accessor: an alias for `Reg<LUT395H_SPEC>`"]
pub type LUT395H = crate::Reg<lut395h::LUT395H_SPEC>;
#[doc = "Graphic MMU LUT entry 395 high"]
pub mod lut395h;
#[doc = "LUT396H register accessor: an alias for `Reg<LUT396H_SPEC>`"]
pub type LUT396H = crate::Reg<lut396h::LUT396H_SPEC>;
#[doc = "Graphic MMU LUT entry 396 high"]
pub mod lut396h;
#[doc = "LUT397H register accessor: an alias for `Reg<LUT397H_SPEC>`"]
pub type LUT397H = crate::Reg<lut397h::LUT397H_SPEC>;
#[doc = "Graphic MMU LUT entry 397 high"]
pub mod lut397h;
#[doc = "LUT398H register accessor: an alias for `Reg<LUT398H_SPEC>`"]
pub type LUT398H = crate::Reg<lut398h::LUT398H_SPEC>;
#[doc = "Graphic MMU LUT entry 398 high"]
pub mod lut398h;
#[doc = "LUT399H register accessor: an alias for `Reg<LUT399H_SPEC>`"]
pub type LUT399H = crate::Reg<lut399h::LUT399H_SPEC>;
#[doc = "Graphic MMU LUT entry 399 high"]
pub mod lut399h;
#[doc = "LUT400H register accessor: an alias for `Reg<LUT400H_SPEC>`"]
pub type LUT400H = crate::Reg<lut400h::LUT400H_SPEC>;
#[doc = "Graphic MMU LUT entry 400 high"]
pub mod lut400h;
#[doc = "LUT401H register accessor: an alias for `Reg<LUT401H_SPEC>`"]
pub type LUT401H = crate::Reg<lut401h::LUT401H_SPEC>;
#[doc = "Graphic MMU LUT entry 401 high"]
pub mod lut401h;
#[doc = "LUT402H register accessor: an alias for `Reg<LUT402H_SPEC>`"]
pub type LUT402H = crate::Reg<lut402h::LUT402H_SPEC>;
#[doc = "Graphic MMU LUT entry 402 high"]
pub mod lut402h;
#[doc = "LUT403H register accessor: an alias for `Reg<LUT403H_SPEC>`"]
pub type LUT403H = crate::Reg<lut403h::LUT403H_SPEC>;
#[doc = "Graphic MMU LUT entry 403 high"]
pub mod lut403h;
#[doc = "LUT404H register accessor: an alias for `Reg<LUT404H_SPEC>`"]
pub type LUT404H = crate::Reg<lut404h::LUT404H_SPEC>;
#[doc = "Graphic MMU LUT entry 404 high"]
pub mod lut404h;
#[doc = "LUT405H register accessor: an alias for `Reg<LUT405H_SPEC>`"]
pub type LUT405H = crate::Reg<lut405h::LUT405H_SPEC>;
#[doc = "Graphic MMU LUT entry 405 high"]
pub mod lut405h;
#[doc = "LUT406H register accessor: an alias for `Reg<LUT406H_SPEC>`"]
pub type LUT406H = crate::Reg<lut406h::LUT406H_SPEC>;
#[doc = "Graphic MMU LUT entry 406 high"]
pub mod lut406h;
#[doc = "LUT407H register accessor: an alias for `Reg<LUT407H_SPEC>`"]
pub type LUT407H = crate::Reg<lut407h::LUT407H_SPEC>;
#[doc = "Graphic MMU LUT entry 407 high"]
pub mod lut407h;
#[doc = "LUT408H register accessor: an alias for `Reg<LUT408H_SPEC>`"]
pub type LUT408H = crate::Reg<lut408h::LUT408H_SPEC>;
#[doc = "Graphic MMU LUT entry 408 high"]
pub mod lut408h;
#[doc = "LUT409H register accessor: an alias for `Reg<LUT409H_SPEC>`"]
pub type LUT409H = crate::Reg<lut409h::LUT409H_SPEC>;
#[doc = "Graphic MMU LUT entry 409 high"]
pub mod lut409h;
#[doc = "LUT410H register accessor: an alias for `Reg<LUT410H_SPEC>`"]
pub type LUT410H = crate::Reg<lut410h::LUT410H_SPEC>;
#[doc = "Graphic MMU LUT entry 410 high"]
pub mod lut410h;
#[doc = "LUT411H register accessor: an alias for `Reg<LUT411H_SPEC>`"]
pub type LUT411H = crate::Reg<lut411h::LUT411H_SPEC>;
#[doc = "Graphic MMU LUT entry 411 high"]
pub mod lut411h;
#[doc = "LUT412H register accessor: an alias for `Reg<LUT412H_SPEC>`"]
pub type LUT412H = crate::Reg<lut412h::LUT412H_SPEC>;
#[doc = "Graphic MMU LUT entry 412 high"]
pub mod lut412h;
#[doc = "LUT413H register accessor: an alias for `Reg<LUT413H_SPEC>`"]
pub type LUT413H = crate::Reg<lut413h::LUT413H_SPEC>;
#[doc = "Graphic MMU LUT entry 413 high"]
pub mod lut413h;
#[doc = "LUT414H register accessor: an alias for `Reg<LUT414H_SPEC>`"]
pub type LUT414H = crate::Reg<lut414h::LUT414H_SPEC>;
#[doc = "Graphic MMU LUT entry 414 high"]
pub mod lut414h;
#[doc = "LUT415H register accessor: an alias for `Reg<LUT415H_SPEC>`"]
pub type LUT415H = crate::Reg<lut415h::LUT415H_SPEC>;
#[doc = "Graphic MMU LUT entry 415 high"]
pub mod lut415h;
#[doc = "LUT416H register accessor: an alias for `Reg<LUT416H_SPEC>`"]
pub type LUT416H = crate::Reg<lut416h::LUT416H_SPEC>;
#[doc = "Graphic MMU LUT entry 416 high"]
pub mod lut416h;
#[doc = "LUT417H register accessor: an alias for `Reg<LUT417H_SPEC>`"]
pub type LUT417H = crate::Reg<lut417h::LUT417H_SPEC>;
#[doc = "Graphic MMU LUT entry 417 high"]
pub mod lut417h;
#[doc = "LUT418H register accessor: an alias for `Reg<LUT418H_SPEC>`"]
pub type LUT418H = crate::Reg<lut418h::LUT418H_SPEC>;
#[doc = "Graphic MMU LUT entry 418 high"]
pub mod lut418h;
#[doc = "LUT419H register accessor: an alias for `Reg<LUT419H_SPEC>`"]
pub type LUT419H = crate::Reg<lut419h::LUT419H_SPEC>;
#[doc = "Graphic MMU LUT entry 419 high"]
pub mod lut419h;
#[doc = "LUT420H register accessor: an alias for `Reg<LUT420H_SPEC>`"]
pub type LUT420H = crate::Reg<lut420h::LUT420H_SPEC>;
#[doc = "Graphic MMU LUT entry 420 high"]
pub mod lut420h;
#[doc = "LUT421H register accessor: an alias for `Reg<LUT421H_SPEC>`"]
pub type LUT421H = crate::Reg<lut421h::LUT421H_SPEC>;
#[doc = "Graphic MMU LUT entry 421 high"]
pub mod lut421h;
#[doc = "LUT422H register accessor: an alias for `Reg<LUT422H_SPEC>`"]
pub type LUT422H = crate::Reg<lut422h::LUT422H_SPEC>;
#[doc = "Graphic MMU LUT entry 422 high"]
pub mod lut422h;
#[doc = "LUT423H register accessor: an alias for `Reg<LUT423H_SPEC>`"]
pub type LUT423H = crate::Reg<lut423h::LUT423H_SPEC>;
#[doc = "Graphic MMU LUT entry 423 high"]
pub mod lut423h;
#[doc = "LUT424H register accessor: an alias for `Reg<LUT424H_SPEC>`"]
pub type LUT424H = crate::Reg<lut424h::LUT424H_SPEC>;
#[doc = "Graphic MMU LUT entry 424 high"]
pub mod lut424h;
#[doc = "LUT425H register accessor: an alias for `Reg<LUT425H_SPEC>`"]
pub type LUT425H = crate::Reg<lut425h::LUT425H_SPEC>;
#[doc = "Graphic MMU LUT entry 425 high"]
pub mod lut425h;
#[doc = "LUT426H register accessor: an alias for `Reg<LUT426H_SPEC>`"]
pub type LUT426H = crate::Reg<lut426h::LUT426H_SPEC>;
#[doc = "Graphic MMU LUT entry 426 high"]
pub mod lut426h;
#[doc = "LUT427H register accessor: an alias for `Reg<LUT427H_SPEC>`"]
pub type LUT427H = crate::Reg<lut427h::LUT427H_SPEC>;
#[doc = "Graphic MMU LUT entry 427 high"]
pub mod lut427h;
#[doc = "LUT428H register accessor: an alias for `Reg<LUT428H_SPEC>`"]
pub type LUT428H = crate::Reg<lut428h::LUT428H_SPEC>;
#[doc = "Graphic MMU LUT entry 428 high"]
pub mod lut428h;
#[doc = "LUT429H register accessor: an alias for `Reg<LUT429H_SPEC>`"]
pub type LUT429H = crate::Reg<lut429h::LUT429H_SPEC>;
#[doc = "Graphic MMU LUT entry 429 high"]
pub mod lut429h;
#[doc = "LUT430H register accessor: an alias for `Reg<LUT430H_SPEC>`"]
pub type LUT430H = crate::Reg<lut430h::LUT430H_SPEC>;
#[doc = "Graphic MMU LUT entry 430 high"]
pub mod lut430h;
#[doc = "LUT431H register accessor: an alias for `Reg<LUT431H_SPEC>`"]
pub type LUT431H = crate::Reg<lut431h::LUT431H_SPEC>;
#[doc = "Graphic MMU LUT entry 431 high"]
pub mod lut431h;
#[doc = "LUT432H register accessor: an alias for `Reg<LUT432H_SPEC>`"]
pub type LUT432H = crate::Reg<lut432h::LUT432H_SPEC>;
#[doc = "Graphic MMU LUT entry 432 high"]
pub mod lut432h;
#[doc = "LUT433H register accessor: an alias for `Reg<LUT433H_SPEC>`"]
pub type LUT433H = crate::Reg<lut433h::LUT433H_SPEC>;
#[doc = "Graphic MMU LUT entry 433 high"]
pub mod lut433h;
#[doc = "LUT434H register accessor: an alias for `Reg<LUT434H_SPEC>`"]
pub type LUT434H = crate::Reg<lut434h::LUT434H_SPEC>;
#[doc = "Graphic MMU LUT entry 434 high"]
pub mod lut434h;
#[doc = "LUT435H register accessor: an alias for `Reg<LUT435H_SPEC>`"]
pub type LUT435H = crate::Reg<lut435h::LUT435H_SPEC>;
#[doc = "Graphic MMU LUT entry 435 high"]
pub mod lut435h;
#[doc = "LUT436H register accessor: an alias for `Reg<LUT436H_SPEC>`"]
pub type LUT436H = crate::Reg<lut436h::LUT436H_SPEC>;
#[doc = "Graphic MMU LUT entry 436 high"]
pub mod lut436h;
#[doc = "LUT437H register accessor: an alias for `Reg<LUT437H_SPEC>`"]
pub type LUT437H = crate::Reg<lut437h::LUT437H_SPEC>;
#[doc = "Graphic MMU LUT entry 437 high"]
pub mod lut437h;
#[doc = "LUT438H register accessor: an alias for `Reg<LUT438H_SPEC>`"]
pub type LUT438H = crate::Reg<lut438h::LUT438H_SPEC>;
#[doc = "Graphic MMU LUT entry 438 high"]
pub mod lut438h;
#[doc = "LUT439H register accessor: an alias for `Reg<LUT439H_SPEC>`"]
pub type LUT439H = crate::Reg<lut439h::LUT439H_SPEC>;
#[doc = "Graphic MMU LUT entry 439 high"]
pub mod lut439h;
#[doc = "LUT440H register accessor: an alias for `Reg<LUT440H_SPEC>`"]
pub type LUT440H = crate::Reg<lut440h::LUT440H_SPEC>;
#[doc = "Graphic MMU LUT entry 440 high"]
pub mod lut440h;
#[doc = "LUT441H register accessor: an alias for `Reg<LUT441H_SPEC>`"]
pub type LUT441H = crate::Reg<lut441h::LUT441H_SPEC>;
#[doc = "Graphic MMU LUT entry 441 high"]
pub mod lut441h;
#[doc = "LUT442H register accessor: an alias for `Reg<LUT442H_SPEC>`"]
pub type LUT442H = crate::Reg<lut442h::LUT442H_SPEC>;
#[doc = "Graphic MMU LUT entry 442 high"]
pub mod lut442h;
#[doc = "LUT443H register accessor: an alias for `Reg<LUT443H_SPEC>`"]
pub type LUT443H = crate::Reg<lut443h::LUT443H_SPEC>;
#[doc = "Graphic MMU LUT entry 443 high"]
pub mod lut443h;
#[doc = "LUT444H register accessor: an alias for `Reg<LUT444H_SPEC>`"]
pub type LUT444H = crate::Reg<lut444h::LUT444H_SPEC>;
#[doc = "Graphic MMU LUT entry 444 high"]
pub mod lut444h;
#[doc = "LUT445H register accessor: an alias for `Reg<LUT445H_SPEC>`"]
pub type LUT445H = crate::Reg<lut445h::LUT445H_SPEC>;
#[doc = "Graphic MMU LUT entry 445 high"]
pub mod lut445h;
#[doc = "LUT446H register accessor: an alias for `Reg<LUT446H_SPEC>`"]
pub type LUT446H = crate::Reg<lut446h::LUT446H_SPEC>;
#[doc = "Graphic MMU LUT entry 446 high"]
pub mod lut446h;
#[doc = "LUT447H register accessor: an alias for `Reg<LUT447H_SPEC>`"]
pub type LUT447H = crate::Reg<lut447h::LUT447H_SPEC>;
#[doc = "Graphic MMU LUT entry 447 high"]
pub mod lut447h;
#[doc = "LUT448H register accessor: an alias for `Reg<LUT448H_SPEC>`"]
pub type LUT448H = crate::Reg<lut448h::LUT448H_SPEC>;
#[doc = "Graphic MMU LUT entry 448 high"]
pub mod lut448h;
#[doc = "LUT449H register accessor: an alias for `Reg<LUT449H_SPEC>`"]
pub type LUT449H = crate::Reg<lut449h::LUT449H_SPEC>;
#[doc = "Graphic MMU LUT entry 449 high"]
pub mod lut449h;
#[doc = "LUT450H register accessor: an alias for `Reg<LUT450H_SPEC>`"]
pub type LUT450H = crate::Reg<lut450h::LUT450H_SPEC>;
#[doc = "Graphic MMU LUT entry 450 high"]
pub mod lut450h;
#[doc = "LUT451H register accessor: an alias for `Reg<LUT451H_SPEC>`"]
pub type LUT451H = crate::Reg<lut451h::LUT451H_SPEC>;
#[doc = "Graphic MMU LUT entry 451 high"]
pub mod lut451h;
#[doc = "LUT452H register accessor: an alias for `Reg<LUT452H_SPEC>`"]
pub type LUT452H = crate::Reg<lut452h::LUT452H_SPEC>;
#[doc = "Graphic MMU LUT entry 452 high"]
pub mod lut452h;
#[doc = "LUT453H register accessor: an alias for `Reg<LUT453H_SPEC>`"]
pub type LUT453H = crate::Reg<lut453h::LUT453H_SPEC>;
#[doc = "Graphic MMU LUT entry 453 high"]
pub mod lut453h;
#[doc = "LUT454H register accessor: an alias for `Reg<LUT454H_SPEC>`"]
pub type LUT454H = crate::Reg<lut454h::LUT454H_SPEC>;
#[doc = "Graphic MMU LUT entry 454 high"]
pub mod lut454h;
#[doc = "LUT455H register accessor: an alias for `Reg<LUT455H_SPEC>`"]
pub type LUT455H = crate::Reg<lut455h::LUT455H_SPEC>;
#[doc = "Graphic MMU LUT entry 455 high"]
pub mod lut455h;
#[doc = "LUT456H register accessor: an alias for `Reg<LUT456H_SPEC>`"]
pub type LUT456H = crate::Reg<lut456h::LUT456H_SPEC>;
#[doc = "Graphic MMU LUT entry 456 high"]
pub mod lut456h;
#[doc = "LUT457H register accessor: an alias for `Reg<LUT457H_SPEC>`"]
pub type LUT457H = crate::Reg<lut457h::LUT457H_SPEC>;
#[doc = "Graphic MMU LUT entry 457 high"]
pub mod lut457h;
#[doc = "LUT458H register accessor: an alias for `Reg<LUT458H_SPEC>`"]
pub type LUT458H = crate::Reg<lut458h::LUT458H_SPEC>;
#[doc = "Graphic MMU LUT entry 458 high"]
pub mod lut458h;
#[doc = "LUT459H register accessor: an alias for `Reg<LUT459H_SPEC>`"]
pub type LUT459H = crate::Reg<lut459h::LUT459H_SPEC>;
#[doc = "Graphic MMU LUT entry 459 high"]
pub mod lut459h;
#[doc = "LUT460H register accessor: an alias for `Reg<LUT460H_SPEC>`"]
pub type LUT460H = crate::Reg<lut460h::LUT460H_SPEC>;
#[doc = "Graphic MMU LUT entry 460 high"]
pub mod lut460h;
#[doc = "LUT461H register accessor: an alias for `Reg<LUT461H_SPEC>`"]
pub type LUT461H = crate::Reg<lut461h::LUT461H_SPEC>;
#[doc = "Graphic MMU LUT entry 461 high"]
pub mod lut461h;
#[doc = "LUT462H register accessor: an alias for `Reg<LUT462H_SPEC>`"]
pub type LUT462H = crate::Reg<lut462h::LUT462H_SPEC>;
#[doc = "Graphic MMU LUT entry 462 high"]
pub mod lut462h;
#[doc = "LUT463H register accessor: an alias for `Reg<LUT463H_SPEC>`"]
pub type LUT463H = crate::Reg<lut463h::LUT463H_SPEC>;
#[doc = "Graphic MMU LUT entry 463 high"]
pub mod lut463h;
#[doc = "LUT464H register accessor: an alias for `Reg<LUT464H_SPEC>`"]
pub type LUT464H = crate::Reg<lut464h::LUT464H_SPEC>;
#[doc = "Graphic MMU LUT entry 464 high"]
pub mod lut464h;
#[doc = "LUT465H register accessor: an alias for `Reg<LUT465H_SPEC>`"]
pub type LUT465H = crate::Reg<lut465h::LUT465H_SPEC>;
#[doc = "Graphic MMU LUT entry 465 high"]
pub mod lut465h;
#[doc = "LUT466H register accessor: an alias for `Reg<LUT466H_SPEC>`"]
pub type LUT466H = crate::Reg<lut466h::LUT466H_SPEC>;
#[doc = "Graphic MMU LUT entry 466 high"]
pub mod lut466h;
#[doc = "LUT467H register accessor: an alias for `Reg<LUT467H_SPEC>`"]
pub type LUT467H = crate::Reg<lut467h::LUT467H_SPEC>;
#[doc = "Graphic MMU LUT entry 467 high"]
pub mod lut467h;
#[doc = "LUT468H register accessor: an alias for `Reg<LUT468H_SPEC>`"]
pub type LUT468H = crate::Reg<lut468h::LUT468H_SPEC>;
#[doc = "Graphic MMU LUT entry 468 high"]
pub mod lut468h;
#[doc = "LUT469H register accessor: an alias for `Reg<LUT469H_SPEC>`"]
pub type LUT469H = crate::Reg<lut469h::LUT469H_SPEC>;
#[doc = "Graphic MMU LUT entry 469 high"]
pub mod lut469h;
#[doc = "LUT470H register accessor: an alias for `Reg<LUT470H_SPEC>`"]
pub type LUT470H = crate::Reg<lut470h::LUT470H_SPEC>;
#[doc = "Graphic MMU LUT entry 470 high"]
pub mod lut470h;
#[doc = "LUT471H register accessor: an alias for `Reg<LUT471H_SPEC>`"]
pub type LUT471H = crate::Reg<lut471h::LUT471H_SPEC>;
#[doc = "Graphic MMU LUT entry 471 high"]
pub mod lut471h;
#[doc = "LUT472H register accessor: an alias for `Reg<LUT472H_SPEC>`"]
pub type LUT472H = crate::Reg<lut472h::LUT472H_SPEC>;
#[doc = "Graphic MMU LUT entry 472 high"]
pub mod lut472h;
#[doc = "LUT473H register accessor: an alias for `Reg<LUT473H_SPEC>`"]
pub type LUT473H = crate::Reg<lut473h::LUT473H_SPEC>;
#[doc = "Graphic MMU LUT entry 473 high"]
pub mod lut473h;
#[doc = "LUT474H register accessor: an alias for `Reg<LUT474H_SPEC>`"]
pub type LUT474H = crate::Reg<lut474h::LUT474H_SPEC>;
#[doc = "Graphic MMU LUT entry 474 high"]
pub mod lut474h;
#[doc = "LUT475H register accessor: an alias for `Reg<LUT475H_SPEC>`"]
pub type LUT475H = crate::Reg<lut475h::LUT475H_SPEC>;
#[doc = "Graphic MMU LUT entry 475 high"]
pub mod lut475h;
#[doc = "LUT476H register accessor: an alias for `Reg<LUT476H_SPEC>`"]
pub type LUT476H = crate::Reg<lut476h::LUT476H_SPEC>;
#[doc = "Graphic MMU LUT entry 476 high"]
pub mod lut476h;
#[doc = "LUT477H register accessor: an alias for `Reg<LUT477H_SPEC>`"]
pub type LUT477H = crate::Reg<lut477h::LUT477H_SPEC>;
#[doc = "Graphic MMU LUT entry 477 high"]
pub mod lut477h;
#[doc = "LUT478H register accessor: an alias for `Reg<LUT478H_SPEC>`"]
pub type LUT478H = crate::Reg<lut478h::LUT478H_SPEC>;
#[doc = "Graphic MMU LUT entry 478 high"]
pub mod lut478h;
#[doc = "LUT479H register accessor: an alias for `Reg<LUT479H_SPEC>`"]
pub type LUT479H = crate::Reg<lut479h::LUT479H_SPEC>;
#[doc = "Graphic MMU LUT entry 479 high"]
pub mod lut479h;
#[doc = "LUT480H register accessor: an alias for `Reg<LUT480H_SPEC>`"]
pub type LUT480H = crate::Reg<lut480h::LUT480H_SPEC>;
#[doc = "Graphic MMU LUT entry 480 high"]
pub mod lut480h;
#[doc = "LUT481H register accessor: an alias for `Reg<LUT481H_SPEC>`"]
pub type LUT481H = crate::Reg<lut481h::LUT481H_SPEC>;
#[doc = "Graphic MMU LUT entry 481 high"]
pub mod lut481h;
#[doc = "LUT482H register accessor: an alias for `Reg<LUT482H_SPEC>`"]
pub type LUT482H = crate::Reg<lut482h::LUT482H_SPEC>;
#[doc = "Graphic MMU LUT entry 482 high"]
pub mod lut482h;
#[doc = "LUT483H register accessor: an alias for `Reg<LUT483H_SPEC>`"]
pub type LUT483H = crate::Reg<lut483h::LUT483H_SPEC>;
#[doc = "Graphic MMU LUT entry 483 high"]
pub mod lut483h;
#[doc = "LUT484H register accessor: an alias for `Reg<LUT484H_SPEC>`"]
pub type LUT484H = crate::Reg<lut484h::LUT484H_SPEC>;
#[doc = "Graphic MMU LUT entry 484 high"]
pub mod lut484h;
#[doc = "LUT485H register accessor: an alias for `Reg<LUT485H_SPEC>`"]
pub type LUT485H = crate::Reg<lut485h::LUT485H_SPEC>;
#[doc = "Graphic MMU LUT entry 485 high"]
pub mod lut485h;
#[doc = "LUT486H register accessor: an alias for `Reg<LUT486H_SPEC>`"]
pub type LUT486H = crate::Reg<lut486h::LUT486H_SPEC>;
#[doc = "Graphic MMU LUT entry 486 high"]
pub mod lut486h;
#[doc = "LUT487H register accessor: an alias for `Reg<LUT487H_SPEC>`"]
pub type LUT487H = crate::Reg<lut487h::LUT487H_SPEC>;
#[doc = "Graphic MMU LUT entry 487 high"]
pub mod lut487h;
#[doc = "LUT488H register accessor: an alias for `Reg<LUT488H_SPEC>`"]
pub type LUT488H = crate::Reg<lut488h::LUT488H_SPEC>;
#[doc = "Graphic MMU LUT entry 488 high"]
pub mod lut488h;
#[doc = "LUT489H register accessor: an alias for `Reg<LUT489H_SPEC>`"]
pub type LUT489H = crate::Reg<lut489h::LUT489H_SPEC>;
#[doc = "Graphic MMU LUT entry 489 high"]
pub mod lut489h;
#[doc = "LUT490H register accessor: an alias for `Reg<LUT490H_SPEC>`"]
pub type LUT490H = crate::Reg<lut490h::LUT490H_SPEC>;
#[doc = "Graphic MMU LUT entry 490 high"]
pub mod lut490h;
#[doc = "LUT491H register accessor: an alias for `Reg<LUT491H_SPEC>`"]
pub type LUT491H = crate::Reg<lut491h::LUT491H_SPEC>;
#[doc = "Graphic MMU LUT entry 491 high"]
pub mod lut491h;
#[doc = "LUT492H register accessor: an alias for `Reg<LUT492H_SPEC>`"]
pub type LUT492H = crate::Reg<lut492h::LUT492H_SPEC>;
#[doc = "Graphic MMU LUT entry 492 high"]
pub mod lut492h;
#[doc = "LUT493H register accessor: an alias for `Reg<LUT493H_SPEC>`"]
pub type LUT493H = crate::Reg<lut493h::LUT493H_SPEC>;
#[doc = "Graphic MMU LUT entry 493 high"]
pub mod lut493h;
#[doc = "LUT494H register accessor: an alias for `Reg<LUT494H_SPEC>`"]
pub type LUT494H = crate::Reg<lut494h::LUT494H_SPEC>;
#[doc = "Graphic MMU LUT entry 494 high"]
pub mod lut494h;
#[doc = "LUT495H register accessor: an alias for `Reg<LUT495H_SPEC>`"]
pub type LUT495H = crate::Reg<lut495h::LUT495H_SPEC>;
#[doc = "Graphic MMU LUT entry 495 high"]
pub mod lut495h;
#[doc = "LUT496H register accessor: an alias for `Reg<LUT496H_SPEC>`"]
pub type LUT496H = crate::Reg<lut496h::LUT496H_SPEC>;
#[doc = "Graphic MMU LUT entry 496 high"]
pub mod lut496h;
#[doc = "LUT497H register accessor: an alias for `Reg<LUT497H_SPEC>`"]
pub type LUT497H = crate::Reg<lut497h::LUT497H_SPEC>;
#[doc = "Graphic MMU LUT entry 497 high"]
pub mod lut497h;
#[doc = "LUT498H register accessor: an alias for `Reg<LUT498H_SPEC>`"]
pub type LUT498H = crate::Reg<lut498h::LUT498H_SPEC>;
#[doc = "Graphic MMU LUT entry 498 high"]
pub mod lut498h;
#[doc = "LUT499H register accessor: an alias for `Reg<LUT499H_SPEC>`"]
pub type LUT499H = crate::Reg<lut499h::LUT499H_SPEC>;
#[doc = "Graphic MMU LUT entry 499 high"]
pub mod lut499h;
#[doc = "LUT500H register accessor: an alias for `Reg<LUT500H_SPEC>`"]
pub type LUT500H = crate::Reg<lut500h::LUT500H_SPEC>;
#[doc = "Graphic MMU LUT entry 500 high"]
pub mod lut500h;
#[doc = "LUT501H register accessor: an alias for `Reg<LUT501H_SPEC>`"]
pub type LUT501H = crate::Reg<lut501h::LUT501H_SPEC>;
#[doc = "Graphic MMU LUT entry 501 high"]
pub mod lut501h;
#[doc = "LUT502H register accessor: an alias for `Reg<LUT502H_SPEC>`"]
pub type LUT502H = crate::Reg<lut502h::LUT502H_SPEC>;
#[doc = "Graphic MMU LUT entry 502 high"]
pub mod lut502h;
#[doc = "LUT503H register accessor: an alias for `Reg<LUT503H_SPEC>`"]
pub type LUT503H = crate::Reg<lut503h::LUT503H_SPEC>;
#[doc = "Graphic MMU LUT entry 503 high"]
pub mod lut503h;
#[doc = "LUT504H register accessor: an alias for `Reg<LUT504H_SPEC>`"]
pub type LUT504H = crate::Reg<lut504h::LUT504H_SPEC>;
#[doc = "Graphic MMU LUT entry 504 high"]
pub mod lut504h;
#[doc = "LUT505H register accessor: an alias for `Reg<LUT505H_SPEC>`"]
pub type LUT505H = crate::Reg<lut505h::LUT505H_SPEC>;
#[doc = "Graphic MMU LUT entry 505 high"]
pub mod lut505h;
#[doc = "LUT506H register accessor: an alias for `Reg<LUT506H_SPEC>`"]
pub type LUT506H = crate::Reg<lut506h::LUT506H_SPEC>;
#[doc = "Graphic MMU LUT entry 506 high"]
pub mod lut506h;
#[doc = "LUT507H register accessor: an alias for `Reg<LUT507H_SPEC>`"]
pub type LUT507H = crate::Reg<lut507h::LUT507H_SPEC>;
#[doc = "Graphic MMU LUT entry 507 high"]
pub mod lut507h;
#[doc = "LUT508H register accessor: an alias for `Reg<LUT508H_SPEC>`"]
pub type LUT508H = crate::Reg<lut508h::LUT508H_SPEC>;
#[doc = "Graphic MMU LUT entry 508 high"]
pub mod lut508h;
#[doc = "LUT509H register accessor: an alias for `Reg<LUT509H_SPEC>`"]
pub type LUT509H = crate::Reg<lut509h::LUT509H_SPEC>;
#[doc = "Graphic MMU LUT entry 509 high"]
pub mod lut509h;
#[doc = "LUT510H register accessor: an alias for `Reg<LUT510H_SPEC>`"]
pub type LUT510H = crate::Reg<lut510h::LUT510H_SPEC>;
#[doc = "Graphic MMU LUT entry 510 high"]
pub mod lut510h;
#[doc = "LUT511H register accessor: an alias for `Reg<LUT511H_SPEC>`"]
pub type LUT511H = crate::Reg<lut511h::LUT511H_SPEC>;
#[doc = "Graphic MMU LUT entry 511 high"]
pub mod lut511h;
#[doc = "LUT512H register accessor: an alias for `Reg<LUT512H_SPEC>`"]
pub type LUT512H = crate::Reg<lut512h::LUT512H_SPEC>;
#[doc = "Graphic MMU LUT entry 512 high"]
pub mod lut512h;
#[doc = "LUT513H register accessor: an alias for `Reg<LUT513H_SPEC>`"]
pub type LUT513H = crate::Reg<lut513h::LUT513H_SPEC>;
#[doc = "Graphic MMU LUT entry 513 high"]
pub mod lut513h;
#[doc = "LUT514H register accessor: an alias for `Reg<LUT514H_SPEC>`"]
pub type LUT514H = crate::Reg<lut514h::LUT514H_SPEC>;
#[doc = "Graphic MMU LUT entry 514 high"]
pub mod lut514h;
#[doc = "LUT515H register accessor: an alias for `Reg<LUT515H_SPEC>`"]
pub type LUT515H = crate::Reg<lut515h::LUT515H_SPEC>;
#[doc = "Graphic MMU LUT entry 515 high"]
pub mod lut515h;
#[doc = "LUT516H register accessor: an alias for `Reg<LUT516H_SPEC>`"]
pub type LUT516H = crate::Reg<lut516h::LUT516H_SPEC>;
#[doc = "Graphic MMU LUT entry 516 high"]
pub mod lut516h;
#[doc = "LUT517H register accessor: an alias for `Reg<LUT517H_SPEC>`"]
pub type LUT517H = crate::Reg<lut517h::LUT517H_SPEC>;
#[doc = "Graphic MMU LUT entry 517 high"]
pub mod lut517h;
#[doc = "LUT518H register accessor: an alias for `Reg<LUT518H_SPEC>`"]
pub type LUT518H = crate::Reg<lut518h::LUT518H_SPEC>;
#[doc = "Graphic MMU LUT entry 518 high"]
pub mod lut518h;
#[doc = "LUT519H register accessor: an alias for `Reg<LUT519H_SPEC>`"]
pub type LUT519H = crate::Reg<lut519h::LUT519H_SPEC>;
#[doc = "Graphic MMU LUT entry 519 high"]
pub mod lut519h;
#[doc = "LUT520H register accessor: an alias for `Reg<LUT520H_SPEC>`"]
pub type LUT520H = crate::Reg<lut520h::LUT520H_SPEC>;
#[doc = "Graphic MMU LUT entry 520 high"]
pub mod lut520h;
#[doc = "LUT521H register accessor: an alias for `Reg<LUT521H_SPEC>`"]
pub type LUT521H = crate::Reg<lut521h::LUT521H_SPEC>;
#[doc = "Graphic MMU LUT entry 521 high"]
pub mod lut521h;
#[doc = "LUT522H register accessor: an alias for `Reg<LUT522H_SPEC>`"]
pub type LUT522H = crate::Reg<lut522h::LUT522H_SPEC>;
#[doc = "Graphic MMU LUT entry 522 high"]
pub mod lut522h;
#[doc = "LUT523H register accessor: an alias for `Reg<LUT523H_SPEC>`"]
pub type LUT523H = crate::Reg<lut523h::LUT523H_SPEC>;
#[doc = "Graphic MMU LUT entry 523 high"]
pub mod lut523h;
#[doc = "LUT524H register accessor: an alias for `Reg<LUT524H_SPEC>`"]
pub type LUT524H = crate::Reg<lut524h::LUT524H_SPEC>;
#[doc = "Graphic MMU LUT entry 524 high"]
pub mod lut524h;
#[doc = "LUT525H register accessor: an alias for `Reg<LUT525H_SPEC>`"]
pub type LUT525H = crate::Reg<lut525h::LUT525H_SPEC>;
#[doc = "Graphic MMU LUT entry 525 high"]
pub mod lut525h;
#[doc = "LUT526H register accessor: an alias for `Reg<LUT526H_SPEC>`"]
pub type LUT526H = crate::Reg<lut526h::LUT526H_SPEC>;
#[doc = "Graphic MMU LUT entry 526 high"]
pub mod lut526h;
#[doc = "LUT527H register accessor: an alias for `Reg<LUT527H_SPEC>`"]
pub type LUT527H = crate::Reg<lut527h::LUT527H_SPEC>;
#[doc = "Graphic MMU LUT entry 527 high"]
pub mod lut527h;
#[doc = "LUT528H register accessor: an alias for `Reg<LUT528H_SPEC>`"]
pub type LUT528H = crate::Reg<lut528h::LUT528H_SPEC>;
#[doc = "Graphic MMU LUT entry 528 high"]
pub mod lut528h;
#[doc = "LUT529H register accessor: an alias for `Reg<LUT529H_SPEC>`"]
pub type LUT529H = crate::Reg<lut529h::LUT529H_SPEC>;
#[doc = "Graphic MMU LUT entry 529 high"]
pub mod lut529h;
#[doc = "LUT530H register accessor: an alias for `Reg<LUT530H_SPEC>`"]
pub type LUT530H = crate::Reg<lut530h::LUT530H_SPEC>;
#[doc = "Graphic MMU LUT entry 530 high"]
pub mod lut530h;
#[doc = "LUT531H register accessor: an alias for `Reg<LUT531H_SPEC>`"]
pub type LUT531H = crate::Reg<lut531h::LUT531H_SPEC>;
#[doc = "Graphic MMU LUT entry 531 high"]
pub mod lut531h;
#[doc = "LUT532H register accessor: an alias for `Reg<LUT532H_SPEC>`"]
pub type LUT532H = crate::Reg<lut532h::LUT532H_SPEC>;
#[doc = "Graphic MMU LUT entry 532 high"]
pub mod lut532h;
#[doc = "LUT533H register accessor: an alias for `Reg<LUT533H_SPEC>`"]
pub type LUT533H = crate::Reg<lut533h::LUT533H_SPEC>;
#[doc = "Graphic MMU LUT entry 533 high"]
pub mod lut533h;
#[doc = "LUT534H register accessor: an alias for `Reg<LUT534H_SPEC>`"]
pub type LUT534H = crate::Reg<lut534h::LUT534H_SPEC>;
#[doc = "Graphic MMU LUT entry 534 high"]
pub mod lut534h;
#[doc = "LUT535H register accessor: an alias for `Reg<LUT535H_SPEC>`"]
pub type LUT535H = crate::Reg<lut535h::LUT535H_SPEC>;
#[doc = "Graphic MMU LUT entry 535 high"]
pub mod lut535h;
#[doc = "LUT536H register accessor: an alias for `Reg<LUT536H_SPEC>`"]
pub type LUT536H = crate::Reg<lut536h::LUT536H_SPEC>;
#[doc = "Graphic MMU LUT entry 536 high"]
pub mod lut536h;
#[doc = "LUT537H register accessor: an alias for `Reg<LUT537H_SPEC>`"]
pub type LUT537H = crate::Reg<lut537h::LUT537H_SPEC>;
#[doc = "Graphic MMU LUT entry 537 high"]
pub mod lut537h;
#[doc = "LUT538H register accessor: an alias for `Reg<LUT538H_SPEC>`"]
pub type LUT538H = crate::Reg<lut538h::LUT538H_SPEC>;
#[doc = "Graphic MMU LUT entry 538 high"]
pub mod lut538h;
#[doc = "LUT539H register accessor: an alias for `Reg<LUT539H_SPEC>`"]
pub type LUT539H = crate::Reg<lut539h::LUT539H_SPEC>;
#[doc = "Graphic MMU LUT entry 539 high"]
pub mod lut539h;
#[doc = "LUT540H register accessor: an alias for `Reg<LUT540H_SPEC>`"]
pub type LUT540H = crate::Reg<lut540h::LUT540H_SPEC>;
#[doc = "Graphic MMU LUT entry 540 high"]
pub mod lut540h;
#[doc = "LUT541H register accessor: an alias for `Reg<LUT541H_SPEC>`"]
pub type LUT541H = crate::Reg<lut541h::LUT541H_SPEC>;
#[doc = "Graphic MMU LUT entry 541 high"]
pub mod lut541h;
#[doc = "LUT542H register accessor: an alias for `Reg<LUT542H_SPEC>`"]
pub type LUT542H = crate::Reg<lut542h::LUT542H_SPEC>;
#[doc = "Graphic MMU LUT entry 542 high"]
pub mod lut542h;
#[doc = "LUT543H register accessor: an alias for `Reg<LUT543H_SPEC>`"]
pub type LUT543H = crate::Reg<lut543h::LUT543H_SPEC>;
#[doc = "Graphic MMU LUT entry 543 high"]
pub mod lut543h;
#[doc = "LUT544H register accessor: an alias for `Reg<LUT544H_SPEC>`"]
pub type LUT544H = crate::Reg<lut544h::LUT544H_SPEC>;
#[doc = "Graphic MMU LUT entry 544 high"]
pub mod lut544h;
#[doc = "LUT545H register accessor: an alias for `Reg<LUT545H_SPEC>`"]
pub type LUT545H = crate::Reg<lut545h::LUT545H_SPEC>;
#[doc = "Graphic MMU LUT entry 545 high"]
pub mod lut545h;
#[doc = "LUT546H register accessor: an alias for `Reg<LUT546H_SPEC>`"]
pub type LUT546H = crate::Reg<lut546h::LUT546H_SPEC>;
#[doc = "Graphic MMU LUT entry 546 high"]
pub mod lut546h;
#[doc = "LUT547H register accessor: an alias for `Reg<LUT547H_SPEC>`"]
pub type LUT547H = crate::Reg<lut547h::LUT547H_SPEC>;
#[doc = "Graphic MMU LUT entry 547 high"]
pub mod lut547h;
#[doc = "LUT548H register accessor: an alias for `Reg<LUT548H_SPEC>`"]
pub type LUT548H = crate::Reg<lut548h::LUT548H_SPEC>;
#[doc = "Graphic MMU LUT entry 548 high"]
pub mod lut548h;
#[doc = "LUT549H register accessor: an alias for `Reg<LUT549H_SPEC>`"]
pub type LUT549H = crate::Reg<lut549h::LUT549H_SPEC>;
#[doc = "Graphic MMU LUT entry 549 high"]
pub mod lut549h;
#[doc = "LUT550H register accessor: an alias for `Reg<LUT550H_SPEC>`"]
pub type LUT550H = crate::Reg<lut550h::LUT550H_SPEC>;
#[doc = "Graphic MMU LUT entry 550 high"]
pub mod lut550h;
#[doc = "LUT551H register accessor: an alias for `Reg<LUT551H_SPEC>`"]
pub type LUT551H = crate::Reg<lut551h::LUT551H_SPEC>;
#[doc = "Graphic MMU LUT entry 551 high"]
pub mod lut551h;
#[doc = "LUT552H register accessor: an alias for `Reg<LUT552H_SPEC>`"]
pub type LUT552H = crate::Reg<lut552h::LUT552H_SPEC>;
#[doc = "Graphic MMU LUT entry 552 high"]
pub mod lut552h;
#[doc = "LUT553H register accessor: an alias for `Reg<LUT553H_SPEC>`"]
pub type LUT553H = crate::Reg<lut553h::LUT553H_SPEC>;
#[doc = "Graphic MMU LUT entry 553 high"]
pub mod lut553h;
#[doc = "LUT554H register accessor: an alias for `Reg<LUT554H_SPEC>`"]
pub type LUT554H = crate::Reg<lut554h::LUT554H_SPEC>;
#[doc = "Graphic MMU LUT entry 554 high"]
pub mod lut554h;
#[doc = "LUT555H register accessor: an alias for `Reg<LUT555H_SPEC>`"]
pub type LUT555H = crate::Reg<lut555h::LUT555H_SPEC>;
#[doc = "Graphic MMU LUT entry 555 high"]
pub mod lut555h;
#[doc = "LUT556H register accessor: an alias for `Reg<LUT556H_SPEC>`"]
pub type LUT556H = crate::Reg<lut556h::LUT556H_SPEC>;
#[doc = "Graphic MMU LUT entry 556 high"]
pub mod lut556h;
#[doc = "LUT557H register accessor: an alias for `Reg<LUT557H_SPEC>`"]
pub type LUT557H = crate::Reg<lut557h::LUT557H_SPEC>;
#[doc = "Graphic MMU LUT entry 557 high"]
pub mod lut557h;
#[doc = "LUT558H register accessor: an alias for `Reg<LUT558H_SPEC>`"]
pub type LUT558H = crate::Reg<lut558h::LUT558H_SPEC>;
#[doc = "Graphic MMU LUT entry 558 high"]
pub mod lut558h;
#[doc = "LUT559H register accessor: an alias for `Reg<LUT559H_SPEC>`"]
pub type LUT559H = crate::Reg<lut559h::LUT559H_SPEC>;
#[doc = "Graphic MMU LUT entry 559 high"]
pub mod lut559h;
#[doc = "LUT560H register accessor: an alias for `Reg<LUT560H_SPEC>`"]
pub type LUT560H = crate::Reg<lut560h::LUT560H_SPEC>;
#[doc = "Graphic MMU LUT entry 560 high"]
pub mod lut560h;
#[doc = "LUT561H register accessor: an alias for `Reg<LUT561H_SPEC>`"]
pub type LUT561H = crate::Reg<lut561h::LUT561H_SPEC>;
#[doc = "Graphic MMU LUT entry 561 high"]
pub mod lut561h;
#[doc = "LUT562H register accessor: an alias for `Reg<LUT562H_SPEC>`"]
pub type LUT562H = crate::Reg<lut562h::LUT562H_SPEC>;
#[doc = "Graphic MMU LUT entry 562 high"]
pub mod lut562h;
#[doc = "LUT563H register accessor: an alias for `Reg<LUT563H_SPEC>`"]
pub type LUT563H = crate::Reg<lut563h::LUT563H_SPEC>;
#[doc = "Graphic MMU LUT entry 563 high"]
pub mod lut563h;
#[doc = "LUT564H register accessor: an alias for `Reg<LUT564H_SPEC>`"]
pub type LUT564H = crate::Reg<lut564h::LUT564H_SPEC>;
#[doc = "Graphic MMU LUT entry 564 high"]
pub mod lut564h;
#[doc = "LUT565H register accessor: an alias for `Reg<LUT565H_SPEC>`"]
pub type LUT565H = crate::Reg<lut565h::LUT565H_SPEC>;
#[doc = "Graphic MMU LUT entry 565 high"]
pub mod lut565h;
#[doc = "LUT566H register accessor: an alias for `Reg<LUT566H_SPEC>`"]
pub type LUT566H = crate::Reg<lut566h::LUT566H_SPEC>;
#[doc = "Graphic MMU LUT entry 566 high"]
pub mod lut566h;
#[doc = "LUT567H register accessor: an alias for `Reg<LUT567H_SPEC>`"]
pub type LUT567H = crate::Reg<lut567h::LUT567H_SPEC>;
#[doc = "Graphic MMU LUT entry 567 high"]
pub mod lut567h;
#[doc = "LUT568H register accessor: an alias for `Reg<LUT568H_SPEC>`"]
pub type LUT568H = crate::Reg<lut568h::LUT568H_SPEC>;
#[doc = "Graphic MMU LUT entry 568 high"]
pub mod lut568h;
#[doc = "LUT569H register accessor: an alias for `Reg<LUT569H_SPEC>`"]
pub type LUT569H = crate::Reg<lut569h::LUT569H_SPEC>;
#[doc = "Graphic MMU LUT entry 569 high"]
pub mod lut569h;
#[doc = "LUT570H register accessor: an alias for `Reg<LUT570H_SPEC>`"]
pub type LUT570H = crate::Reg<lut570h::LUT570H_SPEC>;
#[doc = "Graphic MMU LUT entry 570 high"]
pub mod lut570h;
#[doc = "LUT571H register accessor: an alias for `Reg<LUT571H_SPEC>`"]
pub type LUT571H = crate::Reg<lut571h::LUT571H_SPEC>;
#[doc = "Graphic MMU LUT entry 571 high"]
pub mod lut571h;
#[doc = "LUT572H register accessor: an alias for `Reg<LUT572H_SPEC>`"]
pub type LUT572H = crate::Reg<lut572h::LUT572H_SPEC>;
#[doc = "Graphic MMU LUT entry 572 high"]
pub mod lut572h;
#[doc = "LUT573H register accessor: an alias for `Reg<LUT573H_SPEC>`"]
pub type LUT573H = crate::Reg<lut573h::LUT573H_SPEC>;
#[doc = "Graphic MMU LUT entry 573 high"]
pub mod lut573h;
#[doc = "LUT574H register accessor: an alias for `Reg<LUT574H_SPEC>`"]
pub type LUT574H = crate::Reg<lut574h::LUT574H_SPEC>;
#[doc = "Graphic MMU LUT entry 574 high"]
pub mod lut574h;
#[doc = "LUT575H register accessor: an alias for `Reg<LUT575H_SPEC>`"]
pub type LUT575H = crate::Reg<lut575h::LUT575H_SPEC>;
#[doc = "Graphic MMU LUT entry 575 high"]
pub mod lut575h;
#[doc = "LUT576H register accessor: an alias for `Reg<LUT576H_SPEC>`"]
pub type LUT576H = crate::Reg<lut576h::LUT576H_SPEC>;
#[doc = "Graphic MMU LUT entry 576 high"]
pub mod lut576h;
#[doc = "LUT577H register accessor: an alias for `Reg<LUT577H_SPEC>`"]
pub type LUT577H = crate::Reg<lut577h::LUT577H_SPEC>;
#[doc = "Graphic MMU LUT entry 577 high"]
pub mod lut577h;
#[doc = "LUT578H register accessor: an alias for `Reg<LUT578H_SPEC>`"]
pub type LUT578H = crate::Reg<lut578h::LUT578H_SPEC>;
#[doc = "Graphic MMU LUT entry 578 high"]
pub mod lut578h;
#[doc = "LUT579H register accessor: an alias for `Reg<LUT579H_SPEC>`"]
pub type LUT579H = crate::Reg<lut579h::LUT579H_SPEC>;
#[doc = "Graphic MMU LUT entry 579 high"]
pub mod lut579h;
#[doc = "LUT580H register accessor: an alias for `Reg<LUT580H_SPEC>`"]
pub type LUT580H = crate::Reg<lut580h::LUT580H_SPEC>;
#[doc = "Graphic MMU LUT entry 580 high"]
pub mod lut580h;
#[doc = "LUT581H register accessor: an alias for `Reg<LUT581H_SPEC>`"]
pub type LUT581H = crate::Reg<lut581h::LUT581H_SPEC>;
#[doc = "Graphic MMU LUT entry 581 high"]
pub mod lut581h;
#[doc = "LUT582H register accessor: an alias for `Reg<LUT582H_SPEC>`"]
pub type LUT582H = crate::Reg<lut582h::LUT582H_SPEC>;
#[doc = "Graphic MMU LUT entry 582 high"]
pub mod lut582h;
#[doc = "LUT583H register accessor: an alias for `Reg<LUT583H_SPEC>`"]
pub type LUT583H = crate::Reg<lut583h::LUT583H_SPEC>;
#[doc = "Graphic MMU LUT entry 583 high"]
pub mod lut583h;
#[doc = "LUT584H register accessor: an alias for `Reg<LUT584H_SPEC>`"]
pub type LUT584H = crate::Reg<lut584h::LUT584H_SPEC>;
#[doc = "Graphic MMU LUT entry 584 high"]
pub mod lut584h;
#[doc = "LUT585H register accessor: an alias for `Reg<LUT585H_SPEC>`"]
pub type LUT585H = crate::Reg<lut585h::LUT585H_SPEC>;
#[doc = "Graphic MMU LUT entry 585 high"]
pub mod lut585h;
#[doc = "LUT586H register accessor: an alias for `Reg<LUT586H_SPEC>`"]
pub type LUT586H = crate::Reg<lut586h::LUT586H_SPEC>;
#[doc = "Graphic MMU LUT entry 586 high"]
pub mod lut586h;
#[doc = "LUT587H register accessor: an alias for `Reg<LUT587H_SPEC>`"]
pub type LUT587H = crate::Reg<lut587h::LUT587H_SPEC>;
#[doc = "Graphic MMU LUT entry 587 high"]
pub mod lut587h;
#[doc = "LUT588H register accessor: an alias for `Reg<LUT588H_SPEC>`"]
pub type LUT588H = crate::Reg<lut588h::LUT588H_SPEC>;
#[doc = "Graphic MMU LUT entry 588 high"]
pub mod lut588h;
#[doc = "LUT589H register accessor: an alias for `Reg<LUT589H_SPEC>`"]
pub type LUT589H = crate::Reg<lut589h::LUT589H_SPEC>;
#[doc = "Graphic MMU LUT entry 589 high"]
pub mod lut589h;
#[doc = "LUT590H register accessor: an alias for `Reg<LUT590H_SPEC>`"]
pub type LUT590H = crate::Reg<lut590h::LUT590H_SPEC>;
#[doc = "Graphic MMU LUT entry 590 high"]
pub mod lut590h;
#[doc = "LUT591H register accessor: an alias for `Reg<LUT591H_SPEC>`"]
pub type LUT591H = crate::Reg<lut591h::LUT591H_SPEC>;
#[doc = "Graphic MMU LUT entry 591 high"]
pub mod lut591h;
#[doc = "LUT592H register accessor: an alias for `Reg<LUT592H_SPEC>`"]
pub type LUT592H = crate::Reg<lut592h::LUT592H_SPEC>;
#[doc = "Graphic MMU LUT entry 592 high"]
pub mod lut592h;
#[doc = "LUT593H register accessor: an alias for `Reg<LUT593H_SPEC>`"]
pub type LUT593H = crate::Reg<lut593h::LUT593H_SPEC>;
#[doc = "Graphic MMU LUT entry 593 high"]
pub mod lut593h;
#[doc = "LUT594H register accessor: an alias for `Reg<LUT594H_SPEC>`"]
pub type LUT594H = crate::Reg<lut594h::LUT594H_SPEC>;
#[doc = "Graphic MMU LUT entry 594 high"]
pub mod lut594h;
#[doc = "LUT595H register accessor: an alias for `Reg<LUT595H_SPEC>`"]
pub type LUT595H = crate::Reg<lut595h::LUT595H_SPEC>;
#[doc = "Graphic MMU LUT entry 595 high"]
pub mod lut595h;
#[doc = "LUT596H register accessor: an alias for `Reg<LUT596H_SPEC>`"]
pub type LUT596H = crate::Reg<lut596h::LUT596H_SPEC>;
#[doc = "Graphic MMU LUT entry 596 high"]
pub mod lut596h;
#[doc = "LUT597H register accessor: an alias for `Reg<LUT597H_SPEC>`"]
pub type LUT597H = crate::Reg<lut597h::LUT597H_SPEC>;
#[doc = "Graphic MMU LUT entry 597 high"]
pub mod lut597h;
#[doc = "LUT598H register accessor: an alias for `Reg<LUT598H_SPEC>`"]
pub type LUT598H = crate::Reg<lut598h::LUT598H_SPEC>;
#[doc = "Graphic MMU LUT entry 598 high"]
pub mod lut598h;
#[doc = "LUT599H register accessor: an alias for `Reg<LUT599H_SPEC>`"]
pub type LUT599H = crate::Reg<lut599h::LUT599H_SPEC>;
#[doc = "Graphic MMU LUT entry 599 high"]
pub mod lut599h;
#[doc = "LUT600H register accessor: an alias for `Reg<LUT600H_SPEC>`"]
pub type LUT600H = crate::Reg<lut600h::LUT600H_SPEC>;
#[doc = "Graphic MMU LUT entry 600 high"]
pub mod lut600h;
#[doc = "LUT601H register accessor: an alias for `Reg<LUT601H_SPEC>`"]
pub type LUT601H = crate::Reg<lut601h::LUT601H_SPEC>;
#[doc = "Graphic MMU LUT entry 601 high"]
pub mod lut601h;
#[doc = "LUT602H register accessor: an alias for `Reg<LUT602H_SPEC>`"]
pub type LUT602H = crate::Reg<lut602h::LUT602H_SPEC>;
#[doc = "Graphic MMU LUT entry 602 high"]
pub mod lut602h;
#[doc = "LUT603H register accessor: an alias for `Reg<LUT603H_SPEC>`"]
pub type LUT603H = crate::Reg<lut603h::LUT603H_SPEC>;
#[doc = "Graphic MMU LUT entry 603 high"]
pub mod lut603h;
#[doc = "LUT604H register accessor: an alias for `Reg<LUT604H_SPEC>`"]
pub type LUT604H = crate::Reg<lut604h::LUT604H_SPEC>;
#[doc = "Graphic MMU LUT entry 604 high"]
pub mod lut604h;
#[doc = "LUT605H register accessor: an alias for `Reg<LUT605H_SPEC>`"]
pub type LUT605H = crate::Reg<lut605h::LUT605H_SPEC>;
#[doc = "Graphic MMU LUT entry 605 high"]
pub mod lut605h;
#[doc = "LUT606H register accessor: an alias for `Reg<LUT606H_SPEC>`"]
pub type LUT606H = crate::Reg<lut606h::LUT606H_SPEC>;
#[doc = "Graphic MMU LUT entry 606 high"]
pub mod lut606h;
#[doc = "LUT607H register accessor: an alias for `Reg<LUT607H_SPEC>`"]
pub type LUT607H = crate::Reg<lut607h::LUT607H_SPEC>;
#[doc = "Graphic MMU LUT entry 607 high"]
pub mod lut607h;
#[doc = "LUT608H register accessor: an alias for `Reg<LUT608H_SPEC>`"]
pub type LUT608H = crate::Reg<lut608h::LUT608H_SPEC>;
#[doc = "Graphic MMU LUT entry 608 high"]
pub mod lut608h;
#[doc = "LUT609H register accessor: an alias for `Reg<LUT609H_SPEC>`"]
pub type LUT609H = crate::Reg<lut609h::LUT609H_SPEC>;
#[doc = "Graphic MMU LUT entry 609 high"]
pub mod lut609h;
#[doc = "LUT610H register accessor: an alias for `Reg<LUT610H_SPEC>`"]
pub type LUT610H = crate::Reg<lut610h::LUT610H_SPEC>;
#[doc = "Graphic MMU LUT entry 610 high"]
pub mod lut610h;
#[doc = "LUT611H register accessor: an alias for `Reg<LUT611H_SPEC>`"]
pub type LUT611H = crate::Reg<lut611h::LUT611H_SPEC>;
#[doc = "Graphic MMU LUT entry 611 high"]
pub mod lut611h;
#[doc = "LUT612H register accessor: an alias for `Reg<LUT612H_SPEC>`"]
pub type LUT612H = crate::Reg<lut612h::LUT612H_SPEC>;
#[doc = "Graphic MMU LUT entry 612 high"]
pub mod lut612h;
#[doc = "LUT613H register accessor: an alias for `Reg<LUT613H_SPEC>`"]
pub type LUT613H = crate::Reg<lut613h::LUT613H_SPEC>;
#[doc = "Graphic MMU LUT entry 613 high"]
pub mod lut613h;
#[doc = "LUT614H register accessor: an alias for `Reg<LUT614H_SPEC>`"]
pub type LUT614H = crate::Reg<lut614h::LUT614H_SPEC>;
#[doc = "Graphic MMU LUT entry 614 high"]
pub mod lut614h;
#[doc = "LUT615H register accessor: an alias for `Reg<LUT615H_SPEC>`"]
pub type LUT615H = crate::Reg<lut615h::LUT615H_SPEC>;
#[doc = "Graphic MMU LUT entry 615 high"]
pub mod lut615h;
#[doc = "LUT616H register accessor: an alias for `Reg<LUT616H_SPEC>`"]
pub type LUT616H = crate::Reg<lut616h::LUT616H_SPEC>;
#[doc = "Graphic MMU LUT entry 616 high"]
pub mod lut616h;
#[doc = "LUT617H register accessor: an alias for `Reg<LUT617H_SPEC>`"]
pub type LUT617H = crate::Reg<lut617h::LUT617H_SPEC>;
#[doc = "Graphic MMU LUT entry 617 high"]
pub mod lut617h;
#[doc = "LUT618H register accessor: an alias for `Reg<LUT618H_SPEC>`"]
pub type LUT618H = crate::Reg<lut618h::LUT618H_SPEC>;
#[doc = "Graphic MMU LUT entry 618 high"]
pub mod lut618h;
#[doc = "LUT619H register accessor: an alias for `Reg<LUT619H_SPEC>`"]
pub type LUT619H = crate::Reg<lut619h::LUT619H_SPEC>;
#[doc = "Graphic MMU LUT entry 619 high"]
pub mod lut619h;
#[doc = "LUT620H register accessor: an alias for `Reg<LUT620H_SPEC>`"]
pub type LUT620H = crate::Reg<lut620h::LUT620H_SPEC>;
#[doc = "Graphic MMU LUT entry 620 high"]
pub mod lut620h;
#[doc = "LUT621H register accessor: an alias for `Reg<LUT621H_SPEC>`"]
pub type LUT621H = crate::Reg<lut621h::LUT621H_SPEC>;
#[doc = "Graphic MMU LUT entry 621 high"]
pub mod lut621h;
#[doc = "LUT622H register accessor: an alias for `Reg<LUT622H_SPEC>`"]
pub type LUT622H = crate::Reg<lut622h::LUT622H_SPEC>;
#[doc = "Graphic MMU LUT entry 622 high"]
pub mod lut622h;
#[doc = "LUT623H register accessor: an alias for `Reg<LUT623H_SPEC>`"]
pub type LUT623H = crate::Reg<lut623h::LUT623H_SPEC>;
#[doc = "Graphic MMU LUT entry 623 high"]
pub mod lut623h;
#[doc = "LUT624H register accessor: an alias for `Reg<LUT624H_SPEC>`"]
pub type LUT624H = crate::Reg<lut624h::LUT624H_SPEC>;
#[doc = "Graphic MMU LUT entry 624 high"]
pub mod lut624h;
#[doc = "LUT625H register accessor: an alias for `Reg<LUT625H_SPEC>`"]
pub type LUT625H = crate::Reg<lut625h::LUT625H_SPEC>;
#[doc = "Graphic MMU LUT entry 625 high"]
pub mod lut625h;
#[doc = "LUT626H register accessor: an alias for `Reg<LUT626H_SPEC>`"]
pub type LUT626H = crate::Reg<lut626h::LUT626H_SPEC>;
#[doc = "Graphic MMU LUT entry 626 high"]
pub mod lut626h;
#[doc = "LUT627H register accessor: an alias for `Reg<LUT627H_SPEC>`"]
pub type LUT627H = crate::Reg<lut627h::LUT627H_SPEC>;
#[doc = "Graphic MMU LUT entry 627 high"]
pub mod lut627h;
#[doc = "LUT628H register accessor: an alias for `Reg<LUT628H_SPEC>`"]
pub type LUT628H = crate::Reg<lut628h::LUT628H_SPEC>;
#[doc = "Graphic MMU LUT entry 628 high"]
pub mod lut628h;
#[doc = "LUT629H register accessor: an alias for `Reg<LUT629H_SPEC>`"]
pub type LUT629H = crate::Reg<lut629h::LUT629H_SPEC>;
#[doc = "Graphic MMU LUT entry 629 high"]
pub mod lut629h;
#[doc = "LUT630H register accessor: an alias for `Reg<LUT630H_SPEC>`"]
pub type LUT630H = crate::Reg<lut630h::LUT630H_SPEC>;
#[doc = "Graphic MMU LUT entry 630 high"]
pub mod lut630h;
#[doc = "LUT631H register accessor: an alias for `Reg<LUT631H_SPEC>`"]
pub type LUT631H = crate::Reg<lut631h::LUT631H_SPEC>;
#[doc = "Graphic MMU LUT entry 631 high"]
pub mod lut631h;
#[doc = "LUT632H register accessor: an alias for `Reg<LUT632H_SPEC>`"]
pub type LUT632H = crate::Reg<lut632h::LUT632H_SPEC>;
#[doc = "Graphic MMU LUT entry 632 high"]
pub mod lut632h;
#[doc = "LUT633H register accessor: an alias for `Reg<LUT633H_SPEC>`"]
pub type LUT633H = crate::Reg<lut633h::LUT633H_SPEC>;
#[doc = "Graphic MMU LUT entry 633 high"]
pub mod lut633h;
#[doc = "LUT634H register accessor: an alias for `Reg<LUT634H_SPEC>`"]
pub type LUT634H = crate::Reg<lut634h::LUT634H_SPEC>;
#[doc = "Graphic MMU LUT entry 634 high"]
pub mod lut634h;
#[doc = "LUT635H register accessor: an alias for `Reg<LUT635H_SPEC>`"]
pub type LUT635H = crate::Reg<lut635h::LUT635H_SPEC>;
#[doc = "Graphic MMU LUT entry 635 high"]
pub mod lut635h;
#[doc = "LUT636H register accessor: an alias for `Reg<LUT636H_SPEC>`"]
pub type LUT636H = crate::Reg<lut636h::LUT636H_SPEC>;
#[doc = "Graphic MMU LUT entry 636 high"]
pub mod lut636h;
#[doc = "LUT637H register accessor: an alias for `Reg<LUT637H_SPEC>`"]
pub type LUT637H = crate::Reg<lut637h::LUT637H_SPEC>;
#[doc = "Graphic MMU LUT entry 637 high"]
pub mod lut637h;
#[doc = "LUT638H register accessor: an alias for `Reg<LUT638H_SPEC>`"]
pub type LUT638H = crate::Reg<lut638h::LUT638H_SPEC>;
#[doc = "Graphic MMU LUT entry 638 high"]
pub mod lut638h;
#[doc = "LUT639H register accessor: an alias for `Reg<LUT639H_SPEC>`"]
pub type LUT639H = crate::Reg<lut639h::LUT639H_SPEC>;
#[doc = "Graphic MMU LUT entry 639 high"]
pub mod lut639h;
#[doc = "LUT640H register accessor: an alias for `Reg<LUT640H_SPEC>`"]
pub type LUT640H = crate::Reg<lut640h::LUT640H_SPEC>;
#[doc = "Graphic MMU LUT entry 640 high"]
pub mod lut640h;
#[doc = "LUT641H register accessor: an alias for `Reg<LUT641H_SPEC>`"]
pub type LUT641H = crate::Reg<lut641h::LUT641H_SPEC>;
#[doc = "Graphic MMU LUT entry 641 high"]
pub mod lut641h;
#[doc = "LUT642H register accessor: an alias for `Reg<LUT642H_SPEC>`"]
pub type LUT642H = crate::Reg<lut642h::LUT642H_SPEC>;
#[doc = "Graphic MMU LUT entry 642 high"]
pub mod lut642h;
#[doc = "LUT643H register accessor: an alias for `Reg<LUT643H_SPEC>`"]
pub type LUT643H = crate::Reg<lut643h::LUT643H_SPEC>;
#[doc = "Graphic MMU LUT entry 643 high"]
pub mod lut643h;
#[doc = "LUT644H register accessor: an alias for `Reg<LUT644H_SPEC>`"]
pub type LUT644H = crate::Reg<lut644h::LUT644H_SPEC>;
#[doc = "Graphic MMU LUT entry 644 high"]
pub mod lut644h;
#[doc = "LUT645H register accessor: an alias for `Reg<LUT645H_SPEC>`"]
pub type LUT645H = crate::Reg<lut645h::LUT645H_SPEC>;
#[doc = "Graphic MMU LUT entry 645 high"]
pub mod lut645h;
#[doc = "LUT646H register accessor: an alias for `Reg<LUT646H_SPEC>`"]
pub type LUT646H = crate::Reg<lut646h::LUT646H_SPEC>;
#[doc = "Graphic MMU LUT entry 646 high"]
pub mod lut646h;
#[doc = "LUT647H register accessor: an alias for `Reg<LUT647H_SPEC>`"]
pub type LUT647H = crate::Reg<lut647h::LUT647H_SPEC>;
#[doc = "Graphic MMU LUT entry 647 high"]
pub mod lut647h;
#[doc = "LUT648H register accessor: an alias for `Reg<LUT648H_SPEC>`"]
pub type LUT648H = crate::Reg<lut648h::LUT648H_SPEC>;
#[doc = "Graphic MMU LUT entry 648 high"]
pub mod lut648h;
#[doc = "LUT649H register accessor: an alias for `Reg<LUT649H_SPEC>`"]
pub type LUT649H = crate::Reg<lut649h::LUT649H_SPEC>;
#[doc = "Graphic MMU LUT entry 649 high"]
pub mod lut649h;
#[doc = "LUT650H register accessor: an alias for `Reg<LUT650H_SPEC>`"]
pub type LUT650H = crate::Reg<lut650h::LUT650H_SPEC>;
#[doc = "Graphic MMU LUT entry 650 high"]
pub mod lut650h;
#[doc = "LUT651H register accessor: an alias for `Reg<LUT651H_SPEC>`"]
pub type LUT651H = crate::Reg<lut651h::LUT651H_SPEC>;
#[doc = "Graphic MMU LUT entry 651 high"]
pub mod lut651h;
#[doc = "LUT652H register accessor: an alias for `Reg<LUT652H_SPEC>`"]
pub type LUT652H = crate::Reg<lut652h::LUT652H_SPEC>;
#[doc = "Graphic MMU LUT entry 652 high"]
pub mod lut652h;
#[doc = "LUT653H register accessor: an alias for `Reg<LUT653H_SPEC>`"]
pub type LUT653H = crate::Reg<lut653h::LUT653H_SPEC>;
#[doc = "Graphic MMU LUT entry 653 high"]
pub mod lut653h;
#[doc = "LUT654H register accessor: an alias for `Reg<LUT654H_SPEC>`"]
pub type LUT654H = crate::Reg<lut654h::LUT654H_SPEC>;
#[doc = "Graphic MMU LUT entry 654 high"]
pub mod lut654h;
#[doc = "LUT655H register accessor: an alias for `Reg<LUT655H_SPEC>`"]
pub type LUT655H = crate::Reg<lut655h::LUT655H_SPEC>;
#[doc = "Graphic MMU LUT entry 655 high"]
pub mod lut655h;
#[doc = "LUT656H register accessor: an alias for `Reg<LUT656H_SPEC>`"]
pub type LUT656H = crate::Reg<lut656h::LUT656H_SPEC>;
#[doc = "Graphic MMU LUT entry 656 high"]
pub mod lut656h;
#[doc = "LUT657H register accessor: an alias for `Reg<LUT657H_SPEC>`"]
pub type LUT657H = crate::Reg<lut657h::LUT657H_SPEC>;
#[doc = "Graphic MMU LUT entry 657 high"]
pub mod lut657h;
#[doc = "LUT658H register accessor: an alias for `Reg<LUT658H_SPEC>`"]
pub type LUT658H = crate::Reg<lut658h::LUT658H_SPEC>;
#[doc = "Graphic MMU LUT entry 658 high"]
pub mod lut658h;
#[doc = "LUT659H register accessor: an alias for `Reg<LUT659H_SPEC>`"]
pub type LUT659H = crate::Reg<lut659h::LUT659H_SPEC>;
#[doc = "Graphic MMU LUT entry 659 high"]
pub mod lut659h;
#[doc = "LUT660H register accessor: an alias for `Reg<LUT660H_SPEC>`"]
pub type LUT660H = crate::Reg<lut660h::LUT660H_SPEC>;
#[doc = "Graphic MMU LUT entry 660 high"]
pub mod lut660h;
#[doc = "LUT661H register accessor: an alias for `Reg<LUT661H_SPEC>`"]
pub type LUT661H = crate::Reg<lut661h::LUT661H_SPEC>;
#[doc = "Graphic MMU LUT entry 661 high"]
pub mod lut661h;
#[doc = "LUT662H register accessor: an alias for `Reg<LUT662H_SPEC>`"]
pub type LUT662H = crate::Reg<lut662h::LUT662H_SPEC>;
#[doc = "Graphic MMU LUT entry 662 high"]
pub mod lut662h;
#[doc = "LUT663H register accessor: an alias for `Reg<LUT663H_SPEC>`"]
pub type LUT663H = crate::Reg<lut663h::LUT663H_SPEC>;
#[doc = "Graphic MMU LUT entry 663 high"]
pub mod lut663h;
#[doc = "LUT664H register accessor: an alias for `Reg<LUT664H_SPEC>`"]
pub type LUT664H = crate::Reg<lut664h::LUT664H_SPEC>;
#[doc = "Graphic MMU LUT entry 664 high"]
pub mod lut664h;
#[doc = "LUT665H register accessor: an alias for `Reg<LUT665H_SPEC>`"]
pub type LUT665H = crate::Reg<lut665h::LUT665H_SPEC>;
#[doc = "Graphic MMU LUT entry 665 high"]
pub mod lut665h;
#[doc = "LUT666H register accessor: an alias for `Reg<LUT666H_SPEC>`"]
pub type LUT666H = crate::Reg<lut666h::LUT666H_SPEC>;
#[doc = "Graphic MMU LUT entry 666 high"]
pub mod lut666h;
#[doc = "LUT667H register accessor: an alias for `Reg<LUT667H_SPEC>`"]
pub type LUT667H = crate::Reg<lut667h::LUT667H_SPEC>;
#[doc = "Graphic MMU LUT entry 667 high"]
pub mod lut667h;
#[doc = "LUT668H register accessor: an alias for `Reg<LUT668H_SPEC>`"]
pub type LUT668H = crate::Reg<lut668h::LUT668H_SPEC>;
#[doc = "Graphic MMU LUT entry 668 high"]
pub mod lut668h;
#[doc = "LUT669H register accessor: an alias for `Reg<LUT669H_SPEC>`"]
pub type LUT669H = crate::Reg<lut669h::LUT669H_SPEC>;
#[doc = "Graphic MMU LUT entry 669 high"]
pub mod lut669h;
#[doc = "LUT670H register accessor: an alias for `Reg<LUT670H_SPEC>`"]
pub type LUT670H = crate::Reg<lut670h::LUT670H_SPEC>;
#[doc = "Graphic MMU LUT entry 670 high"]
pub mod lut670h;
#[doc = "LUT671H register accessor: an alias for `Reg<LUT671H_SPEC>`"]
pub type LUT671H = crate::Reg<lut671h::LUT671H_SPEC>;
#[doc = "Graphic MMU LUT entry 671 high"]
pub mod lut671h;
#[doc = "LUT672H register accessor: an alias for `Reg<LUT672H_SPEC>`"]
pub type LUT672H = crate::Reg<lut672h::LUT672H_SPEC>;
#[doc = "Graphic MMU LUT entry 672 high"]
pub mod lut672h;
#[doc = "LUT673H register accessor: an alias for `Reg<LUT673H_SPEC>`"]
pub type LUT673H = crate::Reg<lut673h::LUT673H_SPEC>;
#[doc = "Graphic MMU LUT entry 673 high"]
pub mod lut673h;
#[doc = "LUT674H register accessor: an alias for `Reg<LUT674H_SPEC>`"]
pub type LUT674H = crate::Reg<lut674h::LUT674H_SPEC>;
#[doc = "Graphic MMU LUT entry 674 high"]
pub mod lut674h;
#[doc = "LUT675H register accessor: an alias for `Reg<LUT675H_SPEC>`"]
pub type LUT675H = crate::Reg<lut675h::LUT675H_SPEC>;
#[doc = "Graphic MMU LUT entry 675 high"]
pub mod lut675h;
#[doc = "LUT676H register accessor: an alias for `Reg<LUT676H_SPEC>`"]
pub type LUT676H = crate::Reg<lut676h::LUT676H_SPEC>;
#[doc = "Graphic MMU LUT entry 676 high"]
pub mod lut676h;
#[doc = "LUT677H register accessor: an alias for `Reg<LUT677H_SPEC>`"]
pub type LUT677H = crate::Reg<lut677h::LUT677H_SPEC>;
#[doc = "Graphic MMU LUT entry 677 high"]
pub mod lut677h;
#[doc = "LUT678H register accessor: an alias for `Reg<LUT678H_SPEC>`"]
pub type LUT678H = crate::Reg<lut678h::LUT678H_SPEC>;
#[doc = "Graphic MMU LUT entry 678 high"]
pub mod lut678h;
#[doc = "LUT679H register accessor: an alias for `Reg<LUT679H_SPEC>`"]
pub type LUT679H = crate::Reg<lut679h::LUT679H_SPEC>;
#[doc = "Graphic MMU LUT entry 679 high"]
pub mod lut679h;
#[doc = "LUT680H register accessor: an alias for `Reg<LUT680H_SPEC>`"]
pub type LUT680H = crate::Reg<lut680h::LUT680H_SPEC>;
#[doc = "Graphic MMU LUT entry 680 high"]
pub mod lut680h;
#[doc = "LUT681H register accessor: an alias for `Reg<LUT681H_SPEC>`"]
pub type LUT681H = crate::Reg<lut681h::LUT681H_SPEC>;
#[doc = "Graphic MMU LUT entry 681 high"]
pub mod lut681h;
#[doc = "LUT682H register accessor: an alias for `Reg<LUT682H_SPEC>`"]
pub type LUT682H = crate::Reg<lut682h::LUT682H_SPEC>;
#[doc = "Graphic MMU LUT entry 682 high"]
pub mod lut682h;
#[doc = "LUT683H register accessor: an alias for `Reg<LUT683H_SPEC>`"]
pub type LUT683H = crate::Reg<lut683h::LUT683H_SPEC>;
#[doc = "Graphic MMU LUT entry 683 high"]
pub mod lut683h;
#[doc = "LUT684H register accessor: an alias for `Reg<LUT684H_SPEC>`"]
pub type LUT684H = crate::Reg<lut684h::LUT684H_SPEC>;
#[doc = "Graphic MMU LUT entry 684 high"]
pub mod lut684h;
#[doc = "LUT685H register accessor: an alias for `Reg<LUT685H_SPEC>`"]
pub type LUT685H = crate::Reg<lut685h::LUT685H_SPEC>;
#[doc = "Graphic MMU LUT entry 685 high"]
pub mod lut685h;
#[doc = "LUT686H register accessor: an alias for `Reg<LUT686H_SPEC>`"]
pub type LUT686H = crate::Reg<lut686h::LUT686H_SPEC>;
#[doc = "Graphic MMU LUT entry 686 high"]
pub mod lut686h;
#[doc = "LUT687H register accessor: an alias for `Reg<LUT687H_SPEC>`"]
pub type LUT687H = crate::Reg<lut687h::LUT687H_SPEC>;
#[doc = "Graphic MMU LUT entry 687 high"]
pub mod lut687h;
#[doc = "LUT688H register accessor: an alias for `Reg<LUT688H_SPEC>`"]
pub type LUT688H = crate::Reg<lut688h::LUT688H_SPEC>;
#[doc = "Graphic MMU LUT entry 688 high"]
pub mod lut688h;
#[doc = "LUT689H register accessor: an alias for `Reg<LUT689H_SPEC>`"]
pub type LUT689H = crate::Reg<lut689h::LUT689H_SPEC>;
#[doc = "Graphic MMU LUT entry 689 high"]
pub mod lut689h;
#[doc = "LUT690H register accessor: an alias for `Reg<LUT690H_SPEC>`"]
pub type LUT690H = crate::Reg<lut690h::LUT690H_SPEC>;
#[doc = "Graphic MMU LUT entry 690 high"]
pub mod lut690h;
#[doc = "LUT691H register accessor: an alias for `Reg<LUT691H_SPEC>`"]
pub type LUT691H = crate::Reg<lut691h::LUT691H_SPEC>;
#[doc = "Graphic MMU LUT entry 691 high"]
pub mod lut691h;
#[doc = "LUT692H register accessor: an alias for `Reg<LUT692H_SPEC>`"]
pub type LUT692H = crate::Reg<lut692h::LUT692H_SPEC>;
#[doc = "Graphic MMU LUT entry 692 high"]
pub mod lut692h;
#[doc = "LUT693H register accessor: an alias for `Reg<LUT693H_SPEC>`"]
pub type LUT693H = crate::Reg<lut693h::LUT693H_SPEC>;
#[doc = "Graphic MMU LUT entry 693 high"]
pub mod lut693h;
#[doc = "LUT694H register accessor: an alias for `Reg<LUT694H_SPEC>`"]
pub type LUT694H = crate::Reg<lut694h::LUT694H_SPEC>;
#[doc = "Graphic MMU LUT entry 694 high"]
pub mod lut694h;
#[doc = "LUT695H register accessor: an alias for `Reg<LUT695H_SPEC>`"]
pub type LUT695H = crate::Reg<lut695h::LUT695H_SPEC>;
#[doc = "Graphic MMU LUT entry 695 high"]
pub mod lut695h;
#[doc = "LUT696H register accessor: an alias for `Reg<LUT696H_SPEC>`"]
pub type LUT696H = crate::Reg<lut696h::LUT696H_SPEC>;
#[doc = "Graphic MMU LUT entry 696 high"]
pub mod lut696h;
#[doc = "LUT697H register accessor: an alias for `Reg<LUT697H_SPEC>`"]
pub type LUT697H = crate::Reg<lut697h::LUT697H_SPEC>;
#[doc = "Graphic MMU LUT entry 697 high"]
pub mod lut697h;
#[doc = "LUT698H register accessor: an alias for `Reg<LUT698H_SPEC>`"]
pub type LUT698H = crate::Reg<lut698h::LUT698H_SPEC>;
#[doc = "Graphic MMU LUT entry 698 high"]
pub mod lut698h;
#[doc = "LUT699H register accessor: an alias for `Reg<LUT699H_SPEC>`"]
pub type LUT699H = crate::Reg<lut699h::LUT699H_SPEC>;
#[doc = "Graphic MMU LUT entry 699 high"]
pub mod lut699h;
#[doc = "LUT700H register accessor: an alias for `Reg<LUT700H_SPEC>`"]
pub type LUT700H = crate::Reg<lut700h::LUT700H_SPEC>;
#[doc = "Graphic MMU LUT entry 700 high"]
pub mod lut700h;
#[doc = "LUT701H register accessor: an alias for `Reg<LUT701H_SPEC>`"]
pub type LUT701H = crate::Reg<lut701h::LUT701H_SPEC>;
#[doc = "Graphic MMU LUT entry 701 high"]
pub mod lut701h;
#[doc = "LUT702H register accessor: an alias for `Reg<LUT702H_SPEC>`"]
pub type LUT702H = crate::Reg<lut702h::LUT702H_SPEC>;
#[doc = "Graphic MMU LUT entry 702 high"]
pub mod lut702h;
#[doc = "LUT703H register accessor: an alias for `Reg<LUT703H_SPEC>`"]
pub type LUT703H = crate::Reg<lut703h::LUT703H_SPEC>;
#[doc = "Graphic MMU LUT entry 703 high"]
pub mod lut703h;
#[doc = "LUT704H register accessor: an alias for `Reg<LUT704H_SPEC>`"]
pub type LUT704H = crate::Reg<lut704h::LUT704H_SPEC>;
#[doc = "Graphic MMU LUT entry 704 high"]
pub mod lut704h;
#[doc = "LUT705H register accessor: an alias for `Reg<LUT705H_SPEC>`"]
pub type LUT705H = crate::Reg<lut705h::LUT705H_SPEC>;
#[doc = "Graphic MMU LUT entry 705 high"]
pub mod lut705h;
#[doc = "LUT706H register accessor: an alias for `Reg<LUT706H_SPEC>`"]
pub type LUT706H = crate::Reg<lut706h::LUT706H_SPEC>;
#[doc = "Graphic MMU LUT entry 706 high"]
pub mod lut706h;
#[doc = "LUT707H register accessor: an alias for `Reg<LUT707H_SPEC>`"]
pub type LUT707H = crate::Reg<lut707h::LUT707H_SPEC>;
#[doc = "Graphic MMU LUT entry 707 high"]
pub mod lut707h;
#[doc = "LUT708H register accessor: an alias for `Reg<LUT708H_SPEC>`"]
pub type LUT708H = crate::Reg<lut708h::LUT708H_SPEC>;
#[doc = "Graphic MMU LUT entry 708 high"]
pub mod lut708h;
#[doc = "LUT709H register accessor: an alias for `Reg<LUT709H_SPEC>`"]
pub type LUT709H = crate::Reg<lut709h::LUT709H_SPEC>;
#[doc = "Graphic MMU LUT entry 709 high"]
pub mod lut709h;
#[doc = "LUT710H register accessor: an alias for `Reg<LUT710H_SPEC>`"]
pub type LUT710H = crate::Reg<lut710h::LUT710H_SPEC>;
#[doc = "Graphic MMU LUT entry 710 high"]
pub mod lut710h;
#[doc = "LUT711H register accessor: an alias for `Reg<LUT711H_SPEC>`"]
pub type LUT711H = crate::Reg<lut711h::LUT711H_SPEC>;
#[doc = "Graphic MMU LUT entry 711 high"]
pub mod lut711h;
#[doc = "LUT712H register accessor: an alias for `Reg<LUT712H_SPEC>`"]
pub type LUT712H = crate::Reg<lut712h::LUT712H_SPEC>;
#[doc = "Graphic MMU LUT entry 712 high"]
pub mod lut712h;
#[doc = "LUT713H register accessor: an alias for `Reg<LUT713H_SPEC>`"]
pub type LUT713H = crate::Reg<lut713h::LUT713H_SPEC>;
#[doc = "Graphic MMU LUT entry 713 high"]
pub mod lut713h;
#[doc = "LUT714H register accessor: an alias for `Reg<LUT714H_SPEC>`"]
pub type LUT714H = crate::Reg<lut714h::LUT714H_SPEC>;
#[doc = "Graphic MMU LUT entry 714 high"]
pub mod lut714h;
#[doc = "LUT715H register accessor: an alias for `Reg<LUT715H_SPEC>`"]
pub type LUT715H = crate::Reg<lut715h::LUT715H_SPEC>;
#[doc = "Graphic MMU LUT entry 715 high"]
pub mod lut715h;
#[doc = "LUT716H register accessor: an alias for `Reg<LUT716H_SPEC>`"]
pub type LUT716H = crate::Reg<lut716h::LUT716H_SPEC>;
#[doc = "Graphic MMU LUT entry 716 high"]
pub mod lut716h;
#[doc = "LUT717H register accessor: an alias for `Reg<LUT717H_SPEC>`"]
pub type LUT717H = crate::Reg<lut717h::LUT717H_SPEC>;
#[doc = "Graphic MMU LUT entry 717 high"]
pub mod lut717h;
#[doc = "LUT718H register accessor: an alias for `Reg<LUT718H_SPEC>`"]
pub type LUT718H = crate::Reg<lut718h::LUT718H_SPEC>;
#[doc = "Graphic MMU LUT entry 718 high"]
pub mod lut718h;
#[doc = "LUT719H register accessor: an alias for `Reg<LUT719H_SPEC>`"]
pub type LUT719H = crate::Reg<lut719h::LUT719H_SPEC>;
#[doc = "Graphic MMU LUT entry 719 high"]
pub mod lut719h;
#[doc = "LUT720H register accessor: an alias for `Reg<LUT720H_SPEC>`"]
pub type LUT720H = crate::Reg<lut720h::LUT720H_SPEC>;
#[doc = "Graphic MMU LUT entry 720 high"]
pub mod lut720h;
#[doc = "LUT721H register accessor: an alias for `Reg<LUT721H_SPEC>`"]
pub type LUT721H = crate::Reg<lut721h::LUT721H_SPEC>;
#[doc = "Graphic MMU LUT entry 721 high"]
pub mod lut721h;
#[doc = "LUT722H register accessor: an alias for `Reg<LUT722H_SPEC>`"]
pub type LUT722H = crate::Reg<lut722h::LUT722H_SPEC>;
#[doc = "Graphic MMU LUT entry 722 high"]
pub mod lut722h;
#[doc = "LUT723H register accessor: an alias for `Reg<LUT723H_SPEC>`"]
pub type LUT723H = crate::Reg<lut723h::LUT723H_SPEC>;
#[doc = "Graphic MMU LUT entry 723 high"]
pub mod lut723h;
#[doc = "LUT724H register accessor: an alias for `Reg<LUT724H_SPEC>`"]
pub type LUT724H = crate::Reg<lut724h::LUT724H_SPEC>;
#[doc = "Graphic MMU LUT entry 724 high"]
pub mod lut724h;
#[doc = "LUT725H register accessor: an alias for `Reg<LUT725H_SPEC>`"]
pub type LUT725H = crate::Reg<lut725h::LUT725H_SPEC>;
#[doc = "Graphic MMU LUT entry 725 high"]
pub mod lut725h;
#[doc = "LUT726H register accessor: an alias for `Reg<LUT726H_SPEC>`"]
pub type LUT726H = crate::Reg<lut726h::LUT726H_SPEC>;
#[doc = "Graphic MMU LUT entry 726 high"]
pub mod lut726h;
#[doc = "LUT727H register accessor: an alias for `Reg<LUT727H_SPEC>`"]
pub type LUT727H = crate::Reg<lut727h::LUT727H_SPEC>;
#[doc = "Graphic MMU LUT entry 727 high"]
pub mod lut727h;
#[doc = "LUT728H register accessor: an alias for `Reg<LUT728H_SPEC>`"]
pub type LUT728H = crate::Reg<lut728h::LUT728H_SPEC>;
#[doc = "Graphic MMU LUT entry 728 high"]
pub mod lut728h;
#[doc = "LUT729H register accessor: an alias for `Reg<LUT729H_SPEC>`"]
pub type LUT729H = crate::Reg<lut729h::LUT729H_SPEC>;
#[doc = "Graphic MMU LUT entry 729 high"]
pub mod lut729h;
#[doc = "LUT730H register accessor: an alias for `Reg<LUT730H_SPEC>`"]
pub type LUT730H = crate::Reg<lut730h::LUT730H_SPEC>;
#[doc = "Graphic MMU LUT entry 730 high"]
pub mod lut730h;
#[doc = "LUT731H register accessor: an alias for `Reg<LUT731H_SPEC>`"]
pub type LUT731H = crate::Reg<lut731h::LUT731H_SPEC>;
#[doc = "Graphic MMU LUT entry 731 high"]
pub mod lut731h;
#[doc = "LUT732H register accessor: an alias for `Reg<LUT732H_SPEC>`"]
pub type LUT732H = crate::Reg<lut732h::LUT732H_SPEC>;
#[doc = "Graphic MMU LUT entry 732 high"]
pub mod lut732h;
#[doc = "LUT733H register accessor: an alias for `Reg<LUT733H_SPEC>`"]
pub type LUT733H = crate::Reg<lut733h::LUT733H_SPEC>;
#[doc = "Graphic MMU LUT entry 733 high"]
pub mod lut733h;
#[doc = "LUT734H register accessor: an alias for `Reg<LUT734H_SPEC>`"]
pub type LUT734H = crate::Reg<lut734h::LUT734H_SPEC>;
#[doc = "Graphic MMU LUT entry 734 high"]
pub mod lut734h;
#[doc = "LUT735H register accessor: an alias for `Reg<LUT735H_SPEC>`"]
pub type LUT735H = crate::Reg<lut735h::LUT735H_SPEC>;
#[doc = "Graphic MMU LUT entry 735 high"]
pub mod lut735h;
#[doc = "LUT736H register accessor: an alias for `Reg<LUT736H_SPEC>`"]
pub type LUT736H = crate::Reg<lut736h::LUT736H_SPEC>;
#[doc = "Graphic MMU LUT entry 736 high"]
pub mod lut736h;
#[doc = "LUT737H register accessor: an alias for `Reg<LUT737H_SPEC>`"]
pub type LUT737H = crate::Reg<lut737h::LUT737H_SPEC>;
#[doc = "Graphic MMU LUT entry 737 high"]
pub mod lut737h;
#[doc = "LUT738H register accessor: an alias for `Reg<LUT738H_SPEC>`"]
pub type LUT738H = crate::Reg<lut738h::LUT738H_SPEC>;
#[doc = "Graphic MMU LUT entry 738 high"]
pub mod lut738h;
#[doc = "LUT739H register accessor: an alias for `Reg<LUT739H_SPEC>`"]
pub type LUT739H = crate::Reg<lut739h::LUT739H_SPEC>;
#[doc = "Graphic MMU LUT entry 739 high"]
pub mod lut739h;
#[doc = "LUT740H register accessor: an alias for `Reg<LUT740H_SPEC>`"]
pub type LUT740H = crate::Reg<lut740h::LUT740H_SPEC>;
#[doc = "Graphic MMU LUT entry 740 high"]
pub mod lut740h;
#[doc = "LUT741H register accessor: an alias for `Reg<LUT741H_SPEC>`"]
pub type LUT741H = crate::Reg<lut741h::LUT741H_SPEC>;
#[doc = "Graphic MMU LUT entry 741 high"]
pub mod lut741h;
#[doc = "LUT742H register accessor: an alias for `Reg<LUT742H_SPEC>`"]
pub type LUT742H = crate::Reg<lut742h::LUT742H_SPEC>;
#[doc = "Graphic MMU LUT entry 742 high"]
pub mod lut742h;
#[doc = "LUT743H register accessor: an alias for `Reg<LUT743H_SPEC>`"]
pub type LUT743H = crate::Reg<lut743h::LUT743H_SPEC>;
#[doc = "Graphic MMU LUT entry 743 high"]
pub mod lut743h;
#[doc = "LUT744H register accessor: an alias for `Reg<LUT744H_SPEC>`"]
pub type LUT744H = crate::Reg<lut744h::LUT744H_SPEC>;
#[doc = "Graphic MMU LUT entry 744 high"]
pub mod lut744h;
#[doc = "LUT745H register accessor: an alias for `Reg<LUT745H_SPEC>`"]
pub type LUT745H = crate::Reg<lut745h::LUT745H_SPEC>;
#[doc = "Graphic MMU LUT entry 745 high"]
pub mod lut745h;
#[doc = "LUT746H register accessor: an alias for `Reg<LUT746H_SPEC>`"]
pub type LUT746H = crate::Reg<lut746h::LUT746H_SPEC>;
#[doc = "Graphic MMU LUT entry 746 high"]
pub mod lut746h;
#[doc = "LUT747H register accessor: an alias for `Reg<LUT747H_SPEC>`"]
pub type LUT747H = crate::Reg<lut747h::LUT747H_SPEC>;
#[doc = "Graphic MMU LUT entry 747 high"]
pub mod lut747h;
#[doc = "LUT748H register accessor: an alias for `Reg<LUT748H_SPEC>`"]
pub type LUT748H = crate::Reg<lut748h::LUT748H_SPEC>;
#[doc = "Graphic MMU LUT entry 748 high"]
pub mod lut748h;
#[doc = "LUT749H register accessor: an alias for `Reg<LUT749H_SPEC>`"]
pub type LUT749H = crate::Reg<lut749h::LUT749H_SPEC>;
#[doc = "Graphic MMU LUT entry 749 high"]
pub mod lut749h;
#[doc = "LUT750H register accessor: an alias for `Reg<LUT750H_SPEC>`"]
pub type LUT750H = crate::Reg<lut750h::LUT750H_SPEC>;
#[doc = "Graphic MMU LUT entry 750 high"]
pub mod lut750h;
#[doc = "LUT751H register accessor: an alias for `Reg<LUT751H_SPEC>`"]
pub type LUT751H = crate::Reg<lut751h::LUT751H_SPEC>;
#[doc = "Graphic MMU LUT entry 751 high"]
pub mod lut751h;
#[doc = "LUT752H register accessor: an alias for `Reg<LUT752H_SPEC>`"]
pub type LUT752H = crate::Reg<lut752h::LUT752H_SPEC>;
#[doc = "Graphic MMU LUT entry 752 high"]
pub mod lut752h;
#[doc = "LUT753H register accessor: an alias for `Reg<LUT753H_SPEC>`"]
pub type LUT753H = crate::Reg<lut753h::LUT753H_SPEC>;
#[doc = "Graphic MMU LUT entry 753 high"]
pub mod lut753h;
#[doc = "LUT754H register accessor: an alias for `Reg<LUT754H_SPEC>`"]
pub type LUT754H = crate::Reg<lut754h::LUT754H_SPEC>;
#[doc = "Graphic MMU LUT entry 754 high"]
pub mod lut754h;
#[doc = "LUT755H register accessor: an alias for `Reg<LUT755H_SPEC>`"]
pub type LUT755H = crate::Reg<lut755h::LUT755H_SPEC>;
#[doc = "Graphic MMU LUT entry 755 high"]
pub mod lut755h;
#[doc = "LUT756H register accessor: an alias for `Reg<LUT756H_SPEC>`"]
pub type LUT756H = crate::Reg<lut756h::LUT756H_SPEC>;
#[doc = "Graphic MMU LUT entry 756 high"]
pub mod lut756h;
#[doc = "LUT757H register accessor: an alias for `Reg<LUT757H_SPEC>`"]
pub type LUT757H = crate::Reg<lut757h::LUT757H_SPEC>;
#[doc = "Graphic MMU LUT entry 757 high"]
pub mod lut757h;
#[doc = "LUT758H register accessor: an alias for `Reg<LUT758H_SPEC>`"]
pub type LUT758H = crate::Reg<lut758h::LUT758H_SPEC>;
#[doc = "Graphic MMU LUT entry 758 high"]
pub mod lut758h;
#[doc = "LUT759H register accessor: an alias for `Reg<LUT759H_SPEC>`"]
pub type LUT759H = crate::Reg<lut759h::LUT759H_SPEC>;
#[doc = "Graphic MMU LUT entry 759 high"]
pub mod lut759h;
#[doc = "LUT760H register accessor: an alias for `Reg<LUT760H_SPEC>`"]
pub type LUT760H = crate::Reg<lut760h::LUT760H_SPEC>;
#[doc = "Graphic MMU LUT entry 760 high"]
pub mod lut760h;
#[doc = "LUT761H register accessor: an alias for `Reg<LUT761H_SPEC>`"]
pub type LUT761H = crate::Reg<lut761h::LUT761H_SPEC>;
#[doc = "Graphic MMU LUT entry 761 high"]
pub mod lut761h;
#[doc = "LUT762H register accessor: an alias for `Reg<LUT762H_SPEC>`"]
pub type LUT762H = crate::Reg<lut762h::LUT762H_SPEC>;
#[doc = "Graphic MMU LUT entry 762 high"]
pub mod lut762h;
#[doc = "LUT763H register accessor: an alias for `Reg<LUT763H_SPEC>`"]
pub type LUT763H = crate::Reg<lut763h::LUT763H_SPEC>;
#[doc = "Graphic MMU LUT entry 763 high"]
pub mod lut763h;
#[doc = "LUT764H register accessor: an alias for `Reg<LUT764H_SPEC>`"]
pub type LUT764H = crate::Reg<lut764h::LUT764H_SPEC>;
#[doc = "Graphic MMU LUT entry 764 high"]
pub mod lut764h;
#[doc = "LUT765H register accessor: an alias for `Reg<LUT765H_SPEC>`"]
pub type LUT765H = crate::Reg<lut765h::LUT765H_SPEC>;
#[doc = "Graphic MMU LUT entry 765 high"]
pub mod lut765h;
#[doc = "LUT766H register accessor: an alias for `Reg<LUT766H_SPEC>`"]
pub type LUT766H = crate::Reg<lut766h::LUT766H_SPEC>;
#[doc = "Graphic MMU LUT entry 766 high"]
pub mod lut766h;
#[doc = "LUT767H register accessor: an alias for `Reg<LUT767H_SPEC>`"]
pub type LUT767H = crate::Reg<lut767h::LUT767H_SPEC>;
#[doc = "Graphic MMU LUT entry 767 high"]
pub mod lut767h;
#[doc = "LUT768H register accessor: an alias for `Reg<LUT768H_SPEC>`"]
pub type LUT768H = crate::Reg<lut768h::LUT768H_SPEC>;
#[doc = "Graphic MMU LUT entry 768 high"]
pub mod lut768h;
#[doc = "LUT769H register accessor: an alias for `Reg<LUT769H_SPEC>`"]
pub type LUT769H = crate::Reg<lut769h::LUT769H_SPEC>;
#[doc = "Graphic MMU LUT entry 769 high"]
pub mod lut769h;
#[doc = "LUT770H register accessor: an alias for `Reg<LUT770H_SPEC>`"]
pub type LUT770H = crate::Reg<lut770h::LUT770H_SPEC>;
#[doc = "Graphic MMU LUT entry 770 high"]
pub mod lut770h;
#[doc = "LUT771H register accessor: an alias for `Reg<LUT771H_SPEC>`"]
pub type LUT771H = crate::Reg<lut771h::LUT771H_SPEC>;
#[doc = "Graphic MMU LUT entry 771 high"]
pub mod lut771h;
#[doc = "LUT772H register accessor: an alias for `Reg<LUT772H_SPEC>`"]
pub type LUT772H = crate::Reg<lut772h::LUT772H_SPEC>;
#[doc = "Graphic MMU LUT entry 772 high"]
pub mod lut772h;
#[doc = "LUT773H register accessor: an alias for `Reg<LUT773H_SPEC>`"]
pub type LUT773H = crate::Reg<lut773h::LUT773H_SPEC>;
#[doc = "Graphic MMU LUT entry 773 high"]
pub mod lut773h;
#[doc = "LUT774H register accessor: an alias for `Reg<LUT774H_SPEC>`"]
pub type LUT774H = crate::Reg<lut774h::LUT774H_SPEC>;
#[doc = "Graphic MMU LUT entry 774 high"]
pub mod lut774h;
#[doc = "LUT775H register accessor: an alias for `Reg<LUT775H_SPEC>`"]
pub type LUT775H = crate::Reg<lut775h::LUT775H_SPEC>;
#[doc = "Graphic MMU LUT entry 775 high"]
pub mod lut775h;
#[doc = "LUT776H register accessor: an alias for `Reg<LUT776H_SPEC>`"]
pub type LUT776H = crate::Reg<lut776h::LUT776H_SPEC>;
#[doc = "Graphic MMU LUT entry 776 high"]
pub mod lut776h;
#[doc = "LUT777H register accessor: an alias for `Reg<LUT777H_SPEC>`"]
pub type LUT777H = crate::Reg<lut777h::LUT777H_SPEC>;
#[doc = "Graphic MMU LUT entry 777 high"]
pub mod lut777h;
#[doc = "LUT778H register accessor: an alias for `Reg<LUT778H_SPEC>`"]
pub type LUT778H = crate::Reg<lut778h::LUT778H_SPEC>;
#[doc = "Graphic MMU LUT entry 778 high"]
pub mod lut778h;
#[doc = "LUT779H register accessor: an alias for `Reg<LUT779H_SPEC>`"]
pub type LUT779H = crate::Reg<lut779h::LUT779H_SPEC>;
#[doc = "Graphic MMU LUT entry 779 high"]
pub mod lut779h;
#[doc = "LUT780H register accessor: an alias for `Reg<LUT780H_SPEC>`"]
pub type LUT780H = crate::Reg<lut780h::LUT780H_SPEC>;
#[doc = "Graphic MMU LUT entry 780 high"]
pub mod lut780h;
#[doc = "LUT781H register accessor: an alias for `Reg<LUT781H_SPEC>`"]
pub type LUT781H = crate::Reg<lut781h::LUT781H_SPEC>;
#[doc = "Graphic MMU LUT entry 781 high"]
pub mod lut781h;
#[doc = "LUT782H register accessor: an alias for `Reg<LUT782H_SPEC>`"]
pub type LUT782H = crate::Reg<lut782h::LUT782H_SPEC>;
#[doc = "Graphic MMU LUT entry 782 high"]
pub mod lut782h;
#[doc = "LUT783H register accessor: an alias for `Reg<LUT783H_SPEC>`"]
pub type LUT783H = crate::Reg<lut783h::LUT783H_SPEC>;
#[doc = "Graphic MMU LUT entry 783 high"]
pub mod lut783h;
#[doc = "LUT784H register accessor: an alias for `Reg<LUT784H_SPEC>`"]
pub type LUT784H = crate::Reg<lut784h::LUT784H_SPEC>;
#[doc = "Graphic MMU LUT entry 784 high"]
pub mod lut784h;
#[doc = "LUT785H register accessor: an alias for `Reg<LUT785H_SPEC>`"]
pub type LUT785H = crate::Reg<lut785h::LUT785H_SPEC>;
#[doc = "Graphic MMU LUT entry 785 high"]
pub mod lut785h;
#[doc = "LUT786H register accessor: an alias for `Reg<LUT786H_SPEC>`"]
pub type LUT786H = crate::Reg<lut786h::LUT786H_SPEC>;
#[doc = "Graphic MMU LUT entry 786 high"]
pub mod lut786h;
#[doc = "LUT787H register accessor: an alias for `Reg<LUT787H_SPEC>`"]
pub type LUT787H = crate::Reg<lut787h::LUT787H_SPEC>;
#[doc = "Graphic MMU LUT entry 787 high"]
pub mod lut787h;
#[doc = "LUT788H register accessor: an alias for `Reg<LUT788H_SPEC>`"]
pub type LUT788H = crate::Reg<lut788h::LUT788H_SPEC>;
#[doc = "Graphic MMU LUT entry 788 high"]
pub mod lut788h;
#[doc = "LUT789H register accessor: an alias for `Reg<LUT789H_SPEC>`"]
pub type LUT789H = crate::Reg<lut789h::LUT789H_SPEC>;
#[doc = "Graphic MMU LUT entry 789 high"]
pub mod lut789h;
#[doc = "LUT790H register accessor: an alias for `Reg<LUT790H_SPEC>`"]
pub type LUT790H = crate::Reg<lut790h::LUT790H_SPEC>;
#[doc = "Graphic MMU LUT entry 790 high"]
pub mod lut790h;
#[doc = "LUT791H register accessor: an alias for `Reg<LUT791H_SPEC>`"]
pub type LUT791H = crate::Reg<lut791h::LUT791H_SPEC>;
#[doc = "Graphic MMU LUT entry 791 high"]
pub mod lut791h;
#[doc = "LUT792H register accessor: an alias for `Reg<LUT792H_SPEC>`"]
pub type LUT792H = crate::Reg<lut792h::LUT792H_SPEC>;
#[doc = "Graphic MMU LUT entry 792 high"]
pub mod lut792h;
#[doc = "LUT793H register accessor: an alias for `Reg<LUT793H_SPEC>`"]
pub type LUT793H = crate::Reg<lut793h::LUT793H_SPEC>;
#[doc = "Graphic MMU LUT entry 793 high"]
pub mod lut793h;
#[doc = "LUT794H register accessor: an alias for `Reg<LUT794H_SPEC>`"]
pub type LUT794H = crate::Reg<lut794h::LUT794H_SPEC>;
#[doc = "Graphic MMU LUT entry 794 high"]
pub mod lut794h;
#[doc = "LUT795H register accessor: an alias for `Reg<LUT795H_SPEC>`"]
pub type LUT795H = crate::Reg<lut795h::LUT795H_SPEC>;
#[doc = "Graphic MMU LUT entry 795 high"]
pub mod lut795h;
#[doc = "LUT796H register accessor: an alias for `Reg<LUT796H_SPEC>`"]
pub type LUT796H = crate::Reg<lut796h::LUT796H_SPEC>;
#[doc = "Graphic MMU LUT entry 796 high"]
pub mod lut796h;
#[doc = "LUT797H register accessor: an alias for `Reg<LUT797H_SPEC>`"]
pub type LUT797H = crate::Reg<lut797h::LUT797H_SPEC>;
#[doc = "Graphic MMU LUT entry 797 high"]
pub mod lut797h;
#[doc = "LUT798H register accessor: an alias for `Reg<LUT798H_SPEC>`"]
pub type LUT798H = crate::Reg<lut798h::LUT798H_SPEC>;
#[doc = "Graphic MMU LUT entry 798 high"]
pub mod lut798h;
#[doc = "LUT799H register accessor: an alias for `Reg<LUT799H_SPEC>`"]
pub type LUT799H = crate::Reg<lut799h::LUT799H_SPEC>;
#[doc = "Graphic MMU LUT entry 799 high"]
pub mod lut799h;
#[doc = "LUT800H register accessor: an alias for `Reg<LUT800H_SPEC>`"]
pub type LUT800H = crate::Reg<lut800h::LUT800H_SPEC>;
#[doc = "Graphic MMU LUT entry 800 high"]
pub mod lut800h;
#[doc = "LUT801H register accessor: an alias for `Reg<LUT801H_SPEC>`"]
pub type LUT801H = crate::Reg<lut801h::LUT801H_SPEC>;
#[doc = "Graphic MMU LUT entry 801 high"]
pub mod lut801h;
#[doc = "LUT802H register accessor: an alias for `Reg<LUT802H_SPEC>`"]
pub type LUT802H = crate::Reg<lut802h::LUT802H_SPEC>;
#[doc = "Graphic MMU LUT entry 802 high"]
pub mod lut802h;
#[doc = "LUT803H register accessor: an alias for `Reg<LUT803H_SPEC>`"]
pub type LUT803H = crate::Reg<lut803h::LUT803H_SPEC>;
#[doc = "Graphic MMU LUT entry 803 high"]
pub mod lut803h;
#[doc = "LUT804H register accessor: an alias for `Reg<LUT804H_SPEC>`"]
pub type LUT804H = crate::Reg<lut804h::LUT804H_SPEC>;
#[doc = "Graphic MMU LUT entry 804 high"]
pub mod lut804h;
#[doc = "LUT805H register accessor: an alias for `Reg<LUT805H_SPEC>`"]
pub type LUT805H = crate::Reg<lut805h::LUT805H_SPEC>;
#[doc = "Graphic MMU LUT entry 805 high"]
pub mod lut805h;
#[doc = "LUT806H register accessor: an alias for `Reg<LUT806H_SPEC>`"]
pub type LUT806H = crate::Reg<lut806h::LUT806H_SPEC>;
#[doc = "Graphic MMU LUT entry 806 high"]
pub mod lut806h;
#[doc = "LUT807H register accessor: an alias for `Reg<LUT807H_SPEC>`"]
pub type LUT807H = crate::Reg<lut807h::LUT807H_SPEC>;
#[doc = "Graphic MMU LUT entry 807 high"]
pub mod lut807h;
#[doc = "LUT808H register accessor: an alias for `Reg<LUT808H_SPEC>`"]
pub type LUT808H = crate::Reg<lut808h::LUT808H_SPEC>;
#[doc = "Graphic MMU LUT entry 808 high"]
pub mod lut808h;
#[doc = "LUT809H register accessor: an alias for `Reg<LUT809H_SPEC>`"]
pub type LUT809H = crate::Reg<lut809h::LUT809H_SPEC>;
#[doc = "Graphic MMU LUT entry 809 high"]
pub mod lut809h;
#[doc = "LUT810H register accessor: an alias for `Reg<LUT810H_SPEC>`"]
pub type LUT810H = crate::Reg<lut810h::LUT810H_SPEC>;
#[doc = "Graphic MMU LUT entry 810 high"]
pub mod lut810h;
#[doc = "LUT811H register accessor: an alias for `Reg<LUT811H_SPEC>`"]
pub type LUT811H = crate::Reg<lut811h::LUT811H_SPEC>;
#[doc = "Graphic MMU LUT entry 811 high"]
pub mod lut811h;
#[doc = "LUT812H register accessor: an alias for `Reg<LUT812H_SPEC>`"]
pub type LUT812H = crate::Reg<lut812h::LUT812H_SPEC>;
#[doc = "Graphic MMU LUT entry 812 high"]
pub mod lut812h;
#[doc = "LUT813H register accessor: an alias for `Reg<LUT813H_SPEC>`"]
pub type LUT813H = crate::Reg<lut813h::LUT813H_SPEC>;
#[doc = "Graphic MMU LUT entry 813 high"]
pub mod lut813h;
#[doc = "LUT814H register accessor: an alias for `Reg<LUT814H_SPEC>`"]
pub type LUT814H = crate::Reg<lut814h::LUT814H_SPEC>;
#[doc = "Graphic MMU LUT entry 814 high"]
pub mod lut814h;
#[doc = "LUT815H register accessor: an alias for `Reg<LUT815H_SPEC>`"]
pub type LUT815H = crate::Reg<lut815h::LUT815H_SPEC>;
#[doc = "Graphic MMU LUT entry 815 high"]
pub mod lut815h;
#[doc = "LUT816H register accessor: an alias for `Reg<LUT816H_SPEC>`"]
pub type LUT816H = crate::Reg<lut816h::LUT816H_SPEC>;
#[doc = "Graphic MMU LUT entry 816 high"]
pub mod lut816h;
#[doc = "LUT817H register accessor: an alias for `Reg<LUT817H_SPEC>`"]
pub type LUT817H = crate::Reg<lut817h::LUT817H_SPEC>;
#[doc = "Graphic MMU LUT entry 817 high"]
pub mod lut817h;
#[doc = "LUT818H register accessor: an alias for `Reg<LUT818H_SPEC>`"]
pub type LUT818H = crate::Reg<lut818h::LUT818H_SPEC>;
#[doc = "Graphic MMU LUT entry 818 high"]
pub mod lut818h;
#[doc = "LUT819H register accessor: an alias for `Reg<LUT819H_SPEC>`"]
pub type LUT819H = crate::Reg<lut819h::LUT819H_SPEC>;
#[doc = "Graphic MMU LUT entry 819 high"]
pub mod lut819h;
#[doc = "LUT820H register accessor: an alias for `Reg<LUT820H_SPEC>`"]
pub type LUT820H = crate::Reg<lut820h::LUT820H_SPEC>;
#[doc = "Graphic MMU LUT entry 820 high"]
pub mod lut820h;
#[doc = "LUT821H register accessor: an alias for `Reg<LUT821H_SPEC>`"]
pub type LUT821H = crate::Reg<lut821h::LUT821H_SPEC>;
#[doc = "Graphic MMU LUT entry 821 high"]
pub mod lut821h;
#[doc = "LUT822H register accessor: an alias for `Reg<LUT822H_SPEC>`"]
pub type LUT822H = crate::Reg<lut822h::LUT822H_SPEC>;
#[doc = "Graphic MMU LUT entry 822 high"]
pub mod lut822h;
#[doc = "LUT823H register accessor: an alias for `Reg<LUT823H_SPEC>`"]
pub type LUT823H = crate::Reg<lut823h::LUT823H_SPEC>;
#[doc = "Graphic MMU LUT entry 823 high"]
pub mod lut823h;
#[doc = "LUT824H register accessor: an alias for `Reg<LUT824H_SPEC>`"]
pub type LUT824H = crate::Reg<lut824h::LUT824H_SPEC>;
#[doc = "Graphic MMU LUT entry 824 high"]
pub mod lut824h;
#[doc = "LUT825H register accessor: an alias for `Reg<LUT825H_SPEC>`"]
pub type LUT825H = crate::Reg<lut825h::LUT825H_SPEC>;
#[doc = "Graphic MMU LUT entry 825 high"]
pub mod lut825h;
#[doc = "LUT826H register accessor: an alias for `Reg<LUT826H_SPEC>`"]
pub type LUT826H = crate::Reg<lut826h::LUT826H_SPEC>;
#[doc = "Graphic MMU LUT entry 826 high"]
pub mod lut826h;
#[doc = "LUT827H register accessor: an alias for `Reg<LUT827H_SPEC>`"]
pub type LUT827H = crate::Reg<lut827h::LUT827H_SPEC>;
#[doc = "Graphic MMU LUT entry 827 high"]
pub mod lut827h;
#[doc = "LUT828H register accessor: an alias for `Reg<LUT828H_SPEC>`"]
pub type LUT828H = crate::Reg<lut828h::LUT828H_SPEC>;
#[doc = "Graphic MMU LUT entry 828 high"]
pub mod lut828h;
#[doc = "LUT829H register accessor: an alias for `Reg<LUT829H_SPEC>`"]
pub type LUT829H = crate::Reg<lut829h::LUT829H_SPEC>;
#[doc = "Graphic MMU LUT entry 829 high"]
pub mod lut829h;
#[doc = "LUT830H register accessor: an alias for `Reg<LUT830H_SPEC>`"]
pub type LUT830H = crate::Reg<lut830h::LUT830H_SPEC>;
#[doc = "Graphic MMU LUT entry 830 high"]
pub mod lut830h;
#[doc = "LUT831H register accessor: an alias for `Reg<LUT831H_SPEC>`"]
pub type LUT831H = crate::Reg<lut831h::LUT831H_SPEC>;
#[doc = "Graphic MMU LUT entry 831 high"]
pub mod lut831h;
#[doc = "LUT832H register accessor: an alias for `Reg<LUT832H_SPEC>`"]
pub type LUT832H = crate::Reg<lut832h::LUT832H_SPEC>;
#[doc = "Graphic MMU LUT entry 832 high"]
pub mod lut832h;
#[doc = "LUT833H register accessor: an alias for `Reg<LUT833H_SPEC>`"]
pub type LUT833H = crate::Reg<lut833h::LUT833H_SPEC>;
#[doc = "Graphic MMU LUT entry 833 high"]
pub mod lut833h;
#[doc = "LUT834H register accessor: an alias for `Reg<LUT834H_SPEC>`"]
pub type LUT834H = crate::Reg<lut834h::LUT834H_SPEC>;
#[doc = "Graphic MMU LUT entry 834 high"]
pub mod lut834h;
#[doc = "LUT835H register accessor: an alias for `Reg<LUT835H_SPEC>`"]
pub type LUT835H = crate::Reg<lut835h::LUT835H_SPEC>;
#[doc = "Graphic MMU LUT entry 835 high"]
pub mod lut835h;
#[doc = "LUT836H register accessor: an alias for `Reg<LUT836H_SPEC>`"]
pub type LUT836H = crate::Reg<lut836h::LUT836H_SPEC>;
#[doc = "Graphic MMU LUT entry 836 high"]
pub mod lut836h;
#[doc = "LUT837H register accessor: an alias for `Reg<LUT837H_SPEC>`"]
pub type LUT837H = crate::Reg<lut837h::LUT837H_SPEC>;
#[doc = "Graphic MMU LUT entry 837 high"]
pub mod lut837h;
#[doc = "LUT838H register accessor: an alias for `Reg<LUT838H_SPEC>`"]
pub type LUT838H = crate::Reg<lut838h::LUT838H_SPEC>;
#[doc = "Graphic MMU LUT entry 838 high"]
pub mod lut838h;
#[doc = "LUT839H register accessor: an alias for `Reg<LUT839H_SPEC>`"]
pub type LUT839H = crate::Reg<lut839h::LUT839H_SPEC>;
#[doc = "Graphic MMU LUT entry 839 high"]
pub mod lut839h;
#[doc = "LUT840H register accessor: an alias for `Reg<LUT840H_SPEC>`"]
pub type LUT840H = crate::Reg<lut840h::LUT840H_SPEC>;
#[doc = "Graphic MMU LUT entry 840 high"]
pub mod lut840h;
#[doc = "LUT841H register accessor: an alias for `Reg<LUT841H_SPEC>`"]
pub type LUT841H = crate::Reg<lut841h::LUT841H_SPEC>;
#[doc = "Graphic MMU LUT entry 841 high"]
pub mod lut841h;
#[doc = "LUT842H register accessor: an alias for `Reg<LUT842H_SPEC>`"]
pub type LUT842H = crate::Reg<lut842h::LUT842H_SPEC>;
#[doc = "Graphic MMU LUT entry 842 high"]
pub mod lut842h;
#[doc = "LUT843H register accessor: an alias for `Reg<LUT843H_SPEC>`"]
pub type LUT843H = crate::Reg<lut843h::LUT843H_SPEC>;
#[doc = "Graphic MMU LUT entry 843 high"]
pub mod lut843h;
#[doc = "LUT844H register accessor: an alias for `Reg<LUT844H_SPEC>`"]
pub type LUT844H = crate::Reg<lut844h::LUT844H_SPEC>;
#[doc = "Graphic MMU LUT entry 844 high"]
pub mod lut844h;
#[doc = "LUT845H register accessor: an alias for `Reg<LUT845H_SPEC>`"]
pub type LUT845H = crate::Reg<lut845h::LUT845H_SPEC>;
#[doc = "Graphic MMU LUT entry 845 high"]
pub mod lut845h;
#[doc = "LUT846H register accessor: an alias for `Reg<LUT846H_SPEC>`"]
pub type LUT846H = crate::Reg<lut846h::LUT846H_SPEC>;
#[doc = "Graphic MMU LUT entry 846 high"]
pub mod lut846h;
#[doc = "LUT847H register accessor: an alias for `Reg<LUT847H_SPEC>`"]
pub type LUT847H = crate::Reg<lut847h::LUT847H_SPEC>;
#[doc = "Graphic MMU LUT entry 847 high"]
pub mod lut847h;
#[doc = "LUT848H register accessor: an alias for `Reg<LUT848H_SPEC>`"]
pub type LUT848H = crate::Reg<lut848h::LUT848H_SPEC>;
#[doc = "Graphic MMU LUT entry 848 high"]
pub mod lut848h;
#[doc = "LUT849H register accessor: an alias for `Reg<LUT849H_SPEC>`"]
pub type LUT849H = crate::Reg<lut849h::LUT849H_SPEC>;
#[doc = "Graphic MMU LUT entry 849 high"]
pub mod lut849h;
#[doc = "LUT850H register accessor: an alias for `Reg<LUT850H_SPEC>`"]
pub type LUT850H = crate::Reg<lut850h::LUT850H_SPEC>;
#[doc = "Graphic MMU LUT entry 850 high"]
pub mod lut850h;
#[doc = "LUT851H register accessor: an alias for `Reg<LUT851H_SPEC>`"]
pub type LUT851H = crate::Reg<lut851h::LUT851H_SPEC>;
#[doc = "Graphic MMU LUT entry 851 high"]
pub mod lut851h;
#[doc = "LUT852H register accessor: an alias for `Reg<LUT852H_SPEC>`"]
pub type LUT852H = crate::Reg<lut852h::LUT852H_SPEC>;
#[doc = "Graphic MMU LUT entry 852 high"]
pub mod lut852h;
#[doc = "LUT853H register accessor: an alias for `Reg<LUT853H_SPEC>`"]
pub type LUT853H = crate::Reg<lut853h::LUT853H_SPEC>;
#[doc = "Graphic MMU LUT entry 853 high"]
pub mod lut853h;
#[doc = "LUT854H register accessor: an alias for `Reg<LUT854H_SPEC>`"]
pub type LUT854H = crate::Reg<lut854h::LUT854H_SPEC>;
#[doc = "Graphic MMU LUT entry 854 high"]
pub mod lut854h;
#[doc = "LUT855H register accessor: an alias for `Reg<LUT855H_SPEC>`"]
pub type LUT855H = crate::Reg<lut855h::LUT855H_SPEC>;
#[doc = "Graphic MMU LUT entry 855 high"]
pub mod lut855h;
#[doc = "LUT856H register accessor: an alias for `Reg<LUT856H_SPEC>`"]
pub type LUT856H = crate::Reg<lut856h::LUT856H_SPEC>;
#[doc = "Graphic MMU LUT entry 856 high"]
pub mod lut856h;
#[doc = "LUT857H register accessor: an alias for `Reg<LUT857H_SPEC>`"]
pub type LUT857H = crate::Reg<lut857h::LUT857H_SPEC>;
#[doc = "Graphic MMU LUT entry 857 high"]
pub mod lut857h;
#[doc = "LUT858H register accessor: an alias for `Reg<LUT858H_SPEC>`"]
pub type LUT858H = crate::Reg<lut858h::LUT858H_SPEC>;
#[doc = "Graphic MMU LUT entry 858 high"]
pub mod lut858h;
#[doc = "LUT859H register accessor: an alias for `Reg<LUT859H_SPEC>`"]
pub type LUT859H = crate::Reg<lut859h::LUT859H_SPEC>;
#[doc = "Graphic MMU LUT entry 859 high"]
pub mod lut859h;
#[doc = "LUT860H register accessor: an alias for `Reg<LUT860H_SPEC>`"]
pub type LUT860H = crate::Reg<lut860h::LUT860H_SPEC>;
#[doc = "Graphic MMU LUT entry 860 high"]
pub mod lut860h;
#[doc = "LUT861H register accessor: an alias for `Reg<LUT861H_SPEC>`"]
pub type LUT861H = crate::Reg<lut861h::LUT861H_SPEC>;
#[doc = "Graphic MMU LUT entry 861 high"]
pub mod lut861h;
#[doc = "LUT862H register accessor: an alias for `Reg<LUT862H_SPEC>`"]
pub type LUT862H = crate::Reg<lut862h::LUT862H_SPEC>;
#[doc = "Graphic MMU LUT entry 862 high"]
pub mod lut862h;
#[doc = "LUT863H register accessor: an alias for `Reg<LUT863H_SPEC>`"]
pub type LUT863H = crate::Reg<lut863h::LUT863H_SPEC>;
#[doc = "Graphic MMU LUT entry 863 high"]
pub mod lut863h;
#[doc = "LUT864H register accessor: an alias for `Reg<LUT864H_SPEC>`"]
pub type LUT864H = crate::Reg<lut864h::LUT864H_SPEC>;
#[doc = "Graphic MMU LUT entry 864 high"]
pub mod lut864h;
#[doc = "LUT865H register accessor: an alias for `Reg<LUT865H_SPEC>`"]
pub type LUT865H = crate::Reg<lut865h::LUT865H_SPEC>;
#[doc = "Graphic MMU LUT entry 865 high"]
pub mod lut865h;
#[doc = "LUT866H register accessor: an alias for `Reg<LUT866H_SPEC>`"]
pub type LUT866H = crate::Reg<lut866h::LUT866H_SPEC>;
#[doc = "Graphic MMU LUT entry 866 high"]
pub mod lut866h;
#[doc = "LUT867H register accessor: an alias for `Reg<LUT867H_SPEC>`"]
pub type LUT867H = crate::Reg<lut867h::LUT867H_SPEC>;
#[doc = "Graphic MMU LUT entry 867 high"]
pub mod lut867h;
#[doc = "LUT868H register accessor: an alias for `Reg<LUT868H_SPEC>`"]
pub type LUT868H = crate::Reg<lut868h::LUT868H_SPEC>;
#[doc = "Graphic MMU LUT entry 868 high"]
pub mod lut868h;
#[doc = "LUT869H register accessor: an alias for `Reg<LUT869H_SPEC>`"]
pub type LUT869H = crate::Reg<lut869h::LUT869H_SPEC>;
#[doc = "Graphic MMU LUT entry 869 high"]
pub mod lut869h;
#[doc = "LUT870H register accessor: an alias for `Reg<LUT870H_SPEC>`"]
pub type LUT870H = crate::Reg<lut870h::LUT870H_SPEC>;
#[doc = "Graphic MMU LUT entry 870 high"]
pub mod lut870h;
#[doc = "LUT871H register accessor: an alias for `Reg<LUT871H_SPEC>`"]
pub type LUT871H = crate::Reg<lut871h::LUT871H_SPEC>;
#[doc = "Graphic MMU LUT entry 871 high"]
pub mod lut871h;
#[doc = "LUT872H register accessor: an alias for `Reg<LUT872H_SPEC>`"]
pub type LUT872H = crate::Reg<lut872h::LUT872H_SPEC>;
#[doc = "Graphic MMU LUT entry 872 high"]
pub mod lut872h;
#[doc = "LUT873H register accessor: an alias for `Reg<LUT873H_SPEC>`"]
pub type LUT873H = crate::Reg<lut873h::LUT873H_SPEC>;
#[doc = "Graphic MMU LUT entry 873 high"]
pub mod lut873h;
#[doc = "LUT874H register accessor: an alias for `Reg<LUT874H_SPEC>`"]
pub type LUT874H = crate::Reg<lut874h::LUT874H_SPEC>;
#[doc = "Graphic MMU LUT entry 874 high"]
pub mod lut874h;
#[doc = "LUT875H register accessor: an alias for `Reg<LUT875H_SPEC>`"]
pub type LUT875H = crate::Reg<lut875h::LUT875H_SPEC>;
#[doc = "Graphic MMU LUT entry 875 high"]
pub mod lut875h;
#[doc = "LUT876H register accessor: an alias for `Reg<LUT876H_SPEC>`"]
pub type LUT876H = crate::Reg<lut876h::LUT876H_SPEC>;
#[doc = "Graphic MMU LUT entry 876 high"]
pub mod lut876h;
#[doc = "LUT877H register accessor: an alias for `Reg<LUT877H_SPEC>`"]
pub type LUT877H = crate::Reg<lut877h::LUT877H_SPEC>;
#[doc = "Graphic MMU LUT entry 877 high"]
pub mod lut877h;
#[doc = "LUT878H register accessor: an alias for `Reg<LUT878H_SPEC>`"]
pub type LUT878H = crate::Reg<lut878h::LUT878H_SPEC>;
#[doc = "Graphic MMU LUT entry 878 high"]
pub mod lut878h;
#[doc = "LUT879H register accessor: an alias for `Reg<LUT879H_SPEC>`"]
pub type LUT879H = crate::Reg<lut879h::LUT879H_SPEC>;
#[doc = "Graphic MMU LUT entry 879 high"]
pub mod lut879h;
#[doc = "LUT880H register accessor: an alias for `Reg<LUT880H_SPEC>`"]
pub type LUT880H = crate::Reg<lut880h::LUT880H_SPEC>;
#[doc = "Graphic MMU LUT entry 880 high"]
pub mod lut880h;
#[doc = "LUT881H register accessor: an alias for `Reg<LUT881H_SPEC>`"]
pub type LUT881H = crate::Reg<lut881h::LUT881H_SPEC>;
#[doc = "Graphic MMU LUT entry 881 high"]
pub mod lut881h;
#[doc = "LUT882H register accessor: an alias for `Reg<LUT882H_SPEC>`"]
pub type LUT882H = crate::Reg<lut882h::LUT882H_SPEC>;
#[doc = "Graphic MMU LUT entry 882 high"]
pub mod lut882h;
#[doc = "LUT883H register accessor: an alias for `Reg<LUT883H_SPEC>`"]
pub type LUT883H = crate::Reg<lut883h::LUT883H_SPEC>;
#[doc = "Graphic MMU LUT entry 883 high"]
pub mod lut883h;
#[doc = "LUT884H register accessor: an alias for `Reg<LUT884H_SPEC>`"]
pub type LUT884H = crate::Reg<lut884h::LUT884H_SPEC>;
#[doc = "Graphic MMU LUT entry 884 high"]
pub mod lut884h;
#[doc = "LUT885H register accessor: an alias for `Reg<LUT885H_SPEC>`"]
pub type LUT885H = crate::Reg<lut885h::LUT885H_SPEC>;
#[doc = "Graphic MMU LUT entry 885 high"]
pub mod lut885h;
#[doc = "LUT886H register accessor: an alias for `Reg<LUT886H_SPEC>`"]
pub type LUT886H = crate::Reg<lut886h::LUT886H_SPEC>;
#[doc = "Graphic MMU LUT entry 886 high"]
pub mod lut886h;
#[doc = "LUT887H register accessor: an alias for `Reg<LUT887H_SPEC>`"]
pub type LUT887H = crate::Reg<lut887h::LUT887H_SPEC>;
#[doc = "Graphic MMU LUT entry 887 high"]
pub mod lut887h;
#[doc = "LUT888H register accessor: an alias for `Reg<LUT888H_SPEC>`"]
pub type LUT888H = crate::Reg<lut888h::LUT888H_SPEC>;
#[doc = "Graphic MMU LUT entry 888 high"]
pub mod lut888h;
#[doc = "LUT889H register accessor: an alias for `Reg<LUT889H_SPEC>`"]
pub type LUT889H = crate::Reg<lut889h::LUT889H_SPEC>;
#[doc = "Graphic MMU LUT entry 889 high"]
pub mod lut889h;
#[doc = "LUT890H register accessor: an alias for `Reg<LUT890H_SPEC>`"]
pub type LUT890H = crate::Reg<lut890h::LUT890H_SPEC>;
#[doc = "Graphic MMU LUT entry 890 high"]
pub mod lut890h;
#[doc = "LUT891H register accessor: an alias for `Reg<LUT891H_SPEC>`"]
pub type LUT891H = crate::Reg<lut891h::LUT891H_SPEC>;
#[doc = "Graphic MMU LUT entry 891 high"]
pub mod lut891h;
#[doc = "LUT892H register accessor: an alias for `Reg<LUT892H_SPEC>`"]
pub type LUT892H = crate::Reg<lut892h::LUT892H_SPEC>;
#[doc = "Graphic MMU LUT entry 892 high"]
pub mod lut892h;
#[doc = "LUT893H register accessor: an alias for `Reg<LUT893H_SPEC>`"]
pub type LUT893H = crate::Reg<lut893h::LUT893H_SPEC>;
#[doc = "Graphic MMU LUT entry 893 high"]
pub mod lut893h;
#[doc = "LUT894H register accessor: an alias for `Reg<LUT894H_SPEC>`"]
pub type LUT894H = crate::Reg<lut894h::LUT894H_SPEC>;
#[doc = "Graphic MMU LUT entry 894 high"]
pub mod lut894h;
#[doc = "LUT895H register accessor: an alias for `Reg<LUT895H_SPEC>`"]
pub type LUT895H = crate::Reg<lut895h::LUT895H_SPEC>;
#[doc = "Graphic MMU LUT entry 895 high"]
pub mod lut895h;
#[doc = "LUT896H register accessor: an alias for `Reg<LUT896H_SPEC>`"]
pub type LUT896H = crate::Reg<lut896h::LUT896H_SPEC>;
#[doc = "Graphic MMU LUT entry 896 high"]
pub mod lut896h;
#[doc = "LUT897H register accessor: an alias for `Reg<LUT897H_SPEC>`"]
pub type LUT897H = crate::Reg<lut897h::LUT897H_SPEC>;
#[doc = "Graphic MMU LUT entry 897 high"]
pub mod lut897h;
#[doc = "LUT898H register accessor: an alias for `Reg<LUT898H_SPEC>`"]
pub type LUT898H = crate::Reg<lut898h::LUT898H_SPEC>;
#[doc = "Graphic MMU LUT entry 898 high"]
pub mod lut898h;
#[doc = "LUT899H register accessor: an alias for `Reg<LUT899H_SPEC>`"]
pub type LUT899H = crate::Reg<lut899h::LUT899H_SPEC>;
#[doc = "Graphic MMU LUT entry 899 high"]
pub mod lut899h;
#[doc = "LUT900H register accessor: an alias for `Reg<LUT900H_SPEC>`"]
pub type LUT900H = crate::Reg<lut900h::LUT900H_SPEC>;
#[doc = "Graphic MMU LUT entry 900 high"]
pub mod lut900h;
#[doc = "LUT901H register accessor: an alias for `Reg<LUT901H_SPEC>`"]
pub type LUT901H = crate::Reg<lut901h::LUT901H_SPEC>;
#[doc = "Graphic MMU LUT entry 901 high"]
pub mod lut901h;
#[doc = "LUT902H register accessor: an alias for `Reg<LUT902H_SPEC>`"]
pub type LUT902H = crate::Reg<lut902h::LUT902H_SPEC>;
#[doc = "Graphic MMU LUT entry 902 high"]
pub mod lut902h;
#[doc = "LUT903H register accessor: an alias for `Reg<LUT903H_SPEC>`"]
pub type LUT903H = crate::Reg<lut903h::LUT903H_SPEC>;
#[doc = "Graphic MMU LUT entry 903 high"]
pub mod lut903h;
#[doc = "LUT904H register accessor: an alias for `Reg<LUT904H_SPEC>`"]
pub type LUT904H = crate::Reg<lut904h::LUT904H_SPEC>;
#[doc = "Graphic MMU LUT entry 904 high"]
pub mod lut904h;
#[doc = "LUT905H register accessor: an alias for `Reg<LUT905H_SPEC>`"]
pub type LUT905H = crate::Reg<lut905h::LUT905H_SPEC>;
#[doc = "Graphic MMU LUT entry 905 high"]
pub mod lut905h;
#[doc = "LUT906H register accessor: an alias for `Reg<LUT906H_SPEC>`"]
pub type LUT906H = crate::Reg<lut906h::LUT906H_SPEC>;
#[doc = "Graphic MMU LUT entry 906 high"]
pub mod lut906h;
#[doc = "LUT907H register accessor: an alias for `Reg<LUT907H_SPEC>`"]
pub type LUT907H = crate::Reg<lut907h::LUT907H_SPEC>;
#[doc = "Graphic MMU LUT entry 907 high"]
pub mod lut907h;
#[doc = "LUT908H register accessor: an alias for `Reg<LUT908H_SPEC>`"]
pub type LUT908H = crate::Reg<lut908h::LUT908H_SPEC>;
#[doc = "Graphic MMU LUT entry 908 high"]
pub mod lut908h;
#[doc = "LUT909H register accessor: an alias for `Reg<LUT909H_SPEC>`"]
pub type LUT909H = crate::Reg<lut909h::LUT909H_SPEC>;
#[doc = "Graphic MMU LUT entry 909 high"]
pub mod lut909h;
#[doc = "LUT910H register accessor: an alias for `Reg<LUT910H_SPEC>`"]
pub type LUT910H = crate::Reg<lut910h::LUT910H_SPEC>;
#[doc = "Graphic MMU LUT entry 910 high"]
pub mod lut910h;
#[doc = "LUT911H register accessor: an alias for `Reg<LUT911H_SPEC>`"]
pub type LUT911H = crate::Reg<lut911h::LUT911H_SPEC>;
#[doc = "Graphic MMU LUT entry 911 high"]
pub mod lut911h;
#[doc = "LUT912H register accessor: an alias for `Reg<LUT912H_SPEC>`"]
pub type LUT912H = crate::Reg<lut912h::LUT912H_SPEC>;
#[doc = "Graphic MMU LUT entry 912 high"]
pub mod lut912h;
#[doc = "LUT913H register accessor: an alias for `Reg<LUT913H_SPEC>`"]
pub type LUT913H = crate::Reg<lut913h::LUT913H_SPEC>;
#[doc = "Graphic MMU LUT entry 913 high"]
pub mod lut913h;
#[doc = "LUT914H register accessor: an alias for `Reg<LUT914H_SPEC>`"]
pub type LUT914H = crate::Reg<lut914h::LUT914H_SPEC>;
#[doc = "Graphic MMU LUT entry 914 high"]
pub mod lut914h;
#[doc = "LUT915H register accessor: an alias for `Reg<LUT915H_SPEC>`"]
pub type LUT915H = crate::Reg<lut915h::LUT915H_SPEC>;
#[doc = "Graphic MMU LUT entry 915 high"]
pub mod lut915h;
#[doc = "LUT916H register accessor: an alias for `Reg<LUT916H_SPEC>`"]
pub type LUT916H = crate::Reg<lut916h::LUT916H_SPEC>;
#[doc = "Graphic MMU LUT entry 916 high"]
pub mod lut916h;
#[doc = "LUT917H register accessor: an alias for `Reg<LUT917H_SPEC>`"]
pub type LUT917H = crate::Reg<lut917h::LUT917H_SPEC>;
#[doc = "Graphic MMU LUT entry 917 high"]
pub mod lut917h;
#[doc = "LUT918H register accessor: an alias for `Reg<LUT918H_SPEC>`"]
pub type LUT918H = crate::Reg<lut918h::LUT918H_SPEC>;
#[doc = "Graphic MMU LUT entry 918 high"]
pub mod lut918h;
#[doc = "LUT919H register accessor: an alias for `Reg<LUT919H_SPEC>`"]
pub type LUT919H = crate::Reg<lut919h::LUT919H_SPEC>;
#[doc = "Graphic MMU LUT entry 919 high"]
pub mod lut919h;
#[doc = "LUT920H register accessor: an alias for `Reg<LUT920H_SPEC>`"]
pub type LUT920H = crate::Reg<lut920h::LUT920H_SPEC>;
#[doc = "Graphic MMU LUT entry 920 high"]
pub mod lut920h;
#[doc = "LUT921H register accessor: an alias for `Reg<LUT921H_SPEC>`"]
pub type LUT921H = crate::Reg<lut921h::LUT921H_SPEC>;
#[doc = "Graphic MMU LUT entry 921 high"]
pub mod lut921h;
#[doc = "LUT922H register accessor: an alias for `Reg<LUT922H_SPEC>`"]
pub type LUT922H = crate::Reg<lut922h::LUT922H_SPEC>;
#[doc = "Graphic MMU LUT entry 922 high"]
pub mod lut922h;
#[doc = "LUT923H register accessor: an alias for `Reg<LUT923H_SPEC>`"]
pub type LUT923H = crate::Reg<lut923h::LUT923H_SPEC>;
#[doc = "Graphic MMU LUT entry 923 high"]
pub mod lut923h;
#[doc = "LUT924H register accessor: an alias for `Reg<LUT924H_SPEC>`"]
pub type LUT924H = crate::Reg<lut924h::LUT924H_SPEC>;
#[doc = "Graphic MMU LUT entry 924 high"]
pub mod lut924h;
#[doc = "LUT925H register accessor: an alias for `Reg<LUT925H_SPEC>`"]
pub type LUT925H = crate::Reg<lut925h::LUT925H_SPEC>;
#[doc = "Graphic MMU LUT entry 925 high"]
pub mod lut925h;
#[doc = "LUT926H register accessor: an alias for `Reg<LUT926H_SPEC>`"]
pub type LUT926H = crate::Reg<lut926h::LUT926H_SPEC>;
#[doc = "Graphic MMU LUT entry 926 high"]
pub mod lut926h;
#[doc = "LUT927H register accessor: an alias for `Reg<LUT927H_SPEC>`"]
pub type LUT927H = crate::Reg<lut927h::LUT927H_SPEC>;
#[doc = "Graphic MMU LUT entry 927 high"]
pub mod lut927h;
#[doc = "LUT928H register accessor: an alias for `Reg<LUT928H_SPEC>`"]
pub type LUT928H = crate::Reg<lut928h::LUT928H_SPEC>;
#[doc = "Graphic MMU LUT entry 928 high"]
pub mod lut928h;
#[doc = "LUT929H register accessor: an alias for `Reg<LUT929H_SPEC>`"]
pub type LUT929H = crate::Reg<lut929h::LUT929H_SPEC>;
#[doc = "Graphic MMU LUT entry 929 high"]
pub mod lut929h;
#[doc = "LUT930H register accessor: an alias for `Reg<LUT930H_SPEC>`"]
pub type LUT930H = crate::Reg<lut930h::LUT930H_SPEC>;
#[doc = "Graphic MMU LUT entry 930 high"]
pub mod lut930h;
#[doc = "LUT931H register accessor: an alias for `Reg<LUT931H_SPEC>`"]
pub type LUT931H = crate::Reg<lut931h::LUT931H_SPEC>;
#[doc = "Graphic MMU LUT entry 931 high"]
pub mod lut931h;
#[doc = "LUT932H register accessor: an alias for `Reg<LUT932H_SPEC>`"]
pub type LUT932H = crate::Reg<lut932h::LUT932H_SPEC>;
#[doc = "Graphic MMU LUT entry 932 high"]
pub mod lut932h;
#[doc = "LUT933H register accessor: an alias for `Reg<LUT933H_SPEC>`"]
pub type LUT933H = crate::Reg<lut933h::LUT933H_SPEC>;
#[doc = "Graphic MMU LUT entry 933 high"]
pub mod lut933h;
#[doc = "LUT934H register accessor: an alias for `Reg<LUT934H_SPEC>`"]
pub type LUT934H = crate::Reg<lut934h::LUT934H_SPEC>;
#[doc = "Graphic MMU LUT entry 934 high"]
pub mod lut934h;
#[doc = "LUT935H register accessor: an alias for `Reg<LUT935H_SPEC>`"]
pub type LUT935H = crate::Reg<lut935h::LUT935H_SPEC>;
#[doc = "Graphic MMU LUT entry 935 high"]
pub mod lut935h;
#[doc = "LUT936H register accessor: an alias for `Reg<LUT936H_SPEC>`"]
pub type LUT936H = crate::Reg<lut936h::LUT936H_SPEC>;
#[doc = "Graphic MMU LUT entry 936 high"]
pub mod lut936h;
#[doc = "LUT937H register accessor: an alias for `Reg<LUT937H_SPEC>`"]
pub type LUT937H = crate::Reg<lut937h::LUT937H_SPEC>;
#[doc = "Graphic MMU LUT entry 937 high"]
pub mod lut937h;
#[doc = "LUT938H register accessor: an alias for `Reg<LUT938H_SPEC>`"]
pub type LUT938H = crate::Reg<lut938h::LUT938H_SPEC>;
#[doc = "Graphic MMU LUT entry 938 high"]
pub mod lut938h;
#[doc = "LUT939H register accessor: an alias for `Reg<LUT939H_SPEC>`"]
pub type LUT939H = crate::Reg<lut939h::LUT939H_SPEC>;
#[doc = "Graphic MMU LUT entry 939 high"]
pub mod lut939h;
#[doc = "LUT940H register accessor: an alias for `Reg<LUT940H_SPEC>`"]
pub type LUT940H = crate::Reg<lut940h::LUT940H_SPEC>;
#[doc = "Graphic MMU LUT entry 940 high"]
pub mod lut940h;
#[doc = "LUT941H register accessor: an alias for `Reg<LUT941H_SPEC>`"]
pub type LUT941H = crate::Reg<lut941h::LUT941H_SPEC>;
#[doc = "Graphic MMU LUT entry 941 high"]
pub mod lut941h;
#[doc = "LUT942H register accessor: an alias for `Reg<LUT942H_SPEC>`"]
pub type LUT942H = crate::Reg<lut942h::LUT942H_SPEC>;
#[doc = "Graphic MMU LUT entry 942 high"]
pub mod lut942h;
#[doc = "LUT943H register accessor: an alias for `Reg<LUT943H_SPEC>`"]
pub type LUT943H = crate::Reg<lut943h::LUT943H_SPEC>;
#[doc = "Graphic MMU LUT entry 943 high"]
pub mod lut943h;
#[doc = "LUT944H register accessor: an alias for `Reg<LUT944H_SPEC>`"]
pub type LUT944H = crate::Reg<lut944h::LUT944H_SPEC>;
#[doc = "Graphic MMU LUT entry 944 high"]
pub mod lut944h;
#[doc = "LUT945H register accessor: an alias for `Reg<LUT945H_SPEC>`"]
pub type LUT945H = crate::Reg<lut945h::LUT945H_SPEC>;
#[doc = "Graphic MMU LUT entry 945 high"]
pub mod lut945h;
#[doc = "LUT946H register accessor: an alias for `Reg<LUT946H_SPEC>`"]
pub type LUT946H = crate::Reg<lut946h::LUT946H_SPEC>;
#[doc = "Graphic MMU LUT entry 946 high"]
pub mod lut946h;
#[doc = "LUT947H register accessor: an alias for `Reg<LUT947H_SPEC>`"]
pub type LUT947H = crate::Reg<lut947h::LUT947H_SPEC>;
#[doc = "Graphic MMU LUT entry 947 high"]
pub mod lut947h;
#[doc = "LUT948H register accessor: an alias for `Reg<LUT948H_SPEC>`"]
pub type LUT948H = crate::Reg<lut948h::LUT948H_SPEC>;
#[doc = "Graphic MMU LUT entry 948 high"]
pub mod lut948h;
#[doc = "LUT949H register accessor: an alias for `Reg<LUT949H_SPEC>`"]
pub type LUT949H = crate::Reg<lut949h::LUT949H_SPEC>;
#[doc = "Graphic MMU LUT entry 949 high"]
pub mod lut949h;
#[doc = "LUT950H register accessor: an alias for `Reg<LUT950H_SPEC>`"]
pub type LUT950H = crate::Reg<lut950h::LUT950H_SPEC>;
#[doc = "Graphic MMU LUT entry 950 high"]
pub mod lut950h;
#[doc = "LUT951H register accessor: an alias for `Reg<LUT951H_SPEC>`"]
pub type LUT951H = crate::Reg<lut951h::LUT951H_SPEC>;
#[doc = "Graphic MMU LUT entry 951 high"]
pub mod lut951h;
#[doc = "LUT952H register accessor: an alias for `Reg<LUT952H_SPEC>`"]
pub type LUT952H = crate::Reg<lut952h::LUT952H_SPEC>;
#[doc = "Graphic MMU LUT entry 952 high"]
pub mod lut952h;
#[doc = "LUT953H register accessor: an alias for `Reg<LUT953H_SPEC>`"]
pub type LUT953H = crate::Reg<lut953h::LUT953H_SPEC>;
#[doc = "Graphic MMU LUT entry 953 high"]
pub mod lut953h;
#[doc = "LUT954H register accessor: an alias for `Reg<LUT954H_SPEC>`"]
pub type LUT954H = crate::Reg<lut954h::LUT954H_SPEC>;
#[doc = "Graphic MMU LUT entry 954 high"]
pub mod lut954h;
#[doc = "LUT955H register accessor: an alias for `Reg<LUT955H_SPEC>`"]
pub type LUT955H = crate::Reg<lut955h::LUT955H_SPEC>;
#[doc = "Graphic MMU LUT entry 955 high"]
pub mod lut955h;
#[doc = "LUT956H register accessor: an alias for `Reg<LUT956H_SPEC>`"]
pub type LUT956H = crate::Reg<lut956h::LUT956H_SPEC>;
#[doc = "Graphic MMU LUT entry 956 high"]
pub mod lut956h;
#[doc = "LUT957H register accessor: an alias for `Reg<LUT957H_SPEC>`"]
pub type LUT957H = crate::Reg<lut957h::LUT957H_SPEC>;
#[doc = "Graphic MMU LUT entry 957 high"]
pub mod lut957h;
#[doc = "LUT958H register accessor: an alias for `Reg<LUT958H_SPEC>`"]
pub type LUT958H = crate::Reg<lut958h::LUT958H_SPEC>;
#[doc = "Graphic MMU LUT entry 958 high"]
pub mod lut958h;
#[doc = "LUT959H register accessor: an alias for `Reg<LUT959H_SPEC>`"]
pub type LUT959H = crate::Reg<lut959h::LUT959H_SPEC>;
#[doc = "Graphic MMU LUT entry 959 high"]
pub mod lut959h;
#[doc = "LUT960H register accessor: an alias for `Reg<LUT960H_SPEC>`"]
pub type LUT960H = crate::Reg<lut960h::LUT960H_SPEC>;
#[doc = "Graphic MMU LUT entry 960 high"]
pub mod lut960h;
#[doc = "LUT961H register accessor: an alias for `Reg<LUT961H_SPEC>`"]
pub type LUT961H = crate::Reg<lut961h::LUT961H_SPEC>;
#[doc = "Graphic MMU LUT entry 961 high"]
pub mod lut961h;
#[doc = "LUT962H register accessor: an alias for `Reg<LUT962H_SPEC>`"]
pub type LUT962H = crate::Reg<lut962h::LUT962H_SPEC>;
#[doc = "Graphic MMU LUT entry 962 high"]
pub mod lut962h;
#[doc = "LUT963H register accessor: an alias for `Reg<LUT963H_SPEC>`"]
pub type LUT963H = crate::Reg<lut963h::LUT963H_SPEC>;
#[doc = "Graphic MMU LUT entry 963 high"]
pub mod lut963h;
#[doc = "LUT964H register accessor: an alias for `Reg<LUT964H_SPEC>`"]
pub type LUT964H = crate::Reg<lut964h::LUT964H_SPEC>;
#[doc = "Graphic MMU LUT entry 964 high"]
pub mod lut964h;
#[doc = "LUT965H register accessor: an alias for `Reg<LUT965H_SPEC>`"]
pub type LUT965H = crate::Reg<lut965h::LUT965H_SPEC>;
#[doc = "Graphic MMU LUT entry 965 high"]
pub mod lut965h;
#[doc = "LUT966H register accessor: an alias for `Reg<LUT966H_SPEC>`"]
pub type LUT966H = crate::Reg<lut966h::LUT966H_SPEC>;
#[doc = "Graphic MMU LUT entry 966 high"]
pub mod lut966h;
#[doc = "LUT967H register accessor: an alias for `Reg<LUT967H_SPEC>`"]
pub type LUT967H = crate::Reg<lut967h::LUT967H_SPEC>;
#[doc = "Graphic MMU LUT entry 967 high"]
pub mod lut967h;
#[doc = "LUT968H register accessor: an alias for `Reg<LUT968H_SPEC>`"]
pub type LUT968H = crate::Reg<lut968h::LUT968H_SPEC>;
#[doc = "Graphic MMU LUT entry 968 high"]
pub mod lut968h;
#[doc = "LUT969H register accessor: an alias for `Reg<LUT969H_SPEC>`"]
pub type LUT969H = crate::Reg<lut969h::LUT969H_SPEC>;
#[doc = "Graphic MMU LUT entry 969 high"]
pub mod lut969h;
#[doc = "LUT970H register accessor: an alias for `Reg<LUT970H_SPEC>`"]
pub type LUT970H = crate::Reg<lut970h::LUT970H_SPEC>;
#[doc = "Graphic MMU LUT entry 970 high"]
pub mod lut970h;
#[doc = "LUT971H register accessor: an alias for `Reg<LUT971H_SPEC>`"]
pub type LUT971H = crate::Reg<lut971h::LUT971H_SPEC>;
#[doc = "Graphic MMU LUT entry 971 high"]
pub mod lut971h;
#[doc = "LUT972H register accessor: an alias for `Reg<LUT972H_SPEC>`"]
pub type LUT972H = crate::Reg<lut972h::LUT972H_SPEC>;
#[doc = "Graphic MMU LUT entry 972 high"]
pub mod lut972h;
#[doc = "LUT973H register accessor: an alias for `Reg<LUT973H_SPEC>`"]
pub type LUT973H = crate::Reg<lut973h::LUT973H_SPEC>;
#[doc = "Graphic MMU LUT entry 973 high"]
pub mod lut973h;
#[doc = "LUT974H register accessor: an alias for `Reg<LUT974H_SPEC>`"]
pub type LUT974H = crate::Reg<lut974h::LUT974H_SPEC>;
#[doc = "Graphic MMU LUT entry 974 high"]
pub mod lut974h;
#[doc = "LUT975H register accessor: an alias for `Reg<LUT975H_SPEC>`"]
pub type LUT975H = crate::Reg<lut975h::LUT975H_SPEC>;
#[doc = "Graphic MMU LUT entry 975 high"]
pub mod lut975h;
#[doc = "LUT976H register accessor: an alias for `Reg<LUT976H_SPEC>`"]
pub type LUT976H = crate::Reg<lut976h::LUT976H_SPEC>;
#[doc = "Graphic MMU LUT entry 976 high"]
pub mod lut976h;
#[doc = "LUT977H register accessor: an alias for `Reg<LUT977H_SPEC>`"]
pub type LUT977H = crate::Reg<lut977h::LUT977H_SPEC>;
#[doc = "Graphic MMU LUT entry 977 high"]
pub mod lut977h;
#[doc = "LUT978H register accessor: an alias for `Reg<LUT978H_SPEC>`"]
pub type LUT978H = crate::Reg<lut978h::LUT978H_SPEC>;
#[doc = "Graphic MMU LUT entry 978 high"]
pub mod lut978h;
#[doc = "LUT979H register accessor: an alias for `Reg<LUT979H_SPEC>`"]
pub type LUT979H = crate::Reg<lut979h::LUT979H_SPEC>;
#[doc = "Graphic MMU LUT entry 979 high"]
pub mod lut979h;
#[doc = "LUT980H register accessor: an alias for `Reg<LUT980H_SPEC>`"]
pub type LUT980H = crate::Reg<lut980h::LUT980H_SPEC>;
#[doc = "Graphic MMU LUT entry 980 high"]
pub mod lut980h;
#[doc = "LUT981H register accessor: an alias for `Reg<LUT981H_SPEC>`"]
pub type LUT981H = crate::Reg<lut981h::LUT981H_SPEC>;
#[doc = "Graphic MMU LUT entry 981 high"]
pub mod lut981h;
#[doc = "LUT982H register accessor: an alias for `Reg<LUT982H_SPEC>`"]
pub type LUT982H = crate::Reg<lut982h::LUT982H_SPEC>;
#[doc = "Graphic MMU LUT entry 982 high"]
pub mod lut982h;
#[doc = "LUT983H register accessor: an alias for `Reg<LUT983H_SPEC>`"]
pub type LUT983H = crate::Reg<lut983h::LUT983H_SPEC>;
#[doc = "Graphic MMU LUT entry 983 high"]
pub mod lut983h;
#[doc = "LUT984H register accessor: an alias for `Reg<LUT984H_SPEC>`"]
pub type LUT984H = crate::Reg<lut984h::LUT984H_SPEC>;
#[doc = "Graphic MMU LUT entry 984 high"]
pub mod lut984h;
#[doc = "LUT985H register accessor: an alias for `Reg<LUT985H_SPEC>`"]
pub type LUT985H = crate::Reg<lut985h::LUT985H_SPEC>;
#[doc = "Graphic MMU LUT entry 985 high"]
pub mod lut985h;
#[doc = "LUT986H register accessor: an alias for `Reg<LUT986H_SPEC>`"]
pub type LUT986H = crate::Reg<lut986h::LUT986H_SPEC>;
#[doc = "Graphic MMU LUT entry 986 high"]
pub mod lut986h;
#[doc = "LUT987H register accessor: an alias for `Reg<LUT987H_SPEC>`"]
pub type LUT987H = crate::Reg<lut987h::LUT987H_SPEC>;
#[doc = "Graphic MMU LUT entry 987 high"]
pub mod lut987h;
#[doc = "LUT988H register accessor: an alias for `Reg<LUT988H_SPEC>`"]
pub type LUT988H = crate::Reg<lut988h::LUT988H_SPEC>;
#[doc = "Graphic MMU LUT entry 988 high"]
pub mod lut988h;
#[doc = "LUT989H register accessor: an alias for `Reg<LUT989H_SPEC>`"]
pub type LUT989H = crate::Reg<lut989h::LUT989H_SPEC>;
#[doc = "Graphic MMU LUT entry 989 high"]
pub mod lut989h;
#[doc = "LUT990H register accessor: an alias for `Reg<LUT990H_SPEC>`"]
pub type LUT990H = crate::Reg<lut990h::LUT990H_SPEC>;
#[doc = "Graphic MMU LUT entry 990 high"]
pub mod lut990h;
#[doc = "LUT991H register accessor: an alias for `Reg<LUT991H_SPEC>`"]
pub type LUT991H = crate::Reg<lut991h::LUT991H_SPEC>;
#[doc = "Graphic MMU LUT entry 991 high"]
pub mod lut991h;
#[doc = "LUT992H register accessor: an alias for `Reg<LUT992H_SPEC>`"]
pub type LUT992H = crate::Reg<lut992h::LUT992H_SPEC>;
#[doc = "Graphic MMU LUT entry 992 high"]
pub mod lut992h;
#[doc = "LUT993H register accessor: an alias for `Reg<LUT993H_SPEC>`"]
pub type LUT993H = crate::Reg<lut993h::LUT993H_SPEC>;
#[doc = "Graphic MMU LUT entry 993 high"]
pub mod lut993h;
#[doc = "LUT994H register accessor: an alias for `Reg<LUT994H_SPEC>`"]
pub type LUT994H = crate::Reg<lut994h::LUT994H_SPEC>;
#[doc = "Graphic MMU LUT entry 994 high"]
pub mod lut994h;
#[doc = "LUT995H register accessor: an alias for `Reg<LUT995H_SPEC>`"]
pub type LUT995H = crate::Reg<lut995h::LUT995H_SPEC>;
#[doc = "Graphic MMU LUT entry 995 high"]
pub mod lut995h;
#[doc = "LUT996H register accessor: an alias for `Reg<LUT996H_SPEC>`"]
pub type LUT996H = crate::Reg<lut996h::LUT996H_SPEC>;
#[doc = "Graphic MMU LUT entry 996 high"]
pub mod lut996h;
#[doc = "LUT997H register accessor: an alias for `Reg<LUT997H_SPEC>`"]
pub type LUT997H = crate::Reg<lut997h::LUT997H_SPEC>;
#[doc = "Graphic MMU LUT entry 997 high"]
pub mod lut997h;
#[doc = "LUT998H register accessor: an alias for `Reg<LUT998H_SPEC>`"]
pub type LUT998H = crate::Reg<lut998h::LUT998H_SPEC>;
#[doc = "Graphic MMU LUT entry 998 high"]
pub mod lut998h;
#[doc = "LUT999H register accessor: an alias for `Reg<LUT999H_SPEC>`"]
pub type LUT999H = crate::Reg<lut999h::LUT999H_SPEC>;
#[doc = "Graphic MMU LUT entry 999 high"]
pub mod lut999h;
#[doc = "LUT1000H register accessor: an alias for `Reg<LUT1000H_SPEC>`"]
pub type LUT1000H = crate::Reg<lut1000h::LUT1000H_SPEC>;
#[doc = "Graphic MMU LUT entry 1000 high"]
pub mod lut1000h;
#[doc = "LUT1001H register accessor: an alias for `Reg<LUT1001H_SPEC>`"]
pub type LUT1001H = crate::Reg<lut1001h::LUT1001H_SPEC>;
#[doc = "Graphic MMU LUT entry 1001 high"]
pub mod lut1001h;
#[doc = "LUT1002H register accessor: an alias for `Reg<LUT1002H_SPEC>`"]
pub type LUT1002H = crate::Reg<lut1002h::LUT1002H_SPEC>;
#[doc = "Graphic MMU LUT entry 1002 high"]
pub mod lut1002h;
#[doc = "LUT1003H register accessor: an alias for `Reg<LUT1003H_SPEC>`"]
pub type LUT1003H = crate::Reg<lut1003h::LUT1003H_SPEC>;
#[doc = "Graphic MMU LUT entry 1003 high"]
pub mod lut1003h;
#[doc = "LUT1004H register accessor: an alias for `Reg<LUT1004H_SPEC>`"]
pub type LUT1004H = crate::Reg<lut1004h::LUT1004H_SPEC>;
#[doc = "Graphic MMU LUT entry 1004 high"]
pub mod lut1004h;
#[doc = "LUT1005H register accessor: an alias for `Reg<LUT1005H_SPEC>`"]
pub type LUT1005H = crate::Reg<lut1005h::LUT1005H_SPEC>;
#[doc = "Graphic MMU LUT entry 1005 high"]
pub mod lut1005h;
#[doc = "LUT1006H register accessor: an alias for `Reg<LUT1006H_SPEC>`"]
pub type LUT1006H = crate::Reg<lut1006h::LUT1006H_SPEC>;
#[doc = "Graphic MMU LUT entry 1006 high"]
pub mod lut1006h;
#[doc = "LUT1007H register accessor: an alias for `Reg<LUT1007H_SPEC>`"]
pub type LUT1007H = crate::Reg<lut1007h::LUT1007H_SPEC>;
#[doc = "Graphic MMU LUT entry 1007 high"]
pub mod lut1007h;
#[doc = "LUT1008H register accessor: an alias for `Reg<LUT1008H_SPEC>`"]
pub type LUT1008H = crate::Reg<lut1008h::LUT1008H_SPEC>;
#[doc = "Graphic MMU LUT entry 1008 high"]
pub mod lut1008h;
#[doc = "LUT1009H register accessor: an alias for `Reg<LUT1009H_SPEC>`"]
pub type LUT1009H = crate::Reg<lut1009h::LUT1009H_SPEC>;
#[doc = "Graphic MMU LUT entry 1009 high"]
pub mod lut1009h;
#[doc = "LUT1010H register accessor: an alias for `Reg<LUT1010H_SPEC>`"]
pub type LUT1010H = crate::Reg<lut1010h::LUT1010H_SPEC>;
#[doc = "Graphic MMU LUT entry 1010 high"]
pub mod lut1010h;
#[doc = "LUT1011H register accessor: an alias for `Reg<LUT1011H_SPEC>`"]
pub type LUT1011H = crate::Reg<lut1011h::LUT1011H_SPEC>;
#[doc = "Graphic MMU LUT entry 1011 high"]
pub mod lut1011h;
#[doc = "LUT1012H register accessor: an alias for `Reg<LUT1012H_SPEC>`"]
pub type LUT1012H = crate::Reg<lut1012h::LUT1012H_SPEC>;
#[doc = "Graphic MMU LUT entry 1012 high"]
pub mod lut1012h;
#[doc = "LUT1013H register accessor: an alias for `Reg<LUT1013H_SPEC>`"]
pub type LUT1013H = crate::Reg<lut1013h::LUT1013H_SPEC>;
#[doc = "Graphic MMU LUT entry 1013 high"]
pub mod lut1013h;
#[doc = "LUT1014H register accessor: an alias for `Reg<LUT1014H_SPEC>`"]
pub type LUT1014H = crate::Reg<lut1014h::LUT1014H_SPEC>;
#[doc = "Graphic MMU LUT entry 1014 high"]
pub mod lut1014h;
#[doc = "LUT1015H register accessor: an alias for `Reg<LUT1015H_SPEC>`"]
pub type LUT1015H = crate::Reg<lut1015h::LUT1015H_SPEC>;
#[doc = "Graphic MMU LUT entry 1015 high"]
pub mod lut1015h;
#[doc = "LUT1016H register accessor: an alias for `Reg<LUT1016H_SPEC>`"]
pub type LUT1016H = crate::Reg<lut1016h::LUT1016H_SPEC>;
#[doc = "Graphic MMU LUT entry 1016 high"]
pub mod lut1016h;
#[doc = "LUT1017H register accessor: an alias for `Reg<LUT1017H_SPEC>`"]
pub type LUT1017H = crate::Reg<lut1017h::LUT1017H_SPEC>;
#[doc = "Graphic MMU LUT entry 1017 high"]
pub mod lut1017h;
#[doc = "LUT1018H register accessor: an alias for `Reg<LUT1018H_SPEC>`"]
pub type LUT1018H = crate::Reg<lut1018h::LUT1018H_SPEC>;
#[doc = "Graphic MMU LUT entry 1018 high"]
pub mod lut1018h;
#[doc = "LUT1019H register accessor: an alias for `Reg<LUT1019H_SPEC>`"]
pub type LUT1019H = crate::Reg<lut1019h::LUT1019H_SPEC>;
#[doc = "Graphic MMU LUT entry 1019 high"]
pub mod lut1019h;
#[doc = "LUT1020H register accessor: an alias for `Reg<LUT1020H_SPEC>`"]
pub type LUT1020H = crate::Reg<lut1020h::LUT1020H_SPEC>;
#[doc = "Graphic MMU LUT entry 1020 high"]
pub mod lut1020h;
#[doc = "LUT1021H register accessor: an alias for `Reg<LUT1021H_SPEC>`"]
pub type LUT1021H = crate::Reg<lut1021h::LUT1021H_SPEC>;
#[doc = "Graphic MMU LUT entry 1021 high"]
pub mod lut1021h;
#[doc = "LUT1022H register accessor: an alias for `Reg<LUT1022H_SPEC>`"]
pub type LUT1022H = crate::Reg<lut1022h::LUT1022H_SPEC>;
#[doc = "Graphic MMU LUT entry 1022 high"]
pub mod lut1022h;
#[doc = "LUT1023H register accessor: an alias for `Reg<LUT1023H_SPEC>`"]
pub type LUT1023H = crate::Reg<lut1023h::LUT1023H_SPEC>;
#[doc = "Graphic MMU LUT entry 1023 high"]
pub mod lut1023h;
