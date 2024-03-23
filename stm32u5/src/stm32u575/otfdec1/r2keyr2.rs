#[doc = "Register `R2KEYR2` writer"]
pub type W = crate::W<R2KEYR2rs>;
#[doc = "Field `REGx_KEY_` writer - REGx_KEY"]
pub type REGX_KEY__W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - REGx_KEY"]
    #[inline(always)]
    #[must_use]
    pub fn regx_key_(&mut self) -> REGX_KEY__W<R2KEYR2rs> {
        REGX_KEY__W::new(self, 0)
    }
}
#[doc = "OTFDEC region x key register 2\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r2keyr2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R2KEYR2rs;
impl crate::RegisterSpec for R2KEYR2rs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`r2keyr2::W`](W) writer structure"]
impl crate::Writable for R2KEYR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets R2KEYR2 to value 0"]
impl crate::Resettable for R2KEYR2rs {
    const RESET_VALUE: u32 = 0;
}
