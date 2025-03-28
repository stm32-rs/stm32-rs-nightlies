#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr: CR,
    sr: SR,
    fcr: FCR,
    _reserved3: [u8; 0x04],
    dvr: DVR,
    _reserved4: [u8; 0x0c],
    b0cr: B0CR,
    b1cr: B1CR,
    b2cr: B2CR,
    b3cr: B3CR,
    _reserved8: [u8; 0x0fc4],
    verr: VERR,
    ipidr: IPIDR,
    sidr: SIDR,
    lut: [LUT; 1024],
}
impl RegisterBlock {
    ///0x00 - Graphic MMU configuration register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x04 - Graphic MMU status register
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0x08 - Graphic MMU flag clear register
    #[inline(always)]
    pub const fn fcr(&self) -> &FCR {
        &self.fcr
    }
    ///0x10 - Graphic MMU default value register
    #[inline(always)]
    pub const fn dvr(&self) -> &DVR {
        &self.dvr
    }
    ///0x20 - Graphic MMU buffer 0 configuration register
    #[inline(always)]
    pub const fn b0cr(&self) -> &B0CR {
        &self.b0cr
    }
    ///0x24 - Graphic MMU buffer 1 configuration register
    #[inline(always)]
    pub const fn b1cr(&self) -> &B1CR {
        &self.b1cr
    }
    ///0x28 - Graphic MMU buffer 2 configuration register
    #[inline(always)]
    pub const fn b2cr(&self) -> &B2CR {
        &self.b2cr
    }
    ///0x2c - Graphic MMU buffer 3 configuration register
    #[inline(always)]
    pub const fn b3cr(&self) -> &B3CR {
        &self.b3cr
    }
    ///0xff4 - Graphic MMU version register
    #[inline(always)]
    pub const fn verr(&self) -> &VERR {
        &self.verr
    }
    ///0xff8 - Graphic MMU identification register
    #[inline(always)]
    pub const fn ipidr(&self) -> &IPIDR {
        &self.ipidr
    }
    ///0xffc - Graphic MMU size identification register
    #[inline(always)]
    pub const fn sidr(&self) -> &SIDR {
        &self.sidr
    }
    ///0x1000..0x3000 - Cluster LUT%s, containing LUT*L, LUT*H
    #[inline(always)]
    pub const fn lut(&self, n: usize) -> &LUT {
        &self.lut[n]
    }
    ///Iterator for array of:
    ///0x1000..0x3000 - Cluster LUT%s, containing LUT*L, LUT*H
    #[inline(always)]
    pub fn lut_iter(&self) -> impl Iterator<Item = &LUT> {
        self.lut.iter()
    }
}
/**CR (rw) register accessor: Graphic MMU configuration register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R5.html#GFXMMU:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///Graphic MMU configuration register
pub mod cr;
/**SR (r) register accessor: Graphic MMU status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R5.html#GFXMMU:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///Graphic MMU status register
pub mod sr;
/**FCR (w) register accessor: Graphic MMU flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R5.html#GFXMMU:FCR)

For information about available fields see [`mod@fcr`] module*/
pub type FCR = crate::Reg<fcr::FCRrs>;
///Graphic MMU flag clear register
pub mod fcr;
/**DVR (rw) register accessor: Graphic MMU default value register

You can [`read`](crate::Reg::read) this register and get [`dvr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dvr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R5.html#GFXMMU:DVR)

For information about available fields see [`mod@dvr`] module*/
pub type DVR = crate::Reg<dvr::DVRrs>;
///Graphic MMU default value register
pub mod dvr;
/**B0CR (rw) register accessor: Graphic MMU buffer 0 configuration register

You can [`read`](crate::Reg::read) this register and get [`b0cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`b0cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R5.html#GFXMMU:B0CR)

For information about available fields see [`mod@b0cr`] module*/
pub type B0CR = crate::Reg<b0cr::B0CRrs>;
///Graphic MMU buffer 0 configuration register
pub mod b0cr;
/**B1CR (rw) register accessor: Graphic MMU buffer 1 configuration register

You can [`read`](crate::Reg::read) this register and get [`b1cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`b1cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R5.html#GFXMMU:B1CR)

For information about available fields see [`mod@b1cr`] module*/
pub type B1CR = crate::Reg<b1cr::B1CRrs>;
///Graphic MMU buffer 1 configuration register
pub mod b1cr;
/**B2CR (rw) register accessor: Graphic MMU buffer 2 configuration register

You can [`read`](crate::Reg::read) this register and get [`b2cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`b2cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R5.html#GFXMMU:B2CR)

For information about available fields see [`mod@b2cr`] module*/
pub type B2CR = crate::Reg<b2cr::B2CRrs>;
///Graphic MMU buffer 2 configuration register
pub mod b2cr;
/**B3CR (rw) register accessor: Graphic MMU buffer 3 configuration register

You can [`read`](crate::Reg::read) this register and get [`b3cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`b3cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R5.html#GFXMMU:B3CR)

For information about available fields see [`mod@b3cr`] module*/
pub type B3CR = crate::Reg<b3cr::B3CRrs>;
///Graphic MMU buffer 3 configuration register
pub mod b3cr;
/**VERR (r) register accessor: Graphic MMU version register

You can [`read`](crate::Reg::read) this register and get [`verr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R5.html#GFXMMU:VERR)

For information about available fields see [`mod@verr`] module*/
pub type VERR = crate::Reg<verr::VERRrs>;
///Graphic MMU version register
pub mod verr;
/**IPIDR (r) register accessor: Graphic MMU identification register

You can [`read`](crate::Reg::read) this register and get [`ipidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R5.html#GFXMMU:IPIDR)

For information about available fields see [`mod@ipidr`] module*/
pub type IPIDR = crate::Reg<ipidr::IPIDRrs>;
///Graphic MMU identification register
pub mod ipidr;
/**SIDR (r) register accessor: Graphic MMU size identification register

You can [`read`](crate::Reg::read) this register and get [`sidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R5.html#GFXMMU:SIDR)

For information about available fields see [`mod@sidr`] module*/
pub type SIDR = crate::Reg<sidr::SIDRrs>;
///Graphic MMU size identification register
pub mod sidr;
///Cluster LUT%s, containing LUT*L, LUT*H
pub use self::lut::LUT;
///Cluster
///Cluster LUT%s, containing LUT*L, LUT*H
pub mod lut;
