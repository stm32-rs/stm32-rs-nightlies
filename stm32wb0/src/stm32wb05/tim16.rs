#[repr(C)]
#[derive(Debug)]
///Register block
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
    _reserved_7_cnt: [u8; 0x04],
    psc: PSC,
    _reserved9: [u8; 0x02],
    arr: ARR,
    _reserved10: [u8; 0x02],
    rcr: RCR,
    _reserved11: [u8; 0x02],
    ccr: [CCR; 1],
    _reserved12: [u8; 0x0e],
    bdtr: BDTR,
    dcr: DCR,
    _reserved14: [u8; 0x02],
    dmar: DMAR,
    _reserved15: [u8; 0x12],
    af1: AF1,
    _reserved16: [u8; 0x04],
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
    ///0x20 - CCER register
    #[inline(always)]
    pub const fn ccer(&self) -> &CCER {
        &self.ccer
    }
    ///0x24 - 16-bit counter register
    #[inline(always)]
    pub const fn cnt16(&self) -> &CNT16 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(36).cast() }
    }
    ///0x24 - CNT register
    #[inline(always)]
    pub const fn cnt(&self) -> &CNT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(36).cast() }
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
    ///0x34 - capture/compare register
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `CCR1` register.</div>
    #[inline(always)]
    pub const fn ccr(&self, n: usize) -> &CCR {
        &self.ccr[n]
    }
    ///Iterator for array of:
    ///0x34 - capture/compare register
    #[inline(always)]
    pub fn ccr_iter(&self) -> impl Iterator<Item = &CCR> {
        self.ccr.iter()
    }
    ///0x34 - capture/compare register
    #[inline(always)]
    pub const fn ccr1(&self) -> &CCR {
        self.ccr(0)
    }
    ///0x44 - BDTR register
    #[inline(always)]
    pub const fn bdtr(&self) -> &BDTR {
        &self.bdtr
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
    ///0x60 - AF1 register
    #[inline(always)]
    pub const fn af1(&self) -> &AF1 {
        &self.af1
    }
    ///0x68 - TISEL register
    #[inline(always)]
    pub const fn tisel(&self) -> &TISEL {
        &self.tisel
    }
}
/**CR1 (rw) register accessor: CR1 register

You can [`read`](crate::Reg::read) this register and get [`cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#TIM16:CR1)

For information about available fields see [`mod@cr1`] module*/
pub type CR1 = crate::Reg<cr1::CR1rs>;
///CR1 register
pub mod cr1;
/**CR2 (rw) register accessor: CR2 register

You can [`read`](crate::Reg::read) this register and get [`cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#TIM16:CR2)

For information about available fields see [`mod@cr2`] module*/
pub type CR2 = crate::Reg<cr2::CR2rs>;
///CR2 register
pub mod cr2;
/**DIER (rw) register accessor: DIER register

You can [`read`](crate::Reg::read) this register and get [`dier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#TIM16:DIER)

For information about available fields see [`mod@dier`] module*/
pub type DIER = crate::Reg<dier::DIERrs>;
///DIER register
pub mod dier;
/**SR (rw) register accessor: SR register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#TIM16:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///SR register
pub mod sr;
/**EGR (rw) register accessor: EGR register

You can [`read`](crate::Reg::read) this register and get [`egr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`egr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#TIM16:EGR)

For information about available fields see [`mod@egr`] module*/
pub type EGR = crate::Reg<egr::EGRrs>;
///EGR register
pub mod egr;
/**CCMR1_Output (rw) register accessor: CCMR1 register

You can [`read`](crate::Reg::read) this register and get [`ccmr1_output::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr1_output::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#TIM16:CCMR1_Output)

For information about available fields see [`mod@ccmr1_output`] module*/
#[doc(alias = "CCMR1_Output")]
pub type CCMR1_OUTPUT = crate::Reg<ccmr1_output::CCMR1_OUTPUTrs>;
///CCMR1 register
pub mod ccmr1_output;
/**CCMR1_Input (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`ccmr1_input::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr1_input::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#TIM16:CCMR1_Input)

For information about available fields see [`mod@ccmr1_input`] module*/
#[doc(alias = "CCMR1_Input")]
pub type CCMR1_INPUT = crate::Reg<ccmr1_input::CCMR1_INPUTrs>;
///
pub mod ccmr1_input;
/**CCER (rw) register accessor: CCER register

You can [`read`](crate::Reg::read) this register and get [`ccer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#TIM16:CCER)

For information about available fields see [`mod@ccer`] module*/
pub type CCER = crate::Reg<ccer::CCERrs>;
///CCER register
pub mod ccer;
/**CNT (rw) register accessor: CNT register

You can [`read`](crate::Reg::read) this register and get [`cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#TIM16:CNT)

For information about available fields see [`mod@cnt`] module*/
pub type CNT = crate::Reg<cnt::CNTrs>;
///CNT register
pub mod cnt;
/**CNT16 (rw) register accessor: 16-bit counter register

You can [`read`](crate::Reg::read) this register and get [`cnt16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnt16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#TIM16:CNT16)

For information about available fields see [`mod@cnt16`] module*/
pub type CNT16 = crate::Reg<cnt16::CNT16rs>;
///16-bit counter register
pub mod cnt16;
/**PSC (rw) register accessor: PSC register

You can [`read`](crate::Reg::read) this register and get [`psc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#TIM16:PSC)

For information about available fields see [`mod@psc`] module*/
pub type PSC = crate::Reg<psc::PSCrs>;
///PSC register
pub mod psc;
/**ARR (rw) register accessor: ARR register

You can [`read`](crate::Reg::read) this register and get [`arr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#TIM16:ARR)

For information about available fields see [`mod@arr`] module*/
pub type ARR = crate::Reg<arr::ARRrs>;
///ARR register
pub mod arr;
/**RCR (rw) register accessor: RCR register

You can [`read`](crate::Reg::read) this register and get [`rcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#TIM16:RCR)

For information about available fields see [`mod@rcr`] module*/
pub type RCR = crate::Reg<rcr::RCRrs>;
///RCR register
pub mod rcr;
/**CCR (rw) register accessor: capture/compare register

You can [`read`](crate::Reg::read) this register and get [`ccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#TIM16:CCR[1])

For information about available fields see [`mod@ccr`] module*/
pub type CCR = crate::Reg<ccr::CCRrs>;
///capture/compare register
pub mod ccr;
/**BDTR (rw) register accessor: BDTR register

You can [`read`](crate::Reg::read) this register and get [`bdtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#TIM16:BDTR)

For information about available fields see [`mod@bdtr`] module*/
pub type BDTR = crate::Reg<bdtr::BDTRrs>;
///BDTR register
pub mod bdtr;
/**DCR (rw) register accessor: DCR register

You can [`read`](crate::Reg::read) this register and get [`dcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#TIM16:DCR)

For information about available fields see [`mod@dcr`] module*/
pub type DCR = crate::Reg<dcr::DCRrs>;
///DCR register
pub mod dcr;
/**DMAR (rw) register accessor: DMAR register

You can [`read`](crate::Reg::read) this register and get [`dmar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#TIM16:DMAR)

For information about available fields see [`mod@dmar`] module*/
pub type DMAR = crate::Reg<dmar::DMARrs>;
///DMAR register
pub mod dmar;
/**AF1 (rw) register accessor: AF1 register

You can [`read`](crate::Reg::read) this register and get [`af1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`af1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#TIM16:AF1)

For information about available fields see [`mod@af1`] module*/
pub type AF1 = crate::Reg<af1::AF1rs>;
///AF1 register
pub mod af1;
/**TISEL (rw) register accessor: TISEL register

You can [`read`](crate::Reg::read) this register and get [`tisel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tisel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#TIM16:TISEL)

For information about available fields see [`mod@tisel`] module*/
pub type TISEL = crate::Reg<tisel::TISELrs>;
///TISEL register
pub mod tisel;
