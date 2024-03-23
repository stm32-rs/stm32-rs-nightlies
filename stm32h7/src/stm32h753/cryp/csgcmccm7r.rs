#[doc = "Register `CSGCMCCM7R` reader"]
pub type R = crate::R<CSGCMCCM7Rrs>;
#[doc = "Register `CSGCMCCM7R` writer"]
pub type W = crate::W<CSGCMCCM7Rrs>;
#[doc = "Field `CSGCMCCM7` reader - CSGCMCCM7"]
pub type CSGCMCCM7_R = crate::FieldReader<u32>;
#[doc = "Field `CSGCMCCM7` writer - CSGCMCCM7"]
pub type CSGCMCCM7_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSGCMCCM7"]
    #[inline(always)]
    pub fn csgcmccm7(&self) -> CSGCMCCM7_R {
        CSGCMCCM7_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSGCMCCM7"]
    #[inline(always)]
    #[must_use]
    pub fn csgcmccm7(&mut self) -> CSGCMCCM7_W<CSGCMCCM7Rrs> {
        CSGCMCCM7_W::new(self, 0)
    }
}
#[doc = "context swap register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csgcmccm7r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csgcmccm7r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSGCMCCM7Rrs;
impl crate::RegisterSpec for CSGCMCCM7Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csgcmccm7r::R`](R) reader structure"]
impl crate::Readable for CSGCMCCM7Rrs {}
#[doc = "`write(|w| ..)` method takes [`csgcmccm7r::W`](W) writer structure"]
impl crate::Writable for CSGCMCCM7Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSGCMCCM7R to value 0"]
impl crate::Resettable for CSGCMCCM7Rrs {
    const RESET_VALUE: u32 = 0;
}
