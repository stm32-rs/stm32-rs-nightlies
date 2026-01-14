///Register `KEYR5` writer
pub type W = crate::W<KEYR5rs>;
///Field `KEY` writer - Cryptographic key, bits \[191:160\] Refer to the AES_KEYR0 register for description of the KEY\[255:0\] bitfield. Writing to this register has no effect when key size is 128-bit (KEYSIZE=0). When KEYVALID is set in AES_SR a write to this register clears KEYVALID if EN is cleared in AES_CR.
pub type KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<KEYR5rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - Cryptographic key, bits \[191:160\] Refer to the AES_KEYR0 register for description of the KEY\[255:0\] bitfield. Writing to this register has no effect when key size is 128-bit (KEYSIZE=0). When KEYVALID is set in AES_SR a write to this register clears KEYVALID if EN is cleared in AES_CR.
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W<'_, KEYR5rs> {
        KEY_W::new(self, 0)
    }
}
/**AES key register 5

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyr5::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#AES:KEYR5)*/
pub struct KEYR5rs;
impl crate::RegisterSpec for KEYR5rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`keyr5::W`](W) writer structure
impl crate::Writable for KEYR5rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets KEYR5 to value 0
impl crate::Resettable for KEYR5rs {}
