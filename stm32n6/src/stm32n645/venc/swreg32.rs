///Register `SWREG32` reader
pub type R = crate::R<SWREG32rs>;
///Register `SWREG32` writer
pub type W = crate::W<SWREG32rs>;
///Field `SWREG_FIELD` reader - H.264 Checkpoint 8 -10 / Encoder control register 14 (all format mode)
pub type SWREG_FIELD_R = crate::FieldReader<u32>;
///Field `SWREG_FIELD` writer - H.264 Checkpoint 8 -10 / Encoder control register 14 (all format mode)
pub type SWREG_FIELD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - H.264 Checkpoint 8 -10 / Encoder control register 14 (all format mode)
    #[inline(always)]
    pub fn swreg_field(&self) -> SWREG_FIELD_R {
        SWREG_FIELD_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWREG32")
            .field("swreg_field", &self.swreg_field())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - H.264 Checkpoint 8 -10 / Encoder control register 14 (all format mode)
    #[inline(always)]
    pub fn swreg_field(&mut self) -> SWREG_FIELD_W<'_, SWREG32rs> {
        SWREG_FIELD_W::new(self, 0)
    }
}
/**VENC encoder control register 14

You can [`read`](crate::Reg::read) this register and get [`swreg32::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg32::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#VENC:SWREG32)*/
pub struct SWREG32rs;
impl crate::RegisterSpec for SWREG32rs {
    type Ux = u32;
}
///`read()` method returns [`swreg32::R`](R) reader structure
impl crate::Readable for SWREG32rs {}
///`write(|w| ..)` method takes [`swreg32::W`](W) writer structure
impl crate::Writable for SWREG32rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWREG32 to value 0
impl crate::Resettable for SWREG32rs {}
