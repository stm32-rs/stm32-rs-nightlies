#[doc = "Register `AHBENR` reader"]
pub type R = crate::R<AHBENRrs>;
#[doc = "Register `AHBENR` writer"]
pub type W = crate::W<AHBENRrs>;
#[doc = "Field `DMA1EN` reader - DMA1 clock enable"]
pub type DMA1EN_R = crate::BitReader;
#[doc = "Field `DMA1EN` writer - DMA1 clock enable"]
pub type DMA1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2EN` reader - DMA2 clock enable"]
pub type DMA2EN_R = crate::BitReader;
#[doc = "Field `DMA2EN` writer - DMA2 clock enable"]
pub type DMA2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASHEN` reader - Flash memory interface clock enable"]
pub type FLASHEN_R = crate::BitReader;
#[doc = "Field `FLASHEN` writer - Flash memory interface clock enable"]
pub type FLASHEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCEN` reader - CRC clock enable"]
pub type CRCEN_R = crate::BitReader;
#[doc = "Field `CRCEN` writer - CRC clock enable"]
pub type CRCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AESEN` reader - AES hardware accelerator"]
pub type AESEN_R = crate::BitReader;
#[doc = "Field `AESEN` writer - AES hardware accelerator"]
pub type AESEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RNGEN` reader - Random number generator clock enable"]
pub type RNGEN_R = crate::BitReader;
#[doc = "Field `RNGEN` writer - Random number generator clock enable"]
pub type RNGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DMA1 clock enable"]
    #[inline(always)]
    pub fn dma1en(&self) -> DMA1EN_R {
        DMA1EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA2 clock enable"]
    #[inline(always)]
    pub fn dma2en(&self) -> DMA2EN_R {
        DMA2EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Flash memory interface clock enable"]
    #[inline(always)]
    pub fn flashen(&self) -> FLASHEN_R {
        FLASHEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC clock enable"]
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - AES hardware accelerator"]
    #[inline(always)]
    pub fn aesen(&self) -> AESEN_R {
        AESEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Random number generator clock enable"]
    #[inline(always)]
    pub fn rngen(&self) -> RNGEN_R {
        RNGEN_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma1en(&mut self) -> DMA1EN_W<AHBENRrs> {
        DMA1EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - DMA2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma2en(&mut self) -> DMA2EN_W<AHBENRrs> {
        DMA2EN_W::new(self, 1)
    }
    #[doc = "Bit 8 - Flash memory interface clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn flashen(&mut self) -> FLASHEN_W<AHBENRrs> {
        FLASHEN_W::new(self, 8)
    }
    #[doc = "Bit 12 - CRC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn crcen(&mut self) -> CRCEN_W<AHBENRrs> {
        CRCEN_W::new(self, 12)
    }
    #[doc = "Bit 16 - AES hardware accelerator"]
    #[inline(always)]
    #[must_use]
    pub fn aesen(&mut self) -> AESEN_W<AHBENRrs> {
        AESEN_W::new(self, 16)
    }
    #[doc = "Bit 18 - Random number generator clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn rngen(&mut self) -> RNGEN_W<AHBENRrs> {
        RNGEN_W::new(self, 18)
    }
}
#[doc = "AHB peripheral clock enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahbenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahbenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHBENRrs;
impl crate::RegisterSpec for AHBENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbenr::R`](R) reader structure"]
impl crate::Readable for AHBENRrs {}
#[doc = "`write(|w| ..)` method takes [`ahbenr::W`](W) writer structure"]
impl crate::Writable for AHBENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHBENR to value 0x0100"]
impl crate::Resettable for AHBENRrs {
    const RESET_VALUE: u32 = 0x0100;
}
