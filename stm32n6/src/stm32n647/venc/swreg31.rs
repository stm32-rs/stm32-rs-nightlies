///Register `SWREG31` reader
pub type R = crate::R<SWREG31rs>;
///Register `SWREG31` writer
pub type W = crate::W<SWREG31rs>;
///Field `SWREG_FIELD` reader - H.264 checkpoint 7 -8 (all format mode)
pub type SWREG_FIELD_R = crate::FieldReader<u32>;
///Field `SWREG_FIELD` writer - H.264 checkpoint 7 -8 (all format mode)
pub type SWREG_FIELD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - H.264 checkpoint 7 -8 (all format mode)
    #[inline(always)]
    pub fn swreg_field(&self) -> SWREG_FIELD_R {
        SWREG_FIELD_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWREG31")
            .field("swreg_field", &self.swreg_field())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - H.264 checkpoint 7 -8 (all format mode)
    #[inline(always)]
    pub fn swreg_field(&mut self) -> SWREG_FIELD_W<'_, SWREG31rs> {
        SWREG_FIELD_W::new(self, 0)
    }
}
/**VENC encoder control register 13

You can [`read`](crate::Reg::read) this register and get [`swreg31::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg31::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#VENC:SWREG31)*/
pub struct SWREG31rs;
impl crate::RegisterSpec for SWREG31rs {
    type Ux = u32;
}
///`read()` method returns [`swreg31::R`](R) reader structure
impl crate::Readable for SWREG31rs {}
///`write(|w| ..)` method takes [`swreg31::W`](W) writer structure
impl crate::Writable for SWREG31rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWREG31 to value 0
impl crate::Resettable for SWREG31rs {}
