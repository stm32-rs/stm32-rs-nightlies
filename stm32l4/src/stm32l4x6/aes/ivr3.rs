///Register `IVR3` reader
pub type R = crate::R<IVR3rs>;
///Register `IVR3` writer
pub type W = crate::W<IVR3rs>;
///Field `AES_IVR3` reader - Initialization Vector Register (MSB IVR \[127:96\])
pub type AES_IVR3_R = crate::FieldReader<u32>;
///Field `AES_IVR3` writer - Initialization Vector Register (MSB IVR \[127:96\])
pub type AES_IVR3_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Initialization Vector Register (MSB IVR \[127:96\])
    #[inline(always)]
    pub fn aes_ivr3(&self) -> AES_IVR3_R {
        AES_IVR3_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IVR3")
            .field("aes_ivr3", &self.aes_ivr3())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Initialization Vector Register (MSB IVR \[127:96\])
    #[inline(always)]
    #[must_use]
    pub fn aes_ivr3(&mut self) -> AES_IVR3_W<IVR3rs> {
        AES_IVR3_W::new(self, 0)
    }
}
/**initialization vector register 3

You can [`read`](crate::Reg::read) this register and get [`ivr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ivr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x6.html#AES:IVR3)*/
pub struct IVR3rs;
impl crate::RegisterSpec for IVR3rs {
    type Ux = u32;
}
///`read()` method returns [`ivr3::R`](R) reader structure
impl crate::Readable for IVR3rs {}
///`write(|w| ..)` method takes [`ivr3::W`](W) writer structure
impl crate::Writable for IVR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IVR3 to value 0
impl crate::Resettable for IVR3rs {
    const RESET_VALUE: u32 = 0;
}
