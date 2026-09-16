///Register `IVR2` reader
pub type R = crate::R<IVR2rs>;
///Register `IVR2` writer
pub type W = crate::W<IVR2rs>;
///Field `IVR2` reader - Initialization vector register
pub type IVR2_R = crate::FieldReader<u32>;
///Field `IVR2` writer - Initialization vector register
pub type IVR2_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Initialization vector register
    #[inline(always)]
    pub fn ivr2(&self) -> IVR2_R {
        IVR2_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IVR2").field("ivr2", &self.ivr2()).finish()
    }
}
impl W {
    ///Bits 0:31 - Initialization vector register
    #[inline(always)]
    pub fn ivr2(&mut self) -> IVR2_W<'_, IVR2rs> {
        IVR2_W::new(self, 0)
    }
}
/**initialization vector register

You can [`read`](crate::Reg::read) this register and get [`ivr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ivr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F733.html#CRYP:IVR2)*/
pub struct IVR2rs;
impl crate::RegisterSpec for IVR2rs {
    type Ux = u32;
}
///`read()` method returns [`ivr2::R`](R) reader structure
impl crate::Readable for IVR2rs {}
///`write(|w| ..)` method takes [`ivr2::W`](W) writer structure
impl crate::Writable for IVR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IVR2 to value 0
impl crate::Resettable for IVR2rs {}
