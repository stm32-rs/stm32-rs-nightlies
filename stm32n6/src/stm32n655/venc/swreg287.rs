///Register `SWREG287` reader
pub type R = crate::R<SWREG287rs>;
///Register `SWREG287` writer
pub type W = crate::W<SWREG287rs>;
///Field `SWREG_FIELD` reader - segment 3: penalty value (all format mode)
pub type SWREG_FIELD_R = crate::FieldReader<u32>;
///Field `SWREG_FIELD` writer - segment 3: penalty value (all format mode)
pub type SWREG_FIELD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - segment 3: penalty value (all format mode)
    #[inline(always)]
    pub fn swreg_field(&self) -> SWREG_FIELD_R {
        SWREG_FIELD_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWREG287")
            .field("swreg_field", &self.swreg_field())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - segment 3: penalty value (all format mode)
    #[inline(always)]
    pub fn swreg_field(&mut self) -> SWREG_FIELD_W<'_, SWREG287rs> {
        SWREG_FIELD_W::new(self, 0)
    }
}
/**VENC segment 3: penalty value register

You can [`read`](crate::Reg::read) this register and get [`swreg287::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg287::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG287)*/
pub struct SWREG287rs;
impl crate::RegisterSpec for SWREG287rs {
    type Ux = u32;
}
///`read()` method returns [`swreg287::R`](R) reader structure
impl crate::Readable for SWREG287rs {}
///`write(|w| ..)` method takes [`swreg287::W`](W) writer structure
impl crate::Writable for SWREG287rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWREG287 to value 0
impl crate::Resettable for SWREG287rs {}
