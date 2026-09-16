///Register `SWREG61` reader
pub type R = crate::R<SWREG61rs>;
///Register `SWREG61` writer
pub type W = crate::W<SWREG61rs>;
///Field `SWREG_FIELD` reader - 2nd ROI area (all format mode)
pub type SWREG_FIELD_R = crate::FieldReader<u32>;
///Field `SWREG_FIELD` writer - 2nd ROI area (all format mode)
pub type SWREG_FIELD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - 2nd ROI area (all format mode)
    #[inline(always)]
    pub fn swreg_field(&self) -> SWREG_FIELD_R {
        SWREG_FIELD_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWREG61")
            .field("swreg_field", &self.swreg_field())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - 2nd ROI area (all format mode)
    #[inline(always)]
    pub fn swreg_field(&mut self) -> SWREG_FIELD_W<'_, SWREG61rs> {
        SWREG_FIELD_W::new(self, 0)
    }
}
/**VENC 2nd ROI area register

You can [`read`](crate::Reg::read) this register and get [`swreg61::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg61::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#VENC:SWREG61)*/
pub struct SWREG61rs;
impl crate::RegisterSpec for SWREG61rs {
    type Ux = u32;
}
///`read()` method returns [`swreg61::R`](R) reader structure
impl crate::Readable for SWREG61rs {}
///`write(|w| ..)` method takes [`swreg61::W`](W) writer structure
impl crate::Writable for SWREG61rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWREG61 to value 0
impl crate::Resettable for SWREG61rs {}
