#[doc = "Register `CSGCMCCM3R` reader"]
pub type R = crate::R<CSGCMCCM3Rrs>;
#[doc = "Register `CSGCMCCM3R` writer"]
pub type W = crate::W<CSGCMCCM3Rrs>;
#[doc = "Field `CSGCMCCM3` reader - CSGCMCCM3"]
pub type CSGCMCCM3_R = crate::FieldReader<u32>;
#[doc = "Field `CSGCMCCM3` writer - CSGCMCCM3"]
pub type CSGCMCCM3_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSGCMCCM3"]
    #[inline(always)]
    pub fn csgcmccm3(&self) -> CSGCMCCM3_R {
        CSGCMCCM3_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSGCMCCM3"]
    #[inline(always)]
    #[must_use]
    pub fn csgcmccm3(&mut self) -> CSGCMCCM3_W<CSGCMCCM3Rrs> {
        CSGCMCCM3_W::new(self, 0)
    }
}
#[doc = "context swap register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csgcmccm3r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csgcmccm3r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSGCMCCM3Rrs;
impl crate::RegisterSpec for CSGCMCCM3Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csgcmccm3r::R`](R) reader structure"]
impl crate::Readable for CSGCMCCM3Rrs {}
#[doc = "`write(|w| ..)` method takes [`csgcmccm3r::W`](W) writer structure"]
impl crate::Writable for CSGCMCCM3Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSGCMCCM3R to value 0"]
impl crate::Resettable for CSGCMCCM3Rrs {
    const RESET_VALUE: u32 = 0;
}
