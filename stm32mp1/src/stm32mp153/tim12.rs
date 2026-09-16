#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    _cr1: _CR1,
    _reserved1: [u8; 0x02],
    _cr2: _CR2,
    _smcr: _SMCR,
    _dier: _DIER,
    _reserved4: [u8; 0x02],
    _sr: _SR,
    _egr: _EGR,
    _reserved6: [u8; 0x02],
    _reserved_6__ccmr1: [u8; 0x04],
    _reserved7: [u8; 0x04],
    _ccer: _CCER,
    _cnt: _CNT,
    _psc: _PSC,
    _reserved10: [u8; 0x02],
    _arr: _ARR,
    _reserved11: [u8; 0x06],
    _ccr1: _CCR1,
    _reserved12: [u8; 0x02],
    _ccr2: _CCR2,
    _reserved13: [u8; 0x2e],
    _tisel: _TISEL,
}
impl RegisterBlock {
    ///0x00 - TIM12 control register 1
    #[inline(always)]
    pub const fn _cr1(&self) -> &_CR1 {
        &self._cr1
    }
    ///0x04 - TIM12 control register 2
    #[inline(always)]
    pub const fn _cr2(&self) -> &_CR2 {
        &self._cr2
    }
    ///0x08 - TIM12 slave mode control register
    #[inline(always)]
    pub const fn _smcr(&self) -> &_SMCR {
        &self._smcr
    }
    ///0x0c - TIM12 interrupt enable register
    #[inline(always)]
    pub const fn _dier(&self) -> &_DIER {
        &self._dier
    }
    ///0x10 - TIM12 status register
    #[inline(always)]
    pub const fn _sr(&self) -> &_SR {
        &self._sr
    }
    ///0x14 - TIM12 event generation register
    #[inline(always)]
    pub const fn _egr(&self) -> &_EGR {
        &self._egr
    }
    ///0x18 - TIM12 capture/compare mode register 1
    #[inline(always)]
    pub const fn _ccmr1_output(&self) -> &_CCMR1_OUTPUT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).cast() }
    }
    ///0x18 - TIM12 capture/compare mode register 1
    #[inline(always)]
    pub const fn _ccmr1_input(&self) -> &_CCMR1_INPUT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).cast() }
    }
    ///0x20 - TIM12 capture/compare enable register
    #[inline(always)]
    pub const fn _ccer(&self) -> &_CCER {
        &self._ccer
    }
    ///0x24 - TIM12 counter
    #[inline(always)]
    pub const fn _cnt(&self) -> &_CNT {
        &self._cnt
    }
    ///0x28 - TIM12 prescaler
    #[inline(always)]
    pub const fn _psc(&self) -> &_PSC {
        &self._psc
    }
    ///0x2c - TIM12 auto-reload register
    #[inline(always)]
    pub const fn _arr(&self) -> &_ARR {
        &self._arr
    }
    ///0x34 - TIM12 capture/compare register 1
    #[inline(always)]
    pub const fn _ccr1(&self) -> &_CCR1 {
        &self._ccr1
    }
    ///0x38 - TIM12 capture/compare register 2
    #[inline(always)]
    pub const fn _ccr2(&self) -> &_CCR2 {
        &self._ccr2
    }
    ///0x68 - TIM12 timer input selection register
    #[inline(always)]
    pub const fn _tisel(&self) -> &_TISEL {
        &self._tisel
    }
}
/**_CR1 (rw) register accessor: TIM12 control register 1

You can [`read`](crate::Reg::read) this register and get [`_cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM12:_CR1)

For information about available fields see [`mod@_cr1`] module*/
pub type _CR1 = crate::Reg<_cr1::_CR1rs>;
///TIM12 control register 1
pub mod _cr1;
/**_CR2 (rw) register accessor: TIM12 control register 2

You can [`read`](crate::Reg::read) this register and get [`_cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM12:_CR2)

For information about available fields see [`mod@_cr2`] module*/
pub type _CR2 = crate::Reg<_cr2::_CR2rs>;
///TIM12 control register 2
pub mod _cr2;
/**_SMCR (rw) register accessor: TIM12 slave mode control register

You can [`read`](crate::Reg::read) this register and get [`_smcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_smcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM12:_SMCR)

For information about available fields see [`mod@_smcr`] module*/
pub type _SMCR = crate::Reg<_smcr::_SMCRrs>;
///TIM12 slave mode control register
pub mod _smcr;
/**_DIER (rw) register accessor: TIM12 interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`_dier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_dier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM12:_DIER)

For information about available fields see [`mod@_dier`] module*/
pub type _DIER = crate::Reg<_dier::_DIERrs>;
///TIM12 interrupt enable register
pub mod _dier;
/**_SR (rw) register accessor: TIM12 status register

You can [`read`](crate::Reg::read) this register and get [`_sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM12:_SR)

For information about available fields see [`mod@_sr`] module*/
pub type _SR = crate::Reg<_sr::_SRrs>;
///TIM12 status register
pub mod _sr;
/**_EGR (w) register accessor: TIM12 event generation register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_egr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM12:_EGR)

For information about available fields see [`mod@_egr`] module*/
pub type _EGR = crate::Reg<_egr::_EGRrs>;
///TIM12 event generation register
pub mod _egr;
/**_CCMR1_input (rw) register accessor: TIM12 capture/compare mode register 1

You can [`read`](crate::Reg::read) this register and get [`_ccmr1_input::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_ccmr1_input::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM12:_CCMR1_input)

For information about available fields see [`mod@_ccmr1_input`] module*/
#[doc(alias = "_CCMR1_input")]
pub type _CCMR1_INPUT = crate::Reg<_ccmr1_input::_CCMR1_INPUTrs>;
///TIM12 capture/compare mode register 1
pub mod _ccmr1_input;
/**_CCMR1_output (rw) register accessor: TIM12 capture/compare mode register 1

You can [`read`](crate::Reg::read) this register and get [`_ccmr1_output::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_ccmr1_output::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM12:_CCMR1_output)

For information about available fields see [`mod@_ccmr1_output`] module*/
#[doc(alias = "_CCMR1_output")]
pub type _CCMR1_OUTPUT = crate::Reg<_ccmr1_output::_CCMR1_OUTPUTrs>;
///TIM12 capture/compare mode register 1
pub mod _ccmr1_output;
/**_CCER (rw) register accessor: TIM12 capture/compare enable register

You can [`read`](crate::Reg::read) this register and get [`_ccer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_ccer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM12:_CCER)

For information about available fields see [`mod@_ccer`] module*/
pub type _CCER = crate::Reg<_ccer::_CCERrs>;
///TIM12 capture/compare enable register
pub mod _ccer;
/**_CNT (rw) register accessor: TIM12 counter

You can [`read`](crate::Reg::read) this register and get [`_cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM12:_CNT)

For information about available fields see [`mod@_cnt`] module*/
pub type _CNT = crate::Reg<_cnt::_CNTrs>;
///TIM12 counter
pub mod _cnt;
/**_PSC (rw) register accessor: TIM12 prescaler

You can [`read`](crate::Reg::read) this register and get [`_psc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_psc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM12:_PSC)

For information about available fields see [`mod@_psc`] module*/
pub type _PSC = crate::Reg<_psc::_PSCrs>;
///TIM12 prescaler
pub mod _psc;
/**_ARR (rw) register accessor: TIM12 auto-reload register

You can [`read`](crate::Reg::read) this register and get [`_arr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_arr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM12:_ARR)

For information about available fields see [`mod@_arr`] module*/
pub type _ARR = crate::Reg<_arr::_ARRrs>;
///TIM12 auto-reload register
pub mod _arr;
/**_CCR1 (rw) register accessor: TIM12 capture/compare register 1

You can [`read`](crate::Reg::read) this register and get [`_ccr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_ccr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM12:_CCR1)

For information about available fields see [`mod@_ccr1`] module*/
pub type _CCR1 = crate::Reg<_ccr1::_CCR1rs>;
///TIM12 capture/compare register 1
pub mod _ccr1;
/**_CCR2 (rw) register accessor: TIM12 capture/compare register 2

You can [`read`](crate::Reg::read) this register and get [`_ccr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_ccr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM12:_CCR2)

For information about available fields see [`mod@_ccr2`] module*/
pub type _CCR2 = crate::Reg<_ccr2::_CCR2rs>;
///TIM12 capture/compare register 2
pub mod _ccr2;
/**_TISEL (rw) register accessor: TIM12 timer input selection register

You can [`read`](crate::Reg::read) this register and get [`_tisel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_tisel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM12:_TISEL)

For information about available fields see [`mod@_tisel`] module*/
pub type _TISEL = crate::Reg<_tisel::_TISELrs>;
///TIM12 timer input selection register
pub mod _tisel;
