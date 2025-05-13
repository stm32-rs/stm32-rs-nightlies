#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    hcr: HCR,
    vtr: VTR,
    vmcr: VMCR,
    _reserved3: [u8; 0x04],
    misr: MISR,
    _reserved4: [u8; 0x0c],
    eisr0: EISR0,
    _reserved5: [u8; 0x0c],
    elsr0: ELSR0,
    _reserved6: [u8; 0xbc],
    apr0: APR0,
    _reserved7: [u8; 0x0c],
    lr0: LR0,
    lr1: LR1,
    lr2: LR2,
    lr3: LR3,
}
impl RegisterBlock {
    ///0x00 - GICH hypervisor control register
    #[inline(always)]
    pub const fn hcr(&self) -> &HCR {
        &self.hcr
    }
    ///0x04 - GICH VGIC type register
    #[inline(always)]
    pub const fn vtr(&self) -> &VTR {
        &self.vtr
    }
    ///0x08 - GICH virtual machine control register
    #[inline(always)]
    pub const fn vmcr(&self) -> &VMCR {
        &self.vmcr
    }
    ///0x10 - GICH maintenance interrupt status register
    #[inline(always)]
    pub const fn misr(&self) -> &MISR {
        &self.misr
    }
    ///0x20 - GICH end of interrupt status register
    #[inline(always)]
    pub const fn eisr0(&self) -> &EISR0 {
        &self.eisr0
    }
    ///0x30 - GICH empty list status register
    #[inline(always)]
    pub const fn elsr0(&self) -> &ELSR0 {
        &self.elsr0
    }
    ///0xf0 - GICH active priority register
    #[inline(always)]
    pub const fn apr0(&self) -> &APR0 {
        &self.apr0
    }
    ///0x100 - GICH list register 0
    #[inline(always)]
    pub const fn lr0(&self) -> &LR0 {
        &self.lr0
    }
    ///0x104 - GICH list register 1
    #[inline(always)]
    pub const fn lr1(&self) -> &LR1 {
        &self.lr1
    }
    ///0x108 - GICH list register 2
    #[inline(always)]
    pub const fn lr2(&self) -> &LR2 {
        &self.lr2
    }
    ///0x10c - GICH list register 3
    #[inline(always)]
    pub const fn lr3(&self) -> &LR3 {
        &self.lr3
    }
}
/**HCR (rw) register accessor: GICH hypervisor control register

You can [`read`](crate::Reg::read) this register and get [`hcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICH:HCR)

For information about available fields see [`mod@hcr`] module*/
pub type HCR = crate::Reg<hcr::HCRrs>;
///GICH hypervisor control register
pub mod hcr;
/**VTR (r) register accessor: GICH VGIC type register

You can [`read`](crate::Reg::read) this register and get [`vtr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICH:VTR)

For information about available fields see [`mod@vtr`] module*/
pub type VTR = crate::Reg<vtr::VTRrs>;
///GICH VGIC type register
pub mod vtr;
/**VMCR (rw) register accessor: GICH virtual machine control register

You can [`read`](crate::Reg::read) this register and get [`vmcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vmcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICH:VMCR)

For information about available fields see [`mod@vmcr`] module*/
pub type VMCR = crate::Reg<vmcr::VMCRrs>;
///GICH virtual machine control register
pub mod vmcr;
/**MISR (r) register accessor: GICH maintenance interrupt status register

You can [`read`](crate::Reg::read) this register and get [`misr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICH:MISR)

For information about available fields see [`mod@misr`] module*/
pub type MISR = crate::Reg<misr::MISRrs>;
///GICH maintenance interrupt status register
pub mod misr;
/**EISR0 (r) register accessor: GICH end of interrupt status register

You can [`read`](crate::Reg::read) this register and get [`eisr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICH:EISR0)

For information about available fields see [`mod@eisr0`] module*/
pub type EISR0 = crate::Reg<eisr0::EISR0rs>;
///GICH end of interrupt status register
pub mod eisr0;
/**ELSR0 (r) register accessor: GICH empty list status register

You can [`read`](crate::Reg::read) this register and get [`elsr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICH:ELSR0)

For information about available fields see [`mod@elsr0`] module*/
pub type ELSR0 = crate::Reg<elsr0::ELSR0rs>;
///GICH empty list status register
pub mod elsr0;
/**APR0 (rw) register accessor: GICH active priority register

You can [`read`](crate::Reg::read) this register and get [`apr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICH:APR0)

For information about available fields see [`mod@apr0`] module*/
pub type APR0 = crate::Reg<apr0::APR0rs>;
///GICH active priority register
pub mod apr0;
/**LR0 (rw) register accessor: GICH list register 0

You can [`read`](crate::Reg::read) this register and get [`lr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICH:LR0)

For information about available fields see [`mod@lr0`] module*/
pub type LR0 = crate::Reg<lr0::LR0rs>;
///GICH list register 0
pub mod lr0;
/**LR1 (rw) register accessor: GICH list register 1

You can [`read`](crate::Reg::read) this register and get [`lr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICH:LR1)

For information about available fields see [`mod@lr1`] module*/
pub type LR1 = crate::Reg<lr1::LR1rs>;
///GICH list register 1
pub mod lr1;
/**LR2 (rw) register accessor: GICH list register 2

You can [`read`](crate::Reg::read) this register and get [`lr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICH:LR2)

For information about available fields see [`mod@lr2`] module*/
pub type LR2 = crate::Reg<lr2::LR2rs>;
///GICH list register 2
pub mod lr2;
/**LR3 (rw) register accessor: GICH list register 3

You can [`read`](crate::Reg::read) this register and get [`lr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICH:LR3)

For information about available fields see [`mod@lr3`] module*/
pub type LR3 = crate::Reg<lr3::LR3rs>;
///GICH list register 3
pub mod lr3;
