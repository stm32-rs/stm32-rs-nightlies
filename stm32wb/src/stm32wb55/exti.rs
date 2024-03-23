#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    rtsr1: RTSR1,
    ftsr1: FTSR1,
    swier1: SWIER1,
    pr1: PR1,
    _reserved4: [u8; 0x10],
    rtsr2: RTSR2,
    ftsr2: FTSR2,
    swier2: SWIER2,
    pr2: PR2,
    _reserved8: [u8; 0x50],
    imr1: IMR1,
    emr1: EMR1,
    _reserved10: [u8; 0x08],
    imr2: IMR2,
    emr2: EMR2,
    _reserved12: [u8; 0x28],
    c2imr1: C2IMR1,
    c2emr1: C2EMR1,
    _reserved14: [u8; 0x08],
    c2imr2: C2IMR2,
    c2emr2: C2EMR2,
    _reserved16: [u8; 0x0300],
    hwcfgr7: HWCFGR7,
    hwcfgr6: HWCFGR6,
    hwcfgr5: HWCFGR5,
    hwcfgr4: HWCFGR4,
    hwcfgr3: HWCFGR3,
    hwcfgr2: HWCFGR2,
    hwcfgr1: HWCFGR1,
    verr: VERR,
    ipidr: IPIDR,
    sidr: SIDR,
}
impl RegisterBlock {
    #[doc = "0x00 - rising trigger selection register"]
    #[inline(always)]
    pub const fn rtsr1(&self) -> &RTSR1 {
        &self.rtsr1
    }
    #[doc = "0x04 - falling trigger selection register"]
    #[inline(always)]
    pub const fn ftsr1(&self) -> &FTSR1 {
        &self.ftsr1
    }
    #[doc = "0x08 - software interrupt event register"]
    #[inline(always)]
    pub const fn swier1(&self) -> &SWIER1 {
        &self.swier1
    }
    #[doc = "0x0c - EXTI pending register"]
    #[inline(always)]
    pub const fn pr1(&self) -> &PR1 {
        &self.pr1
    }
    #[doc = "0x20 - rising trigger selection register"]
    #[inline(always)]
    pub const fn rtsr2(&self) -> &RTSR2 {
        &self.rtsr2
    }
    #[doc = "0x24 - falling trigger selection register"]
    #[inline(always)]
    pub const fn ftsr2(&self) -> &FTSR2 {
        &self.ftsr2
    }
    #[doc = "0x28 - software interrupt event register"]
    #[inline(always)]
    pub const fn swier2(&self) -> &SWIER2 {
        &self.swier2
    }
    #[doc = "0x2c - pending register"]
    #[inline(always)]
    pub const fn pr2(&self) -> &PR2 {
        &self.pr2
    }
    #[doc = "0x80 - CPUm wakeup with interrupt mask register"]
    #[inline(always)]
    pub const fn imr1(&self) -> &IMR1 {
        &self.imr1
    }
    #[doc = "0x84 - CPUm wakeup with event mask register"]
    #[inline(always)]
    pub const fn emr1(&self) -> &EMR1 {
        &self.emr1
    }
    #[doc = "0x90 - CPUm wakeup with interrupt mask register"]
    #[inline(always)]
    pub const fn imr2(&self) -> &IMR2 {
        &self.imr2
    }
    #[doc = "0x94 - CPUm wakeup with event mask register"]
    #[inline(always)]
    pub const fn emr2(&self) -> &EMR2 {
        &self.emr2
    }
    #[doc = "0xc0 - CPUm wakeup with interrupt mask register"]
    #[inline(always)]
    pub const fn c2imr1(&self) -> &C2IMR1 {
        &self.c2imr1
    }
    #[doc = "0xc4 - CPUm wakeup with event mask register"]
    #[inline(always)]
    pub const fn c2emr1(&self) -> &C2EMR1 {
        &self.c2emr1
    }
    #[doc = "0xd0 - CPUm wakeup with interrupt mask register"]
    #[inline(always)]
    pub const fn c2imr2(&self) -> &C2IMR2 {
        &self.c2imr2
    }
    #[doc = "0xd4 - CPUm wakeup with event mask register"]
    #[inline(always)]
    pub const fn c2emr2(&self) -> &C2EMR2 {
        &self.c2emr2
    }
    #[doc = "0x3d8 - EXTI Hardware configuration registers"]
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
    #[doc = "0x3f0 - Hardware configuration register 1"]
    #[inline(always)]
    pub const fn hwcfgr1(&self) -> &HWCFGR1 {
        &self.hwcfgr1
    }
    #[doc = "0x3f4 - EXTI IP Version register"]
    #[inline(always)]
    pub const fn verr(&self) -> &VERR {
        &self.verr
    }
    #[doc = "0x3f8 - Identification register"]
    #[inline(always)]
    pub const fn ipidr(&self) -> &IPIDR {
        &self.ipidr
    }
    #[doc = "0x3fc - Size ID register"]
    #[inline(always)]
    pub const fn sidr(&self) -> &SIDR {
        &self.sidr
    }
}
#[doc = "RTSR1 (rw) register accessor: rising trigger selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtsr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtsr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtsr1`]
module"]
pub type RTSR1 = crate::Reg<rtsr1::RTSR1rs>;
#[doc = "rising trigger selection register"]
pub mod rtsr1;
#[doc = "FTSR1 (rw) register accessor: falling trigger selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ftsr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ftsr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ftsr1`]
module"]
pub type FTSR1 = crate::Reg<ftsr1::FTSR1rs>;
#[doc = "falling trigger selection register"]
pub mod ftsr1;
#[doc = "SWIER1 (rw) register accessor: software interrupt event register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swier1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swier1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swier1`]
module"]
pub type SWIER1 = crate::Reg<swier1::SWIER1rs>;
#[doc = "software interrupt event register"]
pub mod swier1;
#[doc = "PR1 (rw) register accessor: EXTI pending register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1`]
module"]
pub type PR1 = crate::Reg<pr1::PR1rs>;
#[doc = "EXTI pending register"]
pub mod pr1;
#[doc = "RTSR2 (rw) register accessor: rising trigger selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtsr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtsr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtsr2`]
module"]
pub type RTSR2 = crate::Reg<rtsr2::RTSR2rs>;
#[doc = "rising trigger selection register"]
pub mod rtsr2;
#[doc = "FTSR2 (rw) register accessor: falling trigger selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ftsr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ftsr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ftsr2`]
module"]
pub type FTSR2 = crate::Reg<ftsr2::FTSR2rs>;
#[doc = "falling trigger selection register"]
pub mod ftsr2;
#[doc = "SWIER2 (rw) register accessor: software interrupt event register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swier2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swier2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swier2`]
module"]
pub type SWIER2 = crate::Reg<swier2::SWIER2rs>;
#[doc = "software interrupt event register"]
pub mod swier2;
#[doc = "PR2 (rw) register accessor: pending register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr2`]
module"]
pub type PR2 = crate::Reg<pr2::PR2rs>;
#[doc = "pending register"]
pub mod pr2;
#[doc = "IMR1 (rw) register accessor: CPUm wakeup with interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr1`]
module"]
pub type IMR1 = crate::Reg<imr1::IMR1rs>;
#[doc = "CPUm wakeup with interrupt mask register"]
pub mod imr1;
#[doc = "C2IMR1 (rw) register accessor: CPUm wakeup with interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2imr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2imr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c2imr1`]
module"]
pub type C2IMR1 = crate::Reg<c2imr1::C2IMR1rs>;
#[doc = "CPUm wakeup with interrupt mask register"]
pub mod c2imr1;
#[doc = "EMR1 (rw) register accessor: CPUm wakeup with event mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emr1`]
module"]
pub type EMR1 = crate::Reg<emr1::EMR1rs>;
#[doc = "CPUm wakeup with event mask register"]
pub mod emr1;
#[doc = "C2EMR1 (rw) register accessor: CPUm wakeup with event mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2emr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2emr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c2emr1`]
module"]
pub type C2EMR1 = crate::Reg<c2emr1::C2EMR1rs>;
#[doc = "CPUm wakeup with event mask register"]
pub mod c2emr1;
#[doc = "IMR2 (rw) register accessor: CPUm wakeup with interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr2`]
module"]
pub type IMR2 = crate::Reg<imr2::IMR2rs>;
#[doc = "CPUm wakeup with interrupt mask register"]
pub mod imr2;
#[doc = "C2IMR2 (rw) register accessor: CPUm wakeup with interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2imr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2imr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c2imr2`]
module"]
pub type C2IMR2 = crate::Reg<c2imr2::C2IMR2rs>;
#[doc = "CPUm wakeup with interrupt mask register"]
pub mod c2imr2;
#[doc = "EMR2 (rw) register accessor: CPUm wakeup with event mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emr2`]
module"]
pub type EMR2 = crate::Reg<emr2::EMR2rs>;
#[doc = "CPUm wakeup with event mask register"]
pub mod emr2;
#[doc = "C2EMR2 (rw) register accessor: CPUm wakeup with event mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2emr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2emr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c2emr2`]
module"]
pub type C2EMR2 = crate::Reg<c2emr2::C2EMR2rs>;
#[doc = "CPUm wakeup with event mask register"]
pub mod c2emr2;
#[doc = "HWCFGR5 (r) register accessor: Hardware configuration registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwcfgr5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwcfgr5`]
module"]
pub type HWCFGR5 = crate::Reg<hwcfgr5::HWCFGR5rs>;
#[doc = "Hardware configuration registers"]
pub mod hwcfgr5;
#[doc = "HWCFGR6 (r) register accessor: Hardware configuration registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwcfgr6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwcfgr6`]
module"]
pub type HWCFGR6 = crate::Reg<hwcfgr6::HWCFGR6rs>;
#[doc = "Hardware configuration registers"]
pub mod hwcfgr6;
#[doc = "HWCFGR7 (r) register accessor: EXTI Hardware configuration registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwcfgr7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwcfgr7`]
module"]
pub type HWCFGR7 = crate::Reg<hwcfgr7::HWCFGR7rs>;
#[doc = "EXTI Hardware configuration registers"]
pub mod hwcfgr7;
#[doc = "HWCFGR2 (r) register accessor: Hardware configuration registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwcfgr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwcfgr2`]
module"]
pub type HWCFGR2 = crate::Reg<hwcfgr2::HWCFGR2rs>;
#[doc = "Hardware configuration registers"]
pub mod hwcfgr2;
#[doc = "HWCFGR3 (r) register accessor: Hardware configuration registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwcfgr3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwcfgr3`]
module"]
pub type HWCFGR3 = crate::Reg<hwcfgr3::HWCFGR3rs>;
#[doc = "Hardware configuration registers"]
pub mod hwcfgr3;
#[doc = "HWCFGR4 (r) register accessor: Hardware configuration registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwcfgr4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwcfgr4`]
module"]
pub type HWCFGR4 = crate::Reg<hwcfgr4::HWCFGR4rs>;
#[doc = "Hardware configuration registers"]
pub mod hwcfgr4;
#[doc = "HWCFGR1 (r) register accessor: Hardware configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwcfgr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwcfgr1`]
module"]
pub type HWCFGR1 = crate::Reg<hwcfgr1::HWCFGR1rs>;
#[doc = "Hardware configuration register 1"]
pub mod hwcfgr1;
#[doc = "VERR (r) register accessor: EXTI IP Version register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`verr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@verr`]
module"]
pub type VERR = crate::Reg<verr::VERRrs>;
#[doc = "EXTI IP Version register"]
pub mod verr;
#[doc = "IPIDR (r) register accessor: Identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipidr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipidr`]
module"]
pub type IPIDR = crate::Reg<ipidr::IPIDRrs>;
#[doc = "Identification register"]
pub mod ipidr;
#[doc = "SIDR (r) register accessor: Size ID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sidr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sidr`]
module"]
pub type SIDR = crate::Reg<sidr::SIDRrs>;
#[doc = "Size ID register"]
pub mod sidr;
