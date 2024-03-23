#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    chep0r: CHEP0R,
    chep1r: CHEP1R,
    chep2r: CHEP2R,
    chep3r: CHEP3R,
    chep4r: CHEP4R,
    chep5r: CHEP5R,
    chep6r: CHEP6R,
    chep7r: CHEP7R,
    _reserved8: [u8; 0x20],
    cntr: CNTR,
    istr: ISTR,
    fnr: FNR,
    daddr: DADDR,
    _reserved12: [u8; 0x04],
    lpmcsr: LPMCSR,
    bcdr: BCDR,
}
impl RegisterBlock {
    #[doc = "0x00 - USB endpoint/channel 0 register"]
    #[inline(always)]
    pub const fn chep0r(&self) -> &CHEP0R {
        &self.chep0r
    }
    #[doc = "0x04 - USB endpoint/channel 1 register"]
    #[inline(always)]
    pub const fn chep1r(&self) -> &CHEP1R {
        &self.chep1r
    }
    #[doc = "0x08 - USB endpoint/channel 2 register"]
    #[inline(always)]
    pub const fn chep2r(&self) -> &CHEP2R {
        &self.chep2r
    }
    #[doc = "0x0c - USB endpoint/channel 3 register"]
    #[inline(always)]
    pub const fn chep3r(&self) -> &CHEP3R {
        &self.chep3r
    }
    #[doc = "0x10 - USB endpoint/channel 4 register"]
    #[inline(always)]
    pub const fn chep4r(&self) -> &CHEP4R {
        &self.chep4r
    }
    #[doc = "0x14 - USB endpoint/channel 5 register"]
    #[inline(always)]
    pub const fn chep5r(&self) -> &CHEP5R {
        &self.chep5r
    }
    #[doc = "0x18 - USB endpoint/channel 6 register"]
    #[inline(always)]
    pub const fn chep6r(&self) -> &CHEP6R {
        &self.chep6r
    }
    #[doc = "0x1c - USB endpoint/channel 7 register"]
    #[inline(always)]
    pub const fn chep7r(&self) -> &CHEP7R {
        &self.chep7r
    }
    #[doc = "0x40 - "]
    #[inline(always)]
    pub const fn cntr(&self) -> &CNTR {
        &self.cntr
    }
    #[doc = "0x44 - USB interrupt status register"]
    #[inline(always)]
    pub const fn istr(&self) -> &ISTR {
        &self.istr
    }
    #[doc = "0x48 - USB frame number register"]
    #[inline(always)]
    pub const fn fnr(&self) -> &FNR {
        &self.fnr
    }
    #[doc = "0x4c - USB_DADDR"]
    #[inline(always)]
    pub const fn daddr(&self) -> &DADDR {
        &self.daddr
    }
    #[doc = "0x54 - USB_LPMCSR"]
    #[inline(always)]
    pub const fn lpmcsr(&self) -> &LPMCSR {
        &self.lpmcsr
    }
    #[doc = "0x58 - USB_BCDR"]
    #[inline(always)]
    pub const fn bcdr(&self) -> &BCDR {
        &self.bcdr
    }
}
#[doc = "CHEP0R (rw) register accessor: USB endpoint/channel 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chep0r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chep0r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chep0r`]
module"]
pub type CHEP0R = crate::Reg<chep0r::CHEP0Rrs>;
#[doc = "USB endpoint/channel 0 register"]
pub mod chep0r;
#[doc = "CHEP1R (rw) register accessor: USB endpoint/channel 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chep1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chep1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chep1r`]
module"]
pub type CHEP1R = crate::Reg<chep1r::CHEP1Rrs>;
#[doc = "USB endpoint/channel 1 register"]
pub mod chep1r;
#[doc = "CHEP2R (rw) register accessor: USB endpoint/channel 2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chep2r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chep2r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chep2r`]
module"]
pub type CHEP2R = crate::Reg<chep2r::CHEP2Rrs>;
#[doc = "USB endpoint/channel 2 register"]
pub mod chep2r;
#[doc = "CHEP3R (rw) register accessor: USB endpoint/channel 3 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chep3r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chep3r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chep3r`]
module"]
pub type CHEP3R = crate::Reg<chep3r::CHEP3Rrs>;
#[doc = "USB endpoint/channel 3 register"]
pub mod chep3r;
#[doc = "CHEP4R (rw) register accessor: USB endpoint/channel 4 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chep4r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chep4r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chep4r`]
module"]
pub type CHEP4R = crate::Reg<chep4r::CHEP4Rrs>;
#[doc = "USB endpoint/channel 4 register"]
pub mod chep4r;
#[doc = "CHEP5R (rw) register accessor: USB endpoint/channel 5 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chep5r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chep5r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chep5r`]
module"]
pub type CHEP5R = crate::Reg<chep5r::CHEP5Rrs>;
#[doc = "USB endpoint/channel 5 register"]
pub mod chep5r;
#[doc = "CHEP6R (rw) register accessor: USB endpoint/channel 6 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chep6r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chep6r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chep6r`]
module"]
pub type CHEP6R = crate::Reg<chep6r::CHEP6Rrs>;
#[doc = "USB endpoint/channel 6 register"]
pub mod chep6r;
#[doc = "CHEP7R (rw) register accessor: USB endpoint/channel 7 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chep7r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chep7r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chep7r`]
module"]
pub type CHEP7R = crate::Reg<chep7r::CHEP7Rrs>;
#[doc = "USB endpoint/channel 7 register"]
pub mod chep7r;
#[doc = "CNTR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cntr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cntr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cntr`]
module"]
pub type CNTR = crate::Reg<cntr::CNTRrs>;
#[doc = ""]
pub mod cntr;
#[doc = "ISTR (rw) register accessor: USB interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`istr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`istr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@istr`]
module"]
pub type ISTR = crate::Reg<istr::ISTRrs>;
#[doc = "USB interrupt status register"]
pub mod istr;
#[doc = "FNR (r) register accessor: USB frame number register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fnr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fnr`]
module"]
pub type FNR = crate::Reg<fnr::FNRrs>;
#[doc = "USB frame number register"]
pub mod fnr;
#[doc = "DADDR (rw) register accessor: USB_DADDR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`daddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`daddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@daddr`]
module"]
pub type DADDR = crate::Reg<daddr::DADDRrs>;
#[doc = "USB_DADDR"]
pub mod daddr;
#[doc = "LPMCSR (rw) register accessor: USB_LPMCSR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpmcsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpmcsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpmcsr`]
module"]
pub type LPMCSR = crate::Reg<lpmcsr::LPMCSRrs>;
#[doc = "USB_LPMCSR"]
pub mod lpmcsr;
#[doc = "BCDR (rw) register accessor: USB_BCDR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdr`]
module"]
pub type BCDR = crate::Reg<bcdr::BCDRrs>;
#[doc = "USB_BCDR"]
pub mod bcdr;
