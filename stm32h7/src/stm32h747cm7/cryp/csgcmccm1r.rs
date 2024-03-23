#[doc = "Register `CSGCMCCM1R` reader"]
pub type R = crate::R<CSGCMCCM1Rrs>;
#[doc = "Register `CSGCMCCM1R` writer"]
pub type W = crate::W<CSGCMCCM1Rrs>;
#[doc = "Field `CSGCMCCM1R` reader - CSGCMCCM1R"]
pub type CSGCMCCM1R_R = crate::FieldReader<u32>;
#[doc = "Field `CSGCMCCM1R` writer - CSGCMCCM1R"]
pub type CSGCMCCM1R_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSGCMCCM1R"]
    #[inline(always)]
    pub fn csgcmccm1r(&self) -> CSGCMCCM1R_R {
        CSGCMCCM1R_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSGCMCCM1R"]
    #[inline(always)]
    #[must_use]
    pub fn csgcmccm1r(&mut self) -> CSGCMCCM1R_W<CSGCMCCM1Rrs> {
        CSGCMCCM1R_W::new(self, 0)
    }
}
#[doc = "context swap register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csgcmccm1r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csgcmccm1r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSGCMCCM1Rrs;
impl crate::RegisterSpec for CSGCMCCM1Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csgcmccm1r::R`](R) reader structure"]
impl crate::Readable for CSGCMCCM1Rrs {}
#[doc = "`write(|w| ..)` method takes [`csgcmccm1r::W`](W) writer structure"]
impl crate::Writable for CSGCMCCM1Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSGCMCCM1R to value 0"]
impl crate::Resettable for CSGCMCCM1Rrs {
    const RESET_VALUE: u32 = 0;
}
