#[doc = "Register `CSGCM4R` reader"]
pub type R = crate::R<CSGCM4Rrs>;
#[doc = "Register `CSGCM4R` writer"]
pub type W = crate::W<CSGCM4Rrs>;
#[doc = "Field `CSGCM4R` reader - CSGCM4R"]
pub type CSGCM4R_R = crate::FieldReader<u32>;
#[doc = "Field `CSGCM4R` writer - CSGCM4R"]
pub type CSGCM4R_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSGCM4R"]
    #[inline(always)]
    pub fn csgcm4r(&self) -> CSGCM4R_R {
        CSGCM4R_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSGCM4R"]
    #[inline(always)]
    #[must_use]
    pub fn csgcm4r(&mut self) -> CSGCM4R_W<CSGCM4Rrs> {
        CSGCM4R_W::new(self, 0)
    }
}
#[doc = "context swap register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csgcm4r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csgcm4r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSGCM4Rrs;
impl crate::RegisterSpec for CSGCM4Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csgcm4r::R`](R) reader structure"]
impl crate::Readable for CSGCM4Rrs {}
#[doc = "`write(|w| ..)` method takes [`csgcm4r::W`](W) writer structure"]
impl crate::Writable for CSGCM4Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSGCM4R to value 0"]
impl crate::Resettable for CSGCM4Rrs {
    const RESET_VALUE: u32 = 0;
}
