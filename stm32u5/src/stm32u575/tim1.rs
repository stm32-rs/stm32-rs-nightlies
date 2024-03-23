#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tim1_cr1: TIM1_CR1,
    _reserved1: [u8; 0x02],
    tim1_cr2: TIM1_CR2,
    tim1_smcr: TIM1_SMCR,
    tim1_dier: TIM1_DIER,
    tim1_sr: TIM1_SR,
    tim1_egr: TIM1_EGR,
    _reserved6: [u8; 0x02],
    _reserved_6_tim1_ccmr1: [u8; 0x04],
    _reserved_7_tim1_ccmr2: [u8; 0x04],
    tim1_ccer: TIM1_CCER,
    tim1_cnt: TIM1_CNT,
    tim1_psc: TIM1_PSC,
    _reserved11: [u8; 0x02],
    tim1_arr: TIM1_ARR,
    tim1_rcr: TIM1_RCR,
    _reserved13: [u8; 0x02],
    tim1_ccr1: TIM1_CCR1,
    tim1_ccr2: TIM1_CCR2,
    tim1_ccr3: TIM1_CCR3,
    tim1_ccr4: TIM1_CCR4,
    tim1_bdtr: TIM1_BDTR,
    tim1_ccr5: TIM1_CCR5,
    tim1_ccr6: TIM1_CCR6,
    tim1_ccmr3: TIM1_CCMR3,
    tim1_dtr2: TIM1_DTR2,
    tim1_ecr: TIM1_ECR,
    tim1_tisel: TIM1_TISEL,
    tim1_af1: TIM1_AF1,
    tim1_af2: TIM1_AF2,
    _reserved26: [u8; 0x0374],
    tim1_dcr: TIM1_DCR,
    tim1_dmar: TIM1_DMAR,
}
impl RegisterBlock {
    #[doc = "0x00 - TIM1 control register 1"]
    #[inline(always)]
    pub const fn tim1_cr1(&self) -> &TIM1_CR1 {
        &self.tim1_cr1
    }
    #[doc = "0x04 - control register 2"]
    #[inline(always)]
    pub const fn tim1_cr2(&self) -> &TIM1_CR2 {
        &self.tim1_cr2
    }
    #[doc = "0x08 - TIM1 slave mode control register"]
    #[inline(always)]
    pub const fn tim1_smcr(&self) -> &TIM1_SMCR {
        &self.tim1_smcr
    }
    #[doc = "0x0c - TIM1 DMA/interrupt enable register"]
    #[inline(always)]
    pub const fn tim1_dier(&self) -> &TIM1_DIER {
        &self.tim1_dier
    }
    #[doc = "0x10 - TIM1 status register"]
    #[inline(always)]
    pub const fn tim1_sr(&self) -> &TIM1_SR {
        &self.tim1_sr
    }
    #[doc = "0x14 - TIM1 event generation register"]
    #[inline(always)]
    pub const fn tim1_egr(&self) -> &TIM1_EGR {
        &self.tim1_egr
    }
    #[doc = "0x18 - TIM1 capture/compare mode register 1 \\[alternate\\]"]
    #[inline(always)]
    pub const fn tim1_ccmr1_input(&self) -> &TIM1_CCMR1_INPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(24).cast() }
    }
    #[doc = "0x18 - capture/compare mode register 1 (output mode)"]
    #[inline(always)]
    pub const fn tim1_ccmr1_output(&self) -> &TIM1_CCMR1_OUTPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(24).cast() }
    }
    #[doc = "0x1c - TIM1 capture/compare mode register 2 \\[alternate\\]"]
    #[inline(always)]
    pub const fn tim1_ccmr2_input(&self) -> &TIM1_CCMR2_INPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(28).cast() }
    }
    #[doc = "0x1c - capture/compare mode register 2 (output mode)"]
    #[inline(always)]
    pub const fn tim1_ccmr2_output(&self) -> &TIM1_CCMR2_OUTPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(28).cast() }
    }
    #[doc = "0x20 - TIM1 capture/compare enable register"]
    #[inline(always)]
    pub const fn tim1_ccer(&self) -> &TIM1_CCER {
        &self.tim1_ccer
    }
    #[doc = "0x24 - TIM1 counter"]
    #[inline(always)]
    pub const fn tim1_cnt(&self) -> &TIM1_CNT {
        &self.tim1_cnt
    }
    #[doc = "0x28 - TIM1 prescaler"]
    #[inline(always)]
    pub const fn tim1_psc(&self) -> &TIM1_PSC {
        &self.tim1_psc
    }
    #[doc = "0x2c - TIM1 auto-reload register"]
    #[inline(always)]
    pub const fn tim1_arr(&self) -> &TIM1_ARR {
        &self.tim1_arr
    }
    #[doc = "0x30 - TIM1 repetition counter register"]
    #[inline(always)]
    pub const fn tim1_rcr(&self) -> &TIM1_RCR {
        &self.tim1_rcr
    }
    #[doc = "0x34 - TIM1 capture/compare register 1"]
    #[inline(always)]
    pub const fn tim1_ccr1(&self) -> &TIM1_CCR1 {
        &self.tim1_ccr1
    }
    #[doc = "0x38 - TIM1 capture/compare register 2"]
    #[inline(always)]
    pub const fn tim1_ccr2(&self) -> &TIM1_CCR2 {
        &self.tim1_ccr2
    }
    #[doc = "0x3c - TIM1 capture/compare register 3"]
    #[inline(always)]
    pub const fn tim1_ccr3(&self) -> &TIM1_CCR3 {
        &self.tim1_ccr3
    }
    #[doc = "0x40 - TIM1 capture/compare register 4"]
    #[inline(always)]
    pub const fn tim1_ccr4(&self) -> &TIM1_CCR4 {
        &self.tim1_ccr4
    }
    #[doc = "0x44 - TIM1 break and dead-time register"]
    #[inline(always)]
    pub const fn tim1_bdtr(&self) -> &TIM1_BDTR {
        &self.tim1_bdtr
    }
    #[doc = "0x48 - TIM1 capture/compare register 5"]
    #[inline(always)]
    pub const fn tim1_ccr5(&self) -> &TIM1_CCR5 {
        &self.tim1_ccr5
    }
    #[doc = "0x4c - TIM1 capture/compare register 6"]
    #[inline(always)]
    pub const fn tim1_ccr6(&self) -> &TIM1_CCR6 {
        &self.tim1_ccr6
    }
    #[doc = "0x50 - TIM1 capture/compare mode register 3"]
    #[inline(always)]
    pub const fn tim1_ccmr3(&self) -> &TIM1_CCMR3 {
        &self.tim1_ccmr3
    }
    #[doc = "0x54 - TIM1 timer deadtime register 2"]
    #[inline(always)]
    pub const fn tim1_dtr2(&self) -> &TIM1_DTR2 {
        &self.tim1_dtr2
    }
    #[doc = "0x58 - TIM1 timer encoder control register"]
    #[inline(always)]
    pub const fn tim1_ecr(&self) -> &TIM1_ECR {
        &self.tim1_ecr
    }
    #[doc = "0x5c - TIM1 timer input selection register"]
    #[inline(always)]
    pub const fn tim1_tisel(&self) -> &TIM1_TISEL {
        &self.tim1_tisel
    }
    #[doc = "0x60 - TIM1 alternate function option register 1"]
    #[inline(always)]
    pub const fn tim1_af1(&self) -> &TIM1_AF1 {
        &self.tim1_af1
    }
    #[doc = "0x64 - TIM1 alternate function register 2"]
    #[inline(always)]
    pub const fn tim1_af2(&self) -> &TIM1_AF2 {
        &self.tim1_af2
    }
    #[doc = "0x3dc - TIM1 DMA control register"]
    #[inline(always)]
    pub const fn tim1_dcr(&self) -> &TIM1_DCR {
        &self.tim1_dcr
    }
    #[doc = "0x3e0 - TIM1 DMA address for full transfer"]
    #[inline(always)]
    pub const fn tim1_dmar(&self) -> &TIM1_DMAR {
        &self.tim1_dmar
    }
}
#[doc = "TIM1_CR1 (rw) register accessor: TIM1 control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_cr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_cr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_cr1`]
module"]
pub type TIM1_CR1 = crate::Reg<tim1_cr1::TIM1_CR1rs>;
#[doc = "TIM1 control register 1"]
pub mod tim1_cr1;
#[doc = "TIM1_CR2 (rw) register accessor: control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_cr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_cr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_cr2`]
module"]
pub type TIM1_CR2 = crate::Reg<tim1_cr2::TIM1_CR2rs>;
#[doc = "control register 2"]
pub mod tim1_cr2;
#[doc = "TIM1_SMCR (rw) register accessor: TIM1 slave mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_smcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_smcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_smcr`]
module"]
pub type TIM1_SMCR = crate::Reg<tim1_smcr::TIM1_SMCRrs>;
#[doc = "TIM1 slave mode control register"]
pub mod tim1_smcr;
#[doc = "TIM1_DIER (rw) register accessor: TIM1 DMA/interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_dier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_dier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_dier`]
module"]
pub type TIM1_DIER = crate::Reg<tim1_dier::TIM1_DIERrs>;
#[doc = "TIM1 DMA/interrupt enable register"]
pub mod tim1_dier;
#[doc = "TIM1_SR (rw) register accessor: TIM1 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_sr`]
module"]
pub type TIM1_SR = crate::Reg<tim1_sr::TIM1_SRrs>;
#[doc = "TIM1 status register"]
pub mod tim1_sr;
#[doc = "TIM1_EGR (w) register accessor: TIM1 event generation register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_egr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_egr`]
module"]
pub type TIM1_EGR = crate::Reg<tim1_egr::TIM1_EGRrs>;
#[doc = "TIM1 event generation register"]
pub mod tim1_egr;
#[doc = "TIM1_CCMR1_Output (rw) register accessor: capture/compare mode register 1 (output mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_ccmr1_output::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_ccmr1_output::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_ccmr1_output`]
module"]
#[doc(alias = "TIM1_CCMR1_Output")]
pub type TIM1_CCMR1_OUTPUT = crate::Reg<tim1_ccmr1_output::TIM1_CCMR1_OUTPUTrs>;
#[doc = "capture/compare mode register 1 (output mode)"]
pub mod tim1_ccmr1_output;
#[doc = "TIM1_CCMR1_Input (rw) register accessor: TIM1 capture/compare mode register 1 \\[alternate\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_ccmr1_input::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_ccmr1_input::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_ccmr1_input`]
module"]
#[doc(alias = "TIM1_CCMR1_Input")]
pub type TIM1_CCMR1_INPUT = crate::Reg<tim1_ccmr1_input::TIM1_CCMR1_INPUTrs>;
#[doc = "TIM1 capture/compare mode register 1 \\[alternate\\]"]
pub mod tim1_ccmr1_input;
#[doc = "TIM1_CCMR2_Output (rw) register accessor: capture/compare mode register 2 (output mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_ccmr2_output::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_ccmr2_output::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_ccmr2_output`]
module"]
#[doc(alias = "TIM1_CCMR2_Output")]
pub type TIM1_CCMR2_OUTPUT = crate::Reg<tim1_ccmr2_output::TIM1_CCMR2_OUTPUTrs>;
#[doc = "capture/compare mode register 2 (output mode)"]
pub mod tim1_ccmr2_output;
#[doc = "TIM1_CCMR2_Input (rw) register accessor: TIM1 capture/compare mode register 2 \\[alternate\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_ccmr2_input::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_ccmr2_input::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_ccmr2_input`]
module"]
#[doc(alias = "TIM1_CCMR2_Input")]
pub type TIM1_CCMR2_INPUT = crate::Reg<tim1_ccmr2_input::TIM1_CCMR2_INPUTrs>;
#[doc = "TIM1 capture/compare mode register 2 \\[alternate\\]"]
pub mod tim1_ccmr2_input;
#[doc = "TIM1_CCER (rw) register accessor: TIM1 capture/compare enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_ccer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_ccer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_ccer`]
module"]
pub type TIM1_CCER = crate::Reg<tim1_ccer::TIM1_CCERrs>;
#[doc = "TIM1 capture/compare enable register"]
pub mod tim1_ccer;
#[doc = "TIM1_CNT (rw) register accessor: TIM1 counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_cnt`]
module"]
pub type TIM1_CNT = crate::Reg<tim1_cnt::TIM1_CNTrs>;
#[doc = "TIM1 counter"]
pub mod tim1_cnt;
#[doc = "TIM1_PSC (rw) register accessor: TIM1 prescaler\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_psc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_psc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_psc`]
module"]
pub type TIM1_PSC = crate::Reg<tim1_psc::TIM1_PSCrs>;
#[doc = "TIM1 prescaler"]
pub mod tim1_psc;
#[doc = "TIM1_ARR (rw) register accessor: TIM1 auto-reload register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_arr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_arr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_arr`]
module"]
pub type TIM1_ARR = crate::Reg<tim1_arr::TIM1_ARRrs>;
#[doc = "TIM1 auto-reload register"]
pub mod tim1_arr;
#[doc = "TIM1_RCR (rw) register accessor: TIM1 repetition counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_rcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_rcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_rcr`]
module"]
pub type TIM1_RCR = crate::Reg<tim1_rcr::TIM1_RCRrs>;
#[doc = "TIM1 repetition counter register"]
pub mod tim1_rcr;
#[doc = "TIM1_CCR1 (rw) register accessor: TIM1 capture/compare register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_ccr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_ccr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_ccr1`]
module"]
pub type TIM1_CCR1 = crate::Reg<tim1_ccr1::TIM1_CCR1rs>;
#[doc = "TIM1 capture/compare register 1"]
pub mod tim1_ccr1;
#[doc = "TIM1_CCR2 (rw) register accessor: TIM1 capture/compare register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_ccr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_ccr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_ccr2`]
module"]
pub type TIM1_CCR2 = crate::Reg<tim1_ccr2::TIM1_CCR2rs>;
#[doc = "TIM1 capture/compare register 2"]
pub mod tim1_ccr2;
#[doc = "TIM1_CCR3 (rw) register accessor: TIM1 capture/compare register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_ccr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_ccr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_ccr3`]
module"]
pub type TIM1_CCR3 = crate::Reg<tim1_ccr3::TIM1_CCR3rs>;
#[doc = "TIM1 capture/compare register 3"]
pub mod tim1_ccr3;
#[doc = "TIM1_CCR4 (rw) register accessor: TIM1 capture/compare register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_ccr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_ccr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_ccr4`]
module"]
pub type TIM1_CCR4 = crate::Reg<tim1_ccr4::TIM1_CCR4rs>;
#[doc = "TIM1 capture/compare register 4"]
pub mod tim1_ccr4;
#[doc = "TIM1_BDTR (rw) register accessor: TIM1 break and dead-time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_bdtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_bdtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_bdtr`]
module"]
pub type TIM1_BDTR = crate::Reg<tim1_bdtr::TIM1_BDTRrs>;
#[doc = "TIM1 break and dead-time register"]
pub mod tim1_bdtr;
#[doc = "TIM1_CCR5 (rw) register accessor: TIM1 capture/compare register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_ccr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_ccr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_ccr5`]
module"]
pub type TIM1_CCR5 = crate::Reg<tim1_ccr5::TIM1_CCR5rs>;
#[doc = "TIM1 capture/compare register 5"]
pub mod tim1_ccr5;
#[doc = "TIM1_CCR6 (rw) register accessor: TIM1 capture/compare register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_ccr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_ccr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_ccr6`]
module"]
pub type TIM1_CCR6 = crate::Reg<tim1_ccr6::TIM1_CCR6rs>;
#[doc = "TIM1 capture/compare register 6"]
pub mod tim1_ccr6;
#[doc = "TIM1_CCMR3 (rw) register accessor: TIM1 capture/compare mode register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_ccmr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_ccmr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_ccmr3`]
module"]
pub type TIM1_CCMR3 = crate::Reg<tim1_ccmr3::TIM1_CCMR3rs>;
#[doc = "TIM1 capture/compare mode register 3"]
pub mod tim1_ccmr3;
#[doc = "TIM1_DTR2 (rw) register accessor: TIM1 timer deadtime register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_dtr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_dtr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_dtr2`]
module"]
pub type TIM1_DTR2 = crate::Reg<tim1_dtr2::TIM1_DTR2rs>;
#[doc = "TIM1 timer deadtime register 2"]
pub mod tim1_dtr2;
#[doc = "TIM1_ECR (rw) register accessor: TIM1 timer encoder control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_ecr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_ecr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_ecr`]
module"]
pub type TIM1_ECR = crate::Reg<tim1_ecr::TIM1_ECRrs>;
#[doc = "TIM1 timer encoder control register"]
pub mod tim1_ecr;
#[doc = "TIM1_TISEL (rw) register accessor: TIM1 timer input selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_tisel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_tisel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_tisel`]
module"]
pub type TIM1_TISEL = crate::Reg<tim1_tisel::TIM1_TISELrs>;
#[doc = "TIM1 timer input selection register"]
pub mod tim1_tisel;
#[doc = "TIM1_AF1 (rw) register accessor: TIM1 alternate function option register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_af1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_af1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_af1`]
module"]
pub type TIM1_AF1 = crate::Reg<tim1_af1::TIM1_AF1rs>;
#[doc = "TIM1 alternate function option register 1"]
pub mod tim1_af1;
#[doc = "TIM1_AF2 (rw) register accessor: TIM1 alternate function register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_af2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_af2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_af2`]
module"]
pub type TIM1_AF2 = crate::Reg<tim1_af2::TIM1_AF2rs>;
#[doc = "TIM1 alternate function register 2"]
pub mod tim1_af2;
#[doc = "TIM1_DCR (rw) register accessor: TIM1 DMA control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_dcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_dcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_dcr`]
module"]
pub type TIM1_DCR = crate::Reg<tim1_dcr::TIM1_DCRrs>;
#[doc = "TIM1 DMA control register"]
pub mod tim1_dcr;
#[doc = "TIM1_DMAR (rw) register accessor: TIM1 DMA address for full transfer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_dmar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_dmar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_dmar`]
module"]
pub type TIM1_DMAR = crate::Reg<tim1_dmar::TIM1_DMARrs>;
#[doc = "TIM1 DMA address for full transfer"]
pub mod tim1_dmar;
