#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    exti_rtsr1: EXTI_RTSR1,
    exti_ftsr1: EXTI_FTSR1,
    exti_swier1: EXTI_SWIER1,
    exti_rpr1: EXTI_RPR1,
    exti_fpr1: EXTI_FPR1,
    exti_seccfgr1: EXTI_SECCFGR1,
    exti_privcfgr1: EXTI_PRIVCFGR1,
    _reserved7: [u8; 0x44],
    exti_exticr1: EXTI_EXTICR1,
    exti_exticr2: EXTI_EXTICR2,
    exti_exticr3: EXTI_EXTICR3,
    exti_exticr4: EXTI_EXTICR4,
    exti_lockr: EXTI_LOCKR,
    _reserved12: [u8; 0x0c],
    exti_imr1: EXTI_IMR1,
    exti_emr1: EXTI_EMR1,
}
impl RegisterBlock {
    #[doc = "0x00 - EXTI rising trigger selection register"]
    #[inline(always)]
    pub const fn exti_rtsr1(&self) -> &EXTI_RTSR1 {
        &self.exti_rtsr1
    }
    #[doc = "0x04 - EXTI falling trigger selection register"]
    #[inline(always)]
    pub const fn exti_ftsr1(&self) -> &EXTI_FTSR1 {
        &self.exti_ftsr1
    }
    #[doc = "0x08 - EXTI software interrupt event register"]
    #[inline(always)]
    pub const fn exti_swier1(&self) -> &EXTI_SWIER1 {
        &self.exti_swier1
    }
    #[doc = "0x0c - EXTI rising edge pending register"]
    #[inline(always)]
    pub const fn exti_rpr1(&self) -> &EXTI_RPR1 {
        &self.exti_rpr1
    }
    #[doc = "0x10 - EXTI falling edge pending register"]
    #[inline(always)]
    pub const fn exti_fpr1(&self) -> &EXTI_FPR1 {
        &self.exti_fpr1
    }
    #[doc = "0x14 - EXTI security configuration register"]
    #[inline(always)]
    pub const fn exti_seccfgr1(&self) -> &EXTI_SECCFGR1 {
        &self.exti_seccfgr1
    }
    #[doc = "0x18 - EXTI privilege configuration register"]
    #[inline(always)]
    pub const fn exti_privcfgr1(&self) -> &EXTI_PRIVCFGR1 {
        &self.exti_privcfgr1
    }
    #[doc = "0x60 - EXTI external interrupt selection register"]
    #[inline(always)]
    pub const fn exti_exticr1(&self) -> &EXTI_EXTICR1 {
        &self.exti_exticr1
    }
    #[doc = "0x64 - EXTI external interrupt selection register"]
    #[inline(always)]
    pub const fn exti_exticr2(&self) -> &EXTI_EXTICR2 {
        &self.exti_exticr2
    }
    #[doc = "0x68 - EXTI external interrupt selection register"]
    #[inline(always)]
    pub const fn exti_exticr3(&self) -> &EXTI_EXTICR3 {
        &self.exti_exticr3
    }
    #[doc = "0x6c - EXTI external interrupt selection register"]
    #[inline(always)]
    pub const fn exti_exticr4(&self) -> &EXTI_EXTICR4 {
        &self.exti_exticr4
    }
    #[doc = "0x70 - EXTI lock register"]
    #[inline(always)]
    pub const fn exti_lockr(&self) -> &EXTI_LOCKR {
        &self.exti_lockr
    }
    #[doc = "0x80 - EXTI CPU wake-up with interrupt mask register"]
    #[inline(always)]
    pub const fn exti_imr1(&self) -> &EXTI_IMR1 {
        &self.exti_imr1
    }
    #[doc = "0x84 - EXTI CPU wake-up with event mask register"]
    #[inline(always)]
    pub const fn exti_emr1(&self) -> &EXTI_EMR1 {
        &self.exti_emr1
    }
}
#[doc = "EXTI_RTSR1 (rw) register accessor: EXTI rising trigger selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_rtsr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_rtsr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exti_rtsr1`]
module"]
pub type EXTI_RTSR1 = crate::Reg<exti_rtsr1::EXTI_RTSR1rs>;
#[doc = "EXTI rising trigger selection register"]
pub mod exti_rtsr1;
#[doc = "EXTI_FTSR1 (rw) register accessor: EXTI falling trigger selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_ftsr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_ftsr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exti_ftsr1`]
module"]
pub type EXTI_FTSR1 = crate::Reg<exti_ftsr1::EXTI_FTSR1rs>;
#[doc = "EXTI falling trigger selection register"]
pub mod exti_ftsr1;
#[doc = "EXTI_SWIER1 (rw) register accessor: EXTI software interrupt event register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_swier1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_swier1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exti_swier1`]
module"]
pub type EXTI_SWIER1 = crate::Reg<exti_swier1::EXTI_SWIER1rs>;
#[doc = "EXTI software interrupt event register"]
pub mod exti_swier1;
#[doc = "EXTI_RPR1 (rw) register accessor: EXTI rising edge pending register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_rpr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_rpr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exti_rpr1`]
module"]
pub type EXTI_RPR1 = crate::Reg<exti_rpr1::EXTI_RPR1rs>;
#[doc = "EXTI rising edge pending register"]
pub mod exti_rpr1;
#[doc = "EXTI_FPR1 (rw) register accessor: EXTI falling edge pending register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_fpr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_fpr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exti_fpr1`]
module"]
pub type EXTI_FPR1 = crate::Reg<exti_fpr1::EXTI_FPR1rs>;
#[doc = "EXTI falling edge pending register"]
pub mod exti_fpr1;
#[doc = "EXTI_SECCFGR1 (rw) register accessor: EXTI security configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_seccfgr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_seccfgr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exti_seccfgr1`]
module"]
pub type EXTI_SECCFGR1 = crate::Reg<exti_seccfgr1::EXTI_SECCFGR1rs>;
#[doc = "EXTI security configuration register"]
pub mod exti_seccfgr1;
#[doc = "EXTI_PRIVCFGR1 (rw) register accessor: EXTI privilege configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_privcfgr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_privcfgr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exti_privcfgr1`]
module"]
pub type EXTI_PRIVCFGR1 = crate::Reg<exti_privcfgr1::EXTI_PRIVCFGR1rs>;
#[doc = "EXTI privilege configuration register"]
pub mod exti_privcfgr1;
#[doc = "EXTI_EXTICR1 (rw) register accessor: EXTI external interrupt selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_exticr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_exticr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exti_exticr1`]
module"]
pub type EXTI_EXTICR1 = crate::Reg<exti_exticr1::EXTI_EXTICR1rs>;
#[doc = "EXTI external interrupt selection register"]
pub mod exti_exticr1;
#[doc = "EXTI_EXTICR2 (rw) register accessor: EXTI external interrupt selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_exticr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_exticr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exti_exticr2`]
module"]
pub type EXTI_EXTICR2 = crate::Reg<exti_exticr2::EXTI_EXTICR2rs>;
#[doc = "EXTI external interrupt selection register"]
pub mod exti_exticr2;
#[doc = "EXTI_EXTICR3 (rw) register accessor: EXTI external interrupt selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_exticr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_exticr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exti_exticr3`]
module"]
pub type EXTI_EXTICR3 = crate::Reg<exti_exticr3::EXTI_EXTICR3rs>;
#[doc = "EXTI external interrupt selection register"]
pub mod exti_exticr3;
#[doc = "EXTI_EXTICR4 (rw) register accessor: EXTI external interrupt selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_exticr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_exticr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exti_exticr4`]
module"]
pub type EXTI_EXTICR4 = crate::Reg<exti_exticr4::EXTI_EXTICR4rs>;
#[doc = "EXTI external interrupt selection register"]
pub mod exti_exticr4;
#[doc = "EXTI_LOCKR (rw) register accessor: EXTI lock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_lockr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_lockr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exti_lockr`]
module"]
pub type EXTI_LOCKR = crate::Reg<exti_lockr::EXTI_LOCKRrs>;
#[doc = "EXTI lock register"]
pub mod exti_lockr;
#[doc = "EXTI_IMR1 (rw) register accessor: EXTI CPU wake-up with interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_imr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_imr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exti_imr1`]
module"]
pub type EXTI_IMR1 = crate::Reg<exti_imr1::EXTI_IMR1rs>;
#[doc = "EXTI CPU wake-up with interrupt mask register"]
pub mod exti_imr1;
#[doc = "EXTI_EMR1 (rw) register accessor: EXTI CPU wake-up with event mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_emr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_emr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exti_emr1`]
module"]
pub type EXTI_EMR1 = crate::Reg<exti_emr1::EXTI_EMR1rs>;
#[doc = "EXTI CPU wake-up with event mask register"]
pub mod exti_emr1;
