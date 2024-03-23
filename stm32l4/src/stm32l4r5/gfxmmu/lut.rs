#[repr(C)]
#[doc = "Cluster LUT%s, containing LUT*L, LUT*H"]
pub struct LUT {
    lutl: LUTL,
    luth: LUTH,
}
impl LUT {
    #[doc = "0x00 - Graphic MMU LUT entry x low"]
    #[inline(always)]
    pub const fn lutl(&self) -> &LUTL {
        &self.lutl
    }
    #[doc = "0x04 - Graphic MMU LUT entry x high"]
    #[inline(always)]
    pub const fn luth(&self) -> &LUTH {
        &self.luth
    }
}
#[doc = "LUTL (rw) register accessor: Graphic MMU LUT entry x low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lutl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lutl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lutl`]
module"]
pub type LUTL = crate::Reg<lutl::LUTLrs>;
#[doc = "Graphic MMU LUT entry x low"]
pub mod lutl;
#[doc = "LUTH (rw) register accessor: Graphic MMU LUT entry x high\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`luth::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`luth::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@luth`]
module"]
pub type LUTH = crate::Reg<luth::LUTHrs>;
#[doc = "Graphic MMU LUT entry x high"]
pub mod luth;
