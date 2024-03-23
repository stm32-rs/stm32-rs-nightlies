#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    usbphyc_pll: USBPHYC_PLL,
    _reserved1: [u8; 0x04],
    usbphyc_misc: USBPHYC_MISC,
    _reserved2: [u8; 0x0100],
    usbphyc_tune1: USBPHYC_TUNE1,
    _reserved3: [u8; 0xfc],
    usbphyc_tune2: USBPHYC_TUNE2,
    _reserved4: [u8; 0x0dec],
    usbphyc_verr: USBPHYC_VERR,
}
impl RegisterBlock {
    #[doc = "0x00 - This register is used to control the PLL of the HS PHY."]
    #[inline(always)]
    pub const fn usbphyc_pll(&self) -> &USBPHYC_PLL {
        &self.usbphyc_pll
    }
    #[doc = "0x08 - This register is used to control the switch between controllers for the HS PHY."]
    #[inline(always)]
    pub const fn usbphyc_misc(&self) -> &USBPHYC_MISC {
        &self.usbphyc_misc
    }
    #[doc = "0x10c - This register is used to control the tune interface of the HS PHY, port #x."]
    #[inline(always)]
    pub const fn usbphyc_tune1(&self) -> &USBPHYC_TUNE1 {
        &self.usbphyc_tune1
    }
    #[doc = "0x20c - This register is used to control the tune interface of the HS PHY, port #x."]
    #[inline(always)]
    pub const fn usbphyc_tune2(&self) -> &USBPHYC_TUNE2 {
        &self.usbphyc_tune2
    }
    #[doc = "0xffc - This register defines the version of this IP."]
    #[inline(always)]
    pub const fn usbphyc_verr(&self) -> &USBPHYC_VERR {
        &self.usbphyc_verr
    }
}
#[doc = "USBPHYC_PLL (rw) register accessor: This register is used to control the PLL of the HS PHY.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbphyc_pll::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbphyc_pll::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbphyc_pll`]
module"]
pub type USBPHYC_PLL = crate::Reg<usbphyc_pll::USBPHYC_PLLrs>;
#[doc = "This register is used to control the PLL of the HS PHY."]
pub mod usbphyc_pll;
#[doc = "USBPHYC_MISC (rw) register accessor: This register is used to control the switch between controllers for the HS PHY.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbphyc_misc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbphyc_misc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbphyc_misc`]
module"]
pub type USBPHYC_MISC = crate::Reg<usbphyc_misc::USBPHYC_MISCrs>;
#[doc = "This register is used to control the switch between controllers for the HS PHY."]
pub mod usbphyc_misc;
#[doc = "USBPHYC_TUNE1 (rw) register accessor: This register is used to control the tune interface of the HS PHY, port #x.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbphyc_tune1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbphyc_tune1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbphyc_tune1`]
module"]
pub type USBPHYC_TUNE1 = crate::Reg<usbphyc_tune1::USBPHYC_TUNE1rs>;
#[doc = "This register is used to control the tune interface of the HS PHY, port #x."]
pub mod usbphyc_tune1;
#[doc = "USBPHYC_TUNE2 (rw) register accessor: This register is used to control the tune interface of the HS PHY, port #x.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbphyc_tune2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbphyc_tune2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbphyc_tune2`]
module"]
pub type USBPHYC_TUNE2 = crate::Reg<usbphyc_tune2::USBPHYC_TUNE2rs>;
#[doc = "This register is used to control the tune interface of the HS PHY, port #x."]
pub mod usbphyc_tune2;
#[doc = "USBPHYC_VERR (r) register accessor: This register defines the version of this IP.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbphyc_verr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbphyc_verr`]
module"]
pub type USBPHYC_VERR = crate::Reg<usbphyc_verr::USBPHYC_VERRrs>;
#[doc = "This register defines the version of this IP."]
pub mod usbphyc_verr;
