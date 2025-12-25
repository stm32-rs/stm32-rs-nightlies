///Register `AHBRSTR` reader
pub type R = crate::R<AHBRSTRrs>;
///Register `AHBRSTR` writer
pub type W = crate::W<AHBRSTRrs>;
/**IO port A reset

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOARSTW {
    ///1: Reset the module
    Reset = 1,
}
impl From<GPIOARSTW> for bool {
    #[inline(always)]
    fn from(variant: GPIOARSTW) -> Self {
        variant as u8 != 0
    }
}
///Field `GPIOARST` reader - IO port A reset
pub type GPIOARST_R = crate::BitReader<GPIOARSTW>;
impl GPIOARST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<GPIOARSTW> {
        match self.bits {
            true => Some(GPIOARSTW::Reset),
            _ => None,
        }
    }
    ///Reset the module
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == GPIOARSTW::Reset
    }
}
///Field `GPIOARST` writer - IO port A reset
pub type GPIOARST_W<'a, REG> = crate::BitWriter<'a, REG, GPIOARSTW>;
impl<'a, REG> GPIOARST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Reset the module
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOARSTW::Reset)
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
///Field `GPIOHRST` reader - IO port H reset
pub use GPIOARST_R as GPIOHRST_R;
///Field `GPIOFRST` reader - IO port F reset
pub use GPIOARST_R as GPIOFRST_R;
///Field `GPIOGRST` reader - IO port G reset
pub use GPIOARST_R as GPIOGRST_R;
///Field `CRCRST` reader - CRC reset
pub use GPIOARST_R as CRCRST_R;
///Field `FLITFRST` reader - FLITF reset
pub use GPIOARST_R as FLITFRST_R;
///Field `DMA1RST` reader - DMA1 reset
pub use GPIOARST_R as DMA1RST_R;
///Field `DMA2RST` reader - DMA2 reset
pub use GPIOARST_R as DMA2RST_R;
///Field `FSMCRST` reader - FSMC reset
pub use GPIOARST_R as FSMCRST_R;
///Field `GPIOBRST` writer - IO port B reset
pub use GPIOARST_W as GPIOBRST_W;
///Field `GPIOCRST` writer - IO port C reset
pub use GPIOARST_W as GPIOCRST_W;
///Field `GPIODRST` writer - IO port D reset
pub use GPIOARST_W as GPIODRST_W;
///Field `GPIOERST` writer - IO port E reset
pub use GPIOARST_W as GPIOERST_W;
///Field `GPIOHRST` writer - IO port H reset
pub use GPIOARST_W as GPIOHRST_W;
///Field `GPIOFRST` writer - IO port F reset
pub use GPIOARST_W as GPIOFRST_W;
///Field `GPIOGRST` writer - IO port G reset
pub use GPIOARST_W as GPIOGRST_W;
///Field `CRCRST` writer - CRC reset
pub use GPIOARST_W as CRCRST_W;
///Field `FLITFRST` writer - FLITF reset
pub use GPIOARST_W as FLITFRST_W;
///Field `DMA1RST` writer - DMA1 reset
pub use GPIOARST_W as DMA1RST_W;
///Field `DMA2RST` writer - DMA2 reset
pub use GPIOARST_W as DMA2RST_W;
///Field `FSMCRST` writer - FSMC reset
pub use GPIOARST_W as FSMCRST_W;
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
    ///Bit 5 - IO port H reset
    #[inline(always)]
    pub fn gpiohrst(&self) -> GPIOHRST_R {
        GPIOHRST_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - IO port F reset
    #[inline(always)]
    pub fn gpiofrst(&self) -> GPIOFRST_R {
        GPIOFRST_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - IO port G reset
    #[inline(always)]
    pub fn gpiogrst(&self) -> GPIOGRST_R {
        GPIOGRST_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 12 - CRC reset
    #[inline(always)]
    pub fn crcrst(&self) -> CRCRST_R {
        CRCRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 15 - FLITF reset
    #[inline(always)]
    pub fn flitfrst(&self) -> FLITFRST_R {
        FLITFRST_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 24 - DMA1 reset
    #[inline(always)]
    pub fn dma1rst(&self) -> DMA1RST_R {
        DMA1RST_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - DMA2 reset
    #[inline(always)]
    pub fn dma2rst(&self) -> DMA2RST_R {
        DMA2RST_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 30 - FSMC reset
    #[inline(always)]
    pub fn fsmcrst(&self) -> FSMCRST_R {
        FSMCRST_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBRSTR")
            .field("gpioarst", &self.gpioarst())
            .field("fsmcrst", &self.fsmcrst())
            .field("dma2rst", &self.dma2rst())
            .field("dma1rst", &self.dma1rst())
            .field("flitfrst", &self.flitfrst())
            .field("crcrst", &self.crcrst())
            .field("gpiogrst", &self.gpiogrst())
            .field("gpiofrst", &self.gpiofrst())
            .field("gpiohrst", &self.gpiohrst())
            .field("gpioerst", &self.gpioerst())
            .field("gpiodrst", &self.gpiodrst())
            .field("gpiocrst", &self.gpiocrst())
            .field("gpiobrst", &self.gpiobrst())
            .finish()
    }
}
impl W {
    ///Bit 0 - IO port A reset
    #[inline(always)]
    pub fn gpioarst(&mut self) -> GPIOARST_W<'_, AHBRSTRrs> {
        GPIOARST_W::new(self, 0)
    }
    ///Bit 1 - IO port B reset
    #[inline(always)]
    pub fn gpiobrst(&mut self) -> GPIOBRST_W<'_, AHBRSTRrs> {
        GPIOBRST_W::new(self, 1)
    }
    ///Bit 2 - IO port C reset
    #[inline(always)]
    pub fn gpiocrst(&mut self) -> GPIOCRST_W<'_, AHBRSTRrs> {
        GPIOCRST_W::new(self, 2)
    }
    ///Bit 3 - IO port D reset
    #[inline(always)]
    pub fn gpiodrst(&mut self) -> GPIODRST_W<'_, AHBRSTRrs> {
        GPIODRST_W::new(self, 3)
    }
    ///Bit 4 - IO port E reset
    #[inline(always)]
    pub fn gpioerst(&mut self) -> GPIOERST_W<'_, AHBRSTRrs> {
        GPIOERST_W::new(self, 4)
    }
    ///Bit 5 - IO port H reset
    #[inline(always)]
    pub fn gpiohrst(&mut self) -> GPIOHRST_W<'_, AHBRSTRrs> {
        GPIOHRST_W::new(self, 5)
    }
    ///Bit 6 - IO port F reset
    #[inline(always)]
    pub fn gpiofrst(&mut self) -> GPIOFRST_W<'_, AHBRSTRrs> {
        GPIOFRST_W::new(self, 6)
    }
    ///Bit 7 - IO port G reset
    #[inline(always)]
    pub fn gpiogrst(&mut self) -> GPIOGRST_W<'_, AHBRSTRrs> {
        GPIOGRST_W::new(self, 7)
    }
    ///Bit 12 - CRC reset
    #[inline(always)]
    pub fn crcrst(&mut self) -> CRCRST_W<'_, AHBRSTRrs> {
        CRCRST_W::new(self, 12)
    }
    ///Bit 15 - FLITF reset
    #[inline(always)]
    pub fn flitfrst(&mut self) -> FLITFRST_W<'_, AHBRSTRrs> {
        FLITFRST_W::new(self, 15)
    }
    ///Bit 24 - DMA1 reset
    #[inline(always)]
    pub fn dma1rst(&mut self) -> DMA1RST_W<'_, AHBRSTRrs> {
        DMA1RST_W::new(self, 24)
    }
    ///Bit 25 - DMA2 reset
    #[inline(always)]
    pub fn dma2rst(&mut self) -> DMA2RST_W<'_, AHBRSTRrs> {
        DMA2RST_W::new(self, 25)
    }
    ///Bit 30 - FSMC reset
    #[inline(always)]
    pub fn fsmcrst(&mut self) -> FSMCRST_W<'_, AHBRSTRrs> {
        FSMCRST_W::new(self, 30)
    }
}
/**AHB peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`ahbrstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbrstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L100.html#RCC:AHBRSTR)*/
pub struct AHBRSTRrs;
impl crate::RegisterSpec for AHBRSTRrs {
    type Ux = u32;
}
///`read()` method returns [`ahbrstr::R`](R) reader structure
impl crate::Readable for AHBRSTRrs {}
///`write(|w| ..)` method takes [`ahbrstr::W`](W) writer structure
impl crate::Writable for AHBRSTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHBRSTR to value 0
impl crate::Resettable for AHBRSTRrs {}
