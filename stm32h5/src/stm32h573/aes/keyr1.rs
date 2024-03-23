#[doc = "Register `KEYR1` writer"]
pub type W = crate::W<KEYR1rs>;
#[doc = "Field `KEY` writer - Cryptographic key, bits \\[63:32\\]
Refer to the AES_KEYR0 register for description of the KEY\\[255:0\\]
bitfield."]
pub type KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Cryptographic key, bits \\[63:32\\]
Refer to the AES_KEYR0 register for description of the KEY\\[255:0\\]
bitfield."]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<KEYR1rs> {
        KEY_W::new(self, 0)
    }
}
#[doc = "AES key register 1\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keyr1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEYR1rs;
impl crate::RegisterSpec for KEYR1rs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`keyr1::W`](W) writer structure"]
impl crate::Writable for KEYR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KEYR1 to value 0"]
impl crate::Resettable for KEYR1rs {
    const RESET_VALUE: u32 = 0;
}
