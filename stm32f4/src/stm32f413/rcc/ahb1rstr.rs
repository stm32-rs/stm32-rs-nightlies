///Register `AHB1RSTR` reader
pub type R = crate::R<AHB1RSTRrs>;
///Register `AHB1RSTR` writer
pub type W = crate::W<AHB1RSTRrs>;
/**IO port A reset

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOARST {
    ///1: Reset the selected module
    Reset = 1,
}
impl From<GPIOARST> for bool {
    #[inline(always)]
    fn from(variant: GPIOARST) -> Self {
        variant as u8 != 0
    }
}
///Field `GPIOARST` reader - IO port A reset
pub type GPIOARST_R = crate::BitReader<GPIOARST>;
impl GPIOARST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<GPIOARST> {
        match self.bits {
            true => Some(GPIOARST::Reset),
            _ => None,
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == GPIOARST::Reset
    }
}
///Field `GPIOARST` writer - IO port A reset
pub type GPIOARST_W<'a, REG> = crate::BitWriter<'a, REG, GPIOARST>;
impl<'a, REG> GPIOARST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOARST::Reset)
    }
}
///Field `GPIOBRST` reader - IO port B reset
pub use GPIOARST_R as GPIOBRST_R;
///Field `GPIOCRST` reader - IO port C reset
pub use GPIOARST_R as GPIOCRST_R;
///Field `GPIODRST` reader - IO port D reset
pub use GPIOARST_R as GPIODRST_R;
///Field `GPIOERST` reader - IO port E reset
pub use GPIOARST_R as GPIOERST_R;
///Field `GPIOFRST` reader - IO port F reset
pub use GPIOARST_R as GPIOFRST_R;
///Field `GPIOGRST` reader - IO port G reset
pub use GPIOARST_R as GPIOGRST_R;
///Field `GPIOHRST` reader - IO port H reset
pub use GPIOARST_R as GPIOHRST_R;
///Field `CRCRST` reader - CRC reset
pub use GPIOARST_R as CRCRST_R;
///Field `DMA1RST` reader - DMA2 reset
pub use GPIOARST_R as DMA1RST_R;
///Field `DMA2RST` reader - DMA2 reset
pub use GPIOARST_R as DMA2RST_R;
///Field `GPIOBRST` writer - IO port B reset
pub use GPIOARST_W as GPIOBRST_W;
///Field `GPIOCRST` writer - IO port C reset
pub use GPIOARST_W as GPIOCRST_W;
///Field `GPIODRST` writer - IO port D reset
pub use GPIOARST_W as GPIODRST_W;
///Field `GPIOERST` writer - IO port E reset
pub use GPIOARST_W as GPIOERST_W;
///Field `GPIOFRST` writer - IO port F reset
pub use GPIOARST_W as GPIOFRST_W;
///Field `GPIOGRST` writer - IO port G reset
pub use GPIOARST_W as GPIOGRST_W;
///Field `GPIOHRST` writer - IO port H reset
pub use GPIOARST_W as GPIOHRST_W;
///Field `CRCRST` writer - CRC reset
pub use GPIOARST_W as CRCRST_W;
///Field `DMA1RST` writer - DMA2 reset
pub use GPIOARST_W as DMA1RST_W;
///Field `DMA2RST` writer - DMA2 reset
pub use GPIOARST_W as DMA2RST_W;
impl R {
    ///Bit 0 - IO port A reset
    #[inline(always)]
    pub fn gpioarst(&self) -> GPIOARST_R {
        GPIOARST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - IO port B reset
    #[inline(always)]
    pub fn gpiobrst(&self) -> GPIOBRST_R {
        GPIOBRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - IO port C reset
    #[inline(always)]
    pub fn gpiocrst(&self) -> GPIOCRST_R {
        GPIOCRST_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - IO port D reset
    #[inline(always)]
    pub fn gpiodrst(&self) -> GPIODRST_R {
        GPIODRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - IO port E reset
    #[inline(always)]
    pub fn gpioerst(&self) -> GPIOERST_R {
        GPIOERST_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - IO port F reset
    #[inline(always)]
    pub fn gpiofrst(&self) -> GPIOFRST_R {
        GPIOFRST_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - IO port G reset
    #[inline(always)]
    pub fn gpiogrst(&self) -> GPIOGRST_R {
        GPIOGRST_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - IO port H reset
    #[inline(always)]
    pub fn gpiohrst(&self) -> GPIOHRST_R {
        GPIOHRST_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 12 - CRC reset
    #[inline(always)]
    pub fn crcrst(&self) -> CRCRST_R {
        CRCRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 21 - DMA2 reset
    #[inline(always)]
    pub fn dma1rst(&self) -> DMA1RST_R {
        DMA1RST_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - DMA2 reset
    #[inline(always)]
    pub fn dma2rst(&self) -> DMA2RST_R {
        DMA2RST_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB1RSTR")
            .field("gpioarst", &self.gpioarst())
            .field("gpiobrst", &self.gpiobrst())
            .field("gpiocrst", &self.gpiocrst())
            .field("gpiodrst", &self.gpiodrst())
            .field("gpioerst", &self.gpioerst())
            .field("gpiofrst", &self.gpiofrst())
            .field("gpiogrst", &self.gpiogrst())
            .field("gpiohrst", &self.gpiohrst())
            .field("crcrst", &self.crcrst())
            .field("dma1rst", &self.dma1rst())
            .field("dma2rst", &self.dma2rst())
            .finish()
    }
}
impl W {
    ///Bit 0 - IO port A reset
    #[inline(always)]
    pub fn gpioarst(&mut self) -> GPIOARST_W<'_, AHB1RSTRrs> {
        GPIOARST_W::new(self, 0)
    }
    ///Bit 1 - IO port B reset
    #[inline(always)]
    pub fn gpiobrst(&mut self) -> GPIOBRST_W<'_, AHB1RSTRrs> {
        GPIOBRST_W::new(self, 1)
    }
    ///Bit 2 - IO port C reset
    #[inline(always)]
    pub fn gpiocrst(&mut self) -> GPIOCRST_W<'_, AHB1RSTRrs> {
        GPIOCRST_W::new(self, 2)
    }
    ///Bit 3 - IO port D reset
    #[inline(always)]
    pub fn gpiodrst(&mut self) -> GPIODRST_W<'_, AHB1RSTRrs> {
        GPIODRST_W::new(self, 3)
    }
    ///Bit 4 - IO port E reset
    #[inline(always)]
    pub fn gpioerst(&mut self) -> GPIOERST_W<'_, AHB1RSTRrs> {
        GPIOERST_W::new(self, 4)
    }
    ///Bit 5 - IO port F reset
    #[inline(always)]
    pub fn gpiofrst(&mut self) -> GPIOFRST_W<'_, AHB1RSTRrs> {
        GPIOFRST_W::new(self, 5)
    }
    ///Bit 6 - IO port G reset
    #[inline(always)]
    pub fn gpiogrst(&mut self) -> GPIOGRST_W<'_, AHB1RSTRrs> {
        GPIOGRST_W::new(self, 6)
    }
    ///Bit 7 - IO port H reset
    #[inline(always)]
    pub fn gpiohrst(&mut self) -> GPIOHRST_W<'_, AHB1RSTRrs> {
        GPIOHRST_W::new(self, 7)
    }
    ///Bit 12 - CRC reset
    #[inline(always)]
    pub fn crcrst(&mut self) -> CRCRST_W<'_, AHB1RSTRrs> {
        CRCRST_W::new(self, 12)
    }
    ///Bit 21 - DMA2 reset
    #[inline(always)]
    pub fn dma1rst(&mut self) -> DMA1RST_W<'_, AHB1RSTRrs> {
        DMA1RST_W::new(self, 21)
    }
    ///Bit 22 - DMA2 reset
    #[inline(always)]
    pub fn dma2rst(&mut self) -> DMA2RST_W<'_, AHB1RSTRrs> {
        DMA2RST_W::new(self, 22)
    }
}
/**AHB1 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`ahb1rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F413.html#RCC:AHB1RSTR)*/
pub struct AHB1RSTRrs;
impl crate::RegisterSpec for AHB1RSTRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb1rstr::R`](R) reader structure
impl crate::Readable for AHB1RSTRrs {}
///`write(|w| ..)` method takes [`ahb1rstr::W`](W) writer structure
impl crate::Writable for AHB1RSTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB1RSTR to value 0
impl crate::Resettable for AHB1RSTRrs {}
