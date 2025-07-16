#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    _reserved_0_isr: [u8; 0x04],
    _reserved_1_icr: [u8; 0x04],
    _reserved_2_dier: [u8; 0x04],
}
impl RegisterBlock {
    ///0x00 - LPTIM1 interrupt and status register
    #[inline(always)]
    pub const fn isr_input(&self) -> &ISR_INPUT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    ///0x00 - LPTIM1 interrupt and status register
    #[inline(always)]
    pub const fn isr_output(&self) -> &ISR_OUTPUT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    ///0x04 - LPTIM1 interrupt clear register
    #[inline(always)]
    pub const fn icr_input(&self) -> &ICR_INPUT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    ///0x04 - LPTIM1 interrupt clear register
    #[inline(always)]
    pub const fn icr_output(&self) -> &ICR_OUTPUT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    ///0x08 - LPTIM1 interrupt enable register
    #[inline(always)]
    pub const fn dier_input(&self) -> &DIER_INPUT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).cast() }
    }
    ///0x08 - LPTIM1 interrupt enable register
    #[inline(always)]
    pub const fn dier_output(&self) -> &DIER_OUTPUT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).cast() }
    }
}
/**ISR_OUTPUT (r) register accessor: LPTIM1 interrupt and status register

You can [`read`](crate::Reg::read) this register and get [`isr_output::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#LPTIM1:ISR_OUTPUT)

For information about available fields see [`mod@isr_output`] module*/
pub type ISR_OUTPUT = crate::Reg<isr_output::ISR_OUTPUTrs>;
///LPTIM1 interrupt and status register
pub mod isr_output;
/**ISR_INPUT (r) register accessor: LPTIM1 interrupt and status register

You can [`read`](crate::Reg::read) this register and get [`isr_input::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#LPTIM1:ISR_INPUT)

For information about available fields see [`mod@isr_input`] module*/
pub type ISR_INPUT = crate::Reg<isr_input::ISR_INPUTrs>;
///LPTIM1 interrupt and status register
pub mod isr_input;
/**ICR_OUTPUT (w) register accessor: LPTIM1 interrupt clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr_output::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#LPTIM1:ICR_OUTPUT)

For information about available fields see [`mod@icr_output`] module*/
pub type ICR_OUTPUT = crate::Reg<icr_output::ICR_OUTPUTrs>;
///LPTIM1 interrupt clear register
pub mod icr_output;
/**ICR_INPUT (w) register accessor: LPTIM1 interrupt clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr_input::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#LPTIM1:ICR_INPUT)

For information about available fields see [`mod@icr_input`] module*/
pub type ICR_INPUT = crate::Reg<icr_input::ICR_INPUTrs>;
///LPTIM1 interrupt clear register
pub mod icr_input;
/**DIER_OUTPUT (rw) register accessor: LPTIM1 interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`dier_output::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dier_output::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#LPTIM1:DIER_OUTPUT)

For information about available fields see [`mod@dier_output`] module*/
pub type DIER_OUTPUT = crate::Reg<dier_output::DIER_OUTPUTrs>;
///LPTIM1 interrupt enable register
pub mod dier_output;
/**DIER_INPUT (rw) register accessor: LPTIM1 interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`dier_input::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dier_input::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#LPTIM1:DIER_INPUT)

For information about available fields see [`mod@dier_input`] module*/
pub type DIER_INPUT = crate::Reg<dier_input::DIER_INPUTrs>;
///LPTIM1 interrupt enable register
pub mod dier_input;
