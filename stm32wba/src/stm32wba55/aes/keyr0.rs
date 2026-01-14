///Register `KEYR0` writer
pub type W = crate::W<KEYR0rs>;
///Field `KEY` writer - Cryptographic key, bits \[31:0\] This write-only bitfield contains the bits \[31:0\] of the AES encryption or decryption key, depending on the operating mode MODE\[1:0\] in AES_CR. The AES_KEYRx registers may be written only when KEYSIZE value is correct and when the AES peripheral is disabled (EN bit of the AES_CR register cleared). A special writing sequence is also required, as described in KEYVALID bit of the AES_SR register. Note that, if KMOD\[1:0\] is at 10 (shared key), the key is directly loaded from SAES peripheral to AES_KEYRx registers (hence writes to key register is ignored and KEIF is set). When KEYVALID is set a write to this register clears KEYVALID if AES is disabled.
pub type KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<KEYR0rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - Cryptographic key, bits \[31:0\] This write-only bitfield contains the bits \[31:0\] of the AES encryption or decryption key, depending on the operating mode MODE\[1:0\] in AES_CR. The AES_KEYRx registers may be written only when KEYSIZE value is correct and when the AES peripheral is disabled (EN bit of the AES_CR register cleared). A special writing sequence is also required, as described in KEYVALID bit of the AES_SR register. Note that, if KMOD\[1:0\] is at 10 (shared key), the key is directly loaded from SAES peripheral to AES_KEYRx registers (hence writes to key register is ignored and KEIF is set). When KEYVALID is set a write to this register clears KEYVALID if AES is disabled.
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W<'_, KEYR0rs> {
        KEY_W::new(self, 0)
    }
}
/**AES key register 0

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyr0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#AES:KEYR0)*/
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
