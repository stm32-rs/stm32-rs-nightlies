#[doc = "Register `CSGCMCCM6R` reader"]
pub type R = crate::R<CSGCMCCM6Rrs>;
#[doc = "Register `CSGCMCCM6R` writer"]
pub type W = crate::W<CSGCMCCM6Rrs>;
#[doc = "Field `CSGCMCCM6R` reader - CSGCMCCM6R"]
pub type CSGCMCCM6R_R = crate::FieldReader<u32>;
#[doc = "Field `CSGCMCCM6R` writer - CSGCMCCM6R"]
pub type CSGCMCCM6R_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSGCMCCM6R"]
    #[inline(always)]
    pub fn csgcmccm6r(&self) -> CSGCMCCM6R_R {
        CSGCMCCM6R_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSGCMCCM6R"]
    #[inline(always)]
    #[must_use]
    pub fn csgcmccm6r(&mut self) -> CSGCMCCM6R_W<CSGCMCCM6Rrs> {
        CSGCMCCM6R_W::new(self, 0)
    }
}
#[doc = "context swap register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csgcmccm6r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csgcmccm6r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSGCMCCM6Rrs;
impl crate::RegisterSpec for CSGCMCCM6Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csgcmccm6r::R`](R) reader structure"]
impl crate::Readable for CSGCMCCM6Rrs {}
#[doc = "`write(|w| ..)` method takes [`csgcmccm6r::W`](W) writer structure"]
impl crate::Writable for CSGCMCCM6Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSGCMCCM6R to value 0"]
impl crate::Resettable for CSGCMCCM6Rrs {
    const RESET_VALUE: u32 = 0;
}
