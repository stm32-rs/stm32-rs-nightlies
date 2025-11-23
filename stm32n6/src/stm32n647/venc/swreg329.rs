///Register `SWREG329` reader
pub type R = crate::R<SWREG329rs>;
///Register `SWREG329` writer
pub type W = crate::W<SWREG329rs>;
///Field `SWREG_FIELD` reader - segment 11: penalty value (all format mode)
pub type SWREG_FIELD_R = crate::FieldReader<u32>;
///Field `SWREG_FIELD` writer - segment 11: penalty value (all format mode)
pub type SWREG_FIELD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - segment 11: penalty value (all format mode)
    #[inline(always)]
    pub fn swreg_field(&self) -> SWREG_FIELD_R {
        SWREG_FIELD_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWREG329")
            .field("swreg_field", &self.swreg_field())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - segment 11: penalty value (all format mode)
    #[inline(always)]
    pub fn swreg_field(&mut self) -> SWREG_FIELD_W<'_, SWREG329rs> {
        SWREG_FIELD_W::new(self, 0)
    }
}
/**VENC segment 11: penalty value register

You can [`read`](crate::Reg::read) this register and get [`swreg329::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg329::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#VENC:SWREG329)*/
pub struct SWREG329rs;
impl crate::RegisterSpec for SWREG329rs {
    type Ux = u32;
}
///`read()` method returns [`swreg329::R`](R) reader structure
impl crate::Readable for SWREG329rs {}
///`write(|w| ..)` method takes [`swreg329::W`](W) writer structure
impl crate::Writable for SWREG329rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWREG329 to value 0
impl crate::Resettable for SWREG329rs {}
