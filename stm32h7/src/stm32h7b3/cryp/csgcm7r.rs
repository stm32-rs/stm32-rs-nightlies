#[doc = "Register `CSGCM7R` reader"]
pub type R = crate::R<CSGCM7Rrs>;
#[doc = "Register `CSGCM7R` writer"]
pub type W = crate::W<CSGCM7Rrs>;
#[doc = "Field `CSGCM7R` reader - CSGCM7R"]
pub type CSGCM7R_R = crate::FieldReader<u32>;
#[doc = "Field `CSGCM7R` writer - CSGCM7R"]
pub type CSGCM7R_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSGCM7R"]
    #[inline(always)]
    pub fn csgcm7r(&self) -> CSGCM7R_R {
        CSGCM7R_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSGCM7R"]
    #[inline(always)]
    #[must_use]
    pub fn csgcm7r(&mut self) -> CSGCM7R_W<CSGCM7Rrs> {
        CSGCM7R_W::new(self, 0)
    }
}
#[doc = "context swap register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csgcm7r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csgcm7r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSGCM7Rrs;
impl crate::RegisterSpec for CSGCM7Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csgcm7r::R`](R) reader structure"]
impl crate::Readable for CSGCM7Rrs {}
#[doc = "`write(|w| ..)` method takes [`csgcm7r::W`](W) writer structure"]
impl crate::Writable for CSGCM7Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSGCM7R to value 0"]
impl crate::Resettable for CSGCM7Rrs {
    const RESET_VALUE: u32 = 0;
}
