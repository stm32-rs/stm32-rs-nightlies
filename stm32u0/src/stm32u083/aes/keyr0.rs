///Register `KEYR0` writer
pub type W = crate::W<KEYR0rs>;
///Field `KEY` writer - Cryptographic key, bits \[31:0\] These are bits \[31:0\] of the write-only bitfield KEY\[255:0\] AES encryption or decryption key, depending on the MODE\[1:0\] bitfield of the AES_CR register. Writes to AES_KEYRx registers are ignored when AES is enabled (EN bit set). A special writing sequence is required. In this sequence, any valid write to AES_KEYRx register clears the KEYVALID flag except for the sequence-completing write that sets it. Also refer to the description of the KEYVALID flag in the AES_SR register.
pub type KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<KEYR0rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - Cryptographic key, bits \[31:0\] These are bits \[31:0\] of the write-only bitfield KEY\[255:0\] AES encryption or decryption key, depending on the MODE\[1:0\] bitfield of the AES_CR register. Writes to AES_KEYRx registers are ignored when AES is enabled (EN bit set). A special writing sequence is required. In this sequence, any valid write to AES_KEYRx register clears the KEYVALID flag except for the sequence-completing write that sets it. Also refer to the description of the KEYVALID flag in the AES_SR register.
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W<'_, KEYR0rs> {
        KEY_W::new(self, 0)
    }
}
/**AES key register 0

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyr0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#AES:KEYR0)*/
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
