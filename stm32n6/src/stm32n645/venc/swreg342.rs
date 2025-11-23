///Register `SWREG342` reader
pub type R = crate::R<SWREG342rs>;
///Register `SWREG342` writer
pub type W = crate::W<SWREG342rs>;
///Field `SWREG_FIELD` reader - segment 15: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame (all format mode)
pub type SWREG_FIELD_R = crate::FieldReader<u32>;
///Field `SWREG_FIELD` writer - segment 15: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame (all format mode)
pub type SWREG_FIELD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - segment 15: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame (all format mode)
    #[inline(always)]
    pub fn swreg_field(&self) -> SWREG_FIELD_R {
        SWREG_FIELD_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWREG342")
            .field("swreg_field", &self.swreg_field())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - segment 15: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame (all format mode)
    #[inline(always)]
    pub fn swreg_field(&mut self) -> SWREG_FIELD_W<'_, SWREG342rs> {
        SWREG_FIELD_W::new(self, 0)
    }
}
/**VENC segment 15: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame register

You can [`read`](crate::Reg::read) this register and get [`swreg342::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg342::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#VENC:SWREG342)*/
pub struct SWREG342rs;
impl crate::RegisterSpec for SWREG342rs {
    type Ux = u32;
}
///`read()` method returns [`swreg342::R`](R) reader structure
impl crate::Readable for SWREG342rs {}
///`write(|w| ..)` method takes [`swreg342::W`](W) writer structure
impl crate::Writable for SWREG342rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWREG342 to value 0
impl crate::Resettable for SWREG342rs {}
