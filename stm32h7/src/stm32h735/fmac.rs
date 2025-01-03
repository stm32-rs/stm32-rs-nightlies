#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    fmac_x1bufcfg: FMAC_X1BUFCFG,
    fmac_x2bufcfg: FMAC_X2BUFCFG,
    fmac_ybufcfg: FMAC_YBUFCFG,
    fmac_param: FMAC_PARAM,
    fmac_cr: FMAC_CR,
    fmac_sr: FMAC_SR,
    fmac_wdata: FMAC_WDATA,
    fmac_rdata: FMAC_RDATA,
}
impl RegisterBlock {
    ///0x00 - FMAC X1 buffer configuration register
    #[inline(always)]
    pub const fn fmac_x1bufcfg(&self) -> &FMAC_X1BUFCFG {
        &self.fmac_x1bufcfg
    }
    ///0x04 - FMAC X2 buffer configuration register
    #[inline(always)]
    pub const fn fmac_x2bufcfg(&self) -> &FMAC_X2BUFCFG {
        &self.fmac_x2bufcfg
    }
    ///0x08 - FMAC Y buffer configuration register
    #[inline(always)]
    pub const fn fmac_ybufcfg(&self) -> &FMAC_YBUFCFG {
        &self.fmac_ybufcfg
    }
    ///0x0c - FMAC parameter register
    #[inline(always)]
    pub const fn fmac_param(&self) -> &FMAC_PARAM {
        &self.fmac_param
    }
    ///0x10 - FMAC control register
    #[inline(always)]
    pub const fn fmac_cr(&self) -> &FMAC_CR {
        &self.fmac_cr
    }
    ///0x14 - FMAC status register
    #[inline(always)]
    pub const fn fmac_sr(&self) -> &FMAC_SR {
        &self.fmac_sr
    }
    ///0x18 - FMAC write data register
    #[inline(always)]
    pub const fn fmac_wdata(&self) -> &FMAC_WDATA {
        &self.fmac_wdata
    }
    ///0x1c - FMAC read data register
    #[inline(always)]
    pub const fn fmac_rdata(&self) -> &FMAC_RDATA {
        &self.fmac_rdata
    }
}
/**FMAC_X1BUFCFG (rw) register accessor: FMAC X1 buffer configuration register

You can [`read`](crate::Reg::read) this register and get [`fmac_x1bufcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmac_x1bufcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FMAC:FMAC_X1BUFCFG)

For information about available fields see [`mod@fmac_x1bufcfg`]
module*/
pub type FMAC_X1BUFCFG = crate::Reg<fmac_x1bufcfg::FMAC_X1BUFCFGrs>;
///FMAC X1 buffer configuration register
pub mod fmac_x1bufcfg;
/**FMAC_X2BUFCFG (rw) register accessor: FMAC X2 buffer configuration register

You can [`read`](crate::Reg::read) this register and get [`fmac_x2bufcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmac_x2bufcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FMAC:FMAC_X2BUFCFG)

For information about available fields see [`mod@fmac_x2bufcfg`]
module*/
pub type FMAC_X2BUFCFG = crate::Reg<fmac_x2bufcfg::FMAC_X2BUFCFGrs>;
///FMAC X2 buffer configuration register
pub mod fmac_x2bufcfg;
/**FMAC_YBUFCFG (rw) register accessor: FMAC Y buffer configuration register

You can [`read`](crate::Reg::read) this register and get [`fmac_ybufcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmac_ybufcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FMAC:FMAC_YBUFCFG)

For information about available fields see [`mod@fmac_ybufcfg`]
module*/
pub type FMAC_YBUFCFG = crate::Reg<fmac_ybufcfg::FMAC_YBUFCFGrs>;
///FMAC Y buffer configuration register
pub mod fmac_ybufcfg;
/**FMAC_PARAM (rw) register accessor: FMAC parameter register

You can [`read`](crate::Reg::read) this register and get [`fmac_param::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmac_param::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FMAC:FMAC_PARAM)

For information about available fields see [`mod@fmac_param`]
module*/
pub type FMAC_PARAM = crate::Reg<fmac_param::FMAC_PARAMrs>;
///FMAC parameter register
pub mod fmac_param;
/**FMAC_CR (rw) register accessor: FMAC control register

You can [`read`](crate::Reg::read) this register and get [`fmac_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmac_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FMAC:FMAC_CR)

For information about available fields see [`mod@fmac_cr`]
module*/
pub type FMAC_CR = crate::Reg<fmac_cr::FMAC_CRrs>;
///FMAC control register
pub mod fmac_cr;
/**FMAC_SR (r) register accessor: FMAC status register

You can [`read`](crate::Reg::read) this register and get [`fmac_sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FMAC:FMAC_SR)

For information about available fields see [`mod@fmac_sr`]
module*/
pub type FMAC_SR = crate::Reg<fmac_sr::FMAC_SRrs>;
///FMAC status register
pub mod fmac_sr;
/**FMAC_WDATA (w) register accessor: FMAC write data register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmac_wdata::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FMAC:FMAC_WDATA)

For information about available fields see [`mod@fmac_wdata`]
module*/
pub type FMAC_WDATA = crate::Reg<fmac_wdata::FMAC_WDATArs>;
///FMAC write data register
pub mod fmac_wdata;
/**FMAC_RDATA (r) register accessor: FMAC read data register

You can [`read`](crate::Reg::read) this register and get [`fmac_rdata::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FMAC:FMAC_RDATA)

For information about available fields see [`mod@fmac_rdata`]
module*/
pub type FMAC_RDATA = crate::Reg<fmac_rdata::FMAC_RDATArs>;
///FMAC read data register
pub mod fmac_rdata;
