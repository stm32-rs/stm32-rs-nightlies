///Register `RCC_AHBENR` reader
pub type R = crate::R<RCC_AHBENRrs>;
///Register `RCC_AHBENR` writer
pub type W = crate::W<RCC_AHBENRrs>;
///Field `DMA1EN` reader - DMA1 and DMAMUX clock enable Set and cleared by software. DMAMUX is enabled as long as at least one DMA peripheral is enabled.
pub type DMA1EN_R = crate::BitReader;
///Field `DMA1EN` writer - DMA1 and DMAMUX clock enable Set and cleared by software. DMAMUX is enabled as long as at least one DMA peripheral is enabled.
pub type DMA1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMA2EN` reader - DMA2 and DMAMUX clock enable Set and cleared by software. DMAMUX is enabled as long as at least one DMA peripheral is enabled.
pub type DMA2EN_R = crate::BitReader;
///Field `DMA2EN` writer - DMA2 and DMAMUX clock enable Set and cleared by software. DMAMUX is enabled as long as at least one DMA peripheral is enabled.
pub type DMA2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLASHEN` reader - Flash memory interface clock enable Set and cleared by software. This bit can only be cleared when the flash memory is in power down mode.
pub type FLASHEN_R = crate::BitReader;
///Field `FLASHEN` writer - Flash memory interface clock enable Set and cleared by software. This bit can only be cleared when the flash memory is in power down mode.
pub type FLASHEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRCEN` reader - CRC clock enable Set and cleared by software.
pub type CRCEN_R = crate::BitReader;
///Field `CRCEN` writer - CRC clock enable Set and cleared by software.
pub type CRCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RNGEN` reader - Random number generator clock enable Set and cleared by software.
pub type RNGEN_R = crate::BitReader;
///Field `RNGEN` writer - Random number generator clock enable Set and cleared by software.
pub type RNGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSCEN` reader - Touch sensing controller clock enable Set and cleared by software.
pub type TSCEN_R = crate::BitReader;
///Field `TSCEN` writer - Touch sensing controller clock enable Set and cleared by software.
pub type TSCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - DMA1 and DMAMUX clock enable Set and cleared by software. DMAMUX is enabled as long as at least one DMA peripheral is enabled.
    #[inline(always)]
    pub fn dma1en(&self) -> DMA1EN_R {
        DMA1EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DMA2 and DMAMUX clock enable Set and cleared by software. DMAMUX is enabled as long as at least one DMA peripheral is enabled.
    #[inline(always)]
    pub fn dma2en(&self) -> DMA2EN_R {
        DMA2EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 8 - Flash memory interface clock enable Set and cleared by software. This bit can only be cleared when the flash memory is in power down mode.
    #[inline(always)]
    pub fn flashen(&self) -> FLASHEN_R {
        FLASHEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 12 - CRC clock enable Set and cleared by software.
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 18 - Random number generator clock enable Set and cleared by software.
    #[inline(always)]
    pub fn rngen(&self) -> RNGEN_R {
        RNGEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 24 - Touch sensing controller clock enable Set and cleared by software.
    #[inline(always)]
    pub fn tscen(&self) -> TSCEN_R {
        TSCEN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_AHBENR")
            .field("dma1en", &self.dma1en())
            .field("dma2en", &self.dma2en())
            .field("flashen", &self.flashen())
            .field("crcen", &self.crcen())
            .field("rngen", &self.rngen())
            .field("tscen", &self.tscen())
            .finish()
    }
}
impl W {
    ///Bit 0 - DMA1 and DMAMUX clock enable Set and cleared by software. DMAMUX is enabled as long as at least one DMA peripheral is enabled.
    #[inline(always)]
    pub fn dma1en(&mut self) -> DMA1EN_W<RCC_AHBENRrs> {
        DMA1EN_W::new(self, 0)
    }
    ///Bit 1 - DMA2 and DMAMUX clock enable Set and cleared by software. DMAMUX is enabled as long as at least one DMA peripheral is enabled.
    #[inline(always)]
    pub fn dma2en(&mut self) -> DMA2EN_W<RCC_AHBENRrs> {
        DMA2EN_W::new(self, 1)
    }
    ///Bit 8 - Flash memory interface clock enable Set and cleared by software. This bit can only be cleared when the flash memory is in power down mode.
    #[inline(always)]
    pub fn flashen(&mut self) -> FLASHEN_W<RCC_AHBENRrs> {
        FLASHEN_W::new(self, 8)
    }
    ///Bit 12 - CRC clock enable Set and cleared by software.
    #[inline(always)]
    pub fn crcen(&mut self) -> CRCEN_W<RCC_AHBENRrs> {
        CRCEN_W::new(self, 12)
    }
    ///Bit 18 - Random number generator clock enable Set and cleared by software.
    #[inline(always)]
    pub fn rngen(&mut self) -> RNGEN_W<RCC_AHBENRrs> {
        RNGEN_W::new(self, 18)
    }
    ///Bit 24 - Touch sensing controller clock enable Set and cleared by software.
    #[inline(always)]
    pub fn tscen(&mut self) -> TSCEN_W<RCC_AHBENRrs> {
        TSCEN_W::new(self, 24)
    }
}
/**AHB peripheral clock enable register

You can [`read`](crate::Reg::read) this register and get [`rcc_ahbenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_ahbenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#RCC:RCC_AHBENR)*/
pub struct RCC_AHBENRrs;
impl crate::RegisterSpec for RCC_AHBENRrs {
    type Ux = u32;
}
///`read()` method returns [`rcc_ahbenr::R`](R) reader structure
impl crate::Readable for RCC_AHBENRrs {}
///`write(|w| ..)` method takes [`rcc_ahbenr::W`](W) writer structure
impl crate::Writable for RCC_AHBENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RCC_AHBENR to value 0x0100
impl crate::Resettable for RCC_AHBENRrs {
    const RESET_VALUE: u32 = 0x0100;
}