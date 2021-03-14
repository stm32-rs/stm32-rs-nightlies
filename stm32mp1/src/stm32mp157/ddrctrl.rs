#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DDRCTRL master register 0"]
    pub ddrctrl_mstr: DDRCTRL_MSTR,
    #[doc = "0x04 - DDRCTRL operating mode status register"]
    pub ddrctrl_stat: DDRCTRL_STAT,
    _reserved2: [u8; 8usize],
    #[doc = "0x10 - Mode Register Read/Write Control Register 0. Do not enable more than one of the following fields simultaneously: sw_init_int pda_en mpr_en"]
    pub ddrctrl_mrctrl0: DDRCTRL_MRCTRL0,
    #[doc = "0x14 - DDRCTRL mode register read/write control register 1"]
    pub ddrctrl_mrctrl1: DDRCTRL_MRCTRL1,
    #[doc = "0x18 - DDRCTRL mode register read/write status register"]
    pub ddrctrl_mrstat: DDRCTRL_MRSTAT,
    _reserved5: [u8; 4usize],
    #[doc = "0x20 - DDRCTRL temperature derate enable register"]
    pub ddrctrl_derateen: DDRCTRL_DERATEEN,
    #[doc = "0x24 - DDRCTRL temperature derate interval register"]
    pub ddrctrl_derateint: DDRCTRL_DERATEINT,
    _reserved7: [u8; 8usize],
    #[doc = "0x30 - DDRCTRL low power control register"]
    pub ddrctrl_pwrctl: DDRCTRL_PWRCTL,
    #[doc = "0x34 - DDRCTRL low power timing register"]
    pub ddrctrl_pwrtmg: DDRCTRL_PWRTMG,
    #[doc = "0x38 - DDRCTRL hardware low power control register"]
    pub ddrctrl_hwlpctl: DDRCTRL_HWLPCTL,
    _reserved10: [u8; 20usize],
    #[doc = "0x50 - DDRCTRL refresh control register 0"]
    pub ddrctrl_rfshctl0: DDRCTRL_RFSHCTL0,
    _reserved11: [u8; 12usize],
    #[doc = "0x60 - DDRCTRL refresh control register 3"]
    pub ddrctrl_rfshctl3: DDRCTRL_RFSHCTL3,
    #[doc = "0x64 - DDRCTRL refresh timing register"]
    pub ddrctrl_rfshtmg: DDRCTRL_RFSHTMG,
    _reserved13: [u8; 88usize],
    #[doc = "0xc0 - DDRCTRL CRC parity control register 0"]
    pub ddrctrl_crcparctl0: DDRCTRL_CRCPARCTL0,
    _reserved14: [u8; 8usize],
    #[doc = "0xcc - DDRCTRL CRC parity status register"]
    pub ddrctrl_crcparstat: DDRCTRL_CRCPARSTAT,
    #[doc = "0xd0 - DDRCTRL SDRAM initialization register 0"]
    pub ddrctrl_init0: DDRCTRL_INIT0,
    #[doc = "0xd4 - DDRCTRL SDRAM initialization register 1"]
    pub ddrctrl_init1: DDRCTRL_INIT1,
    #[doc = "0xd8 - DDRCTRL SDRAM initialization register 2"]
    pub ddrctrl_init2: DDRCTRL_INIT2,
    #[doc = "0xdc - DDRCTRL SDRAM initialization register 3"]
    pub ddrctrl_init3: DDRCTRL_INIT3,
    #[doc = "0xe0 - DDRCTRL SDRAM initialization register 4"]
    pub ddrctrl_init4: DDRCTRL_INIT4,
    #[doc = "0xe4 - DDRCTRL SDRAM initialization register 5"]
    pub ddrctrl_init5: DDRCTRL_INIT5,
    _reserved21: [u8; 8usize],
    #[doc = "0xf0 - DDRCTRL DIMM control register"]
    pub ddrctrl_dimmctl: DDRCTRL_DIMMCTL,
    _reserved22: [u8; 12usize],
    #[doc = "0x100 - DDRCTRL SDRAM timing register 0"]
    pub ddrctrl_dramtmg0: DDRCTRL_DRAMTMG0,
    #[doc = "0x104 - DDRCTRL SDRAM timing register 1"]
    pub ddrctrl_dramtmg1: DDRCTRL_DRAMTMG1,
    #[doc = "0x108 - DDRCTRL SDRAM timing register 2"]
    pub ddrctrl_dramtmg2: DDRCTRL_DRAMTMG2,
    #[doc = "0x10c - DDRCTRL SDRAM timing register 3"]
    pub ddrctrl_dramtmg3: DDRCTRL_DRAMTMG3,
    #[doc = "0x110 - DDRCTRL SDRAM timing register 4"]
    pub ddrctrl_dramtmg4: DDRCTRL_DRAMTMG4,
    #[doc = "0x114 - DDRCTRL SDRAM timing register 5"]
    pub ddrctrl_dramtmg5: DDRCTRL_DRAMTMG5,
    #[doc = "0x118 - DDRCTRL SDRAM timing register 6"]
    pub ddrctrl_dramtmg6: DDRCTRL_DRAMTMG6,
    #[doc = "0x11c - DDRCTRL SDRAM timing register 7"]
    pub ddrctrl_dramtmg7: DDRCTRL_DRAMTMG7,
    #[doc = "0x120 - DDRCTRL SDRAM timing register 8"]
    pub ddrctrl_dramtmg8: DDRCTRL_DRAMTMG8,
    _reserved31: [u8; 20usize],
    #[doc = "0x138 - DDRCTRL SDRAM timing register 14"]
    pub ddrctrl_dramtmg14: DDRCTRL_DRAMTMG14,
    #[doc = "0x13c - DDRCTRL SDRAM timing register 15"]
    pub ddrctrl_dramtmg15: DDRCTRL_DRAMTMG15,
    _reserved33: [u8; 64usize],
    #[doc = "0x180 - DDRCTRL ZQ control register 0"]
    pub ddrctrl_zqctl0: DDRCTRL_ZQCTL0,
    #[doc = "0x184 - DDRCTRL ZQ control register 1"]
    pub ddrctrl_zqctl1: DDRCTRL_ZQCTL1,
    #[doc = "0x188 - DDRCTRL ZQ control register 2"]
    pub ddrctrl_zqctl2: DDRCTRL_ZQCTL2,
    #[doc = "0x18c - DDRCTRL ZQ status register"]
    pub ddrctrl_zqstat: DDRCTRL_ZQSTAT,
    #[doc = "0x190 - DDRCTRL DFI timing register 0"]
    pub ddrctrl_dfitmg0: DDRCTRL_DFITMG0,
    #[doc = "0x194 - DDRCTRL DFI timing register 1"]
    pub ddrctrl_dfitmg1: DDRCTRL_DFITMG1,
    #[doc = "0x198 - DDRCTRL low power configuration register 0"]
    pub ddrctrl_dfilpcfg0: DDRCTRL_DFILPCFG0,
    _reserved40: [u8; 4usize],
    #[doc = "0x1a0 - DDRCTRL DFI update register 0"]
    pub ddrctrl_dfiupd0: DDRCTRL_DFIUPD0,
    #[doc = "0x1a4 - DDRCTRL DFI update register 1"]
    pub ddrctrl_dfiupd1: DDRCTRL_DFIUPD1,
    #[doc = "0x1a8 - DDRCTRL DFI update register 2"]
    pub ddrctrl_dfiupd2: DDRCTRL_DFIUPD2,
    _reserved43: [u8; 4usize],
    #[doc = "0x1b0 - DDRCTRL DFI miscellaneous control register"]
    pub ddrctrl_dfimisc: DDRCTRL_DFIMISC,
    _reserved44: [u8; 8usize],
    #[doc = "0x1bc - DDRCTRL DFI status register"]
    pub ddrctrl_dfistat: DDRCTRL_DFISTAT,
    _reserved45: [u8; 4usize],
    #[doc = "0x1c4 - DDRCTRL DFI PHY master register"]
    pub ddrctrl_dfiphymstr: DDRCTRL_DFIPHYMSTR,
    _reserved46: [u8; 60usize],
    #[doc = "0x204 - DDRCTRL address map register 1"]
    pub ddrctrl_addrmap1: DDRCTRL_ADDRMAP1,
    #[doc = "0x208 - DDRCTRL address map register 2"]
    pub ddrctrl_addrmap2: DDRCTRL_ADDRMAP2,
    #[doc = "0x20c - DDRCTRL address map register 3"]
    pub ddrctrl_addrmap3: DDRCTRL_ADDRMAP3,
    #[doc = "0x210 - DDRCTRL address map register 4"]
    pub ddrctrl_addrmap4: DDRCTRL_ADDRMAP4,
    #[doc = "0x214 - DDRCTRL address map register 5"]
    pub ddrctrl_addrmap5: DDRCTRL_ADDRMAP5,
    #[doc = "0x218 - DDRCTRL address register 6"]
    pub ddrctrl_addrmap6: DDRCTRL_ADDRMAP6,
    _reserved52: [u8; 8usize],
    #[doc = "0x224 - DDRCTRL address map register 9"]
    pub ddrctrl_addrmap9: DDRCTRL_ADDRMAP9,
    #[doc = "0x228 - DDRCTRL address map register 10"]
    pub ddrctrl_addrmap10: DDRCTRL_ADDRMAP10,
    #[doc = "0x22c - DDRCTRL address map register 11"]
    pub ddrctrl_addrmap11: DDRCTRL_ADDRMAP11,
    _reserved55: [u8; 16usize],
    #[doc = "0x240 - DDRCTRL ODT configuration register"]
    pub ddrctrl_odtcfg: DDRCTRL_ODTCFG,
    #[doc = "0x244 - DDRCTRL ODT/Rank map register"]
    pub ddrctrl_odtmap: DDRCTRL_ODTMAP,
    _reserved57: [u8; 8usize],
    #[doc = "0x250 - DDRCTRL scheduler control register"]
    pub ddrctrl_sched: DDRCTRL_SCHED,
    #[doc = "0x254 - DDRCTRL scheduler control register 1"]
    pub ddrctrl_sched1: DDRCTRL_SCHED1,
    _reserved59: [u8; 4usize],
    #[doc = "0x25c - DDRCTRL high priority read CAM register 1"]
    pub ddrctrl_perfhpr1: DDRCTRL_PERFHPR1,
    _reserved60: [u8; 4usize],
    #[doc = "0x264 - DDRCTRL low priority read CAM register 1"]
    pub ddrctrl_perflpr1: DDRCTRL_PERFLPR1,
    _reserved61: [u8; 4usize],
    #[doc = "0x26c - DDRCTRL write CAM register 1"]
    pub ddrctrl_perfwr1: DDRCTRL_PERFWR1,
    _reserved62: [u8; 144usize],
    #[doc = "0x300 - DDRCTRL debug register 0"]
    pub ddrctrl_dbg0: DDRCTRL_DBG0,
    #[doc = "0x304 - DDRCTRL debug register 1"]
    pub ddrctrl_dbg1: DDRCTRL_DBG1,
    #[doc = "0x308 - DDRCTRL CAM debug register"]
    pub ddrctrl_dbgcam: DDRCTRL_DBGCAM,
    #[doc = "0x30c - DDRCTRL command debug register"]
    pub ddrctrl_dbgcmd: DDRCTRL_DBGCMD,
    #[doc = "0x310 - DDRCTRL status debug register"]
    pub ddrctrl_dbgstat: DDRCTRL_DBGSTAT,
    _reserved67: [u8; 12usize],
    #[doc = "0x320 - DDRCTRL software register programming control enable"]
    pub ddrctrl_swctl: DDRCTRL_SWCTL,
    #[doc = "0x324 - DDRCTRL software register programming control status"]
    pub ddrctrl_swstat: DDRCTRL_SWSTAT,
    _reserved69: [u8; 68usize],
    #[doc = "0x36c - AXI Poison configuration register common for all AXI ports."]
    pub ddrctrl_poisoncfg: DDRCTRL_POISONCFG,
    #[doc = "0x370 - DDRCTRL AXI Poison status register"]
    pub ddrctrl_poisonstat: DDRCTRL_POISONSTAT,
    _reserved71: [u8; 136usize],
    #[doc = "0x3fc - DDRCTRL port status register"]
    pub ddrctrl_pstat: DDRCTRL_PSTAT,
    #[doc = "0x400 - DDRCTRL port common configuration register"]
    pub ddrctrl_pccfg: DDRCTRL_PCCFG,
    #[doc = "0x404 - DDRCTRL port 0 configuration read register"]
    pub ddrctrl_pcfgr_0: DDRCTRL_PCFGR_0,
    #[doc = "0x408 - DDRCTRL port 0 configuration write register"]
    pub ddrctrl_pcfgw_0: DDRCTRL_PCFGW_0,
    _reserved75: [u8; 132usize],
    #[doc = "0x490 - DDRCTRL port 0 control register"]
    pub ddrctrl_pctrl_0: DDRCTRL_PCTRL_0,
    #[doc = "0x494 - DDRCTRL port 0 read Q0S configuration register 0"]
    pub ddrctrl_pcfgqos0_0: DDRCTRL_PCFGQOS0_0,
    #[doc = "0x498 - DDRCTRL port 0 read Q0S configuration register 1"]
    pub ddrctrl_pcfgqos1_0: DDRCTRL_PCFGQOS1_0,
    #[doc = "0x49c - DDRCTRL port 0 write Q0S configuration register 0"]
    pub ddrctrl_pcfgwqos0_0: DDRCTRL_PCFGWQOS0_0,
    #[doc = "0x4a0 - DDRCTRL port 0 write Q0S configuration register 1"]
    pub ddrctrl_pcfgwqos1_0: DDRCTRL_PCFGWQOS1_0,
    _reserved80: [u8; 16usize],
    #[doc = "0x4b4 - DDRCTRL port 1 configuration read register"]
    pub ddrctrl_pcfgr_1: DDRCTRL_PCFGR_1,
    #[doc = "0x4b8 - DDRCTRL port 1 configuration write register"]
    pub ddrctrl_pcfgw_1: DDRCTRL_PCFGW_1,
    _reserved82: [u8; 132usize],
    #[doc = "0x540 - DDRCTRL port 1 control register"]
    pub ddrctrl_pctrl_1: DDRCTRL_PCTRL_1,
    #[doc = "0x544 - DDRCTRL port 1 read Q0S configuration register 0"]
    pub ddrctrl_pcfgqos0_1: DDRCTRL_PCFGQOS0_1,
    #[doc = "0x548 - DDRCTRL port 1 read Q0S configuration register 1"]
    pub ddrctrl_pcfgqos1_1: DDRCTRL_PCFGQOS1_1,
    #[doc = "0x54c - DDRCTRL port 1 write Q0S configuration register 0"]
    pub ddrctrl_pcfgwqos0_1: DDRCTRL_PCFGWQOS0_1,
    #[doc = "0x550 - DDRCTRL port 1 write Q0S configuration register 1"]
    pub ddrctrl_pcfgwqos1_1: DDRCTRL_PCFGWQOS1_1,
}
#[doc = "DDRCTRL master register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_mstr](ddrctrl_mstr) module"]
pub type DDRCTRL_MSTR = crate::Reg<u32, _DDRCTRL_MSTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_MSTR;
#[doc = "`read()` method returns [ddrctrl_mstr::R](ddrctrl_mstr::R) reader structure"]
impl crate::Readable for DDRCTRL_MSTR {}
#[doc = "`write(|w| ..)` method takes [ddrctrl_mstr::W](ddrctrl_mstr::W) writer structure"]
impl crate::Writable for DDRCTRL_MSTR {}
#[doc = "DDRCTRL master register 0"]
pub mod ddrctrl_mstr;
#[doc = "DDRCTRL operating mode status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_stat](ddrctrl_stat) module"]
pub type DDRCTRL_STAT = crate::Reg<u32, _DDRCTRL_STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_STAT;
#[doc = "`read()` method returns [ddrctrl_stat::R](ddrctrl_stat::R) reader structure"]
impl crate::Readable for DDRCTRL_STAT {}
#[doc = "DDRCTRL operating mode status register"]
pub mod ddrctrl_stat;
#[doc = "Mode Register Read/Write Control Register 0. Do not enable more than one of the following fields simultaneously: sw_init_int pda_en mpr_en\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_mrctrl0](ddrctrl_mrctrl0) module"]
pub type DDRCTRL_MRCTRL0 = crate::Reg<u32, _DDRCTRL_MRCTRL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_MRCTRL0;
#[doc = "`read()` method returns [ddrctrl_mrctrl0::R](ddrctrl_mrctrl0::R) reader structure"]
impl crate::Readable for DDRCTRL_MRCTRL0 {}
#[doc = "`write(|w| ..)` method takes [ddrctrl_mrctrl0::W](ddrctrl_mrctrl0::W) writer structure"]
impl crate::Writable for DDRCTRL_MRCTRL0 {}
#[doc = "Mode Register Read/Write Control Register 0. Do not enable more than one of the following fields simultaneously: sw_init_int pda_en mpr_en"]
pub mod ddrctrl_mrctrl0;
#[doc = "DDRCTRL mode register read/write control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_mrctrl1](ddrctrl_mrctrl1) module"]
pub type DDRCTRL_MRCTRL1 = crate::Reg<u32, _DDRCTRL_MRCTRL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_MRCTRL1;
#[doc = "`read()` method returns [ddrctrl_mrctrl1::R](ddrctrl_mrctrl1::R) reader structure"]
impl crate::Readable for DDRCTRL_MRCTRL1 {}
#[doc = "`write(|w| ..)` method takes [ddrctrl_mrctrl1::W](ddrctrl_mrctrl1::W) writer structure"]
impl crate::Writable for DDRCTRL_MRCTRL1 {}
#[doc = "DDRCTRL mode register read/write control register 1"]
pub mod ddrctrl_mrctrl1;
#[doc = "DDRCTRL mode register read/write status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_mrstat](ddrctrl_mrstat) module"]
pub type DDRCTRL_MRSTAT = crate::Reg<u32, _DDRCTRL_MRSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_MRSTAT;
#[doc = "`read()` method returns [ddrctrl_mrstat::R](ddrctrl_mrstat::R) reader structure"]
impl crate::Readable for DDRCTRL_MRSTAT {}
#[doc = "DDRCTRL mode register read/write status register"]
pub mod ddrctrl_mrstat;
#[doc = "DDRCTRL temperature derate enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_derateen](ddrctrl_derateen) module"]
pub type DDRCTRL_DERATEEN = crate::Reg<u32, _DDRCTRL_DERATEEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_DERATEEN;
#[doc = "`read()` method returns [ddrctrl_derateen::R](ddrctrl_derateen::R) reader structure"]
impl crate::Readable for DDRCTRL_DERATEEN {}
#[doc = "`write(|w| ..)` method takes [ddrctrl_derateen::W](ddrctrl_derateen::W) writer structure"]
impl crate::Writable for DDRCTRL_DERATEEN {}
#[doc = "DDRCTRL temperature derate enable register"]
pub mod ddrctrl_derateen;
#[doc = "DDRCTRL temperature derate interval register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_derateint](ddrctrl_derateint) module"]
pub type DDRCTRL_DERATEINT = crate::Reg<u32, _DDRCTRL_DERATEINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_DERATEINT;
#[doc = "`read()` method returns [ddrctrl_derateint::R](ddrctrl_derateint::R) reader structure"]
impl crate::Readable for DDRCTRL_DERATEINT {}
#[doc = "`write(|w| ..)` method takes [ddrctrl_derateint::W](ddrctrl_derateint::W) writer structure"]
impl crate::Writable for DDRCTRL_DERATEINT {}
#[doc = "DDRCTRL temperature derate interval register"]
pub mod ddrctrl_derateint;
#[doc = "DDRCTRL low power control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_pwrctl](ddrctrl_pwrctl) module"]
pub type DDRCTRL_PWRCTL = crate::Reg<u32, _DDRCTRL_PWRCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_PWRCTL;
#[doc = "`read()` method returns [ddrctrl_pwrctl::R](ddrctrl_pwrctl::R) reader structure"]
impl crate::Readable for DDRCTRL_PWRCTL {}
#[doc = "`write(|w| ..)` method takes [ddrctrl_pwrctl::W](ddrctrl_pwrctl::W) writer structure"]
impl crate::Writable for DDRCTRL_PWRCTL {}
#[doc = "DDRCTRL low power control register"]
pub mod ddrctrl_pwrctl;
#[doc = "DDRCTRL low power timing register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_pwrtmg](ddrctrl_pwrtmg) module"]
pub type DDRCTRL_PWRTMG = crate::Reg<u32, _DDRCTRL_PWRTMG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_PWRTMG;
#[doc = "`read()` method returns [ddrctrl_pwrtmg::R](ddrctrl_pwrtmg::R) reader structure"]
impl crate::Readable for DDRCTRL_PWRTMG {}
#[doc = "`write(|w| ..)` method takes [ddrctrl_pwrtmg::W](ddrctrl_pwrtmg::W) writer structure"]
impl crate::Writable for DDRCTRL_PWRTMG {}
#[doc = "DDRCTRL low power timing register"]
pub mod ddrctrl_pwrtmg;
#[doc = "DDRCTRL hardware low power control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_hwlpctl](ddrctrl_hwlpctl) module"]
pub type DDRCTRL_HWLPCTL = crate::Reg<u32, _DDRCTRL_HWLPCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_HWLPCTL;
#[doc = "`read()` method returns [ddrctrl_hwlpctl::R](ddrctrl_hwlpctl::R) reader structure"]
impl crate::Readable for DDRCTRL_HWLPCTL {}
#[doc = "`write(|w| ..)` method takes [ddrctrl_hwlpctl::W](ddrctrl_hwlpctl::W) writer structure"]
impl crate::Writable for DDRCTRL_HWLPCTL {}
#[doc = "DDRCTRL hardware low power control register"]
pub mod ddrctrl_hwlpctl;
#[doc = "DDRCTRL refresh control register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_rfshctl0](ddrctrl_rfshctl0) module"]
pub type DDRCTRL_RFSHCTL0 = crate::Reg<u32, _DDRCTRL_RFSHCTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_RFSHCTL0;
#[doc = "`read()` method returns [ddrctrl_rfshctl0::R](ddrctrl_rfshctl0::R) reader structure"]
impl crate::Readable for DDRCTRL_RFSHCTL0 {}
#[doc = "`write(|w| ..)` method takes [ddrctrl_rfshctl0::W](ddrctrl_rfshctl0::W) writer structure"]
impl crate::Writable for DDRCTRL_RFSHCTL0 {}
#[doc = "DDRCTRL refresh control register 0"]
pub mod ddrctrl_rfshctl0;
#[doc = "DDRCTRL refresh control register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_rfshctl3](ddrctrl_rfshctl3) module"]
pub type DDRCTRL_RFSHCTL3 = crate::Reg<u32, _DDRCTRL_RFSHCTL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_RFSHCTL3;
#[doc = "`read()` method returns [ddrctrl_rfshctl3::R](ddrctrl_rfshctl3::R) reader structure"]
impl crate::Readable for DDRCTRL_RFSHCTL3 {}
#[doc = "`write(|w| ..)` method takes [ddrctrl_rfshctl3::W](ddrctrl_rfshctl3::W) writer structure"]
impl crate::Writable for DDRCTRL_RFSHCTL3 {}
#[doc = "DDRCTRL refresh control register 3"]
pub mod ddrctrl_rfshctl3;
#[doc = "DDRCTRL refresh timing register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_rfshtmg](ddrctrl_rfshtmg) module"]
pub type DDRCTRL_RFSHTMG = crate::Reg<u32, _DDRCTRL_RFSHTMG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_RFSHTMG;
#[doc = "`read()` method returns [ddrctrl_rfshtmg::R](ddrctrl_rfshtmg::R) reader structure"]
impl crate::Readable for DDRCTRL_RFSHTMG {}
#[doc = "`write(|w| ..)` method takes [ddrctrl_rfshtmg::W](ddrctrl_rfshtmg::W) writer structure"]
impl crate::Writable for DDRCTRL_RFSHTMG {}
#[doc = "DDRCTRL refresh timing register"]
pub mod ddrctrl_rfshtmg;
#[doc = "DDRCTRL CRC parity control register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_crcparctl0](ddrctrl_crcparctl0) module"]
pub type DDRCTRL_CRCPARCTL0 = crate::Reg<u32, _DDRCTRL_CRCPARCTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_CRCPARCTL0;
#[doc = "`read()` method returns [ddrctrl_crcparctl0::R](ddrctrl_crcparctl0::R) reader structure"]
impl crate::Readable for DDRCTRL_CRCPARCTL0 {}
#[doc = "`write(|w| ..)` method takes [ddrctrl_crcparctl0::W](ddrctrl_crcparctl0::W) writer structure"]
impl crate::Writable for DDRCTRL_CRCPARCTL0 {}
#[doc = "DDRCTRL CRC parity control register 0"]
pub mod ddrctrl_crcparctl0;
#[doc = "DDRCTRL CRC parity status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_crcparstat](ddrctrl_crcparstat) module"]
pub type DDRCTRL_CRCPARSTAT = crate::Reg<u32, _DDRCTRL_CRCPARSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_CRCPARSTAT;
#[doc = "`read()` method returns [ddrctrl_crcparstat::R](ddrctrl_crcparstat::R) reader structure"]
impl crate::Readable for DDRCTRL_CRCPARSTAT {}
#[doc = "DDRCTRL CRC parity status register"]
pub mod ddrctrl_crcparstat;
#[doc = "DDRCTRL SDRAM initialization register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_init0](ddrctrl_init0) module"]
pub type DDRCTRL_INIT0 = crate::Reg<u32, _DDRCTRL_INIT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_INIT0;
#[doc = "`read()` method returns [ddrctrl_init0::R](ddrctrl_init0::R) reader structure"]
impl crate::Readable for DDRCTRL_INIT0 {}
#[doc = "`write(|w| ..)` method takes [ddrctrl_init0::W](ddrctrl_init0::W) writer structure"]
impl crate::Writable for DDRCTRL_INIT0 {}
#[doc = "DDRCTRL SDRAM initialization register 0"]
pub mod ddrctrl_init0;
#[doc = "DDRCTRL SDRAM initialization register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_init1](ddrctrl_init1) module"]
pub type DDRCTRL_INIT1 = crate::Reg<u32, _DDRCTRL_INIT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_INIT1;
#[doc = "`read()` method returns [ddrctrl_init1::R](ddrctrl_init1::R) reader structure"]
impl crate::Readable for DDRCTRL_INIT1 {}
#[doc = "`write(|w| ..)` method takes [ddrctrl_init1::W](ddrctrl_init1::W) writer structure"]
impl crate::Writable for DDRCTRL_INIT1 {}
#[doc = "DDRCTRL SDRAM initialization register 1"]
pub mod ddrctrl_init1;
#[doc = "DDRCTRL SDRAM initialization register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_init2](ddrctrl_init2) module"]
pub type DDRCTRL_INIT2 = crate::Reg<u32, _DDRCTRL_INIT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_INIT2;
#[doc = "`read()` method returns [ddrctrl_init2::R](ddrctrl_init2::R) reader structure"]
impl crate::Readable for DDRCTRL_INIT2 {}
#[doc = "`write(|w| ..)` method takes [ddrctrl_init2::W](ddrctrl_init2::W) writer structure"]
impl crate::Writable for DDRCTRL_INIT2 {}
#[doc = "DDRCTRL SDRAM initialization register 2"]
pub mod ddrctrl_init2;
#[doc = "DDRCTRL SDRAM initialization register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_init3](ddrctrl_init3) module"]
pub type DDRCTRL_INIT3 = crate::Reg<u32, _DDRCTRL_INIT3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_INIT3;
#[doc = "`read()` method returns [ddrctrl_init3::R](ddrctrl_init3::R) reader structure"]
impl crate::Readable for DDRCTRL_INIT3 {}
#[doc = "`write(|w| ..)` method takes [ddrctrl_init3::W](ddrctrl_init3::W) writer structure"]
impl crate::Writable for DDRCTRL_INIT3 {}
#[doc = "DDRCTRL SDRAM initialization register 3"]
pub mod ddrctrl_init3;
#[doc = "DDRCTRL SDRAM initialization register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_init4](ddrctrl_init4) module"]
pub type DDRCTRL_INIT4 = crate::Reg<u32, _DDRCTRL_INIT4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_INIT4;
#[doc = "`read()` method returns [ddrctrl_init4::R](ddrctrl_init4::R) reader structure"]
impl crate::Readable for DDRCTRL_INIT4 {}
#[doc = "`write(|w| ..)` method takes [ddrctrl_init4::W](ddrctrl_init4::W) writer structure"]
impl crate::Writable for DDRCTRL_INIT4 {}
#[doc = "DDRCTRL SDRAM initialization register 4"]
pub mod ddrctrl_init4;
#[doc = "DDRCTRL SDRAM initialization register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_init5](ddrctrl_init5) module"]
pub type DDRCTRL_INIT5 = crate::Reg<u32, _DDRCTRL_INIT5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_INIT5;
#[doc = "`read()` method returns [ddrctrl_init5::R](ddrctrl_init5::R) reader structure"]
impl crate::Readable for DDRCTRL_INIT5 {}
#[doc = "`write(|w| ..)` method takes [ddrctrl_init5::W](ddrctrl_init5::W) writer structure"]
impl crate::Writable for DDRCTRL_INIT5 {}
#[doc = "DDRCTRL SDRAM initialization register 5"]
pub mod ddrctrl_init5;
#[doc = "DDRCTRL DIMM control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_dimmctl](ddrctrl_dimmctl) module"]
pub type DDRCTRL_DIMMCTL = crate::Reg<u32, _DDRCTRL_DIMMCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_DIMMCTL;
#[doc = "`read()` method returns [ddrctrl_dimmctl::R](ddrctrl_dimmctl::R) reader structure"]
impl crate::Readable for DDRCTRL_DIMMCTL {}
#[doc = "`write(|w| ..)` method takes [ddrctrl_dimmctl::W](ddrctrl_dimmctl::W) writer structure"]
impl crate::Writable for DDRCTRL_DIMMCTL {}
#[doc = "DDRCTRL DIMM control register"]
pub mod ddrctrl_dimmctl;
#[doc = "DDRCTRL SDRAM timing register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_dramtmg0](ddrctrl_dramtmg0) module"]
pub type DDRCTRL_DRAMTMG0 = crate::Reg<u32, _DDRCTRL_DRAMTMG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_DRAMTMG0;
#[doc = "`read()` method returns [ddrctrl_dramtmg0::R](ddrctrl_dramtmg0::R) reader structure"]
impl crate::Readable for DDRCTRL_DRAMTMG0 {}
#[doc = "`write(|w| ..)` method takes [ddrctrl_dramtmg0::W](ddrctrl_dramtmg0::W) writer structure"]
impl crate::Writable for DDRCTRL_DRAMTMG0 {}
#[doc = "DDRCTRL SDRAM timing register 0"]
pub mod ddrctrl_dramtmg0;
#[doc = "DDRCTRL SDRAM timing register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_dramtmg1](ddrctrl_dramtmg1) module"]
pub type DDRCTRL_DRAMTMG1 = crate::Reg<u32, _DDRCTRL_DRAMTMG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_DRAMTMG1;
#[doc = "`read()` method returns [ddrctrl_dramtmg1::R](ddrctrl_dramtmg1::R) reader structure"]
impl crate::Readable for DDRCTRL_DRAMTMG1 {}
#[doc = "`write(|w| ..)` method takes [ddrctrl_dramtmg1::W](ddrctrl_dramtmg1::W) writer structure"]
impl crate::Writable for DDRCTRL_DRAMTMG1 {}
#[doc = "DDRCTRL SDRAM timing register 1"]
pub mod ddrctrl_dramtmg1;
#[doc = "DDRCTRL SDRAM timing register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_dramtmg2](ddrctrl_dramtmg2) module"]
pub type DDRCTRL_DRAMTMG2 = crate::Reg<u32, _DDRCTRL_DRAMTMG2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_DRAMTMG2;
#[doc = "`read()` method returns [ddrctrl_dramtmg2::R](ddrctrl_dramtmg2::R) reader structure"]
impl crate::Readable for DDRCTRL_DRAMTMG2 {}
#[doc = "`write(|w| ..)` method takes [ddrctrl_dramtmg2::W](ddrctrl_dramtmg2::W) writer structure"]
impl crate::Writable for DDRCTRL_DRAMTMG2 {}
#[doc = "DDRCTRL SDRAM timing register 2"]
pub mod ddrctrl_dramtmg2;
#[doc = "DDRCTRL SDRAM timing register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_dramtmg3](ddrctrl_dramtmg3) module"]
pub type DDRCTRL_DRAMTMG3 = crate::Reg<u32, _DDRCTRL_DRAMTMG3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_DRAMTMG3;
#[doc = "`read()` method returns [ddrctrl_dramtmg3::R](ddrctrl_dramtmg3::R) reader structure"]
impl crate::Readable for DDRCTRL_DRAMTMG3 {}
#[doc = "`write(|w| ..)` method takes [ddrctrl_dramtmg3::W](ddrctrl_dramtmg3::W) writer structure"]
impl crate::Writable for DDRCTRL_DRAMTMG3 {}
#[doc = "DDRCTRL SDRAM timing register 3"]
pub mod ddrctrl_dramtmg3;
#[doc = "DDRCTRL SDRAM timing register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_dramtmg4](ddrctrl_dramtmg4) module"]
pub type DDRCTRL_DRAMTMG4 = crate::Reg<u32, _DDRCTRL_DRAMTMG4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_DRAMTMG4;
#[doc = "`read()` method returns [ddrctrl_dramtmg4::R](ddrctrl_dramtmg4::R) reader structure"]
impl crate::Readable for DDRCTRL_DRAMTMG4 {}
#[doc = "`write(|w| ..)` method takes [ddrctrl_dramtmg4::W](ddrctrl_dramtmg4::W) writer structure"]
impl crate::Writable for DDRCTRL_DRAMTMG4 {}
#[doc = "DDRCTRL SDRAM timing register 4"]
pub mod ddrctrl_dramtmg4;
#[doc = "DDRCTRL SDRAM timing register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_dramtmg5](ddrctrl_dramtmg5) module"]
pub type DDRCTRL_DRAMTMG5 = crate::Reg<u32, _DDRCTRL_DRAMTMG5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_DRAMTMG5;
#[doc = "`read()` method returns [ddrctrl_dramtmg5::R](ddrctrl_dramtmg5::R) reader structure"]
impl crate::Readable for DDRCTRL_DRAMTMG5 {}
#[doc = "`write(|w| ..)` method takes [ddrctrl_dramtmg5::W](ddrctrl_dramtmg5::W) writer structure"]
impl crate::Writable for DDRCTRL_DRAMTMG5 {}
#[doc = "DDRCTRL SDRAM timing register 5"]
pub mod ddrctrl_dramtmg5;
#[doc = "DDRCTRL SDRAM timing register 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_dramtmg6](ddrctrl_dramtmg6) module"]
pub type DDRCTRL_DRAMTMG6 = crate::Reg<u32, _DDRCTRL_DRAMTMG6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_DRAMTMG6;
#[doc = "`read()` method returns [ddrctrl_dramtmg6::R](ddrctrl_dramtmg6::R) reader structure"]
impl crate::Readable for DDRCTRL_DRAMTMG6 {}
#[doc = "`write(|w| ..)` method takes [ddrctrl_dramtmg6::W](ddrctrl_dramtmg6::W) writer structure"]
impl crate::Writable for DDRCTRL_DRAMTMG6 {}
#[doc = "DDRCTRL SDRAM timing register 6"]
pub mod ddrctrl_dramtmg6;
#[doc = "DDRCTRL SDRAM timing register 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_dramtmg7](ddrctrl_dramtmg7) module"]
pub type DDRCTRL_DRAMTMG7 = crate::Reg<u32, _DDRCTRL_DRAMTMG7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_DRAMTMG7;
#[doc = "`read()` method returns [ddrctrl_dramtmg7::R](ddrctrl_dramtmg7::R) reader structure"]
impl crate::Readable for DDRCTRL_DRAMTMG7 {}
#[doc = "`write(|w| ..)` method takes [ddrctrl_dramtmg7::W](ddrctrl_dramtmg7::W) writer structure"]
impl crate::Writable for DDRCTRL_DRAMTMG7 {}
#[doc = "DDRCTRL SDRAM timing register 7"]
pub mod ddrctrl_dramtmg7;
#[doc = "DDRCTRL SDRAM timing register 8\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_dramtmg8](ddrctrl_dramtmg8) module"]
pub type DDRCTRL_DRAMTMG8 = crate::Reg<u32, _DDRCTRL_DRAMTMG8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_DRAMTMG8;
#[doc = "`read()` method returns [ddrctrl_dramtmg8::R](ddrctrl_dramtmg8::R) reader structure"]
impl crate::Readable for DDRCTRL_DRAMTMG8 {}
#[doc = "`write(|w| ..)` method takes [ddrctrl_dramtmg8::W](ddrctrl_dramtmg8::W) writer structure"]
impl crate::Writable for DDRCTRL_DRAMTMG8 {}
#[doc = "DDRCTRL SDRAM timing register 8"]
pub mod ddrctrl_dramtmg8;
#[doc = "DDRCTRL SDRAM timing register 14\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_dramtmg14](ddrctrl_dramtmg14) module"]
pub type DDRCTRL_DRAMTMG14 = crate::Reg<u32, _DDRCTRL_DRAMTMG14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_DRAMTMG14;
#[doc = "`read()` method returns [ddrctrl_dramtmg14::R](ddrctrl_dramtmg14::R) reader structure"]
impl crate::Readable for DDRCTRL_DRAMTMG14 {}
#[doc = "`write(|w| ..)` method takes [ddrctrl_dramtmg14::W](ddrctrl_dramtmg14::W) writer structure"]
impl crate::Writable for DDRCTRL_DRAMTMG14 {}
#[doc = "DDRCTRL SDRAM timing register 14"]
pub mod ddrctrl_dramtmg14;
#[doc = "DDRCTRL SDRAM timing register 15\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_dramtmg15](ddrctrl_dramtmg15) module"]
pub type DDRCTRL_DRAMTMG15 = crate::Reg<u32, _DDRCTRL_DRAMTMG15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_DRAMTMG15;
#[doc = "`read()` method returns [ddrctrl_dramtmg15::R](ddrctrl_dramtmg15::R) reader structure"]
impl crate::Readable for DDRCTRL_DRAMTMG15 {}
#[doc = "`write(|w| ..)` method takes [ddrctrl_dramtmg15::W](ddrctrl_dramtmg15::W) writer structure"]
impl crate::Writable for DDRCTRL_DRAMTMG15 {}
#[doc = "DDRCTRL SDRAM timing register 15"]
pub mod ddrctrl_dramtmg15;
#[doc = "DDRCTRL ZQ control register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_zqctl0](ddrctrl_zqctl0) module"]
pub type DDRCTRL_ZQCTL0 = crate::Reg<u32, _DDRCTRL_ZQCTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_ZQCTL0;
#[doc = "`read()` method returns [ddrctrl_zqctl0::R](ddrctrl_zqctl0::R) reader structure"]
impl crate::Readable for DDRCTRL_ZQCTL0 {}
#[doc = "`write(|w| ..)` method takes [ddrctrl_zqctl0::W](ddrctrl_zqctl0::W) writer structure"]
impl crate::Writable for DDRCTRL_ZQCTL0 {}
#[doc = "DDRCTRL ZQ control register 0"]
pub mod ddrctrl_zqctl0;
#[doc = "DDRCTRL ZQ control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_zqctl1](ddrctrl_zqctl1) module"]
pub type DDRCTRL_ZQCTL1 = crate::Reg<u32, _DDRCTRL_ZQCTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_ZQCTL1;
#[doc = "`read()` method returns [ddrctrl_zqctl1::R](ddrctrl_zqctl1::R) reader structure"]
impl crate::Readable for DDRCTRL_ZQCTL1 {}
#[doc = "`write(|w| ..)` method takes [ddrctrl_zqctl1::W](ddrctrl_zqctl1::W) writer structure"]
impl crate::Writable for DDRCTRL_ZQCTL1 {}
#[doc = "DDRCTRL ZQ control register 1"]
pub mod ddrctrl_zqctl1;
#[doc = "DDRCTRL ZQ control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_zqctl2](ddrctrl_zqctl2) module"]
pub type DDRCTRL_ZQCTL2 = crate::Reg<u32, _DDRCTRL_ZQCTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_ZQCTL2;
#[doc = "`read()` method returns [ddrctrl_zqctl2::R](ddrctrl_zqctl2::R) reader structure"]
impl crate::Readable for DDRCTRL_ZQCTL2 {}
#[doc = "`write(|w| ..)` method takes [ddrctrl_zqctl2::W](ddrctrl_zqctl2::W) writer structure"]
impl crate::Writable for DDRCTRL_ZQCTL2 {}
#[doc = "DDRCTRL ZQ control register 2"]
pub mod ddrctrl_zqctl2;
#[doc = "DDRCTRL ZQ status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_zqstat](ddrctrl_zqstat) module"]
pub type DDRCTRL_ZQSTAT = crate::Reg<u32, _DDRCTRL_ZQSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_ZQSTAT;
#[doc = "`read()` method returns [ddrctrl_zqstat::R](ddrctrl_zqstat::R) reader structure"]
impl crate::Readable for DDRCTRL_ZQSTAT {}
#[doc = "DDRCTRL ZQ status register"]
pub mod ddrctrl_zqstat;
#[doc = "DDRCTRL DFI timing register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_dfitmg0](ddrctrl_dfitmg0) module"]
pub type DDRCTRL_DFITMG0 = crate::Reg<u32, _DDRCTRL_DFITMG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_DFITMG0;
#[doc = "`read()` method returns [ddrctrl_dfitmg0::R](ddrctrl_dfitmg0::R) reader structure"]
impl crate::Readable for DDRCTRL_DFITMG0 {}
#[doc = "`write(|w| ..)` method takes [ddrctrl_dfitmg0::W](ddrctrl_dfitmg0::W) writer structure"]
impl crate::Writable for DDRCTRL_DFITMG0 {}
#[doc = "DDRCTRL DFI timing register 0"]
pub mod ddrctrl_dfitmg0;
#[doc = "DDRCTRL DFI timing register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_dfitmg1](ddrctrl_dfitmg1) module"]
pub type DDRCTRL_DFITMG1 = crate::Reg<u32, _DDRCTRL_DFITMG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_DFITMG1;
#[doc = "`read()` method returns [ddrctrl_dfitmg1::R](ddrctrl_dfitmg1::R) reader structure"]
impl crate::Readable for DDRCTRL_DFITMG1 {}
#[doc = "`write(|w| ..)` method takes [ddrctrl_dfitmg1::W](ddrctrl_dfitmg1::W) writer structure"]
impl crate::Writable for DDRCTRL_DFITMG1 {}
#[doc = "DDRCTRL DFI timing register 1"]
pub mod ddrctrl_dfitmg1;
#[doc = "DDRCTRL low power configuration register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_dfilpcfg0](ddrctrl_dfilpcfg0) module"]
pub type DDRCTRL_DFILPCFG0 = crate::Reg<u32, _DDRCTRL_DFILPCFG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_DFILPCFG0;
#[doc = "`read()` method returns [ddrctrl_dfilpcfg0::R](ddrctrl_dfilpcfg0::R) reader structure"]
impl crate::Readable for DDRCTRL_DFILPCFG0 {}
#[doc = "`write(|w| ..)` method takes [ddrctrl_dfilpcfg0::W](ddrctrl_dfilpcfg0::W) writer structure"]
impl crate::Writable for DDRCTRL_DFILPCFG0 {}
#[doc = "DDRCTRL low power configuration register 0"]
pub mod ddrctrl_dfilpcfg0;
#[doc = "DDRCTRL DFI update register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_dfiupd0](ddrctrl_dfiupd0) module"]
pub type DDRCTRL_DFIUPD0 = crate::Reg<u32, _DDRCTRL_DFIUPD0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_DFIUPD0;
#[doc = "`read()` method returns [ddrctrl_dfiupd0::R](ddrctrl_dfiupd0::R) reader structure"]
impl crate::Readable for DDRCTRL_DFIUPD0 {}
#[doc = "`write(|w| ..)` method takes [ddrctrl_dfiupd0::W](ddrctrl_dfiupd0::W) writer structure"]
impl crate::Writable for DDRCTRL_DFIUPD0 {}
#[doc = "DDRCTRL DFI update register 0"]
pub mod ddrctrl_dfiupd0;
#[doc = "DDRCTRL DFI update register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_dfiupd1](ddrctrl_dfiupd1) module"]
pub type DDRCTRL_DFIUPD1 = crate::Reg<u32, _DDRCTRL_DFIUPD1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_DFIUPD1;
#[doc = "`read()` method returns [ddrctrl_dfiupd1::R](ddrctrl_dfiupd1::R) reader structure"]
impl crate::Readable for DDRCTRL_DFIUPD1 {}
#[doc = "`write(|w| ..)` method takes [ddrctrl_dfiupd1::W](ddrctrl_dfiupd1::W) writer structure"]
impl crate::Writable for DDRCTRL_DFIUPD1 {}
#[doc = "DDRCTRL DFI update register 1"]
pub mod ddrctrl_dfiupd1;
#[doc = "DDRCTRL DFI update register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_dfiupd2](ddrctrl_dfiupd2) module"]
pub type DDRCTRL_DFIUPD2 = crate::Reg<u32, _DDRCTRL_DFIUPD2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_DFIUPD2;
#[doc = "`read()` method returns [ddrctrl_dfiupd2::R](ddrctrl_dfiupd2::R) reader structure"]
impl crate::Readable for DDRCTRL_DFIUPD2 {}
#[doc = "`write(|w| ..)` method takes [ddrctrl_dfiupd2::W](ddrctrl_dfiupd2::W) writer structure"]
impl crate::Writable for DDRCTRL_DFIUPD2 {}
#[doc = "DDRCTRL DFI update register 2"]
pub mod ddrctrl_dfiupd2;
#[doc = "DDRCTRL DFI miscellaneous control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_dfimisc](ddrctrl_dfimisc) module"]
pub type DDRCTRL_DFIMISC = crate::Reg<u32, _DDRCTRL_DFIMISC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_DFIMISC;
#[doc = "`read()` method returns [ddrctrl_dfimisc::R](ddrctrl_dfimisc::R) reader structure"]
impl crate::Readable for DDRCTRL_DFIMISC {}
#[doc = "`write(|w| ..)` method takes [ddrctrl_dfimisc::W](ddrctrl_dfimisc::W) writer structure"]
impl crate::Writable for DDRCTRL_DFIMISC {}
#[doc = "DDRCTRL DFI miscellaneous control register"]
pub mod ddrctrl_dfimisc;
#[doc = "DDRCTRL DFI status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_dfistat](ddrctrl_dfistat) module"]
pub type DDRCTRL_DFISTAT = crate::Reg<u32, _DDRCTRL_DFISTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_DFISTAT;
#[doc = "`read()` method returns [ddrctrl_dfistat::R](ddrctrl_dfistat::R) reader structure"]
impl crate::Readable for DDRCTRL_DFISTAT {}
#[doc = "DDRCTRL DFI status register"]
pub mod ddrctrl_dfistat;
#[doc = "DDRCTRL DFI PHY master register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_dfiphymstr](ddrctrl_dfiphymstr) module"]
pub type DDRCTRL_DFIPHYMSTR = crate::Reg<u32, _DDRCTRL_DFIPHYMSTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_DFIPHYMSTR;
#[doc = "`read()` method returns [ddrctrl_dfiphymstr::R](ddrctrl_dfiphymstr::R) reader structure"]
impl crate::Readable for DDRCTRL_DFIPHYMSTR {}
#[doc = "`write(|w| ..)` method takes [ddrctrl_dfiphymstr::W](ddrctrl_dfiphymstr::W) writer structure"]
impl crate::Writable for DDRCTRL_DFIPHYMSTR {}
#[doc = "DDRCTRL DFI PHY master register"]
pub mod ddrctrl_dfiphymstr;
#[doc = "DDRCTRL address map register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_addrmap1](ddrctrl_addrmap1) module"]
pub type DDRCTRL_ADDRMAP1 = crate::Reg<u32, _DDRCTRL_ADDRMAP1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_ADDRMAP1;
#[doc = "`read()` method returns [ddrctrl_addrmap1::R](ddrctrl_addrmap1::R) reader structure"]
impl crate::Readable for DDRCTRL_ADDRMAP1 {}
#[doc = "`write(|w| ..)` method takes [ddrctrl_addrmap1::W](ddrctrl_addrmap1::W) writer structure"]
impl crate::Writable for DDRCTRL_ADDRMAP1 {}
#[doc = "DDRCTRL address map register 1"]
pub mod ddrctrl_addrmap1;
#[doc = "DDRCTRL address map register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_addrmap2](ddrctrl_addrmap2) module"]
pub type DDRCTRL_ADDRMAP2 = crate::Reg<u32, _DDRCTRL_ADDRMAP2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_ADDRMAP2;
#[doc = "`read()` method returns [ddrctrl_addrmap2::R](ddrctrl_addrmap2::R) reader structure"]
impl crate::Readable for DDRCTRL_ADDRMAP2 {}
#[doc = "`write(|w| ..)` method takes [ddrctrl_addrmap2::W](ddrctrl_addrmap2::W) writer structure"]
impl crate::Writable for DDRCTRL_ADDRMAP2 {}
#[doc = "DDRCTRL address map register 2"]
pub mod ddrctrl_addrmap2;
#[doc = "DDRCTRL address map register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_addrmap3](ddrctrl_addrmap3) module"]
pub type DDRCTRL_ADDRMAP3 = crate::Reg<u32, _DDRCTRL_ADDRMAP3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_ADDRMAP3;
#[doc = "`read()` method returns [ddrctrl_addrmap3::R](ddrctrl_addrmap3::R) reader structure"]
impl crate::Readable for DDRCTRL_ADDRMAP3 {}
#[doc = "`write(|w| ..)` method takes [ddrctrl_addrmap3::W](ddrctrl_addrmap3::W) writer structure"]
impl crate::Writable for DDRCTRL_ADDRMAP3 {}
#[doc = "DDRCTRL address map register 3"]
pub mod ddrctrl_addrmap3;
#[doc = "DDRCTRL address map register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_addrmap4](ddrctrl_addrmap4) module"]
pub type DDRCTRL_ADDRMAP4 = crate::Reg<u32, _DDRCTRL_ADDRMAP4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_ADDRMAP4;
#[doc = "`read()` method returns [ddrctrl_addrmap4::R](ddrctrl_addrmap4::R) reader structure"]
impl crate::Readable for DDRCTRL_ADDRMAP4 {}
#[doc = "`write(|w| ..)` method takes [ddrctrl_addrmap4::W](ddrctrl_addrmap4::W) writer structure"]
impl crate::Writable for DDRCTRL_ADDRMAP4 {}
#[doc = "DDRCTRL address map register 4"]
pub mod ddrctrl_addrmap4;
#[doc = "DDRCTRL address map register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_addrmap5](ddrctrl_addrmap5) module"]
pub type DDRCTRL_ADDRMAP5 = crate::Reg<u32, _DDRCTRL_ADDRMAP5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_ADDRMAP5;
#[doc = "`read()` method returns [ddrctrl_addrmap5::R](ddrctrl_addrmap5::R) reader structure"]
impl crate::Readable for DDRCTRL_ADDRMAP5 {}
#[doc = "`write(|w| ..)` method takes [ddrctrl_addrmap5::W](ddrctrl_addrmap5::W) writer structure"]
impl crate::Writable for DDRCTRL_ADDRMAP5 {}
#[doc = "DDRCTRL address map register 5"]
pub mod ddrctrl_addrmap5;
#[doc = "DDRCTRL address register 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_addrmap6](ddrctrl_addrmap6) module"]
pub type DDRCTRL_ADDRMAP6 = crate::Reg<u32, _DDRCTRL_ADDRMAP6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_ADDRMAP6;
#[doc = "`read()` method returns [ddrctrl_addrmap6::R](ddrctrl_addrmap6::R) reader structure"]
impl crate::Readable for DDRCTRL_ADDRMAP6 {}
#[doc = "`write(|w| ..)` method takes [ddrctrl_addrmap6::W](ddrctrl_addrmap6::W) writer structure"]
impl crate::Writable for DDRCTRL_ADDRMAP6 {}
#[doc = "DDRCTRL address register 6"]
pub mod ddrctrl_addrmap6;
#[doc = "DDRCTRL address map register 9\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_addrmap9](ddrctrl_addrmap9) module"]
pub type DDRCTRL_ADDRMAP9 = crate::Reg<u32, _DDRCTRL_ADDRMAP9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_ADDRMAP9;
#[doc = "`read()` method returns [ddrctrl_addrmap9::R](ddrctrl_addrmap9::R) reader structure"]
impl crate::Readable for DDRCTRL_ADDRMAP9 {}
#[doc = "`write(|w| ..)` method takes [ddrctrl_addrmap9::W](ddrctrl_addrmap9::W) writer structure"]
impl crate::Writable for DDRCTRL_ADDRMAP9 {}
#[doc = "DDRCTRL address map register 9"]
pub mod ddrctrl_addrmap9;
#[doc = "DDRCTRL address map register 10\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_addrmap10](ddrctrl_addrmap10) module"]
pub type DDRCTRL_ADDRMAP10 = crate::Reg<u32, _DDRCTRL_ADDRMAP10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_ADDRMAP10;
#[doc = "`read()` method returns [ddrctrl_addrmap10::R](ddrctrl_addrmap10::R) reader structure"]
impl crate::Readable for DDRCTRL_ADDRMAP10 {}
#[doc = "`write(|w| ..)` method takes [ddrctrl_addrmap10::W](ddrctrl_addrmap10::W) writer structure"]
impl crate::Writable for DDRCTRL_ADDRMAP10 {}
#[doc = "DDRCTRL address map register 10"]
pub mod ddrctrl_addrmap10;
#[doc = "DDRCTRL address map register 11\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_addrmap11](ddrctrl_addrmap11) module"]
pub type DDRCTRL_ADDRMAP11 = crate::Reg<u32, _DDRCTRL_ADDRMAP11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_ADDRMAP11;
#[doc = "`read()` method returns [ddrctrl_addrmap11::R](ddrctrl_addrmap11::R) reader structure"]
impl crate::Readable for DDRCTRL_ADDRMAP11 {}
#[doc = "`write(|w| ..)` method takes [ddrctrl_addrmap11::W](ddrctrl_addrmap11::W) writer structure"]
impl crate::Writable for DDRCTRL_ADDRMAP11 {}
#[doc = "DDRCTRL address map register 11"]
pub mod ddrctrl_addrmap11;
#[doc = "DDRCTRL ODT configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_odtcfg](ddrctrl_odtcfg) module"]
pub type DDRCTRL_ODTCFG = crate::Reg<u32, _DDRCTRL_ODTCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_ODTCFG;
#[doc = "`read()` method returns [ddrctrl_odtcfg::R](ddrctrl_odtcfg::R) reader structure"]
impl crate::Readable for DDRCTRL_ODTCFG {}
#[doc = "`write(|w| ..)` method takes [ddrctrl_odtcfg::W](ddrctrl_odtcfg::W) writer structure"]
impl crate::Writable for DDRCTRL_ODTCFG {}
#[doc = "DDRCTRL ODT configuration register"]
pub mod ddrctrl_odtcfg;
#[doc = "DDRCTRL ODT/Rank map register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_odtmap](ddrctrl_odtmap) module"]
pub type DDRCTRL_ODTMAP = crate::Reg<u32, _DDRCTRL_ODTMAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_ODTMAP;
#[doc = "`read()` method returns [ddrctrl_odtmap::R](ddrctrl_odtmap::R) reader structure"]
impl crate::Readable for DDRCTRL_ODTMAP {}
#[doc = "`write(|w| ..)` method takes [ddrctrl_odtmap::W](ddrctrl_odtmap::W) writer structure"]
impl crate::Writable for DDRCTRL_ODTMAP {}
#[doc = "DDRCTRL ODT/Rank map register"]
pub mod ddrctrl_odtmap;
#[doc = "DDRCTRL scheduler control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_sched](ddrctrl_sched) module"]
pub type DDRCTRL_SCHED = crate::Reg<u32, _DDRCTRL_SCHED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_SCHED;
#[doc = "`read()` method returns [ddrctrl_sched::R](ddrctrl_sched::R) reader structure"]
impl crate::Readable for DDRCTRL_SCHED {}
#[doc = "`write(|w| ..)` method takes [ddrctrl_sched::W](ddrctrl_sched::W) writer structure"]
impl crate::Writable for DDRCTRL_SCHED {}
#[doc = "DDRCTRL scheduler control register"]
pub mod ddrctrl_sched;
#[doc = "DDRCTRL scheduler control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_sched1](ddrctrl_sched1) module"]
pub type DDRCTRL_SCHED1 = crate::Reg<u32, _DDRCTRL_SCHED1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_SCHED1;
#[doc = "`read()` method returns [ddrctrl_sched1::R](ddrctrl_sched1::R) reader structure"]
impl crate::Readable for DDRCTRL_SCHED1 {}
#[doc = "`write(|w| ..)` method takes [ddrctrl_sched1::W](ddrctrl_sched1::W) writer structure"]
impl crate::Writable for DDRCTRL_SCHED1 {}
#[doc = "DDRCTRL scheduler control register 1"]
pub mod ddrctrl_sched1;
#[doc = "DDRCTRL high priority read CAM register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_perfhpr1](ddrctrl_perfhpr1) module"]
pub type DDRCTRL_PERFHPR1 = crate::Reg<u32, _DDRCTRL_PERFHPR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_PERFHPR1;
#[doc = "`read()` method returns [ddrctrl_perfhpr1::R](ddrctrl_perfhpr1::R) reader structure"]
impl crate::Readable for DDRCTRL_PERFHPR1 {}
#[doc = "`write(|w| ..)` method takes [ddrctrl_perfhpr1::W](ddrctrl_perfhpr1::W) writer structure"]
impl crate::Writable for DDRCTRL_PERFHPR1 {}
#[doc = "DDRCTRL high priority read CAM register 1"]
pub mod ddrctrl_perfhpr1;
#[doc = "DDRCTRL low priority read CAM register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_perflpr1](ddrctrl_perflpr1) module"]
pub type DDRCTRL_PERFLPR1 = crate::Reg<u32, _DDRCTRL_PERFLPR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_PERFLPR1;
#[doc = "`read()` method returns [ddrctrl_perflpr1::R](ddrctrl_perflpr1::R) reader structure"]
impl crate::Readable for DDRCTRL_PERFLPR1 {}
#[doc = "`write(|w| ..)` method takes [ddrctrl_perflpr1::W](ddrctrl_perflpr1::W) writer structure"]
impl crate::Writable for DDRCTRL_PERFLPR1 {}
#[doc = "DDRCTRL low priority read CAM register 1"]
pub mod ddrctrl_perflpr1;
#[doc = "DDRCTRL write CAM register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_perfwr1](ddrctrl_perfwr1) module"]
pub type DDRCTRL_PERFWR1 = crate::Reg<u32, _DDRCTRL_PERFWR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_PERFWR1;
#[doc = "`read()` method returns [ddrctrl_perfwr1::R](ddrctrl_perfwr1::R) reader structure"]
impl crate::Readable for DDRCTRL_PERFWR1 {}
#[doc = "`write(|w| ..)` method takes [ddrctrl_perfwr1::W](ddrctrl_perfwr1::W) writer structure"]
impl crate::Writable for DDRCTRL_PERFWR1 {}
#[doc = "DDRCTRL write CAM register 1"]
pub mod ddrctrl_perfwr1;
#[doc = "DDRCTRL debug register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_dbg0](ddrctrl_dbg0) module"]
pub type DDRCTRL_DBG0 = crate::Reg<u32, _DDRCTRL_DBG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_DBG0;
#[doc = "`read()` method returns [ddrctrl_dbg0::R](ddrctrl_dbg0::R) reader structure"]
impl crate::Readable for DDRCTRL_DBG0 {}
#[doc = "`write(|w| ..)` method takes [ddrctrl_dbg0::W](ddrctrl_dbg0::W) writer structure"]
impl crate::Writable for DDRCTRL_DBG0 {}
#[doc = "DDRCTRL debug register 0"]
pub mod ddrctrl_dbg0;
#[doc = "DDRCTRL debug register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_dbg1](ddrctrl_dbg1) module"]
pub type DDRCTRL_DBG1 = crate::Reg<u32, _DDRCTRL_DBG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_DBG1;
#[doc = "`read()` method returns [ddrctrl_dbg1::R](ddrctrl_dbg1::R) reader structure"]
impl crate::Readable for DDRCTRL_DBG1 {}
#[doc = "`write(|w| ..)` method takes [ddrctrl_dbg1::W](ddrctrl_dbg1::W) writer structure"]
impl crate::Writable for DDRCTRL_DBG1 {}
#[doc = "DDRCTRL debug register 1"]
pub mod ddrctrl_dbg1;
#[doc = "DDRCTRL CAM debug register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_dbgcam](ddrctrl_dbgcam) module"]
pub type DDRCTRL_DBGCAM = crate::Reg<u32, _DDRCTRL_DBGCAM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_DBGCAM;
#[doc = "`read()` method returns [ddrctrl_dbgcam::R](ddrctrl_dbgcam::R) reader structure"]
impl crate::Readable for DDRCTRL_DBGCAM {}
#[doc = "DDRCTRL CAM debug register"]
pub mod ddrctrl_dbgcam;
#[doc = "DDRCTRL command debug register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_dbgcmd](ddrctrl_dbgcmd) module"]
pub type DDRCTRL_DBGCMD = crate::Reg<u32, _DDRCTRL_DBGCMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_DBGCMD;
#[doc = "`read()` method returns [ddrctrl_dbgcmd::R](ddrctrl_dbgcmd::R) reader structure"]
impl crate::Readable for DDRCTRL_DBGCMD {}
#[doc = "`write(|w| ..)` method takes [ddrctrl_dbgcmd::W](ddrctrl_dbgcmd::W) writer structure"]
impl crate::Writable for DDRCTRL_DBGCMD {}
#[doc = "DDRCTRL command debug register"]
pub mod ddrctrl_dbgcmd;
#[doc = "DDRCTRL status debug register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_dbgstat](ddrctrl_dbgstat) module"]
pub type DDRCTRL_DBGSTAT = crate::Reg<u32, _DDRCTRL_DBGSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_DBGSTAT;
#[doc = "`read()` method returns [ddrctrl_dbgstat::R](ddrctrl_dbgstat::R) reader structure"]
impl crate::Readable for DDRCTRL_DBGSTAT {}
#[doc = "DDRCTRL status debug register"]
pub mod ddrctrl_dbgstat;
#[doc = "DDRCTRL software register programming control enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_swctl](ddrctrl_swctl) module"]
pub type DDRCTRL_SWCTL = crate::Reg<u32, _DDRCTRL_SWCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_SWCTL;
#[doc = "`read()` method returns [ddrctrl_swctl::R](ddrctrl_swctl::R) reader structure"]
impl crate::Readable for DDRCTRL_SWCTL {}
#[doc = "`write(|w| ..)` method takes [ddrctrl_swctl::W](ddrctrl_swctl::W) writer structure"]
impl crate::Writable for DDRCTRL_SWCTL {}
#[doc = "DDRCTRL software register programming control enable"]
pub mod ddrctrl_swctl;
#[doc = "DDRCTRL software register programming control status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_swstat](ddrctrl_swstat) module"]
pub type DDRCTRL_SWSTAT = crate::Reg<u32, _DDRCTRL_SWSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_SWSTAT;
#[doc = "`read()` method returns [ddrctrl_swstat::R](ddrctrl_swstat::R) reader structure"]
impl crate::Readable for DDRCTRL_SWSTAT {}
#[doc = "DDRCTRL software register programming control status"]
pub mod ddrctrl_swstat;
#[doc = "AXI Poison configuration register common for all AXI ports.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_poisoncfg](ddrctrl_poisoncfg) module"]
pub type DDRCTRL_POISONCFG = crate::Reg<u32, _DDRCTRL_POISONCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_POISONCFG;
#[doc = "`read()` method returns [ddrctrl_poisoncfg::R](ddrctrl_poisoncfg::R) reader structure"]
impl crate::Readable for DDRCTRL_POISONCFG {}
#[doc = "`write(|w| ..)` method takes [ddrctrl_poisoncfg::W](ddrctrl_poisoncfg::W) writer structure"]
impl crate::Writable for DDRCTRL_POISONCFG {}
#[doc = "AXI Poison configuration register common for all AXI ports."]
pub mod ddrctrl_poisoncfg;
#[doc = "DDRCTRL AXI Poison status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_poisonstat](ddrctrl_poisonstat) module"]
pub type DDRCTRL_POISONSTAT = crate::Reg<u32, _DDRCTRL_POISONSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_POISONSTAT;
#[doc = "`read()` method returns [ddrctrl_poisonstat::R](ddrctrl_poisonstat::R) reader structure"]
impl crate::Readable for DDRCTRL_POISONSTAT {}
#[doc = "DDRCTRL AXI Poison status register"]
pub mod ddrctrl_poisonstat;
#[doc = "DDRCTRL port status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_pstat](ddrctrl_pstat) module"]
pub type DDRCTRL_PSTAT = crate::Reg<u32, _DDRCTRL_PSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_PSTAT;
#[doc = "`read()` method returns [ddrctrl_pstat::R](ddrctrl_pstat::R) reader structure"]
impl crate::Readable for DDRCTRL_PSTAT {}
#[doc = "DDRCTRL port status register"]
pub mod ddrctrl_pstat;
#[doc = "DDRCTRL port common configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_pccfg](ddrctrl_pccfg) module"]
pub type DDRCTRL_PCCFG = crate::Reg<u32, _DDRCTRL_PCCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_PCCFG;
#[doc = "`read()` method returns [ddrctrl_pccfg::R](ddrctrl_pccfg::R) reader structure"]
impl crate::Readable for DDRCTRL_PCCFG {}
#[doc = "`write(|w| ..)` method takes [ddrctrl_pccfg::W](ddrctrl_pccfg::W) writer structure"]
impl crate::Writable for DDRCTRL_PCCFG {}
#[doc = "DDRCTRL port common configuration register"]
pub mod ddrctrl_pccfg;
#[doc = "DDRCTRL port 0 configuration read register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_pcfgr_0](ddrctrl_pcfgr_0) module"]
pub type DDRCTRL_PCFGR_0 = crate::Reg<u32, _DDRCTRL_PCFGR_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_PCFGR_0;
#[doc = "`read()` method returns [ddrctrl_pcfgr_0::R](ddrctrl_pcfgr_0::R) reader structure"]
impl crate::Readable for DDRCTRL_PCFGR_0 {}
#[doc = "`write(|w| ..)` method takes [ddrctrl_pcfgr_0::W](ddrctrl_pcfgr_0::W) writer structure"]
impl crate::Writable for DDRCTRL_PCFGR_0 {}
#[doc = "DDRCTRL port 0 configuration read register"]
pub mod ddrctrl_pcfgr_0;
#[doc = "DDRCTRL port 0 configuration write register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_pcfgw_0](ddrctrl_pcfgw_0) module"]
pub type DDRCTRL_PCFGW_0 = crate::Reg<u32, _DDRCTRL_PCFGW_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_PCFGW_0;
#[doc = "`read()` method returns [ddrctrl_pcfgw_0::R](ddrctrl_pcfgw_0::R) reader structure"]
impl crate::Readable for DDRCTRL_PCFGW_0 {}
#[doc = "`write(|w| ..)` method takes [ddrctrl_pcfgw_0::W](ddrctrl_pcfgw_0::W) writer structure"]
impl crate::Writable for DDRCTRL_PCFGW_0 {}
#[doc = "DDRCTRL port 0 configuration write register"]
pub mod ddrctrl_pcfgw_0;
#[doc = "DDRCTRL port 0 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_pctrl_0](ddrctrl_pctrl_0) module"]
pub type DDRCTRL_PCTRL_0 = crate::Reg<u32, _DDRCTRL_PCTRL_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_PCTRL_0;
#[doc = "`read()` method returns [ddrctrl_pctrl_0::R](ddrctrl_pctrl_0::R) reader structure"]
impl crate::Readable for DDRCTRL_PCTRL_0 {}
#[doc = "`write(|w| ..)` method takes [ddrctrl_pctrl_0::W](ddrctrl_pctrl_0::W) writer structure"]
impl crate::Writable for DDRCTRL_PCTRL_0 {}
#[doc = "DDRCTRL port 0 control register"]
pub mod ddrctrl_pctrl_0;
#[doc = "DDRCTRL port 0 read Q0S configuration register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_pcfgqos0_0](ddrctrl_pcfgqos0_0) module"]
pub type DDRCTRL_PCFGQOS0_0 = crate::Reg<u32, _DDRCTRL_PCFGQOS0_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_PCFGQOS0_0;
#[doc = "`read()` method returns [ddrctrl_pcfgqos0_0::R](ddrctrl_pcfgqos0_0::R) reader structure"]
impl crate::Readable for DDRCTRL_PCFGQOS0_0 {}
#[doc = "`write(|w| ..)` method takes [ddrctrl_pcfgqos0_0::W](ddrctrl_pcfgqos0_0::W) writer structure"]
impl crate::Writable for DDRCTRL_PCFGQOS0_0 {}
#[doc = "DDRCTRL port 0 read Q0S configuration register 0"]
pub mod ddrctrl_pcfgqos0_0;
#[doc = "DDRCTRL port 0 read Q0S configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_pcfgqos1_0](ddrctrl_pcfgqos1_0) module"]
pub type DDRCTRL_PCFGQOS1_0 = crate::Reg<u32, _DDRCTRL_PCFGQOS1_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_PCFGQOS1_0;
#[doc = "`read()` method returns [ddrctrl_pcfgqos1_0::R](ddrctrl_pcfgqos1_0::R) reader structure"]
impl crate::Readable for DDRCTRL_PCFGQOS1_0 {}
#[doc = "`write(|w| ..)` method takes [ddrctrl_pcfgqos1_0::W](ddrctrl_pcfgqos1_0::W) writer structure"]
impl crate::Writable for DDRCTRL_PCFGQOS1_0 {}
#[doc = "DDRCTRL port 0 read Q0S configuration register 1"]
pub mod ddrctrl_pcfgqos1_0;
#[doc = "DDRCTRL port 0 write Q0S configuration register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_pcfgwqos0_0](ddrctrl_pcfgwqos0_0) module"]
pub type DDRCTRL_PCFGWQOS0_0 = crate::Reg<u32, _DDRCTRL_PCFGWQOS0_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_PCFGWQOS0_0;
#[doc = "`read()` method returns [ddrctrl_pcfgwqos0_0::R](ddrctrl_pcfgwqos0_0::R) reader structure"]
impl crate::Readable for DDRCTRL_PCFGWQOS0_0 {}
#[doc = "`write(|w| ..)` method takes [ddrctrl_pcfgwqos0_0::W](ddrctrl_pcfgwqos0_0::W) writer structure"]
impl crate::Writable for DDRCTRL_PCFGWQOS0_0 {}
#[doc = "DDRCTRL port 0 write Q0S configuration register 0"]
pub mod ddrctrl_pcfgwqos0_0;
#[doc = "DDRCTRL port 0 write Q0S configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_pcfgwqos1_0](ddrctrl_pcfgwqos1_0) module"]
pub type DDRCTRL_PCFGWQOS1_0 = crate::Reg<u32, _DDRCTRL_PCFGWQOS1_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_PCFGWQOS1_0;
#[doc = "`read()` method returns [ddrctrl_pcfgwqos1_0::R](ddrctrl_pcfgwqos1_0::R) reader structure"]
impl crate::Readable for DDRCTRL_PCFGWQOS1_0 {}
#[doc = "`write(|w| ..)` method takes [ddrctrl_pcfgwqos1_0::W](ddrctrl_pcfgwqos1_0::W) writer structure"]
impl crate::Writable for DDRCTRL_PCFGWQOS1_0 {}
#[doc = "DDRCTRL port 0 write Q0S configuration register 1"]
pub mod ddrctrl_pcfgwqos1_0;
#[doc = "DDRCTRL port 1 configuration read register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_pcfgr_1](ddrctrl_pcfgr_1) module"]
pub type DDRCTRL_PCFGR_1 = crate::Reg<u32, _DDRCTRL_PCFGR_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_PCFGR_1;
#[doc = "`read()` method returns [ddrctrl_pcfgr_1::R](ddrctrl_pcfgr_1::R) reader structure"]
impl crate::Readable for DDRCTRL_PCFGR_1 {}
#[doc = "`write(|w| ..)` method takes [ddrctrl_pcfgr_1::W](ddrctrl_pcfgr_1::W) writer structure"]
impl crate::Writable for DDRCTRL_PCFGR_1 {}
#[doc = "DDRCTRL port 1 configuration read register"]
pub mod ddrctrl_pcfgr_1;
#[doc = "DDRCTRL port 1 configuration write register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_pcfgw_1](ddrctrl_pcfgw_1) module"]
pub type DDRCTRL_PCFGW_1 = crate::Reg<u32, _DDRCTRL_PCFGW_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_PCFGW_1;
#[doc = "`read()` method returns [ddrctrl_pcfgw_1::R](ddrctrl_pcfgw_1::R) reader structure"]
impl crate::Readable for DDRCTRL_PCFGW_1 {}
#[doc = "`write(|w| ..)` method takes [ddrctrl_pcfgw_1::W](ddrctrl_pcfgw_1::W) writer structure"]
impl crate::Writable for DDRCTRL_PCFGW_1 {}
#[doc = "DDRCTRL port 1 configuration write register"]
pub mod ddrctrl_pcfgw_1;
#[doc = "DDRCTRL port 1 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_pctrl_1](ddrctrl_pctrl_1) module"]
pub type DDRCTRL_PCTRL_1 = crate::Reg<u32, _DDRCTRL_PCTRL_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_PCTRL_1;
#[doc = "`read()` method returns [ddrctrl_pctrl_1::R](ddrctrl_pctrl_1::R) reader structure"]
impl crate::Readable for DDRCTRL_PCTRL_1 {}
#[doc = "`write(|w| ..)` method takes [ddrctrl_pctrl_1::W](ddrctrl_pctrl_1::W) writer structure"]
impl crate::Writable for DDRCTRL_PCTRL_1 {}
#[doc = "DDRCTRL port 1 control register"]
pub mod ddrctrl_pctrl_1;
#[doc = "DDRCTRL port 1 read Q0S configuration register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_pcfgqos0_1](ddrctrl_pcfgqos0_1) module"]
pub type DDRCTRL_PCFGQOS0_1 = crate::Reg<u32, _DDRCTRL_PCFGQOS0_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_PCFGQOS0_1;
#[doc = "`read()` method returns [ddrctrl_pcfgqos0_1::R](ddrctrl_pcfgqos0_1::R) reader structure"]
impl crate::Readable for DDRCTRL_PCFGQOS0_1 {}
#[doc = "`write(|w| ..)` method takes [ddrctrl_pcfgqos0_1::W](ddrctrl_pcfgqos0_1::W) writer structure"]
impl crate::Writable for DDRCTRL_PCFGQOS0_1 {}
#[doc = "DDRCTRL port 1 read Q0S configuration register 0"]
pub mod ddrctrl_pcfgqos0_1;
#[doc = "DDRCTRL port 1 read Q0S configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_pcfgqos1_1](ddrctrl_pcfgqos1_1) module"]
pub type DDRCTRL_PCFGQOS1_1 = crate::Reg<u32, _DDRCTRL_PCFGQOS1_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_PCFGQOS1_1;
#[doc = "`read()` method returns [ddrctrl_pcfgqos1_1::R](ddrctrl_pcfgqos1_1::R) reader structure"]
impl crate::Readable for DDRCTRL_PCFGQOS1_1 {}
#[doc = "`write(|w| ..)` method takes [ddrctrl_pcfgqos1_1::W](ddrctrl_pcfgqos1_1::W) writer structure"]
impl crate::Writable for DDRCTRL_PCFGQOS1_1 {}
#[doc = "DDRCTRL port 1 read Q0S configuration register 1"]
pub mod ddrctrl_pcfgqos1_1;
#[doc = "DDRCTRL port 1 write Q0S configuration register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_pcfgwqos0_1](ddrctrl_pcfgwqos0_1) module"]
pub type DDRCTRL_PCFGWQOS0_1 = crate::Reg<u32, _DDRCTRL_PCFGWQOS0_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_PCFGWQOS0_1;
#[doc = "`read()` method returns [ddrctrl_pcfgwqos0_1::R](ddrctrl_pcfgwqos0_1::R) reader structure"]
impl crate::Readable for DDRCTRL_PCFGWQOS0_1 {}
#[doc = "`write(|w| ..)` method takes [ddrctrl_pcfgwqos0_1::W](ddrctrl_pcfgwqos0_1::W) writer structure"]
impl crate::Writable for DDRCTRL_PCFGWQOS0_1 {}
#[doc = "DDRCTRL port 1 write Q0S configuration register 0"]
pub mod ddrctrl_pcfgwqos0_1;
#[doc = "DDRCTRL port 1 write Q0S configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_pcfgwqos1_1](ddrctrl_pcfgwqos1_1) module"]
pub type DDRCTRL_PCFGWQOS1_1 = crate::Reg<u32, _DDRCTRL_PCFGWQOS1_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRCTRL_PCFGWQOS1_1;
#[doc = "`read()` method returns [ddrctrl_pcfgwqos1_1::R](ddrctrl_pcfgwqos1_1::R) reader structure"]
impl crate::Readable for DDRCTRL_PCFGWQOS1_1 {}
#[doc = "`write(|w| ..)` method takes [ddrctrl_pcfgwqos1_1::W](ddrctrl_pcfgwqos1_1::W) writer structure"]
impl crate::Writable for DDRCTRL_PCFGWQOS1_1 {}
#[doc = "DDRCTRL port 1 write Q0S configuration register 1"]
pub mod ddrctrl_pcfgwqos1_1;
