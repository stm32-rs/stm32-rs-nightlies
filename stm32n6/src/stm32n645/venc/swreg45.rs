///Register `SWREG45` reader
pub type R = crate::R<SWREG45rs>;
///Register `SWREG45` writer
pub type W = crate::W<SWREG45rs>;
///Field `SWREG_FIELD` reader - Stabilization matrix 4 (left position) output (all format mode)
pub type SWREG_FIELD_R = crate::FieldReader<u32>;
///Field `SWREG_FIELD` writer - Stabilization matrix 4 (left position) output (all format mode)
pub type SWREG_FIELD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Stabilization matrix 4 (left position) output (all format mode)
    #[inline(always)]
    pub fn swreg_field(&self) -> SWREG_FIELD_R {
        SWREG_FIELD_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWREG45")
            .field("swreg_field", &self.swreg_field())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Stabilization matrix 4 (left position) output (all format mode)
    #[inline(always)]
    pub fn swreg_field(&mut self) -> SWREG_FIELD_W<'_, SWREG45rs> {
        SWREG_FIELD_W::new(self, 0)
    }
}
/**VENC stabilization matrix 4, left position output register

You can [`read`](crate::Reg::read) this register and get [`swreg45::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg45::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#VENC:SWREG45)*/
pub struct SWREG45rs;
impl crate::RegisterSpec for SWREG45rs {
    type Ux = u32;
}
///`read()` method returns [`swreg45::R`](R) reader structure
impl crate::Readable for SWREG45rs {}
///`write(|w| ..)` method takes [`swreg45::W`](W) writer structure
impl crate::Writable for SWREG45rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWREG45 to value 0
impl crate::Resettable for SWREG45rs {}
