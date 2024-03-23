#[doc = "Register `AHBRSTR` reader"]
pub type R = crate::R<AHBRSTRrs>;
#[doc = "Register `AHBRSTR` writer"]
pub type W = crate::W<AHBRSTRrs>;
#[doc = "Field `DMA1RST` reader - DMA1 and DMAMUX reset Set and cleared by software."]
pub type DMA1RST_R = crate::BitReader;
#[doc = "Field `DMA1RST` writer - DMA1 and DMAMUX reset Set and cleared by software."]
pub type DMA1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASHRST` reader - Flash memory interface reset Set and cleared by software. This bit can only be set when the Flash memory is in power down mode."]
pub type FLASHRST_R = crate::BitReader;
#[doc = "Field `FLASHRST` writer - Flash memory interface reset Set and cleared by software. This bit can only be set when the Flash memory is in power down mode."]
pub type FLASHRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCRST` reader - CRC reset Set and cleared by software."]
pub type CRCRST_R = crate::BitReader;
#[doc = "Field `CRCRST` writer - CRC reset Set and cleared by software."]
pub type CRCRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DMA1 and DMAMUX reset Set and cleared by software."]
    #[inline(always)]
    pub fn dma1rst(&self) -> DMA1RST_R {
        DMA1RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Flash memory interface reset Set and cleared by software. This bit can only be set when the Flash memory is in power down mode."]
    #[inline(always)]
    pub fn flashrst(&self) -> FLASHRST_R {
        FLASHRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC reset Set and cleared by software."]
    #[inline(always)]
    pub fn crcrst(&self) -> CRCRST_R {
        CRCRST_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA1 and DMAMUX reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn dma1rst(&mut self) -> DMA1RST_W<AHBRSTRrs> {
        DMA1RST_W::new(self, 0)
    }
    #[doc = "Bit 8 - Flash memory interface reset Set and cleared by software. This bit can only be set when the Flash memory is in power down mode."]
    #[inline(always)]
    #[must_use]
    pub fn flashrst(&mut self) -> FLASHRST_W<AHBRSTRrs> {
        FLASHRST_W::new(self, 8)
    }
    #[doc = "Bit 12 - CRC reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn crcrst(&mut self) -> CRCRST_W<AHBRSTRrs> {
        CRCRST_W::new(self, 12)
    }
}
#[doc = "RCC AHB peripheral reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahbrstr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahbrstr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHBRSTRrs;
impl crate::RegisterSpec for AHBRSTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbrstr::R`](R) reader structure"]
impl crate::Readable for AHBRSTRrs {}
#[doc = "`write(|w| ..)` method takes [`ahbrstr::W`](W) writer structure"]
impl crate::Writable for AHBRSTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHBRSTR to value 0"]
impl crate::Resettable for AHBRSTRrs {
    const RESET_VALUE: u32 = 0;
}
