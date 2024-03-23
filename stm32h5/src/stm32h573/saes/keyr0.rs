#[doc = "Register `KEYR0` writer"]
pub type W = crate::W<KEYR0rs>;
#[doc = "Field `KEY` writer - Cryptographic key, bits \\[31:0\\]
This write-only bitfield contains the bits \\[31:0\\]
of the AES encryption or decryption key, depending on the operating mode: - In Mode 1 (encryption), Mode 2 (key derivation): the value to write into the bitfield is the encryption key. - In Mode 3 (decryption): the value to write into the bitfield is the encryption key to be derived before being used for decryption. The SAES_KEYRx registers may be written only when KEYSIZE value is correct and when the SAES peripheral is disabled (EN bit of the SAES_CR register cleared). A special writing sequence is also required, as described in KEYVALID bit of the SAES_SR register. Note that, if KEYSEL is different from 0 and KEYVALID = 0, the key is directly loaded to SAES_KEYRx registers (hence writes to key register is ignored and KEIF is set). Refer to for more details."]
pub type KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Cryptographic key, bits \\[31:0\\]
This write-only bitfield contains the bits \\[31:0\\]
of the AES encryption or decryption key, depending on the operating mode: - In Mode 1 (encryption), Mode 2 (key derivation): the value to write into the bitfield is the encryption key. - In Mode 3 (decryption): the value to write into the bitfield is the encryption key to be derived before being used for decryption. The SAES_KEYRx registers may be written only when KEYSIZE value is correct and when the SAES peripheral is disabled (EN bit of the SAES_CR register cleared). A special writing sequence is also required, as described in KEYVALID bit of the SAES_SR register. Note that, if KEYSEL is different from 0 and KEYVALID = 0, the key is directly loaded to SAES_KEYRx registers (hence writes to key register is ignored and KEIF is set). Refer to for more details."]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<KEYR0rs> {
        KEY_W::new(self, 0)
    }
}
#[doc = "SAES key register 0\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keyr0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEYR0rs;
impl crate::RegisterSpec for KEYR0rs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`keyr0::W`](W) writer structure"]
impl crate::Writable for KEYR0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KEYR0 to value 0"]
impl crate::Resettable for KEYR0rs {
    const RESET_VALUE: u32 = 0;
}
