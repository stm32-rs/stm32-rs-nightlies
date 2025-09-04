///Register `SWREG297` reader
pub type R = crate::R<SWREG297rs>;
///Register `SWREG297` writer
pub type W = crate::W<SWREG297rs>;
///Field `SWREG_FIELD` reader - MBRC control (all format mode)
pub type SWREG_FIELD_R = crate::FieldReader<u32>;
///Field `SWREG_FIELD` writer - MBRC control (all format mode)
pub type SWREG_FIELD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - MBRC control (all format mode)
    #[inline(always)]
    pub fn swreg_field(&self) -> SWREG_FIELD_R {
        SWREG_FIELD_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWREG297")
            .field("swreg_field", &self.swreg_field())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - MBRC control (all format mode)
    #[inline(always)]
    pub fn swreg_field(&mut self) -> SWREG_FIELD_W<SWREG297rs> {
        SWREG_FIELD_W::new(self, 0)
    }
}
/**VENC MBRC control register

You can [`read`](crate::Reg::read) this register and get [`swreg297::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg297::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG297)*/
pub struct SWREG297rs;
impl crate::RegisterSpec for SWREG297rs {
    type Ux = u32;
}
///`read()` method returns [`swreg297::R`](R) reader structure
impl crate::Readable for SWREG297rs {}
///`write(|w| ..)` method takes [`swreg297::W`](W) writer structure
impl crate::Writable for SWREG297rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWREG297 to value 0
impl crate::Resettable for SWREG297rs {}
