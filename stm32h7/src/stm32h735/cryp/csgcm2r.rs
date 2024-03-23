#[doc = "Register `CSGCM2R` reader"]
pub type R = crate::R<CSGCM2Rrs>;
#[doc = "Register `CSGCM2R` writer"]
pub type W = crate::W<CSGCM2Rrs>;
#[doc = "Field `CSGCM2R` reader - CSGCM2R"]
pub type CSGCM2R_R = crate::FieldReader<u32>;
#[doc = "Field `CSGCM2R` writer - CSGCM2R"]
pub type CSGCM2R_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSGCM2R"]
    #[inline(always)]
    pub fn csgcm2r(&self) -> CSGCM2R_R {
        CSGCM2R_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSGCM2R"]
    #[inline(always)]
    #[must_use]
    pub fn csgcm2r(&mut self) -> CSGCM2R_W<CSGCM2Rrs> {
        CSGCM2R_W::new(self, 0)
    }
}
#[doc = "context swap register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csgcm2r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csgcm2r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSGCM2Rrs;
impl crate::RegisterSpec for CSGCM2Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csgcm2r::R`](R) reader structure"]
impl crate::Readable for CSGCM2Rrs {}
#[doc = "`write(|w| ..)` method takes [`csgcm2r::W`](W) writer structure"]
impl crate::Writable for CSGCM2Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSGCM2R to value 0"]
impl crate::Resettable for CSGCM2Rrs {
    const RESET_VALUE: u32 = 0;
}
