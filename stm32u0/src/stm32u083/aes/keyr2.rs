///Register `KEYR2` writer
pub type W = crate::W<KEYR2rs>;
///Field `KEY` writer - Cryptographic key, bits \[95:64\] Refer to the AES_KEYR0 register for description of the KEY\[255:0\] bitfield and for information relative to writing AES_KEYRx registers.
pub type KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<KEYR2rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - Cryptographic key, bits \[95:64\] Refer to the AES_KEYR0 register for description of the KEY\[255:0\] bitfield and for information relative to writing AES_KEYRx registers.
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W<'_, KEYR2rs> {
        KEY_W::new(self, 0)
    }
}
/**AES key register 2

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyr2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#AES:KEYR2)*/
pub struct KEYR2rs;
impl crate::RegisterSpec for KEYR2rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`keyr2::W`](W) writer structure
impl crate::Writable for KEYR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets KEYR2 to value 0
impl crate::Resettable for KEYR2rs {}
