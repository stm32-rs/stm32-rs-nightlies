#[doc = "Register `KEYR4` writer"]
pub type W = crate::W<KEYR4rs>;
#[doc = "Field `KEY` writer - Cryptographic key, bits \\[159:128\\])"]
pub type KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Cryptographic key, bits \\[159:128\\])"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<KEYR4rs> {
        KEY_W::new(self, 0)
    }
}
#[doc = "key register 4\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keyr4::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEYR4rs;
impl crate::RegisterSpec for KEYR4rs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`keyr4::W`](W) writer structure"]
impl crate::Writable for KEYR4rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KEYR4 to value 0"]
impl crate::Resettable for KEYR4rs {
    const RESET_VALUE: u32 = 0;
}
