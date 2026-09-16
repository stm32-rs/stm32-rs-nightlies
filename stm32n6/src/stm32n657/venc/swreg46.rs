///Register `SWREG46` reader
pub type R = crate::R<SWREG46rs>;
///Register `SWREG46` writer
pub type W = crate::W<SWREG46rs>;
///Field `SWREG_FIELD` reader - Stabilization matrix 5 (GMV position) output (all format mode)
pub type SWREG_FIELD_R = crate::FieldReader<u32>;
///Field `SWREG_FIELD` writer - Stabilization matrix 5 (GMV position) output (all format mode)
pub type SWREG_FIELD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Stabilization matrix 5 (GMV position) output (all format mode)
    #[inline(always)]
    pub fn swreg_field(&self) -> SWREG_FIELD_R {
        SWREG_FIELD_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWREG46")
            .field("swreg_field", &self.swreg_field())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Stabilization matrix 5 (GMV position) output (all format mode)
    #[inline(always)]
    pub fn swreg_field(&mut self) -> SWREG_FIELD_W<'_, SWREG46rs> {
        SWREG_FIELD_W::new(self, 0)
    }
}
/**VENC stabilization matrix 5, GMV position output register

You can [`read`](crate::Reg::read) this register and get [`swreg46::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg46::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#VENC:SWREG46)*/
pub struct SWREG46rs;
impl crate::RegisterSpec for SWREG46rs {
    type Ux = u32;
}
///`read()` method returns [`swreg46::R`](R) reader structure
impl crate::Readable for SWREG46rs {}
///`write(|w| ..)` method takes [`swreg46::W`](W) writer structure
impl crate::Writable for SWREG46rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWREG46 to value 0
impl crate::Resettable for SWREG46rs {}
