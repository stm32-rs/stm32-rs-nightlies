///Register `SWREG14` reader
pub type R = crate::R<SWREG14rs>;
///Register `SWREG14` writer
pub type W = crate::W<SWREG14rs>;
///Field `SWREG_FIELD` reader - Encoder control register 0 (such as picture information or encoding mode) (all format mode)
pub type SWREG_FIELD_R = crate::FieldReader<u32>;
///Field `SWREG_FIELD` writer - Encoder control register 0 (such as picture information or encoding mode) (all format mode)
pub type SWREG_FIELD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Encoder control register 0 (such as picture information or encoding mode) (all format mode)
    #[inline(always)]
    pub fn swreg_field(&self) -> SWREG_FIELD_R {
        SWREG_FIELD_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWREG14")
            .field("swreg_field", &self.swreg_field())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Encoder control register 0 (such as picture information or encoding mode) (all format mode)
    #[inline(always)]
    pub fn swreg_field(&mut self) -> SWREG_FIELD_W<'_, SWREG14rs> {
        SWREG_FIELD_W::new(self, 0)
    }
}
/**VENC encoder control register 0

You can [`read`](crate::Reg::read) this register and get [`swreg14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#VENC:SWREG14)*/
pub struct SWREG14rs;
impl crate::RegisterSpec for SWREG14rs {
    type Ux = u32;
}
///`read()` method returns [`swreg14::R`](R) reader structure
impl crate::Readable for SWREG14rs {}
///`write(|w| ..)` method takes [`swreg14::W`](W) writer structure
impl crate::Writable for SWREG14rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWREG14 to value 0
impl crate::Resettable for SWREG14rs {}
