///Register `AHB5LPENR` reader
pub type R = crate::R<AHB5LPENRrs>;
///Register `AHB5LPENR` writer
pub type W = crate::W<AHB5LPENRrs>;
/**HPDMA1 low-power peripheral clock enable Set and reset by software.

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HPDMA1LPEN {
    ///0: The selected clock is disabled during csleep mode
    Disabled = 0,
    ///1: The selected clock is enabled during csleep mode
    Enabled = 1,
}
impl From<HPDMA1LPEN> for bool {
    #[inline(always)]
    fn from(variant: HPDMA1LPEN) -> Self {
        variant as u8 != 0
    }
}
///Field `HPDMA1LPEN` reader - HPDMA1 low-power peripheral clock enable Set and reset by software.
pub type HPDMA1LPEN_R = crate::BitReader<HPDMA1LPEN>;
impl HPDMA1LPEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HPDMA1LPEN {
        match self.bits {
            false => HPDMA1LPEN::Disabled,
            true => HPDMA1LPEN::Enabled,
        }
    }
    ///The selected clock is disabled during csleep mode
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HPDMA1LPEN::Disabled
    }
    ///The selected clock is enabled during csleep mode
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HPDMA1LPEN::Enabled
    }
}
///Field `HPDMA1LPEN` writer - HPDMA1 low-power peripheral clock enable Set and reset by software.
pub type HPDMA1LPEN_W<'a, REG> = crate::BitWriter<'a, REG, HPDMA1LPEN>;
impl<'a, REG> HPDMA1LPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The selected clock is disabled during csleep mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(HPDMA1LPEN::Disabled)
    }
    ///The selected clock is enabled during csleep mode
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(HPDMA1LPEN::Enabled)
    }
}
///Field `DMA2DLPEN` reader - DMA2D low-power peripheral clock enable Set and reset by software.
pub use HPDMA1LPEN_R as DMA2DLPEN_R;
///Field `FLITFLPEN` reader - FLITF low-power peripheral clock enable Set and reset by software.
pub use HPDMA1LPEN_R as FLITFLPEN_R;
///Field `JPEGLPEN` reader - JPEG clock enable during Sleep mode Set and reset by software.
pub use HPDMA1LPEN_R as JPEGLPEN_R;
///Field `FMCLPEN` reader - FMC and MCE3 peripheral clocks enable during Sleep mode Set and reset by software. The hardware prevents writing this bit if FMCCKP = 1. The peripheral clocks of the FMC are the kernel clock selected by FMCSEL, and the hclk5 bus interface clock.
pub use HPDMA1LPEN_R as FMCLPEN_R;
///Field `XSPI1LPEN` reader - XSPI1 and MCE1 low-power peripheral clock enable Set and reset by software. The hardware prevents writing this bit if XSPICKP = 1.
pub use HPDMA1LPEN_R as XSPI1LPEN_R;
///Field `SDMMC1LPEN` reader - SDMMC1 and SDMMC1 delay low-power peripheral clock enable Set and reset by software.
pub use HPDMA1LPEN_R as SDMMC1LPEN_R;
///Field `XSPI2LPEN` reader - XSPI2 and MCE2 low-power peripheral clock enable Set and reset by software. The hardware prevents writing this bit if XSPICKP = 1.
pub use HPDMA1LPEN_R as XSPI2LPEN_R;
///Field `XSPIMLPEN` reader - XSPIM low-power peripheral clock enable Set and reset by software.
pub use HPDMA1LPEN_R as XSPIMLPEN_R;
///Field `GFXMMULPEN` reader - GFXMMU low-power peripheral clock enable Set and reset by software.
pub use HPDMA1LPEN_R as GFXMMULPEN_R;
///Field `GPULPEN` reader - GPU low-power peripheral clock enable Set and reset by software.
pub use HPDMA1LPEN_R as GPULPEN_R;
///Field `DTCM1LPEN` reader - DTCM1 low-power peripheral clock enable Set and reset by software.
pub use HPDMA1LPEN_R as DTCM1LPEN_R;
///Field `DTCM2LPEN` reader - DTCM2 low-power peripheral clock enable Set and reset by software.
pub use HPDMA1LPEN_R as DTCM2LPEN_R;
///Field `ITCMLPEN` reader - ITCM low-power peripheral clock enable Set and reset by software.
pub use HPDMA1LPEN_R as ITCMLPEN_R;
/**Field `AXISRAMLPEN` reader - AXISRAM\[4:1\]
low-power peripheral clock enable Set and reset by software.*/
pub use HPDMA1LPEN_R as AXISRAMLPEN_R;
///Field `DMA2DLPEN` writer - DMA2D low-power peripheral clock enable Set and reset by software.
pub use HPDMA1LPEN_W as DMA2DLPEN_W;
///Field `FLITFLPEN` writer - FLITF low-power peripheral clock enable Set and reset by software.
pub use HPDMA1LPEN_W as FLITFLPEN_W;
///Field `JPEGLPEN` writer - JPEG clock enable during Sleep mode Set and reset by software.
pub use HPDMA1LPEN_W as JPEGLPEN_W;
///Field `FMCLPEN` writer - FMC and MCE3 peripheral clocks enable during Sleep mode Set and reset by software. The hardware prevents writing this bit if FMCCKP = 1. The peripheral clocks of the FMC are the kernel clock selected by FMCSEL, and the hclk5 bus interface clock.
pub use HPDMA1LPEN_W as FMCLPEN_W;
///Field `XSPI1LPEN` writer - XSPI1 and MCE1 low-power peripheral clock enable Set and reset by software. The hardware prevents writing this bit if XSPICKP = 1.
pub use HPDMA1LPEN_W as XSPI1LPEN_W;
///Field `SDMMC1LPEN` writer - SDMMC1 and SDMMC1 delay low-power peripheral clock enable Set and reset by software.
pub use HPDMA1LPEN_W as SDMMC1LPEN_W;
///Field `XSPI2LPEN` writer - XSPI2 and MCE2 low-power peripheral clock enable Set and reset by software. The hardware prevents writing this bit if XSPICKP = 1.
pub use HPDMA1LPEN_W as XSPI2LPEN_W;
///Field `XSPIMLPEN` writer - XSPIM low-power peripheral clock enable Set and reset by software.
pub use HPDMA1LPEN_W as XSPIMLPEN_W;
///Field `GFXMMULPEN` writer - GFXMMU low-power peripheral clock enable Set and reset by software.
pub use HPDMA1LPEN_W as GFXMMULPEN_W;
///Field `GPULPEN` writer - GPU low-power peripheral clock enable Set and reset by software.
pub use HPDMA1LPEN_W as GPULPEN_W;
///Field `DTCM1LPEN` writer - DTCM1 low-power peripheral clock enable Set and reset by software.
pub use HPDMA1LPEN_W as DTCM1LPEN_W;
///Field `DTCM2LPEN` writer - DTCM2 low-power peripheral clock enable Set and reset by software.
pub use HPDMA1LPEN_W as DTCM2LPEN_W;
///Field `ITCMLPEN` writer - ITCM low-power peripheral clock enable Set and reset by software.
pub use HPDMA1LPEN_W as ITCMLPEN_W;
/**Field `AXISRAMLPEN` writer - AXISRAM\[4:1\]
low-power peripheral clock enable Set and reset by software.*/
pub use HPDMA1LPEN_W as AXISRAMLPEN_W;
impl R {
    ///Bit 0 - HPDMA1 low-power peripheral clock enable Set and reset by software.
    #[inline(always)]
    pub fn hpdma1lpen(&self) -> HPDMA1LPEN_R {
        HPDMA1LPEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DMA2D low-power peripheral clock enable Set and reset by software.
    #[inline(always)]
    pub fn dma2dlpen(&self) -> DMA2DLPEN_R {
        DMA2DLPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - FLITF low-power peripheral clock enable Set and reset by software.
    #[inline(always)]
    pub fn flitflpen(&self) -> FLITFLPEN_R {
        FLITFLPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - JPEG clock enable during Sleep mode Set and reset by software.
    #[inline(always)]
    pub fn jpeglpen(&self) -> JPEGLPEN_R {
        JPEGLPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - FMC and MCE3 peripheral clocks enable during Sleep mode Set and reset by software. The hardware prevents writing this bit if FMCCKP = 1. The peripheral clocks of the FMC are the kernel clock selected by FMCSEL, and the hclk5 bus interface clock.
    #[inline(always)]
    pub fn fmclpen(&self) -> FMCLPEN_R {
        FMCLPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - XSPI1 and MCE1 low-power peripheral clock enable Set and reset by software. The hardware prevents writing this bit if XSPICKP = 1.
    #[inline(always)]
    pub fn xspi1lpen(&self) -> XSPI1LPEN_R {
        XSPI1LPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - SDMMC1 and SDMMC1 delay low-power peripheral clock enable Set and reset by software.
    #[inline(always)]
    pub fn sdmmc1lpen(&self) -> SDMMC1LPEN_R {
        SDMMC1LPEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 12 - XSPI2 and MCE2 low-power peripheral clock enable Set and reset by software. The hardware prevents writing this bit if XSPICKP = 1.
    #[inline(always)]
    pub fn xspi2lpen(&self) -> XSPI2LPEN_R {
        XSPI2LPEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - XSPIM low-power peripheral clock enable Set and reset by software.
    #[inline(always)]
    pub fn xspimlpen(&self) -> XSPIMLPEN_R {
        XSPIMLPEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 19 - GFXMMU low-power peripheral clock enable Set and reset by software.
    #[inline(always)]
    pub fn gfxmmulpen(&self) -> GFXMMULPEN_R {
        GFXMMULPEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - GPU low-power peripheral clock enable Set and reset by software.
    #[inline(always)]
    pub fn gpulpen(&self) -> GPULPEN_R {
        GPULPEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 28 - DTCM1 low-power peripheral clock enable Set and reset by software.
    #[inline(always)]
    pub fn dtcm1lpen(&self) -> DTCM1LPEN_R {
        DTCM1LPEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - DTCM2 low-power peripheral clock enable Set and reset by software.
    #[inline(always)]
    pub fn dtcm2lpen(&self) -> DTCM2LPEN_R {
        DTCM2LPEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - ITCM low-power peripheral clock enable Set and reset by software.
    #[inline(always)]
    pub fn itcmlpen(&self) -> ITCMLPEN_R {
        ITCMLPEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    /**Bit 31 - AXISRAM\[4:1\]
    low-power peripheral clock enable Set and reset by software.*/
    #[inline(always)]
    pub fn axisramlpen(&self) -> AXISRAMLPEN_R {
        AXISRAMLPEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB5LPENR")
            .field("hpdma1lpen", &self.hpdma1lpen())
            .field("dma2dlpen", &self.dma2dlpen())
            .field("flitflpen", &self.flitflpen())
            .field("jpeglpen", &self.jpeglpen())
            .field("fmclpen", &self.fmclpen())
            .field("xspi1lpen", &self.xspi1lpen())
            .field("sdmmc1lpen", &self.sdmmc1lpen())
            .field("xspi2lpen", &self.xspi2lpen())
            .field("xspimlpen", &self.xspimlpen())
            .field("gfxmmulpen", &self.gfxmmulpen())
            .field("gpulpen", &self.gpulpen())
            .field("dtcm1lpen", &self.dtcm1lpen())
            .field("dtcm2lpen", &self.dtcm2lpen())
            .field("itcmlpen", &self.itcmlpen())
            .field("axisramlpen", &self.axisramlpen())
            .finish()
    }
}
impl W {
    ///Bit 0 - HPDMA1 low-power peripheral clock enable Set and reset by software.
    #[inline(always)]
    pub fn hpdma1lpen(&mut self) -> HPDMA1LPEN_W<AHB5LPENRrs> {
        HPDMA1LPEN_W::new(self, 0)
    }
    ///Bit 1 - DMA2D low-power peripheral clock enable Set and reset by software.
    #[inline(always)]
    pub fn dma2dlpen(&mut self) -> DMA2DLPEN_W<AHB5LPENRrs> {
        DMA2DLPEN_W::new(self, 1)
    }
    ///Bit 2 - FLITF low-power peripheral clock enable Set and reset by software.
    #[inline(always)]
    pub fn flitflpen(&mut self) -> FLITFLPEN_W<AHB5LPENRrs> {
        FLITFLPEN_W::new(self, 2)
    }
    ///Bit 3 - JPEG clock enable during Sleep mode Set and reset by software.
    #[inline(always)]
    pub fn jpeglpen(&mut self) -> JPEGLPEN_W<AHB5LPENRrs> {
        JPEGLPEN_W::new(self, 3)
    }
    ///Bit 4 - FMC and MCE3 peripheral clocks enable during Sleep mode Set and reset by software. The hardware prevents writing this bit if FMCCKP = 1. The peripheral clocks of the FMC are the kernel clock selected by FMCSEL, and the hclk5 bus interface clock.
    #[inline(always)]
    pub fn fmclpen(&mut self) -> FMCLPEN_W<AHB5LPENRrs> {
        FMCLPEN_W::new(self, 4)
    }
    ///Bit 5 - XSPI1 and MCE1 low-power peripheral clock enable Set and reset by software. The hardware prevents writing this bit if XSPICKP = 1.
    #[inline(always)]
    pub fn xspi1lpen(&mut self) -> XSPI1LPEN_W<AHB5LPENRrs> {
        XSPI1LPEN_W::new(self, 5)
    }
    ///Bit 8 - SDMMC1 and SDMMC1 delay low-power peripheral clock enable Set and reset by software.
    #[inline(always)]
    pub fn sdmmc1lpen(&mut self) -> SDMMC1LPEN_W<AHB5LPENRrs> {
        SDMMC1LPEN_W::new(self, 8)
    }
    ///Bit 12 - XSPI2 and MCE2 low-power peripheral clock enable Set and reset by software. The hardware prevents writing this bit if XSPICKP = 1.
    #[inline(always)]
    pub fn xspi2lpen(&mut self) -> XSPI2LPEN_W<AHB5LPENRrs> {
        XSPI2LPEN_W::new(self, 12)
    }
    ///Bit 14 - XSPIM low-power peripheral clock enable Set and reset by software.
    #[inline(always)]
    pub fn xspimlpen(&mut self) -> XSPIMLPEN_W<AHB5LPENRrs> {
        XSPIMLPEN_W::new(self, 14)
    }
    ///Bit 19 - GFXMMU low-power peripheral clock enable Set and reset by software.
    #[inline(always)]
    pub fn gfxmmulpen(&mut self) -> GFXMMULPEN_W<AHB5LPENRrs> {
        GFXMMULPEN_W::new(self, 19)
    }
    ///Bit 20 - GPU low-power peripheral clock enable Set and reset by software.
    #[inline(always)]
    pub fn gpulpen(&mut self) -> GPULPEN_W<AHB5LPENRrs> {
        GPULPEN_W::new(self, 20)
    }
    ///Bit 28 - DTCM1 low-power peripheral clock enable Set and reset by software.
    #[inline(always)]
    pub fn dtcm1lpen(&mut self) -> DTCM1LPEN_W<AHB5LPENRrs> {
        DTCM1LPEN_W::new(self, 28)
    }
    ///Bit 29 - DTCM2 low-power peripheral clock enable Set and reset by software.
    #[inline(always)]
    pub fn dtcm2lpen(&mut self) -> DTCM2LPEN_W<AHB5LPENRrs> {
        DTCM2LPEN_W::new(self, 29)
    }
    ///Bit 30 - ITCM low-power peripheral clock enable Set and reset by software.
    #[inline(always)]
    pub fn itcmlpen(&mut self) -> ITCMLPEN_W<AHB5LPENRrs> {
        ITCMLPEN_W::new(self, 30)
    }
    /**Bit 31 - AXISRAM\[4:1\]
    low-power peripheral clock enable Set and reset by software.*/
    #[inline(always)]
    pub fn axisramlpen(&mut self) -> AXISRAMLPEN_W<AHB5LPENRrs> {
        AXISRAMLPEN_W::new(self, 31)
    }
}
/**RCC AHB5 low-power clock enable register

You can [`read`](crate::Reg::read) this register and get [`ahb5lpenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb5lpenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:AHB5LPENR)*/
pub struct AHB5LPENRrs;
impl crate::RegisterSpec for AHB5LPENRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb5lpenr::R`](R) reader structure
impl crate::Readable for AHB5LPENRrs {}
///`write(|w| ..)` method takes [`ahb5lpenr::W`](W) writer structure
impl crate::Writable for AHB5LPENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets AHB5LPENR to value 0xf018_513f
impl crate::Resettable for AHB5LPENRrs {
    const RESET_VALUE: u32 = 0xf018_513f;
}
