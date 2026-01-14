#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    power: POWER,
    clkcr: CLKCR,
    argr: ARGR,
    cmdr: CMDR,
    respcmdr: RESPCMDR,
    resp1r: RESP1R,
    resp2r: RESP2R,
    resp3r: RESP3R,
    resp4r: RESP4R,
    dtimer: DTIMER,
    dlenr: DLENR,
    dctrl: DCTRL,
    dcntr: DCNTR,
    star: STAR,
    icr: ICR,
    maskr: MASKR,
    acktimer: ACKTIMER,
    _reserved17: [u8; 0x0c],
    idmactrlr: IDMACTRLR,
    idmabsizer: IDMABSIZER,
    idmabaser: IDMABASER,
    _reserved20: [u8; 0x08],
    idmalar: IDMALAR,
    idmabar: IDMABAR,
    _reserved22: [u8; 0x14],
    fifor: [FIFOR; 16],
}
impl RegisterBlock {
    ///0x00 - SDMMC power control register
    #[inline(always)]
    pub const fn power(&self) -> &POWER {
        &self.power
    }
    ///0x04 - SDMMC clock control register
    #[inline(always)]
    pub const fn clkcr(&self) -> &CLKCR {
        &self.clkcr
    }
    ///0x08 - SDMMC argument register
    #[inline(always)]
    pub const fn argr(&self) -> &ARGR {
        &self.argr
    }
    ///0x0c - SDMMC command register
    #[inline(always)]
    pub const fn cmdr(&self) -> &CMDR {
        &self.cmdr
    }
    ///0x10 - SDMMC command response register
    #[inline(always)]
    pub const fn respcmdr(&self) -> &RESPCMDR {
        &self.respcmdr
    }
    ///0x14 - SDMMC response 1 register
    #[inline(always)]
    pub const fn resp1r(&self) -> &RESP1R {
        &self.resp1r
    }
    ///0x18 - SDMMC response 2 register
    #[inline(always)]
    pub const fn resp2r(&self) -> &RESP2R {
        &self.resp2r
    }
    ///0x1c - SDMMC response 3 register
    #[inline(always)]
    pub const fn resp3r(&self) -> &RESP3R {
        &self.resp3r
    }
    ///0x20 - SDMMC response 4 register
    #[inline(always)]
    pub const fn resp4r(&self) -> &RESP4R {
        &self.resp4r
    }
    ///0x24 - SDMMC data timer register
    #[inline(always)]
    pub const fn dtimer(&self) -> &DTIMER {
        &self.dtimer
    }
    ///0x28 - SDMMC data length register
    #[inline(always)]
    pub const fn dlenr(&self) -> &DLENR {
        &self.dlenr
    }
    ///0x2c - SDMMC data control register
    #[inline(always)]
    pub const fn dctrl(&self) -> &DCTRL {
        &self.dctrl
    }
    ///0x30 - SDMMC data counter register
    #[inline(always)]
    pub const fn dcntr(&self) -> &DCNTR {
        &self.dcntr
    }
    ///0x34 - SDMMC status register
    #[inline(always)]
    pub const fn star(&self) -> &STAR {
        &self.star
    }
    ///0x38 - SDMMC interrupt clear register
    #[inline(always)]
    pub const fn icr(&self) -> &ICR {
        &self.icr
    }
    ///0x3c - SDMMC mask register
    #[inline(always)]
    pub const fn maskr(&self) -> &MASKR {
        &self.maskr
    }
    ///0x40 - SDMMC acknowledgment timer register
    #[inline(always)]
    pub const fn acktimer(&self) -> &ACKTIMER {
        &self.acktimer
    }
    ///0x50 - SDMMC DMA control register
    #[inline(always)]
    pub const fn idmactrlr(&self) -> &IDMACTRLR {
        &self.idmactrlr
    }
    ///0x54 - SDMMC IDMA buffer size register
    #[inline(always)]
    pub const fn idmabsizer(&self) -> &IDMABSIZER {
        &self.idmabsizer
    }
    ///0x58 - SDMMC IDMA buffer base address register
    #[inline(always)]
    pub const fn idmabaser(&self) -> &IDMABASER {
        &self.idmabaser
    }
    ///0x64 - SDMMC IDMA linked list address register
    #[inline(always)]
    pub const fn idmalar(&self) -> &IDMALAR {
        &self.idmalar
    }
    ///0x68 - SDMMC IDMA linked list memory base register
    #[inline(always)]
    pub const fn idmabar(&self) -> &IDMABAR {
        &self.idmabar
    }
    ///0x80..0xc0 - SDMMC data FIFO registers %s
    #[inline(always)]
    pub const fn fifor(&self, n: usize) -> &FIFOR {
        &self.fifor[n]
    }
    ///Iterator for array of:
    ///0x80..0xc0 - SDMMC data FIFO registers %s
    #[inline(always)]
    pub fn fifor_iter(&self) -> impl Iterator<Item = &FIFOR> {
        self.fifor.iter()
    }
}
/**POWER (rw) register accessor: SDMMC power control register

You can [`read`](crate::Reg::read) this register and get [`power::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#SDMMC1:POWER)

For information about available fields see [`mod@power`] module*/
pub type POWER = crate::Reg<power::POWERrs>;
///SDMMC power control register
pub mod power;
/**CLKCR (rw) register accessor: SDMMC clock control register

You can [`read`](crate::Reg::read) this register and get [`clkcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#SDMMC1:CLKCR)

For information about available fields see [`mod@clkcr`] module*/
pub type CLKCR = crate::Reg<clkcr::CLKCRrs>;
///SDMMC clock control register
pub mod clkcr;
/**ARGR (rw) register accessor: SDMMC argument register

You can [`read`](crate::Reg::read) this register and get [`argr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`argr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#SDMMC1:ARGR)

For information about available fields see [`mod@argr`] module*/
pub type ARGR = crate::Reg<argr::ARGRrs>;
///SDMMC argument register
pub mod argr;
/**CMDR (rw) register accessor: SDMMC command register

You can [`read`](crate::Reg::read) this register and get [`cmdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#SDMMC1:CMDR)

For information about available fields see [`mod@cmdr`] module*/
pub type CMDR = crate::Reg<cmdr::CMDRrs>;
///SDMMC command register
pub mod cmdr;
/**RESPCMDR (r) register accessor: SDMMC command response register

You can [`read`](crate::Reg::read) this register and get [`respcmdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#SDMMC1:RESPCMDR)

For information about available fields see [`mod@respcmdr`] module*/
pub type RESPCMDR = crate::Reg<respcmdr::RESPCMDRrs>;
///SDMMC command response register
pub mod respcmdr;
/**RESP1R (r) register accessor: SDMMC response 1 register

You can [`read`](crate::Reg::read) this register and get [`resp1r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#SDMMC1:RESP1R)

For information about available fields see [`mod@resp1r`] module*/
pub type RESP1R = crate::Reg<resp1r::RESP1Rrs>;
///SDMMC response 1 register
pub mod resp1r;
/**RESP2R (r) register accessor: SDMMC response 2 register

You can [`read`](crate::Reg::read) this register and get [`resp2r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#SDMMC1:RESP2R)

For information about available fields see [`mod@resp2r`] module*/
pub type RESP2R = crate::Reg<resp2r::RESP2Rrs>;
///SDMMC response 2 register
pub mod resp2r;
/**RESP3R (r) register accessor: SDMMC response 3 register

You can [`read`](crate::Reg::read) this register and get [`resp3r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#SDMMC1:RESP3R)

For information about available fields see [`mod@resp3r`] module*/
pub type RESP3R = crate::Reg<resp3r::RESP3Rrs>;
///SDMMC response 3 register
pub mod resp3r;
/**RESP4R (r) register accessor: SDMMC response 4 register

You can [`read`](crate::Reg::read) this register and get [`resp4r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#SDMMC1:RESP4R)

For information about available fields see [`mod@resp4r`] module*/
pub type RESP4R = crate::Reg<resp4r::RESP4Rrs>;
///SDMMC response 4 register
pub mod resp4r;
/**DTIMER (rw) register accessor: SDMMC data timer register

You can [`read`](crate::Reg::read) this register and get [`dtimer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtimer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#SDMMC1:DTIMER)

For information about available fields see [`mod@dtimer`] module*/
pub type DTIMER = crate::Reg<dtimer::DTIMERrs>;
///SDMMC data timer register
pub mod dtimer;
/**DLENR (rw) register accessor: SDMMC data length register

You can [`read`](crate::Reg::read) this register and get [`dlenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dlenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#SDMMC1:DLENR)

For information about available fields see [`mod@dlenr`] module*/
pub type DLENR = crate::Reg<dlenr::DLENRrs>;
///SDMMC data length register
pub mod dlenr;
/**DCTRL (rw) register accessor: SDMMC data control register

You can [`read`](crate::Reg::read) this register and get [`dctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#SDMMC1:DCTRL)

For information about available fields see [`mod@dctrl`] module*/
pub type DCTRL = crate::Reg<dctrl::DCTRLrs>;
///SDMMC data control register
pub mod dctrl;
/**DCNTR (r) register accessor: SDMMC data counter register

You can [`read`](crate::Reg::read) this register and get [`dcntr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#SDMMC1:DCNTR)

For information about available fields see [`mod@dcntr`] module*/
pub type DCNTR = crate::Reg<dcntr::DCNTRrs>;
///SDMMC data counter register
pub mod dcntr;
/**STAR (r) register accessor: SDMMC status register

You can [`read`](crate::Reg::read) this register and get [`star::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#SDMMC1:STAR)

For information about available fields see [`mod@star`] module*/
pub type STAR = crate::Reg<star::STARrs>;
///SDMMC status register
pub mod star;
/**ICR (rw) register accessor: SDMMC interrupt clear register

You can [`read`](crate::Reg::read) this register and get [`icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#SDMMC1:ICR)

For information about available fields see [`mod@icr`] module*/
pub type ICR = crate::Reg<icr::ICRrs>;
///SDMMC interrupt clear register
pub mod icr;
/**MASKR (rw) register accessor: SDMMC mask register

You can [`read`](crate::Reg::read) this register and get [`maskr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maskr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#SDMMC1:MASKR)

For information about available fields see [`mod@maskr`] module*/
pub type MASKR = crate::Reg<maskr::MASKRrs>;
///SDMMC mask register
pub mod maskr;
/**ACKTIMER (rw) register accessor: SDMMC acknowledgment timer register

You can [`read`](crate::Reg::read) this register and get [`acktimer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acktimer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#SDMMC1:ACKTIMER)

For information about available fields see [`mod@acktimer`] module*/
pub type ACKTIMER = crate::Reg<acktimer::ACKTIMERrs>;
///SDMMC acknowledgment timer register
pub mod acktimer;
/**IDMACTRLR (rw) register accessor: SDMMC DMA control register

You can [`read`](crate::Reg::read) this register and get [`idmactrlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idmactrlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#SDMMC1:IDMACTRLR)

For information about available fields see [`mod@idmactrlr`] module*/
pub type IDMACTRLR = crate::Reg<idmactrlr::IDMACTRLRrs>;
///SDMMC DMA control register
pub mod idmactrlr;
/**IDMABSIZER (rw) register accessor: SDMMC IDMA buffer size register

You can [`read`](crate::Reg::read) this register and get [`idmabsizer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idmabsizer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#SDMMC1:IDMABSIZER)

For information about available fields see [`mod@idmabsizer`] module*/
pub type IDMABSIZER = crate::Reg<idmabsizer::IDMABSIZERrs>;
///SDMMC IDMA buffer size register
pub mod idmabsizer;
/**IDMABASER (rw) register accessor: SDMMC IDMA buffer base address register

You can [`read`](crate::Reg::read) this register and get [`idmabaser::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idmabaser::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#SDMMC1:IDMABASER)

For information about available fields see [`mod@idmabaser`] module*/
pub type IDMABASER = crate::Reg<idmabaser::IDMABASERrs>;
///SDMMC IDMA buffer base address register
pub mod idmabaser;
/**IDMALAR (rw) register accessor: SDMMC IDMA linked list address register

You can [`read`](crate::Reg::read) this register and get [`idmalar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idmalar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#SDMMC1:IDMALAR)

For information about available fields see [`mod@idmalar`] module*/
pub type IDMALAR = crate::Reg<idmalar::IDMALARrs>;
///SDMMC IDMA linked list address register
pub mod idmalar;
/**IDMABAR (rw) register accessor: SDMMC IDMA linked list memory base register

You can [`read`](crate::Reg::read) this register and get [`idmabar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idmabar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#SDMMC1:IDMABAR)

For information about available fields see [`mod@idmabar`] module*/
pub type IDMABAR = crate::Reg<idmabar::IDMABARrs>;
///SDMMC IDMA linked list memory base register
pub mod idmabar;
/**FIFOR (rw) register accessor: SDMMC data FIFO registers %s

You can [`read`](crate::Reg::read) this register and get [`fifor::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifor::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#SDMMC1:FIFOR[0])

For information about available fields see [`mod@fifor`] module*/
pub type FIFOR = crate::Reg<fifor::FIFORrs>;
///SDMMC data FIFO registers %s
pub mod fifor;
