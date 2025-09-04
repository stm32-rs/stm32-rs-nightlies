///Register `SWREG259` reader
pub type R = crate::R<SWREG259rs>;
///Register `SWREG259` writer
pub type W = crate::W<SWREG259rs>;
///Field `SWREG_FIELD` reader - segment 1: intra 4x4 mode 5-7 penalty (all format mode)
pub type SWREG_FIELD_R = crate::FieldReader<u32>;
///Field `SWREG_FIELD` writer - segment 1: intra 4x4 mode 5-7 penalty (all format mode)
pub type SWREG_FIELD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - segment 1: intra 4x4 mode 5-7 penalty (all format mode)
    #[inline(always)]
    pub fn swreg_field(&self) -> SWREG_FIELD_R {
        SWREG_FIELD_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWREG259")
            .field("swreg_field", &self.swreg_field())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - segment 1: intra 4x4 mode 5-7 penalty (all format mode)
    #[inline(always)]
    pub fn swreg_field(&mut self) -> SWREG_FIELD_W<SWREG259rs> {
        SWREG_FIELD_W::new(self, 0)
    }
}
/**VENC segment 1: intra 4x4 mode 5-7 penalty register

You can [`read`](crate::Reg::read) this register and get [`swreg259::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg259::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#VENC:SWREG259)*/
pub struct SWREG259rs;
impl crate::RegisterSpec for SWREG259rs {
    type Ux = u32;
}
///`read()` method returns [`swreg259::R`](R) reader structure
impl crate::Readable for SWREG259rs {}
///`write(|w| ..)` method takes [`swreg259::W`](W) writer structure
impl crate::Writable for SWREG259rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWREG259 to value 0
impl crate::Resettable for SWREG259rs {}
