#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    ccr: [CCR; 8],
}
impl RegisterBlock {
    ///0x00..0x20 - CxCR register
    #[inline(always)]
    pub const fn ccr(&self, n: usize) -> &CCR {
        &self.ccr[n]
    }
    ///Iterator for array of:
    ///0x00..0x20 - CxCR register
    #[inline(always)]
    pub fn ccr_iter(&self) -> impl Iterator<Item = &CCR> {
        self.ccr.iter()
    }
    ///0x00 - CxCR register
    #[inline(always)]
    pub const fn c0cr(&self) -> &CCR {
        self.ccr(0)
    }
    ///0x04 - CxCR register
    #[inline(always)]
    pub const fn c1cr(&self) -> &CCR {
        self.ccr(1)
    }
    ///0x08 - CxCR register
    #[inline(always)]
    pub const fn c2cr(&self) -> &CCR {
        self.ccr(2)
    }
    ///0x0c - CxCR register
    #[inline(always)]
    pub const fn c3cr(&self) -> &CCR {
        self.ccr(3)
    }
    ///0x10 - CxCR register
    #[inline(always)]
    pub const fn c4cr(&self) -> &CCR {
        self.ccr(4)
    }
    ///0x14 - CxCR register
    #[inline(always)]
    pub const fn c5cr(&self) -> &CCR {
        self.ccr(5)
    }
    ///0x18 - CxCR register
    #[inline(always)]
    pub const fn c6cr(&self) -> &CCR {
        self.ccr(6)
    }
    ///0x1c - CxCR register
    #[inline(always)]
    pub const fn c7cr(&self) -> &CCR {
        self.ccr(7)
    }
}
/**CCR (rw) register accessor: CxCR register

You can [`read`](crate::Reg::read) this register and get [`ccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB06.html#DMAMUX:C[0]CR)

For information about available fields see [`mod@ccr`] module*/
pub type CCR = crate::Reg<ccr::CCRrs>;
///CxCR register
pub mod ccr;
