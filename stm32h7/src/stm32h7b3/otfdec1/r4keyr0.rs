#[doc = "Register `R4KEYR0` writer"]
pub type W = crate::W<R4KEYR0rs>;
#[doc = "Field `REGx_KEY` writer - REGx_KEY"]
pub type REGX_KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - REGx_KEY"]
    #[inline(always)]
    #[must_use]
    pub fn regx_key(&mut self) -> REGX_KEY_W<R4KEYR0rs> {
        REGX_KEY_W::new(self, 0)
    }
}
#[doc = "OTFDEC region x key register 0\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r4keyr0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R4KEYR0rs;
impl crate::RegisterSpec for R4KEYR0rs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`r4keyr0::W`](W) writer structure"]
impl crate::Writable for R4KEYR0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets R4KEYR0 to value 0"]
impl crate::Resettable for R4KEYR0rs {
    const RESET_VALUE: u32 = 0;
}
