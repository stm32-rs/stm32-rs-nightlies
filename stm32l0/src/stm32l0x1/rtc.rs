#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tr: TR,
    dr: DR,
    cr: CR,
    isr: ISR,
    prer: PRER,
    wutr: WUTR,
    _reserved6: [u8; 0x04],
    alrmr: [ALRMR; 2],
    wpr: WPR,
    ssr: SSR,
    shiftr: SHIFTR,
    tstr: TSTR,
    tsdr: TSDR,
    tsssr: TSSSR,
    calr: CALR,
    tampcr: TAMPCR,
    alrmssr: [ALRMSSR; 2],
    or: OR,
    bkpr: [BKPR; 5],
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
    #[doc = "0x08 - RTC control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x0c - RTC initialization and status register"]
    #[inline(always)]
    pub const fn isr(&self) -> &ISR {
        &self.isr
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
    #[doc = "0x1c..0x24 - Alarm %s register"]
    #[inline(always)]
    pub const fn alrmr(&self, n: usize) -> &ALRMR {
        &self.alrmr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1c..0x24 - Alarm %s register"]
    #[inline(always)]
    pub fn alrmr_iter(&self) -> impl Iterator<Item = &ALRMR> {
        self.alrmr.iter()
    }
    #[doc = "0x1c - Alarm A register"]
    #[inline(always)]
    pub const fn alrmar(&self) -> &ALRMR {
        self.alrmr(0)
    }
    #[doc = "0x20 - Alarm B register"]
    #[inline(always)]
    pub const fn alrmbr(&self) -> &ALRMR {
        self.alrmr(1)
    }
    #[doc = "0x24 - write protection register"]
    #[inline(always)]
    pub const fn wpr(&self) -> &WPR {
        &self.wpr
    }
    #[doc = "0x28 - RTC sub second register"]
    #[inline(always)]
    pub const fn ssr(&self) -> &SSR {
        &self.ssr
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
    #[doc = "0x38 - RTC time-stamp sub second register"]
    #[inline(always)]
    pub const fn tsssr(&self) -> &TSSSR {
        &self.tsssr
    }
    #[doc = "0x3c - RTC calibration register"]
    #[inline(always)]
    pub const fn calr(&self) -> &CALR {
        &self.calr
    }
    #[doc = "0x40 - RTC tamper configuration register"]
    #[inline(always)]
    pub const fn tampcr(&self) -> &TAMPCR {
        &self.tampcr
    }
    #[doc = "0x44..0x4c - Alarm %s sub-second register"]
    #[inline(always)]
    pub const fn alrmssr(&self, n: usize) -> &ALRMSSR {
        &self.alrmssr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x44..0x4c - Alarm %s sub-second register"]
    #[inline(always)]
    pub fn alrmssr_iter(&self) -> impl Iterator<Item = &ALRMSSR> {
        self.alrmssr.iter()
    }
    #[doc = "0x44 - Alarm A sub-second register"]
    #[inline(always)]
    pub const fn alrmassr(&self) -> &ALRMSSR {
        self.alrmssr(0)
    }
    #[doc = "0x48 - Alarm B sub-second register"]
    #[inline(always)]
    pub const fn alrmbssr(&self) -> &ALRMSSR {
        self.alrmssr(1)
    }
    #[doc = "0x4c - option register"]
    #[inline(always)]
    pub const fn or(&self) -> &OR {
        &self.or
    }
    #[doc = "0x50..0x64 - RTC backup registers"]
    #[inline(always)]
    pub const fn bkpr(&self, n: usize) -> &BKPR {
        &self.bkpr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x50..0x64 - RTC backup registers"]
    #[inline(always)]
    pub fn bkpr_iter(&self) -> impl Iterator<Item = &BKPR> {
        self.bkpr.iter()
    }
    #[doc = "0x50 - RTC backup registers"]
    #[inline(always)]
    pub const fn bkp0r(&self) -> &BKPR {
        self.bkpr(0)
    }
    #[doc = "0x54 - RTC backup registers"]
    #[inline(always)]
    pub const fn bkp1r(&self) -> &BKPR {
        self.bkpr(1)
    }
    #[doc = "0x58 - RTC backup registers"]
    #[inline(always)]
    pub const fn bkp2r(&self) -> &BKPR {
        self.bkpr(2)
    }
    #[doc = "0x5c - RTC backup registers"]
    #[inline(always)]
    pub const fn bkp3r(&self) -> &BKPR {
        self.bkpr(3)
    }
    #[doc = "0x60 - RTC backup registers"]
    #[inline(always)]
    pub const fn bkp4r(&self) -> &BKPR {
        self.bkpr(4)
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
#[doc = "CR (rw) register accessor: RTC control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CRrs>;
#[doc = "RTC control register"]
pub mod cr;
#[doc = "ISR (rw) register accessor: RTC initialization and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`]
module"]
pub type ISR = crate::Reg<isr::ISRrs>;
#[doc = "RTC initialization and status register"]
pub mod isr;
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
#[doc = "ALRMR (rw) register accessor: Alarm %s register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alrmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alrmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alrmr`]
module"]
pub type ALRMR = crate::Reg<alrmr::ALRMRrs>;
#[doc = "Alarm %s register"]
pub mod alrmr;
#[doc = "WPR (w) register accessor: write protection register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wpr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpr`]
module"]
pub type WPR = crate::Reg<wpr::WPRrs>;
#[doc = "write protection register"]
pub mod wpr;
#[doc = "SSR (r) register accessor: RTC sub second register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssr`]
module"]
pub type SSR = crate::Reg<ssr::SSRrs>;
#[doc = "RTC sub second register"]
pub mod ssr;
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
#[doc = "CALR (rw) register accessor: RTC calibration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`calr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`calr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@calr`]
module"]
pub type CALR = crate::Reg<calr::CALRrs>;
#[doc = "RTC calibration register"]
pub mod calr;
#[doc = "TAMPCR (rw) register accessor: RTC tamper configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tampcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tampcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tampcr`]
module"]
pub type TAMPCR = crate::Reg<tampcr::TAMPCRrs>;
#[doc = "RTC tamper configuration register"]
pub mod tampcr;
#[doc = "ALRMSSR (rw) register accessor: Alarm %s sub-second register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alrmssr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alrmssr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alrmssr`]
module"]
pub type ALRMSSR = crate::Reg<alrmssr::ALRMSSRrs>;
#[doc = "Alarm %s sub-second register"]
pub mod alrmssr;
#[doc = "OR (rw) register accessor: option register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`or::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`or::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@or`]
module"]
pub type OR = crate::Reg<or::ORrs>;
#[doc = "option register"]
pub mod or;
#[doc = "BKPR (rw) register accessor: RTC backup registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bkpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bkpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkpr`]
module"]
pub type BKPR = crate::Reg<bkpr::BKPRrs>;
#[doc = "RTC backup registers"]
pub mod bkpr;
