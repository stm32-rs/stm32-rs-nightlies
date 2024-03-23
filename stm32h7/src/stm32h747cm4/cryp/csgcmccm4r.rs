#[doc = "Register `CSGCMCCM4R` reader"]
pub type R = crate::R<CSGCMCCM4Rrs>;
#[doc = "Register `CSGCMCCM4R` writer"]
pub type W = crate::W<CSGCMCCM4Rrs>;
#[doc = "Field `CSGCMCCM4R` reader - CSGCMCCM4R"]
pub type CSGCMCCM4R_R = crate::FieldReader<u32>;
#[doc = "Field `CSGCMCCM4R` writer - CSGCMCCM4R"]
pub type CSGCMCCM4R_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSGCMCCM4R"]
    #[inline(always)]
    pub fn csgcmccm4r(&self) -> CSGCMCCM4R_R {
        CSGCMCCM4R_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSGCMCCM4R"]
    #[inline(always)]
    #[must_use]
    pub fn csgcmccm4r(&mut self) -> CSGCMCCM4R_W<CSGCMCCM4Rrs> {
        CSGCMCCM4R_W::new(self, 0)
    }
}
#[doc = "context swap register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csgcmccm4r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csgcmccm4r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSGCMCCM4Rrs;
impl crate::RegisterSpec for CSGCMCCM4Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csgcmccm4r::R`](R) reader structure"]
impl crate::Readable for CSGCMCCM4Rrs {}
#[doc = "`write(|w| ..)` method takes [`csgcmccm4r::W`](W) writer structure"]
impl crate::Writable for CSGCMCCM4Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSGCMCCM4R to value 0"]
impl crate::Resettable for CSGCMCCM4Rrs {
    const RESET_VALUE: u32 = 0;
}
