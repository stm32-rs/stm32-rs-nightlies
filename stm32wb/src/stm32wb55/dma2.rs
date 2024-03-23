#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    isr: ISR,
    ifcr: IFCR,
    ccr1: CCR1,
    cndtr1: CNDTR1,
    cpar1: CPAR1,
    cmar1: CMAR1,
    _reserved6: [u8; 0x04],
    ccr2: CCR2,
    cndtr2: CNDTR2,
    cpar2: CPAR2,
    cmar2: CMAR2,
    _reserved10: [u8; 0x04],
    ccr3: CCR3,
    cndtr3: CNDTR3,
    cpar3: CPAR3,
    cmar3: CMAR3,
    _reserved14: [u8; 0x04],
    ccr4: CCR4,
    cndtr4: CNDTR4,
    cpar4: CPAR4,
    cmar4: CMAR4,
    _reserved18: [u8; 0x04],
    ccr5: CCR5,
    cndtr5: CNDTR5,
    cpar5: CPAR5,
    cmar5: CMAR5,
    _reserved22: [u8; 0x04],
    ccr6: CCR6,
    cndtr6: CNDTR6,
    cpar6: CPAR6,
    cmar6: CMAR6,
    _reserved26: [u8; 0x04],
    ccr7: CCR7,
    cndtr7: CNDTR7,
    cpar7: CPAR7,
    cmar7: CMAR7,
    _reserved30: [u8; 0x18],
    cselr: CSELR,
}
impl RegisterBlock {
    #[doc = "0x00 - interrupt status register"]
    #[inline(always)]
    pub const fn isr(&self) -> &ISR {
        &self.isr
    }
    #[doc = "0x04 - interrupt flag clear register"]
    #[inline(always)]
    pub const fn ifcr(&self) -> &IFCR {
        &self.ifcr
    }
    #[doc = "0x08 - channel x configuration register"]
    #[inline(always)]
    pub const fn ccr1(&self) -> &CCR1 {
        &self.ccr1
    }
    #[doc = "0x0c - channel x number of data register"]
    #[inline(always)]
    pub const fn cndtr1(&self) -> &CNDTR1 {
        &self.cndtr1
    }
    #[doc = "0x10 - channel x peripheral address register"]
    #[inline(always)]
    pub const fn cpar1(&self) -> &CPAR1 {
        &self.cpar1
    }
    #[doc = "0x14 - channel x memory address register"]
    #[inline(always)]
    pub const fn cmar1(&self) -> &CMAR1 {
        &self.cmar1
    }
    #[doc = "0x1c - channel x configuration register"]
    #[inline(always)]
    pub const fn ccr2(&self) -> &CCR2 {
        &self.ccr2
    }
    #[doc = "0x20 - channel x number of data register"]
    #[inline(always)]
    pub const fn cndtr2(&self) -> &CNDTR2 {
        &self.cndtr2
    }
    #[doc = "0x24 - channel x peripheral address register"]
    #[inline(always)]
    pub const fn cpar2(&self) -> &CPAR2 {
        &self.cpar2
    }
    #[doc = "0x28 - channel x memory address register"]
    #[inline(always)]
    pub const fn cmar2(&self) -> &CMAR2 {
        &self.cmar2
    }
    #[doc = "0x30 - channel x configuration register"]
    #[inline(always)]
    pub const fn ccr3(&self) -> &CCR3 {
        &self.ccr3
    }
    #[doc = "0x34 - channel x number of data register"]
    #[inline(always)]
    pub const fn cndtr3(&self) -> &CNDTR3 {
        &self.cndtr3
    }
    #[doc = "0x38 - channel x peripheral address register"]
    #[inline(always)]
    pub const fn cpar3(&self) -> &CPAR3 {
        &self.cpar3
    }
    #[doc = "0x3c - channel x memory address register"]
    #[inline(always)]
    pub const fn cmar3(&self) -> &CMAR3 {
        &self.cmar3
    }
    #[doc = "0x44 - channel x configuration register"]
    #[inline(always)]
    pub const fn ccr4(&self) -> &CCR4 {
        &self.ccr4
    }
    #[doc = "0x48 - channel x number of data register"]
    #[inline(always)]
    pub const fn cndtr4(&self) -> &CNDTR4 {
        &self.cndtr4
    }
    #[doc = "0x4c - channel x peripheral address register"]
    #[inline(always)]
    pub const fn cpar4(&self) -> &CPAR4 {
        &self.cpar4
    }
    #[doc = "0x50 - channel x memory address register"]
    #[inline(always)]
    pub const fn cmar4(&self) -> &CMAR4 {
        &self.cmar4
    }
    #[doc = "0x58 - channel x configuration register"]
    #[inline(always)]
    pub const fn ccr5(&self) -> &CCR5 {
        &self.ccr5
    }
    #[doc = "0x5c - channel x number of data register"]
    #[inline(always)]
    pub const fn cndtr5(&self) -> &CNDTR5 {
        &self.cndtr5
    }
    #[doc = "0x60 - channel x peripheral address register"]
    #[inline(always)]
    pub const fn cpar5(&self) -> &CPAR5 {
        &self.cpar5
    }
    #[doc = "0x64 - channel x memory address register"]
    #[inline(always)]
    pub const fn cmar5(&self) -> &CMAR5 {
        &self.cmar5
    }
    #[doc = "0x6c - channel x configuration register"]
    #[inline(always)]
    pub const fn ccr6(&self) -> &CCR6 {
        &self.ccr6
    }
    #[doc = "0x70 - channel x number of data register"]
    #[inline(always)]
    pub const fn cndtr6(&self) -> &CNDTR6 {
        &self.cndtr6
    }
    #[doc = "0x74 - channel x peripheral address register"]
    #[inline(always)]
    pub const fn cpar6(&self) -> &CPAR6 {
        &self.cpar6
    }
    #[doc = "0x78 - channel x memory address register"]
    #[inline(always)]
    pub const fn cmar6(&self) -> &CMAR6 {
        &self.cmar6
    }
    #[doc = "0x80 - channel x configuration register"]
    #[inline(always)]
    pub const fn ccr7(&self) -> &CCR7 {
        &self.ccr7
    }
    #[doc = "0x84 - channel x number of data register"]
    #[inline(always)]
    pub const fn cndtr7(&self) -> &CNDTR7 {
        &self.cndtr7
    }
    #[doc = "0x88 - channel x peripheral address register"]
    #[inline(always)]
    pub const fn cpar7(&self) -> &CPAR7 {
        &self.cpar7
    }
    #[doc = "0x8c - channel x memory address register"]
    #[inline(always)]
    pub const fn cmar7(&self) -> &CMAR7 {
        &self.cmar7
    }
    #[doc = "0xa8 - channel selection register"]
    #[inline(always)]
    pub const fn cselr(&self) -> &CSELR {
        &self.cselr
    }
}
#[doc = "ISR (r) register accessor: interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`]
module"]
pub type ISR = crate::Reg<isr::ISRrs>;
#[doc = "interrupt status register"]
pub mod isr;
#[doc = "IFCR (w) register accessor: interrupt flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifcr`]
module"]
pub type IFCR = crate::Reg<ifcr::IFCRrs>;
#[doc = "interrupt flag clear register"]
pub mod ifcr;
#[doc = "CCR1 (rw) register accessor: channel x configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr1`]
module"]
pub type CCR1 = crate::Reg<ccr1::CCR1rs>;
#[doc = "channel x configuration register"]
pub mod ccr1;
#[doc = "CNDTR1 (rw) register accessor: channel x number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cndtr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cndtr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cndtr1`]
module"]
pub type CNDTR1 = crate::Reg<cndtr1::CNDTR1rs>;
#[doc = "channel x number of data register"]
pub mod cndtr1;
#[doc = "CPAR1 (rw) register accessor: channel x peripheral address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpar1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpar1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpar1`]
module"]
pub type CPAR1 = crate::Reg<cpar1::CPAR1rs>;
#[doc = "channel x peripheral address register"]
pub mod cpar1;
#[doc = "CMAR1 (rw) register accessor: channel x memory address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmar1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmar1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmar1`]
module"]
pub type CMAR1 = crate::Reg<cmar1::CMAR1rs>;
#[doc = "channel x memory address register"]
pub mod cmar1;
#[doc = "CCR2 (rw) register accessor: channel x configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr2`]
module"]
pub type CCR2 = crate::Reg<ccr2::CCR2rs>;
#[doc = "channel x configuration register"]
pub mod ccr2;
#[doc = "CNDTR2 (rw) register accessor: channel x number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cndtr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cndtr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cndtr2`]
module"]
pub type CNDTR2 = crate::Reg<cndtr2::CNDTR2rs>;
#[doc = "channel x number of data register"]
pub mod cndtr2;
#[doc = "CPAR2 (rw) register accessor: channel x peripheral address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpar2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpar2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpar2`]
module"]
pub type CPAR2 = crate::Reg<cpar2::CPAR2rs>;
#[doc = "channel x peripheral address register"]
pub mod cpar2;
#[doc = "CMAR2 (rw) register accessor: channel x memory address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmar2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmar2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmar2`]
module"]
pub type CMAR2 = crate::Reg<cmar2::CMAR2rs>;
#[doc = "channel x memory address register"]
pub mod cmar2;
#[doc = "CCR3 (rw) register accessor: channel x configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr3`]
module"]
pub type CCR3 = crate::Reg<ccr3::CCR3rs>;
#[doc = "channel x configuration register"]
pub mod ccr3;
#[doc = "CNDTR3 (rw) register accessor: channel x number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cndtr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cndtr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cndtr3`]
module"]
pub type CNDTR3 = crate::Reg<cndtr3::CNDTR3rs>;
#[doc = "channel x number of data register"]
pub mod cndtr3;
#[doc = "CPAR3 (rw) register accessor: channel x peripheral address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpar3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpar3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpar3`]
module"]
pub type CPAR3 = crate::Reg<cpar3::CPAR3rs>;
#[doc = "channel x peripheral address register"]
pub mod cpar3;
#[doc = "CMAR3 (rw) register accessor: channel x memory address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmar3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmar3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmar3`]
module"]
pub type CMAR3 = crate::Reg<cmar3::CMAR3rs>;
#[doc = "channel x memory address register"]
pub mod cmar3;
#[doc = "CCR4 (rw) register accessor: channel x configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr4`]
module"]
pub type CCR4 = crate::Reg<ccr4::CCR4rs>;
#[doc = "channel x configuration register"]
pub mod ccr4;
#[doc = "CNDTR4 (rw) register accessor: channel x number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cndtr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cndtr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cndtr4`]
module"]
pub type CNDTR4 = crate::Reg<cndtr4::CNDTR4rs>;
#[doc = "channel x number of data register"]
pub mod cndtr4;
#[doc = "CPAR4 (rw) register accessor: channel x peripheral address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpar4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpar4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpar4`]
module"]
pub type CPAR4 = crate::Reg<cpar4::CPAR4rs>;
#[doc = "channel x peripheral address register"]
pub mod cpar4;
#[doc = "CMAR4 (rw) register accessor: channel x memory address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmar4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmar4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmar4`]
module"]
pub type CMAR4 = crate::Reg<cmar4::CMAR4rs>;
#[doc = "channel x memory address register"]
pub mod cmar4;
#[doc = "CCR5 (rw) register accessor: channel x configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr5`]
module"]
pub type CCR5 = crate::Reg<ccr5::CCR5rs>;
#[doc = "channel x configuration register"]
pub mod ccr5;
#[doc = "CNDTR5 (rw) register accessor: channel x number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cndtr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cndtr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cndtr5`]
module"]
pub type CNDTR5 = crate::Reg<cndtr5::CNDTR5rs>;
#[doc = "channel x number of data register"]
pub mod cndtr5;
#[doc = "CPAR5 (rw) register accessor: channel x peripheral address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpar5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpar5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpar5`]
module"]
pub type CPAR5 = crate::Reg<cpar5::CPAR5rs>;
#[doc = "channel x peripheral address register"]
pub mod cpar5;
#[doc = "CMAR5 (rw) register accessor: channel x memory address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmar5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmar5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmar5`]
module"]
pub type CMAR5 = crate::Reg<cmar5::CMAR5rs>;
#[doc = "channel x memory address register"]
pub mod cmar5;
#[doc = "CCR6 (rw) register accessor: channel x configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr6`]
module"]
pub type CCR6 = crate::Reg<ccr6::CCR6rs>;
#[doc = "channel x configuration register"]
pub mod ccr6;
#[doc = "CNDTR6 (rw) register accessor: channel x number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cndtr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cndtr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cndtr6`]
module"]
pub type CNDTR6 = crate::Reg<cndtr6::CNDTR6rs>;
#[doc = "channel x number of data register"]
pub mod cndtr6;
#[doc = "CPAR6 (rw) register accessor: channel x peripheral address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpar6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpar6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpar6`]
module"]
pub type CPAR6 = crate::Reg<cpar6::CPAR6rs>;
#[doc = "channel x peripheral address register"]
pub mod cpar6;
#[doc = "CMAR6 (rw) register accessor: channel x memory address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmar6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmar6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmar6`]
module"]
pub type CMAR6 = crate::Reg<cmar6::CMAR6rs>;
#[doc = "channel x memory address register"]
pub mod cmar6;
#[doc = "CCR7 (rw) register accessor: channel x configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr7`]
module"]
pub type CCR7 = crate::Reg<ccr7::CCR7rs>;
#[doc = "channel x configuration register"]
pub mod ccr7;
#[doc = "CNDTR7 (rw) register accessor: channel x number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cndtr7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cndtr7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cndtr7`]
module"]
pub type CNDTR7 = crate::Reg<cndtr7::CNDTR7rs>;
#[doc = "channel x number of data register"]
pub mod cndtr7;
#[doc = "CPAR7 (rw) register accessor: channel x peripheral address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpar7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpar7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpar7`]
module"]
pub type CPAR7 = crate::Reg<cpar7::CPAR7rs>;
#[doc = "channel x peripheral address register"]
pub mod cpar7;
#[doc = "CMAR7 (rw) register accessor: channel x memory address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmar7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmar7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmar7`]
module"]
pub type CMAR7 = crate::Reg<cmar7::CMAR7rs>;
#[doc = "channel x memory address register"]
pub mod cmar7;
#[doc = "CSELR (rw) register accessor: channel selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cselr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cselr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cselr`]
module"]
pub type CSELR = crate::Reg<cselr::CSELRrs>;
#[doc = "channel selection register"]
pub mod cselr;
