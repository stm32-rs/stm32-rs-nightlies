#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    rtsr1: RTSR1,
    ftsr1: FTSR1,
    swier1: SWIER1,
    rpr1: RPR1,
    fpr1: FPR1,
    seccfgr1: SECCFGR1,
    privcfgr1: PRIVCFGR1,
    _reserved7: [u8; 0x04],
    rtsr2: RTSR2,
    ftsr2: FTSR2,
    swier2: SWIER2,
    rpr2: RPR2,
    fpr2: FPR2,
    seccfgr2: SECCFGR2,
    privcfgr2: PRIVCFGR2,
    _reserved14: [u8; 0x24],
    exticr1: EXTICR1,
    exticr2: EXTICR2,
    exticr3: EXTICR3,
    exticr4: EXTICR4,
    lockr: LOCKR,
    _reserved19: [u8; 0x0c],
    imr1: IMR1,
    emr1: EMR1,
    _reserved21: [u8; 0x08],
    imr2: IMR2,
    emr2: EMR2,
}
impl RegisterBlock {
    #[doc = "0x00 - EXTI rising trigger selection register"]
    #[inline(always)]
    pub const fn rtsr1(&self) -> &RTSR1 {
        &self.rtsr1
    }
    #[doc = "0x04 - EXTI falling trigger selection register"]
    #[inline(always)]
    pub const fn ftsr1(&self) -> &FTSR1 {
        &self.ftsr1
    }
    #[doc = "0x08 - EXTI software interrupt event register"]
    #[inline(always)]
    pub const fn swier1(&self) -> &SWIER1 {
        &self.swier1
    }
    #[doc = "0x0c - EXTI rising edge pending register"]
    #[inline(always)]
    pub const fn rpr1(&self) -> &RPR1 {
        &self.rpr1
    }
    #[doc = "0x10 - EXTI falling edge pending register"]
    #[inline(always)]
    pub const fn fpr1(&self) -> &FPR1 {
        &self.fpr1
    }
    #[doc = "0x14 - EXTI security configuration register"]
    #[inline(always)]
    pub const fn seccfgr1(&self) -> &SECCFGR1 {
        &self.seccfgr1
    }
    #[doc = "0x18 - EXTI privilege configuration register"]
    #[inline(always)]
    pub const fn privcfgr1(&self) -> &PRIVCFGR1 {
        &self.privcfgr1
    }
    #[doc = "0x20 - EXTI rising trigger selection register 2"]
    #[inline(always)]
    pub const fn rtsr2(&self) -> &RTSR2 {
        &self.rtsr2
    }
    #[doc = "0x24 - EXTI falling trigger selection register 2"]
    #[inline(always)]
    pub const fn ftsr2(&self) -> &FTSR2 {
        &self.ftsr2
    }
    #[doc = "0x28 - EXTI software interrupt event register 2"]
    #[inline(always)]
    pub const fn swier2(&self) -> &SWIER2 {
        &self.swier2
    }
    #[doc = "0x2c - EXTI rising edge pending register 2"]
    #[inline(always)]
    pub const fn rpr2(&self) -> &RPR2 {
        &self.rpr2
    }
    #[doc = "0x30 - EXTI falling edge pending register 2"]
    #[inline(always)]
    pub const fn fpr2(&self) -> &FPR2 {
        &self.fpr2
    }
    #[doc = "0x34 - EXTI security configuration register 2"]
    #[inline(always)]
    pub const fn seccfgr2(&self) -> &SECCFGR2 {
        &self.seccfgr2
    }
    #[doc = "0x38 - EXTI privilege configuration register 2"]
    #[inline(always)]
    pub const fn privcfgr2(&self) -> &PRIVCFGR2 {
        &self.privcfgr2
    }
    #[doc = "0x60 - EXTI external interrupt selection register"]
    #[inline(always)]
    pub const fn exticr1(&self) -> &EXTICR1 {
        &self.exticr1
    }
    #[doc = "0x64 - EXTI external interrupt selection register"]
    #[inline(always)]
    pub const fn exticr2(&self) -> &EXTICR2 {
        &self.exticr2
    }
    #[doc = "0x68 - EXTI external interrupt selection register"]
    #[inline(always)]
    pub const fn exticr3(&self) -> &EXTICR3 {
        &self.exticr3
    }
    #[doc = "0x6c - EXTI external interrupt selection register"]
    #[inline(always)]
    pub const fn exticr4(&self) -> &EXTICR4 {
        &self.exticr4
    }
    #[doc = "0x70 - EXTI lock register"]
    #[inline(always)]
    pub const fn lockr(&self) -> &LOCKR {
        &self.lockr
    }
    #[doc = "0x80 - EXTI CPU wakeup with interrupt mask register"]
    #[inline(always)]
    pub const fn imr1(&self) -> &IMR1 {
        &self.imr1
    }
    #[doc = "0x84 - EXTI CPU wakeup with event mask register"]
    #[inline(always)]
    pub const fn emr1(&self) -> &EMR1 {
        &self.emr1
    }
    #[doc = "0x90 - EXTI CPU wakeup with interrupt mask register 2"]
    #[inline(always)]
    pub const fn imr2(&self) -> &IMR2 {
        &self.imr2
    }
    #[doc = "0x94 - EXTI CPU wakeup with event mask register 2"]
    #[inline(always)]
    pub const fn emr2(&self) -> &EMR2 {
        &self.emr2
    }
}
#[doc = "RTSR1 (rw) register accessor: EXTI rising trigger selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtsr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtsr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtsr1`]
module"]
pub type RTSR1 = crate::Reg<rtsr1::RTSR1rs>;
#[doc = "EXTI rising trigger selection register"]
pub mod rtsr1;
#[doc = "FTSR1 (rw) register accessor: EXTI falling trigger selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ftsr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ftsr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ftsr1`]
module"]
pub type FTSR1 = crate::Reg<ftsr1::FTSR1rs>;
#[doc = "EXTI falling trigger selection register"]
pub mod ftsr1;
#[doc = "SWIER1 (rw) register accessor: EXTI software interrupt event register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swier1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swier1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swier1`]
module"]
pub type SWIER1 = crate::Reg<swier1::SWIER1rs>;
#[doc = "EXTI software interrupt event register"]
pub mod swier1;
#[doc = "RPR1 (rw) register accessor: EXTI rising edge pending register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rpr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rpr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rpr1`]
module"]
pub type RPR1 = crate::Reg<rpr1::RPR1rs>;
#[doc = "EXTI rising edge pending register"]
pub mod rpr1;
#[doc = "FPR1 (rw) register accessor: EXTI falling edge pending register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fpr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fpr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpr1`]
module"]
pub type FPR1 = crate::Reg<fpr1::FPR1rs>;
#[doc = "EXTI falling edge pending register"]
pub mod fpr1;
#[doc = "SECCFGR1 (rw) register accessor: EXTI security configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seccfgr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seccfgr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seccfgr1`]
module"]
pub type SECCFGR1 = crate::Reg<seccfgr1::SECCFGR1rs>;
#[doc = "EXTI security configuration register"]
pub mod seccfgr1;
#[doc = "PRIVCFGR1 (rw) register accessor: EXTI privilege configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`privcfgr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`privcfgr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@privcfgr1`]
module"]
pub type PRIVCFGR1 = crate::Reg<privcfgr1::PRIVCFGR1rs>;
#[doc = "EXTI privilege configuration register"]
pub mod privcfgr1;
#[doc = "RTSR2 (rw) register accessor: EXTI rising trigger selection register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtsr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtsr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtsr2`]
module"]
pub type RTSR2 = crate::Reg<rtsr2::RTSR2rs>;
#[doc = "EXTI rising trigger selection register 2"]
pub mod rtsr2;
#[doc = "FTSR2 (rw) register accessor: EXTI falling trigger selection register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ftsr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ftsr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ftsr2`]
module"]
pub type FTSR2 = crate::Reg<ftsr2::FTSR2rs>;
#[doc = "EXTI falling trigger selection register 2"]
pub mod ftsr2;
#[doc = "SWIER2 (rw) register accessor: EXTI software interrupt event register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swier2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swier2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swier2`]
module"]
pub type SWIER2 = crate::Reg<swier2::SWIER2rs>;
#[doc = "EXTI software interrupt event register 2"]
pub mod swier2;
#[doc = "RPR2 (rw) register accessor: EXTI rising edge pending register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rpr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rpr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rpr2`]
module"]
pub type RPR2 = crate::Reg<rpr2::RPR2rs>;
#[doc = "EXTI rising edge pending register 2"]
pub mod rpr2;
#[doc = "FPR2 (rw) register accessor: EXTI falling edge pending register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fpr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fpr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpr2`]
module"]
pub type FPR2 = crate::Reg<fpr2::FPR2rs>;
#[doc = "EXTI falling edge pending register 2"]
pub mod fpr2;
#[doc = "SECCFGR2 (rw) register accessor: EXTI security configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seccfgr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seccfgr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seccfgr2`]
module"]
pub type SECCFGR2 = crate::Reg<seccfgr2::SECCFGR2rs>;
#[doc = "EXTI security configuration register 2"]
pub mod seccfgr2;
#[doc = "PRIVCFGR2 (rw) register accessor: EXTI privilege configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`privcfgr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`privcfgr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@privcfgr2`]
module"]
pub type PRIVCFGR2 = crate::Reg<privcfgr2::PRIVCFGR2rs>;
#[doc = "EXTI privilege configuration register 2"]
pub mod privcfgr2;
#[doc = "EXTICR1 (rw) register accessor: EXTI external interrupt selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exticr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exticr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exticr1`]
module"]
pub type EXTICR1 = crate::Reg<exticr1::EXTICR1rs>;
#[doc = "EXTI external interrupt selection register"]
pub mod exticr1;
#[doc = "EXTICR2 (rw) register accessor: EXTI external interrupt selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exticr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exticr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exticr2`]
module"]
pub type EXTICR2 = crate::Reg<exticr2::EXTICR2rs>;
#[doc = "EXTI external interrupt selection register"]
pub mod exticr2;
#[doc = "EXTICR3 (rw) register accessor: EXTI external interrupt selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exticr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exticr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exticr3`]
module"]
pub type EXTICR3 = crate::Reg<exticr3::EXTICR3rs>;
#[doc = "EXTI external interrupt selection register"]
pub mod exticr3;
#[doc = "EXTICR4 (rw) register accessor: EXTI external interrupt selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exticr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exticr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exticr4`]
module"]
pub type EXTICR4 = crate::Reg<exticr4::EXTICR4rs>;
#[doc = "EXTI external interrupt selection register"]
pub mod exticr4;
#[doc = "LOCKR (rw) register accessor: EXTI lock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lockr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lockr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lockr`]
module"]
pub type LOCKR = crate::Reg<lockr::LOCKRrs>;
#[doc = "EXTI lock register"]
pub mod lockr;
#[doc = "IMR1 (rw) register accessor: EXTI CPU wakeup with interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr1`]
module"]
pub type IMR1 = crate::Reg<imr1::IMR1rs>;
#[doc = "EXTI CPU wakeup with interrupt mask register"]
pub mod imr1;
#[doc = "EMR1 (rw) register accessor: EXTI CPU wakeup with event mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emr1`]
module"]
pub type EMR1 = crate::Reg<emr1::EMR1rs>;
#[doc = "EXTI CPU wakeup with event mask register"]
pub mod emr1;
#[doc = "IMR2 (rw) register accessor: EXTI CPU wakeup with interrupt mask register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr2`]
module"]
pub type IMR2 = crate::Reg<imr2::IMR2rs>;
#[doc = "EXTI CPU wakeup with interrupt mask register 2"]
pub mod imr2;
#[doc = "EMR2 (rw) register accessor: EXTI CPU wakeup with event mask register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emr2`]
module"]
pub type EMR2 = crate::Reg<emr2::EMR2rs>;
#[doc = "EXTI CPU wakeup with event mask register 2"]
pub mod emr2;
