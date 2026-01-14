///Register `FVR%s` reader
pub type R = crate::R<FVRrs>;
///Register `FVR%s` writer
pub type W = crate::W<FVRrs>;
///Field `FV` reader - fuse value
pub type FV_R = crate::FieldReader<u32>;
///Field `FV` writer - fuse value
pub type FV_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - fuse value
    #[inline(always)]
    pub fn fv(&self) -> FV_R {
        FV_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FVR").field("fv", &self.fv()).finish()
    }
}
impl W {
    ///Bits 0:31 - fuse value
    #[inline(always)]
    pub fn fv(&mut self) -> FV_W<'_, FVRrs> {
        FV_W::new(self, 0)
    }
}
/**BSEC fuse word %s value register

You can [`read`](crate::Reg::read) this register and get [`fvr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fvr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#BSEC:FVR[0])*/
pub struct FVRrs;
impl crate::RegisterSpec for FVRrs {
    type Ux = u32;
}
///`read()` method returns [`fvr::R`](R) reader structure
impl crate::Readable for FVRrs {}
///`write(|w| ..)` method takes [`fvr::W`](W) writer structure
impl crate::Writable for FVRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FVR%s to value 0
impl crate::Resettable for FVRrs {}
