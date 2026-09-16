#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    bcr1: BCR1,
    btr: (),
    _reserved2: [u8; 0x04],
    bcr: (),
    _reserved3: [u8; 0xfc],
    bwtr: (),
}
impl RegisterBlock {
    ///0x00 - SRAM/NOR-Flash chip-select control register 1
    #[inline(always)]
    pub const fn bcr1(&self) -> &BCR1 {
        &self.bcr1
    }
    ///0x04..0x14 - SRAM/NOR-Flash chip-select timing register %s
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `BTR1` register.</div>
    #[inline(always)]
    pub const fn btr(&self, n: usize) -> &BTR {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4)
                .add(8 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x04..0x14 - SRAM/NOR-Flash chip-select timing register %s
    #[inline(always)]
    pub fn btr_iter(&self) -> impl Iterator<Item = &BTR> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4)
                .add(8 * n)
                .cast()
        })
    }
    ///0x04 - SRAM/NOR-Flash chip-select timing register 1
    #[inline(always)]
    pub const fn btr1(&self) -> &BTR {
        self.btr(0)
    }
    ///0x0c - SRAM/NOR-Flash chip-select timing register 2
    #[inline(always)]
    pub const fn btr2(&self) -> &BTR {
        self.btr(1)
    }
    ///0x14 - SRAM/NOR-Flash chip-select timing register 3
    #[inline(always)]
    pub const fn btr3(&self) -> &BTR {
        self.btr(2)
    }
    ///0x1c - SRAM/NOR-Flash chip-select timing register 4
    #[inline(always)]
    pub const fn btr4(&self) -> &BTR {
        self.btr(3)
    }
    ///0x08..0x14 - SRAM/NOR-Flash chip-select control register %s
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `BCR2` register.</div>
    #[inline(always)]
    pub const fn bcr(&self, n: usize) -> &BCR {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(8)
                .add(8 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x08..0x14 - SRAM/NOR-Flash chip-select control register %s
    #[inline(always)]
    pub fn bcr_iter(&self) -> impl Iterator<Item = &BCR> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(8)
                .add(8 * n)
                .cast()
        })
    }
    ///0x08 - SRAM/NOR-Flash chip-select control register 2
    #[inline(always)]
    pub const fn bcr2(&self) -> &BCR {
        self.bcr(0)
    }
    ///0x10 - SRAM/NOR-Flash chip-select control register 3
    #[inline(always)]
    pub const fn bcr3(&self) -> &BCR {
        self.bcr(1)
    }
    ///0x18 - SRAM/NOR-Flash chip-select control register 4
    #[inline(always)]
    pub const fn bcr4(&self) -> &BCR {
        self.bcr(2)
    }
    ///0x104..0x114 - SRAM/NOR-Flash write timing registers %s
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `BWTR1` register.</div>
    #[inline(always)]
    pub const fn bwtr(&self, n: usize) -> &BWTR {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(260)
                .add(8 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x104..0x114 - SRAM/NOR-Flash write timing registers %s
    #[inline(always)]
    pub fn bwtr_iter(&self) -> impl Iterator<Item = &BWTR> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(260)
                .add(8 * n)
                .cast()
        })
    }
    ///0x104 - SRAM/NOR-Flash write timing registers 1
    #[inline(always)]
    pub const fn bwtr1(&self) -> &BWTR {
        self.bwtr(0)
    }
    ///0x10c - SRAM/NOR-Flash write timing registers 2
    #[inline(always)]
    pub const fn bwtr2(&self) -> &BWTR {
        self.bwtr(1)
    }
    ///0x114 - SRAM/NOR-Flash write timing registers 3
    #[inline(always)]
    pub const fn bwtr3(&self) -> &BWTR {
        self.bwtr(2)
    }
    ///0x11c - SRAM/NOR-Flash write timing registers 4
    #[inline(always)]
    pub const fn bwtr4(&self) -> &BWTR {
        self.bwtr(3)
    }
}
/**BCR1 (rw) register accessor: SRAM/NOR-Flash chip-select control register 1

You can [`read`](crate::Reg::read) this register and get [`bcr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F100.html#FSMC:BCR1)

For information about available fields see [`mod@bcr1`] module*/
pub type BCR1 = crate::Reg<bcr1::BCR1rs>;
///SRAM/NOR-Flash chip-select control register 1
pub mod bcr1;
/**BTR (rw) register accessor: SRAM/NOR-Flash chip-select timing register %s

You can [`read`](crate::Reg::read) this register and get [`btr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`btr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F100.html#FSMC:BTR[1])

For information about available fields see [`mod@btr`] module*/
pub type BTR = crate::Reg<btr::BTRrs>;
///SRAM/NOR-Flash chip-select timing register %s
pub mod btr;
/**BCR (rw) register accessor: SRAM/NOR-Flash chip-select control register %s

You can [`read`](crate::Reg::read) this register and get [`bcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F100.html#FSMC:BCR[2])

For information about available fields see [`mod@bcr`] module*/
pub type BCR = crate::Reg<bcr::BCRrs>;
///SRAM/NOR-Flash chip-select control register %s
pub mod bcr;
/**BWTR (rw) register accessor: SRAM/NOR-Flash write timing registers %s

You can [`read`](crate::Reg::read) this register and get [`bwtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bwtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F100.html#FSMC:BWTR[1])

For information about available fields see [`mod@bwtr`] module*/
pub type BWTR = crate::Reg<bwtr::BWTRrs>;
///SRAM/NOR-Flash write timing registers %s
pub mod bwtr;
