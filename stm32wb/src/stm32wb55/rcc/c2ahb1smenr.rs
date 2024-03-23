#[doc = "Register `C2AHB1SMENR` reader"]
pub type R = crate::R<C2AHB1SMENRrs>;
#[doc = "Register `C2AHB1SMENR` writer"]
pub type W = crate::W<C2AHB1SMENRrs>;
#[doc = "Field `DMA1SMEN` reader - CPU2 DMA1 clocks enable during Sleep and Stop modes"]
pub type DMA1SMEN_R = crate::BitReader;
#[doc = "Field `DMA1SMEN` writer - CPU2 DMA1 clocks enable during Sleep and Stop modes"]
pub type DMA1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2SMEN` reader - CPU2 DMA2 clocks enable during Sleep and Stop modes"]
pub type DMA2SMEN_R = crate::BitReader;
#[doc = "Field `DMA2SMEN` writer - CPU2 DMA2 clocks enable during Sleep and Stop modes"]
pub type DMA2SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAMUXSMEN` reader - CPU2 DMAMUX clocks enable during Sleep and Stop modes"]
pub type DMAMUXSMEN_R = crate::BitReader;
#[doc = "Field `DMAMUXSMEN` writer - CPU2 DMAMUX clocks enable during Sleep and Stop modes"]
pub type DMAMUXSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM1SMEN` reader - SRAM1 interface clock enable during CPU1 CSleep mode"]
pub type SRAM1SMEN_R = crate::BitReader;
#[doc = "Field `SRAM1SMEN` writer - SRAM1 interface clock enable during CPU1 CSleep mode"]
pub type SRAM1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCSMEN` reader - CPU2 CRCSMEN"]
pub type CRCSMEN_R = crate::BitReader;
#[doc = "Field `CRCSMEN` writer - CPU2 CRCSMEN"]
pub type CRCSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSCSMEN` reader - CPU2 Touch Sensing Controller clocks enable during Sleep and Stop modes"]
pub type TSCSMEN_R = crate::BitReader;
#[doc = "Field `TSCSMEN` writer - CPU2 Touch Sensing Controller clocks enable during Sleep and Stop modes"]
pub type TSCSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CPU2 DMA1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn dma1smen(&self) -> DMA1SMEN_R {
        DMA1SMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CPU2 DMA2 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn dma2smen(&self) -> DMA2SMEN_R {
        DMA2SMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CPU2 DMAMUX clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn dmamuxsmen(&self) -> DMAMUXSMEN_R {
        DMAMUXSMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 9 - SRAM1 interface clock enable during CPU1 CSleep mode"]
    #[inline(always)]
    pub fn sram1smen(&self) -> SRAM1SMEN_R {
        SRAM1SMEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - CPU2 CRCSMEN"]
    #[inline(always)]
    pub fn crcsmen(&self) -> CRCSMEN_R {
        CRCSMEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - CPU2 Touch Sensing Controller clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn tscsmen(&self) -> TSCSMEN_R {
        TSCSMEN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CPU2 DMA1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn dma1smen(&mut self) -> DMA1SMEN_W<C2AHB1SMENRrs> {
        DMA1SMEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - CPU2 DMA2 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn dma2smen(&mut self) -> DMA2SMEN_W<C2AHB1SMENRrs> {
        DMA2SMEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - CPU2 DMAMUX clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn dmamuxsmen(&mut self) -> DMAMUXSMEN_W<C2AHB1SMENRrs> {
        DMAMUXSMEN_W::new(self, 2)
    }
    #[doc = "Bit 9 - SRAM1 interface clock enable during CPU1 CSleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn sram1smen(&mut self) -> SRAM1SMEN_W<C2AHB1SMENRrs> {
        SRAM1SMEN_W::new(self, 9)
    }
    #[doc = "Bit 12 - CPU2 CRCSMEN"]
    #[inline(always)]
    #[must_use]
    pub fn crcsmen(&mut self) -> CRCSMEN_W<C2AHB1SMENRrs> {
        CRCSMEN_W::new(self, 12)
    }
    #[doc = "Bit 16 - CPU2 Touch Sensing Controller clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn tscsmen(&mut self) -> TSCSMEN_W<C2AHB1SMENRrs> {
        TSCSMEN_W::new(self, 16)
    }
}
#[doc = "CPU2 AHB1 peripheral clocks enable in Sleep and Stop modes register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2ahb1smenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2ahb1smenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C2AHB1SMENRrs;
impl crate::RegisterSpec for C2AHB1SMENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c2ahb1smenr::R`](R) reader structure"]
impl crate::Readable for C2AHB1SMENRrs {}
#[doc = "`write(|w| ..)` method takes [`c2ahb1smenr::W`](W) writer structure"]
impl crate::Writable for C2AHB1SMENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C2AHB1SMENR to value 0x0001_1207"]
impl crate::Resettable for C2AHB1SMENRrs {
    const RESET_VALUE: u32 = 0x0001_1207;
}
