#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    _reserved_0_cr: [u8; 0x04],
    cfgr: CFGR,
    _reserved2: [u8; 0x08],
    rdr: RDR,
    rdwr: RDWR,
    tdr: TDR,
    tdwr: TDWR,
    ibidr: IBIDR,
    tgttdr: TGTTDR,
    _reserved8: [u8; 0x08],
    sr: SR,
    ser: SER,
    _reserved10: [u8; 0x08],
    rmr: RMR,
    _reserved11: [u8; 0x0c],
    evr: EVR,
    ier: IER,
    cevr: CEVR,
    _reserved14: [u8; 0x04],
    devr0: DEVR0,
    devr1: DEVR1,
    devr2: DEVR2,
    devr3: DEVR3,
    devr4: DEVR4,
    _reserved19: [u8; 0x1c],
    maxrlr: MAXRLR,
    maxwlr: MAXWLR,
    _reserved21: [u8; 0x08],
    timingr0: TIMINGR0,
    timingr1: TIMINGR1,
    timingr2: TIMINGR2,
    _reserved24: [u8; 0x14],
    bcr: BCR,
    dcr: DCR,
    getcapr: GETCAPR,
    crcapr: CRCAPR,
    getmxdsr: GETMXDSR,
    epidr: EPIDR,
}
impl RegisterBlock {
    ///0x00 - I3C message control register alternate
    #[inline(always)]
    pub const fn cr_alternate(&self) -> &CR_ALTERNATE {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    ///0x00 - I3C message control register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    ///0x04 - I3C configuration register
    #[inline(always)]
    pub const fn cfgr(&self) -> &CFGR {
        &self.cfgr
    }
    ///0x10 - I3C receive data byte register
    #[inline(always)]
    pub const fn rdr(&self) -> &RDR {
        &self.rdr
    }
    ///0x14 - I3C receive data word register
    #[inline(always)]
    pub const fn rdwr(&self) -> &RDWR {
        &self.rdwr
    }
    ///0x18 - I3C transmit data byte register
    #[inline(always)]
    pub const fn tdr(&self) -> &TDR {
        &self.tdr
    }
    ///0x1c - I3C transmit data word register
    #[inline(always)]
    pub const fn tdwr(&self) -> &TDWR {
        &self.tdwr
    }
    ///0x20 - I3C IBI payload data register
    #[inline(always)]
    pub const fn ibidr(&self) -> &IBIDR {
        &self.ibidr
    }
    ///0x24 - I3C target transmit configuration register
    #[inline(always)]
    pub const fn tgttdr(&self) -> &TGTTDR {
        &self.tgttdr
    }
    ///0x30 - I3C status register
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0x34 - I3C status error register
    #[inline(always)]
    pub const fn ser(&self) -> &SER {
        &self.ser
    }
    ///0x40 - I3C received message register
    #[inline(always)]
    pub const fn rmr(&self) -> &RMR {
        &self.rmr
    }
    ///0x50 - I3C event register
    #[inline(always)]
    pub const fn evr(&self) -> &EVR {
        &self.evr
    }
    ///0x54 - I3C interrupt enable register
    #[inline(always)]
    pub const fn ier(&self) -> &IER {
        &self.ier
    }
    ///0x58 - I3C clear event register
    #[inline(always)]
    pub const fn cevr(&self) -> &CEVR {
        &self.cevr
    }
    ///0x60 - I3C own device characteristics register
    #[inline(always)]
    pub const fn devr0(&self) -> &DEVR0 {
        &self.devr0
    }
    ///0x64 - I3C device 1 characteristics register
    #[inline(always)]
    pub const fn devr1(&self) -> &DEVR1 {
        &self.devr1
    }
    ///0x68 - I3C device 2 characteristics register
    #[inline(always)]
    pub const fn devr2(&self) -> &DEVR2 {
        &self.devr2
    }
    ///0x6c - I3C device 3 characteristics register
    #[inline(always)]
    pub const fn devr3(&self) -> &DEVR3 {
        &self.devr3
    }
    ///0x70 - I3C device 4 characteristics register
    #[inline(always)]
    pub const fn devr4(&self) -> &DEVR4 {
        &self.devr4
    }
    ///0x90 - I3C maximum read length register
    #[inline(always)]
    pub const fn maxrlr(&self) -> &MAXRLR {
        &self.maxrlr
    }
    ///0x94 - I3C maximum write length register
    #[inline(always)]
    pub const fn maxwlr(&self) -> &MAXWLR {
        &self.maxwlr
    }
    ///0xa0 - I3C timing register 0
    #[inline(always)]
    pub const fn timingr0(&self) -> &TIMINGR0 {
        &self.timingr0
    }
    ///0xa4 - I3C timing register 1
    #[inline(always)]
    pub const fn timingr1(&self) -> &TIMINGR1 {
        &self.timingr1
    }
    ///0xa8 - I3C timing register 2
    #[inline(always)]
    pub const fn timingr2(&self) -> &TIMINGR2 {
        &self.timingr2
    }
    ///0xc0 - I3C bus characteristics register
    #[inline(always)]
    pub const fn bcr(&self) -> &BCR {
        &self.bcr
    }
    ///0xc4 - I3C device characteristics register
    #[inline(always)]
    pub const fn dcr(&self) -> &DCR {
        &self.dcr
    }
    ///0xc8 - I3C get capability register
    #[inline(always)]
    pub const fn getcapr(&self) -> &GETCAPR {
        &self.getcapr
    }
    ///0xcc - I3C controller-role capability register
    #[inline(always)]
    pub const fn crcapr(&self) -> &CRCAPR {
        &self.crcapr
    }
    ///0xd0 - I3C get capability register
    #[inline(always)]
    pub const fn getmxdsr(&self) -> &GETMXDSR {
        &self.getmxdsr
    }
    ///0xd4 - I3C extended provisioned ID register
    #[inline(always)]
    pub const fn epidr(&self) -> &EPIDR {
        &self.epidr
    }
}
/**CR (w) register accessor: I3C message control register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#I3C1:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///I3C message control register
pub mod cr;
/**CR_ALTERNATE (w) register accessor: I3C message control register alternate

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_alternate::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#I3C1:CR_ALTERNATE)

For information about available fields see [`mod@cr_alternate`] module*/
pub type CR_ALTERNATE = crate::Reg<cr_alternate::CR_ALTERNATErs>;
///I3C message control register alternate
pub mod cr_alternate;
/**CFGR (rw) register accessor: I3C configuration register

You can [`read`](crate::Reg::read) this register and get [`cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#I3C1:CFGR)

For information about available fields see [`mod@cfgr`] module*/
pub type CFGR = crate::Reg<cfgr::CFGRrs>;
///I3C configuration register
pub mod cfgr;
/**RDR (r) register accessor: I3C receive data byte register

You can [`read`](crate::Reg::read) this register and get [`rdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#I3C1:RDR)

For information about available fields see [`mod@rdr`] module*/
pub type RDR = crate::Reg<rdr::RDRrs>;
///I3C receive data byte register
pub mod rdr;
/**RDWR (r) register accessor: I3C receive data word register

You can [`read`](crate::Reg::read) this register and get [`rdwr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#I3C1:RDWR)

For information about available fields see [`mod@rdwr`] module*/
pub type RDWR = crate::Reg<rdwr::RDWRrs>;
///I3C receive data word register
pub mod rdwr;
/**TDR (w) register accessor: I3C transmit data byte register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#I3C1:TDR)

For information about available fields see [`mod@tdr`] module*/
pub type TDR = crate::Reg<tdr::TDRrs>;
///I3C transmit data byte register
pub mod tdr;
/**TDWR (w) register accessor: I3C transmit data word register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdwr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#I3C1:TDWR)

For information about available fields see [`mod@tdwr`] module*/
pub type TDWR = crate::Reg<tdwr::TDWRrs>;
///I3C transmit data word register
pub mod tdwr;
/**IBIDR (rw) register accessor: I3C IBI payload data register

You can [`read`](crate::Reg::read) this register and get [`ibidr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibidr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#I3C1:IBIDR)

For information about available fields see [`mod@ibidr`] module*/
pub type IBIDR = crate::Reg<ibidr::IBIDRrs>;
///I3C IBI payload data register
pub mod ibidr;
/**TGTTDR (rw) register accessor: I3C target transmit configuration register

You can [`read`](crate::Reg::read) this register and get [`tgttdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tgttdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#I3C1:TGTTDR)

For information about available fields see [`mod@tgttdr`] module*/
pub type TGTTDR = crate::Reg<tgttdr::TGTTDRrs>;
///I3C target transmit configuration register
pub mod tgttdr;
/**SR (r) register accessor: I3C status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#I3C1:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///I3C status register
pub mod sr;
/**SER (r) register accessor: I3C status error register

You can [`read`](crate::Reg::read) this register and get [`ser::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#I3C1:SER)

For information about available fields see [`mod@ser`] module*/
pub type SER = crate::Reg<ser::SERrs>;
///I3C status error register
pub mod ser;
/**RMR (r) register accessor: I3C received message register

You can [`read`](crate::Reg::read) this register and get [`rmr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#I3C1:RMR)

For information about available fields see [`mod@rmr`] module*/
pub type RMR = crate::Reg<rmr::RMRrs>;
///I3C received message register
pub mod rmr;
/**EVR (r) register accessor: I3C event register

You can [`read`](crate::Reg::read) this register and get [`evr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#I3C1:EVR)

For information about available fields see [`mod@evr`] module*/
pub type EVR = crate::Reg<evr::EVRrs>;
///I3C event register
pub mod evr;
/**IER (r) register accessor: I3C interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`ier::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#I3C1:IER)

For information about available fields see [`mod@ier`] module*/
pub type IER = crate::Reg<ier::IERrs>;
///I3C interrupt enable register
pub mod ier;
/**CEVR (w) register accessor: I3C clear event register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cevr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#I3C1:CEVR)

For information about available fields see [`mod@cevr`] module*/
pub type CEVR = crate::Reg<cevr::CEVRrs>;
///I3C clear event register
pub mod cevr;
/**DEVR0 (rw) register accessor: I3C own device characteristics register

You can [`read`](crate::Reg::read) this register and get [`devr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#I3C1:DEVR0)

For information about available fields see [`mod@devr0`] module*/
pub type DEVR0 = crate::Reg<devr0::DEVR0rs>;
///I3C own device characteristics register
pub mod devr0;
/**DEVR1 (rw) register accessor: I3C device 1 characteristics register

You can [`read`](crate::Reg::read) this register and get [`devr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#I3C1:DEVR1)

For information about available fields see [`mod@devr1`] module*/
pub type DEVR1 = crate::Reg<devr1::DEVR1rs>;
///I3C device 1 characteristics register
pub mod devr1;
/**DEVR2 (rw) register accessor: I3C device 2 characteristics register

You can [`read`](crate::Reg::read) this register and get [`devr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#I3C1:DEVR2)

For information about available fields see [`mod@devr2`] module*/
pub type DEVR2 = crate::Reg<devr2::DEVR2rs>;
///I3C device 2 characteristics register
pub mod devr2;
/**DEVR3 (rw) register accessor: I3C device 3 characteristics register

You can [`read`](crate::Reg::read) this register and get [`devr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#I3C1:DEVR3)

For information about available fields see [`mod@devr3`] module*/
pub type DEVR3 = crate::Reg<devr3::DEVR3rs>;
///I3C device 3 characteristics register
pub mod devr3;
/**DEVR4 (rw) register accessor: I3C device 4 characteristics register

You can [`read`](crate::Reg::read) this register and get [`devr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#I3C1:DEVR4)

For information about available fields see [`mod@devr4`] module*/
pub type DEVR4 = crate::Reg<devr4::DEVR4rs>;
///I3C device 4 characteristics register
pub mod devr4;
/**MAXRLR (rw) register accessor: I3C maximum read length register

You can [`read`](crate::Reg::read) this register and get [`maxrlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maxrlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#I3C1:MAXRLR)

For information about available fields see [`mod@maxrlr`] module*/
pub type MAXRLR = crate::Reg<maxrlr::MAXRLRrs>;
///I3C maximum read length register
pub mod maxrlr;
/**MAXWLR (rw) register accessor: I3C maximum write length register

You can [`read`](crate::Reg::read) this register and get [`maxwlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maxwlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#I3C1:MAXWLR)

For information about available fields see [`mod@maxwlr`] module*/
pub type MAXWLR = crate::Reg<maxwlr::MAXWLRrs>;
///I3C maximum write length register
pub mod maxwlr;
/**TIMINGR0 (rw) register accessor: I3C timing register 0

You can [`read`](crate::Reg::read) this register and get [`timingr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timingr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#I3C1:TIMINGR0)

For information about available fields see [`mod@timingr0`] module*/
pub type TIMINGR0 = crate::Reg<timingr0::TIMINGR0rs>;
///I3C timing register 0
pub mod timingr0;
/**TIMINGR1 (rw) register accessor: I3C timing register 1

You can [`read`](crate::Reg::read) this register and get [`timingr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timingr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#I3C1:TIMINGR1)

For information about available fields see [`mod@timingr1`] module*/
pub type TIMINGR1 = crate::Reg<timingr1::TIMINGR1rs>;
///I3C timing register 1
pub mod timingr1;
/**TIMINGR2 (rw) register accessor: I3C timing register 2

You can [`read`](crate::Reg::read) this register and get [`timingr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timingr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#I3C1:TIMINGR2)

For information about available fields see [`mod@timingr2`] module*/
pub type TIMINGR2 = crate::Reg<timingr2::TIMINGR2rs>;
///I3C timing register 2
pub mod timingr2;
/**BCR (rw) register accessor: I3C bus characteristics register

You can [`read`](crate::Reg::read) this register and get [`bcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#I3C1:BCR)

For information about available fields see [`mod@bcr`] module*/
pub type BCR = crate::Reg<bcr::BCRrs>;
///I3C bus characteristics register
pub mod bcr;
/**DCR (rw) register accessor: I3C device characteristics register

You can [`read`](crate::Reg::read) this register and get [`dcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#I3C1:DCR)

For information about available fields see [`mod@dcr`] module*/
pub type DCR = crate::Reg<dcr::DCRrs>;
///I3C device characteristics register
pub mod dcr;
/**GETCAPR (rw) register accessor: I3C get capability register

You can [`read`](crate::Reg::read) this register and get [`getcapr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`getcapr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#I3C1:GETCAPR)

For information about available fields see [`mod@getcapr`] module*/
pub type GETCAPR = crate::Reg<getcapr::GETCAPRrs>;
///I3C get capability register
pub mod getcapr;
/**CRCAPR (rw) register accessor: I3C controller-role capability register

You can [`read`](crate::Reg::read) this register and get [`crcapr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcapr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#I3C1:CRCAPR)

For information about available fields see [`mod@crcapr`] module*/
pub type CRCAPR = crate::Reg<crcapr::CRCAPRrs>;
///I3C controller-role capability register
pub mod crcapr;
/**GETMXDSR (rw) register accessor: I3C get capability register

You can [`read`](crate::Reg::read) this register and get [`getmxdsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`getmxdsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#I3C1:GETMXDSR)

For information about available fields see [`mod@getmxdsr`] module*/
pub type GETMXDSR = crate::Reg<getmxdsr::GETMXDSRrs>;
///I3C get capability register
pub mod getmxdsr;
/**EPIDR (rw) register accessor: I3C extended provisioned ID register

You can [`read`](crate::Reg::read) this register and get [`epidr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`epidr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#I3C1:EPIDR)

For information about available fields see [`mod@epidr`] module*/
pub type EPIDR = crate::Reg<epidr::EPIDRrs>;
///I3C extended provisioned ID register
pub mod epidr;
