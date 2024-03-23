#[doc = "Register `C2AHB1ENR` reader"]
pub type R = crate::R<C2AHB1ENRrs>;
#[doc = "Register `C2AHB1ENR` writer"]
pub type W = crate::W<C2AHB1ENRrs>;
#[doc = "Field `DMA1EN` reader - CPU2 DMA1 clock enable"]
pub type DMA1EN_R = crate::BitReader;
#[doc = "Field `DMA1EN` writer - CPU2 DMA1 clock enable"]
pub type DMA1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2EN` reader - CPU2 DMA2 clock enable"]
pub type DMA2EN_R = crate::BitReader;
#[doc = "Field `DMA2EN` writer - CPU2 DMA2 clock enable"]
pub type DMA2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAMUXEN` reader - CPU2 DMAMUX clock enable"]
pub type DMAMUXEN_R = crate::BitReader;
#[doc = "Field `DMAMUXEN` writer - CPU2 DMAMUX clock enable"]
pub type DMAMUXEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM1EN` reader - CPU2 SRAM1 clock enable"]
pub type SRAM1EN_R = crate::BitReader;
#[doc = "Field `SRAM1EN` writer - CPU2 SRAM1 clock enable"]
pub type SRAM1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCEN` reader - CPU2 CRC clock enable"]
pub type CRCEN_R = crate::BitReader;
#[doc = "Field `CRCEN` writer - CPU2 CRC clock enable"]
pub type CRCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSCEN` reader - CPU2 Touch Sensing Controller clock enable"]
pub type TSCEN_R = crate::BitReader;
#[doc = "Field `TSCEN` writer - CPU2 Touch Sensing Controller clock enable"]
pub type TSCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CPU2 DMA1 clock enable"]
    #[inline(always)]
    pub fn dma1en(&self) -> DMA1EN_R {
        DMA1EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CPU2 DMA2 clock enable"]
    #[inline(always)]
    pub fn dma2en(&self) -> DMA2EN_R {
        DMA2EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CPU2 DMAMUX clock enable"]
    #[inline(always)]
    pub fn dmamuxen(&self) -> DMAMUXEN_R {
        DMAMUXEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 9 - CPU2 SRAM1 clock enable"]
    #[inline(always)]
    pub fn sram1en(&self) -> SRAM1EN_R {
        SRAM1EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - CPU2 CRC clock enable"]
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - CPU2 Touch Sensing Controller clock enable"]
    #[inline(always)]
    pub fn tscen(&self) -> TSCEN_R {
        TSCEN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CPU2 DMA1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma1en(&mut self) -> DMA1EN_W<C2AHB1ENRrs> {
        DMA1EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - CPU2 DMA2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma2en(&mut self) -> DMA2EN_W<C2AHB1ENRrs> {
        DMA2EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - CPU2 DMAMUX clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmamuxen(&mut self) -> DMAMUXEN_W<C2AHB1ENRrs> {
        DMAMUXEN_W::new(self, 2)
    }
    #[doc = "Bit 9 - CPU2 SRAM1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn sram1en(&mut self) -> SRAM1EN_W<C2AHB1ENRrs> {
        SRAM1EN_W::new(self, 9)
    }
    #[doc = "Bit 12 - CPU2 CRC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn crcen(&mut self) -> CRCEN_W<C2AHB1ENRrs> {
        CRCEN_W::new(self, 12)
    }
    #[doc = "Bit 16 - CPU2 Touch Sensing Controller clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tscen(&mut self) -> TSCEN_W<C2AHB1ENRrs> {
        TSCEN_W::new(self, 16)
    }
}
#[doc = "CPU2 AHB1 peripheral clock enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2ahb1enr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2ahb1enr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C2AHB1ENRrs;
impl crate::RegisterSpec for C2AHB1ENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c2ahb1enr::R`](R) reader structure"]
impl crate::Readable for C2AHB1ENRrs {}
#[doc = "`write(|w| ..)` method takes [`c2ahb1enr::W`](W) writer structure"]
impl crate::Writable for C2AHB1ENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C2AHB1ENR to value 0"]
impl crate::Resettable for C2AHB1ENRrs {
    const RESET_VALUE: u32 = 0;
}
