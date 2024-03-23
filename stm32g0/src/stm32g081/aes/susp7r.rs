#[doc = "Register `SUSP7R` reader"]
pub type R = crate::R<SUSP7Rrs>;
#[doc = "Register `SUSP7R` writer"]
pub type W = crate::W<SUSP7Rrs>;
#[doc = "Field `AES_SUSP7R` reader - AES suspend register 7"]
pub type AES_SUSP7R_R = crate::FieldReader<u32>;
#[doc = "Field `AES_SUSP7R` writer - AES suspend register 7"]
pub type AES_SUSP7R_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - AES suspend register 7"]
    #[inline(always)]
    pub fn aes_susp7r(&self) -> AES_SUSP7R_R {
        AES_SUSP7R_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - AES suspend register 7"]
    #[inline(always)]
    #[must_use]
    pub fn aes_susp7r(&mut self) -> AES_SUSP7R_W<SUSP7Rrs> {
        AES_SUSP7R_W::new(self, 0)
    }
}
#[doc = "AES suspend register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`susp7r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`susp7r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SUSP7Rrs;
impl crate::RegisterSpec for SUSP7Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`susp7r::R`](R) reader structure"]
impl crate::Readable for SUSP7Rrs {}
#[doc = "`write(|w| ..)` method takes [`susp7r::W`](W) writer structure"]
impl crate::Writable for SUSP7Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SUSP7R to value 0"]
impl crate::Resettable for SUSP7Rrs {
    const RESET_VALUE: u32 = 0;
}
