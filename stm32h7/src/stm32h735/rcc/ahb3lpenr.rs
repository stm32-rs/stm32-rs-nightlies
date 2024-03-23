#[doc = "Register `AHB3LPENR` reader"]
pub type R = crate::R<AHB3LPENRrs>;
#[doc = "Register `AHB3LPENR` writer"]
pub type W = crate::W<AHB3LPENRrs>;
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
#[doc = "Field `FLITFLPEN` reader - FLITF Clock Enable During CSleep Mode"]
pub use MDMALPEN_R as FLITFLPEN_R;
#[doc = "Field `FMCLPEN` reader - FMC Peripheral Clocks Enable During CSleep Mode"]
pub use MDMALPEN_R as FMCLPEN_R;
#[doc = "Field `OCTOSPI1LPEN` reader - OCTOSPI1 and OCTOSPI1 delay block enable during CSleep Mode"]
pub use MDMALPEN_R as OCTOSPI1LPEN_R;
#[doc = "Field `SDMMC1LPEN` reader - SDMMC1 and SDMMC1 Delay Clock Enable During CSleep Mode"]
pub use MDMALPEN_R as SDMMC1LPEN_R;
#[doc = "Field `OCTOSPI2LPEN` reader - OCTOSPI2 and OCTOSPI2 delay block enable during CSleep Mode"]
pub use MDMALPEN_R as OCTOSPI2LPEN_R;
#[doc = "Field `IOMNGRLPEN` reader - OCTOSPI IO manager enable during CSleep Mode"]
pub use MDMALPEN_R as IOMNGRLPEN_R;
#[doc = "Field `OTFD1LPEN` reader - OTFDEC1 enable during CSleep Mode"]
pub use MDMALPEN_R as OTFD1LPEN_R;
#[doc = "Field `OTFD2LPEN` reader - OTFDEC2 enable during CSleep Mode"]
pub use MDMALPEN_R as OTFD2LPEN_R;
#[doc = "Field `D1DTCM1LPEN` reader - D1DTCM1 Block Clock Enable During CSleep mode"]
pub use MDMALPEN_R as D1DTCM1LPEN_R;
#[doc = "Field `DTCM2LPEN` reader - D1 DTCM2 Block Clock Enable During CSleep mode"]
pub use MDMALPEN_R as DTCM2LPEN_R;
#[doc = "Field `ITCMLPEN` reader - D1ITCM Block Clock Enable During CSleep mode"]
pub use MDMALPEN_R as ITCMLPEN_R;
#[doc = "Field `AXISRAMLPEN` reader - AXISRAM Block Clock Enable During CSleep mode"]
pub use MDMALPEN_R as AXISRAMLPEN_R;
#[doc = "Field `DMA2DLPEN` writer - DMA2D Clock Enable During CSleep Mode"]
pub use MDMALPEN_W as DMA2DLPEN_W;
#[doc = "Field `FLITFLPEN` writer - FLITF Clock Enable During CSleep Mode"]
pub use MDMALPEN_W as FLITFLPEN_W;
#[doc = "Field `FMCLPEN` writer - FMC Peripheral Clocks Enable During CSleep Mode"]
pub use MDMALPEN_W as FMCLPEN_W;
#[doc = "Field `OCTOSPI1LPEN` writer - OCTOSPI1 and OCTOSPI1 delay block enable during CSleep Mode"]
pub use MDMALPEN_W as OCTOSPI1LPEN_W;
#[doc = "Field `SDMMC1LPEN` writer - SDMMC1 and SDMMC1 Delay Clock Enable During CSleep Mode"]
pub use MDMALPEN_W as SDMMC1LPEN_W;
#[doc = "Field `OCTOSPI2LPEN` writer - OCTOSPI2 and OCTOSPI2 delay block enable during CSleep Mode"]
pub use MDMALPEN_W as OCTOSPI2LPEN_W;
#[doc = "Field `IOMNGRLPEN` writer - OCTOSPI IO manager enable during CSleep Mode"]
pub use MDMALPEN_W as IOMNGRLPEN_W;
#[doc = "Field `OTFD1LPEN` writer - OTFDEC1 enable during CSleep Mode"]
pub use MDMALPEN_W as OTFD1LPEN_W;
#[doc = "Field `OTFD2LPEN` writer - OTFDEC2 enable during CSleep Mode"]
pub use MDMALPEN_W as OTFD2LPEN_W;
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
    #[doc = "Bit 8 - FLITF Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn flitflpen(&self) -> FLITFLPEN_R {
        FLITFLPEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - FMC Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn fmclpen(&self) -> FMCLPEN_R {
        FMCLPEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - OCTOSPI1 and OCTOSPI1 delay block enable during CSleep Mode"]
    #[inline(always)]
    pub fn octospi1lpen(&self) -> OCTOSPI1LPEN_R {
        OCTOSPI1LPEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - SDMMC1 and SDMMC1 Delay Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn sdmmc1lpen(&self) -> SDMMC1LPEN_R {
        SDMMC1LPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 19 - OCTOSPI2 and OCTOSPI2 delay block enable during CSleep Mode"]
    #[inline(always)]
    pub fn octospi2lpen(&self) -> OCTOSPI2LPEN_R {
        OCTOSPI2LPEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - OCTOSPI IO manager enable during CSleep Mode"]
    #[inline(always)]
    pub fn iomngrlpen(&self) -> IOMNGRLPEN_R {
        IOMNGRLPEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - OTFDEC1 enable during CSleep Mode"]
    #[inline(always)]
    pub fn otfd1lpen(&self) -> OTFD1LPEN_R {
        OTFD1LPEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - OTFDEC2 enable during CSleep Mode"]
    #[inline(always)]
    pub fn otfd2lpen(&self) -> OTFD2LPEN_R {
        OTFD2LPEN_R::new(((self.bits >> 23) & 1) != 0)
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
    pub fn mdmalpen(&mut self) -> MDMALPEN_W<AHB3LPENRrs> {
        MDMALPEN_W::new(self, 0)
    }
    #[doc = "Bit 4 - DMA2D Clock Enable During CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn dma2dlpen(&mut self) -> DMA2DLPEN_W<AHB3LPENRrs> {
        DMA2DLPEN_W::new(self, 4)
    }
    #[doc = "Bit 8 - FLITF Clock Enable During CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn flitflpen(&mut self) -> FLITFLPEN_W<AHB3LPENRrs> {
        FLITFLPEN_W::new(self, 8)
    }
    #[doc = "Bit 12 - FMC Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmclpen(&mut self) -> FMCLPEN_W<AHB3LPENRrs> {
        FMCLPEN_W::new(self, 12)
    }
    #[doc = "Bit 14 - OCTOSPI1 and OCTOSPI1 delay block enable during CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn octospi1lpen(&mut self) -> OCTOSPI1LPEN_W<AHB3LPENRrs> {
        OCTOSPI1LPEN_W::new(self, 14)
    }
    #[doc = "Bit 16 - SDMMC1 and SDMMC1 Delay Clock Enable During CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc1lpen(&mut self) -> SDMMC1LPEN_W<AHB3LPENRrs> {
        SDMMC1LPEN_W::new(self, 16)
    }
    #[doc = "Bit 19 - OCTOSPI2 and OCTOSPI2 delay block enable during CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn octospi2lpen(&mut self) -> OCTOSPI2LPEN_W<AHB3LPENRrs> {
        OCTOSPI2LPEN_W::new(self, 19)
    }
    #[doc = "Bit 21 - OCTOSPI IO manager enable during CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn iomngrlpen(&mut self) -> IOMNGRLPEN_W<AHB3LPENRrs> {
        IOMNGRLPEN_W::new(self, 21)
    }
    #[doc = "Bit 22 - OTFDEC1 enable during CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn otfd1lpen(&mut self) -> OTFD1LPEN_W<AHB3LPENRrs> {
        OTFD1LPEN_W::new(self, 22)
    }
    #[doc = "Bit 23 - OTFDEC2 enable during CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn otfd2lpen(&mut self) -> OTFD2LPEN_W<AHB3LPENRrs> {
        OTFD2LPEN_W::new(self, 23)
    }
    #[doc = "Bit 28 - D1DTCM1 Block Clock Enable During CSleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn d1dtcm1lpen(&mut self) -> D1DTCM1LPEN_W<AHB3LPENRrs> {
        D1DTCM1LPEN_W::new(self, 28)
    }
    #[doc = "Bit 29 - D1 DTCM2 Block Clock Enable During CSleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn dtcm2lpen(&mut self) -> DTCM2LPEN_W<AHB3LPENRrs> {
        DTCM2LPEN_W::new(self, 29)
    }
    #[doc = "Bit 30 - D1ITCM Block Clock Enable During CSleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn itcmlpen(&mut self) -> ITCMLPEN_W<AHB3LPENRrs> {
        ITCMLPEN_W::new(self, 30)
    }
    #[doc = "Bit 31 - AXISRAM Block Clock Enable During CSleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn axisramlpen(&mut self) -> AXISRAMLPEN_W<AHB3LPENRrs> {
        AXISRAMLPEN_W::new(self, 31)
    }
}
#[doc = "RCC AHB3 Sleep Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb3lpenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb3lpenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB3LPENRrs;
impl crate::RegisterSpec for AHB3LPENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb3lpenr::R`](R) reader structure"]
impl crate::Readable for AHB3LPENRrs {}
#[doc = "`write(|w| ..)` method takes [`ahb3lpenr::W`](W) writer structure"]
impl crate::Writable for AHB3LPENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHB3LPENR to value 0"]
impl crate::Resettable for AHB3LPENRrs {
    const RESET_VALUE: u32 = 0;
}
