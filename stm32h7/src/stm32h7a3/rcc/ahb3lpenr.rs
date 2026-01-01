///Register `AHB3LPENR` reader
pub type R = crate::R<AHB3LPENRrs>;
///Register `AHB3LPENR` writer
pub type W = crate::W<AHB3LPENRrs>;
/**MDMA clock enable during CSleep mode Set and reset by software.

Value on reset: 1*/
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
///Field `MDMALPEN` reader - MDMA clock enable during CSleep mode Set and reset by software.
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
///Field `MDMALPEN` writer - MDMA clock enable during CSleep mode Set and reset by software.
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
///Field `DMA2DLPEN` reader - DMA2D clock enable during CSleep mode Set and reset by software.
pub use MDMALPEN_R as DMA2DLPEN_R;
///Field `JPGDECLPEN` reader - JPGDEC clock enable during CSleep mode Set and reset by software.
pub use MDMALPEN_R as JPGDECLPEN_R;
///Field `DMA2DLPEN` writer - DMA2D clock enable during CSleep mode Set and reset by software.
pub use MDMALPEN_W as DMA2DLPEN_W;
///Field `JPGDECLPEN` writer - JPGDEC clock enable during CSleep mode Set and reset by software.
pub use MDMALPEN_W as JPGDECLPEN_W;
///Field `FLASHPREN` reader - Flash interface clock enable during csleep mode
pub type FLASHPREN_R = crate::BitReader;
///Field `FLASHPREN` writer - Flash interface clock enable during csleep mode
pub type FLASHPREN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMCLPEN` reader - FMC peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the FMC are the kernel clock selected by FMCSEL and provided to fmc_ker_ck input, and the rcc_hclk3 bus interface clock.
pub use MDMALPEN_R as FMCLPEN_R;
///Field `OCTOSPI1LPEN` reader - OCTOSPI1 and OCTOSPI1 delay clock enable during CSleep mode Set and reset by software.
pub use MDMALPEN_R as OCTOSPI1LPEN_R;
///Field `SDMMC1LPEN` reader - SDMMC1 and SDMMC1 delay clock enable during CSleep mode Set and reset by software.
pub use MDMALPEN_R as SDMMC1LPEN_R;
///Field `OCTOSPI2LPEN` reader - OCTOSPI2 and OCTOSPI2 delay clock enable during CSleep mode Set and reset by software.
pub use MDMALPEN_R as OCTOSPI2LPEN_R;
///Field `OCTOSPIMLPEN` reader - OCTOSPIM block clock enable during CSleep mode Set and reset by software.
pub use MDMALPEN_R as OCTOSPIMLPEN_R;
///Field `OTFD1LPEN` reader - OTFD1 block clock enable during CSleep mode Set and reset by software.
pub use MDMALPEN_R as OTFD1LPEN_R;
///Field `OTFD2LPEN` reader - OTFD2 block clock enable during CSleep mode Set and reset by software.
pub use MDMALPEN_R as OTFD2LPEN_R;
///Field `GFXMMULPEN` reader - GFXMMU block clock enable during CSleep mode Set and reset by software.
pub use MDMALPEN_R as GFXMMULPEN_R;
///Field `AXISRAM2LPEN` reader - AXISRAM2 block clock enable during CSleep mode Set and reset by software.
pub use MDMALPEN_R as AXISRAM2LPEN_R;
///Field `AXISRAM3LPEN` reader - AXISRAM3 block clock enable during CSleep mode Set and reset by software.
pub use MDMALPEN_R as AXISRAM3LPEN_R;
///Field `DTCM1LPEN` reader - DTCM1 block clock enable during CSleep mode Set and reset by software.
pub use MDMALPEN_R as DTCM1LPEN_R;
///Field `DTCM2LPEN` reader - DTCM2 block clock enable during CSleep mode Set and reset by software.
pub use MDMALPEN_R as DTCM2LPEN_R;
///Field `ITCMLPEN` reader - ITCM block clock enable during CSleep mode Set and reset by software.
pub use MDMALPEN_R as ITCMLPEN_R;
///Field `AXISRAM1LPEN` reader - AXISRAM1 block clock enable during CSleep mode Set and reset by software.
pub use MDMALPEN_R as AXISRAM1LPEN_R;
///Field `FMCLPEN` writer - FMC peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the FMC are the kernel clock selected by FMCSEL and provided to fmc_ker_ck input, and the rcc_hclk3 bus interface clock.
pub use MDMALPEN_W as FMCLPEN_W;
///Field `OCTOSPI1LPEN` writer - OCTOSPI1 and OCTOSPI1 delay clock enable during CSleep mode Set and reset by software.
pub use MDMALPEN_W as OCTOSPI1LPEN_W;
///Field `SDMMC1LPEN` writer - SDMMC1 and SDMMC1 delay clock enable during CSleep mode Set and reset by software.
pub use MDMALPEN_W as SDMMC1LPEN_W;
///Field `OCTOSPI2LPEN` writer - OCTOSPI2 and OCTOSPI2 delay clock enable during CSleep mode Set and reset by software.
pub use MDMALPEN_W as OCTOSPI2LPEN_W;
///Field `OCTOSPIMLPEN` writer - OCTOSPIM block clock enable during CSleep mode Set and reset by software.
pub use MDMALPEN_W as OCTOSPIMLPEN_W;
///Field `OTFD1LPEN` writer - OTFD1 block clock enable during CSleep mode Set and reset by software.
pub use MDMALPEN_W as OTFD1LPEN_W;
///Field `OTFD2LPEN` writer - OTFD2 block clock enable during CSleep mode Set and reset by software.
pub use MDMALPEN_W as OTFD2LPEN_W;
///Field `GFXMMULPEN` writer - GFXMMU block clock enable during CSleep mode Set and reset by software.
pub use MDMALPEN_W as GFXMMULPEN_W;
///Field `AXISRAM2LPEN` writer - AXISRAM2 block clock enable during CSleep mode Set and reset by software.
pub use MDMALPEN_W as AXISRAM2LPEN_W;
///Field `AXISRAM3LPEN` writer - AXISRAM3 block clock enable during CSleep mode Set and reset by software.
pub use MDMALPEN_W as AXISRAM3LPEN_W;
///Field `DTCM1LPEN` writer - DTCM1 block clock enable during CSleep mode Set and reset by software.
pub use MDMALPEN_W as DTCM1LPEN_W;
///Field `DTCM2LPEN` writer - DTCM2 block clock enable during CSleep mode Set and reset by software.
pub use MDMALPEN_W as DTCM2LPEN_W;
///Field `ITCMLPEN` writer - ITCM block clock enable during CSleep mode Set and reset by software.
pub use MDMALPEN_W as ITCMLPEN_W;
///Field `AXISRAM1LPEN` writer - AXISRAM1 block clock enable during CSleep mode Set and reset by software.
pub use MDMALPEN_W as AXISRAM1LPEN_W;
impl R {
    ///Bit 0 - MDMA clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn mdmalpen(&self) -> MDMALPEN_R {
        MDMALPEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - DMA2D clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn dma2dlpen(&self) -> DMA2DLPEN_R {
        DMA2DLPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - JPGDEC clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn jpgdeclpen(&self) -> JPGDECLPEN_R {
        JPGDECLPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - Flash interface clock enable during csleep mode
    #[inline(always)]
    pub fn flashpren(&self) -> FLASHPREN_R {
        FLASHPREN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 12 - FMC peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the FMC are the kernel clock selected by FMCSEL and provided to fmc_ker_ck input, and the rcc_hclk3 bus interface clock.
    #[inline(always)]
    pub fn fmclpen(&self) -> FMCLPEN_R {
        FMCLPEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - OCTOSPI1 and OCTOSPI1 delay clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn octospi1lpen(&self) -> OCTOSPI1LPEN_R {
        OCTOSPI1LPEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - SDMMC1 and SDMMC1 delay clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn sdmmc1lpen(&self) -> SDMMC1LPEN_R {
        SDMMC1LPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 19 - OCTOSPI2 and OCTOSPI2 delay clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn octospi2lpen(&self) -> OCTOSPI2LPEN_R {
        OCTOSPI2LPEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 21 - OCTOSPIM block clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn octospimlpen(&self) -> OCTOSPIMLPEN_R {
        OCTOSPIMLPEN_R::new(((self.bits >> 21) & 1) != 0)
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
    ///Bit 24 - GFXMMU block clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn gfxmmulpen(&self) -> GFXMMULPEN_R {
        GFXMMULPEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 26 - AXISRAM2 block clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn axisram2lpen(&self) -> AXISRAM2LPEN_R {
        AXISRAM2LPEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - AXISRAM3 block clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn axisram3lpen(&self) -> AXISRAM3LPEN_R {
        AXISRAM3LPEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - DTCM1 block clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn dtcm1lpen(&self) -> DTCM1LPEN_R {
        DTCM1LPEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - DTCM2 block clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn dtcm2lpen(&self) -> DTCM2LPEN_R {
        DTCM2LPEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - ITCM block clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn itcmlpen(&self) -> ITCMLPEN_R {
        ITCMLPEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - AXISRAM1 block clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn axisram1lpen(&self) -> AXISRAM1LPEN_R {
        AXISRAM1LPEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB3LPENR")
            .field("mdmalpen", &self.mdmalpen())
            .field("dma2dlpen", &self.dma2dlpen())
            .field("jpgdeclpen", &self.jpgdeclpen())
            .field("flashpren", &self.flashpren())
            .field("fmclpen", &self.fmclpen())
            .field("octospi1lpen", &self.octospi1lpen())
            .field("sdmmc1lpen", &self.sdmmc1lpen())
            .field("octospi2lpen", &self.octospi2lpen())
            .field("octospimlpen", &self.octospimlpen())
            .field("otfd1lpen", &self.otfd1lpen())
            .field("otfd2lpen", &self.otfd2lpen())
            .field("gfxmmulpen", &self.gfxmmulpen())
            .field("axisram2lpen", &self.axisram2lpen())
            .field("axisram3lpen", &self.axisram3lpen())
            .field("dtcm1lpen", &self.dtcm1lpen())
            .field("dtcm2lpen", &self.dtcm2lpen())
            .field("itcmlpen", &self.itcmlpen())
            .field("axisram1lpen", &self.axisram1lpen())
            .finish()
    }
}
impl W {
    ///Bit 0 - MDMA clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn mdmalpen(&mut self) -> MDMALPEN_W<'_, AHB3LPENRrs> {
        MDMALPEN_W::new(self, 0)
    }
    ///Bit 4 - DMA2D clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn dma2dlpen(&mut self) -> DMA2DLPEN_W<'_, AHB3LPENRrs> {
        DMA2DLPEN_W::new(self, 4)
    }
    ///Bit 5 - JPGDEC clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn jpgdeclpen(&mut self) -> JPGDECLPEN_W<'_, AHB3LPENRrs> {
        JPGDECLPEN_W::new(self, 5)
    }
    ///Bit 8 - Flash interface clock enable during csleep mode
    #[inline(always)]
    pub fn flashpren(&mut self) -> FLASHPREN_W<'_, AHB3LPENRrs> {
        FLASHPREN_W::new(self, 8)
    }
    ///Bit 12 - FMC peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the FMC are the kernel clock selected by FMCSEL and provided to fmc_ker_ck input, and the rcc_hclk3 bus interface clock.
    #[inline(always)]
    pub fn fmclpen(&mut self) -> FMCLPEN_W<'_, AHB3LPENRrs> {
        FMCLPEN_W::new(self, 12)
    }
    ///Bit 14 - OCTOSPI1 and OCTOSPI1 delay clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn octospi1lpen(&mut self) -> OCTOSPI1LPEN_W<'_, AHB3LPENRrs> {
        OCTOSPI1LPEN_W::new(self, 14)
    }
    ///Bit 16 - SDMMC1 and SDMMC1 delay clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn sdmmc1lpen(&mut self) -> SDMMC1LPEN_W<'_, AHB3LPENRrs> {
        SDMMC1LPEN_W::new(self, 16)
    }
    ///Bit 19 - OCTOSPI2 and OCTOSPI2 delay clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn octospi2lpen(&mut self) -> OCTOSPI2LPEN_W<'_, AHB3LPENRrs> {
        OCTOSPI2LPEN_W::new(self, 19)
    }
    ///Bit 21 - OCTOSPIM block clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn octospimlpen(&mut self) -> OCTOSPIMLPEN_W<'_, AHB3LPENRrs> {
        OCTOSPIMLPEN_W::new(self, 21)
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
    ///Bit 24 - GFXMMU block clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn gfxmmulpen(&mut self) -> GFXMMULPEN_W<'_, AHB3LPENRrs> {
        GFXMMULPEN_W::new(self, 24)
    }
    ///Bit 26 - AXISRAM2 block clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn axisram2lpen(&mut self) -> AXISRAM2LPEN_W<'_, AHB3LPENRrs> {
        AXISRAM2LPEN_W::new(self, 26)
    }
    ///Bit 27 - AXISRAM3 block clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn axisram3lpen(&mut self) -> AXISRAM3LPEN_W<'_, AHB3LPENRrs> {
        AXISRAM3LPEN_W::new(self, 27)
    }
    ///Bit 28 - DTCM1 block clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn dtcm1lpen(&mut self) -> DTCM1LPEN_W<'_, AHB3LPENRrs> {
        DTCM1LPEN_W::new(self, 28)
    }
    ///Bit 29 - DTCM2 block clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn dtcm2lpen(&mut self) -> DTCM2LPEN_W<'_, AHB3LPENRrs> {
        DTCM2LPEN_W::new(self, 29)
    }
    ///Bit 30 - ITCM block clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn itcmlpen(&mut self) -> ITCMLPEN_W<'_, AHB3LPENRrs> {
        ITCMLPEN_W::new(self, 30)
    }
    ///Bit 31 - AXISRAM1 block clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn axisram1lpen(&mut self) -> AXISRAM1LPEN_W<'_, AHB3LPENRrs> {
        AXISRAM1LPEN_W::new(self, 31)
    }
}
/**

You can [`read`](crate::Reg::read) this register and get [`ahb3lpenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb3lpenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7A3.html#RCC:AHB3LPENR)*/
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
///`reset()` method sets AHB3LPENR to value 0xfde9_5131
impl crate::Resettable for AHB3LPENRrs {
    const RESET_VALUE: u32 = 0xfde9_5131;
}
