#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    ccsr: [CCSR; 7],
}
impl RegisterBlock {
    ///0x00..0x1c - Comparator control/status register
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `C1CSR` register.</div>
    #[inline(always)]
    pub const fn ccsr(&self, n: usize) -> &CCSR {
        &self.ccsr[n]
    }
    ///Iterator for array of:
    ///0x00..0x1c - Comparator control/status register
    #[inline(always)]
    pub fn ccsr_iter(&self) -> impl Iterator<Item = &CCSR> {
        self.ccsr.iter()
    }
    ///0x00 - Comparator control/status register
    #[inline(always)]
    pub const fn c1csr(&self) -> &CCSR {
        self.ccsr(0)
    }
    ///0x04 - Comparator control/status register
    #[inline(always)]
    pub const fn c2csr(&self) -> &CCSR {
        self.ccsr(1)
    }
    ///0x08 - Comparator control/status register
    #[inline(always)]
    pub const fn c3csr(&self) -> &CCSR {
        self.ccsr(2)
    }
    ///0x0c - Comparator control/status register
    #[inline(always)]
    pub const fn c4csr(&self) -> &CCSR {
        self.ccsr(3)
    }
    ///0x10 - Comparator control/status register
    #[inline(always)]
    pub const fn c5csr(&self) -> &CCSR {
        self.ccsr(4)
    }
    ///0x14 - Comparator control/status register
    #[inline(always)]
    pub const fn c6csr(&self) -> &CCSR {
        self.ccsr(5)
    }
    ///0x18 - Comparator control/status register
    #[inline(always)]
    pub const fn c7csr(&self) -> &CCSR {
        self.ccsr(6)
    }
}
/**CCSR (rw) register accessor: Comparator control/status register

You can [`read`](crate::Reg::read) this register and get [`ccsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G471.html#COMP:C[1]CSR)

For information about available fields see [`mod@ccsr`] module*/
pub type CCSR = crate::Reg<ccsr::CCSRrs>;
///Comparator control/status register
pub mod ccsr;
