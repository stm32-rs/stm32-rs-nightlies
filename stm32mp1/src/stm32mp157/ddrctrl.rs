#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    mstr: MSTR,
    stat: STAT,
    _reserved2: [u8; 0x08],
    mrctrl0: MRCTRL0,
    mrctrl1: MRCTRL1,
    mrstat: MRSTAT,
    _reserved5: [u8; 0x04],
    derateen: DERATEEN,
    derateint: DERATEINT,
    _reserved7: [u8; 0x08],
    pwrctl: PWRCTL,
    pwrtmg: PWRTMG,
    hwlpctl: HWLPCTL,
    _reserved10: [u8; 0x14],
    rfshctl0: RFSHCTL0,
    _reserved11: [u8; 0x0c],
    rfshctl3: RFSHCTL3,
    rfshtmg: RFSHTMG,
    _reserved13: [u8; 0x58],
    crcparctl0: CRCPARCTL0,
    _reserved14: [u8; 0x08],
    crcparstat: CRCPARSTAT,
    init0: INIT0,
    init1: INIT1,
    init2: INIT2,
    init3: INIT3,
    init4: INIT4,
    init5: INIT5,
    _reserved21: [u8; 0x08],
    dimmctl: DIMMCTL,
    _reserved22: [u8; 0x0c],
    dramtmg0: DRAMTMG0,
    dramtmg1: DRAMTMG1,
    dramtmg2: DRAMTMG2,
    dramtmg3: DRAMTMG3,
    dramtmg4: DRAMTMG4,
    dramtmg5: DRAMTMG5,
    dramtmg6: DRAMTMG6,
    dramtmg7: DRAMTMG7,
    dramtmg8: DRAMTMG8,
    _reserved31: [u8; 0x14],
    dramtmg14: DRAMTMG14,
    dramtmg15: DRAMTMG15,
    _reserved33: [u8; 0x40],
    zqctl0: ZQCTL0,
    zqctl1: ZQCTL1,
    zqctl2: ZQCTL2,
    zqstat: ZQSTAT,
    dfitmg0: DFITMG0,
    dfitmg1: DFITMG1,
    dfilpcfg0: DFILPCFG0,
    _reserved40: [u8; 0x04],
    dfiupd0: DFIUPD0,
    dfiupd1: DFIUPD1,
    dfiupd2: DFIUPD2,
    _reserved43: [u8; 0x04],
    dfimisc: DFIMISC,
    _reserved44: [u8; 0x08],
    dfistat: DFISTAT,
    _reserved45: [u8; 0x04],
    dfiphymstr: DFIPHYMSTR,
    _reserved46: [u8; 0x3c],
    addrmap1: ADDRMAP1,
    addrmap2: ADDRMAP2,
    addrmap3: ADDRMAP3,
    addrmap4: ADDRMAP4,
    addrmap5: ADDRMAP5,
    addrmap6: ADDRMAP6,
    _reserved52: [u8; 0x08],
    addrmap9: ADDRMAP9,
    addrmap10: ADDRMAP10,
    addrmap11: ADDRMAP11,
    _reserved55: [u8; 0x10],
    odtcfg: ODTCFG,
    odtmap: ODTMAP,
    _reserved57: [u8; 0x08],
    sched: SCHED,
    sched1: SCHED1,
    _reserved59: [u8; 0x04],
    perfhpr1: PERFHPR1,
    _reserved60: [u8; 0x04],
    perflpr1: PERFLPR1,
    _reserved61: [u8; 0x04],
    perfwr1: PERFWR1,
    _reserved62: [u8; 0x90],
    dbg0: DBG0,
    dbg1: DBG1,
    dbgcam: DBGCAM,
    dbgcmd: DBGCMD,
    dbgstat: DBGSTAT,
    _reserved67: [u8; 0x0c],
    swctl: SWCTL,
    swstat: SWSTAT,
    _reserved69: [u8; 0x44],
    poisoncfg: POISONCFG,
    poisonstat: POISONSTAT,
    _reserved71: [u8; 0x88],
    pstat: PSTAT,
    pccfg: PCCFG,
    pcfgr_0: PCFGR_0,
    pcfgw_0: PCFGW_0,
    _reserved75: [u8; 0x84],
    pctrl_0: PCTRL_0,
    pcfgqos0_0: PCFGQOS0_0,
    pcfgqos1_0: PCFGQOS1_0,
    pcfgwqos0_0: PCFGWQOS0_0,
    pcfgwqos1_0: PCFGWQOS1_0,
    _reserved80: [u8; 0x10],
    pcfgr_1: PCFGR_1,
    pcfgw_1: PCFGW_1,
    _reserved82: [u8; 0x84],
    pctrl_1: PCTRL_1,
    pcfgqos0_1: PCFGQOS0_1,
    pcfgqos1_1: PCFGQOS1_1,
    pcfgwqos0_1: PCFGWQOS0_1,
    pcfgwqos1_1: PCFGWQOS1_1,
}
impl RegisterBlock {
    ///0x00 - DDRCTRL master register 0
    #[inline(always)]
    pub const fn mstr(&self) -> &MSTR {
        &self.mstr
    }
    ///0x04 - DDRCTRL operating mode status register
    #[inline(always)]
    pub const fn stat(&self) -> &STAT {
        &self.stat
    }
    ///0x10 - Mode Register Read/Write Control Register 0. Do not enable more than one of the following fields simultaneously: sw_init_int pda_en mpr_en
    #[inline(always)]
    pub const fn mrctrl0(&self) -> &MRCTRL0 {
        &self.mrctrl0
    }
    ///0x14 - DDRCTRL mode register read/write control register 1
    #[inline(always)]
    pub const fn mrctrl1(&self) -> &MRCTRL1 {
        &self.mrctrl1
    }
    ///0x18 - DDRCTRL mode register read/write status register
    #[inline(always)]
    pub const fn mrstat(&self) -> &MRSTAT {
        &self.mrstat
    }
    ///0x20 - DDRCTRL temperature derate enable register
    #[inline(always)]
    pub const fn derateen(&self) -> &DERATEEN {
        &self.derateen
    }
    ///0x24 - DDRCTRL temperature derate interval register
    #[inline(always)]
    pub const fn derateint(&self) -> &DERATEINT {
        &self.derateint
    }
    ///0x30 - DDRCTRL low power control register
    #[inline(always)]
    pub const fn pwrctl(&self) -> &PWRCTL {
        &self.pwrctl
    }
    ///0x34 - DDRCTRL low power timing register
    #[inline(always)]
    pub const fn pwrtmg(&self) -> &PWRTMG {
        &self.pwrtmg
    }
    ///0x38 - DDRCTRL hardware low power control register
    #[inline(always)]
    pub const fn hwlpctl(&self) -> &HWLPCTL {
        &self.hwlpctl
    }
    ///0x50 - DDRCTRL refresh control register 0
    #[inline(always)]
    pub const fn rfshctl0(&self) -> &RFSHCTL0 {
        &self.rfshctl0
    }
    ///0x60 - DDRCTRL refresh control register 3
    #[inline(always)]
    pub const fn rfshctl3(&self) -> &RFSHCTL3 {
        &self.rfshctl3
    }
    ///0x64 - DDRCTRL refresh timing register
    #[inline(always)]
    pub const fn rfshtmg(&self) -> &RFSHTMG {
        &self.rfshtmg
    }
    ///0xc0 - DDRCTRL CRC parity control register 0
    #[inline(always)]
    pub const fn crcparctl0(&self) -> &CRCPARCTL0 {
        &self.crcparctl0
    }
    ///0xcc - DDRCTRL CRC parity status register
    #[inline(always)]
    pub const fn crcparstat(&self) -> &CRCPARSTAT {
        &self.crcparstat
    }
    ///0xd0 - DDRCTRL SDRAM initialization register 0
    #[inline(always)]
    pub const fn init0(&self) -> &INIT0 {
        &self.init0
    }
    ///0xd4 - DDRCTRL SDRAM initialization register 1
    #[inline(always)]
    pub const fn init1(&self) -> &INIT1 {
        &self.init1
    }
    ///0xd8 - DDRCTRL SDRAM initialization register 2
    #[inline(always)]
    pub const fn init2(&self) -> &INIT2 {
        &self.init2
    }
    ///0xdc - DDRCTRL SDRAM initialization register 3
    #[inline(always)]
    pub const fn init3(&self) -> &INIT3 {
        &self.init3
    }
    ///0xe0 - DDRCTRL SDRAM initialization register 4
    #[inline(always)]
    pub const fn init4(&self) -> &INIT4 {
        &self.init4
    }
    ///0xe4 - DDRCTRL SDRAM initialization register 5
    #[inline(always)]
    pub const fn init5(&self) -> &INIT5 {
        &self.init5
    }
    ///0xf0 - DDRCTRL DIMM control register
    #[inline(always)]
    pub const fn dimmctl(&self) -> &DIMMCTL {
        &self.dimmctl
    }
    ///0x100 - DDRCTRL SDRAM timing register 0
    #[inline(always)]
    pub const fn dramtmg0(&self) -> &DRAMTMG0 {
        &self.dramtmg0
    }
    ///0x104 - DDRCTRL SDRAM timing register 1
    #[inline(always)]
    pub const fn dramtmg1(&self) -> &DRAMTMG1 {
        &self.dramtmg1
    }
    ///0x108 - DDRCTRL SDRAM timing register 2
    #[inline(always)]
    pub const fn dramtmg2(&self) -> &DRAMTMG2 {
        &self.dramtmg2
    }
    ///0x10c - DDRCTRL SDRAM timing register 3
    #[inline(always)]
    pub const fn dramtmg3(&self) -> &DRAMTMG3 {
        &self.dramtmg3
    }
    ///0x110 - DDRCTRL SDRAM timing register 4
    #[inline(always)]
    pub const fn dramtmg4(&self) -> &DRAMTMG4 {
        &self.dramtmg4
    }
    ///0x114 - DDRCTRL SDRAM timing register 5
    #[inline(always)]
    pub const fn dramtmg5(&self) -> &DRAMTMG5 {
        &self.dramtmg5
    }
    ///0x118 - DDRCTRL SDRAM timing register 6
    #[inline(always)]
    pub const fn dramtmg6(&self) -> &DRAMTMG6 {
        &self.dramtmg6
    }
    ///0x11c - DDRCTRL SDRAM timing register 7
    #[inline(always)]
    pub const fn dramtmg7(&self) -> &DRAMTMG7 {
        &self.dramtmg7
    }
    ///0x120 - DDRCTRL SDRAM timing register 8
    #[inline(always)]
    pub const fn dramtmg8(&self) -> &DRAMTMG8 {
        &self.dramtmg8
    }
    ///0x138 - DDRCTRL SDRAM timing register 14
    #[inline(always)]
    pub const fn dramtmg14(&self) -> &DRAMTMG14 {
        &self.dramtmg14
    }
    ///0x13c - DDRCTRL SDRAM timing register 15
    #[inline(always)]
    pub const fn dramtmg15(&self) -> &DRAMTMG15 {
        &self.dramtmg15
    }
    ///0x180 - DDRCTRL ZQ control register 0
    #[inline(always)]
    pub const fn zqctl0(&self) -> &ZQCTL0 {
        &self.zqctl0
    }
    ///0x184 - DDRCTRL ZQ control register 1
    #[inline(always)]
    pub const fn zqctl1(&self) -> &ZQCTL1 {
        &self.zqctl1
    }
    ///0x188 - DDRCTRL ZQ control register 2
    #[inline(always)]
    pub const fn zqctl2(&self) -> &ZQCTL2 {
        &self.zqctl2
    }
    ///0x18c - DDRCTRL ZQ status register
    #[inline(always)]
    pub const fn zqstat(&self) -> &ZQSTAT {
        &self.zqstat
    }
    ///0x190 - DDRCTRL DFI timing register 0
    #[inline(always)]
    pub const fn dfitmg0(&self) -> &DFITMG0 {
        &self.dfitmg0
    }
    ///0x194 - DDRCTRL DFI timing register 1
    #[inline(always)]
    pub const fn dfitmg1(&self) -> &DFITMG1 {
        &self.dfitmg1
    }
    ///0x198 - DDRCTRL low power configuration register 0
    #[inline(always)]
    pub const fn dfilpcfg0(&self) -> &DFILPCFG0 {
        &self.dfilpcfg0
    }
    ///0x1a0 - DDRCTRL DFI update register 0
    #[inline(always)]
    pub const fn dfiupd0(&self) -> &DFIUPD0 {
        &self.dfiupd0
    }
    ///0x1a4 - DDRCTRL DFI update register 1
    #[inline(always)]
    pub const fn dfiupd1(&self) -> &DFIUPD1 {
        &self.dfiupd1
    }
    ///0x1a8 - DDRCTRL DFI update register 2
    #[inline(always)]
    pub const fn dfiupd2(&self) -> &DFIUPD2 {
        &self.dfiupd2
    }
    ///0x1b0 - DDRCTRL DFI miscellaneous control register
    #[inline(always)]
    pub const fn dfimisc(&self) -> &DFIMISC {
        &self.dfimisc
    }
    ///0x1bc - DDRCTRL DFI status register
    #[inline(always)]
    pub const fn dfistat(&self) -> &DFISTAT {
        &self.dfistat
    }
    ///0x1c4 - DDRCTRL DFI PHY master register
    #[inline(always)]
    pub const fn dfiphymstr(&self) -> &DFIPHYMSTR {
        &self.dfiphymstr
    }
    ///0x204 - DDRCTRL address map register 1
    #[inline(always)]
    pub const fn addrmap1(&self) -> &ADDRMAP1 {
        &self.addrmap1
    }
    ///0x208 - DDRCTRL address map register 2
    #[inline(always)]
    pub const fn addrmap2(&self) -> &ADDRMAP2 {
        &self.addrmap2
    }
    ///0x20c - DDRCTRL address map register 3
    #[inline(always)]
    pub const fn addrmap3(&self) -> &ADDRMAP3 {
        &self.addrmap3
    }
    ///0x210 - DDRCTRL address map register 4
    #[inline(always)]
    pub const fn addrmap4(&self) -> &ADDRMAP4 {
        &self.addrmap4
    }
    ///0x214 - DDRCTRL address map register 5
    #[inline(always)]
    pub const fn addrmap5(&self) -> &ADDRMAP5 {
        &self.addrmap5
    }
    ///0x218 - DDRCTRL address register 6
    #[inline(always)]
    pub const fn addrmap6(&self) -> &ADDRMAP6 {
        &self.addrmap6
    }
    ///0x224 - DDRCTRL address map register 9
    #[inline(always)]
    pub const fn addrmap9(&self) -> &ADDRMAP9 {
        &self.addrmap9
    }
    ///0x228 - DDRCTRL address map register 10
    #[inline(always)]
    pub const fn addrmap10(&self) -> &ADDRMAP10 {
        &self.addrmap10
    }
    ///0x22c - DDRCTRL address map register 11
    #[inline(always)]
    pub const fn addrmap11(&self) -> &ADDRMAP11 {
        &self.addrmap11
    }
    ///0x240 - DDRCTRL ODT configuration register
    #[inline(always)]
    pub const fn odtcfg(&self) -> &ODTCFG {
        &self.odtcfg
    }
    ///0x244 - DDRCTRL ODT/Rank map register
    #[inline(always)]
    pub const fn odtmap(&self) -> &ODTMAP {
        &self.odtmap
    }
    ///0x250 - DDRCTRL scheduler control register
    #[inline(always)]
    pub const fn sched(&self) -> &SCHED {
        &self.sched
    }
    ///0x254 - DDRCTRL scheduler control register 1
    #[inline(always)]
    pub const fn sched1(&self) -> &SCHED1 {
        &self.sched1
    }
    ///0x25c - DDRCTRL high priority read CAM register 1
    #[inline(always)]
    pub const fn perfhpr1(&self) -> &PERFHPR1 {
        &self.perfhpr1
    }
    ///0x264 - DDRCTRL low priority read CAM register 1
    #[inline(always)]
    pub const fn perflpr1(&self) -> &PERFLPR1 {
        &self.perflpr1
    }
    ///0x26c - DDRCTRL write CAM register 1
    #[inline(always)]
    pub const fn perfwr1(&self) -> &PERFWR1 {
        &self.perfwr1
    }
    ///0x300 - DDRCTRL debug register 0
    #[inline(always)]
    pub const fn dbg0(&self) -> &DBG0 {
        &self.dbg0
    }
    ///0x304 - DDRCTRL debug register 1
    #[inline(always)]
    pub const fn dbg1(&self) -> &DBG1 {
        &self.dbg1
    }
    ///0x308 - DDRCTRL CAM debug register
    #[inline(always)]
    pub const fn dbgcam(&self) -> &DBGCAM {
        &self.dbgcam
    }
    ///0x30c - DDRCTRL command debug register
    #[inline(always)]
    pub const fn dbgcmd(&self) -> &DBGCMD {
        &self.dbgcmd
    }
    ///0x310 - DDRCTRL status debug register
    #[inline(always)]
    pub const fn dbgstat(&self) -> &DBGSTAT {
        &self.dbgstat
    }
    ///0x320 - DDRCTRL software register programming control enable
    #[inline(always)]
    pub const fn swctl(&self) -> &SWCTL {
        &self.swctl
    }
    ///0x324 - DDRCTRL software register programming control status
    #[inline(always)]
    pub const fn swstat(&self) -> &SWSTAT {
        &self.swstat
    }
    ///0x36c - AXI Poison configuration register common for all AXI ports.
    #[inline(always)]
    pub const fn poisoncfg(&self) -> &POISONCFG {
        &self.poisoncfg
    }
    ///0x370 - DDRCTRL AXI Poison status register
    #[inline(always)]
    pub const fn poisonstat(&self) -> &POISONSTAT {
        &self.poisonstat
    }
    ///0x3fc - DDRCTRL port status register
    #[inline(always)]
    pub const fn pstat(&self) -> &PSTAT {
        &self.pstat
    }
    ///0x400 - DDRCTRL port common configuration register
    #[inline(always)]
    pub const fn pccfg(&self) -> &PCCFG {
        &self.pccfg
    }
    ///0x404 - DDRCTRL port 0 configuration read register
    #[inline(always)]
    pub const fn pcfgr_0(&self) -> &PCFGR_0 {
        &self.pcfgr_0
    }
    ///0x408 - DDRCTRL port 0 configuration write register
    #[inline(always)]
    pub const fn pcfgw_0(&self) -> &PCFGW_0 {
        &self.pcfgw_0
    }
    ///0x490 - DDRCTRL port 0 control register
    #[inline(always)]
    pub const fn pctrl_0(&self) -> &PCTRL_0 {
        &self.pctrl_0
    }
    ///0x494 - DDRCTRL port 0 read Q0S configuration register 0
    #[inline(always)]
    pub const fn pcfgqos0_0(&self) -> &PCFGQOS0_0 {
        &self.pcfgqos0_0
    }
    ///0x498 - DDRCTRL port 0 read Q0S configuration register 1
    #[inline(always)]
    pub const fn pcfgqos1_0(&self) -> &PCFGQOS1_0 {
        &self.pcfgqos1_0
    }
    ///0x49c - DDRCTRL port 0 write Q0S configuration register 0
    #[inline(always)]
    pub const fn pcfgwqos0_0(&self) -> &PCFGWQOS0_0 {
        &self.pcfgwqos0_0
    }
    ///0x4a0 - DDRCTRL port 0 write Q0S configuration register 1
    #[inline(always)]
    pub const fn pcfgwqos1_0(&self) -> &PCFGWQOS1_0 {
        &self.pcfgwqos1_0
    }
    ///0x4b4 - DDRCTRL port 1 configuration read register
    #[inline(always)]
    pub const fn pcfgr_1(&self) -> &PCFGR_1 {
        &self.pcfgr_1
    }
    ///0x4b8 - DDRCTRL port 1 configuration write register
    #[inline(always)]
    pub const fn pcfgw_1(&self) -> &PCFGW_1 {
        &self.pcfgw_1
    }
    ///0x540 - DDRCTRL port 1 control register
    #[inline(always)]
    pub const fn pctrl_1(&self) -> &PCTRL_1 {
        &self.pctrl_1
    }
    ///0x544 - DDRCTRL port 1 read Q0S configuration register 0
    #[inline(always)]
    pub const fn pcfgqos0_1(&self) -> &PCFGQOS0_1 {
        &self.pcfgqos0_1
    }
    ///0x548 - DDRCTRL port 1 read Q0S configuration register 1
    #[inline(always)]
    pub const fn pcfgqos1_1(&self) -> &PCFGQOS1_1 {
        &self.pcfgqos1_1
    }
    ///0x54c - DDRCTRL port 1 write Q0S configuration register 0
    #[inline(always)]
    pub const fn pcfgwqos0_1(&self) -> &PCFGWQOS0_1 {
        &self.pcfgwqos0_1
    }
    ///0x550 - DDRCTRL port 1 write Q0S configuration register 1
    #[inline(always)]
    pub const fn pcfgwqos1_1(&self) -> &PCFGWQOS1_1 {
        &self.pcfgwqos1_1
    }
}
/**MSTR (rw) register accessor: DDRCTRL master register 0

You can [`read`](crate::Reg::read) this register and get [`mstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:MSTR)

For information about available fields see [`mod@mstr`] module*/
pub type MSTR = crate::Reg<mstr::MSTRrs>;
///DDRCTRL master register 0
pub mod mstr;
/**STAT (r) register accessor: DDRCTRL operating mode status register

You can [`read`](crate::Reg::read) this register and get [`stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:STAT)

For information about available fields see [`mod@stat`] module*/
pub type STAT = crate::Reg<stat::STATrs>;
///DDRCTRL operating mode status register
pub mod stat;
/**MRCTRL0 (rw) register accessor: Mode Register Read/Write Control Register 0. Do not enable more than one of the following fields simultaneously: sw_init_int pda_en mpr_en

You can [`read`](crate::Reg::read) this register and get [`mrctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mrctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:MRCTRL0)

For information about available fields see [`mod@mrctrl0`] module*/
pub type MRCTRL0 = crate::Reg<mrctrl0::MRCTRL0rs>;
///Mode Register Read/Write Control Register 0. Do not enable more than one of the following fields simultaneously: sw_init_int pda_en mpr_en
pub mod mrctrl0;
/**MRCTRL1 (rw) register accessor: DDRCTRL mode register read/write control register 1

You can [`read`](crate::Reg::read) this register and get [`mrctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mrctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:MRCTRL1)

For information about available fields see [`mod@mrctrl1`] module*/
pub type MRCTRL1 = crate::Reg<mrctrl1::MRCTRL1rs>;
///DDRCTRL mode register read/write control register 1
pub mod mrctrl1;
/**MRSTAT (r) register accessor: DDRCTRL mode register read/write status register

You can [`read`](crate::Reg::read) this register and get [`mrstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:MRSTAT)

For information about available fields see [`mod@mrstat`] module*/
pub type MRSTAT = crate::Reg<mrstat::MRSTATrs>;
///DDRCTRL mode register read/write status register
pub mod mrstat;
/**DERATEEN (rw) register accessor: DDRCTRL temperature derate enable register

You can [`read`](crate::Reg::read) this register and get [`derateen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`derateen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:DERATEEN)

For information about available fields see [`mod@derateen`] module*/
pub type DERATEEN = crate::Reg<derateen::DERATEENrs>;
///DDRCTRL temperature derate enable register
pub mod derateen;
/**DERATEINT (rw) register accessor: DDRCTRL temperature derate interval register

You can [`read`](crate::Reg::read) this register and get [`derateint::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`derateint::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:DERATEINT)

For information about available fields see [`mod@derateint`] module*/
pub type DERATEINT = crate::Reg<derateint::DERATEINTrs>;
///DDRCTRL temperature derate interval register
pub mod derateint;
/**PWRCTL (rw) register accessor: DDRCTRL low power control register

You can [`read`](crate::Reg::read) this register and get [`pwrctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:PWRCTL)

For information about available fields see [`mod@pwrctl`] module*/
pub type PWRCTL = crate::Reg<pwrctl::PWRCTLrs>;
///DDRCTRL low power control register
pub mod pwrctl;
/**PWRTMG (rw) register accessor: DDRCTRL low power timing register

You can [`read`](crate::Reg::read) this register and get [`pwrtmg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrtmg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:PWRTMG)

For information about available fields see [`mod@pwrtmg`] module*/
pub type PWRTMG = crate::Reg<pwrtmg::PWRTMGrs>;
///DDRCTRL low power timing register
pub mod pwrtmg;
/**HWLPCTL (rw) register accessor: DDRCTRL hardware low power control register

You can [`read`](crate::Reg::read) this register and get [`hwlpctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwlpctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:HWLPCTL)

For information about available fields see [`mod@hwlpctl`] module*/
pub type HWLPCTL = crate::Reg<hwlpctl::HWLPCTLrs>;
///DDRCTRL hardware low power control register
pub mod hwlpctl;
/**RFSHCTL0 (rw) register accessor: DDRCTRL refresh control register 0

You can [`read`](crate::Reg::read) this register and get [`rfshctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfshctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:RFSHCTL0)

For information about available fields see [`mod@rfshctl0`] module*/
pub type RFSHCTL0 = crate::Reg<rfshctl0::RFSHCTL0rs>;
///DDRCTRL refresh control register 0
pub mod rfshctl0;
/**RFSHCTL3 (rw) register accessor: DDRCTRL refresh control register 3

You can [`read`](crate::Reg::read) this register and get [`rfshctl3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfshctl3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:RFSHCTL3)

For information about available fields see [`mod@rfshctl3`] module*/
pub type RFSHCTL3 = crate::Reg<rfshctl3::RFSHCTL3rs>;
///DDRCTRL refresh control register 3
pub mod rfshctl3;
/**RFSHTMG (rw) register accessor: DDRCTRL refresh timing register

You can [`read`](crate::Reg::read) this register and get [`rfshtmg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfshtmg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:RFSHTMG)

For information about available fields see [`mod@rfshtmg`] module*/
pub type RFSHTMG = crate::Reg<rfshtmg::RFSHTMGrs>;
///DDRCTRL refresh timing register
pub mod rfshtmg;
/**CRCPARCTL0 (rw) register accessor: DDRCTRL CRC parity control register 0

You can [`read`](crate::Reg::read) this register and get [`crcparctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcparctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:CRCPARCTL0)

For information about available fields see [`mod@crcparctl0`] module*/
pub type CRCPARCTL0 = crate::Reg<crcparctl0::CRCPARCTL0rs>;
///DDRCTRL CRC parity control register 0
pub mod crcparctl0;
/**CRCPARSTAT (r) register accessor: DDRCTRL CRC parity status register

You can [`read`](crate::Reg::read) this register and get [`crcparstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:CRCPARSTAT)

For information about available fields see [`mod@crcparstat`] module*/
pub type CRCPARSTAT = crate::Reg<crcparstat::CRCPARSTATrs>;
///DDRCTRL CRC parity status register
pub mod crcparstat;
/**INIT0 (rw) register accessor: DDRCTRL SDRAM initialization register 0

You can [`read`](crate::Reg::read) this register and get [`init0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`init0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:INIT0)

For information about available fields see [`mod@init0`] module*/
pub type INIT0 = crate::Reg<init0::INIT0rs>;
///DDRCTRL SDRAM initialization register 0
pub mod init0;
/**INIT1 (rw) register accessor: DDRCTRL SDRAM initialization register 1

You can [`read`](crate::Reg::read) this register and get [`init1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`init1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:INIT1)

For information about available fields see [`mod@init1`] module*/
pub type INIT1 = crate::Reg<init1::INIT1rs>;
///DDRCTRL SDRAM initialization register 1
pub mod init1;
/**INIT2 (rw) register accessor: DDRCTRL SDRAM initialization register 2

You can [`read`](crate::Reg::read) this register and get [`init2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`init2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:INIT2)

For information about available fields see [`mod@init2`] module*/
pub type INIT2 = crate::Reg<init2::INIT2rs>;
///DDRCTRL SDRAM initialization register 2
pub mod init2;
/**INIT3 (rw) register accessor: DDRCTRL SDRAM initialization register 3

You can [`read`](crate::Reg::read) this register and get [`init3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`init3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:INIT3)

For information about available fields see [`mod@init3`] module*/
pub type INIT3 = crate::Reg<init3::INIT3rs>;
///DDRCTRL SDRAM initialization register 3
pub mod init3;
/**INIT4 (rw) register accessor: DDRCTRL SDRAM initialization register 4

You can [`read`](crate::Reg::read) this register and get [`init4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`init4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:INIT4)

For information about available fields see [`mod@init4`] module*/
pub type INIT4 = crate::Reg<init4::INIT4rs>;
///DDRCTRL SDRAM initialization register 4
pub mod init4;
/**INIT5 (rw) register accessor: DDRCTRL SDRAM initialization register 5

You can [`read`](crate::Reg::read) this register and get [`init5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`init5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:INIT5)

For information about available fields see [`mod@init5`] module*/
pub type INIT5 = crate::Reg<init5::INIT5rs>;
///DDRCTRL SDRAM initialization register 5
pub mod init5;
/**DIMMCTL (rw) register accessor: DDRCTRL DIMM control register

You can [`read`](crate::Reg::read) this register and get [`dimmctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dimmctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:DIMMCTL)

For information about available fields see [`mod@dimmctl`] module*/
pub type DIMMCTL = crate::Reg<dimmctl::DIMMCTLrs>;
///DDRCTRL DIMM control register
pub mod dimmctl;
/**DRAMTMG0 (rw) register accessor: DDRCTRL SDRAM timing register 0

You can [`read`](crate::Reg::read) this register and get [`dramtmg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dramtmg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:DRAMTMG0)

For information about available fields see [`mod@dramtmg0`] module*/
pub type DRAMTMG0 = crate::Reg<dramtmg0::DRAMTMG0rs>;
///DDRCTRL SDRAM timing register 0
pub mod dramtmg0;
/**DRAMTMG1 (rw) register accessor: DDRCTRL SDRAM timing register 1

You can [`read`](crate::Reg::read) this register and get [`dramtmg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dramtmg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:DRAMTMG1)

For information about available fields see [`mod@dramtmg1`] module*/
pub type DRAMTMG1 = crate::Reg<dramtmg1::DRAMTMG1rs>;
///DDRCTRL SDRAM timing register 1
pub mod dramtmg1;
/**DRAMTMG2 (rw) register accessor: DDRCTRL SDRAM timing register 2

You can [`read`](crate::Reg::read) this register and get [`dramtmg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dramtmg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:DRAMTMG2)

For information about available fields see [`mod@dramtmg2`] module*/
pub type DRAMTMG2 = crate::Reg<dramtmg2::DRAMTMG2rs>;
///DDRCTRL SDRAM timing register 2
pub mod dramtmg2;
/**DRAMTMG3 (rw) register accessor: DDRCTRL SDRAM timing register 3

You can [`read`](crate::Reg::read) this register and get [`dramtmg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dramtmg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:DRAMTMG3)

For information about available fields see [`mod@dramtmg3`] module*/
pub type DRAMTMG3 = crate::Reg<dramtmg3::DRAMTMG3rs>;
///DDRCTRL SDRAM timing register 3
pub mod dramtmg3;
/**DRAMTMG4 (rw) register accessor: DDRCTRL SDRAM timing register 4

You can [`read`](crate::Reg::read) this register and get [`dramtmg4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dramtmg4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:DRAMTMG4)

For information about available fields see [`mod@dramtmg4`] module*/
pub type DRAMTMG4 = crate::Reg<dramtmg4::DRAMTMG4rs>;
///DDRCTRL SDRAM timing register 4
pub mod dramtmg4;
/**DRAMTMG5 (rw) register accessor: DDRCTRL SDRAM timing register 5

You can [`read`](crate::Reg::read) this register and get [`dramtmg5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dramtmg5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:DRAMTMG5)

For information about available fields see [`mod@dramtmg5`] module*/
pub type DRAMTMG5 = crate::Reg<dramtmg5::DRAMTMG5rs>;
///DDRCTRL SDRAM timing register 5
pub mod dramtmg5;
/**DRAMTMG6 (rw) register accessor: DDRCTRL SDRAM timing register 6

You can [`read`](crate::Reg::read) this register and get [`dramtmg6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dramtmg6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:DRAMTMG6)

For information about available fields see [`mod@dramtmg6`] module*/
pub type DRAMTMG6 = crate::Reg<dramtmg6::DRAMTMG6rs>;
///DDRCTRL SDRAM timing register 6
pub mod dramtmg6;
/**DRAMTMG7 (rw) register accessor: DDRCTRL SDRAM timing register 7

You can [`read`](crate::Reg::read) this register and get [`dramtmg7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dramtmg7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:DRAMTMG7)

For information about available fields see [`mod@dramtmg7`] module*/
pub type DRAMTMG7 = crate::Reg<dramtmg7::DRAMTMG7rs>;
///DDRCTRL SDRAM timing register 7
pub mod dramtmg7;
/**DRAMTMG8 (rw) register accessor: DDRCTRL SDRAM timing register 8

You can [`read`](crate::Reg::read) this register and get [`dramtmg8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dramtmg8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:DRAMTMG8)

For information about available fields see [`mod@dramtmg8`] module*/
pub type DRAMTMG8 = crate::Reg<dramtmg8::DRAMTMG8rs>;
///DDRCTRL SDRAM timing register 8
pub mod dramtmg8;
/**DRAMTMG14 (rw) register accessor: DDRCTRL SDRAM timing register 14

You can [`read`](crate::Reg::read) this register and get [`dramtmg14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dramtmg14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:DRAMTMG14)

For information about available fields see [`mod@dramtmg14`] module*/
pub type DRAMTMG14 = crate::Reg<dramtmg14::DRAMTMG14rs>;
///DDRCTRL SDRAM timing register 14
pub mod dramtmg14;
/**DRAMTMG15 (rw) register accessor: DDRCTRL SDRAM timing register 15

You can [`read`](crate::Reg::read) this register and get [`dramtmg15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dramtmg15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:DRAMTMG15)

For information about available fields see [`mod@dramtmg15`] module*/
pub type DRAMTMG15 = crate::Reg<dramtmg15::DRAMTMG15rs>;
///DDRCTRL SDRAM timing register 15
pub mod dramtmg15;
/**ZQCTL0 (rw) register accessor: DDRCTRL ZQ control register 0

You can [`read`](crate::Reg::read) this register and get [`zqctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`zqctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:ZQCTL0)

For information about available fields see [`mod@zqctl0`] module*/
pub type ZQCTL0 = crate::Reg<zqctl0::ZQCTL0rs>;
///DDRCTRL ZQ control register 0
pub mod zqctl0;
/**ZQCTL1 (rw) register accessor: DDRCTRL ZQ control register 1

You can [`read`](crate::Reg::read) this register and get [`zqctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`zqctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:ZQCTL1)

For information about available fields see [`mod@zqctl1`] module*/
pub type ZQCTL1 = crate::Reg<zqctl1::ZQCTL1rs>;
///DDRCTRL ZQ control register 1
pub mod zqctl1;
/**ZQCTL2 (rw) register accessor: DDRCTRL ZQ control register 2

You can [`read`](crate::Reg::read) this register and get [`zqctl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`zqctl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:ZQCTL2)

For information about available fields see [`mod@zqctl2`] module*/
pub type ZQCTL2 = crate::Reg<zqctl2::ZQCTL2rs>;
///DDRCTRL ZQ control register 2
pub mod zqctl2;
/**ZQSTAT (r) register accessor: DDRCTRL ZQ status register

You can [`read`](crate::Reg::read) this register and get [`zqstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:ZQSTAT)

For information about available fields see [`mod@zqstat`] module*/
pub type ZQSTAT = crate::Reg<zqstat::ZQSTATrs>;
///DDRCTRL ZQ status register
pub mod zqstat;
/**DFITMG0 (rw) register accessor: DDRCTRL DFI timing register 0

You can [`read`](crate::Reg::read) this register and get [`dfitmg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfitmg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:DFITMG0)

For information about available fields see [`mod@dfitmg0`] module*/
pub type DFITMG0 = crate::Reg<dfitmg0::DFITMG0rs>;
///DDRCTRL DFI timing register 0
pub mod dfitmg0;
/**DFITMG1 (rw) register accessor: DDRCTRL DFI timing register 1

You can [`read`](crate::Reg::read) this register and get [`dfitmg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfitmg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:DFITMG1)

For information about available fields see [`mod@dfitmg1`] module*/
pub type DFITMG1 = crate::Reg<dfitmg1::DFITMG1rs>;
///DDRCTRL DFI timing register 1
pub mod dfitmg1;
/**DFILPCFG0 (rw) register accessor: DDRCTRL low power configuration register 0

You can [`read`](crate::Reg::read) this register and get [`dfilpcfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfilpcfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:DFILPCFG0)

For information about available fields see [`mod@dfilpcfg0`] module*/
pub type DFILPCFG0 = crate::Reg<dfilpcfg0::DFILPCFG0rs>;
///DDRCTRL low power configuration register 0
pub mod dfilpcfg0;
/**DFIUPD0 (rw) register accessor: DDRCTRL DFI update register 0

You can [`read`](crate::Reg::read) this register and get [`dfiupd0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfiupd0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:DFIUPD0)

For information about available fields see [`mod@dfiupd0`] module*/
pub type DFIUPD0 = crate::Reg<dfiupd0::DFIUPD0rs>;
///DDRCTRL DFI update register 0
pub mod dfiupd0;
/**DFIUPD1 (rw) register accessor: DDRCTRL DFI update register 1

You can [`read`](crate::Reg::read) this register and get [`dfiupd1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfiupd1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:DFIUPD1)

For information about available fields see [`mod@dfiupd1`] module*/
pub type DFIUPD1 = crate::Reg<dfiupd1::DFIUPD1rs>;
///DDRCTRL DFI update register 1
pub mod dfiupd1;
/**DFIUPD2 (rw) register accessor: DDRCTRL DFI update register 2

You can [`read`](crate::Reg::read) this register and get [`dfiupd2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfiupd2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:DFIUPD2)

For information about available fields see [`mod@dfiupd2`] module*/
pub type DFIUPD2 = crate::Reg<dfiupd2::DFIUPD2rs>;
///DDRCTRL DFI update register 2
pub mod dfiupd2;
/**DFIMISC (rw) register accessor: DDRCTRL DFI miscellaneous control register

You can [`read`](crate::Reg::read) this register and get [`dfimisc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfimisc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:DFIMISC)

For information about available fields see [`mod@dfimisc`] module*/
pub type DFIMISC = crate::Reg<dfimisc::DFIMISCrs>;
///DDRCTRL DFI miscellaneous control register
pub mod dfimisc;
/**DFISTAT (r) register accessor: DDRCTRL DFI status register

You can [`read`](crate::Reg::read) this register and get [`dfistat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:DFISTAT)

For information about available fields see [`mod@dfistat`] module*/
pub type DFISTAT = crate::Reg<dfistat::DFISTATrs>;
///DDRCTRL DFI status register
pub mod dfistat;
/**DFIPHYMSTR (rw) register accessor: DDRCTRL DFI PHY master register

You can [`read`](crate::Reg::read) this register and get [`dfiphymstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfiphymstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:DFIPHYMSTR)

For information about available fields see [`mod@dfiphymstr`] module*/
pub type DFIPHYMSTR = crate::Reg<dfiphymstr::DFIPHYMSTRrs>;
///DDRCTRL DFI PHY master register
pub mod dfiphymstr;
/**ADDRMAP1 (rw) register accessor: DDRCTRL address map register 1

You can [`read`](crate::Reg::read) this register and get [`addrmap1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addrmap1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:ADDRMAP1)

For information about available fields see [`mod@addrmap1`] module*/
pub type ADDRMAP1 = crate::Reg<addrmap1::ADDRMAP1rs>;
///DDRCTRL address map register 1
pub mod addrmap1;
/**ADDRMAP2 (rw) register accessor: DDRCTRL address map register 2

You can [`read`](crate::Reg::read) this register and get [`addrmap2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addrmap2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:ADDRMAP2)

For information about available fields see [`mod@addrmap2`] module*/
pub type ADDRMAP2 = crate::Reg<addrmap2::ADDRMAP2rs>;
///DDRCTRL address map register 2
pub mod addrmap2;
/**ADDRMAP3 (rw) register accessor: DDRCTRL address map register 3

You can [`read`](crate::Reg::read) this register and get [`addrmap3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addrmap3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:ADDRMAP3)

For information about available fields see [`mod@addrmap3`] module*/
pub type ADDRMAP3 = crate::Reg<addrmap3::ADDRMAP3rs>;
///DDRCTRL address map register 3
pub mod addrmap3;
/**ADDRMAP4 (rw) register accessor: DDRCTRL address map register 4

You can [`read`](crate::Reg::read) this register and get [`addrmap4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addrmap4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:ADDRMAP4)

For information about available fields see [`mod@addrmap4`] module*/
pub type ADDRMAP4 = crate::Reg<addrmap4::ADDRMAP4rs>;
///DDRCTRL address map register 4
pub mod addrmap4;
/**ADDRMAP5 (rw) register accessor: DDRCTRL address map register 5

You can [`read`](crate::Reg::read) this register and get [`addrmap5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addrmap5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:ADDRMAP5)

For information about available fields see [`mod@addrmap5`] module*/
pub type ADDRMAP5 = crate::Reg<addrmap5::ADDRMAP5rs>;
///DDRCTRL address map register 5
pub mod addrmap5;
/**ADDRMAP6 (rw) register accessor: DDRCTRL address register 6

You can [`read`](crate::Reg::read) this register and get [`addrmap6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addrmap6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:ADDRMAP6)

For information about available fields see [`mod@addrmap6`] module*/
pub type ADDRMAP6 = crate::Reg<addrmap6::ADDRMAP6rs>;
///DDRCTRL address register 6
pub mod addrmap6;
/**ADDRMAP9 (rw) register accessor: DDRCTRL address map register 9

You can [`read`](crate::Reg::read) this register and get [`addrmap9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addrmap9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:ADDRMAP9)

For information about available fields see [`mod@addrmap9`] module*/
pub type ADDRMAP9 = crate::Reg<addrmap9::ADDRMAP9rs>;
///DDRCTRL address map register 9
pub mod addrmap9;
/**ADDRMAP10 (rw) register accessor: DDRCTRL address map register 10

You can [`read`](crate::Reg::read) this register and get [`addrmap10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addrmap10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:ADDRMAP10)

For information about available fields see [`mod@addrmap10`] module*/
pub type ADDRMAP10 = crate::Reg<addrmap10::ADDRMAP10rs>;
///DDRCTRL address map register 10
pub mod addrmap10;
/**ADDRMAP11 (rw) register accessor: DDRCTRL address map register 11

You can [`read`](crate::Reg::read) this register and get [`addrmap11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addrmap11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:ADDRMAP11)

For information about available fields see [`mod@addrmap11`] module*/
pub type ADDRMAP11 = crate::Reg<addrmap11::ADDRMAP11rs>;
///DDRCTRL address map register 11
pub mod addrmap11;
/**ODTCFG (rw) register accessor: DDRCTRL ODT configuration register

You can [`read`](crate::Reg::read) this register and get [`odtcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`odtcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:ODTCFG)

For information about available fields see [`mod@odtcfg`] module*/
pub type ODTCFG = crate::Reg<odtcfg::ODTCFGrs>;
///DDRCTRL ODT configuration register
pub mod odtcfg;
/**ODTMAP (rw) register accessor: DDRCTRL ODT/Rank map register

You can [`read`](crate::Reg::read) this register and get [`odtmap::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`odtmap::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:ODTMAP)

For information about available fields see [`mod@odtmap`] module*/
pub type ODTMAP = crate::Reg<odtmap::ODTMAPrs>;
///DDRCTRL ODT/Rank map register
pub mod odtmap;
/**SCHED (rw) register accessor: DDRCTRL scheduler control register

You can [`read`](crate::Reg::read) this register and get [`sched::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sched::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:SCHED)

For information about available fields see [`mod@sched`] module*/
pub type SCHED = crate::Reg<sched::SCHEDrs>;
///DDRCTRL scheduler control register
pub mod sched;
/**SCHED1 (rw) register accessor: DDRCTRL scheduler control register 1

You can [`read`](crate::Reg::read) this register and get [`sched1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sched1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:SCHED1)

For information about available fields see [`mod@sched1`] module*/
pub type SCHED1 = crate::Reg<sched1::SCHED1rs>;
///DDRCTRL scheduler control register 1
pub mod sched1;
/**PERFHPR1 (rw) register accessor: DDRCTRL high priority read CAM register 1

You can [`read`](crate::Reg::read) this register and get [`perfhpr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`perfhpr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:PERFHPR1)

For information about available fields see [`mod@perfhpr1`] module*/
pub type PERFHPR1 = crate::Reg<perfhpr1::PERFHPR1rs>;
///DDRCTRL high priority read CAM register 1
pub mod perfhpr1;
/**PERFLPR1 (rw) register accessor: DDRCTRL low priority read CAM register 1

You can [`read`](crate::Reg::read) this register and get [`perflpr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`perflpr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:PERFLPR1)

For information about available fields see [`mod@perflpr1`] module*/
pub type PERFLPR1 = crate::Reg<perflpr1::PERFLPR1rs>;
///DDRCTRL low priority read CAM register 1
pub mod perflpr1;
/**PERFWR1 (rw) register accessor: DDRCTRL write CAM register 1

You can [`read`](crate::Reg::read) this register and get [`perfwr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`perfwr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:PERFWR1)

For information about available fields see [`mod@perfwr1`] module*/
pub type PERFWR1 = crate::Reg<perfwr1::PERFWR1rs>;
///DDRCTRL write CAM register 1
pub mod perfwr1;
/**DBG0 (rw) register accessor: DDRCTRL debug register 0

You can [`read`](crate::Reg::read) this register and get [`dbg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:DBG0)

For information about available fields see [`mod@dbg0`] module*/
pub type DBG0 = crate::Reg<dbg0::DBG0rs>;
///DDRCTRL debug register 0
pub mod dbg0;
/**DBG1 (rw) register accessor: DDRCTRL debug register 1

You can [`read`](crate::Reg::read) this register and get [`dbg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:DBG1)

For information about available fields see [`mod@dbg1`] module*/
pub type DBG1 = crate::Reg<dbg1::DBG1rs>;
///DDRCTRL debug register 1
pub mod dbg1;
/**DBGCAM (r) register accessor: DDRCTRL CAM debug register

You can [`read`](crate::Reg::read) this register and get [`dbgcam::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:DBGCAM)

For information about available fields see [`mod@dbgcam`] module*/
pub type DBGCAM = crate::Reg<dbgcam::DBGCAMrs>;
///DDRCTRL CAM debug register
pub mod dbgcam;
/**DBGCMD (rw) register accessor: DDRCTRL command debug register

You can [`read`](crate::Reg::read) this register and get [`dbgcmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgcmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:DBGCMD)

For information about available fields see [`mod@dbgcmd`] module*/
pub type DBGCMD = crate::Reg<dbgcmd::DBGCMDrs>;
///DDRCTRL command debug register
pub mod dbgcmd;
/**DBGSTAT (r) register accessor: DDRCTRL status debug register

You can [`read`](crate::Reg::read) this register and get [`dbgstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:DBGSTAT)

For information about available fields see [`mod@dbgstat`] module*/
pub type DBGSTAT = crate::Reg<dbgstat::DBGSTATrs>;
///DDRCTRL status debug register
pub mod dbgstat;
/**SWCTL (rw) register accessor: DDRCTRL software register programming control enable

You can [`read`](crate::Reg::read) this register and get [`swctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:SWCTL)

For information about available fields see [`mod@swctl`] module*/
pub type SWCTL = crate::Reg<swctl::SWCTLrs>;
///DDRCTRL software register programming control enable
pub mod swctl;
/**SWSTAT (r) register accessor: DDRCTRL software register programming control status

You can [`read`](crate::Reg::read) this register and get [`swstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:SWSTAT)

For information about available fields see [`mod@swstat`] module*/
pub type SWSTAT = crate::Reg<swstat::SWSTATrs>;
///DDRCTRL software register programming control status
pub mod swstat;
/**POISONCFG (rw) register accessor: AXI Poison configuration register common for all AXI ports.

You can [`read`](crate::Reg::read) this register and get [`poisoncfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`poisoncfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:POISONCFG)

For information about available fields see [`mod@poisoncfg`] module*/
pub type POISONCFG = crate::Reg<poisoncfg::POISONCFGrs>;
///AXI Poison configuration register common for all AXI ports.
pub mod poisoncfg;
/**POISONSTAT (r) register accessor: DDRCTRL AXI Poison status register

You can [`read`](crate::Reg::read) this register and get [`poisonstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:POISONSTAT)

For information about available fields see [`mod@poisonstat`] module*/
pub type POISONSTAT = crate::Reg<poisonstat::POISONSTATrs>;
///DDRCTRL AXI Poison status register
pub mod poisonstat;
/**PSTAT (r) register accessor: DDRCTRL port status register

You can [`read`](crate::Reg::read) this register and get [`pstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:PSTAT)

For information about available fields see [`mod@pstat`] module*/
pub type PSTAT = crate::Reg<pstat::PSTATrs>;
///DDRCTRL port status register
pub mod pstat;
/**PCCFG (rw) register accessor: DDRCTRL port common configuration register

You can [`read`](crate::Reg::read) this register and get [`pccfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pccfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:PCCFG)

For information about available fields see [`mod@pccfg`] module*/
pub type PCCFG = crate::Reg<pccfg::PCCFGrs>;
///DDRCTRL port common configuration register
pub mod pccfg;
/**PCFGR_0 (rw) register accessor: DDRCTRL port 0 configuration read register

You can [`read`](crate::Reg::read) this register and get [`pcfgr_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcfgr_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:PCFGR_0)

For information about available fields see [`mod@pcfgr_0`] module*/
pub type PCFGR_0 = crate::Reg<pcfgr_0::PCFGR_0rs>;
///DDRCTRL port 0 configuration read register
pub mod pcfgr_0;
/**PCFGW_0 (rw) register accessor: DDRCTRL port 0 configuration write register

You can [`read`](crate::Reg::read) this register and get [`pcfgw_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcfgw_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:PCFGW_0)

For information about available fields see [`mod@pcfgw_0`] module*/
pub type PCFGW_0 = crate::Reg<pcfgw_0::PCFGW_0rs>;
///DDRCTRL port 0 configuration write register
pub mod pcfgw_0;
/**PCTRL_0 (rw) register accessor: DDRCTRL port 0 control register

You can [`read`](crate::Reg::read) this register and get [`pctrl_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pctrl_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:PCTRL_0)

For information about available fields see [`mod@pctrl_0`] module*/
pub type PCTRL_0 = crate::Reg<pctrl_0::PCTRL_0rs>;
///DDRCTRL port 0 control register
pub mod pctrl_0;
/**PCFGQOS0_0 (rw) register accessor: DDRCTRL port 0 read Q0S configuration register 0

You can [`read`](crate::Reg::read) this register and get [`pcfgqos0_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcfgqos0_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:PCFGQOS0_0)

For information about available fields see [`mod@pcfgqos0_0`] module*/
pub type PCFGQOS0_0 = crate::Reg<pcfgqos0_0::PCFGQOS0_0rs>;
///DDRCTRL port 0 read Q0S configuration register 0
pub mod pcfgqos0_0;
/**PCFGQOS1_0 (rw) register accessor: DDRCTRL port 0 read Q0S configuration register 1

You can [`read`](crate::Reg::read) this register and get [`pcfgqos1_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcfgqos1_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:PCFGQOS1_0)

For information about available fields see [`mod@pcfgqos1_0`] module*/
pub type PCFGQOS1_0 = crate::Reg<pcfgqos1_0::PCFGQOS1_0rs>;
///DDRCTRL port 0 read Q0S configuration register 1
pub mod pcfgqos1_0;
/**PCFGWQOS0_0 (rw) register accessor: DDRCTRL port 0 write Q0S configuration register 0

You can [`read`](crate::Reg::read) this register and get [`pcfgwqos0_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcfgwqos0_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:PCFGWQOS0_0)

For information about available fields see [`mod@pcfgwqos0_0`] module*/
pub type PCFGWQOS0_0 = crate::Reg<pcfgwqos0_0::PCFGWQOS0_0rs>;
///DDRCTRL port 0 write Q0S configuration register 0
pub mod pcfgwqos0_0;
/**PCFGWQOS1_0 (rw) register accessor: DDRCTRL port 0 write Q0S configuration register 1

You can [`read`](crate::Reg::read) this register and get [`pcfgwqos1_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcfgwqos1_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:PCFGWQOS1_0)

For information about available fields see [`mod@pcfgwqos1_0`] module*/
pub type PCFGWQOS1_0 = crate::Reg<pcfgwqos1_0::PCFGWQOS1_0rs>;
///DDRCTRL port 0 write Q0S configuration register 1
pub mod pcfgwqos1_0;
/**PCFGR_1 (rw) register accessor: DDRCTRL port 1 configuration read register

You can [`read`](crate::Reg::read) this register and get [`pcfgr_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcfgr_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:PCFGR_1)

For information about available fields see [`mod@pcfgr_1`] module*/
pub type PCFGR_1 = crate::Reg<pcfgr_1::PCFGR_1rs>;
///DDRCTRL port 1 configuration read register
pub mod pcfgr_1;
/**PCFGW_1 (rw) register accessor: DDRCTRL port 1 configuration write register

You can [`read`](crate::Reg::read) this register and get [`pcfgw_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcfgw_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:PCFGW_1)

For information about available fields see [`mod@pcfgw_1`] module*/
pub type PCFGW_1 = crate::Reg<pcfgw_1::PCFGW_1rs>;
///DDRCTRL port 1 configuration write register
pub mod pcfgw_1;
/**PCTRL_1 (rw) register accessor: DDRCTRL port 1 control register

You can [`read`](crate::Reg::read) this register and get [`pctrl_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pctrl_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:PCTRL_1)

For information about available fields see [`mod@pctrl_1`] module*/
pub type PCTRL_1 = crate::Reg<pctrl_1::PCTRL_1rs>;
///DDRCTRL port 1 control register
pub mod pctrl_1;
/**PCFGQOS0_1 (rw) register accessor: DDRCTRL port 1 read Q0S configuration register 0

You can [`read`](crate::Reg::read) this register and get [`pcfgqos0_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcfgqos0_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:PCFGQOS0_1)

For information about available fields see [`mod@pcfgqos0_1`] module*/
pub type PCFGQOS0_1 = crate::Reg<pcfgqos0_1::PCFGQOS0_1rs>;
///DDRCTRL port 1 read Q0S configuration register 0
pub mod pcfgqos0_1;
/**PCFGQOS1_1 (rw) register accessor: DDRCTRL port 1 read Q0S configuration register 1

You can [`read`](crate::Reg::read) this register and get [`pcfgqos1_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcfgqos1_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:PCFGQOS1_1)

For information about available fields see [`mod@pcfgqos1_1`] module*/
pub type PCFGQOS1_1 = crate::Reg<pcfgqos1_1::PCFGQOS1_1rs>;
///DDRCTRL port 1 read Q0S configuration register 1
pub mod pcfgqos1_1;
/**PCFGWQOS0_1 (rw) register accessor: DDRCTRL port 1 write Q0S configuration register 0

You can [`read`](crate::Reg::read) this register and get [`pcfgwqos0_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcfgwqos0_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:PCFGWQOS0_1)

For information about available fields see [`mod@pcfgwqos0_1`] module*/
pub type PCFGWQOS0_1 = crate::Reg<pcfgwqos0_1::PCFGWQOS0_1rs>;
///DDRCTRL port 1 write Q0S configuration register 0
pub mod pcfgwqos0_1;
/**PCFGWQOS1_1 (rw) register accessor: DDRCTRL port 1 write Q0S configuration register 1

You can [`read`](crate::Reg::read) this register and get [`pcfgwqos1_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcfgwqos1_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:PCFGWQOS1_1)

For information about available fields see [`mod@pcfgwqos1_1`] module*/
pub type PCFGWQOS1_1 = crate::Reg<pcfgwqos1_1::PCFGWQOS1_1rs>;
///DDRCTRL port 1 write Q0S configuration register 1
pub mod pcfgwqos1_1;
