///Register `AHB3RSTR` reader
pub type R = crate::R<AHB3RSTRrs>;
///Register `AHB3RSTR` writer
pub type W = crate::W<AHB3RSTRrs>;
/**MDMA block reset

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MDMARST {
    ///1: Reset the selected module
    Reset = 1,
}
impl From<MDMARST> for bool {
    #[inline(always)]
    fn from(variant: MDMARST) -> Self {
        variant as u8 != 0
    }
}
///Field `MDMARST` reader - MDMA block reset
pub type MDMARST_R = crate::BitReader<MDMARST>;
impl MDMARST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<MDMARST> {
        match self.bits {
            true => Some(MDMARST::Reset),
            _ => None,
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == MDMARST::Reset
    }
}
///Field `MDMARST` writer - MDMA block reset
pub type MDMARST_W<'a, REG> = crate::BitWriter<'a, REG, MDMARST>;
impl<'a, REG> MDMARST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(MDMARST::Reset)
    }
}
///Field `DMA2DRST` reader - DMA2D block reset
pub use MDMARST_R as DMA2DRST_R;
///Field `FMCRST` reader - FMC block reset
pub use MDMARST_R as FMCRST_R;
///Field `OCTOSPI1RST` reader - QUADSPI and QUADSPI delay block reset
pub use MDMARST_R as OCTOSPI1RST_R;
///Field `SDMMC1RST` reader - SDMMC1 and SDMMC1 delay block reset
pub use MDMARST_R as SDMMC1RST_R;
///Field `OCTOSPI2RST` reader - OCTOSPI2 and OCTOSPI2 delay block reset Set and reset by software
pub use MDMARST_R as OCTOSPI2RST_R;
///Field `IOMNGRRST` reader - OCTOSPIM reset Set and reset by software
pub use MDMARST_R as IOMNGRRST_R;
///Field `OTFD1RST` reader - OTFD1 reset Set and reset by software Take care that resetting the OTFD means loosing the decryption key loaded during secure boot.
pub use MDMARST_R as OTFD1RST_R;
///Field `OTFD2RST` reader - OTFD2 reset Set and reset by software Take care that resetting the OTFD means loosing the decryption key loaded during secure boot.
pub use MDMARST_R as OTFD2RST_R;
///Field `CPURST` reader - CPU reset
pub use MDMARST_R as CPURST_R;
///Field `DMA2DRST` writer - DMA2D block reset
pub use MDMARST_W as DMA2DRST_W;
///Field `FMCRST` writer - FMC block reset
pub use MDMARST_W as FMCRST_W;
///Field `OCTOSPI1RST` writer - QUADSPI and QUADSPI delay block reset
pub use MDMARST_W as OCTOSPI1RST_W;
///Field `SDMMC1RST` writer - SDMMC1 and SDMMC1 delay block reset
pub use MDMARST_W as SDMMC1RST_W;
///Field `OCTOSPI2RST` writer - OCTOSPI2 and OCTOSPI2 delay block reset Set and reset by software
pub use MDMARST_W as OCTOSPI2RST_W;
///Field `IOMNGRRST` writer - OCTOSPIM reset Set and reset by software
pub use MDMARST_W as IOMNGRRST_W;
///Field `OTFD1RST` writer - OTFD1 reset Set and reset by software Take care that resetting the OTFD means loosing the decryption key loaded during secure boot.
pub use MDMARST_W as OTFD1RST_W;
///Field `OTFD2RST` writer - OTFD2 reset Set and reset by software Take care that resetting the OTFD means loosing the decryption key loaded during secure boot.
pub use MDMARST_W as OTFD2RST_W;
///Field `CPURST` writer - CPU reset
pub use MDMARST_W as CPURST_W;
impl R {
    ///Bit 0 - MDMA block reset
    #[inline(always)]
    pub fn mdmarst(&self) -> MDMARST_R {
        MDMARST_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - DMA2D block reset
    #[inline(always)]
    pub fn dma2drst(&self) -> DMA2DRST_R {
        DMA2DRST_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 12 - FMC block reset
    #[inline(always)]
    pub fn fmcrst(&self) -> FMCRST_R {
        FMCRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - QUADSPI and QUADSPI delay block reset
    #[inline(always)]
    pub fn octospi1rst(&self) -> OCTOSPI1RST_R {
        OCTOSPI1RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - SDMMC1 and SDMMC1 delay block reset
    #[inline(always)]
    pub fn sdmmc1rst(&self) -> SDMMC1RST_R {
        SDMMC1RST_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 19 - OCTOSPI2 and OCTOSPI2 delay block reset Set and reset by software
    #[inline(always)]
    pub fn octospi2rst(&self) -> OCTOSPI2RST_R {
        OCTOSPI2RST_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 21 - OCTOSPIM reset Set and reset by software
    #[inline(always)]
    pub fn iomngrrst(&self) -> IOMNGRRST_R {
        IOMNGRRST_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - OTFD1 reset Set and reset by software Take care that resetting the OTFD means loosing the decryption key loaded during secure boot.
    #[inline(always)]
    pub fn otfd1rst(&self) -> OTFD1RST_R {
        OTFD1RST_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - OTFD2 reset Set and reset by software Take care that resetting the OTFD means loosing the decryption key loaded during secure boot.
    #[inline(always)]
    pub fn otfd2rst(&self) -> OTFD2RST_R {
        OTFD2RST_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 31 - CPU reset
    #[inline(always)]
    pub fn cpurst(&self) -> CPURST_R {
        CPURST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB3RSTR")
            .field("mdmarst", &self.mdmarst())
            .field("dma2drst", &self.dma2drst())
            .field("fmcrst", &self.fmcrst())
            .field("octospi1rst", &self.octospi1rst())
            .field("sdmmc1rst", &self.sdmmc1rst())
            .field("octospi2rst", &self.octospi2rst())
            .field("iomngrrst", &self.iomngrrst())
            .field("otfd1rst", &self.otfd1rst())
            .field("otfd2rst", &self.otfd2rst())
            .field("cpurst", &self.cpurst())
            .finish()
    }
}
impl W {
    ///Bit 0 - MDMA block reset
    #[inline(always)]
    pub fn mdmarst(&mut self) -> MDMARST_W<'_, AHB3RSTRrs> {
        MDMARST_W::new(self, 0)
    }
    ///Bit 4 - DMA2D block reset
    #[inline(always)]
    pub fn dma2drst(&mut self) -> DMA2DRST_W<'_, AHB3RSTRrs> {
        DMA2DRST_W::new(self, 4)
    }
    ///Bit 12 - FMC block reset
    #[inline(always)]
    pub fn fmcrst(&mut self) -> FMCRST_W<'_, AHB3RSTRrs> {
        FMCRST_W::new(self, 12)
    }
    ///Bit 14 - QUADSPI and QUADSPI delay block reset
    #[inline(always)]
    pub fn octospi1rst(&mut self) -> OCTOSPI1RST_W<'_, AHB3RSTRrs> {
        OCTOSPI1RST_W::new(self, 14)
    }
    ///Bit 16 - SDMMC1 and SDMMC1 delay block reset
    #[inline(always)]
    pub fn sdmmc1rst(&mut self) -> SDMMC1RST_W<'_, AHB3RSTRrs> {
        SDMMC1RST_W::new(self, 16)
    }
    ///Bit 19 - OCTOSPI2 and OCTOSPI2 delay block reset Set and reset by software
    #[inline(always)]
    pub fn octospi2rst(&mut self) -> OCTOSPI2RST_W<'_, AHB3RSTRrs> {
        OCTOSPI2RST_W::new(self, 19)
    }
    ///Bit 21 - OCTOSPIM reset Set and reset by software
    #[inline(always)]
    pub fn iomngrrst(&mut self) -> IOMNGRRST_W<'_, AHB3RSTRrs> {
        IOMNGRRST_W::new(self, 21)
    }
    ///Bit 22 - OTFD1 reset Set and reset by software Take care that resetting the OTFD means loosing the decryption key loaded during secure boot.
    #[inline(always)]
    pub fn otfd1rst(&mut self) -> OTFD1RST_W<'_, AHB3RSTRrs> {
        OTFD1RST_W::new(self, 22)
    }
    ///Bit 23 - OTFD2 reset Set and reset by software Take care that resetting the OTFD means loosing the decryption key loaded during secure boot.
    #[inline(always)]
    pub fn otfd2rst(&mut self) -> OTFD2RST_W<'_, AHB3RSTRrs> {
        OTFD2RST_W::new(self, 23)
    }
    ///Bit 31 - CPU reset
    #[inline(always)]
    pub fn cpurst(&mut self) -> CPURST_W<'_, AHB3RSTRrs> {
        CPURST_W::new(self, 31)
    }
}
/**RCC AHB3 Reset Register

You can [`read`](crate::Reg::read) this register and get [`ahb3rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb3rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H725.html#RCC:AHB3RSTR)*/
pub struct AHB3RSTRrs;
impl crate::RegisterSpec for AHB3RSTRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb3rstr::R`](R) reader structure
impl crate::Readable for AHB3RSTRrs {}
///`write(|w| ..)` method takes [`ahb3rstr::W`](W) writer structure
impl crate::Writable for AHB3RSTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB3RSTR to value 0
impl crate::Resettable for AHB3RSTRrs {}
