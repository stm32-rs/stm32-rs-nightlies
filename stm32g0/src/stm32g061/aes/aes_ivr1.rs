///Register `AES_IVR1` reader
pub type R = crate::R<AES_IVR1rs>;
///Register `AES_IVR1` writer
pub type W = crate::W<AES_IVR1rs>;
/**Field `IVI` reader - Initialization vector input, bits \[63:32\]
Refer to the AES_IVR0 register for description of the IVI\[128:0\]
bitfield.*/
pub type IVI_R = crate::FieldReader<u32>;
/**Field `IVI` writer - Initialization vector input, bits \[63:32\]
Refer to the AES_IVR0 register for description of the IVI\[128:0\]
bitfield.*/
pub type IVI_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    /**Bits 0:31 - Initialization vector input, bits \[63:32\]
    Refer to the AES_IVR0 register for description of the IVI\[128:0\]
    bitfield.*/
    #[inline(always)]
    pub fn ivi(&self) -> IVI_R {
        IVI_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AES_IVR1")
            .field("ivi", &self.ivi())
            .finish()
    }
}
impl W {
    /**Bits 0:31 - Initialization vector input, bits \[63:32\]
    Refer to the AES_IVR0 register for description of the IVI\[128:0\]
    bitfield.*/
    #[inline(always)]
    pub fn ivi(&mut self) -> IVI_W<AES_IVR1rs> {
        IVI_W::new(self, 0)
    }
}
/**AES initialization vector register 1

You can [`read`](crate::Reg::read) this register and get [`aes_ivr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_ivr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G061.html#AES:AES_IVR1)*/
pub struct AES_IVR1rs;
impl crate::RegisterSpec for AES_IVR1rs {
    type Ux = u32;
}
///`read()` method returns [`aes_ivr1::R`](R) reader structure
impl crate::Readable for AES_IVR1rs {}
///`write(|w| ..)` method takes [`aes_ivr1::W`](W) writer structure
impl crate::Writable for AES_IVR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets AES_IVR1 to value 0
impl crate::Resettable for AES_IVR1rs {
    const RESET_VALUE: u32 = 0;
}
