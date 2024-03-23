#[doc = "Register `C1_AHB3LPENR` reader"]
pub type R = crate::R<C1_AHB3LPENRrs>;
#[doc = "Register `C1_AHB3LPENR` writer"]
pub type W = crate::W<C1_AHB3LPENRrs>;
#[doc = "MDMA Clock Enable During CSleep Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MDMALPEN {
    #[doc = "0: The selected clock is disabled during csleep mode"]
    Disabled = 0,
    #[doc = "1: The selected clock is enabled during csleep mode"]
    Enabled = 1,
}
impl From<MDMALPEN> for bool {
    #[inline(always)]
    fn from(variant: MDMALPEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MDMALPEN` reader - MDMA Clock Enable During CSleep Mode"]
pub type MDMALPEN_R = crate::BitReader<MDMALPEN>;
impl MDMALPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MDMALPEN {
        match self.bits {
            false => MDMALPEN::Disabled,
            true => MDMALPEN::Enabled,
        }
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MDMALPEN::Disabled
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MDMALPEN::Enabled
    }
}
#[doc = "Field `MDMALPEN` writer - MDMA Clock Enable During CSleep Mode"]
pub type MDMALPEN_W<'a, REG> = crate::BitWriter<'a, REG, MDMALPEN>;
impl<'a, REG> MDMALPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MDMALPEN::Disabled)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MDMALPEN::Enabled)
    }
}
#[doc = "Field `DMA2DLPEN` reader - DMA2D Clock Enable During CSleep Mode"]
pub use MDMALPEN_R as DMA2DLPEN_R;
#[doc = "Field `JPGDECLPEN` reader - JPGDEC Clock Enable During CSleep Mode"]
pub use MDMALPEN_R as JPGDECLPEN_R;
#[doc = "Field `DMA2DLPEN` writer - DMA2D Clock Enable During CSleep Mode"]
pub use MDMALPEN_W as DMA2DLPEN_W;
#[doc = "Field `JPGDECLPEN` writer - JPGDEC Clock Enable During CSleep Mode"]
pub use MDMALPEN_W as JPGDECLPEN_W;
#[doc = "Field `FLASHPREN` reader - Flash interface clock enable during csleep mode"]
pub type FLASHPREN_R = crate::BitReader;
#[doc = "Field `FLASHPREN` writer - Flash interface clock enable during csleep mode"]
pub type FLASHPREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FMCLPEN` reader - FMC Peripheral Clocks Enable During CSleep Mode"]
pub use MDMALPEN_R as FMCLPEN_R;
#[doc = "Field `QSPILPEN` reader - QUADSPI and QUADSPI Delay Clock Enable During CSleep Mode"]
pub use MDMALPEN_R as QSPILPEN_R;
#[doc = "Field `SDMMC1LPEN` reader - SDMMC1 and SDMMC1 Delay Clock Enable During CSleep Mode"]
pub use MDMALPEN_R as SDMMC1LPEN_R;
#[doc = "Field `D1DTCM1LPEN` reader - D1DTCM1 Block Clock Enable During CSleep mode"]
pub use MDMALPEN_R as D1DTCM1LPEN_R;
#[doc = "Field `DTCM2LPEN` reader - D1 DTCM2 Block Clock Enable During CSleep mode"]
pub use MDMALPEN_R as DTCM2LPEN_R;
#[doc = "Field `ITCMLPEN` reader - D1ITCM Block Clock Enable During CSleep mode"]
pub use MDMALPEN_R as ITCMLPEN_R;
#[doc = "Field `AXISRAMLPEN` reader - AXISRAM Block Clock Enable During CSleep mode"]
pub use MDMALPEN_R as AXISRAMLPEN_R;
#[doc = "Field `FMCLPEN` writer - FMC Peripheral Clocks Enable During CSleep Mode"]
pub use MDMALPEN_W as FMCLPEN_W;
#[doc = "Field `QSPILPEN` writer - QUADSPI and QUADSPI Delay Clock Enable During CSleep Mode"]
pub use MDMALPEN_W as QSPILPEN_W;
#[doc = "Field `SDMMC1LPEN` writer - SDMMC1 and SDMMC1 Delay Clock Enable During CSleep Mode"]
pub use MDMALPEN_W as SDMMC1LPEN_W;
#[doc = "Field `D1DTCM1LPEN` writer - D1DTCM1 Block Clock Enable During CSleep mode"]
pub use MDMALPEN_W as D1DTCM1LPEN_W;
#[doc = "Field `DTCM2LPEN` writer - D1 DTCM2 Block Clock Enable During CSleep mode"]
pub use MDMALPEN_W as DTCM2LPEN_W;
#[doc = "Field `ITCMLPEN` writer - D1ITCM Block Clock Enable During CSleep mode"]
pub use MDMALPEN_W as ITCMLPEN_W;
#[doc = "Field `AXISRAMLPEN` writer - AXISRAM Block Clock Enable During CSleep mode"]
pub use MDMALPEN_W as AXISRAMLPEN_W;
impl R {
    #[doc = "Bit 0 - MDMA Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn mdmalpen(&self) -> MDMALPEN_R {
        MDMALPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - DMA2D Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn dma2dlpen(&self) -> DMA2DLPEN_R {
        DMA2DLPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - JPGDEC Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn jpgdeclpen(&self) -> JPGDECLPEN_R {
        JPGDECLPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Flash interface clock enable during csleep mode"]
    #[inline(always)]
    pub fn flashpren(&self) -> FLASHPREN_R {
        FLASHPREN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - FMC Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn fmclpen(&self) -> FMCLPEN_R {
        FMCLPEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - QUADSPI and QUADSPI Delay Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn qspilpen(&self) -> QSPILPEN_R {
        QSPILPEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - SDMMC1 and SDMMC1 Delay Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn sdmmc1lpen(&self) -> SDMMC1LPEN_R {
        SDMMC1LPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 28 - D1DTCM1 Block Clock Enable During CSleep mode"]
    #[inline(always)]
    pub fn d1dtcm1lpen(&self) -> D1DTCM1LPEN_R {
        D1DTCM1LPEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - D1 DTCM2 Block Clock Enable During CSleep mode"]
    #[inline(always)]
    pub fn dtcm2lpen(&self) -> DTCM2LPEN_R {
        DTCM2LPEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - D1ITCM Block Clock Enable During CSleep mode"]
    #[inline(always)]
    pub fn itcmlpen(&self) -> ITCMLPEN_R {
        ITCMLPEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - AXISRAM Block Clock Enable During CSleep mode"]
    #[inline(always)]
    pub fn axisramlpen(&self) -> AXISRAMLPEN_R {
        AXISRAMLPEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MDMA Clock Enable During CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mdmalpen(&mut self) -> MDMALPEN_W<C1_AHB3LPENRrs> {
        MDMALPEN_W::new(self, 0)
    }
    #[doc = "Bit 4 - DMA2D Clock Enable During CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn dma2dlpen(&mut self) -> DMA2DLPEN_W<C1_AHB3LPENRrs> {
        DMA2DLPEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - JPGDEC Clock Enable During CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn jpgdeclpen(&mut self) -> JPGDECLPEN_W<C1_AHB3LPENRrs> {
        JPGDECLPEN_W::new(self, 5)
    }
    #[doc = "Bit 8 - Flash interface clock enable during csleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn flashpren(&mut self) -> FLASHPREN_W<C1_AHB3LPENRrs> {
        FLASHPREN_W::new(self, 8)
    }
    #[doc = "Bit 12 - FMC Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmclpen(&mut self) -> FMCLPEN_W<C1_AHB3LPENRrs> {
        FMCLPEN_W::new(self, 12)
    }
    #[doc = "Bit 14 - QUADSPI and QUADSPI Delay Clock Enable During CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn qspilpen(&mut self) -> QSPILPEN_W<C1_AHB3LPENRrs> {
        QSPILPEN_W::new(self, 14)
    }
    #[doc = "Bit 16 - SDMMC1 and SDMMC1 Delay Clock Enable During CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc1lpen(&mut self) -> SDMMC1LPEN_W<C1_AHB3LPENRrs> {
        SDMMC1LPEN_W::new(self, 16)
    }
    #[doc = "Bit 28 - D1DTCM1 Block Clock Enable During CSleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn d1dtcm1lpen(&mut self) -> D1DTCM1LPEN_W<C1_AHB3LPENRrs> {
        D1DTCM1LPEN_W::new(self, 28)
    }
    #[doc = "Bit 29 - D1 DTCM2 Block Clock Enable During CSleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn dtcm2lpen(&mut self) -> DTCM2LPEN_W<C1_AHB3LPENRrs> {
        DTCM2LPEN_W::new(self, 29)
    }
    #[doc = "Bit 30 - D1ITCM Block Clock Enable During CSleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn itcmlpen(&mut self) -> ITCMLPEN_W<C1_AHB3LPENRrs> {
        ITCMLPEN_W::new(self, 30)
    }
    #[doc = "Bit 31 - AXISRAM Block Clock Enable During CSleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn axisramlpen(&mut self) -> AXISRAMLPEN_W<C1_AHB3LPENRrs> {
        AXISRAMLPEN_W::new(self, 31)
    }
}
#[doc = "RCC AHB3 Sleep Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1_ahb3lpenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1_ahb3lpenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C1_AHB3LPENRrs;
impl crate::RegisterSpec for C1_AHB3LPENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c1_ahb3lpenr::R`](R) reader structure"]
impl crate::Readable for C1_AHB3LPENRrs {}
#[doc = "`write(|w| ..)` method takes [`c1_ahb3lpenr::W`](W) writer structure"]
impl crate::Writable for C1_AHB3LPENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C1_AHB3LPENR to value 0"]
impl crate::Resettable for C1_AHB3LPENRrs {
    const RESET_VALUE: u32 = 0;
}
