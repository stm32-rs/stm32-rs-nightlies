///Register `SWREG361` reader
pub type R = crate::R<SWREG361rs>;
///Register `SWREG361` writer
pub type W = crate::W<SWREG361rs>;
///Field `SWREG_FIELD` reader - segment 19: penalty value (all format mode)
pub type SWREG_FIELD_R = crate::FieldReader<u32>;
///Field `SWREG_FIELD` writer - segment 19: penalty value (all format mode)
pub type SWREG_FIELD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - segment 19: penalty value (all format mode)
    #[inline(always)]
    pub fn swreg_field(&self) -> SWREG_FIELD_R {
        SWREG_FIELD_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWREG361")
            .field("swreg_field", &self.swreg_field())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - segment 19: penalty value (all format mode)
    #[inline(always)]
    pub fn swreg_field(&mut self) -> SWREG_FIELD_W<'_, SWREG361rs> {
        SWREG_FIELD_W::new(self, 0)
    }
}
/**VENC segment 19: penalty value register

You can [`read`](crate::Reg::read) this register and get [`swreg361::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg361::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG361)*/
pub struct SWREG361rs;
impl crate::RegisterSpec for SWREG361rs {
    type Ux = u32;
}
///`read()` method returns [`swreg361::R`](R) reader structure
impl crate::Readable for SWREG361rs {}
///`write(|w| ..)` method takes [`swreg361::W`](W) writer structure
impl crate::Writable for SWREG361rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWREG361 to value 0
impl crate::Resettable for SWREG361rs {}
