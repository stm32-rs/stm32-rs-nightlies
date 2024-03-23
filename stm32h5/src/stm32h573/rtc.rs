#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tr: TR,
    dr: DR,
    ssr: SSR,
    icsr: ICSR,
    prer: PRER,
    wutr: WUTR,
    cr: CR,
    privcfgr: PRIVCFGR,
    seccfgr: SECCFGR,
    wpr: WPR,
    calr: CALR,
    shiftr: SHIFTR,
    tstr: TSTR,
    tsdr: TSDR,
    tsssr: TSSSR,
    _reserved15: [u8; 0x04],
    alrmr: (),
    _reserved16: [u8; 0x04],
    alrmssr: (),
    _reserved17: [u8; 0x0c],
    sr: SR,
    misr: MISR,
    smisr: SMISR,
    scr: SCR,
    or: OR,
    _reserved22: [u8; 0x0c],
    alrabinr: ALRABINR,
    alrbbinr: ALRBBINR,
}
impl RegisterBlock {
    #[doc = "0x00 - RTC time register"]
    #[inline(always)]
    pub const fn tr(&self) -> &TR {
        &self.tr
    }
    #[doc = "0x04 - RTC date register"]
    #[inline(always)]
    pub const fn dr(&self) -> &DR {
        &self.dr
    }
    #[doc = "0x08 - RTC sub second register"]
    #[inline(always)]
    pub const fn ssr(&self) -> &SSR {
        &self.ssr
    }
    #[doc = "0x0c - RTC initialization control and status register"]
    #[inline(always)]
    pub const fn icsr(&self) -> &ICSR {
        &self.icsr
    }
    #[doc = "0x10 - RTC prescaler register"]
    #[inline(always)]
    pub const fn prer(&self) -> &PRER {
        &self.prer
    }
    #[doc = "0x14 - RTC wakeup timer register"]
    #[inline(always)]
    pub const fn wutr(&self) -> &WUTR {
        &self.wutr
    }
    #[doc = "0x18 - RTC control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x1c - RTC privilege mode control register"]
    #[inline(always)]
    pub const fn privcfgr(&self) -> &PRIVCFGR {
        &self.privcfgr
    }
    #[doc = "0x20 - RTC secure configuration register"]
    #[inline(always)]
    pub const fn seccfgr(&self) -> &SECCFGR {
        &self.seccfgr
    }
    #[doc = "0x24 - RTC write protection register"]
    #[inline(always)]
    pub const fn wpr(&self) -> &WPR {
        &self.wpr
    }
    #[doc = "0x28 - RTC calibration register"]
    #[inline(always)]
    pub const fn calr(&self) -> &CALR {
        &self.calr
    }
    #[doc = "0x2c - RTC shift control register"]
    #[inline(always)]
    pub const fn shiftr(&self) -> &SHIFTR {
        &self.shiftr
    }
    #[doc = "0x30 - RTC timestamp time register"]
    #[inline(always)]
    pub const fn tstr(&self) -> &TSTR {
        &self.tstr
    }
    #[doc = "0x34 - RTC timestamp date register"]
    #[inline(always)]
    pub const fn tsdr(&self) -> &TSDR {
        &self.tsdr
    }
    #[doc = "0x38 - RTC timestamp sub second register"]
    #[inline(always)]
    pub const fn tsssr(&self) -> &TSSSR {
        &self.tsssr
    }
    #[doc = "0x40..0x48 - Alarm %s register"]
    #[inline(always)]
    pub const fn alrmr(&self, n: usize) -> &ALRMR {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe { &*(self as *const Self).cast::<u8>().add(64).add(8 * n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x40..0x48 - Alarm %s register"]
    #[inline(always)]
    pub fn alrmr_iter(&self) -> impl Iterator<Item = &ALRMR> {
        (0..2)
            .map(move |n| unsafe { &*(self as *const Self).cast::<u8>().add(64).add(8 * n).cast() })
    }
    #[doc = "0x40 - Alarm A register"]
    #[inline(always)]
    pub const fn alrmar(&self) -> &ALRMR {
        self.alrmr(0)
    }
    #[doc = "0x48 - Alarm B register"]
    #[inline(always)]
    pub const fn alrmbr(&self) -> &ALRMR {
        self.alrmr(1)
    }
    #[doc = "0x44..0x4c - Alarm %s sub-second register"]
    #[inline(always)]
    pub const fn alrmssr(&self, n: usize) -> &ALRMSSR {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe { &*(self as *const Self).cast::<u8>().add(68).add(8 * n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x44..0x4c - Alarm %s sub-second register"]
    #[inline(always)]
    pub fn alrmssr_iter(&self) -> impl Iterator<Item = &ALRMSSR> {
        (0..2)
            .map(move |n| unsafe { &*(self as *const Self).cast::<u8>().add(68).add(8 * n).cast() })
    }
    #[doc = "0x44 - Alarm A sub-second register"]
    #[inline(always)]
    pub const fn alrmassr(&self) -> &ALRMSSR {
        self.alrmssr(0)
    }
    #[doc = "0x4c - Alarm B sub-second register"]
    #[inline(always)]
    pub const fn alrmbssr(&self) -> &ALRMSSR {
        self.alrmssr(1)
    }
    #[doc = "0x50 - RTC status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    #[doc = "0x54 - RTC non-secure masked interrupt status register"]
    #[inline(always)]
    pub const fn misr(&self) -> &MISR {
        &self.misr
    }
    #[doc = "0x58 - RTC secure masked interrupt status register"]
    #[inline(always)]
    pub const fn smisr(&self) -> &SMISR {
        &self.smisr
    }
    #[doc = "0x5c - RTC status clear register"]
    #[inline(always)]
    pub const fn scr(&self) -> &SCR {
        &self.scr
    }
    #[doc = "0x60 - RTC option register"]
    #[inline(always)]
    pub const fn or(&self) -> &OR {
        &self.or
    }
    #[doc = "0x70 - RTC alarm A binary mode register"]
    #[inline(always)]
    pub const fn alrabinr(&self) -> &ALRABINR {
        &self.alrabinr
    }
    #[doc = "0x74 - RTC alarm B binary mode register"]
    #[inline(always)]
    pub const fn alrbbinr(&self) -> &ALRBBINR {
        &self.alrbbinr
    }
}
#[doc = "TR (rw) register accessor: RTC time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tr`]
module"]
pub type TR = crate::Reg<tr::TRrs>;
#[doc = "RTC time register"]
pub mod tr;
#[doc = "DR (rw) register accessor: RTC date register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr`]
module"]
pub type DR = crate::Reg<dr::DRrs>;
#[doc = "RTC date register"]
pub mod dr;
#[doc = "SSR (r) register accessor: RTC sub second register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssr`]
module"]
pub type SSR = crate::Reg<ssr::SSRrs>;
#[doc = "RTC sub second register"]
pub mod ssr;
#[doc = "ICSR (rw) register accessor: RTC initialization control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icsr`]
module"]
pub type ICSR = crate::Reg<icsr::ICSRrs>;
#[doc = "RTC initialization control and status register"]
pub mod icsr;
#[doc = "PRER (rw) register accessor: RTC prescaler register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prer`]
module"]
pub type PRER = crate::Reg<prer::PRERrs>;
#[doc = "RTC prescaler register"]
pub mod prer;
#[doc = "WUTR (rw) register accessor: RTC wakeup timer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wutr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wutr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wutr`]
module"]
pub type WUTR = crate::Reg<wutr::WUTRrs>;
#[doc = "RTC wakeup timer register"]
pub mod wutr;
#[doc = "CR (rw) register accessor: RTC control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CRrs>;
#[doc = "RTC control register"]
pub mod cr;
#[doc = "PRIVCFGR (rw) register accessor: RTC privilege mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`privcfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`privcfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@privcfgr`]
module"]
pub type PRIVCFGR = crate::Reg<privcfgr::PRIVCFGRrs>;
#[doc = "RTC privilege mode control register"]
pub mod privcfgr;
#[doc = "SECCFGR (rw) register accessor: RTC secure configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seccfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seccfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seccfgr`]
module"]
pub type SECCFGR = crate::Reg<seccfgr::SECCFGRrs>;
#[doc = "RTC secure configuration register"]
pub mod seccfgr;
#[doc = "WPR (w) register accessor: RTC write protection register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wpr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpr`]
module"]
pub type WPR = crate::Reg<wpr::WPRrs>;
#[doc = "RTC write protection register"]
pub mod wpr;
#[doc = "CALR (rw) register accessor: RTC calibration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`calr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`calr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@calr`]
module"]
pub type CALR = crate::Reg<calr::CALRrs>;
#[doc = "RTC calibration register"]
pub mod calr;
#[doc = "SHIFTR (w) register accessor: RTC shift control register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shiftr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shiftr`]
module"]
pub type SHIFTR = crate::Reg<shiftr::SHIFTRrs>;
#[doc = "RTC shift control register"]
pub mod shiftr;
pub use dr as tsdr;
pub use ssr as tsssr;
pub use tr as tstr;
pub use DR as TSDR;
pub use SSR as TSSSR;
pub use TR as TSTR;
#[doc = "ALRMR (rw) register accessor: Alarm %s register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alrmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alrmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alrmr`]
module"]
pub type ALRMR = crate::Reg<alrmr::ALRMRrs>;
#[doc = "Alarm %s register"]
pub mod alrmr;
#[doc = "ALRMSSR (rw) register accessor: Alarm %s sub-second register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alrmssr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alrmssr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alrmssr`]
module"]
pub type ALRMSSR = crate::Reg<alrmssr::ALRMSSRrs>;
#[doc = "Alarm %s sub-second register"]
pub mod alrmssr;
#[doc = "SR (r) register accessor: RTC status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
pub type SR = crate::Reg<sr::SRrs>;
#[doc = "RTC status register"]
pub mod sr;
#[doc = "MISR (r) register accessor: RTC non-secure masked interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`misr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@misr`]
module"]
pub type MISR = crate::Reg<misr::MISRrs>;
#[doc = "RTC non-secure masked interrupt status register"]
pub mod misr;
#[doc = "SMISR (r) register accessor: RTC secure masked interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smisr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smisr`]
module"]
pub type SMISR = crate::Reg<smisr::SMISRrs>;
#[doc = "RTC secure masked interrupt status register"]
pub mod smisr;
#[doc = "SCR (w) register accessor: RTC status clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scr`]
module"]
pub type SCR = crate::Reg<scr::SCRrs>;
#[doc = "RTC status clear register"]
pub mod scr;
#[doc = "OR (rw) register accessor: RTC option register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`or::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`or::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@or`]
module"]
pub type OR = crate::Reg<or::ORrs>;
#[doc = "RTC option register"]
pub mod or;
#[doc = "ALRABINR (rw) register accessor: RTC alarm A binary mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alrabinr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alrabinr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alrabinr`]
module"]
pub type ALRABINR = crate::Reg<alrabinr::ALRABINRrs>;
#[doc = "RTC alarm A binary mode register"]
pub mod alrabinr;
#[doc = "ALRBBINR (rw) register accessor: RTC alarm B binary mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alrbbinr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alrbbinr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alrbbinr`]
module"]
pub type ALRBBINR = crate::Reg<alrbbinr::ALRBBINRrs>;
#[doc = "RTC alarm B binary mode register"]
pub mod alrbbinr;
