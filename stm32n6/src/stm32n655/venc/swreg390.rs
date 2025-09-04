///Register `SWREG390` reader
pub type R = crate::R<SWREG390rs>;
///Register `SWREG390` writer
pub type W = crate::W<SWREG390rs>;
///Field `SWREG_FIELD` reader - segment 27: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame (all format mode)
pub type SWREG_FIELD_R = crate::FieldReader<u32>;
///Field `SWREG_FIELD` writer - segment 27: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame (all format mode)
pub type SWREG_FIELD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - segment 27: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame (all format mode)
    #[inline(always)]
    pub fn swreg_field(&self) -> SWREG_FIELD_R {
        SWREG_FIELD_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWREG390")
            .field("swreg_field", &self.swreg_field())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - segment 27: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame (all format mode)
    #[inline(always)]
    pub fn swreg_field(&mut self) -> SWREG_FIELD_W<SWREG390rs> {
        SWREG_FIELD_W::new(self, 0)
    }
}
/**VENC segment 27: intra 4x4 previous mode favor, intra 16x16mode favor, penalty value for second reference frame register

You can [`read`](crate::Reg::read) this register and get [`swreg390::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg390::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG390)*/
pub struct SWREG390rs;
impl crate::RegisterSpec for SWREG390rs {
    type Ux = u32;
}
///`read()` method returns [`swreg390::R`](R) reader structure
impl crate::Readable for SWREG390rs {}
///`write(|w| ..)` method takes [`swreg390::W`](W) writer structure
impl crate::Writable for SWREG390rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWREG390 to value 0
impl crate::Resettable for SWREG390rs {}
