///Register `AHBRSTR` reader
pub type R = crate::R<AHBRSTRrs>;
///Register `AHBRSTR` writer
pub type W = crate::W<AHBRSTRrs>;
///Field `GPIOARST` reader - IO port A reset
pub type GPIOARST_R = crate::BitReader;
///Field `GPIOARST` writer - IO port A reset
pub type GPIOARST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOBRST` reader - IO port B reset
pub type GPIOBRST_R = crate::BitReader;
///Field `GPIOBRST` writer - IO port B reset
pub type GPIOBRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOCRST` reader - IO port C reset
pub type GPIOCRST_R = crate::BitReader;
///Field `GPIOCRST` writer - IO port C reset
pub type GPIOCRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIODRST` reader - IO port D reset
pub type GPIODRST_R = crate::BitReader;
///Field `GPIODRST` writer - IO port D reset
pub type GPIODRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOERST` reader - IO port E reset
pub type GPIOERST_R = crate::BitReader;
///Field `GPIOERST` writer - IO port E reset
pub type GPIOERST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOHRST` reader - IO port H reset
pub type GPIOHRST_R = crate::BitReader;
///Field `GPIOHRST` writer - IO port H reset
pub type GPIOHRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOFRST` reader - IO port F reset
pub type GPIOFRST_R = crate::BitReader;
///Field `GPIOFRST` writer - IO port F reset
pub type GPIOFRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOGRST` reader - IO port G reset
pub type GPIOGRST_R = crate::BitReader;
///Field `GPIOGRST` writer - IO port G reset
pub type GPIOGRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRCRST` reader - CRC reset
pub type CRCRST_R = crate::BitReader;
///Field `CRCRST` writer - CRC reset
pub type CRCRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLITFRST` reader - FLITF reset
pub type FLITFRST_R = crate::BitReader;
///Field `FLITFRST` writer - FLITF reset
pub type FLITFRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMA1RST` reader - DMA1 reset
pub type DMA1RST_R = crate::BitReader;
///Field `DMA1RST` writer - DMA1 reset
pub type DMA1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMA2RST` reader - DMA2 reset
pub type DMA2RST_R = crate::BitReader;
///Field `DMA2RST` writer - DMA2 reset
pub type DMA2RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FSMCRST` reader - FSMC reset
pub type FSMCRST_R = crate::BitReader;
///Field `FSMCRST` writer - FSMC reset
pub type FSMCRST_W<'a, REG> = crate::BitWriter<'a, REG>;
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
            .field("gpioarst", &self.gpioarst())
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

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L152.html#RCC:AHBRSTR)*/
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
