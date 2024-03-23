#[doc = "Register `CSGCM0R` reader"]
pub type R = crate::R<CSGCM0Rrs>;
#[doc = "Register `CSGCM0R` writer"]
pub type W = crate::W<CSGCM0Rrs>;
#[doc = "Field `CSGCM0` reader - CSGCM0"]
pub type CSGCM0_R = crate::FieldReader<u32>;
#[doc = "Field `CSGCM0` writer - CSGCM0"]
pub type CSGCM0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSGCM0"]
    #[inline(always)]
    pub fn csgcm0(&self) -> CSGCM0_R {
        CSGCM0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSGCM0"]
    #[inline(always)]
    #[must_use]
    pub fn csgcm0(&mut self) -> CSGCM0_W<CSGCM0Rrs> {
        CSGCM0_W::new(self, 0)
    }
}
#[doc = "context swap register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csgcm0r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csgcm0r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSGCM0Rrs;
impl crate::RegisterSpec for CSGCM0Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csgcm0r::R`](R) reader structure"]
impl crate::Readable for CSGCM0Rrs {}
#[doc = "`write(|w| ..)` method takes [`csgcm0r::W`](W) writer structure"]
impl crate::Writable for CSGCM0Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSGCM0R to value 0"]
impl crate::Resettable for CSGCM0Rrs {
    const RESET_VALUE: u32 = 0;
}
