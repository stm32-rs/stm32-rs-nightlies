#[doc = "Register `AHB1SMENR` reader"]
pub type R = crate::R<AHB1SMENRrs>;
#[doc = "Register `AHB1SMENR` writer"]
pub type W = crate::W<AHB1SMENRrs>;
#[doc = "Field `DMA1SMEN` reader - CPU1 DMA1 clocks enable during Sleep and Stop modes"]
pub type DMA1SMEN_R = crate::BitReader;
#[doc = "Field `DMA1SMEN` writer - CPU1 DMA1 clocks enable during Sleep and Stop modes"]
pub type DMA1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2SMEN` reader - CPU1 DMA2 clocks enable during Sleep and Stop modes"]
pub type DMA2SMEN_R = crate::BitReader;
#[doc = "Field `DMA2SMEN` writer - CPU1 DMA2 clocks enable during Sleep and Stop modes"]
pub type DMA2SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAMUXSMEN` reader - CPU1 DMAMUX clocks enable during Sleep and Stop modes"]
pub type DMAMUXSMEN_R = crate::BitReader;
#[doc = "Field `DMAMUXSMEN` writer - CPU1 DMAMUX clocks enable during Sleep and Stop modes"]
pub type DMAMUXSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM1SMEN` reader - CPU1 SRAM1 interface clocks enable during Sleep and Stop modes"]
pub type SRAM1SMEN_R = crate::BitReader;
#[doc = "Field `SRAM1SMEN` writer - CPU1 SRAM1 interface clocks enable during Sleep and Stop modes"]
pub type SRAM1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCSMEN` reader - CPU1 CRCSMEN"]
pub type CRCSMEN_R = crate::BitReader;
#[doc = "Field `CRCSMEN` writer - CPU1 CRCSMEN"]
pub type CRCSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSCSMEN` reader - CPU1 Touch Sensing Controller clocks enable during Sleep and Stop modes"]
pub type TSCSMEN_R = crate::BitReader;
#[doc = "Field `TSCSMEN` writer - CPU1 Touch Sensing Controller clocks enable during Sleep and Stop modes"]
pub type TSCSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CPU1 DMA1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn dma1smen(&self) -> DMA1SMEN_R {
        DMA1SMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CPU1 DMA2 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn dma2smen(&self) -> DMA2SMEN_R {
        DMA2SMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CPU1 DMAMUX clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn dmamuxsmen(&self) -> DMAMUXSMEN_R {
        DMAMUXSMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 9 - CPU1 SRAM1 interface clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn sram1smen(&self) -> SRAM1SMEN_R {
        SRAM1SMEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - CPU1 CRCSMEN"]
    #[inline(always)]
    pub fn crcsmen(&self) -> CRCSMEN_R {
        CRCSMEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - CPU1 Touch Sensing Controller clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn tscsmen(&self) -> TSCSMEN_R {
        TSCSMEN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CPU1 DMA1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn dma1smen(&mut self) -> DMA1SMEN_W<AHB1SMENRrs> {
        DMA1SMEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - CPU1 DMA2 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn dma2smen(&mut self) -> DMA2SMEN_W<AHB1SMENRrs> {
        DMA2SMEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - CPU1 DMAMUX clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn dmamuxsmen(&mut self) -> DMAMUXSMEN_W<AHB1SMENRrs> {
        DMAMUXSMEN_W::new(self, 2)
    }
    #[doc = "Bit 9 - CPU1 SRAM1 interface clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn sram1smen(&mut self) -> SRAM1SMEN_W<AHB1SMENRrs> {
        SRAM1SMEN_W::new(self, 9)
    }
    #[doc = "Bit 12 - CPU1 CRCSMEN"]
    #[inline(always)]
    #[must_use]
    pub fn crcsmen(&mut self) -> CRCSMEN_W<AHB1SMENRrs> {
        CRCSMEN_W::new(self, 12)
    }
    #[doc = "Bit 16 - CPU1 Touch Sensing Controller clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn tscsmen(&mut self) -> TSCSMEN_W<AHB1SMENRrs> {
        TSCSMEN_W::new(self, 16)
    }
}
#[doc = "AHB1 peripheral clocks enable in Sleep and Stop modes register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb1smenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb1smenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB1SMENRrs;
impl crate::RegisterSpec for AHB1SMENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb1smenr::R`](R) reader structure"]
impl crate::Readable for AHB1SMENRrs {}
#[doc = "`write(|w| ..)` method takes [`ahb1smenr::W`](W) writer structure"]
impl crate::Writable for AHB1SMENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHB1SMENR to value 0x0001_1207"]
impl crate::Resettable for AHB1SMENRrs {
    const RESET_VALUE: u32 = 0x0001_1207;
}
