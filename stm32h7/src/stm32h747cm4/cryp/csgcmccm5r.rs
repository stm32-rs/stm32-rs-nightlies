#[doc = "Register `CSGCMCCM5R` reader"]
pub type R = crate::R<CSGCMCCM5Rrs>;
#[doc = "Register `CSGCMCCM5R` writer"]
pub type W = crate::W<CSGCMCCM5Rrs>;
#[doc = "Field `CSGCMCCM5R` reader - CSGCMCCM5R"]
pub type CSGCMCCM5R_R = crate::FieldReader<u32>;
#[doc = "Field `CSGCMCCM5R` writer - CSGCMCCM5R"]
pub type CSGCMCCM5R_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSGCMCCM5R"]
    #[inline(always)]
    pub fn csgcmccm5r(&self) -> CSGCMCCM5R_R {
        CSGCMCCM5R_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSGCMCCM5R"]
    #[inline(always)]
    #[must_use]
    pub fn csgcmccm5r(&mut self) -> CSGCMCCM5R_W<CSGCMCCM5Rrs> {
        CSGCMCCM5R_W::new(self, 0)
    }
}
#[doc = "context swap register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csgcmccm5r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csgcmccm5r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSGCMCCM5Rrs;
impl crate::RegisterSpec for CSGCMCCM5Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csgcmccm5r::R`](R) reader structure"]
impl crate::Readable for CSGCMCCM5Rrs {}
#[doc = "`write(|w| ..)` method takes [`csgcmccm5r::W`](W) writer structure"]
impl crate::Writable for CSGCMCCM5Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSGCMCCM5R to value 0"]
impl crate::Resettable for CSGCMCCM5Rrs {
    const RESET_VALUE: u32 = 0;
}
