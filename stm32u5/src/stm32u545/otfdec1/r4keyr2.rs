#[doc = "Register `R4KEYR2` writer"]
pub type W = crate::W<R4KEYR2rs>;
#[doc = "Field `REG4_KEY` writer - REG4_KEY"]
pub type REG4_KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - REG4_KEY"]
    #[inline(always)]
    #[must_use]
    pub fn reg4_key(&mut self) -> REG4_KEY_W<R4KEYR2rs> {
        REG4_KEY_W::new(self, 0)
    }
}
#[doc = "OTFDEC region x key register 2\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r4keyr2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R4KEYR2rs;
impl crate::RegisterSpec for R4KEYR2rs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`r4keyr2::W`](W) writer structure"]
impl crate::Writable for R4KEYR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets R4KEYR2 to value 0"]
impl crate::Resettable for R4KEYR2rs {
    const RESET_VALUE: u32 = 0;
}
