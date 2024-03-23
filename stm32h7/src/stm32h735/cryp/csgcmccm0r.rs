#[doc = "Register `CSGCMCCM0R` reader"]
pub type R = crate::R<CSGCMCCM0Rrs>;
#[doc = "Register `CSGCMCCM0R` writer"]
pub type W = crate::W<CSGCMCCM0Rrs>;
#[doc = "Field `CSGCMCCM0R` reader - CSGCMCCM0R"]
pub type CSGCMCCM0R_R = crate::FieldReader<u32>;
#[doc = "Field `CSGCMCCM0R` writer - CSGCMCCM0R"]
pub type CSGCMCCM0R_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSGCMCCM0R"]
    #[inline(always)]
    pub fn csgcmccm0r(&self) -> CSGCMCCM0R_R {
        CSGCMCCM0R_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSGCMCCM0R"]
    #[inline(always)]
    #[must_use]
    pub fn csgcmccm0r(&mut self) -> CSGCMCCM0R_W<CSGCMCCM0Rrs> {
        CSGCMCCM0R_W::new(self, 0)
    }
}
#[doc = "context swap register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csgcmccm0r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csgcmccm0r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSGCMCCM0Rrs;
impl crate::RegisterSpec for CSGCMCCM0Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csgcmccm0r::R`](R) reader structure"]
impl crate::Readable for CSGCMCCM0Rrs {}
#[doc = "`write(|w| ..)` method takes [`csgcmccm0r::W`](W) writer structure"]
impl crate::Writable for CSGCMCCM0Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSGCMCCM0R to value 0"]
impl crate::Resettable for CSGCMCCM0Rrs {
    const RESET_VALUE: u32 = 0;
}
