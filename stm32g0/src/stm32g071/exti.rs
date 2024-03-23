#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    rtsr1: RTSR1,
    ftsr1: FTSR1,
    swier1: SWIER1,
    rpr1: RPR1,
    fpr1: FPR1,
    _reserved5: [u8; 0x4c],
    exticr1: EXTICR1,
    exticr2: EXTICR2,
    exticr3: EXTICR3,
    exticr4: EXTICR4,
    _reserved9: [u8; 0x10],
    imr1: IMR1,
    emr1: EMR1,
    _reserved11: [u8; 0x08],
    imr2: IMR2,
    emr2: EMR2,
    _reserved13: [u8; 0x0340],
    hwcfgr7: HWCFGR7,
    hwcfgr6: HWCFGR6,
    hwcfgr5: HWCFGR5,
    hwcfgr4: HWCFGR4,
    hwcfgr3: HWCFGR3,
    hwcfgr2: HWCFGR2,
    hwcfgr1: HWCFGR1,
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
    #[doc = "0x90 - EXTI CPU wakeup with interrupt mask register"]
    #[inline(always)]
    pub const fn imr2(&self) -> &IMR2 {
        &self.imr2
    }
    #[doc = "0x94 - EXTI CPU wakeup with event mask register"]
    #[inline(always)]
    pub const fn emr2(&self) -> &EMR2 {
        &self.emr2
    }
    #[doc = "0x3d8 - Hardware configuration registers"]
    #[inline(always)]
    pub const fn hwcfgr7(&self) -> &HWCFGR7 {
        &self.hwcfgr7
    }
    #[doc = "0x3dc - Hardware configuration registers"]
    #[inline(always)]
    pub const fn hwcfgr6(&self) -> &HWCFGR6 {
        &self.hwcfgr6
    }
    #[doc = "0x3e0 - Hardware configuration registers"]
    #[inline(always)]
    pub const fn hwcfgr5(&self) -> &HWCFGR5 {
        &self.hwcfgr5
    }
    #[doc = "0x3e4 - Hardware configuration registers"]
    #[inline(always)]
    pub const fn hwcfgr4(&self) -> &HWCFGR4 {
        &self.hwcfgr4
    }
    #[doc = "0x3e8 - Hardware configuration registers"]
    #[inline(always)]
    pub const fn hwcfgr3(&self) -> &HWCFGR3 {
        &self.hwcfgr3
    }
    #[doc = "0x3ec - Hardware configuration registers"]
    #[inline(always)]
    pub const fn hwcfgr2(&self) -> &HWCFGR2 {
        &self.hwcfgr2
    }
    #[doc = "0x3f0 - Hardware configuration registers"]
    #[inline(always)]
    pub const fn hwcfgr1(&self) -> &HWCFGR1 {
        &self.hwcfgr1
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
#[doc = "IMR2 (rw) register accessor: EXTI CPU wakeup with interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr2`]
module"]
pub type IMR2 = crate::Reg<imr2::IMR2rs>;
#[doc = "EXTI CPU wakeup with interrupt mask register"]
pub mod imr2;
#[doc = "EMR2 (rw) register accessor: EXTI CPU wakeup with event mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emr2`]
module"]
pub type EMR2 = crate::Reg<emr2::EMR2rs>;
#[doc = "EXTI CPU wakeup with event mask register"]
pub mod emr2;
#[doc = "HWCFGR7 (rw) register accessor: Hardware configuration registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwcfgr7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hwcfgr7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwcfgr7`]
module"]
pub type HWCFGR7 = crate::Reg<hwcfgr7::HWCFGR7rs>;
#[doc = "Hardware configuration registers"]
pub mod hwcfgr7;
#[doc = "HWCFGR6 (rw) register accessor: Hardware configuration registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwcfgr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hwcfgr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwcfgr6`]
module"]
pub type HWCFGR6 = crate::Reg<hwcfgr6::HWCFGR6rs>;
#[doc = "Hardware configuration registers"]
pub mod hwcfgr6;
#[doc = "HWCFGR5 (rw) register accessor: Hardware configuration registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwcfgr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hwcfgr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwcfgr5`]
module"]
pub type HWCFGR5 = crate::Reg<hwcfgr5::HWCFGR5rs>;
#[doc = "Hardware configuration registers"]
pub mod hwcfgr5;
#[doc = "HWCFGR4 (rw) register accessor: Hardware configuration registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwcfgr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hwcfgr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwcfgr4`]
module"]
pub type HWCFGR4 = crate::Reg<hwcfgr4::HWCFGR4rs>;
#[doc = "Hardware configuration registers"]
pub mod hwcfgr4;
#[doc = "HWCFGR3 (rw) register accessor: Hardware configuration registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwcfgr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hwcfgr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwcfgr3`]
module"]
pub type HWCFGR3 = crate::Reg<hwcfgr3::HWCFGR3rs>;
#[doc = "Hardware configuration registers"]
pub mod hwcfgr3;
#[doc = "HWCFGR2 (rw) register accessor: Hardware configuration registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwcfgr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hwcfgr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwcfgr2`]
module"]
pub type HWCFGR2 = crate::Reg<hwcfgr2::HWCFGR2rs>;
#[doc = "Hardware configuration registers"]
pub mod hwcfgr2;
#[doc = "HWCFGR1 (r) register accessor: Hardware configuration registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwcfgr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwcfgr1`]
module"]
pub type HWCFGR1 = crate::Reg<hwcfgr1::HWCFGR1rs>;
#[doc = "Hardware configuration registers"]
pub mod hwcfgr1;
