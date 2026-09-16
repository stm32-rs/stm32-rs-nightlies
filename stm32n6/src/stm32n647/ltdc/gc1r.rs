///Register `GC1R` reader
pub type R = crate::R<GC1Rrs>;
///Field `WBCH` reader - width of blue channel output
pub type WBCH_R = crate::FieldReader;
///Field `WGCH` reader - width of green channel output
pub type WGCH_R = crate::FieldReader;
///Field `WRCH` reader - width of red channel output
pub type WRCH_R = crate::FieldReader;
///Field `PRBA` reader - precise blending ability
pub type PRBA_R = crate::BitReader;
///Field `DT` reader - dithering technique implemented
pub type DT_R = crate::FieldReader;
///Field `GCT` reader - gamma correction technique implemented
pub type GCT_R = crate::FieldReader;
///Field `SHRA` reader - shadow registers ability
pub type SHRA_R = crate::BitReader;
///Field `BCP` reader - background color programmability (unique color blended as background)
pub type BCP_R = crate::BitReader;
///Field `BBA` reader - background blending ability
pub type BBA_R = crate::BitReader;
///Field `LNIP` reader - line-IRQ: line position programmability
pub type LNIP_R = crate::BitReader;
///Field `TP` reader - timing programmability
pub type TP_R = crate::BitReader;
///Field `SPP` reader - sync polarity programmability
pub type SPP_R = crate::BitReader;
///Field `DWP` reader - dither width programmability
pub type DWP_R = crate::BitReader;
///Field `STRA` reader - status register ability
pub type STRA_R = crate::BitReader;
///Field `CRMA` reader - configuration reading mode ability
pub type CRMA_R = crate::BitReader;
///Field `BMA` reader - blind mode ability
pub type BMA_R = crate::BitReader;
impl R {
    ///Bits 0:3 - width of blue channel output
    #[inline(always)]
    pub fn wbch(&self) -> WBCH_R {
        WBCH_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - width of green channel output
    #[inline(always)]
    pub fn wgch(&self) -> WGCH_R {
        WGCH_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - width of red channel output
    #[inline(always)]
    pub fn wrch(&self) -> WRCH_R {
        WRCH_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 12 - precise blending ability
    #[inline(always)]
    pub fn prba(&self) -> PRBA_R {
        PRBA_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 14:15 - dithering technique implemented
    #[inline(always)]
    pub fn dt(&self) -> DT_R {
        DT_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bits 17:19 - gamma correction technique implemented
    #[inline(always)]
    pub fn gct(&self) -> GCT_R {
        GCT_R::new(((self.bits >> 17) & 7) as u8)
    }
    ///Bit 21 - shadow registers ability
    #[inline(always)]
    pub fn shra(&self) -> SHRA_R {
        SHRA_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - background color programmability (unique color blended as background)
    #[inline(always)]
    pub fn bcp(&self) -> BCP_R {
        BCP_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - background blending ability
    #[inline(always)]
    pub fn bba(&self) -> BBA_R {
        BBA_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - line-IRQ: line position programmability
    #[inline(always)]
    pub fn lnip(&self) -> LNIP_R {
        LNIP_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - timing programmability
    #[inline(always)]
    pub fn tp(&self) -> TP_R {
        TP_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 27 - sync polarity programmability
    #[inline(always)]
    pub fn spp(&self) -> SPP_R {
        SPP_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - dither width programmability
    #[inline(always)]
    pub fn dwp(&self) -> DWP_R {
        DWP_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - status register ability
    #[inline(always)]
    pub fn stra(&self) -> STRA_R {
        STRA_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - configuration reading mode ability
    #[inline(always)]
    pub fn crma(&self) -> CRMA_R {
        CRMA_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - blind mode ability
    #[inline(always)]
    pub fn bma(&self) -> BMA_R {
        BMA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GC1R")
            .field("wbch", &self.wbch())
            .field("wgch", &self.wgch())
            .field("wrch", &self.wrch())
            .field("prba", &self.prba())
            .field("dt", &self.dt())
            .field("gct", &self.gct())
            .field("shra", &self.shra())
            .field("bcp", &self.bcp())
            .field("bba", &self.bba())
            .field("lnip", &self.lnip())
            .field("tp", &self.tp())
            .field("spp", &self.spp())
            .field("dwp", &self.dwp())
            .field("stra", &self.stra())
            .field("crma", &self.crma())
            .field("bma", &self.bma())
            .finish()
    }
}
/**LTDC global configuration 1 register

You can [`read`](crate::Reg::read) this register and get [`gc1r::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#LTDC:GC1R)*/
pub struct GC1Rrs;
impl crate::RegisterSpec for GC1Rrs {
    type Ux = u32;
}
///`read()` method returns [`gc1r::R`](R) reader structure
impl crate::Readable for GC1Rrs {}
///`reset()` method sets GC1R to value 0x6be4_d888
impl crate::Resettable for GC1Rrs {
    const RESET_VALUE: u32 = 0x6be4_d888;
}
