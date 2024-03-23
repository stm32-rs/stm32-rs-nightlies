#[doc = "Register `LTDC_BPCR` reader"]
pub type R = crate::R<LTDC_BPCRrs>;
#[doc = "Register `LTDC_BPCR` writer"]
pub type W = crate::W<LTDC_BPCRrs>;
#[doc = "Field `AVBP` reader - AVBP"]
pub type AVBP_R = crate::FieldReader<u16>;
#[doc = "Field `AVBP` writer - AVBP"]
pub type AVBP_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `AHBP` reader - AHBP"]
pub type AHBP_R = crate::FieldReader<u16>;
#[doc = "Field `AHBP` writer - AHBP"]
pub type AHBP_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - AVBP"]
    #[inline(always)]
    pub fn avbp(&self) -> AVBP_R {
        AVBP_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - AHBP"]
    #[inline(always)]
    pub fn ahbp(&self) -> AHBP_R {
        AHBP_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - AVBP"]
    #[inline(always)]
    #[must_use]
    pub fn avbp(&mut self) -> AVBP_W<LTDC_BPCRrs> {
        AVBP_W::new(self, 0)
    }
    #[doc = "Bits 16:27 - AHBP"]
    #[inline(always)]
    #[must_use]
    pub fn ahbp(&mut self) -> AHBP_W<LTDC_BPCRrs> {
        AHBP_W::new(self, 16)
    }
}
#[doc = "This register defines the accumulated number of horizontal synchronization and back porch pixels minus 1 (HSYNCwidth+HBP-1) and the accumulated number of vertical synchronization and back porch lines minus 1 (VSYNCheight+VBP-1). Refer to Figure274 and Section36.4: LTDC programmable parameters for an example of configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ltdc_bpcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ltdc_bpcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LTDC_BPCRrs;
impl crate::RegisterSpec for LTDC_BPCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ltdc_bpcr::R`](R) reader structure"]
impl crate::Readable for LTDC_BPCRrs {}
#[doc = "`write(|w| ..)` method takes [`ltdc_bpcr::W`](W) writer structure"]
impl crate::Writable for LTDC_BPCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LTDC_BPCR to value 0"]
impl crate::Resettable for LTDC_BPCRrs {
    const RESET_VALUE: u32 = 0;
}
