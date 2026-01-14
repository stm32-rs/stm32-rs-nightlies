///Register `SWREG277` reader
pub type R = crate::R<SWREG277rs>;
///Register `SWREG277` writer
pub type W = crate::W<SWREG277rs>;
///Field `SWREG_FIELD` reader - segment 2: Deadzone rate multiplier for plane 0-1 (all format mode)
pub type SWREG_FIELD_R = crate::FieldReader<u32>;
///Field `SWREG_FIELD` writer - segment 2: Deadzone rate multiplier for plane 0-1 (all format mode)
pub type SWREG_FIELD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - segment 2: Deadzone rate multiplier for plane 0-1 (all format mode)
    #[inline(always)]
    pub fn swreg_field(&self) -> SWREG_FIELD_R {
        SWREG_FIELD_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWREG277")
            .field("swreg_field", &self.swreg_field())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - segment 2: Deadzone rate multiplier for plane 0-1 (all format mode)
    #[inline(always)]
    pub fn swreg_field(&mut self) -> SWREG_FIELD_W<'_, SWREG277rs> {
        SWREG_FIELD_W::new(self, 0)
    }
}
/**VENC segment 2: deadzone rate multiplier for plane 0-1 register

You can [`read`](crate::Reg::read) this register and get [`swreg277::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg277::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#VENC:SWREG277)*/
pub struct SWREG277rs;
impl crate::RegisterSpec for SWREG277rs {
    type Ux = u32;
}
///`read()` method returns [`swreg277::R`](R) reader structure
impl crate::Readable for SWREG277rs {}
///`write(|w| ..)` method takes [`swreg277::W`](W) writer structure
impl crate::Writable for SWREG277rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWREG277 to value 0
impl crate::Resettable for SWREG277rs {}
