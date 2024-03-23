#[doc = "Register `LTDC_TWCR` reader"]
pub type R = crate::R<LTDC_TWCRrs>;
#[doc = "Register `LTDC_TWCR` writer"]
pub type W = crate::W<LTDC_TWCRrs>;
#[doc = "Field `TOTALH` reader - TOTALH"]
pub type TOTALH_R = crate::FieldReader<u16>;
#[doc = "Field `TOTALH` writer - TOTALH"]
pub type TOTALH_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `TOTALW` reader - TOTALW"]
pub type TOTALW_R = crate::FieldReader<u16>;
#[doc = "Field `TOTALW` writer - TOTALW"]
pub type TOTALW_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - TOTALH"]
    #[inline(always)]
    pub fn totalh(&self) -> TOTALH_R {
        TOTALH_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - TOTALW"]
    #[inline(always)]
    pub fn totalw(&self) -> TOTALW_R {
        TOTALW_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - TOTALH"]
    #[inline(always)]
    #[must_use]
    pub fn totalh(&mut self) -> TOTALH_W<LTDC_TWCRrs> {
        TOTALH_W::new(self, 0)
    }
    #[doc = "Bits 16:27 - TOTALW"]
    #[inline(always)]
    #[must_use]
    pub fn totalw(&mut self) -> TOTALW_W<LTDC_TWCRrs> {
        TOTALW_W::new(self, 16)
    }
}
#[doc = "This register defines the accumulated number of horizontal synchronization, back porch, active and front porch pixels minus 1 (HSYNCwidth+HBP+activewidth+HFP-1) and the accumulated number of vertical synchronization, back porch lines, active and front lines minus 1 (VSYNCheight+BVBP+activeheight+VFP-1). Refer to Figure274 and Section36.4: LTDC programmable parameters for an example of configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ltdc_twcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ltdc_twcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LTDC_TWCRrs;
impl crate::RegisterSpec for LTDC_TWCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ltdc_twcr::R`](R) reader structure"]
impl crate::Readable for LTDC_TWCRrs {}
#[doc = "`write(|w| ..)` method takes [`ltdc_twcr::W`](W) writer structure"]
impl crate::Writable for LTDC_TWCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LTDC_TWCR to value 0"]
impl crate::Resettable for LTDC_TWCRrs {
    const RESET_VALUE: u32 = 0;
}
