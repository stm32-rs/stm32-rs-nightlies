///Register `IVR1` reader
pub type R = crate::R<IVR1rs>;
///Register `IVR1` writer
pub type W = crate::W<IVR1rs>;
///Field `IV1` reader - Initialization Vector Register (IVR \[63:32\])
pub type IV1_R = crate::FieldReader<u32>;
///Field `IV1` writer - Initialization Vector Register (IVR \[63:32\])
pub type IV1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32, crate::Safe>;
impl R {
    ///Bits 0:31 - Initialization Vector Register (IVR \[63:32\])
    #[inline(always)]
    pub fn iv1(&self) -> IV1_R {
        IV1_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IVR1").field("iv1", &self.iv1()).finish()
    }
}
impl W {
    ///Bits 0:31 - Initialization Vector Register (IVR \[63:32\])
    #[inline(always)]
    pub fn iv1(&mut self) -> IV1_W<IVR1rs> {
        IV1_W::new(self, 0)
    }
}
/**initialization vector register 1

You can [`read`](crate::Reg::read) this register and get [`ivr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ivr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x2.html#AES:IVR1)*/
pub struct IVR1rs;
impl crate::RegisterSpec for IVR1rs {
    type Ux = u32;
}
///`read()` method returns [`ivr1::R`](R) reader structure
impl crate::Readable for IVR1rs {}
///`write(|w| ..)` method takes [`ivr1::W`](W) writer structure
impl crate::Writable for IVR1rs {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IVR1 to value 0
impl crate::Resettable for IVR1rs {
    const RESET_VALUE: u32 = 0;
}
