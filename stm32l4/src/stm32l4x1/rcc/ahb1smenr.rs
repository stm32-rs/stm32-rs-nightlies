///Register `AHB1SMENR` reader
pub type R = crate::R<AHB1SMENRrs>;
///Register `AHB1SMENR` writer
pub type W = crate::W<AHB1SMENRrs>;
///Field `DMA1SMEN` reader - DMA1 clocks enable during Sleep and Stop modes
pub type DMA1SMEN_R = crate::BitReader;
///Field `DMA1SMEN` writer - DMA1 clocks enable during Sleep and Stop modes
pub type DMA1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMA2SMEN` reader - DMA2 clocks enable during Sleep and Stop modes
pub type DMA2SMEN_R = crate::BitReader;
///Field `DMA2SMEN` writer - DMA2 clocks enable during Sleep and Stop modes
pub type DMA2SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLASHSMEN` reader - Flash memory interface clocks enable during Sleep and Stop modes
pub type FLASHSMEN_R = crate::BitReader;
///Field `FLASHSMEN` writer - Flash memory interface clocks enable during Sleep and Stop modes
pub type FLASHSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRAM1SMEN` reader - SRAM1 interface clocks enable during Sleep and Stop modes
pub type SRAM1SMEN_R = crate::BitReader;
///Field `SRAM1SMEN` writer - SRAM1 interface clocks enable during Sleep and Stop modes
pub type SRAM1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRCSMEN` reader - CRCSMEN
pub type CRCSMEN_R = crate::BitReader;
///Field `CRCSMEN` writer - CRCSMEN
pub type CRCSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSCSMEN` reader - Touch Sensing Controller clocks enable during Sleep and Stop modes
pub type TSCSMEN_R = crate::BitReader;
///Field `TSCSMEN` writer - Touch Sensing Controller clocks enable during Sleep and Stop modes
pub type TSCSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - DMA1 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn dma1smen(&self) -> DMA1SMEN_R {
        DMA1SMEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DMA2 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn dma2smen(&self) -> DMA2SMEN_R {
        DMA2SMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 8 - Flash memory interface clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn flashsmen(&self) -> FLASHSMEN_R {
        FLASHSMEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - SRAM1 interface clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn sram1smen(&self) -> SRAM1SMEN_R {
        SRAM1SMEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 12 - CRCSMEN
    #[inline(always)]
    pub fn crcsmen(&self) -> CRCSMEN_R {
        CRCSMEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 16 - Touch Sensing Controller clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn tscsmen(&self) -> TSCSMEN_R {
        TSCSMEN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB1SMENR")
            .field("tscsmen", &self.tscsmen())
            .field("crcsmen", &self.crcsmen())
            .field("sram1smen", &self.sram1smen())
            .field("flashsmen", &self.flashsmen())
            .field("dma2smen", &self.dma2smen())
            .field("dma1smen", &self.dma1smen())
            .finish()
    }
}
impl W {
    ///Bit 0 - DMA1 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn dma1smen(&mut self) -> DMA1SMEN_W<'_, AHB1SMENRrs> {
        DMA1SMEN_W::new(self, 0)
    }
    ///Bit 1 - DMA2 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn dma2smen(&mut self) -> DMA2SMEN_W<'_, AHB1SMENRrs> {
        DMA2SMEN_W::new(self, 1)
    }
    ///Bit 8 - Flash memory interface clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn flashsmen(&mut self) -> FLASHSMEN_W<'_, AHB1SMENRrs> {
        FLASHSMEN_W::new(self, 8)
    }
    ///Bit 9 - SRAM1 interface clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn sram1smen(&mut self) -> SRAM1SMEN_W<'_, AHB1SMENRrs> {
        SRAM1SMEN_W::new(self, 9)
    }
    ///Bit 12 - CRCSMEN
    #[inline(always)]
    pub fn crcsmen(&mut self) -> CRCSMEN_W<'_, AHB1SMENRrs> {
        CRCSMEN_W::new(self, 12)
    }
    ///Bit 16 - Touch Sensing Controller clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn tscsmen(&mut self) -> TSCSMEN_W<'_, AHB1SMENRrs> {
        TSCSMEN_W::new(self, 16)
    }
}
/**AHB1 peripheral clocks enable in Sleep and Stop modes register

You can [`read`](crate::Reg::read) this register and get [`ahb1smenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1smenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x1.html#RCC:AHB1SMENR)*/
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
///`reset()` method sets AHB1SMENR to value 0x0001_1303
impl crate::Resettable for AHB1SMENRrs {
    const RESET_VALUE: u32 = 0x0001_1303;
}
