#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr: CR,
    ier: IER,
    icr: ICR,
    isr: ISR,
    iohcr: IOHCR,
    _reserved5: [u8; 0x04],
    ioascr: IOASCR,
    _reserved6: [u8; 0x04],
    ioscr: IOSCR,
    _reserved7: [u8; 0x04],
    ioccr: IOCCR,
    _reserved8: [u8; 0x04],
    iogcsr: IOGCSR,
    iog1cr: IOG1CR,
    iog2cr: IOG2CR,
    iog3cr: IOG3CR,
    iog4cr: IOG4CR,
    iog5cr: IOG5CR,
    iog6cr: IOG6CR,
}
impl RegisterBlock {
    ///0x00 - TSC control register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x04 - TSC interrupt enable register
    #[inline(always)]
    pub const fn ier(&self) -> &IER {
        &self.ier
    }
    ///0x08 - TSC interrupt clear register
    #[inline(always)]
    pub const fn icr(&self) -> &ICR {
        &self.icr
    }
    ///0x0c - TSC interrupt status register
    #[inline(always)]
    pub const fn isr(&self) -> &ISR {
        &self.isr
    }
    ///0x10 - TSC I/O hysteresis control register
    #[inline(always)]
    pub const fn iohcr(&self) -> &IOHCR {
        &self.iohcr
    }
    ///0x18 - TSC I/O analog switch control register
    #[inline(always)]
    pub const fn ioascr(&self) -> &IOASCR {
        &self.ioascr
    }
    ///0x20 - TSC I/O sampling control register
    #[inline(always)]
    pub const fn ioscr(&self) -> &IOSCR {
        &self.ioscr
    }
    ///0x28 - TSC I/O channel control register
    #[inline(always)]
    pub const fn ioccr(&self) -> &IOCCR {
        &self.ioccr
    }
    ///0x30 - TSC I/O group control status register
    #[inline(always)]
    pub const fn iogcsr(&self) -> &IOGCSR {
        &self.iogcsr
    }
    ///0x34 - TSC I/O group 1 counter register
    #[inline(always)]
    pub const fn iog1cr(&self) -> &IOG1CR {
        &self.iog1cr
    }
    ///0x38 - TSC I/O group 2 counter register
    #[inline(always)]
    pub const fn iog2cr(&self) -> &IOG2CR {
        &self.iog2cr
    }
    ///0x3c - TSC I/O group 3 counter register
    #[inline(always)]
    pub const fn iog3cr(&self) -> &IOG3CR {
        &self.iog3cr
    }
    ///0x40 - TSC I/O group 4 counter register
    #[inline(always)]
    pub const fn iog4cr(&self) -> &IOG4CR {
        &self.iog4cr
    }
    ///0x44 - TSC I/O group 5 counter register
    #[inline(always)]
    pub const fn iog5cr(&self) -> &IOG5CR {
        &self.iog5cr
    }
    ///0x48 - TSC I/O group 6 counter register
    #[inline(always)]
    pub const fn iog6cr(&self) -> &IOG6CR {
        &self.iog6cr
    }
}
/**CR (rw) register accessor: TSC control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#TSC:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///TSC control register
pub mod cr;
/**IER (rw) register accessor: TSC interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#TSC:IER)

For information about available fields see [`mod@ier`] module*/
pub type IER = crate::Reg<ier::IERrs>;
///TSC interrupt enable register
pub mod ier;
/**ICR (rw) register accessor: TSC interrupt clear register

You can [`read`](crate::Reg::read) this register and get [`icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#TSC:ICR)

For information about available fields see [`mod@icr`] module*/
pub type ICR = crate::Reg<icr::ICRrs>;
///TSC interrupt clear register
pub mod icr;
/**ISR (r) register accessor: TSC interrupt status register

You can [`read`](crate::Reg::read) this register and get [`isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#TSC:ISR)

For information about available fields see [`mod@isr`] module*/
pub type ISR = crate::Reg<isr::ISRrs>;
///TSC interrupt status register
pub mod isr;
/**IOHCR (rw) register accessor: TSC I/O hysteresis control register

You can [`read`](crate::Reg::read) this register and get [`iohcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iohcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#TSC:IOHCR)

For information about available fields see [`mod@iohcr`] module*/
pub type IOHCR = crate::Reg<iohcr::IOHCRrs>;
///TSC I/O hysteresis control register
pub mod iohcr;
/**IOASCR (rw) register accessor: TSC I/O analog switch control register

You can [`read`](crate::Reg::read) this register and get [`ioascr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ioascr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#TSC:IOASCR)

For information about available fields see [`mod@ioascr`] module*/
pub type IOASCR = crate::Reg<ioascr::IOASCRrs>;
///TSC I/O analog switch control register
pub mod ioascr;
/**IOSCR (rw) register accessor: TSC I/O sampling control register

You can [`read`](crate::Reg::read) this register and get [`ioscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ioscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#TSC:IOSCR)

For information about available fields see [`mod@ioscr`] module*/
pub type IOSCR = crate::Reg<ioscr::IOSCRrs>;
///TSC I/O sampling control register
pub mod ioscr;
/**IOCCR (rw) register accessor: TSC I/O channel control register

You can [`read`](crate::Reg::read) this register and get [`ioccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ioccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#TSC:IOCCR)

For information about available fields see [`mod@ioccr`] module*/
pub type IOCCR = crate::Reg<ioccr::IOCCRrs>;
///TSC I/O channel control register
pub mod ioccr;
/**IOGCSR (rw) register accessor: TSC I/O group control status register

You can [`read`](crate::Reg::read) this register and get [`iogcsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iogcsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#TSC:IOGCSR)

For information about available fields see [`mod@iogcsr`] module*/
pub type IOGCSR = crate::Reg<iogcsr::IOGCSRrs>;
///TSC I/O group control status register
pub mod iogcsr;
/**IOG1CR (r) register accessor: TSC I/O group 1 counter register

You can [`read`](crate::Reg::read) this register and get [`iog1cr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#TSC:IOG1CR)

For information about available fields see [`mod@iog1cr`] module*/
pub type IOG1CR = crate::Reg<iog1cr::IOG1CRrs>;
///TSC I/O group 1 counter register
pub mod iog1cr;
/**IOG2CR (r) register accessor: TSC I/O group 2 counter register

You can [`read`](crate::Reg::read) this register and get [`iog2cr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#TSC:IOG2CR)

For information about available fields see [`mod@iog2cr`] module*/
pub type IOG2CR = crate::Reg<iog2cr::IOG2CRrs>;
///TSC I/O group 2 counter register
pub mod iog2cr;
/**IOG3CR (r) register accessor: TSC I/O group 3 counter register

You can [`read`](crate::Reg::read) this register and get [`iog3cr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#TSC:IOG3CR)

For information about available fields see [`mod@iog3cr`] module*/
pub type IOG3CR = crate::Reg<iog3cr::IOG3CRrs>;
///TSC I/O group 3 counter register
pub mod iog3cr;
/**IOG4CR (r) register accessor: TSC I/O group 4 counter register

You can [`read`](crate::Reg::read) this register and get [`iog4cr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#TSC:IOG4CR)

For information about available fields see [`mod@iog4cr`] module*/
pub type IOG4CR = crate::Reg<iog4cr::IOG4CRrs>;
///TSC I/O group 4 counter register
pub mod iog4cr;
/**IOG5CR (r) register accessor: TSC I/O group 5 counter register

You can [`read`](crate::Reg::read) this register and get [`iog5cr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#TSC:IOG5CR)

For information about available fields see [`mod@iog5cr`] module*/
pub type IOG5CR = crate::Reg<iog5cr::IOG5CRrs>;
///TSC I/O group 5 counter register
pub mod iog5cr;
/**IOG6CR (r) register accessor: TSC I/O group 6 counter register

You can [`read`](crate::Reg::read) this register and get [`iog6cr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#TSC:IOG6CR)

For information about available fields see [`mod@iog6cr`] module*/
pub type IOG6CR = crate::Reg<iog6cr::IOG6CRrs>;
///TSC I/O group 6 counter register
pub mod iog6cr;
