#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cfgr: CFGR,
    oar: OAR,
    pres: PRES,
    esr: ESR,
    csr: CSR,
    txd: TXD,
    rxd: RXD,
}
impl RegisterBlock {
    ///0x00 - configuration register
    #[inline(always)]
    pub const fn cfgr(&self) -> &CFGR {
        &self.cfgr
    }
    ///0x04 - CEC own address register
    #[inline(always)]
    pub const fn oar(&self) -> &OAR {
        &self.oar
    }
    ///0x08 - Rx Data Register
    #[inline(always)]
    pub const fn pres(&self) -> &PRES {
        &self.pres
    }
    ///0x0c - CEC error status register
    #[inline(always)]
    pub const fn esr(&self) -> &ESR {
        &self.esr
    }
    ///0x10 - CEC control and status register
    #[inline(always)]
    pub const fn csr(&self) -> &CSR {
        &self.csr
    }
    ///0x14 - CEC Tx data register
    #[inline(always)]
    pub const fn txd(&self) -> &TXD {
        &self.txd
    }
    ///0x18 - CEC Rx data register
    #[inline(always)]
    pub const fn rxd(&self) -> &RXD {
        &self.rxd
    }
}
/**CFGR (rw) register accessor: configuration register

You can [`read`](crate::Reg::read) this register and get [`cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F100.html#CEC:CFGR)

For information about available fields see [`mod@cfgr`] module*/
pub type CFGR = crate::Reg<cfgr::CFGRrs>;
///configuration register
pub mod cfgr;
/**OAR (rw) register accessor: CEC own address register

You can [`read`](crate::Reg::read) this register and get [`oar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F100.html#CEC:OAR)

For information about available fields see [`mod@oar`] module*/
pub type OAR = crate::Reg<oar::OARrs>;
///CEC own address register
pub mod oar;
/**PRES (rw) register accessor: Rx Data Register

You can [`read`](crate::Reg::read) this register and get [`pres::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pres::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F100.html#CEC:PRES)

For information about available fields see [`mod@pres`] module*/
pub type PRES = crate::Reg<pres::PRESrs>;
///Rx Data Register
pub mod pres;
/**ESR (r) register accessor: CEC error status register

You can [`read`](crate::Reg::read) this register and get [`esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F100.html#CEC:ESR)

For information about available fields see [`mod@esr`] module*/
pub type ESR = crate::Reg<esr::ESRrs>;
///CEC error status register
pub mod esr;
/**CSR (rw) register accessor: CEC control and status register

You can [`read`](crate::Reg::read) this register and get [`csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F100.html#CEC:CSR)

For information about available fields see [`mod@csr`] module*/
pub type CSR = crate::Reg<csr::CSRrs>;
///CEC control and status register
pub mod csr;
/**TXD (rw) register accessor: CEC Tx data register

You can [`read`](crate::Reg::read) this register and get [`txd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F100.html#CEC:TXD)

For information about available fields see [`mod@txd`] module*/
pub type TXD = crate::Reg<txd::TXDrs>;
///CEC Tx data register
pub mod txd;
/**RXD (r) register accessor: CEC Rx data register

You can [`read`](crate::Reg::read) this register and get [`rxd::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F100.html#CEC:RXD)

For information about available fields see [`mod@rxd`] module*/
pub type RXD = crate::Reg<rxd::RXDrs>;
///CEC Rx data register
pub mod rxd;
