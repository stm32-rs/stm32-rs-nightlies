#[repr(C)]
#[doc = "CAN Filter Bank cluster"]
pub struct FB {
    fr1: FR1,
    fr2: FR2,
}
impl FB {
    #[doc = "0x00 - Filter bank x register 1"]
    #[inline(always)]
    pub const fn fr1(&self) -> &FR1 {
        &self.fr1
    }
    #[doc = "0x04 - Filter bank x register 2"]
    #[inline(always)]
    pub const fn fr2(&self) -> &FR2 {
        &self.fr2
    }
}
#[doc = "FR1 (rw) register accessor: Filter bank x register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fr1`]
module"]
pub type FR1 = crate::Reg<fr1::FR1rs>;
#[doc = "Filter bank x register 1"]
pub mod fr1;
#[doc = "FR2 (rw) register accessor: Filter bank x register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fr2`]
module"]
pub type FR2 = crate::Reg<fr2::FR2rs>;
#[doc = "Filter bank x register 2"]
pub mod fr2;
