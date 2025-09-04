///Register `SWREG318` reader
pub type R = crate::R<SWREG318rs>;
///Register `SWREG318` writer
pub type W = crate::W<SWREG318rs>;
///Field `SWREG_FIELD` reader - segment 9: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame (all format mode)
pub type SWREG_FIELD_R = crate::FieldReader<u32>;
///Field `SWREG_FIELD` writer - segment 9: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame (all format mode)
pub type SWREG_FIELD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - segment 9: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame (all format mode)
    #[inline(always)]
    pub fn swreg_field(&self) -> SWREG_FIELD_R {
        SWREG_FIELD_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWREG318")
            .field("swreg_field", &self.swreg_field())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - segment 9: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame (all format mode)
    #[inline(always)]
    pub fn swreg_field(&mut self) -> SWREG_FIELD_W<SWREG318rs> {
        SWREG_FIELD_W::new(self, 0)
    }
}
/**VENC segment 9: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame register

You can [`read`](crate::Reg::read) this register and get [`swreg318::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg318::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG318)*/
pub struct SWREG318rs;
impl crate::RegisterSpec for SWREG318rs {
    type Ux = u32;
}
///`read()` method returns [`swreg318::R`](R) reader structure
impl crate::Readable for SWREG318rs {}
///`write(|w| ..)` method takes [`swreg318::W`](W) writer structure
impl crate::Writable for SWREG318rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWREG318 to value 0
impl crate::Resettable for SWREG318rs {}
