///Register `AES_IVR2` reader
pub type R = crate::R<AES_IVR2rs>;
///Register `AES_IVR2` writer
pub type W = crate::W<AES_IVR2rs>;
/**Field `IVI` reader - Initialization vector input, bits \[95:64\]
Refer to the AES_IVR0 register for description of the IVI\[128:0\]
bitfield.*/
pub type IVI_R = crate::FieldReader<u32>;
/**Field `IVI` writer - Initialization vector input, bits \[95:64\]
Refer to the AES_IVR0 register for description of the IVI\[128:0\]
bitfield.*/
pub type IVI_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    /**Bits 0:31 - Initialization vector input, bits \[95:64\]
    Refer to the AES_IVR0 register for description of the IVI\[128:0\]
    bitfield.*/
    #[inline(always)]
    pub fn ivi(&self) -> IVI_R {
        IVI_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AES_IVR2")
            .field("ivi", &self.ivi())
            .finish()
    }
}
impl W {
    /**Bits 0:31 - Initialization vector input, bits \[95:64\]
    Refer to the AES_IVR0 register for description of the IVI\[128:0\]
    bitfield.*/
    #[inline(always)]
    pub fn ivi(&mut self) -> IVI_W<AES_IVR2rs> {
        IVI_W::new(self, 0)
    }
}
/**AES initialization vector register 2

You can [`read`](crate::Reg::read) this register and get [`aes_ivr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_ivr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G061.html#AES:AES_IVR2)*/
pub struct AES_IVR2rs;
impl crate::RegisterSpec for AES_IVR2rs {
    type Ux = u32;
}
///`read()` method returns [`aes_ivr2::R`](R) reader structure
impl crate::Readable for AES_IVR2rs {}
///`write(|w| ..)` method takes [`aes_ivr2::W`](W) writer structure
impl crate::Writable for AES_IVR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets AES_IVR2 to value 0
impl crate::Resettable for AES_IVR2rs {
    const RESET_VALUE: u32 = 0;
}
