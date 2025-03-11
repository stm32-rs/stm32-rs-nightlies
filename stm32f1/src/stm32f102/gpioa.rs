#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    crl: CRL,
    crh: CRH,
    idr: IDR,
    odr: ODR,
    bsrr: BSRR,
    brr: BRR,
    lckr: LCKR,
}
impl RegisterBlock {
    ///0x00 - Port configuration register low (GPIOn_CRL)
    #[inline(always)]
    pub const fn crl(&self) -> &CRL {
        &self.crl
    }
    ///0x04 - Port configuration register high (GPIOn_CRL)
    #[inline(always)]
    pub const fn crh(&self) -> &CRH {
        &self.crh
    }
    ///0x08 - Port input data register (GPIOn_IDR)
    #[inline(always)]
    pub const fn idr(&self) -> &IDR {
        &self.idr
    }
    ///0x0c - Port output data register (GPIOn_ODR)
    #[inline(always)]
    pub const fn odr(&self) -> &ODR {
        &self.odr
    }
    ///0x10 - Port bit set/reset register (GPIOn_BSRR)
    #[inline(always)]
    pub const fn bsrr(&self) -> &BSRR {
        &self.bsrr
    }
    ///0x14 - Port bit reset register (GPIOn_BRR)
    #[inline(always)]
    pub const fn brr(&self) -> &BRR {
        &self.brr
    }
    ///0x18 - Port configuration lock register
    #[inline(always)]
    pub const fn lckr(&self) -> &LCKR {
        &self.lckr
    }
}
/**CRL (rw) register accessor: Port configuration register low (GPIOn_CRL)

You can [`read`](crate::Reg::read) this register and get [`crl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F102.html#GPIOA:CRL)

For information about available fields see [`mod@crl`] module*/
pub type CRL = crate::Reg<crl::CRLrs>;
///Port configuration register low (GPIOn_CRL)
pub mod crl;
/**CRH (rw) register accessor: Port configuration register high (GPIOn_CRL)

You can [`read`](crate::Reg::read) this register and get [`crh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F102.html#GPIOA:CRH)

For information about available fields see [`mod@crh`] module*/
pub type CRH = crate::Reg<crh::CRHrs>;
///Port configuration register high (GPIOn_CRL)
pub mod crh;
/**IDR (r) register accessor: Port input data register (GPIOn_IDR)

You can [`read`](crate::Reg::read) this register and get [`idr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F102.html#GPIOA:IDR)

For information about available fields see [`mod@idr`] module*/
pub type IDR = crate::Reg<idr::IDRrs>;
///Port input data register (GPIOn_IDR)
pub mod idr;
/**ODR (rw) register accessor: Port output data register (GPIOn_ODR)

You can [`read`](crate::Reg::read) this register and get [`odr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`odr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F102.html#GPIOA:ODR)

For information about available fields see [`mod@odr`] module*/
pub type ODR = crate::Reg<odr::ODRrs>;
///Port output data register (GPIOn_ODR)
pub mod odr;
/**BSRR (w) register accessor: Port bit set/reset register (GPIOn_BSRR)

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsrr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F102.html#GPIOA:BSRR)

For information about available fields see [`mod@bsrr`] module*/
pub type BSRR = crate::Reg<bsrr::BSRRrs>;
///Port bit set/reset register (GPIOn_BSRR)
pub mod bsrr;
/**BRR (w) register accessor: Port bit reset register (GPIOn_BRR)

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F102.html#GPIOA:BRR)

For information about available fields see [`mod@brr`] module*/
pub type BRR = crate::Reg<brr::BRRrs>;
///Port bit reset register (GPIOn_BRR)
pub mod brr;
/**LCKR (rw) register accessor: Port configuration lock register

You can [`read`](crate::Reg::read) this register and get [`lckr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lckr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F102.html#GPIOA:LCKR)

For information about available fields see [`mod@lckr`] module*/
pub type LCKR = crate::Reg<lckr::LCKRrs>;
///Port configuration lock register
pub mod lckr;
