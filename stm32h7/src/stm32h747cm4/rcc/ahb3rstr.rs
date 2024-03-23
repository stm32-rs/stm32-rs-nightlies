#[doc = "Register `AHB3RSTR` reader"]
pub type R = crate::R<AHB3RSTRrs>;
#[doc = "Register `AHB3RSTR` writer"]
pub type W = crate::W<AHB3RSTRrs>;
#[doc = "MDMA block reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MDMARST {
    #[doc = "1: Reset the selected module"]
    Reset = 1,
}
impl From<MDMARST> for bool {
    #[inline(always)]
    fn from(variant: MDMARST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MDMARST` reader - MDMA block reset"]
pub type MDMARST_R = crate::BitReader<MDMARST>;
impl MDMARST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MDMARST> {
        match self.bits {
            true => Some(MDMARST::Reset),
            _ => None,
        }
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == MDMARST::Reset
    }
}
#[doc = "Field `MDMARST` writer - MDMA block reset"]
pub type MDMARST_W<'a, REG> = crate::BitWriter<'a, REG, MDMARST>;
impl<'a, REG> MDMARST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(MDMARST::Reset)
    }
}
#[doc = "Field `DMA2DRST` reader - DMA2D block reset"]
pub use MDMARST_R as DMA2DRST_R;
#[doc = "Field `JPGDECRST` reader - JPGDEC block reset"]
pub use MDMARST_R as JPGDECRST_R;
#[doc = "Field `FMCRST` reader - FMC block reset"]
pub use MDMARST_R as FMCRST_R;
#[doc = "Field `QSPIRST` reader - QUADSPI and QUADSPI delay block reset"]
pub use MDMARST_R as QSPIRST_R;
#[doc = "Field `SDMMC1RST` reader - SDMMC1 and SDMMC1 delay block reset"]
pub use MDMARST_R as SDMMC1RST_R;
#[doc = "Field `CPURST` reader - CPU reset"]
pub use MDMARST_R as CPURST_R;
#[doc = "Field `DMA2DRST` writer - DMA2D block reset"]
pub use MDMARST_W as DMA2DRST_W;
#[doc = "Field `JPGDECRST` writer - JPGDEC block reset"]
pub use MDMARST_W as JPGDECRST_W;
#[doc = "Field `FMCRST` writer - FMC block reset"]
pub use MDMARST_W as FMCRST_W;
#[doc = "Field `QSPIRST` writer - QUADSPI and QUADSPI delay block reset"]
pub use MDMARST_W as QSPIRST_W;
#[doc = "Field `SDMMC1RST` writer - SDMMC1 and SDMMC1 delay block reset"]
pub use MDMARST_W as SDMMC1RST_W;
#[doc = "Field `CPURST` writer - CPU reset"]
pub use MDMARST_W as CPURST_W;
impl R {
    #[doc = "Bit 0 - MDMA block reset"]
    #[inline(always)]
    pub fn mdmarst(&self) -> MDMARST_R {
        MDMARST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - DMA2D block reset"]
    #[inline(always)]
    pub fn dma2drst(&self) -> DMA2DRST_R {
        DMA2DRST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - JPGDEC block reset"]
    #[inline(always)]
    pub fn jpgdecrst(&self) -> JPGDECRST_R {
        JPGDECRST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 12 - FMC block reset"]
    #[inline(always)]
    pub fn fmcrst(&self) -> FMCRST_R {
        FMCRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - QUADSPI and QUADSPI delay block reset"]
    #[inline(always)]
    pub fn qspirst(&self) -> QSPIRST_R {
        QSPIRST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - SDMMC1 and SDMMC1 delay block reset"]
    #[inline(always)]
    pub fn sdmmc1rst(&self) -> SDMMC1RST_R {
        SDMMC1RST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 31 - CPU reset"]
    #[inline(always)]
    pub fn cpurst(&self) -> CPURST_R {
        CPURST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MDMA block reset"]
    #[inline(always)]
    #[must_use]
    pub fn mdmarst(&mut self) -> MDMARST_W<AHB3RSTRrs> {
        MDMARST_W::new(self, 0)
    }
    #[doc = "Bit 4 - DMA2D block reset"]
    #[inline(always)]
    #[must_use]
    pub fn dma2drst(&mut self) -> DMA2DRST_W<AHB3RSTRrs> {
        DMA2DRST_W::new(self, 4)
    }
    #[doc = "Bit 5 - JPGDEC block reset"]
    #[inline(always)]
    #[must_use]
    pub fn jpgdecrst(&mut self) -> JPGDECRST_W<AHB3RSTRrs> {
        JPGDECRST_W::new(self, 5)
    }
    #[doc = "Bit 12 - FMC block reset"]
    #[inline(always)]
    #[must_use]
    pub fn fmcrst(&mut self) -> FMCRST_W<AHB3RSTRrs> {
        FMCRST_W::new(self, 12)
    }
    #[doc = "Bit 14 - QUADSPI and QUADSPI delay block reset"]
    #[inline(always)]
    #[must_use]
    pub fn qspirst(&mut self) -> QSPIRST_W<AHB3RSTRrs> {
        QSPIRST_W::new(self, 14)
    }
    #[doc = "Bit 16 - SDMMC1 and SDMMC1 delay block reset"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc1rst(&mut self) -> SDMMC1RST_W<AHB3RSTRrs> {
        SDMMC1RST_W::new(self, 16)
    }
    #[doc = "Bit 31 - CPU reset"]
    #[inline(always)]
    #[must_use]
    pub fn cpurst(&mut self) -> CPURST_W<AHB3RSTRrs> {
        CPURST_W::new(self, 31)
    }
}
#[doc = "RCC AHB3 Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb3rstr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb3rstr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB3RSTRrs;
impl crate::RegisterSpec for AHB3RSTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb3rstr::R`](R) reader structure"]
impl crate::Readable for AHB3RSTRrs {}
#[doc = "`write(|w| ..)` method takes [`ahb3rstr::W`](W) writer structure"]
impl crate::Writable for AHB3RSTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHB3RSTR to value 0"]
impl crate::Resettable for AHB3RSTRrs {
    const RESET_VALUE: u32 = 0;
}
