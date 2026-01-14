#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    _reserved_0_isr: [u8; 0x04],
    _reserved_1_icr: [u8; 0x04],
    _reserved_2_dier: [u8; 0x04],
}
impl RegisterBlock {
    ///0x00 - Interrupt and Status Register (intput mode)
    #[inline(always)]
    pub const fn isr_intput(&self) -> &ISR_INTPUT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    ///0x00 - LPTIM1 interrupt and status register \[alternate\]
    #[inline(always)]
    pub const fn isr_output(&self) -> &ISR_OUTPUT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    ///0x04 - Interrupt Clear Register (intput mode)
    #[inline(always)]
    pub const fn icr_intput(&self) -> &ICR_INTPUT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    ///0x04 - LPTIM1 interrupt clear register \[alternate\]
    #[inline(always)]
    pub const fn icr_output(&self) -> &ICR_OUTPUT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    ///0x08 - LPTIM interrupt Enable Register (intput mode)
    #[inline(always)]
    pub const fn dier_intput(&self) -> &DIER_INTPUT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).cast() }
    }
    ///0x08 - LPTIM1 interrupt enable register \[alternate\]
    #[inline(always)]
    pub const fn dier_output(&self) -> &DIER_OUTPUT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).cast() }
    }
}
/**ISR_output (r) register accessor: LPTIM1 interrupt and status register \[alternate\]

You can [`read`](crate::Reg::read) this register and get [`isr_output::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#LPTIM1:ISR_output)

For information about available fields see [`mod@isr_output`] module*/
#[doc(alias = "ISR_output")]
pub type ISR_OUTPUT = crate::Reg<isr_output::ISR_OUTPUTrs>;
///LPTIM1 interrupt and status register \[alternate\]
pub mod isr_output;
/**ISR_intput (r) register accessor: Interrupt and Status Register (intput mode)

You can [`read`](crate::Reg::read) this register and get [`isr_intput::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#LPTIM1:ISR_intput)

For information about available fields see [`mod@isr_intput`] module*/
#[doc(alias = "ISR_intput")]
pub type ISR_INTPUT = crate::Reg<isr_intput::ISR_INTPUTrs>;
///Interrupt and Status Register (intput mode)
pub mod isr_intput;
/**ICR_output (w) register accessor: LPTIM1 interrupt clear register \[alternate\]

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr_output::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#LPTIM1:ICR_output)

For information about available fields see [`mod@icr_output`] module*/
#[doc(alias = "ICR_output")]
pub type ICR_OUTPUT = crate::Reg<icr_output::ICR_OUTPUTrs>;
///LPTIM1 interrupt clear register \[alternate\]
pub mod icr_output;
/**ICR_intput (w) register accessor: Interrupt Clear Register (intput mode)

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr_intput::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#LPTIM1:ICR_intput)

For information about available fields see [`mod@icr_intput`] module*/
#[doc(alias = "ICR_intput")]
pub type ICR_INTPUT = crate::Reg<icr_intput::ICR_INTPUTrs>;
///Interrupt Clear Register (intput mode)
pub mod icr_intput;
/**DIER_output (rw) register accessor: LPTIM1 interrupt enable register \[alternate\]

You can [`read`](crate::Reg::read) this register and get [`dier_output::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dier_output::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#LPTIM1:DIER_output)

For information about available fields see [`mod@dier_output`] module*/
#[doc(alias = "DIER_output")]
pub type DIER_OUTPUT = crate::Reg<dier_output::DIER_OUTPUTrs>;
///LPTIM1 interrupt enable register \[alternate\]
pub mod dier_output;
/**DIER_intput (rw) register accessor: LPTIM interrupt Enable Register (intput mode)

You can [`read`](crate::Reg::read) this register and get [`dier_intput::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dier_intput::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#LPTIM1:DIER_intput)

For information about available fields see [`mod@dier_intput`] module*/
#[doc(alias = "DIER_intput")]
pub type DIER_INTPUT = crate::Reg<dier_intput::DIER_INTPUTrs>;
///LPTIM interrupt Enable Register (intput mode)
pub mod dier_intput;
