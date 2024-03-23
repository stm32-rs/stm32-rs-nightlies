#[doc = "Register `AHB1SMENR` reader"]
pub type R = crate::R<AHB1SMENRrs>;
#[doc = "Register `AHB1SMENR` writer"]
pub type W = crate::W<AHB1SMENRrs>;
#[doc = "Field `DMA1SMEN` reader - DMA1 clocks enable during Sleep and Stop modes"]
pub type DMA1SMEN_R = crate::BitReader;
#[doc = "Field `DMA1SMEN` writer - DMA1 clocks enable during Sleep and Stop modes"]
pub type DMA1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2SMEN` reader - DMA2 clocks enable during Sleep and Stop modes"]
pub type DMA2SMEN_R = crate::BitReader;
#[doc = "Field `DMA2SMEN` writer - DMA2 clocks enable during Sleep and Stop modes"]
pub type DMA2SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAMUX1SMEN` reader - DMAMUX clock enable during Sleep and Stop modes"]
pub type DMAMUX1SMEN_R = crate::BitReader;
#[doc = "Field `DMAMUX1SMEN` writer - DMAMUX clock enable during Sleep and Stop modes"]
pub type DMAMUX1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASHSMEN` reader - Flash memory interface clocks enable during Sleep and Stop modes"]
pub type FLASHSMEN_R = crate::BitReader;
#[doc = "Field `FLASHSMEN` writer - Flash memory interface clocks enable during Sleep and Stop modes"]
pub type FLASHSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM1SMEN` reader - SRAM1 interface clocks enable during Sleep and Stop modes"]
pub type SRAM1SMEN_R = crate::BitReader;
#[doc = "Field `SRAM1SMEN` writer - SRAM1 interface clocks enable during Sleep and Stop modes"]
pub type SRAM1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCSMEN` reader - CRCSMEN"]
pub type CRCSMEN_R = crate::BitReader;
#[doc = "Field `CRCSMEN` writer - CRCSMEN"]
pub type CRCSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSCSMEN` reader - Touch Sensing Controller clocks enable during Sleep and Stop modes"]
pub type TSCSMEN_R = crate::BitReader;
#[doc = "Field `TSCSMEN` writer - Touch Sensing Controller clocks enable during Sleep and Stop modes"]
pub type TSCSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2DSMEN` reader - DMA2D clock enable during Sleep and Stop modes"]
pub type DMA2DSMEN_R = crate::BitReader;
#[doc = "Field `DMA2DSMEN` writer - DMA2D clock enable during Sleep and Stop modes"]
pub type DMA2DSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GFXMMUSMEN` reader - GFXMMU clock enable during Sleep and Stop modes"]
pub type GFXMMUSMEN_R = crate::BitReader;
#[doc = "Field `GFXMMUSMEN` writer - GFXMMU clock enable during Sleep and Stop modes"]
pub type GFXMMUSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DMA1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn dma1smen(&self) -> DMA1SMEN_R {
        DMA1SMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA2 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn dma2smen(&self) -> DMA2SMEN_R {
        DMA2SMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMAMUX clock enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn dmamux1smen(&self) -> DMAMUX1SMEN_R {
        DMAMUX1SMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Flash memory interface clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn flashsmen(&self) -> FLASHSMEN_R {
        FLASHSMEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SRAM1 interface clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn sram1smen(&self) -> SRAM1SMEN_R {
        SRAM1SMEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - CRCSMEN"]
    #[inline(always)]
    pub fn crcsmen(&self) -> CRCSMEN_R {
        CRCSMEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Touch Sensing Controller clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn tscsmen(&self) -> TSCSMEN_R {
        TSCSMEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DMA2D clock enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn dma2dsmen(&self) -> DMA2DSMEN_R {
        DMA2DSMEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - GFXMMU clock enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gfxmmusmen(&self) -> GFXMMUSMEN_R {
        GFXMMUSMEN_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn dma1smen(&mut self) -> DMA1SMEN_W<AHB1SMENRrs> {
        DMA1SMEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - DMA2 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn dma2smen(&mut self) -> DMA2SMEN_W<AHB1SMENRrs> {
        DMA2SMEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - DMAMUX clock enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn dmamux1smen(&mut self) -> DMAMUX1SMEN_W<AHB1SMENRrs> {
        DMAMUX1SMEN_W::new(self, 2)
    }
    #[doc = "Bit 8 - Flash memory interface clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn flashsmen(&mut self) -> FLASHSMEN_W<AHB1SMENRrs> {
        FLASHSMEN_W::new(self, 8)
    }
    #[doc = "Bit 9 - SRAM1 interface clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn sram1smen(&mut self) -> SRAM1SMEN_W<AHB1SMENRrs> {
        SRAM1SMEN_W::new(self, 9)
    }
    #[doc = "Bit 12 - CRCSMEN"]
    #[inline(always)]
    #[must_use]
    pub fn crcsmen(&mut self) -> CRCSMEN_W<AHB1SMENRrs> {
        CRCSMEN_W::new(self, 12)
    }
    #[doc = "Bit 16 - Touch Sensing Controller clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn tscsmen(&mut self) -> TSCSMEN_W<AHB1SMENRrs> {
        TSCSMEN_W::new(self, 16)
    }
    #[doc = "Bit 17 - DMA2D clock enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn dma2dsmen(&mut self) -> DMA2DSMEN_W<AHB1SMENRrs> {
        DMA2DSMEN_W::new(self, 17)
    }
    #[doc = "Bit 18 - GFXMMU clock enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn gfxmmusmen(&mut self) -> GFXMMUSMEN_W<AHB1SMENRrs> {
        GFXMMUSMEN_W::new(self, 18)
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
#[doc = "`reset()` method sets AHB1SMENR to value 0x0001_1303"]
impl crate::Resettable for AHB1SMENRrs {
    const RESET_VALUE: u32 = 0x0001_1303;
}
