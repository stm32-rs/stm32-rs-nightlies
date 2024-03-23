#[doc = "Register `C2AHB1SMENR` reader"]
pub type R = crate::R<C2AHB1SMENRrs>;
#[doc = "Register `C2AHB1SMENR` writer"]
pub type W = crate::W<C2AHB1SMENRrs>;
#[doc = "Field `DMA1SMEN` reader - DMA1 clock enable during CPU2 CSleep mode."]
pub type DMA1SMEN_R = crate::BitReader;
#[doc = "Field `DMA1SMEN` writer - DMA1 clock enable during CPU2 CSleep mode."]
pub type DMA1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2SMEN` reader - DMA2 clock enable during CPU2 CSleep mode."]
pub type DMA2SMEN_R = crate::BitReader;
#[doc = "Field `DMA2SMEN` writer - DMA2 clock enable during CPU2 CSleep mode."]
pub type DMA2SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAMUX1SMEN` reader - DMAMUX1 clock enable during CPU2 CSleep mode."]
pub type DMAMUX1SMEN_R = crate::BitReader;
#[doc = "Field `DMAMUX1SMEN` writer - DMAMUX1 clock enable during CPU2 CSleep mode."]
pub type DMAMUX1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCSMEN` reader - CRC clock enable during CPU2 CSleep mode."]
pub type CRCSMEN_R = crate::BitReader;
#[doc = "Field `CRCSMEN` writer - CRC clock enable during CPU2 CSleep mode."]
pub type CRCSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DMA1 clock enable during CPU2 CSleep mode."]
    #[inline(always)]
    pub fn dma1smen(&self) -> DMA1SMEN_R {
        DMA1SMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA2 clock enable during CPU2 CSleep mode."]
    #[inline(always)]
    pub fn dma2smen(&self) -> DMA2SMEN_R {
        DMA2SMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMAMUX1 clock enable during CPU2 CSleep mode."]
    #[inline(always)]
    pub fn dmamux1smen(&self) -> DMAMUX1SMEN_R {
        DMAMUX1SMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC clock enable during CPU2 CSleep mode."]
    #[inline(always)]
    pub fn crcsmen(&self) -> CRCSMEN_R {
        CRCSMEN_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA1 clock enable during CPU2 CSleep mode."]
    #[inline(always)]
    #[must_use]
    pub fn dma1smen(&mut self) -> DMA1SMEN_W<C2AHB1SMENRrs> {
        DMA1SMEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - DMA2 clock enable during CPU2 CSleep mode."]
    #[inline(always)]
    #[must_use]
    pub fn dma2smen(&mut self) -> DMA2SMEN_W<C2AHB1SMENRrs> {
        DMA2SMEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - DMAMUX1 clock enable during CPU2 CSleep mode."]
    #[inline(always)]
    #[must_use]
    pub fn dmamux1smen(&mut self) -> DMAMUX1SMEN_W<C2AHB1SMENRrs> {
        DMAMUX1SMEN_W::new(self, 2)
    }
    #[doc = "Bit 12 - CRC clock enable during CPU2 CSleep mode."]
    #[inline(always)]
    #[must_use]
    pub fn crcsmen(&mut self) -> CRCSMEN_W<C2AHB1SMENRrs> {
        CRCSMEN_W::new(self, 12)
    }
}
#[doc = "CPU2 AHB1 peripheral clocks enable in Sleep modes register \\[dual core device only\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2ahb1smenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2ahb1smenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets C2AHB1SMENR to value 0x1007"]
impl crate::Resettable for C2AHB1SMENRrs {
    const RESET_VALUE: u32 = 0x1007;
}
