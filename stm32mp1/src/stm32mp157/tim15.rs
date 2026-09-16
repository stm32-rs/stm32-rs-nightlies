#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    _cr1: _CR1,
    _reserved1: [u8; 0x02],
    _cr2: _CR2,
    _reserved2: [u8; 0x02],
    smcr: SMCR,
    _dier: _DIER,
    _reserved4: [u8; 0x02],
    _sr: _SR,
    _reserved5: [u8; 0x02],
    egr: EGR,
    _reserved_6_ccmr1: [u8; 0x04],
    _reserved7: [u8; 0x04],
    _ccer: _CCER,
    _reserved8: [u8; 0x02],
    _cnt: _CNT,
    _psc: _PSC,
    _reserved10: [u8; 0x02],
    _arr: _ARR,
    _reserved11: [u8; 0x02],
    _rcr: _RCR,
    _reserved12: [u8; 0x02],
    _ccr1: _CCR1,
    _reserved13: [u8; 0x02],
    _ccr2: _CCR2,
    _reserved14: [u8; 0x0a],
    bdtr: BDTR,
    _dcr: _DCR,
    _reserved16: [u8; 0x02],
    _dmar: _DMAR,
    _reserved17: [u8; 0x12],
    _af1: _AF1,
    _reserved18: [u8; 0x04],
    _tisel: _TISEL,
}
impl RegisterBlock {
    ///0x00 - TIM15 control register 1
    #[inline(always)]
    pub const fn _cr1(&self) -> &_CR1 {
        &self._cr1
    }
    ///0x04 - TIM15 control register 2
    #[inline(always)]
    pub const fn _cr2(&self) -> &_CR2 {
        &self._cr2
    }
    ///0x08 - slave mode control register
    #[inline(always)]
    pub const fn smcr(&self) -> &SMCR {
        &self.smcr
    }
    ///0x0c - TIM15 DMA/interrupt enable register
    #[inline(always)]
    pub const fn _dier(&self) -> &_DIER {
        &self._dier
    }
    ///0x10 - TIM15 status register
    #[inline(always)]
    pub const fn _sr(&self) -> &_SR {
        &self._sr
    }
    ///0x14 - event generation register
    #[inline(always)]
    pub const fn egr(&self) -> &EGR {
        &self.egr
    }
    ///0x18 - capture/compare mode register 1 (input mode)
    #[inline(always)]
    pub const fn ccmr1_input(&self) -> &CCMR1_INPUT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).cast() }
    }
    ///0x18 - capture/compare mode register 1 (output mode)
    #[inline(always)]
    pub const fn ccmr1_output(&self) -> &CCMR1_OUTPUT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).cast() }
    }
    ///0x20 - TIM15 capture/compare enable register
    #[inline(always)]
    pub const fn _ccer(&self) -> &_CCER {
        &self._ccer
    }
    ///0x24 - TIM15 counter
    #[inline(always)]
    pub const fn _cnt(&self) -> &_CNT {
        &self._cnt
    }
    ///0x28 - TIM15 prescaler
    #[inline(always)]
    pub const fn _psc(&self) -> &_PSC {
        &self._psc
    }
    ///0x2c - TIM15 auto-reload register
    #[inline(always)]
    pub const fn _arr(&self) -> &_ARR {
        &self._arr
    }
    ///0x30 - TIM15 repetition counter register
    #[inline(always)]
    pub const fn _rcr(&self) -> &_RCR {
        &self._rcr
    }
    ///0x34 - TIM15 capture/compare register 1
    #[inline(always)]
    pub const fn _ccr1(&self) -> &_CCR1 {
        &self._ccr1
    }
    ///0x38 - TIM15 capture/compare register 2
    #[inline(always)]
    pub const fn _ccr2(&self) -> &_CCR2 {
        &self._ccr2
    }
    ///0x44 - As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\[3:0\], BKF\[3:0\], AOE, BKP, BKE, OSSI, OSSR and DTG\[7:0\] can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register.
    #[inline(always)]
    pub const fn bdtr(&self) -> &BDTR {
        &self.bdtr
    }
    ///0x48 - TIM15 DMA control register
    #[inline(always)]
    pub const fn _dcr(&self) -> &_DCR {
        &self._dcr
    }
    ///0x4c - TIM15 DMA address for full transfer
    #[inline(always)]
    pub const fn _dmar(&self) -> &_DMAR {
        &self._dmar
    }
    ///0x60 - TIM15 alternate register 1
    #[inline(always)]
    pub const fn _af1(&self) -> &_AF1 {
        &self._af1
    }
    ///0x68 - TIM15 input selection register
    #[inline(always)]
    pub const fn _tisel(&self) -> &_TISEL {
        &self._tisel
    }
}
/**_CR1 (rw) register accessor: TIM15 control register 1

You can [`read`](crate::Reg::read) this register and get [`_cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM15:_CR1)

For information about available fields see [`mod@_cr1`] module*/
pub type _CR1 = crate::Reg<_cr1::_CR1rs>;
///TIM15 control register 1
pub mod _cr1;
/**_CR2 (rw) register accessor: TIM15 control register 2

You can [`read`](crate::Reg::read) this register and get [`_cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM15:_CR2)

For information about available fields see [`mod@_cr2`] module*/
pub type _CR2 = crate::Reg<_cr2::_CR2rs>;
///TIM15 control register 2
pub mod _cr2;
/**SMCR (rw) register accessor: slave mode control register

You can [`read`](crate::Reg::read) this register and get [`smcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM15:SMCR)

For information about available fields see [`mod@smcr`] module*/
pub type SMCR = crate::Reg<smcr::SMCRrs>;
///slave mode control register
pub mod smcr;
/**_DIER (rw) register accessor: TIM15 DMA/interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`_dier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_dier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM15:_DIER)

For information about available fields see [`mod@_dier`] module*/
pub type _DIER = crate::Reg<_dier::_DIERrs>;
///TIM15 DMA/interrupt enable register
pub mod _dier;
/**_SR (rw) register accessor: TIM15 status register

You can [`read`](crate::Reg::read) this register and get [`_sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM15:_SR)

For information about available fields see [`mod@_sr`] module*/
pub type _SR = crate::Reg<_sr::_SRrs>;
///TIM15 status register
pub mod _sr;
/**EGR (w) register accessor: event generation register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`egr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM15:EGR)

For information about available fields see [`mod@egr`] module*/
pub type EGR = crate::Reg<egr::EGRrs>;
///event generation register
pub mod egr;
/**CCMR1_Output (rw) register accessor: capture/compare mode register 1 (output mode)

You can [`read`](crate::Reg::read) this register and get [`ccmr1_output::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr1_output::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM15:CCMR1_Output)

For information about available fields see [`mod@ccmr1_output`] module*/
#[doc(alias = "CCMR1_Output")]
pub type CCMR1_OUTPUT = crate::Reg<ccmr1_output::CCMR1_OUTPUTrs>;
///capture/compare mode register 1 (output mode)
pub mod ccmr1_output;
/**CCMR1_Input (rw) register accessor: capture/compare mode register 1 (input mode)

You can [`read`](crate::Reg::read) this register and get [`ccmr1_input::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr1_input::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM15:CCMR1_Input)

For information about available fields see [`mod@ccmr1_input`] module*/
#[doc(alias = "CCMR1_Input")]
pub type CCMR1_INPUT = crate::Reg<ccmr1_input::CCMR1_INPUTrs>;
///capture/compare mode register 1 (input mode)
pub mod ccmr1_input;
/**_CCER (rw) register accessor: TIM15 capture/compare enable register

You can [`read`](crate::Reg::read) this register and get [`_ccer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_ccer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM15:_CCER)

For information about available fields see [`mod@_ccer`] module*/
pub type _CCER = crate::Reg<_ccer::_CCERrs>;
///TIM15 capture/compare enable register
pub mod _ccer;
/**_CNT (rw) register accessor: TIM15 counter

You can [`read`](crate::Reg::read) this register and get [`_cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM15:_CNT)

For information about available fields see [`mod@_cnt`] module*/
pub type _CNT = crate::Reg<_cnt::_CNTrs>;
///TIM15 counter
pub mod _cnt;
/**_PSC (rw) register accessor: TIM15 prescaler

You can [`read`](crate::Reg::read) this register and get [`_psc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_psc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM15:_PSC)

For information about available fields see [`mod@_psc`] module*/
pub type _PSC = crate::Reg<_psc::_PSCrs>;
///TIM15 prescaler
pub mod _psc;
/**_ARR (rw) register accessor: TIM15 auto-reload register

You can [`read`](crate::Reg::read) this register and get [`_arr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_arr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM15:_ARR)

For information about available fields see [`mod@_arr`] module*/
pub type _ARR = crate::Reg<_arr::_ARRrs>;
///TIM15 auto-reload register
pub mod _arr;
/**_RCR (rw) register accessor: TIM15 repetition counter register

You can [`read`](crate::Reg::read) this register and get [`_rcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_rcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM15:_RCR)

For information about available fields see [`mod@_rcr`] module*/
pub type _RCR = crate::Reg<_rcr::_RCRrs>;
///TIM15 repetition counter register
pub mod _rcr;
/**_CCR1 (rw) register accessor: TIM15 capture/compare register 1

You can [`read`](crate::Reg::read) this register and get [`_ccr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_ccr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM15:_CCR1)

For information about available fields see [`mod@_ccr1`] module*/
pub type _CCR1 = crate::Reg<_ccr1::_CCR1rs>;
///TIM15 capture/compare register 1
pub mod _ccr1;
/**_CCR2 (rw) register accessor: TIM15 capture/compare register 2

You can [`read`](crate::Reg::read) this register and get [`_ccr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_ccr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM15:_CCR2)

For information about available fields see [`mod@_ccr2`] module*/
pub type _CCR2 = crate::Reg<_ccr2::_CCR2rs>;
///TIM15 capture/compare register 2
pub mod _ccr2;
/**BDTR (rw) register accessor: As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\[3:0\], BKF\[3:0\], AOE, BKP, BKE, OSSI, OSSR and DTG\[7:0\] can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register.

You can [`read`](crate::Reg::read) this register and get [`bdtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM15:BDTR)

For information about available fields see [`mod@bdtr`] module*/
pub type BDTR = crate::Reg<bdtr::BDTRrs>;
///As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\[3:0\], BKF\[3:0\], AOE, BKP, BKE, OSSI, OSSR and DTG\[7:0\] can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register.
pub mod bdtr;
/**_DCR (rw) register accessor: TIM15 DMA control register

You can [`read`](crate::Reg::read) this register and get [`_dcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_dcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM15:_DCR)

For information about available fields see [`mod@_dcr`] module*/
pub type _DCR = crate::Reg<_dcr::_DCRrs>;
///TIM15 DMA control register
pub mod _dcr;
/**_DMAR (rw) register accessor: TIM15 DMA address for full transfer

You can [`read`](crate::Reg::read) this register and get [`_dmar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_dmar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM15:_DMAR)

For information about available fields see [`mod@_dmar`] module*/
pub type _DMAR = crate::Reg<_dmar::_DMARrs>;
///TIM15 DMA address for full transfer
pub mod _dmar;
/**_AF1 (rw) register accessor: TIM15 alternate register 1

You can [`read`](crate::Reg::read) this register and get [`_af1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_af1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM15:_AF1)

For information about available fields see [`mod@_af1`] module*/
pub type _AF1 = crate::Reg<_af1::_AF1rs>;
///TIM15 alternate register 1
pub mod _af1;
/**_TISEL (rw) register accessor: TIM15 input selection register

You can [`read`](crate::Reg::read) this register and get [`_tisel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_tisel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM15:_TISEL)

For information about available fields see [`mod@_tisel`] module*/
pub type _TISEL = crate::Reg<_tisel::_TISELrs>;
///TIM15 input selection register
pub mod _tisel;
