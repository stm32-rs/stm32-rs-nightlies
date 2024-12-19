#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    _reserved_0_lptim2_isr: [u8; 0x04],
    _reserved_1_lptim2_icr: [u8; 0x04],
    _reserved_2_lptim2_dier: [u8; 0x04],
    lptim2_cfgr: LPTIM2_CFGR,
    lptim2_cr: LPTIM2_CR,
    lptim2_ccr1: LPTIM2_CCR1,
    lptim2_arr: LPTIM2_ARR,
    lptim2_cnt: LPTIM2_CNT,
    _reserved8: [u8; 0x04],
    lptim2_cfgr2: LPTIM2_CFGR2,
    lptim2_rcr: LPTIM2_RCR,
    lptim2_ccmr1: LPTIM2_CCMR1,
    lptim2_ccmr2: LPTIM2_CCMR2,
    lptim2_ccr2: LPTIM2_CCR2,
    lptim2_ccr3: LPTIM2_CCR3,
    lptim2_ccr4: LPTIM2_CCR4,
}
impl RegisterBlock {
    ///0x00 - LPTIM2 interrupt and status register \[alternate\]
    #[inline(always)]
    pub const fn lptim2_isr_input(&self) -> &LPTIM2_ISR_INPUT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    ///0x00 - LPTIM2 interrupt and status register \[alternate\]
    #[inline(always)]
    pub const fn lptim2_isr_output(&self) -> &LPTIM2_ISR_OUTPUT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    ///0x04 - LPTIM2 interrupt clear register \[alternate\]
    #[inline(always)]
    pub const fn lptim2_icr_input(&self) -> &LPTIM2_ICR_INPUT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    ///0x04 - LPTIM2 interrupt clear register \[alternate\]
    #[inline(always)]
    pub const fn lptim2_icr_output(&self) -> &LPTIM2_ICR_OUTPUT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    ///0x08 - LPTIM2 interrupt enable register \[alternate\]
    #[inline(always)]
    pub const fn lptim2_dier_input(&self) -> &LPTIM2_DIER_INPUT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).cast() }
    }
    ///0x08 - LPTIM2 interrupt enable register \[alternate\]
    #[inline(always)]
    pub const fn lptim2_dier_output(&self) -> &LPTIM2_DIER_OUTPUT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).cast() }
    }
    ///0x0c - LPTIM configuration register
    #[inline(always)]
    pub const fn lptim2_cfgr(&self) -> &LPTIM2_CFGR {
        &self.lptim2_cfgr
    }
    ///0x10 - LPTIM control register
    #[inline(always)]
    pub const fn lptim2_cr(&self) -> &LPTIM2_CR {
        &self.lptim2_cr
    }
    ///0x14 - LPTIM compare register 1
    #[inline(always)]
    pub const fn lptim2_ccr1(&self) -> &LPTIM2_CCR1 {
        &self.lptim2_ccr1
    }
    ///0x18 - LPTIM autoreload register
    #[inline(always)]
    pub const fn lptim2_arr(&self) -> &LPTIM2_ARR {
        &self.lptim2_arr
    }
    ///0x1c - LPTIM counter register
    #[inline(always)]
    pub const fn lptim2_cnt(&self) -> &LPTIM2_CNT {
        &self.lptim2_cnt
    }
    ///0x24 - LPTIM configuration register 2
    #[inline(always)]
    pub const fn lptim2_cfgr2(&self) -> &LPTIM2_CFGR2 {
        &self.lptim2_cfgr2
    }
    ///0x28 - LPTIM repetition register
    #[inline(always)]
    pub const fn lptim2_rcr(&self) -> &LPTIM2_RCR {
        &self.lptim2_rcr
    }
    ///0x2c - LPTIM capture/compare mode register 1
    #[inline(always)]
    pub const fn lptim2_ccmr1(&self) -> &LPTIM2_CCMR1 {
        &self.lptim2_ccmr1
    }
    ///0x30 - LPTIM capture/compare mode register 2
    #[inline(always)]
    pub const fn lptim2_ccmr2(&self) -> &LPTIM2_CCMR2 {
        &self.lptim2_ccmr2
    }
    ///0x34 - LPTIM compare register 2
    #[inline(always)]
    pub const fn lptim2_ccr2(&self) -> &LPTIM2_CCR2 {
        &self.lptim2_ccr2
    }
    ///0x38 - LPTIM compare register 3
    #[inline(always)]
    pub const fn lptim2_ccr3(&self) -> &LPTIM2_CCR3 {
        &self.lptim2_ccr3
    }
    ///0x3c - LPTIM compare register 4
    #[inline(always)]
    pub const fn lptim2_ccr4(&self) -> &LPTIM2_CCR4 {
        &self.lptim2_ccr4
    }
}
/**LPTIM2_ISR_OUTPUT (r) register accessor: LPTIM2 interrupt and status register \[alternate\]

You can [`read`](crate::Reg::read) this register and get [`lptim2_isr_output::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#LPTIM2:LPTIM2_ISR_OUTPUT)

For information about available fields see [`mod@lptim2_isr_output`]
module*/
pub type LPTIM2_ISR_OUTPUT = crate::Reg<lptim2_isr_output::LPTIM2_ISR_OUTPUTrs>;
///LPTIM2 interrupt and status register \[alternate\]
pub mod lptim2_isr_output;
/**LPTIM2_ISR_INPUT (r) register accessor: LPTIM2 interrupt and status register \[alternate\]

You can [`read`](crate::Reg::read) this register and get [`lptim2_isr_input::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#LPTIM2:LPTIM2_ISR_INPUT)

For information about available fields see [`mod@lptim2_isr_input`]
module*/
pub type LPTIM2_ISR_INPUT = crate::Reg<lptim2_isr_input::LPTIM2_ISR_INPUTrs>;
///LPTIM2 interrupt and status register \[alternate\]
pub mod lptim2_isr_input;
/**LPTIM2_ICR_OUTPUT (w) register accessor: LPTIM2 interrupt clear register \[alternate\]

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptim2_icr_output::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#LPTIM2:LPTIM2_ICR_OUTPUT)

For information about available fields see [`mod@lptim2_icr_output`]
module*/
pub type LPTIM2_ICR_OUTPUT = crate::Reg<lptim2_icr_output::LPTIM2_ICR_OUTPUTrs>;
///LPTIM2 interrupt clear register \[alternate\]
pub mod lptim2_icr_output;
/**LPTIM2_ICR_INPUT (w) register accessor: LPTIM2 interrupt clear register \[alternate\]

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptim2_icr_input::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#LPTIM2:LPTIM2_ICR_INPUT)

For information about available fields see [`mod@lptim2_icr_input`]
module*/
pub type LPTIM2_ICR_INPUT = crate::Reg<lptim2_icr_input::LPTIM2_ICR_INPUTrs>;
///LPTIM2 interrupt clear register \[alternate\]
pub mod lptim2_icr_input;
/**LPTIM2_DIER_OUTPUT (rw) register accessor: LPTIM2 interrupt enable register \[alternate\]

You can [`read`](crate::Reg::read) this register and get [`lptim2_dier_output::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptim2_dier_output::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#LPTIM2:LPTIM2_DIER_OUTPUT)

For information about available fields see [`mod@lptim2_dier_output`]
module*/
pub type LPTIM2_DIER_OUTPUT = crate::Reg<lptim2_dier_output::LPTIM2_DIER_OUTPUTrs>;
///LPTIM2 interrupt enable register \[alternate\]
pub mod lptim2_dier_output;
/**LPTIM2_DIER_INPUT (rw) register accessor: LPTIM2 interrupt enable register \[alternate\]

You can [`read`](crate::Reg::read) this register and get [`lptim2_dier_input::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptim2_dier_input::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#LPTIM2:LPTIM2_DIER_INPUT)

For information about available fields see [`mod@lptim2_dier_input`]
module*/
pub type LPTIM2_DIER_INPUT = crate::Reg<lptim2_dier_input::LPTIM2_DIER_INPUTrs>;
///LPTIM2 interrupt enable register \[alternate\]
pub mod lptim2_dier_input;
/**LPTIM2_CFGR (rw) register accessor: LPTIM configuration register

You can [`read`](crate::Reg::read) this register and get [`lptim2_cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptim2_cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#LPTIM2:LPTIM2_CFGR)

For information about available fields see [`mod@lptim2_cfgr`]
module*/
pub type LPTIM2_CFGR = crate::Reg<lptim2_cfgr::LPTIM2_CFGRrs>;
///LPTIM configuration register
pub mod lptim2_cfgr;
/**LPTIM2_CR (rw) register accessor: LPTIM control register

You can [`read`](crate::Reg::read) this register and get [`lptim2_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptim2_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#LPTIM2:LPTIM2_CR)

For information about available fields see [`mod@lptim2_cr`]
module*/
pub type LPTIM2_CR = crate::Reg<lptim2_cr::LPTIM2_CRrs>;
///LPTIM control register
pub mod lptim2_cr;
/**LPTIM2_CCR1 (rw) register accessor: LPTIM compare register 1

You can [`read`](crate::Reg::read) this register and get [`lptim2_ccr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptim2_ccr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#LPTIM2:LPTIM2_CCR1)

For information about available fields see [`mod@lptim2_ccr1`]
module*/
pub type LPTIM2_CCR1 = crate::Reg<lptim2_ccr1::LPTIM2_CCR1rs>;
///LPTIM compare register 1
pub mod lptim2_ccr1;
/**LPTIM2_ARR (rw) register accessor: LPTIM autoreload register

You can [`read`](crate::Reg::read) this register and get [`lptim2_arr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptim2_arr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#LPTIM2:LPTIM2_ARR)

For information about available fields see [`mod@lptim2_arr`]
module*/
pub type LPTIM2_ARR = crate::Reg<lptim2_arr::LPTIM2_ARRrs>;
///LPTIM autoreload register
pub mod lptim2_arr;
/**LPTIM2_CNT (r) register accessor: LPTIM counter register

You can [`read`](crate::Reg::read) this register and get [`lptim2_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#LPTIM2:LPTIM2_CNT)

For information about available fields see [`mod@lptim2_cnt`]
module*/
pub type LPTIM2_CNT = crate::Reg<lptim2_cnt::LPTIM2_CNTrs>;
///LPTIM counter register
pub mod lptim2_cnt;
/**LPTIM2_CFGR2 (rw) register accessor: LPTIM configuration register 2

You can [`read`](crate::Reg::read) this register and get [`lptim2_cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptim2_cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#LPTIM2:LPTIM2_CFGR2)

For information about available fields see [`mod@lptim2_cfgr2`]
module*/
pub type LPTIM2_CFGR2 = crate::Reg<lptim2_cfgr2::LPTIM2_CFGR2rs>;
///LPTIM configuration register 2
pub mod lptim2_cfgr2;
/**LPTIM2_RCR (rw) register accessor: LPTIM repetition register

You can [`read`](crate::Reg::read) this register and get [`lptim2_rcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptim2_rcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#LPTIM2:LPTIM2_RCR)

For information about available fields see [`mod@lptim2_rcr`]
module*/
pub type LPTIM2_RCR = crate::Reg<lptim2_rcr::LPTIM2_RCRrs>;
///LPTIM repetition register
pub mod lptim2_rcr;
/**LPTIM2_CCMR1 (rw) register accessor: LPTIM capture/compare mode register 1

You can [`read`](crate::Reg::read) this register and get [`lptim2_ccmr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptim2_ccmr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#LPTIM2:LPTIM2_CCMR1)

For information about available fields see [`mod@lptim2_ccmr1`]
module*/
pub type LPTIM2_CCMR1 = crate::Reg<lptim2_ccmr1::LPTIM2_CCMR1rs>;
///LPTIM capture/compare mode register 1
pub mod lptim2_ccmr1;
/**LPTIM2_CCMR2 (rw) register accessor: LPTIM capture/compare mode register 2

You can [`read`](crate::Reg::read) this register and get [`lptim2_ccmr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptim2_ccmr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#LPTIM2:LPTIM2_CCMR2)

For information about available fields see [`mod@lptim2_ccmr2`]
module*/
pub type LPTIM2_CCMR2 = crate::Reg<lptim2_ccmr2::LPTIM2_CCMR2rs>;
///LPTIM capture/compare mode register 2
pub mod lptim2_ccmr2;
/**LPTIM2_CCR2 (rw) register accessor: LPTIM compare register 2

You can [`read`](crate::Reg::read) this register and get [`lptim2_ccr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptim2_ccr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#LPTIM2:LPTIM2_CCR2)

For information about available fields see [`mod@lptim2_ccr2`]
module*/
pub type LPTIM2_CCR2 = crate::Reg<lptim2_ccr2::LPTIM2_CCR2rs>;
///LPTIM compare register 2
pub mod lptim2_ccr2;
/**LPTIM2_CCR3 (rw) register accessor: LPTIM compare register 3

You can [`read`](crate::Reg::read) this register and get [`lptim2_ccr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptim2_ccr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#LPTIM2:LPTIM2_CCR3)

For information about available fields see [`mod@lptim2_ccr3`]
module*/
pub type LPTIM2_CCR3 = crate::Reg<lptim2_ccr3::LPTIM2_CCR3rs>;
///LPTIM compare register 3
pub mod lptim2_ccr3;
/**LPTIM2_CCR4 (rw) register accessor: LPTIM compare register 4

You can [`read`](crate::Reg::read) this register and get [`lptim2_ccr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptim2_ccr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#LPTIM2:LPTIM2_CCR4)

For information about available fields see [`mod@lptim2_ccr4`]
module*/
pub type LPTIM2_CCR4 = crate::Reg<lptim2_ccr4::LPTIM2_CCR4rs>;
///LPTIM compare register 4
pub mod lptim2_ccr4;
