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
///Field `DMAMUX1SMEN` reader - DMAMUX1 clock enable during Sleep and Stop modes.
pub type DMAMUX1SMEN_R = crate::BitReader;
///Field `DMAMUX1SMEN` writer - DMAMUX1 clock enable during Sleep and Stop modes.
pub type DMAMUX1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CORDICSMEN` reader - CORDICSM clock enable.
pub type CORDICSMEN_R = crate::BitReader;
///Field `CORDICSMEN` writer - CORDICSM clock enable.
pub type CORDICSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMACSMEN` reader - FMACSM clock enable.
pub type FMACSMEN_R = crate::BitReader;
///Field `FMACSMEN` writer - FMACSM clock enable.
pub type FMACSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLASHSMEN` reader - Flash memory interface clocks enable during Sleep and Stop modes
pub type FLASHSMEN_R = crate::BitReader;
///Field `FLASHSMEN` writer - Flash memory interface clocks enable during Sleep and Stop modes
pub type FLASHSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRAM1SMEN` reader - SRAM1 interface clocks enable during Sleep and Stop modes
pub type SRAM1SMEN_R = crate::BitReader;
///Field `SRAM1SMEN` writer - SRAM1 interface clocks enable during Sleep and Stop modes
pub type SRAM1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRCSMEN` reader - CRC clocks enable during Sleep and Stop modes
pub type CRCSMEN_R = crate::BitReader;
///Field `CRCSMEN` writer - CRC clocks enable during Sleep and Stop modes
pub type CRCSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    ///Bit 2 - DMAMUX1 clock enable during Sleep and Stop modes.
    #[inline(always)]
    pub fn dmamux1smen(&self) -> DMAMUX1SMEN_R {
        DMAMUX1SMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - CORDICSM clock enable.
    #[inline(always)]
    pub fn cordicsmen(&self) -> CORDICSMEN_R {
        CORDICSMEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - FMACSM clock enable.
    #[inline(always)]
    pub fn fmacsmen(&self) -> FMACSMEN_R {
        FMACSMEN_R::new(((self.bits >> 4) & 1) != 0)
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
    ///Bit 12 - CRC clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn crcsmen(&self) -> CRCSMEN_R {
        CRCSMEN_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB1SMENR")
            .field("dma1smen", &self.dma1smen())
            .field("dma2smen", &self.dma2smen())
            .field("dmamux1smen", &self.dmamux1smen())
            .field("cordicsmen", &self.cordicsmen())
            .field("fmacsmen", &self.fmacsmen())
            .field("flashsmen", &self.flashsmen())
            .field("sram1smen", &self.sram1smen())
            .field("crcsmen", &self.crcsmen())
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
    ///Bit 2 - DMAMUX1 clock enable during Sleep and Stop modes.
    #[inline(always)]
    pub fn dmamux1smen(&mut self) -> DMAMUX1SMEN_W<'_, AHB1SMENRrs> {
        DMAMUX1SMEN_W::new(self, 2)
    }
    ///Bit 3 - CORDICSM clock enable.
    #[inline(always)]
    pub fn cordicsmen(&mut self) -> CORDICSMEN_W<'_, AHB1SMENRrs> {
        CORDICSMEN_W::new(self, 3)
    }
    ///Bit 4 - FMACSM clock enable.
    #[inline(always)]
    pub fn fmacsmen(&mut self) -> FMACSMEN_W<'_, AHB1SMENRrs> {
        FMACSMEN_W::new(self, 4)
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
    ///Bit 12 - CRC clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn crcsmen(&mut self) -> CRCSMEN_W<'_, AHB1SMENRrs> {
        CRCSMEN_W::new(self, 12)
    }
}
/**AHB1 peripheral clocks enable in Sleep and Stop modes register

You can [`read`](crate::Reg::read) this register and get [`ahb1smenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1smenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G441.html#RCC:AHB1SMENR)*/
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
///`reset()` method sets AHB1SMENR to value 0x130f
impl crate::Resettable for AHB1SMENRrs {
    const RESET_VALUE: u32 = 0x130f;
}
