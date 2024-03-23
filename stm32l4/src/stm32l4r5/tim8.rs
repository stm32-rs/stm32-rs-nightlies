#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr1: CR1,
    cr2: CR2,
    smcr: SMCR,
    dier: DIER,
    sr: SR,
    egr: EGR,
    _reserved_6_ccmr1: [u8; 0x04],
    _reserved_7_ccmr2: [u8; 0x04],
    ccer: CCER,
    cnt: CNT,
    psc: PSC,
    arr: ARR,
    rcr: RCR,
    ccr: [CCR; 4],
    bdtr: BDTR,
    dcr: DCR,
    dmar: DMAR,
    or1: OR1,
    ccmr3_output: CCMR3_OUTPUT,
    ccr5: CCR5,
    ccr6: CCR6,
    or2: OR2,
    or3: OR3,
}
impl RegisterBlock {
    #[doc = "0x00 - control register 1"]
    #[inline(always)]
    pub const fn cr1(&self) -> &CR1 {
        &self.cr1
    }
    #[doc = "0x04 - control register 2"]
    #[inline(always)]
    pub const fn cr2(&self) -> &CR2 {
        &self.cr2
    }
    #[doc = "0x08 - slave mode control register"]
    #[inline(always)]
    pub const fn smcr(&self) -> &SMCR {
        &self.smcr
    }
    #[doc = "0x0c - DMA/Interrupt enable register"]
    #[inline(always)]
    pub const fn dier(&self) -> &DIER {
        &self.dier
    }
    #[doc = "0x10 - status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    #[doc = "0x14 - event generation register"]
    #[inline(always)]
    pub const fn egr(&self) -> &EGR {
        &self.egr
    }
    #[doc = "0x18 - capture/compare mode register 1 (input mode)"]
    #[inline(always)]
    pub const fn ccmr1_input(&self) -> &CCMR1_INPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(24).cast() }
    }
    #[doc = "0x18 - capture/compare mode register 1 (output mode)"]
    #[inline(always)]
    pub const fn ccmr1_output(&self) -> &CCMR1_OUTPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(24).cast() }
    }
    #[doc = "0x1c - capture/compare mode register 2 (input mode)"]
    #[inline(always)]
    pub const fn ccmr2_input(&self) -> &CCMR2_INPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(28).cast() }
    }
    #[doc = "0x1c - capture/compare mode register 2 (output mode)"]
    #[inline(always)]
    pub const fn ccmr2_output(&self) -> &CCMR2_OUTPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(28).cast() }
    }
    #[doc = "0x20 - capture/compare enable register"]
    #[inline(always)]
    pub const fn ccer(&self) -> &CCER {
        &self.ccer
    }
    #[doc = "0x24 - counter"]
    #[inline(always)]
    pub const fn cnt(&self) -> &CNT {
        &self.cnt
    }
    #[doc = "0x28 - prescaler"]
    #[inline(always)]
    pub const fn psc(&self) -> &PSC {
        &self.psc
    }
    #[doc = "0x2c - auto-reload register"]
    #[inline(always)]
    pub const fn arr(&self) -> &ARR {
        &self.arr
    }
    #[doc = "0x30 - repetition counter register"]
    #[inline(always)]
    pub const fn rcr(&self) -> &RCR {
        &self.rcr
    }
    #[doc = "0x34..0x44 - capture/compare register"]
    #[inline(always)]
    pub const fn ccr(&self, n: usize) -> &CCR {
        &self.ccr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x34..0x44 - capture/compare register"]
    #[inline(always)]
    pub fn ccr_iter(&self) -> impl Iterator<Item = &CCR> {
        self.ccr.iter()
    }
    #[doc = "0x34 - capture/compare register"]
    #[inline(always)]
    pub const fn ccr1(&self) -> &CCR {
        self.ccr(0)
    }
    #[doc = "0x38 - capture/compare register"]
    #[inline(always)]
    pub const fn ccr2(&self) -> &CCR {
        self.ccr(1)
    }
    #[doc = "0x3c - capture/compare register"]
    #[inline(always)]
    pub const fn ccr3(&self) -> &CCR {
        self.ccr(2)
    }
    #[doc = "0x40 - capture/compare register"]
    #[inline(always)]
    pub const fn ccr4(&self) -> &CCR {
        self.ccr(3)
    }
    #[doc = "0x44 - break and dead-time register"]
    #[inline(always)]
    pub const fn bdtr(&self) -> &BDTR {
        &self.bdtr
    }
    #[doc = "0x48 - DMA control register"]
    #[inline(always)]
    pub const fn dcr(&self) -> &DCR {
        &self.dcr
    }
    #[doc = "0x4c - DMA address for full transfer"]
    #[inline(always)]
    pub const fn dmar(&self) -> &DMAR {
        &self.dmar
    }
    #[doc = "0x50 - TIM8 option register 1"]
    #[inline(always)]
    pub const fn or1(&self) -> &OR1 {
        &self.or1
    }
    #[doc = "0x54 - capture/compare mode register 2 (output mode)"]
    #[inline(always)]
    pub const fn ccmr3_output(&self) -> &CCMR3_OUTPUT {
        &self.ccmr3_output
    }
    #[doc = "0x58 - capture/compare register"]
    #[inline(always)]
    pub const fn ccr5(&self) -> &CCR5 {
        &self.ccr5
    }
    #[doc = "0x5c - capture/compare register"]
    #[inline(always)]
    pub const fn ccr6(&self) -> &CCR6 {
        &self.ccr6
    }
    #[doc = "0x60 - DMA address for full transfer"]
    #[inline(always)]
    pub const fn or2(&self) -> &OR2 {
        &self.or2
    }
    #[doc = "0x64 - DMA address for full transfer"]
    #[inline(always)]
    pub const fn or3(&self) -> &OR3 {
        &self.or3
    }
}
#[doc = "CR1 (rw) register accessor: control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`]
module"]
pub type CR1 = crate::Reg<cr1::CR1rs>;
#[doc = "control register 1"]
pub mod cr1;
#[doc = "CR2 (rw) register accessor: control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr2`]
module"]
pub type CR2 = crate::Reg<cr2::CR2rs>;
#[doc = "control register 2"]
pub mod cr2;
#[doc = "SMCR (rw) register accessor: slave mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smcr`]
module"]
pub type SMCR = crate::Reg<smcr::SMCRrs>;
#[doc = "slave mode control register"]
pub mod smcr;
#[doc = "DIER (rw) register accessor: DMA/Interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dier`]
module"]
pub type DIER = crate::Reg<dier::DIERrs>;
#[doc = "DMA/Interrupt enable register"]
pub mod dier;
#[doc = "SR (rw) register accessor: status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
pub type SR = crate::Reg<sr::SRrs>;
#[doc = "status register"]
pub mod sr;
#[doc = "EGR (w) register accessor: event generation register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`egr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@egr`]
module"]
pub type EGR = crate::Reg<egr::EGRrs>;
#[doc = "event generation register"]
pub mod egr;
#[doc = "CCMR1_Output (rw) register accessor: capture/compare mode register 1 (output mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccmr1_output::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccmr1_output::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccmr1_output`]
module"]
#[doc(alias = "CCMR1_Output")]
pub type CCMR1_OUTPUT = crate::Reg<ccmr1_output::CCMR1_OUTPUTrs>;
#[doc = "capture/compare mode register 1 (output mode)"]
pub mod ccmr1_output;
#[doc = "CCMR1_Input (rw) register accessor: capture/compare mode register 1 (input mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccmr1_input::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccmr1_input::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccmr1_input`]
module"]
#[doc(alias = "CCMR1_Input")]
pub type CCMR1_INPUT = crate::Reg<ccmr1_input::CCMR1_INPUTrs>;
#[doc = "capture/compare mode register 1 (input mode)"]
pub mod ccmr1_input;
#[doc = "CCMR2_Output (rw) register accessor: capture/compare mode register 2 (output mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccmr2_output::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccmr2_output::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccmr2_output`]
module"]
#[doc(alias = "CCMR2_Output")]
pub type CCMR2_OUTPUT = crate::Reg<ccmr2_output::CCMR2_OUTPUTrs>;
#[doc = "capture/compare mode register 2 (output mode)"]
pub mod ccmr2_output;
#[doc = "CCMR2_Input (rw) register accessor: capture/compare mode register 2 (input mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccmr2_input::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccmr2_input::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccmr2_input`]
module"]
#[doc(alias = "CCMR2_Input")]
pub type CCMR2_INPUT = crate::Reg<ccmr2_input::CCMR2_INPUTrs>;
#[doc = "capture/compare mode register 2 (input mode)"]
pub mod ccmr2_input;
#[doc = "CCER (rw) register accessor: capture/compare enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccer`]
module"]
pub type CCER = crate::Reg<ccer::CCERrs>;
#[doc = "capture/compare enable register"]
pub mod ccer;
#[doc = "CNT (rw) register accessor: counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnt`]
module"]
pub type CNT = crate::Reg<cnt::CNTrs>;
#[doc = "counter"]
pub mod cnt;
#[doc = "PSC (rw) register accessor: prescaler\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psc`]
module"]
pub type PSC = crate::Reg<psc::PSCrs>;
#[doc = "prescaler"]
pub mod psc;
#[doc = "ARR (rw) register accessor: auto-reload register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arr`]
module"]
pub type ARR = crate::Reg<arr::ARRrs>;
#[doc = "auto-reload register"]
pub mod arr;
#[doc = "RCR (rw) register accessor: repetition counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcr`]
module"]
pub type RCR = crate::Reg<rcr::RCRrs>;
#[doc = "repetition counter register"]
pub mod rcr;
#[doc = "CCR (rw) register accessor: capture/compare register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr`]
module"]
pub type CCR = crate::Reg<ccr::CCRrs>;
#[doc = "capture/compare register"]
pub mod ccr;
#[doc = "BDTR (rw) register accessor: break and dead-time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bdtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bdtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bdtr`]
module"]
pub type BDTR = crate::Reg<bdtr::BDTRrs>;
#[doc = "break and dead-time register"]
pub mod bdtr;
#[doc = "DCR (rw) register accessor: DMA control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcr`]
module"]
pub type DCR = crate::Reg<dcr::DCRrs>;
#[doc = "DMA control register"]
pub mod dcr;
#[doc = "DMAR (rw) register accessor: DMA address for full transfer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmar`]
module"]
pub type DMAR = crate::Reg<dmar::DMARrs>;
#[doc = "DMA address for full transfer"]
pub mod dmar;
#[doc = "OR1 (rw) register accessor: TIM8 option register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`or1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`or1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@or1`]
module"]
pub type OR1 = crate::Reg<or1::OR1rs>;
#[doc = "TIM8 option register 1"]
pub mod or1;
#[doc = "CCMR3_Output (rw) register accessor: capture/compare mode register 2 (output mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccmr3_output::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccmr3_output::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccmr3_output`]
module"]
#[doc(alias = "CCMR3_Output")]
pub type CCMR3_OUTPUT = crate::Reg<ccmr3_output::CCMR3_OUTPUTrs>;
#[doc = "capture/compare mode register 2 (output mode)"]
pub mod ccmr3_output;
#[doc = "CCR5 (rw) register accessor: capture/compare register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr5`]
module"]
pub type CCR5 = crate::Reg<ccr5::CCR5rs>;
#[doc = "capture/compare register"]
pub mod ccr5;
#[doc = "CCR6 (rw) register accessor: capture/compare register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr6`]
module"]
pub type CCR6 = crate::Reg<ccr6::CCR6rs>;
#[doc = "capture/compare register"]
pub mod ccr6;
#[doc = "OR2 (rw) register accessor: DMA address for full transfer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`or2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`or2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@or2`]
module"]
pub type OR2 = crate::Reg<or2::OR2rs>;
#[doc = "DMA address for full transfer"]
pub mod or2;
#[doc = "OR3 (rw) register accessor: DMA address for full transfer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`or3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`or3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@or3`]
module"]
pub type OR3 = crate::Reg<or3::OR3rs>;
#[doc = "DMA address for full transfer"]
pub mod or3;
