#[doc = "Register `R4KEYR3` writer"]
pub type W = crate::W<R4KEYR3rs>;
#[doc = "Field `REG4_KEY` writer - REG4_KEY"]
pub type REG4_KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - REG4_KEY"]
    #[inline(always)]
    #[must_use]
    pub fn reg4_key(&mut self) -> REG4_KEY_W<R4KEYR3rs> {
        REG4_KEY_W::new(self, 0)
    }
}
#[doc = "OTFDEC region x key register 3\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r4keyr3::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R4KEYR3rs;
impl crate::RegisterSpec for R4KEYR3rs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`r4keyr3::W`](W) writer structure"]
impl crate::Writable for R4KEYR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets R4KEYR3 to value 0"]
impl crate::Resettable for R4KEYR3rs {
    const RESET_VALUE: u32 = 0;
}
