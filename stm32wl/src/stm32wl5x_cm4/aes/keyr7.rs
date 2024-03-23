#[doc = "Register `KEYR7` writer"]
pub type W = crate::W<KEYR7rs>;
#[doc = "Field `KEY` writer - AES key register (MSB key \\[255:224\\])"]
pub type KEY_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - AES key register (MSB key \\[255:224\\])"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<KEYR7rs> {
        KEY_W::new(self, 0)
    }
}
#[doc = "key register 7\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keyr7::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEYR7rs;
impl crate::RegisterSpec for KEYR7rs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`keyr7::W`](W) writer structure"]
impl crate::Writable for KEYR7rs {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KEYR7 to value 0"]
impl crate::Resettable for KEYR7rs {
    const RESET_VALUE: u32 = 0;
}
