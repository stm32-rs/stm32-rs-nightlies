///Register `AHB1ENR` reader
pub type R = crate::R<AHB1ENRrs>;
///Register `AHB1ENR` writer
pub type W = crate::W<AHB1ENRrs>;
///Field `DMA1EN` reader - DMA1 clock enable
pub type DMA1EN_R = crate::BitReader;
///Field `DMA1EN` writer - DMA1 clock enable
pub type DMA1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMA2EN` reader - DMA2 clock enable
pub type DMA2EN_R = crate::BitReader;
///Field `DMA2EN` writer - DMA2 clock enable
pub type DMA2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLASHEN` reader - Flash memory interface clock enable
pub type FLASHEN_R = crate::BitReader;
///Field `FLASHEN` writer - Flash memory interface clock enable
pub type FLASHEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRCEN` reader - CRC clock enable
pub type CRCEN_R = crate::BitReader;
///Field `CRCEN` writer - CRC clock enable
pub type CRCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSCEN` reader - Touch Sensing Controller clock enable
pub type TSCEN_R = crate::BitReader;
///Field `TSCEN` writer - Touch Sensing Controller clock enable
pub type TSCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - DMA1 clock enable
    #[inline(always)]
    pub fn dma1en(&self) -> DMA1EN_R {
        DMA1EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DMA2 clock enable
    #[inline(always)]
    pub fn dma2en(&self) -> DMA2EN_R {
        DMA2EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 8 - Flash memory interface clock enable
    #[inline(always)]
    pub fn flashen(&self) -> FLASHEN_R {
        FLASHEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 12 - CRC clock enable
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 16 - Touch Sensing Controller clock enable
    #[inline(always)]
    pub fn tscen(&self) -> TSCEN_R {
        TSCEN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB1ENR")
            .field("tscen", &self.tscen())
            .field("crcen", &self.crcen())
            .field("flashen", &self.flashen())
            .field("dma2en", &self.dma2en())
            .field("dma1en", &self.dma1en())
            .finish()
    }
}
impl W {
    ///Bit 0 - DMA1 clock enable
    #[inline(always)]
    pub fn dma1en(&mut self) -> DMA1EN_W<'_, AHB1ENRrs> {
        DMA1EN_W::new(self, 0)
    }
    ///Bit 1 - DMA2 clock enable
    #[inline(always)]
    pub fn dma2en(&mut self) -> DMA2EN_W<'_, AHB1ENRrs> {
        DMA2EN_W::new(self, 1)
    }
    ///Bit 8 - Flash memory interface clock enable
    #[inline(always)]
    pub fn flashen(&mut self) -> FLASHEN_W<'_, AHB1ENRrs> {
        FLASHEN_W::new(self, 8)
    }
    ///Bit 12 - CRC clock enable
    #[inline(always)]
    pub fn crcen(&mut self) -> CRCEN_W<'_, AHB1ENRrs> {
        CRCEN_W::new(self, 12)
    }
    ///Bit 16 - Touch Sensing Controller clock enable
    #[inline(always)]
    pub fn tscen(&mut self) -> TSCEN_W<'_, AHB1ENRrs> {
        TSCEN_W::new(self, 16)
    }
}
/**AHB1 peripheral clock enable register

You can [`read`](crate::Reg::read) this register and get [`ahb1enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x5.html#RCC:AHB1ENR)*/
pub struct AHB1ENRrs;
impl crate::RegisterSpec for AHB1ENRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb1enr::R`](R) reader structure
impl crate::Readable for AHB1ENRrs {}
///`write(|w| ..)` method takes [`ahb1enr::W`](W) writer structure
impl crate::Writable for AHB1ENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB1ENR to value 0x0100
impl crate::Resettable for AHB1ENRrs {
    const RESET_VALUE: u32 = 0x0100;
}
