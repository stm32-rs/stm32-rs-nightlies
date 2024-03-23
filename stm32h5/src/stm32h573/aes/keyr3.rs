#[doc = "Register `KEYR3` writer"]
pub type W = crate::W<KEYR3rs>;
#[doc = "Field `KEY` writer - Cryptographic key, bits \\[127:96\\]
Refer to the AES_KEYR0 register for description of the KEY\\[255:0\\]
bitfield."]
pub type KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Cryptographic key, bits \\[127:96\\]
Refer to the AES_KEYR0 register for description of the KEY\\[255:0\\]
bitfield."]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<KEYR3rs> {
        KEY_W::new(self, 0)
    }
}
#[doc = "AES key register 3\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keyr3::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEYR3rs;
impl crate::RegisterSpec for KEYR3rs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`keyr3::W`](W) writer structure"]
impl crate::Writable for KEYR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KEYR3 to value 0"]
impl crate::Resettable for KEYR3rs {
    const RESET_VALUE: u32 = 0;
}
