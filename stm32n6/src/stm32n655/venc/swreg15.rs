///Register `SWREG15` reader
pub type R = crate::R<SWREG15rs>;
///Register `SWREG15` writer
pub type W = crate::W<SWREG15rs>;
///Field `SWREG_FIELD` reader - Encoder control register 1 (such as preprocessing control, crop, rotate, input format) (all format mode)
pub type SWREG_FIELD_R = crate::FieldReader<u32>;
///Field `SWREG_FIELD` writer - Encoder control register 1 (such as preprocessing control, crop, rotate, input format) (all format mode)
pub type SWREG_FIELD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Encoder control register 1 (such as preprocessing control, crop, rotate, input format) (all format mode)
    #[inline(always)]
    pub fn swreg_field(&self) -> SWREG_FIELD_R {
        SWREG_FIELD_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWREG15")
            .field("swreg_field", &self.swreg_field())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Encoder control register 1 (such as preprocessing control, crop, rotate, input format) (all format mode)
    #[inline(always)]
    pub fn swreg_field(&mut self) -> SWREG_FIELD_W<'_, SWREG15rs> {
        SWREG_FIELD_W::new(self, 0)
    }
}
/**VENC encoder control register 1

You can [`read`](crate::Reg::read) this register and get [`swreg15::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg15::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG15)*/
pub struct SWREG15rs;
impl crate::RegisterSpec for SWREG15rs {
    type Ux = u32;
}
///`read()` method returns [`swreg15::R`](R) reader structure
impl crate::Readable for SWREG15rs {}
///`write(|w| ..)` method takes [`swreg15::W`](W) writer structure
impl crate::Writable for SWREG15rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWREG15 to value 0
impl crate::Resettable for SWREG15rs {}
