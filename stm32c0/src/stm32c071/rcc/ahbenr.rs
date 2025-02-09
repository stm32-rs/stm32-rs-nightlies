///Register `AHBENR` reader
pub type R = crate::R<AHBENRrs>;
///Register `AHBENR` writer
pub type W = crate::W<AHBENRrs>;
///Field `DMA1EN` reader - DMA1 and DMAMUX clock enable Set and cleared by software. DMAMUX is enabled as long as at least one DMA peripheral is enabled.
pub type DMA1EN_R = crate::BitReader;
///Field `DMA1EN` writer - DMA1 and DMAMUX clock enable Set and cleared by software. DMAMUX is enabled as long as at least one DMA peripheral is enabled.
pub type DMA1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLASHEN` reader - Flash memory interface clock enable Set and cleared by software. This bit can only be cleared when the Flash memory is in power down mode.
pub type FLASHEN_R = crate::BitReader;
///Field `FLASHEN` writer - Flash memory interface clock enable Set and cleared by software. This bit can only be cleared when the Flash memory is in power down mode.
pub type FLASHEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRCEN` reader - CRC clock enable Set and cleared by software.
pub type CRCEN_R = crate::BitReader;
///Field `CRCEN` writer - CRC clock enable Set and cleared by software.
pub type CRCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - DMA1 and DMAMUX clock enable Set and cleared by software. DMAMUX is enabled as long as at least one DMA peripheral is enabled.
    #[inline(always)]
    pub fn dma1en(&self) -> DMA1EN_R {
        DMA1EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 8 - Flash memory interface clock enable Set and cleared by software. This bit can only be cleared when the Flash memory is in power down mode.
    #[inline(always)]
    pub fn flashen(&self) -> FLASHEN_R {
        FLASHEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 12 - CRC clock enable Set and cleared by software.
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBENR")
            .field("dma1en", &self.dma1en())
            .field("flashen", &self.flashen())
            .field("crcen", &self.crcen())
            .finish()
    }
}
impl W {
    ///Bit 0 - DMA1 and DMAMUX clock enable Set and cleared by software. DMAMUX is enabled as long as at least one DMA peripheral is enabled.
    #[inline(always)]
    pub fn dma1en(&mut self) -> DMA1EN_W<AHBENRrs> {
        DMA1EN_W::new(self, 0)
    }
    ///Bit 8 - Flash memory interface clock enable Set and cleared by software. This bit can only be cleared when the Flash memory is in power down mode.
    #[inline(always)]
    pub fn flashen(&mut self) -> FLASHEN_W<AHBENRrs> {
        FLASHEN_W::new(self, 8)
    }
    ///Bit 12 - CRC clock enable Set and cleared by software.
    #[inline(always)]
    pub fn crcen(&mut self) -> CRCEN_W<AHBENRrs> {
        CRCEN_W::new(self, 12)
    }
}
/**RCC AHB peripheral clock enable register

You can [`read`](crate::Reg::read) this register and get [`ahbenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C071.html#RCC:AHBENR)*/
pub struct AHBENRrs;
impl crate::RegisterSpec for AHBENRrs {
    type Ux = u32;
}
///`read()` method returns [`ahbenr::R`](R) reader structure
impl crate::Readable for AHBENRrs {}
///`write(|w| ..)` method takes [`ahbenr::W`](W) writer structure
impl crate::Writable for AHBENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets AHBENR to value 0x0100
impl crate::Resettable for AHBENRrs {
    const RESET_VALUE: u32 = 0x0100;
}
