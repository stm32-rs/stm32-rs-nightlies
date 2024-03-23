#[repr(C)]
#[doc = "Register block"]
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
    #[doc = "0x00 - I3C message control register alternate"]
    #[inline(always)]
    pub const fn cr_alternate(&self) -> &CR_ALTERNATE {
        unsafe { &*(self as *const Self).cast::<u8>().add(0).cast() }
    }
    #[doc = "0x00 - I3C message control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        unsafe { &*(self as *const Self).cast::<u8>().add(0).cast() }
    }
    #[doc = "0x04 - I3C configuration register"]
    #[inline(always)]
    pub const fn cfgr(&self) -> &CFGR {
        &self.cfgr
    }
    #[doc = "0x10 - I3C receive data byte register"]
    #[inline(always)]
    pub const fn rdr(&self) -> &RDR {
        &self.rdr
    }
    #[doc = "0x14 - I3C receive data word register"]
    #[inline(always)]
    pub const fn rdwr(&self) -> &RDWR {
        &self.rdwr
    }
    #[doc = "0x18 - I3C transmit data byte register"]
    #[inline(always)]
    pub const fn tdr(&self) -> &TDR {
        &self.tdr
    }
    #[doc = "0x1c - I3C transmit data word register"]
    #[inline(always)]
    pub const fn tdwr(&self) -> &TDWR {
        &self.tdwr
    }
    #[doc = "0x20 - I3C IBI payload data register"]
    #[inline(always)]
    pub const fn ibidr(&self) -> &IBIDR {
        &self.ibidr
    }
    #[doc = "0x24 - I3C target transmit configuration register"]
    #[inline(always)]
    pub const fn tgttdr(&self) -> &TGTTDR {
        &self.tgttdr
    }
    #[doc = "0x30 - I3C status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    #[doc = "0x34 - I3C status error register"]
    #[inline(always)]
    pub const fn ser(&self) -> &SER {
        &self.ser
    }
    #[doc = "0x40 - I3C received message register"]
    #[inline(always)]
    pub const fn rmr(&self) -> &RMR {
        &self.rmr
    }
    #[doc = "0x50 - I3C event register"]
    #[inline(always)]
    pub const fn evr(&self) -> &EVR {
        &self.evr
    }
    #[doc = "0x54 - I3C interrupt enable register"]
    #[inline(always)]
    pub const fn ier(&self) -> &IER {
        &self.ier
    }
    #[doc = "0x58 - I3C clear event register"]
    #[inline(always)]
    pub const fn cevr(&self) -> &CEVR {
        &self.cevr
    }
    #[doc = "0x60 - I3C own device characteristics register"]
    #[inline(always)]
    pub const fn devr0(&self) -> &DEVR0 {
        &self.devr0
    }
    #[doc = "0x64 - I3C device 1 characteristics register"]
    #[inline(always)]
    pub const fn devr1(&self) -> &DEVR1 {
        &self.devr1
    }
    #[doc = "0x68 - I3C device 2 characteristics register"]
    #[inline(always)]
    pub const fn devr2(&self) -> &DEVR2 {
        &self.devr2
    }
    #[doc = "0x6c - I3C device 3 characteristics register"]
    #[inline(always)]
    pub const fn devr3(&self) -> &DEVR3 {
        &self.devr3
    }
    #[doc = "0x70 - I3C device 4 characteristics register"]
    #[inline(always)]
    pub const fn devr4(&self) -> &DEVR4 {
        &self.devr4
    }
    #[doc = "0x90 - I3C maximum read length register"]
    #[inline(always)]
    pub const fn maxrlr(&self) -> &MAXRLR {
        &self.maxrlr
    }
    #[doc = "0x94 - I3C maximum write length register"]
    #[inline(always)]
    pub const fn maxwlr(&self) -> &MAXWLR {
        &self.maxwlr
    }
    #[doc = "0xa0 - I3C timing register 0"]
    #[inline(always)]
    pub const fn timingr0(&self) -> &TIMINGR0 {
        &self.timingr0
    }
    #[doc = "0xa4 - I3C timing register 1"]
    #[inline(always)]
    pub const fn timingr1(&self) -> &TIMINGR1 {
        &self.timingr1
    }
    #[doc = "0xa8 - I3C timing register 2"]
    #[inline(always)]
    pub const fn timingr2(&self) -> &TIMINGR2 {
        &self.timingr2
    }
    #[doc = "0xc0 - I3C bus characteristics register"]
    #[inline(always)]
    pub const fn bcr(&self) -> &BCR {
        &self.bcr
    }
    #[doc = "0xc4 - I3C device characteristics register"]
    #[inline(always)]
    pub const fn dcr(&self) -> &DCR {
        &self.dcr
    }
    #[doc = "0xc8 - I3C get capability register"]
    #[inline(always)]
    pub const fn getcapr(&self) -> &GETCAPR {
        &self.getcapr
    }
    #[doc = "0xcc - I3C controller-role capability register"]
    #[inline(always)]
    pub const fn crcapr(&self) -> &CRCAPR {
        &self.crcapr
    }
    #[doc = "0xd0 - I3C get capability register"]
    #[inline(always)]
    pub const fn getmxdsr(&self) -> &GETMXDSR {
        &self.getmxdsr
    }
    #[doc = "0xd4 - I3C extended provisioned ID register"]
    #[inline(always)]
    pub const fn epidr(&self) -> &EPIDR {
        &self.epidr
    }
}
#[doc = "CR (w) register accessor: I3C message control register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CRrs>;
#[doc = "I3C message control register"]
pub mod cr;
#[doc = "CR_ALTERNATE (w) register accessor: I3C message control register alternate\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr_alternate::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_alternate`]
module"]
pub type CR_ALTERNATE = crate::Reg<cr_alternate::CR_ALTERNATErs>;
#[doc = "I3C message control register alternate"]
pub mod cr_alternate;
#[doc = "CFGR (rw) register accessor: I3C configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr`]
module"]
pub type CFGR = crate::Reg<cfgr::CFGRrs>;
#[doc = "I3C configuration register"]
pub mod cfgr;
#[doc = "RDR (r) register accessor: I3C receive data byte register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdr`]
module"]
pub type RDR = crate::Reg<rdr::RDRrs>;
#[doc = "I3C receive data byte register"]
pub mod rdr;
#[doc = "RDWR (r) register accessor: I3C receive data word register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdwr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdwr`]
module"]
pub type RDWR = crate::Reg<rdwr::RDWRrs>;
#[doc = "I3C receive data word register"]
pub mod rdwr;
#[doc = "TDR (w) register accessor: I3C transmit data byte register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tdr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tdr`]
module"]
pub type TDR = crate::Reg<tdr::TDRrs>;
#[doc = "I3C transmit data byte register"]
pub mod tdr;
#[doc = "TDWR (w) register accessor: I3C transmit data word register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tdwr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tdwr`]
module"]
pub type TDWR = crate::Reg<tdwr::TDWRrs>;
#[doc = "I3C transmit data word register"]
pub mod tdwr;
#[doc = "IBIDR (rw) register accessor: I3C IBI payload data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ibidr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ibidr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibidr`]
module"]
pub type IBIDR = crate::Reg<ibidr::IBIDRrs>;
#[doc = "I3C IBI payload data register"]
pub mod ibidr;
#[doc = "TGTTDR (rw) register accessor: I3C target transmit configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tgttdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tgttdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tgttdr`]
module"]
pub type TGTTDR = crate::Reg<tgttdr::TGTTDRrs>;
#[doc = "I3C target transmit configuration register"]
pub mod tgttdr;
#[doc = "SR (r) register accessor: I3C status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
pub type SR = crate::Reg<sr::SRrs>;
#[doc = "I3C status register"]
pub mod sr;
#[doc = "SER (r) register accessor: I3C status error register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ser::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ser`]
module"]
pub type SER = crate::Reg<ser::SERrs>;
#[doc = "I3C status error register"]
pub mod ser;
#[doc = "RMR (r) register accessor: I3C received message register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rmr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rmr`]
module"]
pub type RMR = crate::Reg<rmr::RMRrs>;
#[doc = "I3C received message register"]
pub mod rmr;
#[doc = "EVR (r) register accessor: I3C event register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evr`]
module"]
pub type EVR = crate::Reg<evr::EVRrs>;
#[doc = "I3C event register"]
pub mod evr;
#[doc = "IER (r) register accessor: I3C interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`]
module"]
pub type IER = crate::Reg<ier::IERrs>;
#[doc = "I3C interrupt enable register"]
pub mod ier;
#[doc = "CEVR (w) register accessor: I3C clear event register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cevr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cevr`]
module"]
pub type CEVR = crate::Reg<cevr::CEVRrs>;
#[doc = "I3C clear event register"]
pub mod cevr;
#[doc = "DEVR0 (rw) register accessor: I3C own device characteristics register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devr0`]
module"]
pub type DEVR0 = crate::Reg<devr0::DEVR0rs>;
#[doc = "I3C own device characteristics register"]
pub mod devr0;
#[doc = "DEVR1 (rw) register accessor: I3C device 1 characteristics register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devr1`]
module"]
pub type DEVR1 = crate::Reg<devr1::DEVR1rs>;
#[doc = "I3C device 1 characteristics register"]
pub mod devr1;
#[doc = "DEVR2 (rw) register accessor: I3C device 2 characteristics register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devr2`]
module"]
pub type DEVR2 = crate::Reg<devr2::DEVR2rs>;
#[doc = "I3C device 2 characteristics register"]
pub mod devr2;
#[doc = "DEVR3 (rw) register accessor: I3C device 3 characteristics register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devr3`]
module"]
pub type DEVR3 = crate::Reg<devr3::DEVR3rs>;
#[doc = "I3C device 3 characteristics register"]
pub mod devr3;
#[doc = "DEVR4 (rw) register accessor: I3C device 4 characteristics register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devr4`]
module"]
pub type DEVR4 = crate::Reg<devr4::DEVR4rs>;
#[doc = "I3C device 4 characteristics register"]
pub mod devr4;
#[doc = "MAXRLR (rw) register accessor: I3C maximum read length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maxrlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maxrlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maxrlr`]
module"]
pub type MAXRLR = crate::Reg<maxrlr::MAXRLRrs>;
#[doc = "I3C maximum read length register"]
pub mod maxrlr;
#[doc = "MAXWLR (rw) register accessor: I3C maximum write length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maxwlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maxwlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maxwlr`]
module"]
pub type MAXWLR = crate::Reg<maxwlr::MAXWLRrs>;
#[doc = "I3C maximum write length register"]
pub mod maxwlr;
#[doc = "TIMINGR0 (rw) register accessor: I3C timing register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timingr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timingr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timingr0`]
module"]
pub type TIMINGR0 = crate::Reg<timingr0::TIMINGR0rs>;
#[doc = "I3C timing register 0"]
pub mod timingr0;
#[doc = "TIMINGR1 (rw) register accessor: I3C timing register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timingr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timingr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timingr1`]
module"]
pub type TIMINGR1 = crate::Reg<timingr1::TIMINGR1rs>;
#[doc = "I3C timing register 1"]
pub mod timingr1;
#[doc = "TIMINGR2 (rw) register accessor: I3C timing register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timingr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timingr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timingr2`]
module"]
pub type TIMINGR2 = crate::Reg<timingr2::TIMINGR2rs>;
#[doc = "I3C timing register 2"]
pub mod timingr2;
#[doc = "BCR (rw) register accessor: I3C bus characteristics register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcr`]
module"]
pub type BCR = crate::Reg<bcr::BCRrs>;
#[doc = "I3C bus characteristics register"]
pub mod bcr;
#[doc = "DCR (rw) register accessor: I3C device characteristics register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcr`]
module"]
pub type DCR = crate::Reg<dcr::DCRrs>;
#[doc = "I3C device characteristics register"]
pub mod dcr;
#[doc = "GETCAPR (rw) register accessor: I3C get capability register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`getcapr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`getcapr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@getcapr`]
module"]
pub type GETCAPR = crate::Reg<getcapr::GETCAPRrs>;
#[doc = "I3C get capability register"]
pub mod getcapr;
#[doc = "CRCAPR (rw) register accessor: I3C controller-role capability register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crcapr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crcapr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcapr`]
module"]
pub type CRCAPR = crate::Reg<crcapr::CRCAPRrs>;
#[doc = "I3C controller-role capability register"]
pub mod crcapr;
#[doc = "GETMXDSR (rw) register accessor: I3C get capability register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`getmxdsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`getmxdsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@getmxdsr`]
module"]
pub type GETMXDSR = crate::Reg<getmxdsr::GETMXDSRrs>;
#[doc = "I3C get capability register"]
pub mod getmxdsr;
#[doc = "EPIDR (rw) register accessor: I3C extended provisioned ID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epidr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epidr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epidr`]
module"]
pub type EPIDR = crate::Reg<epidr::EPIDRrs>;
#[doc = "I3C extended provisioned ID register"]
pub mod epidr;
