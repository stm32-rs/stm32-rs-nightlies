#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    bcr1: BCR1,
    btr: (),
    _reserved2: [u8; 0x04],
    bcr: (),
    _reserved3: [u8; 0x58],
    pcr: (),
    _reserved4: [u8; 0x04],
    sr: (),
    _reserved5: [u8; 0x04],
    pmem: (),
    _reserved6: [u8; 0x04],
    patt: (),
    _reserved7: [u8; 0x08],
    eccr: (),
    _reserved8: [u8; 0x3c],
    pio4: PIO4,
    _reserved9: [u8; 0x50],
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
    ///0x60..0x6c - PC Card/NAND Flash control register %s
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `PCR2` register.</div>
    #[inline(always)]
    pub const fn pcr(&self, n: usize) -> &PCR {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(96)
                .add(32 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x60..0x6c - PC Card/NAND Flash control register %s
    #[inline(always)]
    pub fn pcr_iter(&self) -> impl Iterator<Item = &PCR> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(96)
                .add(32 * n)
                .cast()
        })
    }
    ///0x60 - PC Card/NAND Flash control register 2
    #[inline(always)]
    pub const fn pcr2(&self) -> &PCR {
        self.pcr(0)
    }
    ///0x80 - PC Card/NAND Flash control register 3
    #[inline(always)]
    pub const fn pcr3(&self) -> &PCR {
        self.pcr(1)
    }
    ///0xa0 - PC Card/NAND Flash control register 4
    #[inline(always)]
    pub const fn pcr4(&self) -> &PCR {
        self.pcr(2)
    }
    ///0x64..0x70 - FIFO status and interrupt register %s
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `SR2` register.</div>
    #[inline(always)]
    pub const fn sr(&self, n: usize) -> &SR {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(100)
                .add(32 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x64..0x70 - FIFO status and interrupt register %s
    #[inline(always)]
    pub fn sr_iter(&self) -> impl Iterator<Item = &SR> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(100)
                .add(32 * n)
                .cast()
        })
    }
    ///0x64 - FIFO status and interrupt register 2
    #[inline(always)]
    pub const fn sr2(&self) -> &SR {
        self.sr(0)
    }
    ///0x84 - FIFO status and interrupt register 3
    #[inline(always)]
    pub const fn sr3(&self) -> &SR {
        self.sr(1)
    }
    ///0xa4 - FIFO status and interrupt register 4
    #[inline(always)]
    pub const fn sr4(&self) -> &SR {
        self.sr(2)
    }
    ///0x68..0x74 - Common memory space timing register %s
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `PMEM2` register.</div>
    #[inline(always)]
    pub const fn pmem(&self, n: usize) -> &PMEM {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(104)
                .add(32 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x68..0x74 - Common memory space timing register %s
    #[inline(always)]
    pub fn pmem_iter(&self) -> impl Iterator<Item = &PMEM> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(104)
                .add(32 * n)
                .cast()
        })
    }
    ///0x68 - Common memory space timing register 2
    #[inline(always)]
    pub const fn pmem2(&self) -> &PMEM {
        self.pmem(0)
    }
    ///0x88 - Common memory space timing register 3
    #[inline(always)]
    pub const fn pmem3(&self) -> &PMEM {
        self.pmem(1)
    }
    ///0xa8 - Common memory space timing register 4
    #[inline(always)]
    pub const fn pmem4(&self) -> &PMEM {
        self.pmem(2)
    }
    ///0x6c..0x78 - Attribute memory space timing register %s
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `PATT2` register.</div>
    #[inline(always)]
    pub const fn patt(&self, n: usize) -> &PATT {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(108)
                .add(32 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x6c..0x78 - Attribute memory space timing register %s
    #[inline(always)]
    pub fn patt_iter(&self) -> impl Iterator<Item = &PATT> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(108)
                .add(32 * n)
                .cast()
        })
    }
    ///0x6c - Attribute memory space timing register 2
    #[inline(always)]
    pub const fn patt2(&self) -> &PATT {
        self.patt(0)
    }
    ///0x8c - Attribute memory space timing register 3
    #[inline(always)]
    pub const fn patt3(&self) -> &PATT {
        self.patt(1)
    }
    ///0xac - Attribute memory space timing register 4
    #[inline(always)]
    pub const fn patt4(&self) -> &PATT {
        self.patt(2)
    }
    ///0x74..0x7c - ECC result register %s
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `ECCR2` register.</div>
    #[inline(always)]
    pub const fn eccr(&self, n: usize) -> &ECCR {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(116)
                .add(32 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x74..0x7c - ECC result register %s
    #[inline(always)]
    pub fn eccr_iter(&self) -> impl Iterator<Item = &ECCR> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(116)
                .add(32 * n)
                .cast()
        })
    }
    ///0x74 - ECC result register 2
    #[inline(always)]
    pub const fn eccr2(&self) -> &ECCR {
        self.eccr(0)
    }
    ///0x94 - ECC result register 3
    #[inline(always)]
    pub const fn eccr3(&self) -> &ECCR {
        self.eccr(1)
    }
    ///0xb0 - I/O space timing register 4
    #[inline(always)]
    pub const fn pio4(&self) -> &PIO4 {
        &self.pio4
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

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F215.html#FSMC:BCR1)

For information about available fields see [`mod@bcr1`] module*/
pub type BCR1 = crate::Reg<bcr1::BCR1rs>;
///SRAM/NOR-Flash chip-select control register 1
pub mod bcr1;
/**BTR (rw) register accessor: SRAM/NOR-Flash chip-select timing register %s

You can [`read`](crate::Reg::read) this register and get [`btr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`btr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F215.html#FSMC:BTR[1])

For information about available fields see [`mod@btr`] module*/
pub type BTR = crate::Reg<btr::BTRrs>;
///SRAM/NOR-Flash chip-select timing register %s
pub mod btr;
/**BCR (rw) register accessor: SRAM/NOR-Flash chip-select control register %s

You can [`read`](crate::Reg::read) this register and get [`bcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F215.html#FSMC:BCR[2])

For information about available fields see [`mod@bcr`] module*/
pub type BCR = crate::Reg<bcr::BCRrs>;
///SRAM/NOR-Flash chip-select control register %s
pub mod bcr;
/**PCR (rw) register accessor: PC Card/NAND Flash control register %s

You can [`read`](crate::Reg::read) this register and get [`pcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F215.html#FSMC:PCR[2])

For information about available fields see [`mod@pcr`] module*/
pub type PCR = crate::Reg<pcr::PCRrs>;
///PC Card/NAND Flash control register %s
pub mod pcr;
/**SR (rw) register accessor: FIFO status and interrupt register %s

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F215.html#FSMC:SR[2])

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///FIFO status and interrupt register %s
pub mod sr;
/**PMEM (rw) register accessor: Common memory space timing register %s

You can [`read`](crate::Reg::read) this register and get [`pmem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F215.html#FSMC:PMEM[2])

For information about available fields see [`mod@pmem`] module*/
pub type PMEM = crate::Reg<pmem::PMEMrs>;
///Common memory space timing register %s
pub mod pmem;
/**PATT (rw) register accessor: Attribute memory space timing register %s

You can [`read`](crate::Reg::read) this register and get [`patt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`patt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F215.html#FSMC:PATT[2])

For information about available fields see [`mod@patt`] module*/
pub type PATT = crate::Reg<patt::PATTrs>;
///Attribute memory space timing register %s
pub mod patt;
/**ECCR (r) register accessor: ECC result register %s

You can [`read`](crate::Reg::read) this register and get [`eccr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F215.html#FSMC:ECCR[2])

For information about available fields see [`mod@eccr`] module*/
pub type ECCR = crate::Reg<eccr::ECCRrs>;
///ECC result register %s
pub mod eccr;
/**PIO4 (rw) register accessor: I/O space timing register 4

You can [`read`](crate::Reg::read) this register and get [`pio4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F215.html#FSMC:PIO4)

For information about available fields see [`mod@pio4`] module*/
pub type PIO4 = crate::Reg<pio4::PIO4rs>;
///I/O space timing register 4
pub mod pio4;
/**BWTR (rw) register accessor: SRAM/NOR-Flash write timing registers %s

You can [`read`](crate::Reg::read) this register and get [`bwtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bwtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F215.html#FSMC:BWTR[1])

For information about available fields see [`mod@bwtr`] module*/
pub type BWTR = crate::Reg<bwtr::BWTRrs>;
///SRAM/NOR-Flash write timing registers %s
pub mod bwtr;
