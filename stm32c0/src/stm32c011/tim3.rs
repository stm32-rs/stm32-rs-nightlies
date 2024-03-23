#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr1: CR1,
    _reserved1: [u8; 0x02],
    cr2: CR2,
    _reserved2: [u8; 0x02],
    smcr: SMCR,
    dier: DIER,
    _reserved4: [u8; 0x02],
    sr: SR,
    _reserved5: [u8; 0x02],
    egr: EGR,
    _reserved6: [u8; 0x02],
    _reserved_6_ccmr1: [u8; 0x04],
    _reserved_7_ccmr2: [u8; 0x04],
    ccer: CCER,
    _reserved9: [u8; 0x02],
    cnt: CNT,
    psc: PSC,
    _reserved11: [u8; 0x02],
    arr: ARR,
    _reserved12: [u8; 0x04],
    ccr1: CCR1,
    ccr2: CCR2,
    ccr3: CCR3,
    ccr4: CCR4,
    _reserved16: [u8; 0x04],
    dcr: DCR,
    _reserved17: [u8; 0x02],
    dmar: DMAR,
    _reserved18: [u8; 0x12],
    af1: AF1,
    _reserved19: [u8; 0x04],
    tisel: TISEL,
}
impl RegisterBlock {
    #[doc = "0x00 - TIM3 control register 1"]
    #[inline(always)]
    pub const fn cr1(&self) -> &CR1 {
        &self.cr1
    }
    #[doc = "0x04 - TIM3 control register 2"]
    #[inline(always)]
    pub const fn cr2(&self) -> &CR2 {
        &self.cr2
    }
    #[doc = "0x08 - TIM3 slave mode control register"]
    #[inline(always)]
    pub const fn smcr(&self) -> &SMCR {
        &self.smcr
    }
    #[doc = "0x0c - TIM3 DMA/Interrupt enable register"]
    #[inline(always)]
    pub const fn dier(&self) -> &DIER {
        &self.dier
    }
    #[doc = "0x10 - TIM3 status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    #[doc = "0x14 - TIM3 event generation register"]
    #[inline(always)]
    pub const fn egr(&self) -> &EGR {
        &self.egr
    }
    #[doc = "0x18 - TIM3 capture/compare mode register 1 \\[alternate\\]"]
    #[inline(always)]
    pub const fn ccmr1_output(&self) -> &CCMR1_OUTPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(24).cast() }
    }
    #[doc = "0x18 - TIM3 capture/compare mode register 1 \\[alternate\\]"]
    #[inline(always)]
    pub const fn ccmr1_input(&self) -> &CCMR1_INPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(24).cast() }
    }
    #[doc = "0x1c - TIM3 capture/compare mode register 2 \\[alternate\\]"]
    #[inline(always)]
    pub const fn ccmr2_output(&self) -> &CCMR2_OUTPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(28).cast() }
    }
    #[doc = "0x1c - TIM3 capture/compare mode register 2 \\[alternate\\]"]
    #[inline(always)]
    pub const fn ccmr2_input(&self) -> &CCMR2_INPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(28).cast() }
    }
    #[doc = "0x20 - TIM3 capture/compare enable register"]
    #[inline(always)]
    pub const fn ccer(&self) -> &CCER {
        &self.ccer
    }
    #[doc = "0x24 - TIM3 counter \\[alternate\\]"]
    #[inline(always)]
    pub const fn cnt(&self) -> &CNT {
        &self.cnt
    }
    #[doc = "0x28 - TIM3 prescaler"]
    #[inline(always)]
    pub const fn psc(&self) -> &PSC {
        &self.psc
    }
    #[doc = "0x2c - TIM3 auto-reload register"]
    #[inline(always)]
    pub const fn arr(&self) -> &ARR {
        &self.arr
    }
    #[doc = "0x34 - TIM3 capture/compare register 1"]
    #[inline(always)]
    pub const fn ccr1(&self) -> &CCR1 {
        &self.ccr1
    }
    #[doc = "0x38 - TIM3 capture/compare register 2"]
    #[inline(always)]
    pub const fn ccr2(&self) -> &CCR2 {
        &self.ccr2
    }
    #[doc = "0x3c - TIM3 capture/compare register 3"]
    #[inline(always)]
    pub const fn ccr3(&self) -> &CCR3 {
        &self.ccr3
    }
    #[doc = "0x40 - TIM3 capture/compare register 4"]
    #[inline(always)]
    pub const fn ccr4(&self) -> &CCR4 {
        &self.ccr4
    }
    #[doc = "0x48 - TIM3 DMA control register"]
    #[inline(always)]
    pub const fn dcr(&self) -> &DCR {
        &self.dcr
    }
    #[doc = "0x4c - TIM3 DMA address for full transfer"]
    #[inline(always)]
    pub const fn dmar(&self) -> &DMAR {
        &self.dmar
    }
    #[doc = "0x60 - TIM3 alternate function option register 1"]
    #[inline(always)]
    pub const fn af1(&self) -> &AF1 {
        &self.af1
    }
    #[doc = "0x68 - TIM3 timer input selection register"]
    #[inline(always)]
    pub const fn tisel(&self) -> &TISEL {
        &self.tisel
    }
}
#[doc = "CR1 (rw) register accessor: TIM3 control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`]
module"]
pub type CR1 = crate::Reg<cr1::CR1rs>;
#[doc = "TIM3 control register 1"]
pub mod cr1;
#[doc = "CR2 (rw) register accessor: TIM3 control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr2`]
module"]
pub type CR2 = crate::Reg<cr2::CR2rs>;
#[doc = "TIM3 control register 2"]
pub mod cr2;
#[doc = "SMCR (rw) register accessor: TIM3 slave mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smcr`]
module"]
pub type SMCR = crate::Reg<smcr::SMCRrs>;
#[doc = "TIM3 slave mode control register"]
pub mod smcr;
#[doc = "DIER (rw) register accessor: TIM3 DMA/Interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dier`]
module"]
pub type DIER = crate::Reg<dier::DIERrs>;
#[doc = "TIM3 DMA/Interrupt enable register"]
pub mod dier;
#[doc = "SR (rw) register accessor: TIM3 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
pub type SR = crate::Reg<sr::SRrs>;
#[doc = "TIM3 status register"]
pub mod sr;
#[doc = "EGR (w) register accessor: TIM3 event generation register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`egr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@egr`]
module"]
pub type EGR = crate::Reg<egr::EGRrs>;
#[doc = "TIM3 event generation register"]
pub mod egr;
#[doc = "CCMR1_input (rw) register accessor: TIM3 capture/compare mode register 1 \\[alternate\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccmr1_input::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccmr1_input::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccmr1_input`]
module"]
#[doc(alias = "CCMR1_input")]
pub type CCMR1_INPUT = crate::Reg<ccmr1_input::CCMR1_INPUTrs>;
#[doc = "TIM3 capture/compare mode register 1 \\[alternate\\]"]
pub mod ccmr1_input;
#[doc = "CCMR1_output (rw) register accessor: TIM3 capture/compare mode register 1 \\[alternate\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccmr1_output::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccmr1_output::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccmr1_output`]
module"]
#[doc(alias = "CCMR1_output")]
pub type CCMR1_OUTPUT = crate::Reg<ccmr1_output::CCMR1_OUTPUTrs>;
#[doc = "TIM3 capture/compare mode register 1 \\[alternate\\]"]
pub mod ccmr1_output;
#[doc = "CCMR2_input (rw) register accessor: TIM3 capture/compare mode register 2 \\[alternate\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccmr2_input::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccmr2_input::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccmr2_input`]
module"]
#[doc(alias = "CCMR2_input")]
pub type CCMR2_INPUT = crate::Reg<ccmr2_input::CCMR2_INPUTrs>;
#[doc = "TIM3 capture/compare mode register 2 \\[alternate\\]"]
pub mod ccmr2_input;
#[doc = "CCMR2_output (rw) register accessor: TIM3 capture/compare mode register 2 \\[alternate\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccmr2_output::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccmr2_output::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccmr2_output`]
module"]
#[doc(alias = "CCMR2_output")]
pub type CCMR2_OUTPUT = crate::Reg<ccmr2_output::CCMR2_OUTPUTrs>;
#[doc = "TIM3 capture/compare mode register 2 \\[alternate\\]"]
pub mod ccmr2_output;
#[doc = "CCER (rw) register accessor: TIM3 capture/compare enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccer`]
module"]
pub type CCER = crate::Reg<ccer::CCERrs>;
#[doc = "TIM3 capture/compare enable register"]
pub mod ccer;
#[doc = "CNT (rw) register accessor: TIM3 counter \\[alternate\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnt`]
module"]
pub type CNT = crate::Reg<cnt::CNTrs>;
#[doc = "TIM3 counter \\[alternate\\]"]
pub mod cnt;
#[doc = "PSC (rw) register accessor: TIM3 prescaler\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psc`]
module"]
pub type PSC = crate::Reg<psc::PSCrs>;
#[doc = "TIM3 prescaler"]
pub mod psc;
#[doc = "ARR (rw) register accessor: TIM3 auto-reload register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arr`]
module"]
pub type ARR = crate::Reg<arr::ARRrs>;
#[doc = "TIM3 auto-reload register"]
pub mod arr;
#[doc = "CCR1 (rw) register accessor: TIM3 capture/compare register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr1`]
module"]
pub type CCR1 = crate::Reg<ccr1::CCR1rs>;
#[doc = "TIM3 capture/compare register 1"]
pub mod ccr1;
#[doc = "CCR2 (rw) register accessor: TIM3 capture/compare register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr2`]
module"]
pub type CCR2 = crate::Reg<ccr2::CCR2rs>;
#[doc = "TIM3 capture/compare register 2"]
pub mod ccr2;
#[doc = "CCR3 (rw) register accessor: TIM3 capture/compare register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr3`]
module"]
pub type CCR3 = crate::Reg<ccr3::CCR3rs>;
#[doc = "TIM3 capture/compare register 3"]
pub mod ccr3;
#[doc = "CCR4 (rw) register accessor: TIM3 capture/compare register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr4`]
module"]
pub type CCR4 = crate::Reg<ccr4::CCR4rs>;
#[doc = "TIM3 capture/compare register 4"]
pub mod ccr4;
#[doc = "DCR (rw) register accessor: TIM3 DMA control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcr`]
module"]
pub type DCR = crate::Reg<dcr::DCRrs>;
#[doc = "TIM3 DMA control register"]
pub mod dcr;
#[doc = "DMAR (rw) register accessor: TIM3 DMA address for full transfer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmar`]
module"]
pub type DMAR = crate::Reg<dmar::DMARrs>;
#[doc = "TIM3 DMA address for full transfer"]
pub mod dmar;
#[doc = "AF1 (rw) register accessor: TIM3 alternate function option register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`af1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`af1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@af1`]
module"]
pub type AF1 = crate::Reg<af1::AF1rs>;
#[doc = "TIM3 alternate function option register 1"]
pub mod af1;
#[doc = "TISEL (rw) register accessor: TIM3 timer input selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tisel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tisel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tisel`]
module"]
pub type TISEL = crate::Reg<tisel::TISELrs>;
#[doc = "TIM3 timer input selection register"]
pub mod tisel;
