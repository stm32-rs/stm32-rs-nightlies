#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr1: CR1,
    _reserved1: [u8; 0x02],
    cr2: CR2,
    _reserved2: [u8; 0x06],
    dier: DIER,
    _reserved3: [u8; 0x02],
    sr: SR,
    _reserved4: [u8; 0x02],
    egr: EGR,
    _reserved5: [u8; 0x02],
    _reserved_5_ccmr1: [u8; 0x04],
    _reserved6: [u8; 0x04],
    ccer: CCER,
    _reserved7: [u8; 0x02],
    cnt: CNT,
    psc: PSC,
    _reserved9: [u8; 0x02],
    arr: ARR,
    rcr: RCR,
    _reserved11: [u8; 0x02],
    ccr1: CCR1,
    _reserved12: [u8; 0x0c],
    bdtr: BDTR,
    _reserved13: [u8; 0x0c],
    dtr2: DTR2,
    _reserved14: [u8; 0x04],
    tisel: TISEL,
    af1: AF1,
    af2: AF2,
    _reserved17: [u8; 0x0374],
    dcr: DCR,
    dmar: DMAR,
}
impl RegisterBlock {
    #[doc = "0x00 - TIM17 control register 1"]
    #[inline(always)]
    pub const fn cr1(&self) -> &CR1 {
        &self.cr1
    }
    #[doc = "0x04 - TIM17 control register 2"]
    #[inline(always)]
    pub const fn cr2(&self) -> &CR2 {
        &self.cr2
    }
    #[doc = "0x0c - TIM17 DMA/interrupt enable register"]
    #[inline(always)]
    pub const fn dier(&self) -> &DIER {
        &self.dier
    }
    #[doc = "0x10 - TIM17 status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    #[doc = "0x14 - TIM17 event generation register"]
    #[inline(always)]
    pub const fn egr(&self) -> &EGR {
        &self.egr
    }
    #[doc = "0x18 - TIM17 capture/compare mode register 1 \\[alternate\\]"]
    #[inline(always)]
    pub const fn ccmr1_output(&self) -> &CCMR1_OUTPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(24).cast() }
    }
    #[doc = "0x18 - TIM17 capture/compare mode register 1 \\[alternate\\]"]
    #[inline(always)]
    pub const fn ccmr1_input(&self) -> &CCMR1_INPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(24).cast() }
    }
    #[doc = "0x20 - TIM17 capture/compare enable register"]
    #[inline(always)]
    pub const fn ccer(&self) -> &CCER {
        &self.ccer
    }
    #[doc = "0x24 - TIM17 counter"]
    #[inline(always)]
    pub const fn cnt(&self) -> &CNT {
        &self.cnt
    }
    #[doc = "0x28 - TIM17 prescaler"]
    #[inline(always)]
    pub const fn psc(&self) -> &PSC {
        &self.psc
    }
    #[doc = "0x2c - TIM17 auto-reload register"]
    #[inline(always)]
    pub const fn arr(&self) -> &ARR {
        &self.arr
    }
    #[doc = "0x30 - TIM17 repetition counter register"]
    #[inline(always)]
    pub const fn rcr(&self) -> &RCR {
        &self.rcr
    }
    #[doc = "0x34 - TIM17 capture/compare register 1"]
    #[inline(always)]
    pub const fn ccr1(&self) -> &CCR1 {
        &self.ccr1
    }
    #[doc = "0x44 - TIM17 break and dead-time register"]
    #[inline(always)]
    pub const fn bdtr(&self) -> &BDTR {
        &self.bdtr
    }
    #[doc = "0x54 - TIM17 timer deadtime register 2"]
    #[inline(always)]
    pub const fn dtr2(&self) -> &DTR2 {
        &self.dtr2
    }
    #[doc = "0x5c - TIM17 input selection register"]
    #[inline(always)]
    pub const fn tisel(&self) -> &TISEL {
        &self.tisel
    }
    #[doc = "0x60 - TIM17 alternate function register 1"]
    #[inline(always)]
    pub const fn af1(&self) -> &AF1 {
        &self.af1
    }
    #[doc = "0x64 - TIM17 alternate function register 2"]
    #[inline(always)]
    pub const fn af2(&self) -> &AF2 {
        &self.af2
    }
    #[doc = "0x3dc - TIM17 DMA control register"]
    #[inline(always)]
    pub const fn dcr(&self) -> &DCR {
        &self.dcr
    }
    #[doc = "0x3e0 - TIM16/TIM17 DMA address for full transfer"]
    #[inline(always)]
    pub const fn dmar(&self) -> &DMAR {
        &self.dmar
    }
}
#[doc = "CR1 (rw) register accessor: TIM17 control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`]
module"]
pub type CR1 = crate::Reg<cr1::CR1rs>;
#[doc = "TIM17 control register 1"]
pub mod cr1;
#[doc = "CR2 (rw) register accessor: TIM17 control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr2`]
module"]
pub type CR2 = crate::Reg<cr2::CR2rs>;
#[doc = "TIM17 control register 2"]
pub mod cr2;
#[doc = "DIER (rw) register accessor: TIM17 DMA/interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dier`]
module"]
pub type DIER = crate::Reg<dier::DIERrs>;
#[doc = "TIM17 DMA/interrupt enable register"]
pub mod dier;
#[doc = "SR (rw) register accessor: TIM17 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
pub type SR = crate::Reg<sr::SRrs>;
#[doc = "TIM17 status register"]
pub mod sr;
#[doc = "EGR (w) register accessor: TIM17 event generation register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`egr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@egr`]
module"]
pub type EGR = crate::Reg<egr::EGRrs>;
#[doc = "TIM17 event generation register"]
pub mod egr;
#[doc = "CCMR1_Input (rw) register accessor: TIM17 capture/compare mode register 1 \\[alternate\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccmr1_input::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccmr1_input::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccmr1_input`]
module"]
#[doc(alias = "CCMR1_Input")]
pub type CCMR1_INPUT = crate::Reg<ccmr1_input::CCMR1_INPUTrs>;
#[doc = "TIM17 capture/compare mode register 1 \\[alternate\\]"]
pub mod ccmr1_input;
#[doc = "CCMR1_Output (rw) register accessor: TIM17 capture/compare mode register 1 \\[alternate\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccmr1_output::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccmr1_output::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccmr1_output`]
module"]
#[doc(alias = "CCMR1_Output")]
pub type CCMR1_OUTPUT = crate::Reg<ccmr1_output::CCMR1_OUTPUTrs>;
#[doc = "TIM17 capture/compare mode register 1 \\[alternate\\]"]
pub mod ccmr1_output;
#[doc = "CCER (rw) register accessor: TIM17 capture/compare enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccer`]
module"]
pub type CCER = crate::Reg<ccer::CCERrs>;
#[doc = "TIM17 capture/compare enable register"]
pub mod ccer;
#[doc = "CNT (rw) register accessor: TIM17 counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnt`]
module"]
pub type CNT = crate::Reg<cnt::CNTrs>;
#[doc = "TIM17 counter"]
pub mod cnt;
#[doc = "PSC (rw) register accessor: TIM17 prescaler\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psc`]
module"]
pub type PSC = crate::Reg<psc::PSCrs>;
#[doc = "TIM17 prescaler"]
pub mod psc;
#[doc = "ARR (rw) register accessor: TIM17 auto-reload register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arr`]
module"]
pub type ARR = crate::Reg<arr::ARRrs>;
#[doc = "TIM17 auto-reload register"]
pub mod arr;
#[doc = "RCR (rw) register accessor: TIM17 repetition counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcr`]
module"]
pub type RCR = crate::Reg<rcr::RCRrs>;
#[doc = "TIM17 repetition counter register"]
pub mod rcr;
#[doc = "CCR1 (rw) register accessor: TIM17 capture/compare register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr1`]
module"]
pub type CCR1 = crate::Reg<ccr1::CCR1rs>;
#[doc = "TIM17 capture/compare register 1"]
pub mod ccr1;
#[doc = "BDTR (rw) register accessor: TIM17 break and dead-time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bdtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bdtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bdtr`]
module"]
pub type BDTR = crate::Reg<bdtr::BDTRrs>;
#[doc = "TIM17 break and dead-time register"]
pub mod bdtr;
#[doc = "DTR2 (rw) register accessor: TIM17 timer deadtime register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtr2`]
module"]
pub type DTR2 = crate::Reg<dtr2::DTR2rs>;
#[doc = "TIM17 timer deadtime register 2"]
pub mod dtr2;
#[doc = "TISEL (rw) register accessor: TIM17 input selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tisel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tisel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tisel`]
module"]
pub type TISEL = crate::Reg<tisel::TISELrs>;
#[doc = "TIM17 input selection register"]
pub mod tisel;
#[doc = "AF1 (rw) register accessor: TIM17 alternate function register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`af1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`af1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@af1`]
module"]
pub type AF1 = crate::Reg<af1::AF1rs>;
#[doc = "TIM17 alternate function register 1"]
pub mod af1;
#[doc = "AF2 (rw) register accessor: TIM17 alternate function register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`af2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`af2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@af2`]
module"]
pub type AF2 = crate::Reg<af2::AF2rs>;
#[doc = "TIM17 alternate function register 2"]
pub mod af2;
#[doc = "DCR (rw) register accessor: TIM17 DMA control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcr`]
module"]
pub type DCR = crate::Reg<dcr::DCRrs>;
#[doc = "TIM17 DMA control register"]
pub mod dcr;
#[doc = "DMAR (rw) register accessor: TIM16/TIM17 DMA address for full transfer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmar`]
module"]
pub type DMAR = crate::Reg<dmar::DMARrs>;
#[doc = "TIM16/TIM17 DMA address for full transfer"]
pub mod dmar;
