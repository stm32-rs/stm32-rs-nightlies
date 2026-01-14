///Register `SWREG49` reader
pub type R = crate::R<SWREG49rs>;
///Register `SWREG49` writer
pub type W = crate::W<SWREG49rs>;
///Field `SWREG_FIELD` reader - Stabilization matrix 8 (down position) output (all format mode)
pub type SWREG_FIELD_R = crate::FieldReader<u32>;
///Field `SWREG_FIELD` writer - Stabilization matrix 8 (down position) output (all format mode)
pub type SWREG_FIELD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Stabilization matrix 8 (down position) output (all format mode)
    #[inline(always)]
    pub fn swreg_field(&self) -> SWREG_FIELD_R {
        SWREG_FIELD_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWREG49")
            .field("swreg_field", &self.swreg_field())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Stabilization matrix 8 (down position) output (all format mode)
    #[inline(always)]
    pub fn swreg_field(&mut self) -> SWREG_FIELD_W<'_, SWREG49rs> {
        SWREG_FIELD_W::new(self, 0)
    }
}
/**VENC stabilization matrix 8, down position output register

You can [`read`](crate::Reg::read) this register and get [`swreg49::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg49::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#VENC:SWREG49)*/
pub struct SWREG49rs;
impl crate::RegisterSpec for SWREG49rs {
    type Ux = u32;
}
///`read()` method returns [`swreg49::R`](R) reader structure
impl crate::Readable for SWREG49rs {}
///`write(|w| ..)` method takes [`swreg49::W`](W) writer structure
impl crate::Writable for SWREG49rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWREG49 to value 0
impl crate::Resettable for SWREG49rs {}
