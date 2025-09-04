///Register `IVR1` reader
pub type R = crate::R<IVR1rs>;
///Register `IVR1` writer
pub type W = crate::W<IVR1rs>;
///Field `IVR1` reader - Initialization vector register
pub type IVR1_R = crate::FieldReader<u32>;
///Field `IVR1` writer - Initialization vector register
pub type IVR1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Initialization vector register
    #[inline(always)]
    pub fn ivr1(&self) -> IVR1_R {
        IVR1_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IVR1").field("ivr1", &self.ivr1()).finish()
    }
}
impl W {
    ///Bits 0:31 - Initialization vector register
    #[inline(always)]
    pub fn ivr1(&mut self) -> IVR1_W<IVR1rs> {
        IVR1_W::new(self, 0)
    }
}
/**initialization vector register

You can [`read`](crate::Reg::read) this register and get [`ivr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ivr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F722.html#CRYP:IVR1)*/
pub struct IVR1rs;
impl crate::RegisterSpec for IVR1rs {
    type Ux = u32;
}
///`read()` method returns [`ivr1::R`](R) reader structure
impl crate::Readable for IVR1rs {}
///`write(|w| ..)` method takes [`ivr1::W`](W) writer structure
impl crate::Writable for IVR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IVR1 to value 0
impl crate::Resettable for IVR1rs {}
