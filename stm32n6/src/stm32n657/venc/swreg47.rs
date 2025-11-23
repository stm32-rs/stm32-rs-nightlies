///Register `SWREG47` reader
pub type R = crate::R<SWREG47rs>;
///Register `SWREG47` writer
pub type W = crate::W<SWREG47rs>;
///Field `SWREG_FIELD` reader - Stabilization matrix 6 (right position) output (all format mode)
pub type SWREG_FIELD_R = crate::FieldReader<u32>;
///Field `SWREG_FIELD` writer - Stabilization matrix 6 (right position) output (all format mode)
pub type SWREG_FIELD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Stabilization matrix 6 (right position) output (all format mode)
    #[inline(always)]
    pub fn swreg_field(&self) -> SWREG_FIELD_R {
        SWREG_FIELD_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWREG47")
            .field("swreg_field", &self.swreg_field())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Stabilization matrix 6 (right position) output (all format mode)
    #[inline(always)]
    pub fn swreg_field(&mut self) -> SWREG_FIELD_W<'_, SWREG47rs> {
        SWREG_FIELD_W::new(self, 0)
    }
}
/**VENC stabilization matrix 6, right position output register

You can [`read`](crate::Reg::read) this register and get [`swreg47::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg47::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#VENC:SWREG47)*/
pub struct SWREG47rs;
impl crate::RegisterSpec for SWREG47rs {
    type Ux = u32;
}
///`read()` method returns [`swreg47::R`](R) reader structure
impl crate::Readable for SWREG47rs {}
///`write(|w| ..)` method takes [`swreg47::W`](W) writer structure
impl crate::Writable for SWREG47rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWREG47 to value 0
impl crate::Resettable for SWREG47rs {}
