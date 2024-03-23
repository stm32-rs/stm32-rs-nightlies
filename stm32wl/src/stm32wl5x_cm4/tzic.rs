#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ier1: IER1,
    _reserved1: [u8; 0x0c],
    misr1: MISR1,
    _reserved2: [u8; 0x0c],
    icr1: ICR1,
}
impl RegisterBlock {
    #[doc = "0x00 - TZIC interrupt enable register 1"]
    #[inline(always)]
    pub const fn ier1(&self) -> &IER1 {
        &self.ier1
    }
    #[doc = "0x10 - TZIC status register 1"]
    #[inline(always)]
    pub const fn misr1(&self) -> &MISR1 {
        &self.misr1
    }
    #[doc = "0x20 - TZIC interrupt status clear register 1"]
    #[inline(always)]
    pub const fn icr1(&self) -> &ICR1 {
        &self.icr1
    }
}
#[doc = "IER1 (rw) register accessor: TZIC interrupt enable register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier1`]
module"]
pub type IER1 = crate::Reg<ier1::IER1rs>;
#[doc = "TZIC interrupt enable register 1"]
pub mod ier1;
#[doc = "MISR1 (r) register accessor: TZIC status register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`misr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@misr1`]
module"]
pub type MISR1 = crate::Reg<misr1::MISR1rs>;
#[doc = "TZIC status register 1"]
pub mod misr1;
#[doc = "ICR1 (rw) register accessor: TZIC interrupt status clear register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr1`]
module"]
pub type ICR1 = crate::Reg<icr1::ICR1rs>;
#[doc = "TZIC interrupt status clear register 1"]
pub mod icr1;
