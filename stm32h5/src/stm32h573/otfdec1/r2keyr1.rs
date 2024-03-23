#[doc = "Register `R2KEYR1` writer"]
pub type W = crate::W<R2KEYR1rs>;
#[doc = "Field `REGx_KEY` writer - Region key, bits \\[63:32\\]
Refer to the OTFDEC_RxKEYR0 register for description of the KEY\\[127:0\\]
bitfield."]
pub type REGX_KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Region key, bits \\[63:32\\]
Refer to the OTFDEC_RxKEYR0 register for description of the KEY\\[127:0\\]
bitfield."]
    #[inline(always)]
    #[must_use]
    pub fn regx_key(&mut self) -> REGX_KEY_W<R2KEYR1rs> {
        REGX_KEY_W::new(self, 0)
    }
}
#[doc = "OTFDEC region 2 key register 1\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r2keyr1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R2KEYR1rs;
impl crate::RegisterSpec for R2KEYR1rs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`r2keyr1::W`](W) writer structure"]
impl crate::Writable for R2KEYR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets R2KEYR1 to value 0"]
impl crate::Resettable for R2KEYR1rs {
    const RESET_VALUE: u32 = 0;
}
