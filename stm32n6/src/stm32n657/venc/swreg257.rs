///Register `SWREG257` reader
pub type R = crate::R<SWREG257rs>;
///Register `SWREG257` writer
pub type W = crate::W<SWREG257rs>;
///Field `SWREG_FIELD` reader - segment 1: intra 16x16 mode 3 and intra 4x4 0-1 penalty (all format mode)
pub type SWREG_FIELD_R = crate::FieldReader<u32>;
///Field `SWREG_FIELD` writer - segment 1: intra 16x16 mode 3 and intra 4x4 0-1 penalty (all format mode)
pub type SWREG_FIELD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - segment 1: intra 16x16 mode 3 and intra 4x4 0-1 penalty (all format mode)
    #[inline(always)]
    pub fn swreg_field(&self) -> SWREG_FIELD_R {
        SWREG_FIELD_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWREG257")
            .field("swreg_field", &self.swreg_field())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - segment 1: intra 16x16 mode 3 and intra 4x4 0-1 penalty (all format mode)
    #[inline(always)]
    pub fn swreg_field(&mut self) -> SWREG_FIELD_W<'_, SWREG257rs> {
        SWREG_FIELD_W::new(self, 0)
    }
}
/**VENC segment 1: intra 16x16 mode 3, intra 4x4 0-1 penalty register

You can [`read`](crate::Reg::read) this register and get [`swreg257::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg257::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#VENC:SWREG257)*/
pub struct SWREG257rs;
impl crate::RegisterSpec for SWREG257rs {
    type Ux = u32;
}
///`read()` method returns [`swreg257::R`](R) reader structure
impl crate::Readable for SWREG257rs {}
///`write(|w| ..)` method takes [`swreg257::W`](W) writer structure
impl crate::Writable for SWREG257rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWREG257 to value 0
impl crate::Resettable for SWREG257rs {}
