#[doc = "Register `CSGCMCCM5R` reader"]
pub type R = crate::R<CSGCMCCM5Rrs>;
#[doc = "Register `CSGCMCCM5R` writer"]
pub type W = crate::W<CSGCMCCM5Rrs>;
#[doc = "Field `CSGCMCCM5` reader - CSGCMCCM5"]
pub type CSGCMCCM5_R = crate::FieldReader<u32>;
#[doc = "Field `CSGCMCCM5` writer - CSGCMCCM5"]
pub type CSGCMCCM5_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSGCMCCM5"]
    #[inline(always)]
    pub fn csgcmccm5(&self) -> CSGCMCCM5_R {
        CSGCMCCM5_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSGCMCCM5"]
    #[inline(always)]
    #[must_use]
    pub fn csgcmccm5(&mut self) -> CSGCMCCM5_W<CSGCMCCM5Rrs> {
        CSGCMCCM5_W::new(self, 0)
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
