#[doc = "Register `R4NONCER0` reader"]
pub type R = crate::R<R4NONCER0rs>;
#[doc = "Register `R4NONCER0` writer"]
pub type W = crate::W<R4NONCER0rs>;
#[doc = "Field `REGx_NONCE` reader - REGx_NONCE"]
pub type REGX_NONCE_R = crate::FieldReader<u32>;
#[doc = "Field `REGx_NONCE` writer - REGx_NONCE"]
pub type REGX_NONCE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REGx_NONCE"]
    #[inline(always)]
    pub fn regx_nonce(&self) -> REGX_NONCE_R {
        REGX_NONCE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REGx_NONCE"]
    #[inline(always)]
    #[must_use]
    pub fn regx_nonce(&mut self) -> REGX_NONCE_W<R4NONCER0rs> {
        REGX_NONCE_W::new(self, 0)
    }
}
#[doc = "OTFDEC region x nonce register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r4noncer0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r4noncer0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R4NONCER0rs;
impl crate::RegisterSpec for R4NONCER0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r4noncer0::R`](R) reader structure"]
impl crate::Readable for R4NONCER0rs {}
#[doc = "`write(|w| ..)` method takes [`r4noncer0::W`](W) writer structure"]
impl crate::Writable for R4NONCER0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets R4NONCER0 to value 0"]
impl crate::Resettable for R4NONCER0rs {
    const RESET_VALUE: u32 = 0;
}
