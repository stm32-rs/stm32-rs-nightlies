#[doc = "Register `SUSP6R` reader"]
pub type R = crate::R<SUSP6Rrs>;
#[doc = "Register `SUSP6R` writer"]
pub type W = crate::W<SUSP6Rrs>;
#[doc = "Field `AES_SUSP6R` reader - AES suspend register 6"]
pub type AES_SUSP6R_R = crate::FieldReader<u32>;
#[doc = "Field `AES_SUSP6R` writer - AES suspend register 6"]
pub type AES_SUSP6R_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - AES suspend register 6"]
    #[inline(always)]
    pub fn aes_susp6r(&self) -> AES_SUSP6R_R {
        AES_SUSP6R_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - AES suspend register 6"]
    #[inline(always)]
    #[must_use]
    pub fn aes_susp6r(&mut self) -> AES_SUSP6R_W<SUSP6Rrs> {
        AES_SUSP6R_W::new(self, 0)
    }
}
#[doc = "AES suspend register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`susp6r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`susp6r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SUSP6Rrs;
impl crate::RegisterSpec for SUSP6Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`susp6r::R`](R) reader structure"]
impl crate::Readable for SUSP6Rrs {}
#[doc = "`write(|w| ..)` method takes [`susp6r::W`](W) writer structure"]
impl crate::Writable for SUSP6Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SUSP6R to value 0"]
impl crate::Resettable for SUSP6Rrs {
    const RESET_VALUE: u32 = 0;
}
