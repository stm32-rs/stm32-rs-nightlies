///Register `SWREG62` reader
pub type R = crate::R<SWREG62rs>;
///Register `SWREG62` writer
pub type W = crate::W<SWREG62rs>;
///Field `SWREG_FIELD` reader - ROI area delta QP, MV (all format mode)
pub type SWREG_FIELD_R = crate::FieldReader<u32>;
///Field `SWREG_FIELD` writer - ROI area delta QP, MV (all format mode)
pub type SWREG_FIELD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - ROI area delta QP, MV (all format mode)
    #[inline(always)]
    pub fn swreg_field(&self) -> SWREG_FIELD_R {
        SWREG_FIELD_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWREG62")
            .field("swreg_field", &self.swreg_field())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - ROI area delta QP, MV (all format mode)
    #[inline(always)]
    pub fn swreg_field(&mut self) -> SWREG_FIELD_W<'_, SWREG62rs> {
        SWREG_FIELD_W::new(self, 0)
    }
}
/**VENC ROI area delta QP, MV register

You can [`read`](crate::Reg::read) this register and get [`swreg62::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg62::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#VENC:SWREG62)*/
pub struct SWREG62rs;
impl crate::RegisterSpec for SWREG62rs {
    type Ux = u32;
}
///`read()` method returns [`swreg62::R`](R) reader structure
impl crate::Readable for SWREG62rs {}
///`write(|w| ..)` method takes [`swreg62::W`](W) writer structure
impl crate::Writable for SWREG62rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWREG62 to value 0
impl crate::Resettable for SWREG62rs {}
