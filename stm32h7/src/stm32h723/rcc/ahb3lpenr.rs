///Register `AHB3LPENR` reader
pub type R = crate::R<AHB3LPENRrs>;
///Register `AHB3LPENR` writer
pub type W = crate::W<AHB3LPENRrs>;
/**MDMA Clock Enable During CSleep Mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MDMALPEN {
    ///0: The selected clock is disabled during csleep mode
    Disabled = 0,
    ///1: The selected clock is enabled during csleep mode
    Enabled = 1,
}
impl From<MDMALPEN> for bool {
    #[inline(always)]
    fn from(variant: MDMALPEN) -> Self {
        variant as u8 != 0
    }
}
///Field `MDMALPEN` reader - MDMA Clock Enable During CSleep Mode
pub type MDMALPEN_R = crate::BitReader<MDMALPEN>;
impl MDMALPEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MDMALPEN {
        match self.bits {
            false => MDMALPEN::Disabled,
            true => MDMALPEN::Enabled,
        }
    }
    ///The selected clock is disabled during csleep mode
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MDMALPEN::Disabled
    }
    ///The selected clock is enabled during csleep mode
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MDMALPEN::Enabled
    }
}
///Field `MDMALPEN` writer - MDMA Clock Enable During CSleep Mode
pub type MDMALPEN_W<'a, REG> = crate::BitWriter<'a, REG, MDMALPEN>;
impl<'a, REG> MDMALPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The selected clock is disabled during csleep mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MDMALPEN::Disabled)
    }
    ///The selected clock is enabled during csleep mode
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MDMALPEN::Enabled)
    }
}
///Field `DMA2DLPEN` reader - DMA2D Clock Enable During CSleep Mode
pub use MDMALPEN_R as DMA2DLPEN_R;
///Field `FLASHLPEN` reader - Flash interface Clock Enable During CSleep Mode
pub use MDMALPEN_R as FLASHLPEN_R;
///Field `FMCLPEN` reader - FMC Peripheral Clocks Enable During CSleep Mode
pub use MDMALPEN_R as FMCLPEN_R;
///Field `OCTO1LPEN` reader - QUADSPI and QUADSPI Delay Clock Enable During CSleep Mode
pub use MDMALPEN_R as OCTO1LPEN_R;
///Field `SDMMC1LPEN` reader - SDMMC1 and SDMMC1 Delay Clock Enable During CSleep Mode
pub use MDMALPEN_R as SDMMC1LPEN_R;
///Field `OCTO2LPEN` reader - OCTOSPI2 and OCTOSPI2 delay clock enable during CSleep mode Set and reset by software.
pub use MDMALPEN_R as OCTO2LPEN_R;
///Field `IOMNGRLPEN` reader - OCTOSPIM block clock enable during CSleep mode Set and reset by software.
pub use MDMALPEN_R as IOMNGRLPEN_R;
///Field `OTFD1LPEN` reader - OTFD1 block clock enable during CSleep mode Set and reset by software.
pub use MDMALPEN_R as OTFD1LPEN_R;
///Field `OTFD2LPEN` reader - OTFD2 block clock enable during CSleep mode Set and reset by software.
pub use MDMALPEN_R as OTFD2LPEN_R;
///Field `DTCM1LPEN` reader - D1DTCM1 Block Clock Enable During CSleep mode
pub use MDMALPEN_R as DTCM1LPEN_R;
///Field `DTCM2LPEN` reader - D1 DTCM2 Block Clock Enable During CSleep mode
pub use MDMALPEN_R as DTCM2LPEN_R;
///Field `ITCMLPEN` reader - D1ITCM Block Clock Enable During CSleep mode
pub use MDMALPEN_R as ITCMLPEN_R;
///Field `AXISRAMLPEN` reader - AXISRAM Block Clock Enable During CSleep mode
pub use MDMALPEN_R as AXISRAMLPEN_R;
///Field `DMA2DLPEN` writer - DMA2D Clock Enable During CSleep Mode
pub use MDMALPEN_W as DMA2DLPEN_W;
///Field `FLASHLPEN` writer - Flash interface Clock Enable During CSleep Mode
pub use MDMALPEN_W as FLASHLPEN_W;
///Field `FMCLPEN` writer - FMC Peripheral Clocks Enable During CSleep Mode
pub use MDMALPEN_W as FMCLPEN_W;
///Field `OCTO1LPEN` writer - QUADSPI and QUADSPI Delay Clock Enable During CSleep Mode
pub use MDMALPEN_W as OCTO1LPEN_W;
///Field `SDMMC1LPEN` writer - SDMMC1 and SDMMC1 Delay Clock Enable During CSleep Mode
pub use MDMALPEN_W as SDMMC1LPEN_W;
///Field `OCTO2LPEN` writer - OCTOSPI2 and OCTOSPI2 delay clock enable during CSleep mode Set and reset by software.
pub use MDMALPEN_W as OCTO2LPEN_W;
///Field `IOMNGRLPEN` writer - OCTOSPIM block clock enable during CSleep mode Set and reset by software.
pub use MDMALPEN_W as IOMNGRLPEN_W;
///Field `OTFD1LPEN` writer - OTFD1 block clock enable during CSleep mode Set and reset by software.
pub use MDMALPEN_W as OTFD1LPEN_W;
///Field `OTFD2LPEN` writer - OTFD2 block clock enable during CSleep mode Set and reset by software.
pub use MDMALPEN_W as OTFD2LPEN_W;
///Field `DTCM1LPEN` writer - D1DTCM1 Block Clock Enable During CSleep mode
pub use MDMALPEN_W as DTCM1LPEN_W;
///Field `DTCM2LPEN` writer - D1 DTCM2 Block Clock Enable During CSleep mode
pub use MDMALPEN_W as DTCM2LPEN_W;
///Field `ITCMLPEN` writer - D1ITCM Block Clock Enable During CSleep mode
pub use MDMALPEN_W as ITCMLPEN_W;
///Field `AXISRAMLPEN` writer - AXISRAM Block Clock Enable During CSleep mode
pub use MDMALPEN_W as AXISRAMLPEN_W;
impl R {
    ///Bit 0 - MDMA Clock Enable During CSleep Mode
    #[inline(always)]
    pub fn mdmalpen(&self) -> MDMALPEN_R {
        MDMALPEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - DMA2D Clock Enable During CSleep Mode
    #[inline(always)]
    pub fn dma2dlpen(&self) -> DMA2DLPEN_R {
        DMA2DLPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - Flash interface Clock Enable During CSleep Mode
    #[inline(always)]
    pub fn flashlpen(&self) -> FLASHLPEN_R {
        FLASHLPEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 12 - FMC Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    pub fn fmclpen(&self) -> FMCLPEN_R {
        FMCLPEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - QUADSPI and QUADSPI Delay Clock Enable During CSleep Mode
    #[inline(always)]
    pub fn octo1lpen(&self) -> OCTO1LPEN_R {
        OCTO1LPEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - SDMMC1 and SDMMC1 Delay Clock Enable During CSleep Mode
    #[inline(always)]
    pub fn sdmmc1lpen(&self) -> SDMMC1LPEN_R {
        SDMMC1LPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 19 - OCTOSPI2 and OCTOSPI2 delay clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn octo2lpen(&self) -> OCTO2LPEN_R {
        OCTO2LPEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 21 - OCTOSPIM block clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn iomngrlpen(&self) -> IOMNGRLPEN_R {
        IOMNGRLPEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - OTFD1 block clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn otfd1lpen(&self) -> OTFD1LPEN_R {
        OTFD1LPEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - OTFD2 block clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn otfd2lpen(&self) -> OTFD2LPEN_R {
        OTFD2LPEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 28 - D1DTCM1 Block Clock Enable During CSleep mode
    #[inline(always)]
    pub fn dtcm1lpen(&self) -> DTCM1LPEN_R {
        DTCM1LPEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - D1 DTCM2 Block Clock Enable During CSleep mode
    #[inline(always)]
    pub fn dtcm2lpen(&self) -> DTCM2LPEN_R {
        DTCM2LPEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - D1ITCM Block Clock Enable During CSleep mode
    #[inline(always)]
    pub fn itcmlpen(&self) -> ITCMLPEN_R {
        ITCMLPEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - AXISRAM Block Clock Enable During CSleep mode
    #[inline(always)]
    pub fn axisramlpen(&self) -> AXISRAMLPEN_R {
        AXISRAMLPEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB3LPENR")
            .field("mdmalpen", &self.mdmalpen())
            .field("dma2dlpen", &self.dma2dlpen())
            .field("flashlpen", &self.flashlpen())
            .field("fmclpen", &self.fmclpen())
            .field("octo1lpen", &self.octo1lpen())
            .field("sdmmc1lpen", &self.sdmmc1lpen())
            .field("octo2lpen", &self.octo2lpen())
            .field("iomngrlpen", &self.iomngrlpen())
            .field("otfd1lpen", &self.otfd1lpen())
            .field("otfd2lpen", &self.otfd2lpen())
            .field("dtcm1lpen", &self.dtcm1lpen())
            .field("dtcm2lpen", &self.dtcm2lpen())
            .field("itcmlpen", &self.itcmlpen())
            .field("axisramlpen", &self.axisramlpen())
            .finish()
    }
}
impl W {
    ///Bit 0 - MDMA Clock Enable During CSleep Mode
    #[inline(always)]
    pub fn mdmalpen(&mut self) -> MDMALPEN_W<'_, AHB3LPENRrs> {
        MDMALPEN_W::new(self, 0)
    }
    ///Bit 4 - DMA2D Clock Enable During CSleep Mode
    #[inline(always)]
    pub fn dma2dlpen(&mut self) -> DMA2DLPEN_W<'_, AHB3LPENRrs> {
        DMA2DLPEN_W::new(self, 4)
    }
    ///Bit 8 - Flash interface Clock Enable During CSleep Mode
    #[inline(always)]
    pub fn flashlpen(&mut self) -> FLASHLPEN_W<'_, AHB3LPENRrs> {
        FLASHLPEN_W::new(self, 8)
    }
    ///Bit 12 - FMC Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    pub fn fmclpen(&mut self) -> FMCLPEN_W<'_, AHB3LPENRrs> {
        FMCLPEN_W::new(self, 12)
    }
    ///Bit 14 - QUADSPI and QUADSPI Delay Clock Enable During CSleep Mode
    #[inline(always)]
    pub fn octo1lpen(&mut self) -> OCTO1LPEN_W<'_, AHB3LPENRrs> {
        OCTO1LPEN_W::new(self, 14)
    }
    ///Bit 16 - SDMMC1 and SDMMC1 Delay Clock Enable During CSleep Mode
    #[inline(always)]
    pub fn sdmmc1lpen(&mut self) -> SDMMC1LPEN_W<'_, AHB3LPENRrs> {
        SDMMC1LPEN_W::new(self, 16)
    }
    ///Bit 19 - OCTOSPI2 and OCTOSPI2 delay clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn octo2lpen(&mut self) -> OCTO2LPEN_W<'_, AHB3LPENRrs> {
        OCTO2LPEN_W::new(self, 19)
    }
    ///Bit 21 - OCTOSPIM block clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn iomngrlpen(&mut self) -> IOMNGRLPEN_W<'_, AHB3LPENRrs> {
        IOMNGRLPEN_W::new(self, 21)
    }
    ///Bit 22 - OTFD1 block clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn otfd1lpen(&mut self) -> OTFD1LPEN_W<'_, AHB3LPENRrs> {
        OTFD1LPEN_W::new(self, 22)
    }
    ///Bit 23 - OTFD2 block clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn otfd2lpen(&mut self) -> OTFD2LPEN_W<'_, AHB3LPENRrs> {
        OTFD2LPEN_W::new(self, 23)
    }
    ///Bit 28 - D1DTCM1 Block Clock Enable During CSleep mode
    #[inline(always)]
    pub fn dtcm1lpen(&mut self) -> DTCM1LPEN_W<'_, AHB3LPENRrs> {
        DTCM1LPEN_W::new(self, 28)
    }
    ///Bit 29 - D1 DTCM2 Block Clock Enable During CSleep mode
    #[inline(always)]
    pub fn dtcm2lpen(&mut self) -> DTCM2LPEN_W<'_, AHB3LPENRrs> {
        DTCM2LPEN_W::new(self, 29)
    }
    ///Bit 30 - D1ITCM Block Clock Enable During CSleep mode
    #[inline(always)]
    pub fn itcmlpen(&mut self) -> ITCMLPEN_W<'_, AHB3LPENRrs> {
        ITCMLPEN_W::new(self, 30)
    }
    ///Bit 31 - AXISRAM Block Clock Enable During CSleep mode
    #[inline(always)]
    pub fn axisramlpen(&mut self) -> AXISRAMLPEN_W<'_, AHB3LPENRrs> {
        AXISRAMLPEN_W::new(self, 31)
    }
}
/**RCC AHB3 Sleep Clock Register

You can [`read`](crate::Reg::read) this register and get [`ahb3lpenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb3lpenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H723.html#RCC:AHB3LPENR)*/
pub struct AHB3LPENRrs;
impl crate::RegisterSpec for AHB3LPENRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb3lpenr::R`](R) reader structure
impl crate::Readable for AHB3LPENRrs {}
///`write(|w| ..)` method takes [`ahb3lpenr::W`](W) writer structure
impl crate::Writable for AHB3LPENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB3LPENR to value 0
impl crate::Resettable for AHB3LPENRrs {}
