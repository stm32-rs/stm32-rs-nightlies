#[doc = "Register `CSGCMCCM2R` reader"]
pub type R = crate::R<CSGCMCCM2Rrs>;
#[doc = "Register `CSGCMCCM2R` writer"]
pub type W = crate::W<CSGCMCCM2Rrs>;
#[doc = "Field `CSGCMCCM2R` reader - CSGCMCCM2R"]
pub type CSGCMCCM2R_R = crate::FieldReader<u32>;
#[doc = "Field `CSGCMCCM2R` writer - CSGCMCCM2R"]
pub type CSGCMCCM2R_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSGCMCCM2R"]
    #[inline(always)]
    pub fn csgcmccm2r(&self) -> CSGCMCCM2R_R {
        CSGCMCCM2R_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSGCMCCM2R"]
    #[inline(always)]
    #[must_use]
    pub fn csgcmccm2r(&mut self) -> CSGCMCCM2R_W<CSGCMCCM2Rrs> {
        CSGCMCCM2R_W::new(self, 0)
    }
}
#[doc = "context swap register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csgcmccm2r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csgcmccm2r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSGCMCCM2Rrs;
impl crate::RegisterSpec for CSGCMCCM2Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csgcmccm2r::R`](R) reader structure"]
impl crate::Readable for CSGCMCCM2Rrs {}
#[doc = "`write(|w| ..)` method takes [`csgcmccm2r::W`](W) writer structure"]
impl crate::Writable for CSGCMCCM2Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSGCMCCM2R to value 0"]
impl crate::Resettable for CSGCMCCM2Rrs {
    const RESET_VALUE: u32 = 0;
}
