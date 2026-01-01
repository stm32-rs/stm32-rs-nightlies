///Register `AHB1SMENR` reader
pub type R = crate::R<AHB1SMENRrs>;
///Register `AHB1SMENR` writer
pub type W = crate::W<AHB1SMENRrs>;
///Field `DMA1SMEN` reader - DMA1 clock enable during CPU1 CSleep mode.
pub type DMA1SMEN_R = crate::BitReader;
///Field `DMA1SMEN` writer - DMA1 clock enable during CPU1 CSleep mode.
pub type DMA1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMA2SMEN` reader - DMA2 clock enable during CPU1 CSleep mode
pub type DMA2SMEN_R = crate::BitReader;
///Field `DMA2SMEN` writer - DMA2 clock enable during CPU1 CSleep mode
pub type DMA2SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMAMUX1SMEN` reader - DMAMUX1 clock enable during CPU1 CSleep mode.
pub type DMAMUX1SMEN_R = crate::BitReader;
///Field `DMAMUX1SMEN` writer - DMAMUX1 clock enable during CPU1 CSleep mode.
pub type DMAMUX1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRCSMEN` reader - CRC clock enable during CPU1 CSleep mode.
pub type CRCSMEN_R = crate::BitReader;
///Field `CRCSMEN` writer - CRC clock enable during CPU1 CSleep mode.
pub type CRCSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - DMA1 clock enable during CPU1 CSleep mode.
    #[inline(always)]
    pub fn dma1smen(&self) -> DMA1SMEN_R {
        DMA1SMEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DMA2 clock enable during CPU1 CSleep mode
    #[inline(always)]
    pub fn dma2smen(&self) -> DMA2SMEN_R {
        DMA2SMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - DMAMUX1 clock enable during CPU1 CSleep mode.
    #[inline(always)]
    pub fn dmamux1smen(&self) -> DMAMUX1SMEN_R {
        DMAMUX1SMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 12 - CRC clock enable during CPU1 CSleep mode.
    #[inline(always)]
    pub fn crcsmen(&self) -> CRCSMEN_R {
        CRCSMEN_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB1SMENR")
            .field("crcsmen", &self.crcsmen())
            .field("dmamux1smen", &self.dmamux1smen())
            .field("dma2smen", &self.dma2smen())
            .field("dma1smen", &self.dma1smen())
            .finish()
    }
}
impl W {
    ///Bit 0 - DMA1 clock enable during CPU1 CSleep mode.
    #[inline(always)]
    pub fn dma1smen(&mut self) -> DMA1SMEN_W<'_, AHB1SMENRrs> {
        DMA1SMEN_W::new(self, 0)
    }
    ///Bit 1 - DMA2 clock enable during CPU1 CSleep mode
    #[inline(always)]
    pub fn dma2smen(&mut self) -> DMA2SMEN_W<'_, AHB1SMENRrs> {
        DMA2SMEN_W::new(self, 1)
    }
    ///Bit 2 - DMAMUX1 clock enable during CPU1 CSleep mode.
    #[inline(always)]
    pub fn dmamux1smen(&mut self) -> DMAMUX1SMEN_W<'_, AHB1SMENRrs> {
        DMAMUX1SMEN_W::new(self, 2)
    }
    ///Bit 12 - CRC clock enable during CPU1 CSleep mode.
    #[inline(always)]
    pub fn crcsmen(&mut self) -> CRCSMEN_W<'_, AHB1SMENRrs> {
        CRCSMEN_W::new(self, 12)
    }
}
/**AHB1 peripheral clocks enable in Sleep modes register

You can [`read`](crate::Reg::read) this register and get [`ahb1smenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1smenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#RCC:AHB1SMENR)*/
pub struct AHB1SMENRrs;
impl crate::RegisterSpec for AHB1SMENRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb1smenr::R`](R) reader structure
impl crate::Readable for AHB1SMENRrs {}
///`write(|w| ..)` method takes [`ahb1smenr::W`](W) writer structure
impl crate::Writable for AHB1SMENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB1SMENR to value 0x1007
impl crate::Resettable for AHB1SMENRrs {
    const RESET_VALUE: u32 = 0x1007;
}
