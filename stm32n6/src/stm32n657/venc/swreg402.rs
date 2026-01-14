///Register `SWREG402` reader
pub type R = crate::R<SWREG402rs>;
///Register `SWREG402` writer
pub type W = crate::W<SWREG402rs>;
///Field `SWREG_FIELD` reader - segment 30: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame (all format mode)
pub type SWREG_FIELD_R = crate::FieldReader<u32>;
///Field `SWREG_FIELD` writer - segment 30: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame (all format mode)
pub type SWREG_FIELD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - segment 30: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame (all format mode)
    #[inline(always)]
    pub fn swreg_field(&self) -> SWREG_FIELD_R {
        SWREG_FIELD_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWREG402")
            .field("swreg_field", &self.swreg_field())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - segment 30: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame (all format mode)
    #[inline(always)]
    pub fn swreg_field(&mut self) -> SWREG_FIELD_W<'_, SWREG402rs> {
        SWREG_FIELD_W::new(self, 0)
    }
}
/**VENC segment 30: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame register

You can [`read`](crate::Reg::read) this register and get [`swreg402::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg402::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#VENC:SWREG402)*/
pub struct SWREG402rs;
impl crate::RegisterSpec for SWREG402rs {
    type Ux = u32;
}
///`read()` method returns [`swreg402::R`](R) reader structure
impl crate::Readable for SWREG402rs {}
///`write(|w| ..)` method takes [`swreg402::W`](W) writer structure
impl crate::Writable for SWREG402rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWREG402 to value 0
impl crate::Resettable for SWREG402rs {}
