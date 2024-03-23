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
    bkpr: [BKPR; 20],
}
impl RegisterBlock {
    #[doc = "0x00 - time register"]
    #[inline(always)]
    pub const fn tr(&self) -> &TR {
        &self.tr
    }
    #[doc = "0x04 - date register"]
    #[inline(always)]
    pub const fn dr(&self) -> &DR {
        &self.dr
    }
    #[doc = "0x08 - control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x0c - initialization and status register"]
    #[inline(always)]
    pub const fn isr(&self) -> &ISR {
        &self.isr
    }
    #[doc = "0x10 - prescaler register"]
    #[inline(always)]
    pub const fn prer(&self) -> &PRER {
        &self.prer
    }
    #[doc = "0x14 - wakeup timer register"]
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
    #[doc = "0x28 - sub second register"]
    #[inline(always)]
    pub const fn ssr(&self) -> &SSR {
        &self.ssr
    }
    #[doc = "0x2c - shift control register"]
    #[inline(always)]
    pub const fn shiftr(&self) -> &SHIFTR {
        &self.shiftr
    }
    #[doc = "0x30 - time stamp time register"]
    #[inline(always)]
    pub const fn tstr(&self) -> &TSTR {
        &self.tstr
    }
    #[doc = "0x34 - time stamp date register"]
    #[inline(always)]
    pub const fn tsdr(&self) -> &TSDR {
        &self.tsdr
    }
    #[doc = "0x38 - timestamp sub second register"]
    #[inline(always)]
    pub const fn tsssr(&self) -> &TSSSR {
        &self.tsssr
    }
    #[doc = "0x3c - calibration register"]
    #[inline(always)]
    pub const fn calr(&self) -> &CALR {
        &self.calr
    }
    #[doc = "0x40 - tamper configuration register"]
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
    #[doc = "0x50..0xa0 - backup register"]
    #[inline(always)]
    pub const fn bkpr(&self, n: usize) -> &BKPR {
        &self.bkpr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x50..0xa0 - backup register"]
    #[inline(always)]
    pub fn bkpr_iter(&self) -> impl Iterator<Item = &BKPR> {
        self.bkpr.iter()
    }
    #[doc = "0x50 - backup register"]
    #[inline(always)]
    pub const fn bkp0r(&self) -> &BKPR {
        self.bkpr(0)
    }
    #[doc = "0x54 - backup register"]
    #[inline(always)]
    pub const fn bkp1r(&self) -> &BKPR {
        self.bkpr(1)
    }
    #[doc = "0x58 - backup register"]
    #[inline(always)]
    pub const fn bkp2r(&self) -> &BKPR {
        self.bkpr(2)
    }
    #[doc = "0x5c - backup register"]
    #[inline(always)]
    pub const fn bkp3r(&self) -> &BKPR {
        self.bkpr(3)
    }
    #[doc = "0x60 - backup register"]
    #[inline(always)]
    pub const fn bkp4r(&self) -> &BKPR {
        self.bkpr(4)
    }
    #[doc = "0x64 - backup register"]
    #[inline(always)]
    pub const fn bkp5r(&self) -> &BKPR {
        self.bkpr(5)
    }
    #[doc = "0x68 - backup register"]
    #[inline(always)]
    pub const fn bkp6r(&self) -> &BKPR {
        self.bkpr(6)
    }
    #[doc = "0x6c - backup register"]
    #[inline(always)]
    pub const fn bkp7r(&self) -> &BKPR {
        self.bkpr(7)
    }
    #[doc = "0x70 - backup register"]
    #[inline(always)]
    pub const fn bkp8r(&self) -> &BKPR {
        self.bkpr(8)
    }
    #[doc = "0x74 - backup register"]
    #[inline(always)]
    pub const fn bkp9r(&self) -> &BKPR {
        self.bkpr(9)
    }
    #[doc = "0x78 - backup register"]
    #[inline(always)]
    pub const fn bkp10r(&self) -> &BKPR {
        self.bkpr(10)
    }
    #[doc = "0x7c - backup register"]
    #[inline(always)]
    pub const fn bkp11r(&self) -> &BKPR {
        self.bkpr(11)
    }
    #[doc = "0x80 - backup register"]
    #[inline(always)]
    pub const fn bkp12r(&self) -> &BKPR {
        self.bkpr(12)
    }
    #[doc = "0x84 - backup register"]
    #[inline(always)]
    pub const fn bkp13r(&self) -> &BKPR {
        self.bkpr(13)
    }
    #[doc = "0x88 - backup register"]
    #[inline(always)]
    pub const fn bkp14r(&self) -> &BKPR {
        self.bkpr(14)
    }
    #[doc = "0x8c - backup register"]
    #[inline(always)]
    pub const fn bkp15r(&self) -> &BKPR {
        self.bkpr(15)
    }
    #[doc = "0x90 - backup register"]
    #[inline(always)]
    pub const fn bkp16r(&self) -> &BKPR {
        self.bkpr(16)
    }
    #[doc = "0x94 - backup register"]
    #[inline(always)]
    pub const fn bkp17r(&self) -> &BKPR {
        self.bkpr(17)
    }
    #[doc = "0x98 - backup register"]
    #[inline(always)]
    pub const fn bkp18r(&self) -> &BKPR {
        self.bkpr(18)
    }
    #[doc = "0x9c - backup register"]
    #[inline(always)]
    pub const fn bkp19r(&self) -> &BKPR {
        self.bkpr(19)
    }
}
#[doc = "TR (rw) register accessor: time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tr`]
module"]
pub type TR = crate::Reg<tr::TRrs>;
#[doc = "time register"]
pub mod tr;
#[doc = "DR (rw) register accessor: date register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr`]
module"]
pub type DR = crate::Reg<dr::DRrs>;
#[doc = "date register"]
pub mod dr;
#[doc = "CR (rw) register accessor: control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CRrs>;
#[doc = "control register"]
pub mod cr;
#[doc = "ISR (rw) register accessor: initialization and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`]
module"]
pub type ISR = crate::Reg<isr::ISRrs>;
#[doc = "initialization and status register"]
pub mod isr;
#[doc = "PRER (rw) register accessor: prescaler register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prer`]
module"]
pub type PRER = crate::Reg<prer::PRERrs>;
#[doc = "prescaler register"]
pub mod prer;
#[doc = "WUTR (rw) register accessor: wakeup timer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wutr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wutr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wutr`]
module"]
pub type WUTR = crate::Reg<wutr::WUTRrs>;
#[doc = "wakeup timer register"]
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
#[doc = "SSR (r) register accessor: sub second register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssr`]
module"]
pub type SSR = crate::Reg<ssr::SSRrs>;
#[doc = "sub second register"]
pub mod ssr;
#[doc = "SHIFTR (w) register accessor: shift control register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shiftr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shiftr`]
module"]
pub type SHIFTR = crate::Reg<shiftr::SHIFTRrs>;
#[doc = "shift control register"]
pub mod shiftr;
pub use dr as tsdr;
pub use ssr as tsssr;
pub use tr as tstr;
pub use DR as TSDR;
pub use SSR as TSSSR;
pub use TR as TSTR;
#[doc = "CALR (rw) register accessor: calibration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`calr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`calr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@calr`]
module"]
pub type CALR = crate::Reg<calr::CALRrs>;
#[doc = "calibration register"]
pub mod calr;
#[doc = "TAMPCR (rw) register accessor: tamper configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tampcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tampcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tampcr`]
module"]
pub type TAMPCR = crate::Reg<tampcr::TAMPCRrs>;
#[doc = "tamper configuration register"]
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
#[doc = "BKPR (rw) register accessor: backup register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bkpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bkpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkpr`]
module"]
pub type BKPR = crate::Reg<bkpr::BKPRrs>;
#[doc = "backup register"]
pub mod bkpr;
