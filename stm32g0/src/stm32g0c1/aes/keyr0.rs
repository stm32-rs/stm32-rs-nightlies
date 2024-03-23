#[doc = "Register `KEYR0` reader"]
pub type R = crate::R<KEYR0rs>;
#[doc = "Register `KEYR0` writer"]
pub type W = crate::W<KEYR0rs>;
#[doc = "Field `KEY` reader - Cryptographic key, bits \\[31:0\\]
This bitfield contains the bits \\[31:0\\]
of the AES encryption or decryption key, depending on the operating mode: - In Mode 1 (encryption), Mode 2 (key derivation) and Mode 4 (key derivation then single decryption): the value to write into the bitfield is the encryption key. - In Mode 3 (decryption): the value to write into the bitfield is the encryption key to be derived before being used for decryption. After writing the encryption key into the bitfield, its reading before enabling AES returns the same value. Its reading after enabling AES and after the CCF flag is set returns the decryption key derived from the encryption key. Note: In mode 4 (key derivation then single decryption) the bitfield always contains the encryption key. The AES_KEYRx registers may be written only when KEYSIZE value is correct and when the AES peripheral is disabled (EN bit of the AES_CR register cleared). Note that, if, the key is directly loaded to AES_KEYRx registers (hence writes to key register is ignored and KEIF is set). Refer to for more details."]
pub type KEY_R = crate::FieldReader<u32>;
#[doc = "Field `KEY` writer - Cryptographic key, bits \\[31:0\\]
This bitfield contains the bits \\[31:0\\]
of the AES encryption or decryption key, depending on the operating mode: - In Mode 1 (encryption), Mode 2 (key derivation) and Mode 4 (key derivation then single decryption): the value to write into the bitfield is the encryption key. - In Mode 3 (decryption): the value to write into the bitfield is the encryption key to be derived before being used for decryption. After writing the encryption key into the bitfield, its reading before enabling AES returns the same value. Its reading after enabling AES and after the CCF flag is set returns the decryption key derived from the encryption key. Note: In mode 4 (key derivation then single decryption) the bitfield always contains the encryption key. The AES_KEYRx registers may be written only when KEYSIZE value is correct and when the AES peripheral is disabled (EN bit of the AES_CR register cleared). Note that, if, the key is directly loaded to AES_KEYRx registers (hence writes to key register is ignored and KEIF is set). Refer to for more details."]
pub type KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Cryptographic key, bits \\[31:0\\]
This bitfield contains the bits \\[31:0\\]
of the AES encryption or decryption key, depending on the operating mode: - In Mode 1 (encryption), Mode 2 (key derivation) and Mode 4 (key derivation then single decryption): the value to write into the bitfield is the encryption key. - In Mode 3 (decryption): the value to write into the bitfield is the encryption key to be derived before being used for decryption. After writing the encryption key into the bitfield, its reading before enabling AES returns the same value. Its reading after enabling AES and after the CCF flag is set returns the decryption key derived from the encryption key. Note: In mode 4 (key derivation then single decryption) the bitfield always contains the encryption key. The AES_KEYRx registers may be written only when KEYSIZE value is correct and when the AES peripheral is disabled (EN bit of the AES_CR register cleared). Note that, if, the key is directly loaded to AES_KEYRx registers (hence writes to key register is ignored and KEIF is set). Refer to for more details."]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Cryptographic key, bits \\[31:0\\]
This bitfield contains the bits \\[31:0\\]
of the AES encryption or decryption key, depending on the operating mode: - In Mode 1 (encryption), Mode 2 (key derivation) and Mode 4 (key derivation then single decryption): the value to write into the bitfield is the encryption key. - In Mode 3 (decryption): the value to write into the bitfield is the encryption key to be derived before being used for decryption. After writing the encryption key into the bitfield, its reading before enabling AES returns the same value. Its reading after enabling AES and after the CCF flag is set returns the decryption key derived from the encryption key. Note: In mode 4 (key derivation then single decryption) the bitfield always contains the encryption key. The AES_KEYRx registers may be written only when KEYSIZE value is correct and when the AES peripheral is disabled (EN bit of the AES_CR register cleared). Note that, if, the key is directly loaded to AES_KEYRx registers (hence writes to key register is ignored and KEIF is set). Refer to for more details."]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<KEYR0rs> {
        KEY_W::new(self, 0)
    }
}
#[doc = "AES key register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`keyr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keyr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEYR0rs;
impl crate::RegisterSpec for KEYR0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`keyr0::R`](R) reader structure"]
impl crate::Readable for KEYR0rs {}
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
