///Register `AES_KEYR2` writer
pub type W = crate::W<AES_KEYR2rs>;
/**Field `KEY` writer - Cryptographic key, bits \[95:64\]
Refer to the AES_KEYR0 register for description of the KEY\[255:0\]
bitfield and for information relative to writing AES_KEYRx registers.*/
pub type KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<AES_KEYR2rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    /**Bits 0:31 - Cryptographic key, bits \[95:64\]
    Refer to the AES_KEYR0 register for description of the KEY\[255:0\]
    bitfield and for information relative to writing AES_KEYRx registers.*/
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W<AES_KEYR2rs> {
        KEY_W::new(self, 0)
    }
}
/**AES key register 2

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_keyr2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#AES:AES_KEYR2)*/
pub struct AES_KEYR2rs;
impl crate::RegisterSpec for AES_KEYR2rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`aes_keyr2::W`](W) writer structure
impl crate::Writable for AES_KEYR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets AES_KEYR2 to value 0
impl crate::Resettable for AES_KEYR2rs {
    const RESET_VALUE: u32 = 0;
}
