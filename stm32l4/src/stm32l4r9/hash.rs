#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr: CR,
    din: DIN,
    str: STR,
    hra: [HRA; 5],
    imr: IMR,
    sr: SR,
    _reserved6: [u8; 0xd0],
    csr: [CSR; 54],
    _reserved7: [u8; 0x0140],
    hr: [HR; 8],
}
impl RegisterBlock {
    ///0x00 - control register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x04 - data input register
    #[inline(always)]
    pub const fn din(&self) -> &DIN {
        &self.din
    }
    ///0x08 - start register
    #[inline(always)]
    pub const fn str(&self) -> &STR {
        &self.str
    }
    ///0x0c..0x20 - HASH digest register alias %s
    #[inline(always)]
    pub const fn hra(&self, n: usize) -> &HRA {
        &self.hra[n]
    }
    ///Iterator for array of:
    ///0x0c..0x20 - HASH digest register alias %s
    #[inline(always)]
    pub fn hra_iter(&self) -> impl Iterator<Item = &HRA> {
        self.hra.iter()
    }
    ///0x20 - interrupt enable register
    #[inline(always)]
    pub const fn imr(&self) -> &IMR {
        &self.imr
    }
    ///0x24 - status register
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0xf8..0x1d0 - HASH context swap register %s
    #[inline(always)]
    pub const fn csr(&self, n: usize) -> &CSR {
        &self.csr[n]
    }
    ///Iterator for array of:
    ///0xf8..0x1d0 - HASH context swap register %s
    #[inline(always)]
    pub fn csr_iter(&self) -> impl Iterator<Item = &CSR> {
        self.csr.iter()
    }
    ///0x310..0x330 - HASH digest register %s
    #[inline(always)]
    pub const fn hr(&self, n: usize) -> &HR {
        &self.hr[n]
    }
    ///Iterator for array of:
    ///0x310..0x330 - HASH digest register %s
    #[inline(always)]
    pub fn hr_iter(&self) -> impl Iterator<Item = &HR> {
        self.hr.iter()
    }
}
/**CR (rw) register accessor: control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#HASH:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///control register
pub mod cr;
/**DIN (rw) register accessor: data input register

You can [`read`](crate::Reg::read) this register and get [`din::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`din::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#HASH:DIN)

For information about available fields see [`mod@din`] module*/
pub type DIN = crate::Reg<din::DINrs>;
///data input register
pub mod din;
/**STR (rw) register accessor: start register

You can [`read`](crate::Reg::read) this register and get [`str::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`str::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#HASH:STR)

For information about available fields see [`mod@str`] module*/
pub type STR = crate::Reg<str::STRrs>;
///start register
pub mod str;
/**HRA (r) register accessor: HASH digest register alias %s

You can [`read`](crate::Reg::read) this register and get [`hra::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#HASH:HRA[0])

For information about available fields see [`mod@hra`] module*/
pub type HRA = crate::Reg<hra::HRArs>;
///HASH digest register alias %s
pub mod hra;
/**IMR (rw) register accessor: interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`imr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#HASH:IMR)

For information about available fields see [`mod@imr`] module*/
pub type IMR = crate::Reg<imr::IMRrs>;
///interrupt enable register
pub mod imr;
/**SR (rw) register accessor: status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#HASH:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///status register
pub mod sr;
/**CSR (rw) register accessor: HASH context swap register %s

You can [`read`](crate::Reg::read) this register and get [`csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#HASH:CSR[0])

For information about available fields see [`mod@csr`] module*/
pub type CSR = crate::Reg<csr::CSRrs>;
///HASH context swap register %s
pub mod csr;
/**HR (r) register accessor: HASH digest register %s

You can [`read`](crate::Reg::read) this register and get [`hr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#HASH:HR[0])

For information about available fields see [`mod@hr`] module*/
pub type HR = crate::Reg<hr::HRrs>;
///HASH digest register %s
pub mod hr;
