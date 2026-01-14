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
    fifothrr: FIFOTHRR,
    _reserved18: [u8; 0x08],
    idmactrlr: IDMACTRLR,
    idmabsizer: IDMABSIZER,
    idmabaser: IDMABASER,
    _reserved21: [u8; 0x08],
    idmalar: IDMALAR,
    idmabar: IDMABAR,
    _reserved23: [u8; 0x14],
    fifor0: FIFOR0,
    fifor1: FIFOR1,
    fifor2: FIFOR2,
    fifor3: FIFOR3,
    fifor4: FIFOR4,
    fifor5: FIFOR5,
    fifor6: FIFOR6,
    fifor7: FIFOR7,
    fifor8: FIFOR8,
    fifor9: FIFOR9,
    fifor10: FIFOR10,
    fifor11: FIFOR11,
    fifor12: FIFOR12,
    fifor13: FIFOR13,
    fifor14: FIFOR14,
    fifor15: FIFOR15,
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
    ///0x44 - SDMMC data FIFO threshold register
    #[inline(always)]
    pub const fn fifothrr(&self) -> &FIFOTHRR {
        &self.fifothrr
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
    ///0x80 - SDMMC data FIFO registers 0
    #[inline(always)]
    pub const fn fifor0(&self) -> &FIFOR0 {
        &self.fifor0
    }
    ///0x84 - SDMMC data FIFO registers 1
    #[inline(always)]
    pub const fn fifor1(&self) -> &FIFOR1 {
        &self.fifor1
    }
    ///0x88 - SDMMC data FIFO registers 2
    #[inline(always)]
    pub const fn fifor2(&self) -> &FIFOR2 {
        &self.fifor2
    }
    ///0x8c - SDMMC data FIFO registers 3
    #[inline(always)]
    pub const fn fifor3(&self) -> &FIFOR3 {
        &self.fifor3
    }
    ///0x90 - SDMMC data FIFO registers 4
    #[inline(always)]
    pub const fn fifor4(&self) -> &FIFOR4 {
        &self.fifor4
    }
    ///0x94 - SDMMC data FIFO registers 5
    #[inline(always)]
    pub const fn fifor5(&self) -> &FIFOR5 {
        &self.fifor5
    }
    ///0x98 - SDMMC data FIFO registers 6
    #[inline(always)]
    pub const fn fifor6(&self) -> &FIFOR6 {
        &self.fifor6
    }
    ///0x9c - SDMMC data FIFO registers 7
    #[inline(always)]
    pub const fn fifor7(&self) -> &FIFOR7 {
        &self.fifor7
    }
    ///0xa0 - SDMMC data FIFO registers 8
    #[inline(always)]
    pub const fn fifor8(&self) -> &FIFOR8 {
        &self.fifor8
    }
    ///0xa4 - SDMMC data FIFO registers 9
    #[inline(always)]
    pub const fn fifor9(&self) -> &FIFOR9 {
        &self.fifor9
    }
    ///0xa8 - SDMMC data FIFO registers 10
    #[inline(always)]
    pub const fn fifor10(&self) -> &FIFOR10 {
        &self.fifor10
    }
    ///0xac - SDMMC data FIFO registers 11
    #[inline(always)]
    pub const fn fifor11(&self) -> &FIFOR11 {
        &self.fifor11
    }
    ///0xb0 - SDMMC data FIFO registers 12
    #[inline(always)]
    pub const fn fifor12(&self) -> &FIFOR12 {
        &self.fifor12
    }
    ///0xb4 - SDMMC data FIFO registers 13
    #[inline(always)]
    pub const fn fifor13(&self) -> &FIFOR13 {
        &self.fifor13
    }
    ///0xb8 - SDMMC data FIFO registers 14
    #[inline(always)]
    pub const fn fifor14(&self) -> &FIFOR14 {
        &self.fifor14
    }
    ///0xbc - SDMMC data FIFO registers 15
    #[inline(always)]
    pub const fn fifor15(&self) -> &FIFOR15 {
        &self.fifor15
    }
}
/**POWER (rw) register accessor: SDMMC power control register

You can [`read`](crate::Reg::read) this register and get [`power::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#SDMMC1:POWER)

For information about available fields see [`mod@power`] module*/
pub type POWER = crate::Reg<power::POWERrs>;
///SDMMC power control register
pub mod power;
/**CLKCR (rw) register accessor: SDMMC clock control register

You can [`read`](crate::Reg::read) this register and get [`clkcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#SDMMC1:CLKCR)

For information about available fields see [`mod@clkcr`] module*/
pub type CLKCR = crate::Reg<clkcr::CLKCRrs>;
///SDMMC clock control register
pub mod clkcr;
/**ARGR (rw) register accessor: SDMMC argument register

You can [`read`](crate::Reg::read) this register and get [`argr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`argr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#SDMMC1:ARGR)

For information about available fields see [`mod@argr`] module*/
pub type ARGR = crate::Reg<argr::ARGRrs>;
///SDMMC argument register
pub mod argr;
/**CMDR (rw) register accessor: SDMMC command register

You can [`read`](crate::Reg::read) this register and get [`cmdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#SDMMC1:CMDR)

For information about available fields see [`mod@cmdr`] module*/
pub type CMDR = crate::Reg<cmdr::CMDRrs>;
///SDMMC command register
pub mod cmdr;
/**RESPCMDR (r) register accessor: SDMMC command response register

You can [`read`](crate::Reg::read) this register and get [`respcmdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#SDMMC1:RESPCMDR)

For information about available fields see [`mod@respcmdr`] module*/
pub type RESPCMDR = crate::Reg<respcmdr::RESPCMDRrs>;
///SDMMC command response register
pub mod respcmdr;
/**RESP1R (r) register accessor: SDMMC response 1 register

You can [`read`](crate::Reg::read) this register and get [`resp1r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#SDMMC1:RESP1R)

For information about available fields see [`mod@resp1r`] module*/
pub type RESP1R = crate::Reg<resp1r::RESP1Rrs>;
///SDMMC response 1 register
pub mod resp1r;
/**RESP2R (r) register accessor: SDMMC response 2 register

You can [`read`](crate::Reg::read) this register and get [`resp2r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#SDMMC1:RESP2R)

For information about available fields see [`mod@resp2r`] module*/
pub type RESP2R = crate::Reg<resp2r::RESP2Rrs>;
///SDMMC response 2 register
pub mod resp2r;
/**RESP3R (r) register accessor: SDMMC response 3 register

You can [`read`](crate::Reg::read) this register and get [`resp3r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#SDMMC1:RESP3R)

For information about available fields see [`mod@resp3r`] module*/
pub type RESP3R = crate::Reg<resp3r::RESP3Rrs>;
///SDMMC response 3 register
pub mod resp3r;
/**RESP4R (r) register accessor: SDMMC response 4 register

You can [`read`](crate::Reg::read) this register and get [`resp4r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#SDMMC1:RESP4R)

For information about available fields see [`mod@resp4r`] module*/
pub type RESP4R = crate::Reg<resp4r::RESP4Rrs>;
///SDMMC response 4 register
pub mod resp4r;
/**DTIMER (rw) register accessor: SDMMC data timer register

You can [`read`](crate::Reg::read) this register and get [`dtimer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtimer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#SDMMC1:DTIMER)

For information about available fields see [`mod@dtimer`] module*/
pub type DTIMER = crate::Reg<dtimer::DTIMERrs>;
///SDMMC data timer register
pub mod dtimer;
/**DLENR (rw) register accessor: SDMMC data length register

You can [`read`](crate::Reg::read) this register and get [`dlenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dlenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#SDMMC1:DLENR)

For information about available fields see [`mod@dlenr`] module*/
pub type DLENR = crate::Reg<dlenr::DLENRrs>;
///SDMMC data length register
pub mod dlenr;
/**DCTRL (rw) register accessor: SDMMC data control register

You can [`read`](crate::Reg::read) this register and get [`dctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#SDMMC1:DCTRL)

For information about available fields see [`mod@dctrl`] module*/
pub type DCTRL = crate::Reg<dctrl::DCTRLrs>;
///SDMMC data control register
pub mod dctrl;
/**DCNTR (r) register accessor: SDMMC data counter register

You can [`read`](crate::Reg::read) this register and get [`dcntr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#SDMMC1:DCNTR)

For information about available fields see [`mod@dcntr`] module*/
pub type DCNTR = crate::Reg<dcntr::DCNTRrs>;
///SDMMC data counter register
pub mod dcntr;
/**STAR (r) register accessor: SDMMC status register

You can [`read`](crate::Reg::read) this register and get [`star::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#SDMMC1:STAR)

For information about available fields see [`mod@star`] module*/
pub type STAR = crate::Reg<star::STARrs>;
///SDMMC status register
pub mod star;
/**ICR (rw) register accessor: SDMMC interrupt clear register

You can [`read`](crate::Reg::read) this register and get [`icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#SDMMC1:ICR)

For information about available fields see [`mod@icr`] module*/
pub type ICR = crate::Reg<icr::ICRrs>;
///SDMMC interrupt clear register
pub mod icr;
/**MASKR (rw) register accessor: SDMMC mask register

You can [`read`](crate::Reg::read) this register and get [`maskr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maskr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#SDMMC1:MASKR)

For information about available fields see [`mod@maskr`] module*/
pub type MASKR = crate::Reg<maskr::MASKRrs>;
///SDMMC mask register
pub mod maskr;
/**ACKTIMER (rw) register accessor: SDMMC acknowledgment timer register

You can [`read`](crate::Reg::read) this register and get [`acktimer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acktimer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#SDMMC1:ACKTIMER)

For information about available fields see [`mod@acktimer`] module*/
pub type ACKTIMER = crate::Reg<acktimer::ACKTIMERrs>;
///SDMMC acknowledgment timer register
pub mod acktimer;
/**FIFOTHRR (rw) register accessor: SDMMC data FIFO threshold register

You can [`read`](crate::Reg::read) this register and get [`fifothrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifothrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#SDMMC1:FIFOTHRR)

For information about available fields see [`mod@fifothrr`] module*/
pub type FIFOTHRR = crate::Reg<fifothrr::FIFOTHRRrs>;
///SDMMC data FIFO threshold register
pub mod fifothrr;
/**IDMACTRLR (rw) register accessor: SDMMC DMA control register

You can [`read`](crate::Reg::read) this register and get [`idmactrlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idmactrlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#SDMMC1:IDMACTRLR)

For information about available fields see [`mod@idmactrlr`] module*/
pub type IDMACTRLR = crate::Reg<idmactrlr::IDMACTRLRrs>;
///SDMMC DMA control register
pub mod idmactrlr;
/**IDMABSIZER (rw) register accessor: SDMMC IDMA buffer size register

You can [`read`](crate::Reg::read) this register and get [`idmabsizer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idmabsizer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#SDMMC1:IDMABSIZER)

For information about available fields see [`mod@idmabsizer`] module*/
pub type IDMABSIZER = crate::Reg<idmabsizer::IDMABSIZERrs>;
///SDMMC IDMA buffer size register
pub mod idmabsizer;
/**IDMABASER (rw) register accessor: SDMMC IDMA buffer base address register

You can [`read`](crate::Reg::read) this register and get [`idmabaser::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idmabaser::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#SDMMC1:IDMABASER)

For information about available fields see [`mod@idmabaser`] module*/
pub type IDMABASER = crate::Reg<idmabaser::IDMABASERrs>;
///SDMMC IDMA buffer base address register
pub mod idmabaser;
/**IDMALAR (rw) register accessor: SDMMC IDMA linked list address register

You can [`read`](crate::Reg::read) this register and get [`idmalar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idmalar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#SDMMC1:IDMALAR)

For information about available fields see [`mod@idmalar`] module*/
pub type IDMALAR = crate::Reg<idmalar::IDMALARrs>;
///SDMMC IDMA linked list address register
pub mod idmalar;
/**IDMABAR (rw) register accessor: SDMMC IDMA linked list memory base register

You can [`read`](crate::Reg::read) this register and get [`idmabar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idmabar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#SDMMC1:IDMABAR)

For information about available fields see [`mod@idmabar`] module*/
pub type IDMABAR = crate::Reg<idmabar::IDMABARrs>;
///SDMMC IDMA linked list memory base register
pub mod idmabar;
/**FIFOR0 (rw) register accessor: SDMMC data FIFO registers 0

You can [`read`](crate::Reg::read) this register and get [`fifor0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifor0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#SDMMC1:FIFOR0)

For information about available fields see [`mod@fifor0`] module*/
pub type FIFOR0 = crate::Reg<fifor0::FIFOR0rs>;
///SDMMC data FIFO registers 0
pub mod fifor0;
/**FIFOR1 (rw) register accessor: SDMMC data FIFO registers 1

You can [`read`](crate::Reg::read) this register and get [`fifor1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifor1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#SDMMC1:FIFOR1)

For information about available fields see [`mod@fifor1`] module*/
pub type FIFOR1 = crate::Reg<fifor1::FIFOR1rs>;
///SDMMC data FIFO registers 1
pub mod fifor1;
/**FIFOR2 (rw) register accessor: SDMMC data FIFO registers 2

You can [`read`](crate::Reg::read) this register and get [`fifor2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifor2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#SDMMC1:FIFOR2)

For information about available fields see [`mod@fifor2`] module*/
pub type FIFOR2 = crate::Reg<fifor2::FIFOR2rs>;
///SDMMC data FIFO registers 2
pub mod fifor2;
/**FIFOR3 (rw) register accessor: SDMMC data FIFO registers 3

You can [`read`](crate::Reg::read) this register and get [`fifor3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifor3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#SDMMC1:FIFOR3)

For information about available fields see [`mod@fifor3`] module*/
pub type FIFOR3 = crate::Reg<fifor3::FIFOR3rs>;
///SDMMC data FIFO registers 3
pub mod fifor3;
/**FIFOR4 (rw) register accessor: SDMMC data FIFO registers 4

You can [`read`](crate::Reg::read) this register and get [`fifor4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifor4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#SDMMC1:FIFOR4)

For information about available fields see [`mod@fifor4`] module*/
pub type FIFOR4 = crate::Reg<fifor4::FIFOR4rs>;
///SDMMC data FIFO registers 4
pub mod fifor4;
/**FIFOR5 (rw) register accessor: SDMMC data FIFO registers 5

You can [`read`](crate::Reg::read) this register and get [`fifor5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifor5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#SDMMC1:FIFOR5)

For information about available fields see [`mod@fifor5`] module*/
pub type FIFOR5 = crate::Reg<fifor5::FIFOR5rs>;
///SDMMC data FIFO registers 5
pub mod fifor5;
/**FIFOR6 (rw) register accessor: SDMMC data FIFO registers 6

You can [`read`](crate::Reg::read) this register and get [`fifor6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifor6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#SDMMC1:FIFOR6)

For information about available fields see [`mod@fifor6`] module*/
pub type FIFOR6 = crate::Reg<fifor6::FIFOR6rs>;
///SDMMC data FIFO registers 6
pub mod fifor6;
/**FIFOR7 (rw) register accessor: SDMMC data FIFO registers 7

You can [`read`](crate::Reg::read) this register and get [`fifor7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifor7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#SDMMC1:FIFOR7)

For information about available fields see [`mod@fifor7`] module*/
pub type FIFOR7 = crate::Reg<fifor7::FIFOR7rs>;
///SDMMC data FIFO registers 7
pub mod fifor7;
/**FIFOR8 (rw) register accessor: SDMMC data FIFO registers 8

You can [`read`](crate::Reg::read) this register and get [`fifor8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifor8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#SDMMC1:FIFOR8)

For information about available fields see [`mod@fifor8`] module*/
pub type FIFOR8 = crate::Reg<fifor8::FIFOR8rs>;
///SDMMC data FIFO registers 8
pub mod fifor8;
/**FIFOR9 (rw) register accessor: SDMMC data FIFO registers 9

You can [`read`](crate::Reg::read) this register and get [`fifor9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifor9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#SDMMC1:FIFOR9)

For information about available fields see [`mod@fifor9`] module*/
pub type FIFOR9 = crate::Reg<fifor9::FIFOR9rs>;
///SDMMC data FIFO registers 9
pub mod fifor9;
/**FIFOR10 (rw) register accessor: SDMMC data FIFO registers 10

You can [`read`](crate::Reg::read) this register and get [`fifor10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifor10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#SDMMC1:FIFOR10)

For information about available fields see [`mod@fifor10`] module*/
pub type FIFOR10 = crate::Reg<fifor10::FIFOR10rs>;
///SDMMC data FIFO registers 10
pub mod fifor10;
/**FIFOR11 (rw) register accessor: SDMMC data FIFO registers 11

You can [`read`](crate::Reg::read) this register and get [`fifor11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifor11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#SDMMC1:FIFOR11)

For information about available fields see [`mod@fifor11`] module*/
pub type FIFOR11 = crate::Reg<fifor11::FIFOR11rs>;
///SDMMC data FIFO registers 11
pub mod fifor11;
/**FIFOR12 (rw) register accessor: SDMMC data FIFO registers 12

You can [`read`](crate::Reg::read) this register and get [`fifor12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifor12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#SDMMC1:FIFOR12)

For information about available fields see [`mod@fifor12`] module*/
pub type FIFOR12 = crate::Reg<fifor12::FIFOR12rs>;
///SDMMC data FIFO registers 12
pub mod fifor12;
/**FIFOR13 (rw) register accessor: SDMMC data FIFO registers 13

You can [`read`](crate::Reg::read) this register and get [`fifor13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifor13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#SDMMC1:FIFOR13)

For information about available fields see [`mod@fifor13`] module*/
pub type FIFOR13 = crate::Reg<fifor13::FIFOR13rs>;
///SDMMC data FIFO registers 13
pub mod fifor13;
/**FIFOR14 (rw) register accessor: SDMMC data FIFO registers 14

You can [`read`](crate::Reg::read) this register and get [`fifor14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifor14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#SDMMC1:FIFOR14)

For information about available fields see [`mod@fifor14`] module*/
pub type FIFOR14 = crate::Reg<fifor14::FIFOR14rs>;
///SDMMC data FIFO registers 14
pub mod fifor14;
/**FIFOR15 (rw) register accessor: SDMMC data FIFO registers 15

You can [`read`](crate::Reg::read) this register and get [`fifor15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifor15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#SDMMC1:FIFOR15)

For information about available fields see [`mod@fifor15`] module*/
pub type FIFOR15 = crate::Reg<fifor15::FIFOR15rs>;
///SDMMC data FIFO registers 15
pub mod fifor15;
