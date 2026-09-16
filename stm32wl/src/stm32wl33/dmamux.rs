#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    c0cr: C0CR,
    c1cr: C1CR,
    c2cr: C2CR,
    c3cr: C3CR,
    c4cr: C4CR,
    c5cr: C5CR,
    c6cr: C6CR,
    c7cr: C7CR,
}
impl RegisterBlock {
    ///0x00 - CxCR register
    #[inline(always)]
    pub const fn c0cr(&self) -> &C0CR {
        &self.c0cr
    }
    ///0x04 - CxCR register
    #[inline(always)]
    pub const fn c1cr(&self) -> &C1CR {
        &self.c1cr
    }
    ///0x08 - CxCR register
    #[inline(always)]
    pub const fn c2cr(&self) -> &C2CR {
        &self.c2cr
    }
    ///0x0c - CxCR register
    #[inline(always)]
    pub const fn c3cr(&self) -> &C3CR {
        &self.c3cr
    }
    ///0x10 - CxCR register
    #[inline(always)]
    pub const fn c4cr(&self) -> &C4CR {
        &self.c4cr
    }
    ///0x14 - CxCR register
    #[inline(always)]
    pub const fn c5cr(&self) -> &C5CR {
        &self.c5cr
    }
    ///0x18 - CxCR register
    #[inline(always)]
    pub const fn c6cr(&self) -> &C6CR {
        &self.c6cr
    }
    ///0x1c - CxCR register
    #[inline(always)]
    pub const fn c7cr(&self) -> &C7CR {
        &self.c7cr
    }
}
/**C0CR (rw) register accessor: CxCR register

You can [`read`](crate::Reg::read) this register and get [`c0cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c0cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DMAMUX:C0CR)

For information about available fields see [`mod@c0cr`] module*/
pub type C0CR = crate::Reg<c0cr::C0CRrs>;
///CxCR register
pub mod c0cr;
/**C1CR (rw) register accessor: CxCR register

You can [`read`](crate::Reg::read) this register and get [`c1cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DMAMUX:C1CR)

For information about available fields see [`mod@c1cr`] module*/
pub type C1CR = crate::Reg<c1cr::C1CRrs>;
///CxCR register
pub mod c1cr;
/**C2CR (rw) register accessor: CxCR register

You can [`read`](crate::Reg::read) this register and get [`c2cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DMAMUX:C2CR)

For information about available fields see [`mod@c2cr`] module*/
pub type C2CR = crate::Reg<c2cr::C2CRrs>;
///CxCR register
pub mod c2cr;
/**C3CR (rw) register accessor: CxCR register

You can [`read`](crate::Reg::read) this register and get [`c3cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c3cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DMAMUX:C3CR)

For information about available fields see [`mod@c3cr`] module*/
pub type C3CR = crate::Reg<c3cr::C3CRrs>;
///CxCR register
pub mod c3cr;
/**C4CR (rw) register accessor: CxCR register

You can [`read`](crate::Reg::read) this register and get [`c4cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c4cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DMAMUX:C4CR)

For information about available fields see [`mod@c4cr`] module*/
pub type C4CR = crate::Reg<c4cr::C4CRrs>;
///CxCR register
pub mod c4cr;
/**C5CR (rw) register accessor: CxCR register

You can [`read`](crate::Reg::read) this register and get [`c5cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c5cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DMAMUX:C5CR)

For information about available fields see [`mod@c5cr`] module*/
pub type C5CR = crate::Reg<c5cr::C5CRrs>;
///CxCR register
pub mod c5cr;
/**C6CR (rw) register accessor: CxCR register

You can [`read`](crate::Reg::read) this register and get [`c6cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c6cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DMAMUX:C6CR)

For information about available fields see [`mod@c6cr`] module*/
pub type C6CR = crate::Reg<c6cr::C6CRrs>;
///CxCR register
pub mod c6cr;
/**C7CR (rw) register accessor: CxCR register

You can [`read`](crate::Reg::read) this register and get [`c7cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c7cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DMAMUX:C7CR)

For information about available fields see [`mod@c7cr`] module*/
pub type C7CR = crate::Reg<c7cr::C7CRrs>;
///CxCR register
pub mod c7cr;
