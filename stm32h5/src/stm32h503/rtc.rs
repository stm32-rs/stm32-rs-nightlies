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
    _reserved8: [u8; 0x04],
    wpr: WPR,
    calr: CALR,
    shiftr: SHIFTR,
    tstr: TSTR,
    tsdr: TSDR,
    tsssr: TSSSR,
    _reserved14: [u8; 0x04],
    alrmar: ALRMAR,
    alrmassr: ALRMASSR,
    alrmbr: ALRMBR,
    alrmbssr: ALRMBSSR,
    sr: SR,
    misr: MISR,
    _reserved20: [u8; 0x04],
    scr: SCR,
    _reserved21: [u8; 0x10],
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
    #[doc = "0x08 - RTC subsecond register"]
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
    #[doc = "0x38 - RTC timestamp subsecond register"]
    #[inline(always)]
    pub const fn tsssr(&self) -> &TSSSR {
        &self.tsssr
    }
    #[doc = "0x40 - RTC alarm A register"]
    #[inline(always)]
    pub const fn alrmar(&self) -> &ALRMAR {
        &self.alrmar
    }
    #[doc = "0x44 - RTC alarm A subsecond register"]
    #[inline(always)]
    pub const fn alrmassr(&self) -> &ALRMASSR {
        &self.alrmassr
    }
    #[doc = "0x48 - RTC alarm B register"]
    #[inline(always)]
    pub const fn alrmbr(&self) -> &ALRMBR {
        &self.alrmbr
    }
    #[doc = "0x4c - RTC alarm B subsecond register"]
    #[inline(always)]
    pub const fn alrmbssr(&self) -> &ALRMBSSR {
        &self.alrmbssr
    }
    #[doc = "0x50 - RTC status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    #[doc = "0x54 - RTC masked interrupt status register"]
    #[inline(always)]
    pub const fn misr(&self) -> &MISR {
        &self.misr
    }
    #[doc = "0x5c - RTC status clear register"]
    #[inline(always)]
    pub const fn scr(&self) -> &SCR {
        &self.scr
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
#[doc = "SSR (r) register accessor: RTC subsecond register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssr`]
module"]
pub type SSR = crate::Reg<ssr::SSRrs>;
#[doc = "RTC subsecond register"]
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
#[doc = "TSTR (r) register accessor: RTC timestamp time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tstr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tstr`]
module"]
pub type TSTR = crate::Reg<tstr::TSTRrs>;
#[doc = "RTC timestamp time register"]
pub mod tstr;
#[doc = "TSDR (r) register accessor: RTC timestamp date register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsdr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsdr`]
module"]
pub type TSDR = crate::Reg<tsdr::TSDRrs>;
#[doc = "RTC timestamp date register"]
pub mod tsdr;
#[doc = "TSSSR (r) register accessor: RTC timestamp subsecond register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsssr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsssr`]
module"]
pub type TSSSR = crate::Reg<tsssr::TSSSRrs>;
#[doc = "RTC timestamp subsecond register"]
pub mod tsssr;
#[doc = "ALRMAR (rw) register accessor: RTC alarm A register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alrmar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alrmar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alrmar`]
module"]
pub type ALRMAR = crate::Reg<alrmar::ALRMARrs>;
#[doc = "RTC alarm A register"]
pub mod alrmar;
#[doc = "ALRMASSR (rw) register accessor: RTC alarm A subsecond register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alrmassr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alrmassr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alrmassr`]
module"]
pub type ALRMASSR = crate::Reg<alrmassr::ALRMASSRrs>;
#[doc = "RTC alarm A subsecond register"]
pub mod alrmassr;
#[doc = "ALRMBR (rw) register accessor: RTC alarm B register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alrmbr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alrmbr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alrmbr`]
module"]
pub type ALRMBR = crate::Reg<alrmbr::ALRMBRrs>;
#[doc = "RTC alarm B register"]
pub mod alrmbr;
#[doc = "ALRMBSSR (rw) register accessor: RTC alarm B subsecond register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alrmbssr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alrmbssr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alrmbssr`]
module"]
pub type ALRMBSSR = crate::Reg<alrmbssr::ALRMBSSRrs>;
#[doc = "RTC alarm B subsecond register"]
pub mod alrmbssr;
#[doc = "SR (r) register accessor: RTC status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
pub type SR = crate::Reg<sr::SRrs>;
#[doc = "RTC status register"]
pub mod sr;
#[doc = "MISR (r) register accessor: RTC masked interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`misr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@misr`]
module"]
pub type MISR = crate::Reg<misr::MISRrs>;
#[doc = "RTC masked interrupt status register"]
pub mod misr;
#[doc = "SCR (w) register accessor: RTC status clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scr`]
module"]
pub type SCR = crate::Reg<scr::SCRrs>;
#[doc = "RTC status clear register"]
pub mod scr;
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
