#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ccsr: [CCSR; 7],
}
impl RegisterBlock {
    #[doc = "0x00..0x1c - Comparator control/status register"]
    #[inline(always)]
    pub const fn ccsr(&self, n: usize) -> &CCSR {
        &self.ccsr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x1c - Comparator control/status register"]
    #[inline(always)]
    pub fn ccsr_iter(&self) -> impl Iterator<Item = &CCSR> {
        self.ccsr.iter()
    }
    #[doc = "0x00 - Comparator control/status register"]
    #[inline(always)]
    pub const fn c1csr(&self) -> &CCSR {
        self.ccsr(0)
    }
    #[doc = "0x04 - Comparator control/status register"]
    #[inline(always)]
    pub const fn c2csr(&self) -> &CCSR {
        self.ccsr(1)
    }
    #[doc = "0x08 - Comparator control/status register"]
    #[inline(always)]
    pub const fn c3csr(&self) -> &CCSR {
        self.ccsr(2)
    }
    #[doc = "0x0c - Comparator control/status register"]
    #[inline(always)]
    pub const fn c4csr(&self) -> &CCSR {
        self.ccsr(3)
    }
    #[doc = "0x10 - Comparator control/status register"]
    #[inline(always)]
    pub const fn c5csr(&self) -> &CCSR {
        self.ccsr(4)
    }
    #[doc = "0x14 - Comparator control/status register"]
    #[inline(always)]
    pub const fn c6csr(&self) -> &CCSR {
        self.ccsr(5)
    }
    #[doc = "0x18 - Comparator control/status register"]
    #[inline(always)]
    pub const fn c7csr(&self) -> &CCSR {
        self.ccsr(6)
    }
}
#[doc = "CCSR (rw) register accessor: Comparator control/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccsr`]
module"]
pub type CCSR = crate::Reg<ccsr::CCSRrs>;
#[doc = "Comparator control/status register"]
pub mod ccsr;
