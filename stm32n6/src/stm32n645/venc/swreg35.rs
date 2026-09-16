///Register `SWREG35` reader
pub type R = crate::R<SWREG35rs>;
///Register `SWREG35` writer
pub type W = crate::W<SWREG35rs>;
///Field `SWREG_FIELD` reader - H.264 Checkpoint word error 5-6 / Encoder control register 17 (all format mode)
pub type SWREG_FIELD_R = crate::FieldReader<u32>;
///Field `SWREG_FIELD` writer - H.264 Checkpoint word error 5-6 / Encoder control register 17 (all format mode)
pub type SWREG_FIELD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - H.264 Checkpoint word error 5-6 / Encoder control register 17 (all format mode)
    #[inline(always)]
    pub fn swreg_field(&self) -> SWREG_FIELD_R {
        SWREG_FIELD_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWREG35")
            .field("swreg_field", &self.swreg_field())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - H.264 Checkpoint word error 5-6 / Encoder control register 17 (all format mode)
    #[inline(always)]
    pub fn swreg_field(&mut self) -> SWREG_FIELD_W<'_, SWREG35rs> {
        SWREG_FIELD_W::new(self, 0)
    }
}
/**VENC H.264 checkpoint word error 5-6/encoder control register 17

You can [`read`](crate::Reg::read) this register and get [`swreg35::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg35::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#VENC:SWREG35)*/
pub struct SWREG35rs;
impl crate::RegisterSpec for SWREG35rs {
    type Ux = u32;
}
///`read()` method returns [`swreg35::R`](R) reader structure
impl crate::Readable for SWREG35rs {}
///`write(|w| ..)` method takes [`swreg35::W`](W) writer structure
impl crate::Writable for SWREG35rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWREG35 to value 0
impl crate::Resettable for SWREG35rs {}
