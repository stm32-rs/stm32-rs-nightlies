///Register `SWREG411` reader
pub type R = crate::R<SWREG411rs>;
///Register `SWREG411` writer
pub type W = crate::W<SWREG411rs>;
///Field `SWREG_FIELD` reader - gain of MB QPdelta. 8.8 format (all format mode)
pub type SWREG_FIELD_R = crate::FieldReader<u32>;
///Field `SWREG_FIELD` writer - gain of MB QPdelta. 8.8 format (all format mode)
pub type SWREG_FIELD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - gain of MB QPdelta. 8.8 format (all format mode)
    #[inline(always)]
    pub fn swreg_field(&self) -> SWREG_FIELD_R {
        SWREG_FIELD_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWREG411")
            .field("swreg_field", &self.swreg_field())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - gain of MB QPdelta. 8.8 format (all format mode)
    #[inline(always)]
    pub fn swreg_field(&mut self) -> SWREG_FIELD_W<'_, SWREG411rs> {
        SWREG_FIELD_W::new(self, 0)
    }
}
/**VENC gain of MB QP delta. 8.8 format register

You can [`read`](crate::Reg::read) this register and get [`swreg411::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg411::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#VENC:SWREG411)*/
pub struct SWREG411rs;
impl crate::RegisterSpec for SWREG411rs {
    type Ux = u32;
}
///`read()` method returns [`swreg411::R`](R) reader structure
impl crate::Readable for SWREG411rs {}
///`write(|w| ..)` method takes [`swreg411::W`](W) writer structure
impl crate::Writable for SWREG411rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWREG411 to value 0
impl crate::Resettable for SWREG411rs {}
