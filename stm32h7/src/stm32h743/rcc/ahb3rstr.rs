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
///Field `JPGDECRST` reader - JPGDEC block reset
pub use MDMARST_R as JPGDECRST_R;
///Field `FMCRST` reader - FMC block reset
pub use MDMARST_R as FMCRST_R;
///Field `QSPIRST` reader - QUADSPI and QUADSPI delay block reset
pub use MDMARST_R as QSPIRST_R;
///Field `SDMMC1RST` reader - SDMMC1 and SDMMC1 delay block reset
pub use MDMARST_R as SDMMC1RST_R;
///Field `CPURST` reader - CPU reset
pub use MDMARST_R as CPURST_R;
///Field `DMA2DRST` writer - DMA2D block reset
pub use MDMARST_W as DMA2DRST_W;
///Field `JPGDECRST` writer - JPGDEC block reset
pub use MDMARST_W as JPGDECRST_W;
///Field `FMCRST` writer - FMC block reset
pub use MDMARST_W as FMCRST_W;
///Field `QSPIRST` writer - QUADSPI and QUADSPI delay block reset
pub use MDMARST_W as QSPIRST_W;
///Field `SDMMC1RST` writer - SDMMC1 and SDMMC1 delay block reset
pub use MDMARST_W as SDMMC1RST_W;
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
    ///Bit 5 - JPGDEC block reset
    #[inline(always)]
    pub fn jpgdecrst(&self) -> JPGDECRST_R {
        JPGDECRST_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 12 - FMC block reset
    #[inline(always)]
    pub fn fmcrst(&self) -> FMCRST_R {
        FMCRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - QUADSPI and QUADSPI delay block reset
    #[inline(always)]
    pub fn qspirst(&self) -> QSPIRST_R {
        QSPIRST_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - SDMMC1 and SDMMC1 delay block reset
    #[inline(always)]
    pub fn sdmmc1rst(&self) -> SDMMC1RST_R {
        SDMMC1RST_R::new(((self.bits >> 16) & 1) != 0)
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
            .field("jpgdecrst", &self.jpgdecrst())
            .field("fmcrst", &self.fmcrst())
            .field("qspirst", &self.qspirst())
            .field("sdmmc1rst", &self.sdmmc1rst())
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
    ///Bit 5 - JPGDEC block reset
    #[inline(always)]
    pub fn jpgdecrst(&mut self) -> JPGDECRST_W<'_, AHB3RSTRrs> {
        JPGDECRST_W::new(self, 5)
    }
    ///Bit 12 - FMC block reset
    #[inline(always)]
    pub fn fmcrst(&mut self) -> FMCRST_W<'_, AHB3RSTRrs> {
        FMCRST_W::new(self, 12)
    }
    ///Bit 14 - QUADSPI and QUADSPI delay block reset
    #[inline(always)]
    pub fn qspirst(&mut self) -> QSPIRST_W<'_, AHB3RSTRrs> {
        QSPIRST_W::new(self, 14)
    }
    ///Bit 16 - SDMMC1 and SDMMC1 delay block reset
    #[inline(always)]
    pub fn sdmmc1rst(&mut self) -> SDMMC1RST_W<'_, AHB3RSTRrs> {
        SDMMC1RST_W::new(self, 16)
    }
    ///Bit 31 - CPU reset
    #[inline(always)]
    pub fn cpurst(&mut self) -> CPURST_W<'_, AHB3RSTRrs> {
        CPURST_W::new(self, 31)
    }
}
/**RCC AHB3 Reset Register

You can [`read`](crate::Reg::read) this register and get [`ahb3rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb3rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743.html#RCC:AHB3RSTR)*/
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
