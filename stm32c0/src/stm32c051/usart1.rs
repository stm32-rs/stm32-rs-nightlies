#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    _reserved_0_usart_cr: [u8; 0x04],
    usart_cr2: USART_CR2,
    usart_cr3: USART_CR3,
    usart_brr: USART_BRR,
    usart_gtpr: USART_GTPR,
    usart_rtor: USART_RTOR,
    usart_rqr: USART_RQR,
    _reserved_7_usart_: [u8; 0x04],
    usart_icr: USART_ICR,
    usart_rdr: USART_RDR,
    usart_tdr: USART_TDR,
    usart_presc: USART_PRESC,
}
impl RegisterBlock {
    ///0x00 - USART control register 1
    #[inline(always)]
    pub const fn usart_cr1_alternate1(&self) -> &USART_CR1_ALTERNATE1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    ///0x00 - USART control register 1
    #[inline(always)]
    pub const fn usart_cr1(&self) -> &USART_CR1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    ///0x04 - USART control register 2
    #[inline(always)]
    pub const fn usart_cr2(&self) -> &USART_CR2 {
        &self.usart_cr2
    }
    ///0x08 - USART control register 3
    #[inline(always)]
    pub const fn usart_cr3(&self) -> &USART_CR3 {
        &self.usart_cr3
    }
    ///0x0c - USART baud rate register
    #[inline(always)]
    pub const fn usart_brr(&self) -> &USART_BRR {
        &self.usart_brr
    }
    ///0x10 - USART guard time and prescaler register
    #[inline(always)]
    pub const fn usart_gtpr(&self) -> &USART_GTPR {
        &self.usart_gtpr
    }
    ///0x14 - USART receiver timeout register
    #[inline(always)]
    pub const fn usart_rtor(&self) -> &USART_RTOR {
        &self.usart_rtor
    }
    ///0x18 - USART request register
    #[inline(always)]
    pub const fn usart_rqr(&self) -> &USART_RQR {
        &self.usart_rqr
    }
    ///0x1c - USART interrupt and status register
    #[inline(always)]
    pub const fn usart_isr_alternate1(&self) -> &USART_ISR_ALTERNATE1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(28).cast() }
    }
    ///0x1c - USART interrupt and status register
    #[inline(always)]
    pub const fn usart_isr(&self) -> &USART_ISR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(28).cast() }
    }
    ///0x20 - USART interrupt flag clear register
    #[inline(always)]
    pub const fn usart_icr(&self) -> &USART_ICR {
        &self.usart_icr
    }
    ///0x24 - USART receive data register
    #[inline(always)]
    pub const fn usart_rdr(&self) -> &USART_RDR {
        &self.usart_rdr
    }
    ///0x28 - USART transmit data register
    #[inline(always)]
    pub const fn usart_tdr(&self) -> &USART_TDR {
        &self.usart_tdr
    }
    ///0x2c - USART prescaler register
    #[inline(always)]
    pub const fn usart_presc(&self) -> &USART_PRESC {
        &self.usart_presc
    }
}
/**USART_CR1 (rw) register accessor: USART control register 1

You can [`read`](crate::Reg::read) this register and get [`usart_cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usart_cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#USART1:USART_CR1)

For information about available fields see [`mod@usart_cr1`] module*/
pub type USART_CR1 = crate::Reg<usart_cr1::USART_CR1rs>;
///USART control register 1
pub mod usart_cr1;
/**USART_CR1_ALTERNATE1 (rw) register accessor: USART control register 1

You can [`read`](crate::Reg::read) this register and get [`usart_cr1_alternate1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usart_cr1_alternate1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#USART1:USART_CR1_ALTERNATE1)

For information about available fields see [`mod@usart_cr1_alternate1`] module*/
pub type USART_CR1_ALTERNATE1 = crate::Reg<usart_cr1_alternate1::USART_CR1_ALTERNATE1rs>;
///USART control register 1
pub mod usart_cr1_alternate1;
/**USART_CR2 (rw) register accessor: USART control register 2

You can [`read`](crate::Reg::read) this register and get [`usart_cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usart_cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#USART1:USART_CR2)

For information about available fields see [`mod@usart_cr2`] module*/
pub type USART_CR2 = crate::Reg<usart_cr2::USART_CR2rs>;
///USART control register 2
pub mod usart_cr2;
/**USART_CR3 (rw) register accessor: USART control register 3

You can [`read`](crate::Reg::read) this register and get [`usart_cr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usart_cr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#USART1:USART_CR3)

For information about available fields see [`mod@usart_cr3`] module*/
pub type USART_CR3 = crate::Reg<usart_cr3::USART_CR3rs>;
///USART control register 3
pub mod usart_cr3;
/**USART_BRR (rw) register accessor: USART baud rate register

You can [`read`](crate::Reg::read) this register and get [`usart_brr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usart_brr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#USART1:USART_BRR)

For information about available fields see [`mod@usart_brr`] module*/
pub type USART_BRR = crate::Reg<usart_brr::USART_BRRrs>;
///USART baud rate register
pub mod usart_brr;
/**USART_GTPR (rw) register accessor: USART guard time and prescaler register

You can [`read`](crate::Reg::read) this register and get [`usart_gtpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usart_gtpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#USART1:USART_GTPR)

For information about available fields see [`mod@usart_gtpr`] module*/
pub type USART_GTPR = crate::Reg<usart_gtpr::USART_GTPRrs>;
///USART guard time and prescaler register
pub mod usart_gtpr;
/**USART_RTOR (rw) register accessor: USART receiver timeout register

You can [`read`](crate::Reg::read) this register and get [`usart_rtor::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usart_rtor::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#USART1:USART_RTOR)

For information about available fields see [`mod@usart_rtor`] module*/
pub type USART_RTOR = crate::Reg<usart_rtor::USART_RTORrs>;
///USART receiver timeout register
pub mod usart_rtor;
/**USART_RQR (w) register accessor: USART request register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usart_rqr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#USART1:USART_RQR)

For information about available fields see [`mod@usart_rqr`] module*/
pub type USART_RQR = crate::Reg<usart_rqr::USART_RQRrs>;
///USART request register
pub mod usart_rqr;
/**USART_ISR (r) register accessor: USART interrupt and status register

You can [`read`](crate::Reg::read) this register and get [`usart_isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#USART1:USART_ISR)

For information about available fields see [`mod@usart_isr`] module*/
pub type USART_ISR = crate::Reg<usart_isr::USART_ISRrs>;
///USART interrupt and status register
pub mod usart_isr;
/**USART_ISR_ALTERNATE1 (r) register accessor: USART interrupt and status register

You can [`read`](crate::Reg::read) this register and get [`usart_isr_alternate1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#USART1:USART_ISR_ALTERNATE1)

For information about available fields see [`mod@usart_isr_alternate1`] module*/
pub type USART_ISR_ALTERNATE1 = crate::Reg<usart_isr_alternate1::USART_ISR_ALTERNATE1rs>;
///USART interrupt and status register
pub mod usart_isr_alternate1;
/**USART_ICR (w) register accessor: USART interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usart_icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#USART1:USART_ICR)

For information about available fields see [`mod@usart_icr`] module*/
pub type USART_ICR = crate::Reg<usart_icr::USART_ICRrs>;
///USART interrupt flag clear register
pub mod usart_icr;
/**USART_RDR (r) register accessor: USART receive data register

You can [`read`](crate::Reg::read) this register and get [`usart_rdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#USART1:USART_RDR)

For information about available fields see [`mod@usart_rdr`] module*/
pub type USART_RDR = crate::Reg<usart_rdr::USART_RDRrs>;
///USART receive data register
pub mod usart_rdr;
/**USART_TDR (rw) register accessor: USART transmit data register

You can [`read`](crate::Reg::read) this register and get [`usart_tdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usart_tdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#USART1:USART_TDR)

For information about available fields see [`mod@usart_tdr`] module*/
pub type USART_TDR = crate::Reg<usart_tdr::USART_TDRrs>;
///USART transmit data register
pub mod usart_tdr;
/**USART_PRESC (rw) register accessor: USART prescaler register

You can [`read`](crate::Reg::read) this register and get [`usart_presc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usart_presc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#USART1:USART_PRESC)

For information about available fields see [`mod@usart_presc`] module*/
pub type USART_PRESC = crate::Reg<usart_presc::USART_PRESCrs>;
///USART prescaler register
pub mod usart_presc;
