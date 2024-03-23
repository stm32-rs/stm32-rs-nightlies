#[doc = "Register `R1KEYR2` writer"]
pub type W = crate::W<R1KEYR2rs>;
#[doc = "Field `REGx_KEY` writer - Region key, bits \\[95:64\\]
Refer to the OTFDEC_RxKEYR0 register for description of the KEY\\[127:0\\]
bitfield."]
pub type REGX_KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Region key, bits \\[95:64\\]
Refer to the OTFDEC_RxKEYR0 register for description of the KEY\\[127:0\\]
bitfield."]
    #[inline(always)]
    #[must_use]
    pub fn regx_key(&mut self) -> REGX_KEY_W<R1KEYR2rs> {
        REGX_KEY_W::new(self, 0)
    }
}
#[doc = "OTFDEC region 1 key register 2\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r1keyr2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R1KEYR2rs;
impl crate::RegisterSpec for R1KEYR2rs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`r1keyr2::W`](W) writer structure"]
impl crate::Writable for R1KEYR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets R1KEYR2 to value 0"]
impl crate::Resettable for R1KEYR2rs {
    const RESET_VALUE: u32 = 0;
}
