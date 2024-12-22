#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    _reserved_0_lptim3_isr: [u8; 0x04],
    _reserved_1_lptim3_icr: [u8; 0x04],
    _reserved_2_lptim3_dier: [u8; 0x04],
    lptim3_cfgr: LPTIM3_CFGR,
    lptim3_cr: LPTIM3_CR,
    lptim3_ccr1: LPTIM3_CCR1,
    lptim3_arr: LPTIM3_ARR,
    lptim3_cnt: LPTIM3_CNT,
    _reserved8: [u8; 0x04],
    lptim3_cfgr2: LPTIM3_CFGR2,
    lptim3_rcr: LPTIM3_RCR,
    lptim3_ccmr1: LPTIM3_CCMR1,
    lptim3_ccmr2: LPTIM3_CCMR2,
    lptim3_ccr2: LPTIM3_CCR2,
    lptim3_ccr3: LPTIM3_CCR3,
    lptim3_ccr4: LPTIM3_CCR4,
}
impl RegisterBlock {
    ///0x00 - LPTIM3 interrupt and status register \[alternate\]
    #[inline(always)]
    pub const fn lptim3_isr_input(&self) -> &LPTIM3_ISR_INPUT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    ///0x00 - LPTIM3 interrupt and status register \[alternate\]
    #[inline(always)]
    pub const fn lptim3_isr_output(&self) -> &LPTIM3_ISR_OUTPUT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    ///0x04 - LPTIM3 interrupt clear register \[alternate\]
    #[inline(always)]
    pub const fn lptim3_icr_input(&self) -> &LPTIM3_ICR_INPUT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    ///0x04 - LPTIM3 interrupt clear register \[alternate\]
    #[inline(always)]
    pub const fn lptim3_icr_output(&self) -> &LPTIM3_ICR_OUTPUT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    ///0x08 - LPTIM3 interrupt enable register \[alternate\]
    #[inline(always)]
    pub const fn lptim3_dier_input(&self) -> &LPTIM3_DIER_INPUT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).cast() }
    }
    ///0x08 - LPTIM3 interrupt enable register \[alternate\]
    #[inline(always)]
    pub const fn lptim3_dier_output(&self) -> &LPTIM3_DIER_OUTPUT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).cast() }
    }
    ///0x0c - LPTIM configuration register
    #[inline(always)]
    pub const fn lptim3_cfgr(&self) -> &LPTIM3_CFGR {
        &self.lptim3_cfgr
    }
    ///0x10 - LPTIM control register
    #[inline(always)]
    pub const fn lptim3_cr(&self) -> &LPTIM3_CR {
        &self.lptim3_cr
    }
    ///0x14 - LPTIM compare register 1
    #[inline(always)]
    pub const fn lptim3_ccr1(&self) -> &LPTIM3_CCR1 {
        &self.lptim3_ccr1
    }
    ///0x18 - LPTIM autoreload register
    #[inline(always)]
    pub const fn lptim3_arr(&self) -> &LPTIM3_ARR {
        &self.lptim3_arr
    }
    ///0x1c - LPTIM counter register
    #[inline(always)]
    pub const fn lptim3_cnt(&self) -> &LPTIM3_CNT {
        &self.lptim3_cnt
    }
    ///0x24 - LPTIM configuration register 2
    #[inline(always)]
    pub const fn lptim3_cfgr2(&self) -> &LPTIM3_CFGR2 {
        &self.lptim3_cfgr2
    }
    ///0x28 - LPTIM repetition register
    #[inline(always)]
    pub const fn lptim3_rcr(&self) -> &LPTIM3_RCR {
        &self.lptim3_rcr
    }
    ///0x2c - LPTIM capture/compare mode register 1
    #[inline(always)]
    pub const fn lptim3_ccmr1(&self) -> &LPTIM3_CCMR1 {
        &self.lptim3_ccmr1
    }
    ///0x30 - LPTIM capture/compare mode register 2
    #[inline(always)]
    pub const fn lptim3_ccmr2(&self) -> &LPTIM3_CCMR2 {
        &self.lptim3_ccmr2
    }
    ///0x34 - LPTIM compare register 2
    #[inline(always)]
    pub const fn lptim3_ccr2(&self) -> &LPTIM3_CCR2 {
        &self.lptim3_ccr2
    }
    ///0x38 - LPTIM compare register 3
    #[inline(always)]
    pub const fn lptim3_ccr3(&self) -> &LPTIM3_CCR3 {
        &self.lptim3_ccr3
    }
    ///0x3c - LPTIM compare register 4
    #[inline(always)]
    pub const fn lptim3_ccr4(&self) -> &LPTIM3_CCR4 {
        &self.lptim3_ccr4
    }
}
/**LPTIM3_ISR_OUTPUT (r) register accessor: LPTIM3 interrupt and status register \[alternate\]

You can [`read`](crate::Reg::read) this register and get [`lptim3_isr_output::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#LPTIM3:LPTIM3_ISR_OUTPUT)

For information about available fields see [`mod@lptim3_isr_output`]
module*/
pub type LPTIM3_ISR_OUTPUT = crate::Reg<lptim3_isr_output::LPTIM3_ISR_OUTPUTrs>;
///LPTIM3 interrupt and status register \[alternate\]
pub mod lptim3_isr_output;
/**LPTIM3_ISR_INPUT (r) register accessor: LPTIM3 interrupt and status register \[alternate\]

You can [`read`](crate::Reg::read) this register and get [`lptim3_isr_input::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#LPTIM3:LPTIM3_ISR_INPUT)

For information about available fields see [`mod@lptim3_isr_input`]
module*/
pub type LPTIM3_ISR_INPUT = crate::Reg<lptim3_isr_input::LPTIM3_ISR_INPUTrs>;
///LPTIM3 interrupt and status register \[alternate\]
pub mod lptim3_isr_input;
/**LPTIM3_ICR_OUTPUT (w) register accessor: LPTIM3 interrupt clear register \[alternate\]

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptim3_icr_output::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#LPTIM3:LPTIM3_ICR_OUTPUT)

For information about available fields see [`mod@lptim3_icr_output`]
module*/
pub type LPTIM3_ICR_OUTPUT = crate::Reg<lptim3_icr_output::LPTIM3_ICR_OUTPUTrs>;
///LPTIM3 interrupt clear register \[alternate\]
pub mod lptim3_icr_output;
/**LPTIM3_ICR_INPUT (w) register accessor: LPTIM3 interrupt clear register \[alternate\]

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptim3_icr_input::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#LPTIM3:LPTIM3_ICR_INPUT)

For information about available fields see [`mod@lptim3_icr_input`]
module*/
pub type LPTIM3_ICR_INPUT = crate::Reg<lptim3_icr_input::LPTIM3_ICR_INPUTrs>;
///LPTIM3 interrupt clear register \[alternate\]
pub mod lptim3_icr_input;
/**LPTIM3_DIER_OUTPUT (rw) register accessor: LPTIM3 interrupt enable register \[alternate\]

You can [`read`](crate::Reg::read) this register and get [`lptim3_dier_output::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptim3_dier_output::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#LPTIM3:LPTIM3_DIER_OUTPUT)

For information about available fields see [`mod@lptim3_dier_output`]
module*/
pub type LPTIM3_DIER_OUTPUT = crate::Reg<lptim3_dier_output::LPTIM3_DIER_OUTPUTrs>;
///LPTIM3 interrupt enable register \[alternate\]
pub mod lptim3_dier_output;
/**LPTIM3_DIER_INPUT (rw) register accessor: LPTIM3 interrupt enable register \[alternate\]

You can [`read`](crate::Reg::read) this register and get [`lptim3_dier_input::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptim3_dier_input::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#LPTIM3:LPTIM3_DIER_INPUT)

For information about available fields see [`mod@lptim3_dier_input`]
module*/
pub type LPTIM3_DIER_INPUT = crate::Reg<lptim3_dier_input::LPTIM3_DIER_INPUTrs>;
///LPTIM3 interrupt enable register \[alternate\]
pub mod lptim3_dier_input;
/**LPTIM3_CFGR (rw) register accessor: LPTIM configuration register

You can [`read`](crate::Reg::read) this register and get [`lptim3_cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptim3_cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#LPTIM3:LPTIM3_CFGR)

For information about available fields see [`mod@lptim3_cfgr`]
module*/
pub type LPTIM3_CFGR = crate::Reg<lptim3_cfgr::LPTIM3_CFGRrs>;
///LPTIM configuration register
pub mod lptim3_cfgr;
/**LPTIM3_CR (rw) register accessor: LPTIM control register

You can [`read`](crate::Reg::read) this register and get [`lptim3_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptim3_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#LPTIM3:LPTIM3_CR)

For information about available fields see [`mod@lptim3_cr`]
module*/
pub type LPTIM3_CR = crate::Reg<lptim3_cr::LPTIM3_CRrs>;
///LPTIM control register
pub mod lptim3_cr;
/**LPTIM3_CCR1 (rw) register accessor: LPTIM compare register 1

You can [`read`](crate::Reg::read) this register and get [`lptim3_ccr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptim3_ccr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#LPTIM3:LPTIM3_CCR1)

For information about available fields see [`mod@lptim3_ccr1`]
module*/
pub type LPTIM3_CCR1 = crate::Reg<lptim3_ccr1::LPTIM3_CCR1rs>;
///LPTIM compare register 1
pub mod lptim3_ccr1;
/**LPTIM3_ARR (rw) register accessor: LPTIM autoreload register

You can [`read`](crate::Reg::read) this register and get [`lptim3_arr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptim3_arr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#LPTIM3:LPTIM3_ARR)

For information about available fields see [`mod@lptim3_arr`]
module*/
pub type LPTIM3_ARR = crate::Reg<lptim3_arr::LPTIM3_ARRrs>;
///LPTIM autoreload register
pub mod lptim3_arr;
/**LPTIM3_CNT (r) register accessor: LPTIM counter register

You can [`read`](crate::Reg::read) this register and get [`lptim3_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#LPTIM3:LPTIM3_CNT)

For information about available fields see [`mod@lptim3_cnt`]
module*/
pub type LPTIM3_CNT = crate::Reg<lptim3_cnt::LPTIM3_CNTrs>;
///LPTIM counter register
pub mod lptim3_cnt;
/**LPTIM3_CFGR2 (rw) register accessor: LPTIM configuration register 2

You can [`read`](crate::Reg::read) this register and get [`lptim3_cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptim3_cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#LPTIM3:LPTIM3_CFGR2)

For information about available fields see [`mod@lptim3_cfgr2`]
module*/
pub type LPTIM3_CFGR2 = crate::Reg<lptim3_cfgr2::LPTIM3_CFGR2rs>;
///LPTIM configuration register 2
pub mod lptim3_cfgr2;
/**LPTIM3_RCR (rw) register accessor: LPTIM repetition register

You can [`read`](crate::Reg::read) this register and get [`lptim3_rcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptim3_rcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#LPTIM3:LPTIM3_RCR)

For information about available fields see [`mod@lptim3_rcr`]
module*/
pub type LPTIM3_RCR = crate::Reg<lptim3_rcr::LPTIM3_RCRrs>;
///LPTIM repetition register
pub mod lptim3_rcr;
/**LPTIM3_CCMR1 (rw) register accessor: LPTIM capture/compare mode register 1

You can [`read`](crate::Reg::read) this register and get [`lptim3_ccmr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptim3_ccmr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#LPTIM3:LPTIM3_CCMR1)

For information about available fields see [`mod@lptim3_ccmr1`]
module*/
pub type LPTIM3_CCMR1 = crate::Reg<lptim3_ccmr1::LPTIM3_CCMR1rs>;
///LPTIM capture/compare mode register 1
pub mod lptim3_ccmr1;
/**LPTIM3_CCMR2 (rw) register accessor: LPTIM capture/compare mode register 2

You can [`read`](crate::Reg::read) this register and get [`lptim3_ccmr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptim3_ccmr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#LPTIM3:LPTIM3_CCMR2)

For information about available fields see [`mod@lptim3_ccmr2`]
module*/
pub type LPTIM3_CCMR2 = crate::Reg<lptim3_ccmr2::LPTIM3_CCMR2rs>;
///LPTIM capture/compare mode register 2
pub mod lptim3_ccmr2;
/**LPTIM3_CCR2 (rw) register accessor: LPTIM compare register 2

You can [`read`](crate::Reg::read) this register and get [`lptim3_ccr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptim3_ccr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#LPTIM3:LPTIM3_CCR2)

For information about available fields see [`mod@lptim3_ccr2`]
module*/
pub type LPTIM3_CCR2 = crate::Reg<lptim3_ccr2::LPTIM3_CCR2rs>;
///LPTIM compare register 2
pub mod lptim3_ccr2;
/**LPTIM3_CCR3 (rw) register accessor: LPTIM compare register 3

You can [`read`](crate::Reg::read) this register and get [`lptim3_ccr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptim3_ccr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#LPTIM3:LPTIM3_CCR3)

For information about available fields see [`mod@lptim3_ccr3`]
module*/
pub type LPTIM3_CCR3 = crate::Reg<lptim3_ccr3::LPTIM3_CCR3rs>;
///LPTIM compare register 3
pub mod lptim3_ccr3;
/**LPTIM3_CCR4 (rw) register accessor: LPTIM compare register 4

You can [`read`](crate::Reg::read) this register and get [`lptim3_ccr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptim3_ccr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#LPTIM3:LPTIM3_CCR4)

For information about available fields see [`mod@lptim3_ccr4`]
module*/
pub type LPTIM3_CCR4 = crate::Reg<lptim3_ccr4::LPTIM3_CCR4rs>;
///LPTIM compare register 4
pub mod lptim3_ccr4;
