///Register `AES_KEYR1` reader
pub type R = crate::R<AES_KEYR1rs>;
///Register `AES_KEYR1` writer
pub type W = crate::W<AES_KEYR1rs>;
/**Field `KEY` reader - Cryptographic key, bits \[63:32\]
Refer to the AES_KEYR0 register for description of the KEY\[255:0\]
bitfield.*/
pub type KEY_R = crate::FieldReader<u32>;
/**Field `KEY` writer - Cryptographic key, bits \[63:32\]
Refer to the AES_KEYR0 register for description of the KEY\[255:0\]
bitfield.*/
pub type KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    /**Bits 0:31 - Cryptographic key, bits \[63:32\]
    Refer to the AES_KEYR0 register for description of the KEY\[255:0\]
    bitfield.*/
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AES_KEYR1")
            .field("key", &self.key())
            .finish()
    }
}
impl W {
    /**Bits 0:31 - Cryptographic key, bits \[63:32\]
    Refer to the AES_KEYR0 register for description of the KEY\[255:0\]
    bitfield.*/
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W<AES_KEYR1rs> {
        KEY_W::new(self, 0)
    }
}
/**AES key register 1

You can [`read`](crate::Reg::read) this register and get [`aes_keyr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_keyr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G061.html#AES:AES_KEYR1)*/
pub struct AES_KEYR1rs;
impl crate::RegisterSpec for AES_KEYR1rs {
    type Ux = u32;
}
///`read()` method returns [`aes_keyr1::R`](R) reader structure
impl crate::Readable for AES_KEYR1rs {}
///`write(|w| ..)` method takes [`aes_keyr1::W`](W) writer structure
impl crate::Writable for AES_KEYR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets AES_KEYR1 to value 0
impl crate::Resettable for AES_KEYR1rs {
    const RESET_VALUE: u32 = 0;
}
