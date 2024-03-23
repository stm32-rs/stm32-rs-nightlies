#[doc = "Register `AHB1RSTR` reader"]
pub type R = crate::R<AHB1RSTRrs>;
#[doc = "Register `AHB1RSTR` writer"]
pub type W = crate::W<AHB1RSTRrs>;
#[doc = "Field `DMA1RST` reader - DMA1 reset"]
pub type DMA1RST_R = crate::BitReader;
#[doc = "Field `DMA1RST` writer - DMA1 reset"]
pub type DMA1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2RST` reader - DMA2 reset"]
pub type DMA2RST_R = crate::BitReader;
#[doc = "Field `DMA2RST` writer - DMA2 reset"]
pub type DMA2RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASHRST` reader - Flash memory interface reset"]
pub type FLASHRST_R = crate::BitReader;
#[doc = "Field `FLASHRST` writer - Flash memory interface reset"]
pub type FLASHRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCRST` reader - CRC reset"]
pub type CRCRST_R = crate::BitReader;
#[doc = "Field `CRCRST` writer - CRC reset"]
pub type CRCRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSCRST` reader - Touch Sensing Controller reset"]
pub type TSCRST_R = crate::BitReader;
#[doc = "Field `TSCRST` writer - Touch Sensing Controller reset"]
pub type TSCRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2DRST` reader - DMA2D reset"]
pub type DMA2DRST_R = crate::BitReader;
#[doc = "Field `DMA2DRST` writer - DMA2D reset"]
pub type DMA2DRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DMA1 reset"]
    #[inline(always)]
    pub fn dma1rst(&self) -> DMA1RST_R {
        DMA1RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA2 reset"]
    #[inline(always)]
    pub fn dma2rst(&self) -> DMA2RST_R {
        DMA2RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Flash memory interface reset"]
    #[inline(always)]
    pub fn flashrst(&self) -> FLASHRST_R {
        FLASHRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC reset"]
    #[inline(always)]
    pub fn crcrst(&self) -> CRCRST_R {
        CRCRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Touch Sensing Controller reset"]
    #[inline(always)]
    pub fn tscrst(&self) -> TSCRST_R {
        TSCRST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DMA2D reset"]
    #[inline(always)]
    pub fn dma2drst(&self) -> DMA2DRST_R {
        DMA2DRST_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn dma1rst(&mut self) -> DMA1RST_W<AHB1RSTRrs> {
        DMA1RST_W::new(self, 0)
    }
    #[doc = "Bit 1 - DMA2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn dma2rst(&mut self) -> DMA2RST_W<AHB1RSTRrs> {
        DMA2RST_W::new(self, 1)
    }
    #[doc = "Bit 8 - Flash memory interface reset"]
    #[inline(always)]
    #[must_use]
    pub fn flashrst(&mut self) -> FLASHRST_W<AHB1RSTRrs> {
        FLASHRST_W::new(self, 8)
    }
    #[doc = "Bit 12 - CRC reset"]
    #[inline(always)]
    #[must_use]
    pub fn crcrst(&mut self) -> CRCRST_W<AHB1RSTRrs> {
        CRCRST_W::new(self, 12)
    }
    #[doc = "Bit 16 - Touch Sensing Controller reset"]
    #[inline(always)]
    #[must_use]
    pub fn tscrst(&mut self) -> TSCRST_W<AHB1RSTRrs> {
        TSCRST_W::new(self, 16)
    }
    #[doc = "Bit 17 - DMA2D reset"]
    #[inline(always)]
    #[must_use]
    pub fn dma2drst(&mut self) -> DMA2DRST_W<AHB1RSTRrs> {
        DMA2DRST_W::new(self, 17)
    }
}
#[doc = "AHB1 peripheral reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb1rstr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb1rstr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB1RSTRrs;
impl crate::RegisterSpec for AHB1RSTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb1rstr::R`](R) reader structure"]
impl crate::Readable for AHB1RSTRrs {}
#[doc = "`write(|w| ..)` method takes [`ahb1rstr::W`](W) writer structure"]
impl crate::Writable for AHB1RSTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHB1RSTR to value 0"]
impl crate::Resettable for AHB1RSTRrs {
    const RESET_VALUE: u32 = 0;
}
