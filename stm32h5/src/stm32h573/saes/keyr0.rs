///Register `KEYR0` writer
pub type W = crate::W<KEYR0rs>;
///Field `KEY` writer - Cryptographic key, bits \[31:0\] This write-only bitfield contains the bits \[31:0\] of the AES encryption or decryption key, depending on the operating mode: - In Mode 1 (encryption), Mode 2 (key derivation): the value to write into the bitfield is the encryption key. - In Mode 3 (decryption): the value to write into the bitfield is the encryption key to be derived before being used for decryption. The SAES_KEYRx registers may be written only when KEYSIZE value is correct and when the SAES peripheral is disabled (EN bit of the SAES_CR register cleared). A special writing sequence is also required, as described in KEYVALID bit of the SAES_SR register. Note that, if KEYSEL is different from 0 and KEYVALID = 0, the key is directly loaded to SAES_KEYRx registers (hence writes to key register is ignored and KEIF is set). Refer to for more details.
pub type KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<KEYR0rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - Cryptographic key, bits \[31:0\] This write-only bitfield contains the bits \[31:0\] of the AES encryption or decryption key, depending on the operating mode: - In Mode 1 (encryption), Mode 2 (key derivation): the value to write into the bitfield is the encryption key. - In Mode 3 (decryption): the value to write into the bitfield is the encryption key to be derived before being used for decryption. The SAES_KEYRx registers may be written only when KEYSIZE value is correct and when the SAES peripheral is disabled (EN bit of the SAES_CR register cleared). A special writing sequence is also required, as described in KEYVALID bit of the SAES_SR register. Note that, if KEYSEL is different from 0 and KEYVALID = 0, the key is directly loaded to SAES_KEYRx registers (hence writes to key register is ignored and KEIF is set). Refer to for more details.
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W<'_, KEYR0rs> {
        KEY_W::new(self, 0)
    }
}
/**SAES key register 0

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyr0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#SAES:KEYR0)*/
pub struct KEYR0rs;
impl crate::RegisterSpec for KEYR0rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`keyr0::W`](W) writer structure
impl crate::Writable for KEYR0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets KEYR0 to value 0
impl crate::Resettable for KEYR0rs {}
