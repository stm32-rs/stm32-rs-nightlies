///Register `SWREG401` reader
pub type R = crate::R<SWREG401rs>;
///Register `SWREG401` writer
pub type W = crate::W<SWREG401rs>;
///Field `SWREG_FIELD` reader - segment 29: penalty value (all format mode)
pub type SWREG_FIELD_R = crate::FieldReader<u32>;
///Field `SWREG_FIELD` writer - segment 29: penalty value (all format mode)
pub type SWREG_FIELD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - segment 29: penalty value (all format mode)
    #[inline(always)]
    pub fn swreg_field(&self) -> SWREG_FIELD_R {
        SWREG_FIELD_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWREG401")
            .field("swreg_field", &self.swreg_field())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - segment 29: penalty value (all format mode)
    #[inline(always)]
    pub fn swreg_field(&mut self) -> SWREG_FIELD_W<'_, SWREG401rs> {
        SWREG_FIELD_W::new(self, 0)
    }
}
/**VENC segment 29: penalty value register

You can [`read`](crate::Reg::read) this register and get [`swreg401::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg401::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#VENC:SWREG401)*/
pub struct SWREG401rs;
impl crate::RegisterSpec for SWREG401rs {
    type Ux = u32;
}
///`read()` method returns [`swreg401::R`](R) reader structure
impl crate::Readable for SWREG401rs {}
///`write(|w| ..)` method takes [`swreg401::W`](W) writer structure
impl crate::Writable for SWREG401rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWREG401 to value 0
impl crate::Resettable for SWREG401rs {}
