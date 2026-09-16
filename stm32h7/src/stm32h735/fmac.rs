#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    x1bufcfg: X1BUFCFG,
    x2bufcfg: X2BUFCFG,
    ybufcfg: YBUFCFG,
    param: PARAM,
    cr: CR,
    sr: SR,
    wdata: WDATA,
    rdata: RDATA,
}
impl RegisterBlock {
    ///0x00 - FMAC X1 buffer configuration register
    #[inline(always)]
    pub const fn x1bufcfg(&self) -> &X1BUFCFG {
        &self.x1bufcfg
    }
    ///0x04 - FMAC X2 buffer configuration register
    #[inline(always)]
    pub const fn x2bufcfg(&self) -> &X2BUFCFG {
        &self.x2bufcfg
    }
    ///0x08 - FMAC Y buffer configuration register
    #[inline(always)]
    pub const fn ybufcfg(&self) -> &YBUFCFG {
        &self.ybufcfg
    }
    ///0x0c - FMAC parameter register
    #[inline(always)]
    pub const fn param(&self) -> &PARAM {
        &self.param
    }
    ///0x10 - FMAC control register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x14 - FMAC status register
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0x18 - FMAC write data register
    #[inline(always)]
    pub const fn wdata(&self) -> &WDATA {
        &self.wdata
    }
    ///0x1c - FMAC read data register
    #[inline(always)]
    pub const fn rdata(&self) -> &RDATA {
        &self.rdata
    }
}
/**X1BUFCFG (rw) register accessor: FMAC X1 buffer configuration register

You can [`read`](crate::Reg::read) this register and get [`x1bufcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`x1bufcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FMAC:X1BUFCFG)

For information about available fields see [`mod@x1bufcfg`] module*/
pub type X1BUFCFG = crate::Reg<x1bufcfg::X1BUFCFGrs>;
///FMAC X1 buffer configuration register
pub mod x1bufcfg;
/**X2BUFCFG (rw) register accessor: FMAC X2 buffer configuration register

You can [`read`](crate::Reg::read) this register and get [`x2bufcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`x2bufcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FMAC:X2BUFCFG)

For information about available fields see [`mod@x2bufcfg`] module*/
pub type X2BUFCFG = crate::Reg<x2bufcfg::X2BUFCFGrs>;
///FMAC X2 buffer configuration register
pub mod x2bufcfg;
/**YBUFCFG (rw) register accessor: FMAC Y buffer configuration register

You can [`read`](crate::Reg::read) this register and get [`ybufcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ybufcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FMAC:YBUFCFG)

For information about available fields see [`mod@ybufcfg`] module*/
pub type YBUFCFG = crate::Reg<ybufcfg::YBUFCFGrs>;
///FMAC Y buffer configuration register
pub mod ybufcfg;
/**PARAM (rw) register accessor: FMAC parameter register

You can [`read`](crate::Reg::read) this register and get [`param::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`param::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FMAC:PARAM)

For information about available fields see [`mod@param`] module*/
pub type PARAM = crate::Reg<param::PARAMrs>;
///FMAC parameter register
pub mod param;
/**CR (rw) register accessor: FMAC control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FMAC:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///FMAC control register
pub mod cr;
/**SR (r) register accessor: FMAC status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FMAC:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///FMAC status register
pub mod sr;
/**WDATA (w) register accessor: FMAC write data register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdata::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FMAC:WDATA)

For information about available fields see [`mod@wdata`] module*/
pub type WDATA = crate::Reg<wdata::WDATArs>;
///FMAC write data register
pub mod wdata;
/**RDATA (r) register accessor: FMAC read data register

You can [`read`](crate::Reg::read) this register and get [`rdata::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FMAC:RDATA)

For information about available fields see [`mod@rdata`] module*/
pub type RDATA = crate::Reg<rdata::RDATArs>;
///FMAC read data register
pub mod rdata;
