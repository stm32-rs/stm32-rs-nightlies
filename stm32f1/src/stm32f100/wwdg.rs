#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: CR,
    cfr: CFR,
    sr: SR,
}
impl RegisterBlock {
    #[doc = "0x00 - Control register (WWDG_CR)"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x04 - Configuration register (WWDG_CFR)"]
    #[inline(always)]
    pub const fn cfr(&self) -> &CFR {
        &self.cfr
    }
    #[doc = "0x08 - Status register (WWDG_SR)"]
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
}
#[doc = "CR (rw) register accessor: Control register (WWDG_CR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CRrs>;
#[doc = "Control register (WWDG_CR)"]
pub mod cr;
#[doc = "CFR (rw) register accessor: Configuration register (WWDG_CFR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfr`]
module"]
pub type CFR = crate::Reg<cfr::CFRrs>;
#[doc = "Configuration register (WWDG_CFR)"]
pub mod cfr;
#[doc = "SR (rw) register accessor: Status register (WWDG_SR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
pub type SR = crate::Reg<sr::SRrs>;
#[doc = "Status register (WWDG_SR)"]
pub mod sr;
