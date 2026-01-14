///Register `CNSLCKR` reader
pub type R = crate::R<CNSLCKRrs>;
///Register `CNSLCKR` writer
pub type W = crate::W<CNSLCKRrs>;
///Field `LOCKNSVTOR` reader - VTOR_NS register lock This bit is set by software and cleared only by a system reset.
pub type LOCKNSVTOR_R = crate::BitReader;
///Field `LOCKNSVTOR` writer - VTOR_NS register lock This bit is set by software and cleared only by a system reset.
pub type LOCKNSVTOR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LOCKNSMPU` reader - non-secure MPU register lock This bit is set by software and cleared only by a system reset. When set, this bit disables write access to non-secure MPU_CTRL_NS, MPU_RNR_NS and MPU_RBAR_NS registers.
pub type LOCKNSMPU_R = crate::BitReader;
///Field `LOCKNSMPU` writer - non-secure MPU register lock This bit is set by software and cleared only by a system reset. When set, this bit disables write access to non-secure MPU_CTRL_NS, MPU_RNR_NS and MPU_RBAR_NS registers.
pub type LOCKNSMPU_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - VTOR_NS register lock This bit is set by software and cleared only by a system reset.
    #[inline(always)]
    pub fn locknsvtor(&self) -> LOCKNSVTOR_R {
        LOCKNSVTOR_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - non-secure MPU register lock This bit is set by software and cleared only by a system reset. When set, this bit disables write access to non-secure MPU_CTRL_NS, MPU_RNR_NS and MPU_RBAR_NS registers.
    #[inline(always)]
    pub fn locknsmpu(&self) -> LOCKNSMPU_R {
        LOCKNSMPU_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CNSLCKR")
            .field("locknsvtor", &self.locknsvtor())
            .field("locknsmpu", &self.locknsmpu())
            .finish()
    }
}
impl W {
    ///Bit 0 - VTOR_NS register lock This bit is set by software and cleared only by a system reset.
    #[inline(always)]
    pub fn locknsvtor(&mut self) -> LOCKNSVTOR_W<'_, CNSLCKRrs> {
        LOCKNSVTOR_W::new(self, 0)
    }
    ///Bit 1 - non-secure MPU register lock This bit is set by software and cleared only by a system reset. When set, this bit disables write access to non-secure MPU_CTRL_NS, MPU_RNR_NS and MPU_RBAR_NS registers.
    #[inline(always)]
    pub fn locknsmpu(&mut self) -> LOCKNSMPU_W<'_, CNSLCKRrs> {
        LOCKNSMPU_W::new(self, 1)
    }
}
/**SBS CPU non-secure lock register

You can [`read`](crate::Reg::read) this register and get [`cnslckr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnslckr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#SBS:CNSLCKR)*/
pub struct CNSLCKRrs;
impl crate::RegisterSpec for CNSLCKRrs {
    type Ux = u32;
}
///`read()` method returns [`cnslckr::R`](R) reader structure
impl crate::Readable for CNSLCKRrs {}
///`write(|w| ..)` method takes [`cnslckr::W`](W) writer structure
impl crate::Writable for CNSLCKRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CNSLCKR to value 0
impl crate::Resettable for CNSLCKRrs {}
