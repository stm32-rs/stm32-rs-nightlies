#[repr(C)]
#[derive(Debug)]
///Register block
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
    _reserved14: [u8; 0x04],
    dcr: DCR,
    dmar: DMAR,
    _reserved16: [u8; 0x18],
    tisel: TISEL,
}
impl RegisterBlock {
    ///0x00 - CR1 register
    #[inline(always)]
    pub const fn cr1(&self) -> &CR1 {
        &self.cr1
    }
    ///0x04 - CR2 register
    #[inline(always)]
    pub const fn cr2(&self) -> &CR2 {
        &self.cr2
    }
    ///0x08 - SMCR register
    #[inline(always)]
    pub const fn smcr(&self) -> &SMCR {
        &self.smcr
    }
    ///0x0c - DIER register
    #[inline(always)]
    pub const fn dier(&self) -> &DIER {
        &self.dier
    }
    ///0x10 - SR register
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0x14 - EGR register
    #[inline(always)]
    pub const fn egr(&self) -> &EGR {
        &self.egr
    }
    ///0x18 -
    #[inline(always)]
    pub const fn ccmr1_input(&self) -> &CCMR1_INPUT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).cast() }
    }
    ///0x18 - CCMR1 register
    #[inline(always)]
    pub const fn ccmr1_output(&self) -> &CCMR1_OUTPUT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).cast() }
    }
    ///0x1c -
    #[inline(always)]
    pub const fn ccmr2_input(&self) -> &CCMR2_INPUT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(28).cast() }
    }
    ///0x1c - CCMR2 register
    #[inline(always)]
    pub const fn ccmr2_output(&self) -> &CCMR2_OUTPUT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(28).cast() }
    }
    ///0x20 - CCER register
    #[inline(always)]
    pub const fn ccer(&self) -> &CCER {
        &self.ccer
    }
    ///0x24 - CNT register
    #[inline(always)]
    pub const fn cnt(&self) -> &CNT {
        &self.cnt
    }
    ///0x28 - PSC register
    #[inline(always)]
    pub const fn psc(&self) -> &PSC {
        &self.psc
    }
    ///0x2c - ARR register
    #[inline(always)]
    pub const fn arr(&self) -> &ARR {
        &self.arr
    }
    ///0x30 - RCR register
    #[inline(always)]
    pub const fn rcr(&self) -> &RCR {
        &self.rcr
    }
    ///0x34..0x44 - capture/compare register
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `CCR1` register.</div>
    #[inline(always)]
    pub const fn ccr(&self, n: usize) -> &CCR {
        &self.ccr[n]
    }
    ///Iterator for array of:
    ///0x34..0x44 - capture/compare register
    #[inline(always)]
    pub fn ccr_iter(&self) -> impl Iterator<Item = &CCR> {
        self.ccr.iter()
    }
    ///0x34 - capture/compare register
    #[inline(always)]
    pub const fn ccr1(&self) -> &CCR {
        self.ccr(0)
    }
    ///0x38 - capture/compare register
    #[inline(always)]
    pub const fn ccr2(&self) -> &CCR {
        self.ccr(1)
    }
    ///0x3c - capture/compare register
    #[inline(always)]
    pub const fn ccr3(&self) -> &CCR {
        self.ccr(2)
    }
    ///0x40 - capture/compare register
    #[inline(always)]
    pub const fn ccr4(&self) -> &CCR {
        self.ccr(3)
    }
    ///0x48 - DCR register
    #[inline(always)]
    pub const fn dcr(&self) -> &DCR {
        &self.dcr
    }
    ///0x4c - DMAR register
    #[inline(always)]
    pub const fn dmar(&self) -> &DMAR {
        &self.dmar
    }
    ///0x68 - TISEL register
    #[inline(always)]
    pub const fn tisel(&self) -> &TISEL {
        &self.tisel
    }
}
/**CR1 (rw) register accessor: CR1 register

You can [`read`](crate::Reg::read) this register and get [`cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#TIM2:CR1)

For information about available fields see [`mod@cr1`] module*/
pub type CR1 = crate::Reg<cr1::CR1rs>;
///CR1 register
pub mod cr1;
/**CR2 (rw) register accessor: CR2 register

You can [`read`](crate::Reg::read) this register and get [`cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#TIM2:CR2)

For information about available fields see [`mod@cr2`] module*/
pub type CR2 = crate::Reg<cr2::CR2rs>;
///CR2 register
pub mod cr2;
/**SMCR (rw) register accessor: SMCR register

You can [`read`](crate::Reg::read) this register and get [`smcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#TIM2:SMCR)

For information about available fields see [`mod@smcr`] module*/
pub type SMCR = crate::Reg<smcr::SMCRrs>;
///SMCR register
pub mod smcr;
/**DIER (rw) register accessor: DIER register

You can [`read`](crate::Reg::read) this register and get [`dier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#TIM2:DIER)

For information about available fields see [`mod@dier`] module*/
pub type DIER = crate::Reg<dier::DIERrs>;
///DIER register
pub mod dier;
/**SR (rw) register accessor: SR register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#TIM2:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///SR register
pub mod sr;
/**EGR (rw) register accessor: EGR register

You can [`read`](crate::Reg::read) this register and get [`egr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`egr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#TIM2:EGR)

For information about available fields see [`mod@egr`] module*/
pub type EGR = crate::Reg<egr::EGRrs>;
///EGR register
pub mod egr;
/**CCMR1_Output (rw) register accessor: CCMR1 register

You can [`read`](crate::Reg::read) this register and get [`ccmr1_output::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr1_output::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#TIM2:CCMR1_Output)

For information about available fields see [`mod@ccmr1_output`] module*/
#[doc(alias = "CCMR1_Output")]
pub type CCMR1_OUTPUT = crate::Reg<ccmr1_output::CCMR1_OUTPUTrs>;
///CCMR1 register
pub mod ccmr1_output;
/**CCMR1_Input (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`ccmr1_input::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr1_input::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#TIM2:CCMR1_Input)

For information about available fields see [`mod@ccmr1_input`] module*/
#[doc(alias = "CCMR1_Input")]
pub type CCMR1_INPUT = crate::Reg<ccmr1_input::CCMR1_INPUTrs>;
///
pub mod ccmr1_input;
/**CCMR2_Output (rw) register accessor: CCMR2 register

You can [`read`](crate::Reg::read) this register and get [`ccmr2_output::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr2_output::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#TIM2:CCMR2_Output)

For information about available fields see [`mod@ccmr2_output`] module*/
#[doc(alias = "CCMR2_Output")]
pub type CCMR2_OUTPUT = crate::Reg<ccmr2_output::CCMR2_OUTPUTrs>;
///CCMR2 register
pub mod ccmr2_output;
/**CCMR2_Input (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`ccmr2_input::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr2_input::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#TIM2:CCMR2_Input)

For information about available fields see [`mod@ccmr2_input`] module*/
#[doc(alias = "CCMR2_Input")]
pub type CCMR2_INPUT = crate::Reg<ccmr2_input::CCMR2_INPUTrs>;
///
pub mod ccmr2_input;
/**CCER (rw) register accessor: CCER register

You can [`read`](crate::Reg::read) this register and get [`ccer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#TIM2:CCER)

For information about available fields see [`mod@ccer`] module*/
pub type CCER = crate::Reg<ccer::CCERrs>;
///CCER register
pub mod ccer;
/**CNT (rw) register accessor: CNT register

You can [`read`](crate::Reg::read) this register and get [`cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#TIM2:CNT)

For information about available fields see [`mod@cnt`] module*/
pub type CNT = crate::Reg<cnt::CNTrs>;
///CNT register
pub mod cnt;
/**PSC (rw) register accessor: PSC register

You can [`read`](crate::Reg::read) this register and get [`psc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#TIM2:PSC)

For information about available fields see [`mod@psc`] module*/
pub type PSC = crate::Reg<psc::PSCrs>;
///PSC register
pub mod psc;
/**ARR (rw) register accessor: ARR register

You can [`read`](crate::Reg::read) this register and get [`arr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#TIM2:ARR)

For information about available fields see [`mod@arr`] module*/
pub type ARR = crate::Reg<arr::ARRrs>;
///ARR register
pub mod arr;
/**RCR (rw) register accessor: RCR register

You can [`read`](crate::Reg::read) this register and get [`rcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#TIM2:RCR)

For information about available fields see [`mod@rcr`] module*/
pub type RCR = crate::Reg<rcr::RCRrs>;
///RCR register
pub mod rcr;
/**CCR (rw) register accessor: capture/compare register

You can [`read`](crate::Reg::read) this register and get [`ccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#TIM2:CCR[1])

For information about available fields see [`mod@ccr`] module*/
pub type CCR = crate::Reg<ccr::CCRrs>;
///capture/compare register
pub mod ccr;
/**DCR (rw) register accessor: DCR register

You can [`read`](crate::Reg::read) this register and get [`dcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#TIM2:DCR)

For information about available fields see [`mod@dcr`] module*/
pub type DCR = crate::Reg<dcr::DCRrs>;
///DCR register
pub mod dcr;
/**DMAR (rw) register accessor: DMAR register

You can [`read`](crate::Reg::read) this register and get [`dmar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#TIM2:DMAR)

For information about available fields see [`mod@dmar`] module*/
pub type DMAR = crate::Reg<dmar::DMARrs>;
///DMAR register
pub mod dmar;
/**TISEL (rw) register accessor: TISEL register

You can [`read`](crate::Reg::read) this register and get [`tisel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tisel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#TIM2:TISEL)

For information about available fields see [`mod@tisel`] module*/
pub type TISEL = crate::Reg<tisel::TISELrs>;
///TISEL register
pub mod tisel;
