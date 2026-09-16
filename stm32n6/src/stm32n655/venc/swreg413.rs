///Register `SWREG413` reader
pub type R = crate::R<SWREG413rs>;
///Register `SWREG413` writer
pub type W = crate::W<SWREG413rs>;
///Field `SWREG_FIELD` reader - average of MB complexity (all format mode)
pub type SWREG_FIELD_R = crate::FieldReader<u32>;
///Field `SWREG_FIELD` writer - average of MB complexity (all format mode)
pub type SWREG_FIELD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - average of MB complexity (all format mode)
    #[inline(always)]
    pub fn swreg_field(&self) -> SWREG_FIELD_R {
        SWREG_FIELD_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWREG413")
            .field("swreg_field", &self.swreg_field())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - average of MB complexity (all format mode)
    #[inline(always)]
    pub fn swreg_field(&mut self) -> SWREG_FIELD_W<'_, SWREG413rs> {
        SWREG_FIELD_W::new(self, 0)
    }
}
/**VENC reference compression control register

You can [`read`](crate::Reg::read) this register and get [`swreg413::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg413::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG413)*/
pub struct SWREG413rs;
impl crate::RegisterSpec for SWREG413rs {
    type Ux = u32;
}
///`read()` method returns [`swreg413::R`](R) reader structure
impl crate::Readable for SWREG413rs {}
///`write(|w| ..)` method takes [`swreg413::W`](W) writer structure
impl crate::Writable for SWREG413rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWREG413 to value 0
impl crate::Resettable for SWREG413rs {}
