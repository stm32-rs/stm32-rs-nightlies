#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    tsc_cr: TSC_CR,
    tsc_ier: TSC_IER,
    tsc_icr: TSC_ICR,
    tsc_isr: TSC_ISR,
    tsc_iohcr: TSC_IOHCR,
    _reserved5: [u8; 0x04],
    tsc_ioascr: TSC_IOASCR,
    _reserved6: [u8; 0x04],
    tsc_ioscr: TSC_IOSCR,
    _reserved7: [u8; 0x04],
    tsc_ioccr: TSC_IOCCR,
    _reserved8: [u8; 0x04],
    tsc_iogcsr: TSC_IOGCSR,
    tsc_iog1cr: TSC_IOG1CR,
    tsc_iog2cr: TSC_IOG2CR,
    tsc_iog3cr: TSC_IOG3CR,
    tsc_iog4cr: TSC_IOG4CR,
    tsc_iog5cr: TSC_IOG5CR,
    tsc_iog6cr: TSC_IOG6CR,
    tsc_iog7cr: TSC_IOG7CR,
}
impl RegisterBlock {
    ///0x00 - TSC control register
    #[inline(always)]
    pub const fn tsc_cr(&self) -> &TSC_CR {
        &self.tsc_cr
    }
    ///0x04 - TSC interrupt enable register
    #[inline(always)]
    pub const fn tsc_ier(&self) -> &TSC_IER {
        &self.tsc_ier
    }
    ///0x08 - TSC interrupt clear register
    #[inline(always)]
    pub const fn tsc_icr(&self) -> &TSC_ICR {
        &self.tsc_icr
    }
    ///0x0c - TSC interrupt status register
    #[inline(always)]
    pub const fn tsc_isr(&self) -> &TSC_ISR {
        &self.tsc_isr
    }
    ///0x10 - TSC I/O hysteresis control register
    #[inline(always)]
    pub const fn tsc_iohcr(&self) -> &TSC_IOHCR {
        &self.tsc_iohcr
    }
    ///0x18 - TSC I/O analog switch control register
    #[inline(always)]
    pub const fn tsc_ioascr(&self) -> &TSC_IOASCR {
        &self.tsc_ioascr
    }
    ///0x20 - TSC I/O sampling control register
    #[inline(always)]
    pub const fn tsc_ioscr(&self) -> &TSC_IOSCR {
        &self.tsc_ioscr
    }
    ///0x28 - TSC I/O channel control register
    #[inline(always)]
    pub const fn tsc_ioccr(&self) -> &TSC_IOCCR {
        &self.tsc_ioccr
    }
    ///0x30 - TSC I/O group control status register
    #[inline(always)]
    pub const fn tsc_iogcsr(&self) -> &TSC_IOGCSR {
        &self.tsc_iogcsr
    }
    ///0x34 - TSC I/O group 1 counter register
    #[inline(always)]
    pub const fn tsc_iog1cr(&self) -> &TSC_IOG1CR {
        &self.tsc_iog1cr
    }
    ///0x38 - TSC I/O group 2 counter register
    #[inline(always)]
    pub const fn tsc_iog2cr(&self) -> &TSC_IOG2CR {
        &self.tsc_iog2cr
    }
    ///0x3c - TSC I/O group 3 counter register
    #[inline(always)]
    pub const fn tsc_iog3cr(&self) -> &TSC_IOG3CR {
        &self.tsc_iog3cr
    }
    ///0x40 - TSC I/O group 4 counter register
    #[inline(always)]
    pub const fn tsc_iog4cr(&self) -> &TSC_IOG4CR {
        &self.tsc_iog4cr
    }
    ///0x44 - TSC I/O group 5 counter register
    #[inline(always)]
    pub const fn tsc_iog5cr(&self) -> &TSC_IOG5CR {
        &self.tsc_iog5cr
    }
    ///0x48 - TSC I/O group 6 counter register
    #[inline(always)]
    pub const fn tsc_iog6cr(&self) -> &TSC_IOG6CR {
        &self.tsc_iog6cr
    }
    ///0x4c - TSC I/O group 7 counter register
    #[inline(always)]
    pub const fn tsc_iog7cr(&self) -> &TSC_IOG7CR {
        &self.tsc_iog7cr
    }
}
/**TSC_CR (rw) register accessor: TSC control register

You can [`read`](crate::Reg::read) this register and get [`tsc_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsc_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#TSC:TSC_CR)

For information about available fields see [`mod@tsc_cr`]
module*/
pub type TSC_CR = crate::Reg<tsc_cr::TSC_CRrs>;
///TSC control register
pub mod tsc_cr;
/**TSC_IER (rw) register accessor: TSC interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`tsc_ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsc_ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#TSC:TSC_IER)

For information about available fields see [`mod@tsc_ier`]
module*/
pub type TSC_IER = crate::Reg<tsc_ier::TSC_IERrs>;
///TSC interrupt enable register
pub mod tsc_ier;
/**TSC_ICR (rw) register accessor: TSC interrupt clear register

You can [`read`](crate::Reg::read) this register and get [`tsc_icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsc_icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#TSC:TSC_ICR)

For information about available fields see [`mod@tsc_icr`]
module*/
pub type TSC_ICR = crate::Reg<tsc_icr::TSC_ICRrs>;
///TSC interrupt clear register
pub mod tsc_icr;
/**TSC_ISR (r) register accessor: TSC interrupt status register

You can [`read`](crate::Reg::read) this register and get [`tsc_isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#TSC:TSC_ISR)

For information about available fields see [`mod@tsc_isr`]
module*/
pub type TSC_ISR = crate::Reg<tsc_isr::TSC_ISRrs>;
///TSC interrupt status register
pub mod tsc_isr;
/**TSC_IOHCR (rw) register accessor: TSC I/O hysteresis control register

You can [`read`](crate::Reg::read) this register and get [`tsc_iohcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsc_iohcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#TSC:TSC_IOHCR)

For information about available fields see [`mod@tsc_iohcr`]
module*/
pub type TSC_IOHCR = crate::Reg<tsc_iohcr::TSC_IOHCRrs>;
///TSC I/O hysteresis control register
pub mod tsc_iohcr;
/**TSC_IOASCR (rw) register accessor: TSC I/O analog switch control register

You can [`read`](crate::Reg::read) this register and get [`tsc_ioascr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsc_ioascr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#TSC:TSC_IOASCR)

For information about available fields see [`mod@tsc_ioascr`]
module*/
pub type TSC_IOASCR = crate::Reg<tsc_ioascr::TSC_IOASCRrs>;
///TSC I/O analog switch control register
pub mod tsc_ioascr;
/**TSC_IOSCR (rw) register accessor: TSC I/O sampling control register

You can [`read`](crate::Reg::read) this register and get [`tsc_ioscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsc_ioscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#TSC:TSC_IOSCR)

For information about available fields see [`mod@tsc_ioscr`]
module*/
pub type TSC_IOSCR = crate::Reg<tsc_ioscr::TSC_IOSCRrs>;
///TSC I/O sampling control register
pub mod tsc_ioscr;
/**TSC_IOCCR (rw) register accessor: TSC I/O channel control register

You can [`read`](crate::Reg::read) this register and get [`tsc_ioccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsc_ioccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#TSC:TSC_IOCCR)

For information about available fields see [`mod@tsc_ioccr`]
module*/
pub type TSC_IOCCR = crate::Reg<tsc_ioccr::TSC_IOCCRrs>;
///TSC I/O channel control register
pub mod tsc_ioccr;
/**TSC_IOGCSR (rw) register accessor: TSC I/O group control status register

You can [`read`](crate::Reg::read) this register and get [`tsc_iogcsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsc_iogcsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#TSC:TSC_IOGCSR)

For information about available fields see [`mod@tsc_iogcsr`]
module*/
pub type TSC_IOGCSR = crate::Reg<tsc_iogcsr::TSC_IOGCSRrs>;
///TSC I/O group control status register
pub mod tsc_iogcsr;
/**TSC_IOG1CR (r) register accessor: TSC I/O group 1 counter register

You can [`read`](crate::Reg::read) this register and get [`tsc_iog1cr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#TSC:TSC_IOG1CR)

For information about available fields see [`mod@tsc_iog1cr`]
module*/
pub type TSC_IOG1CR = crate::Reg<tsc_iog1cr::TSC_IOG1CRrs>;
///TSC I/O group 1 counter register
pub mod tsc_iog1cr;
/**TSC_IOG2CR (r) register accessor: TSC I/O group 2 counter register

You can [`read`](crate::Reg::read) this register and get [`tsc_iog2cr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#TSC:TSC_IOG2CR)

For information about available fields see [`mod@tsc_iog2cr`]
module*/
pub type TSC_IOG2CR = crate::Reg<tsc_iog2cr::TSC_IOG2CRrs>;
///TSC I/O group 2 counter register
pub mod tsc_iog2cr;
/**TSC_IOG3CR (r) register accessor: TSC I/O group 3 counter register

You can [`read`](crate::Reg::read) this register and get [`tsc_iog3cr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#TSC:TSC_IOG3CR)

For information about available fields see [`mod@tsc_iog3cr`]
module*/
pub type TSC_IOG3CR = crate::Reg<tsc_iog3cr::TSC_IOG3CRrs>;
///TSC I/O group 3 counter register
pub mod tsc_iog3cr;
/**TSC_IOG4CR (r) register accessor: TSC I/O group 4 counter register

You can [`read`](crate::Reg::read) this register and get [`tsc_iog4cr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#TSC:TSC_IOG4CR)

For information about available fields see [`mod@tsc_iog4cr`]
module*/
pub type TSC_IOG4CR = crate::Reg<tsc_iog4cr::TSC_IOG4CRrs>;
///TSC I/O group 4 counter register
pub mod tsc_iog4cr;
/**TSC_IOG5CR (r) register accessor: TSC I/O group 5 counter register

You can [`read`](crate::Reg::read) this register and get [`tsc_iog5cr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#TSC:TSC_IOG5CR)

For information about available fields see [`mod@tsc_iog5cr`]
module*/
pub type TSC_IOG5CR = crate::Reg<tsc_iog5cr::TSC_IOG5CRrs>;
///TSC I/O group 5 counter register
pub mod tsc_iog5cr;
/**TSC_IOG6CR (r) register accessor: TSC I/O group 6 counter register

You can [`read`](crate::Reg::read) this register and get [`tsc_iog6cr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#TSC:TSC_IOG6CR)

For information about available fields see [`mod@tsc_iog6cr`]
module*/
pub type TSC_IOG6CR = crate::Reg<tsc_iog6cr::TSC_IOG6CRrs>;
///TSC I/O group 6 counter register
pub mod tsc_iog6cr;
/**TSC_IOG7CR (r) register accessor: TSC I/O group 7 counter register

You can [`read`](crate::Reg::read) this register and get [`tsc_iog7cr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#TSC:TSC_IOG7CR)

For information about available fields see [`mod@tsc_iog7cr`]
module*/
pub type TSC_IOG7CR = crate::Reg<tsc_iog7cr::TSC_IOG7CRrs>;
///TSC I/O group 7 counter register
pub mod tsc_iog7cr;
