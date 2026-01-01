///Register `SWREG48` reader
pub type R = crate::R<SWREG48rs>;
///Register `SWREG48` writer
pub type W = crate::W<SWREG48rs>;
///Field `SWREG_FIELD` reader - Stabilization matrix 7 (down-left position) output (all format mode)
pub type SWREG_FIELD_R = crate::FieldReader<u32>;
///Field `SWREG_FIELD` writer - Stabilization matrix 7 (down-left position) output (all format mode)
pub type SWREG_FIELD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Stabilization matrix 7 (down-left position) output (all format mode)
    #[inline(always)]
    pub fn swreg_field(&self) -> SWREG_FIELD_R {
        SWREG_FIELD_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWREG48")
            .field("swreg_field", &self.swreg_field())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Stabilization matrix 7 (down-left position) output (all format mode)
    #[inline(always)]
    pub fn swreg_field(&mut self) -> SWREG_FIELD_W<'_, SWREG48rs> {
        SWREG_FIELD_W::new(self, 0)
    }
}
/**VENC stabilization matrix 7, down-left position output register

You can [`read`](crate::Reg::read) this register and get [`swreg48::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg48::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG48)*/
pub struct SWREG48rs;
impl crate::RegisterSpec for SWREG48rs {
    type Ux = u32;
}
///`read()` method returns [`swreg48::R`](R) reader structure
impl crate::Readable for SWREG48rs {}
///`write(|w| ..)` method takes [`swreg48::W`](W) writer structure
impl crate::Writable for SWREG48rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWREG48 to value 0
impl crate::Resettable for SWREG48rs {}
