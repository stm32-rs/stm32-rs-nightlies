#[doc = "Register `KEYR0` writer"]
pub type W = crate::W<KEYR0rs>;
#[doc = "Field `KEY` writer - Data Output Register (LSB key \\[31:0\\])"]
pub type KEY_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Data Output Register (LSB key \\[31:0\\])"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<KEYR0rs> {
        KEY_W::new(self, 0)
    }
}
#[doc = "key register 0\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keyr0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEYR0rs;
impl crate::RegisterSpec for KEYR0rs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`keyr0::W`](W) writer structure"]
impl crate::Writable for KEYR0rs {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KEYR0 to value 0"]
impl crate::Resettable for KEYR0rs {
    const RESET_VALUE: u32 = 0;
}
