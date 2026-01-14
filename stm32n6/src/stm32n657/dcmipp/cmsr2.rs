///Register `CMSR2` reader
pub type R = crate::R<CMSR2rs>;
///Field `ATXERRF` reader - AXI transfer error interrupt status flag for the IPPLUG.
pub type ATXERRF_R = crate::BitReader;
///Field `PRERRF` reader - Synchronization error raw interrupt status for the parallel interface.
pub type PRERRF_R = crate::BitReader;
///Field `P0LINEF` reader - Multi-line capture completed raw interrupt status for Pipe0
pub type P0LINEF_R = crate::BitReader;
///Field `P0FRAMEF` reader - Frame capture completed raw interrupt status for Pipe0
pub type P0FRAMEF_R = crate::BitReader;
///Field `P0VSYNCF` reader - VSYNC raw interrupt status for Pipe0
pub type P0VSYNCF_R = crate::BitReader;
///Field `P0LIMITF` reader - Limit raw interrupt status for Pipe0
pub type P0LIMITF_R = crate::BitReader;
///Field `P0OVRF` reader - Overrun raw interrupt status for Pipe0
pub type P0OVRF_R = crate::BitReader;
///Field `P1LINEF` reader - Multi-line capture completed raw interrupt status for Pipe1
pub type P1LINEF_R = crate::BitReader;
///Field `P1FRAMEF` reader - Frame capture completed raw interrupt status for Pipe1
pub type P1FRAMEF_R = crate::BitReader;
///Field `P1VSYNCF` reader - VSYNC raw interrupt status for Pipe1
pub type P1VSYNCF_R = crate::BitReader;
///Field `P1OVRF` reader - Overrun raw interrupt status for Pipe1
pub type P1OVRF_R = crate::BitReader;
///Field `P2LINEF` reader - Multi-line capture completed raw interrupt status for Pipe2
pub type P2LINEF_R = crate::BitReader;
///Field `P2FRAMEF` reader - Frame capture completed raw interrupt status for Pipe2
pub type P2FRAMEF_R = crate::BitReader;
///Field `P2VSYNCF` reader - VSYNC raw interrupt status for Pipe2
pub type P2VSYNCF_R = crate::BitReader;
///Field `P2OVRF` reader - Overrun raw interrupt status for Pipe2
pub type P2OVRF_R = crate::BitReader;
impl R {
    ///Bit 5 - AXI transfer error interrupt status flag for the IPPLUG.
    #[inline(always)]
    pub fn atxerrf(&self) -> ATXERRF_R {
        ATXERRF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Synchronization error raw interrupt status for the parallel interface.
    #[inline(always)]
    pub fn prerrf(&self) -> PRERRF_R {
        PRERRF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - Multi-line capture completed raw interrupt status for Pipe0
    #[inline(always)]
    pub fn p0linef(&self) -> P0LINEF_R {
        P0LINEF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Frame capture completed raw interrupt status for Pipe0
    #[inline(always)]
    pub fn p0framef(&self) -> P0FRAMEF_R {
        P0FRAMEF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - VSYNC raw interrupt status for Pipe0
    #[inline(always)]
    pub fn p0vsyncf(&self) -> P0VSYNCF_R {
        P0VSYNCF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 14 - Limit raw interrupt status for Pipe0
    #[inline(always)]
    pub fn p0limitf(&self) -> P0LIMITF_R {
        P0LIMITF_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Overrun raw interrupt status for Pipe0
    #[inline(always)]
    pub fn p0ovrf(&self) -> P0OVRF_R {
        P0OVRF_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Multi-line capture completed raw interrupt status for Pipe1
    #[inline(always)]
    pub fn p1linef(&self) -> P1LINEF_R {
        P1LINEF_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Frame capture completed raw interrupt status for Pipe1
    #[inline(always)]
    pub fn p1framef(&self) -> P1FRAMEF_R {
        P1FRAMEF_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - VSYNC raw interrupt status for Pipe1
    #[inline(always)]
    pub fn p1vsyncf(&self) -> P1VSYNCF_R {
        P1VSYNCF_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 23 - Overrun raw interrupt status for Pipe1
    #[inline(always)]
    pub fn p1ovrf(&self) -> P1OVRF_R {
        P1OVRF_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Multi-line capture completed raw interrupt status for Pipe2
    #[inline(always)]
    pub fn p2linef(&self) -> P2LINEF_R {
        P2LINEF_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Frame capture completed raw interrupt status for Pipe2
    #[inline(always)]
    pub fn p2framef(&self) -> P2FRAMEF_R {
        P2FRAMEF_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - VSYNC raw interrupt status for Pipe2
    #[inline(always)]
    pub fn p2vsyncf(&self) -> P2VSYNCF_R {
        P2VSYNCF_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 31 - Overrun raw interrupt status for Pipe2
    #[inline(always)]
    pub fn p2ovrf(&self) -> P2OVRF_R {
        P2OVRF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMSR2")
            .field("atxerrf", &self.atxerrf())
            .field("prerrf", &self.prerrf())
            .field("p0linef", &self.p0linef())
            .field("p0framef", &self.p0framef())
            .field("p0vsyncf", &self.p0vsyncf())
            .field("p0limitf", &self.p0limitf())
            .field("p0ovrf", &self.p0ovrf())
            .field("p1linef", &self.p1linef())
            .field("p1framef", &self.p1framef())
            .field("p1vsyncf", &self.p1vsyncf())
            .field("p1ovrf", &self.p1ovrf())
            .field("p2linef", &self.p2linef())
            .field("p2framef", &self.p2framef())
            .field("p2vsyncf", &self.p2vsyncf())
            .field("p2ovrf", &self.p2ovrf())
            .finish()
    }
}
/**DCMIPP common status register 2

You can [`read`](crate::Reg::read) this register and get [`cmsr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:CMSR2)*/
pub struct CMSR2rs;
impl crate::RegisterSpec for CMSR2rs {
    type Ux = u32;
}
///`read()` method returns [`cmsr2::R`](R) reader structure
impl crate::Readable for CMSR2rs {}
///`reset()` method sets CMSR2 to value 0
impl crate::Resettable for CMSR2rs {}
