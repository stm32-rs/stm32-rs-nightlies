///Register `AHB1RSTR` reader
pub type R = crate::R<AHB1RSTRrs>;
///Register `AHB1RSTR` writer
pub type W = crate::W<AHB1RSTRrs>;
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
///Field `GPIOFRST` reader - IO port F reset
pub type GPIOFRST_R = crate::BitReader;
///Field `GPIOFRST` writer - IO port F reset
pub type GPIOFRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOGRST` reader - IO port G reset
pub type GPIOGRST_R = crate::BitReader;
///Field `GPIOGRST` writer - IO port G reset
pub type GPIOGRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOHRST` reader - IO port H reset
pub type GPIOHRST_R = crate::BitReader;
///Field `GPIOHRST` writer - IO port H reset
pub type GPIOHRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOIRST` reader - IO port I reset
pub type GPIOIRST_R = crate::BitReader;
///Field `GPIOIRST` writer - IO port I reset
pub type GPIOIRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRCRST` reader - CRC reset
pub type CRCRST_R = crate::BitReader;
///Field `CRCRST` writer - CRC reset
pub type CRCRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMA1RST` reader - DMA2 reset
pub type DMA1RST_R = crate::BitReader;
///Field `DMA1RST` writer - DMA2 reset
pub type DMA1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMA2RST` reader - DMA2 reset
pub type DMA2RST_R = crate::BitReader;
///Field `DMA2RST` writer - DMA2 reset
pub type DMA2RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ETHMACRST` reader - Ethernet MAC reset
pub type ETHMACRST_R = crate::BitReader;
///Field `ETHMACRST` writer - Ethernet MAC reset
pub type ETHMACRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OTGHSRST` reader - USB OTG HS module reset
pub type OTGHSRST_R = crate::BitReader;
///Field `OTGHSRST` writer - USB OTG HS module reset
pub type OTGHSRST_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    ///Bit 8 - IO port I reset
    #[inline(always)]
    pub fn gpioirst(&self) -> GPIOIRST_R {
        GPIOIRST_R::new(((self.bits >> 8) & 1) != 0)
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
    ///Bit 25 - Ethernet MAC reset
    #[inline(always)]
    pub fn ethmacrst(&self) -> ETHMACRST_R {
        ETHMACRST_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 29 - USB OTG HS module reset
    #[inline(always)]
    pub fn otghsrst(&self) -> OTGHSRST_R {
        OTGHSRST_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB1RSTR")
            .field("otghsrst", &self.otghsrst())
            .field("ethmacrst", &self.ethmacrst())
            .field("dma2rst", &self.dma2rst())
            .field("dma1rst", &self.dma1rst())
            .field("crcrst", &self.crcrst())
            .field("gpioirst", &self.gpioirst())
            .field("gpiohrst", &self.gpiohrst())
            .field("gpiogrst", &self.gpiogrst())
            .field("gpiofrst", &self.gpiofrst())
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
    ///Bit 8 - IO port I reset
    #[inline(always)]
    pub fn gpioirst(&mut self) -> GPIOIRST_W<'_, AHB1RSTRrs> {
        GPIOIRST_W::new(self, 8)
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
    ///Bit 25 - Ethernet MAC reset
    #[inline(always)]
    pub fn ethmacrst(&mut self) -> ETHMACRST_W<'_, AHB1RSTRrs> {
        ETHMACRST_W::new(self, 25)
    }
    ///Bit 29 - USB OTG HS module reset
    #[inline(always)]
    pub fn otghsrst(&mut self) -> OTGHSRST_W<'_, AHB1RSTRrs> {
        OTGHSRST_W::new(self, 29)
    }
}
/**AHB1 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`ahb1rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#RCC:AHB1RSTR)*/
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
