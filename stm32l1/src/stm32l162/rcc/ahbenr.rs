///Register `AHBENR` reader
pub type R = crate::R<AHBENRrs>;
///Register `AHBENR` writer
pub type W = crate::W<AHBENRrs>;
///Field `GPIOPAEN` reader - IO port A clock enable
pub type GPIOPAEN_R = crate::BitReader;
///Field `GPIOPAEN` writer - IO port A clock enable
pub type GPIOPAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOPBEN` reader - IO port B clock enable
pub type GPIOPBEN_R = crate::BitReader;
///Field `GPIOPBEN` writer - IO port B clock enable
pub type GPIOPBEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOPCEN` reader - IO port C clock enable
pub type GPIOPCEN_R = crate::BitReader;
///Field `GPIOPCEN` writer - IO port C clock enable
pub type GPIOPCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOPDEN` reader - IO port D clock enable
pub type GPIOPDEN_R = crate::BitReader;
///Field `GPIOPDEN` writer - IO port D clock enable
pub type GPIOPDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOPEEN` reader - IO port E clock enable
pub type GPIOPEEN_R = crate::BitReader;
///Field `GPIOPEEN` writer - IO port E clock enable
pub type GPIOPEEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOPHEN` reader - IO port H clock enable
pub type GPIOPHEN_R = crate::BitReader;
///Field `GPIOPHEN` writer - IO port H clock enable
pub type GPIOPHEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOPFEN` reader - IO port F clock enable
pub type GPIOPFEN_R = crate::BitReader;
///Field `GPIOPFEN` writer - IO port F clock enable
pub type GPIOPFEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOPGEN` reader - IO port G clock enable
pub type GPIOPGEN_R = crate::BitReader;
///Field `GPIOPGEN` writer - IO port G clock enable
pub type GPIOPGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRCEN` reader - CRC clock enable
pub type CRCEN_R = crate::BitReader;
///Field `CRCEN` writer - CRC clock enable
pub type CRCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLITFEN` reader - FLITF clock enable
pub type FLITFEN_R = crate::BitReader;
///Field `FLITFEN` writer - FLITF clock enable
pub type FLITFEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMA1EN` reader - DMA1 clock enable
pub type DMA1EN_R = crate::BitReader;
///Field `DMA1EN` writer - DMA1 clock enable
pub type DMA1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMA2EN` reader - DMA2 clock enable
pub type DMA2EN_R = crate::BitReader;
///Field `DMA2EN` writer - DMA2 clock enable
pub type DMA2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FSMCEN` reader - FSMCEN
pub type FSMCEN_R = crate::BitReader;
///Field `FSMCEN` writer - FSMCEN
pub type FSMCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - IO port A clock enable
    #[inline(always)]
    pub fn gpiopaen(&self) -> GPIOPAEN_R {
        GPIOPAEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - IO port B clock enable
    #[inline(always)]
    pub fn gpiopben(&self) -> GPIOPBEN_R {
        GPIOPBEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - IO port C clock enable
    #[inline(always)]
    pub fn gpiopcen(&self) -> GPIOPCEN_R {
        GPIOPCEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - IO port D clock enable
    #[inline(always)]
    pub fn gpiopden(&self) -> GPIOPDEN_R {
        GPIOPDEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - IO port E clock enable
    #[inline(always)]
    pub fn gpiopeen(&self) -> GPIOPEEN_R {
        GPIOPEEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - IO port H clock enable
    #[inline(always)]
    pub fn gpiophen(&self) -> GPIOPHEN_R {
        GPIOPHEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - IO port F clock enable
    #[inline(always)]
    pub fn gpiopfen(&self) -> GPIOPFEN_R {
        GPIOPFEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - IO port G clock enable
    #[inline(always)]
    pub fn gpiopgen(&self) -> GPIOPGEN_R {
        GPIOPGEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 12 - CRC clock enable
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 15 - FLITF clock enable
    #[inline(always)]
    pub fn flitfen(&self) -> FLITFEN_R {
        FLITFEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 24 - DMA1 clock enable
    #[inline(always)]
    pub fn dma1en(&self) -> DMA1EN_R {
        DMA1EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - DMA2 clock enable
    #[inline(always)]
    pub fn dma2en(&self) -> DMA2EN_R {
        DMA2EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 30 - FSMCEN
    #[inline(always)]
    pub fn fsmcen(&self) -> FSMCEN_R {
        FSMCEN_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBENR")
            .field("fsmcen", &self.fsmcen())
            .field("dma2en", &self.dma2en())
            .field("dma1en", &self.dma1en())
            .field("flitfen", &self.flitfen())
            .field("crcen", &self.crcen())
            .field("gpiopgen", &self.gpiopgen())
            .field("gpiopfen", &self.gpiopfen())
            .field("gpiophen", &self.gpiophen())
            .field("gpiopeen", &self.gpiopeen())
            .field("gpiopden", &self.gpiopden())
            .field("gpiopcen", &self.gpiopcen())
            .field("gpiopben", &self.gpiopben())
            .field("gpiopaen", &self.gpiopaen())
            .finish()
    }
}
impl W {
    ///Bit 0 - IO port A clock enable
    #[inline(always)]
    pub fn gpiopaen(&mut self) -> GPIOPAEN_W<'_, AHBENRrs> {
        GPIOPAEN_W::new(self, 0)
    }
    ///Bit 1 - IO port B clock enable
    #[inline(always)]
    pub fn gpiopben(&mut self) -> GPIOPBEN_W<'_, AHBENRrs> {
        GPIOPBEN_W::new(self, 1)
    }
    ///Bit 2 - IO port C clock enable
    #[inline(always)]
    pub fn gpiopcen(&mut self) -> GPIOPCEN_W<'_, AHBENRrs> {
        GPIOPCEN_W::new(self, 2)
    }
    ///Bit 3 - IO port D clock enable
    #[inline(always)]
    pub fn gpiopden(&mut self) -> GPIOPDEN_W<'_, AHBENRrs> {
        GPIOPDEN_W::new(self, 3)
    }
    ///Bit 4 - IO port E clock enable
    #[inline(always)]
    pub fn gpiopeen(&mut self) -> GPIOPEEN_W<'_, AHBENRrs> {
        GPIOPEEN_W::new(self, 4)
    }
    ///Bit 5 - IO port H clock enable
    #[inline(always)]
    pub fn gpiophen(&mut self) -> GPIOPHEN_W<'_, AHBENRrs> {
        GPIOPHEN_W::new(self, 5)
    }
    ///Bit 6 - IO port F clock enable
    #[inline(always)]
    pub fn gpiopfen(&mut self) -> GPIOPFEN_W<'_, AHBENRrs> {
        GPIOPFEN_W::new(self, 6)
    }
    ///Bit 7 - IO port G clock enable
    #[inline(always)]
    pub fn gpiopgen(&mut self) -> GPIOPGEN_W<'_, AHBENRrs> {
        GPIOPGEN_W::new(self, 7)
    }
    ///Bit 12 - CRC clock enable
    #[inline(always)]
    pub fn crcen(&mut self) -> CRCEN_W<'_, AHBENRrs> {
        CRCEN_W::new(self, 12)
    }
    ///Bit 15 - FLITF clock enable
    #[inline(always)]
    pub fn flitfen(&mut self) -> FLITFEN_W<'_, AHBENRrs> {
        FLITFEN_W::new(self, 15)
    }
    ///Bit 24 - DMA1 clock enable
    #[inline(always)]
    pub fn dma1en(&mut self) -> DMA1EN_W<'_, AHBENRrs> {
        DMA1EN_W::new(self, 24)
    }
    ///Bit 25 - DMA2 clock enable
    #[inline(always)]
    pub fn dma2en(&mut self) -> DMA2EN_W<'_, AHBENRrs> {
        DMA2EN_W::new(self, 25)
    }
    ///Bit 30 - FSMCEN
    #[inline(always)]
    pub fn fsmcen(&mut self) -> FSMCEN_W<'_, AHBENRrs> {
        FSMCEN_W::new(self, 30)
    }
}
/**AHB peripheral clock enable register

You can [`read`](crate::Reg::read) this register and get [`ahbenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L162.html#RCC:AHBENR)*/
pub struct AHBENRrs;
impl crate::RegisterSpec for AHBENRrs {
    type Ux = u32;
}
///`read()` method returns [`ahbenr::R`](R) reader structure
impl crate::Readable for AHBENRrs {}
///`write(|w| ..)` method takes [`ahbenr::W`](W) writer structure
impl crate::Writable for AHBENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHBENR to value 0x8000
impl crate::Resettable for AHBENRrs {
    const RESET_VALUE: u32 = 0x8000;
}
