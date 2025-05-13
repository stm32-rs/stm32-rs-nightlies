#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cec_cr: CEC_CR,
    cec_cfgr: CEC_CFGR,
    cec_txdr: CEC_TXDR,
    cec_rxdr: CEC_RXDR,
    cec_isr: CEC_ISR,
    cec_ier: CEC_IER,
}
impl RegisterBlock {
    ///0x00 - CEC control register
    #[inline(always)]
    pub const fn cec_cr(&self) -> &CEC_CR {
        &self.cec_cr
    }
    ///0x04 - CEC configuration register
    #[inline(always)]
    pub const fn cec_cfgr(&self) -> &CEC_CFGR {
        &self.cec_cfgr
    }
    ///0x08 - CEC Tx data register
    #[inline(always)]
    pub const fn cec_txdr(&self) -> &CEC_TXDR {
        &self.cec_txdr
    }
    ///0x0c - CEC Rx Data Register
    #[inline(always)]
    pub const fn cec_rxdr(&self) -> &CEC_RXDR {
        &self.cec_rxdr
    }
    ///0x10 - CEC Interrupt and Status Register
    #[inline(always)]
    pub const fn cec_isr(&self) -> &CEC_ISR {
        &self.cec_isr
    }
    ///0x14 - CEC interrupt enable register
    #[inline(always)]
    pub const fn cec_ier(&self) -> &CEC_IER {
        &self.cec_ier
    }
}
/**CEC_CR (rw) register accessor: CEC control register

You can [`read`](crate::Reg::read) this register and get [`cec_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cec_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F446.html#HDMI_CEC:CEC_CR)

For information about available fields see [`mod@cec_cr`] module*/
pub type CEC_CR = crate::Reg<cec_cr::CEC_CRrs>;
///CEC control register
pub mod cec_cr;
/**CEC_CFGR (rw) register accessor: CEC configuration register

You can [`read`](crate::Reg::read) this register and get [`cec_cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cec_cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F446.html#HDMI_CEC:CEC_CFGR)

For information about available fields see [`mod@cec_cfgr`] module*/
pub type CEC_CFGR = crate::Reg<cec_cfgr::CEC_CFGRrs>;
///CEC configuration register
pub mod cec_cfgr;
/**CEC_TXDR (w) register accessor: CEC Tx data register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cec_txdr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F446.html#HDMI_CEC:CEC_TXDR)

For information about available fields see [`mod@cec_txdr`] module*/
pub type CEC_TXDR = crate::Reg<cec_txdr::CEC_TXDRrs>;
///CEC Tx data register
pub mod cec_txdr;
/**CEC_RXDR (r) register accessor: CEC Rx Data Register

You can [`read`](crate::Reg::read) this register and get [`cec_rxdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F446.html#HDMI_CEC:CEC_RXDR)

For information about available fields see [`mod@cec_rxdr`] module*/
pub type CEC_RXDR = crate::Reg<cec_rxdr::CEC_RXDRrs>;
///CEC Rx Data Register
pub mod cec_rxdr;
/**CEC_ISR (rw) register accessor: CEC Interrupt and Status Register

You can [`read`](crate::Reg::read) this register and get [`cec_isr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cec_isr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F446.html#HDMI_CEC:CEC_ISR)

For information about available fields see [`mod@cec_isr`] module*/
pub type CEC_ISR = crate::Reg<cec_isr::CEC_ISRrs>;
///CEC Interrupt and Status Register
pub mod cec_isr;
/**CEC_IER (rw) register accessor: CEC interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`cec_ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cec_ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F446.html#HDMI_CEC:CEC_IER)

For information about available fields see [`mod@cec_ier`] module*/
pub type CEC_IER = crate::Reg<cec_ier::CEC_IERrs>;
///CEC interrupt enable register
pub mod cec_ier;
