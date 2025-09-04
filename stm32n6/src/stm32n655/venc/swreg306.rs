///Register `SWREG306` reader
pub type R = crate::R<SWREG306rs>;
///Register `SWREG306` writer
pub type W = crate::W<SWREG306rs>;
///Field `SWREG_FIELD` reader - segment 6: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame (all format mode)
pub type SWREG_FIELD_R = crate::FieldReader<u32>;
///Field `SWREG_FIELD` writer - segment 6: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame (all format mode)
pub type SWREG_FIELD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - segment 6: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame (all format mode)
    #[inline(always)]
    pub fn swreg_field(&self) -> SWREG_FIELD_R {
        SWREG_FIELD_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWREG306")
            .field("swreg_field", &self.swreg_field())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - segment 6: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame (all format mode)
    #[inline(always)]
    pub fn swreg_field(&mut self) -> SWREG_FIELD_W<SWREG306rs> {
        SWREG_FIELD_W::new(self, 0)
    }
}
/**VENC segment 6: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame register

You can [`read`](crate::Reg::read) this register and get [`swreg306::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg306::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG306)*/
pub struct SWREG306rs;
impl crate::RegisterSpec for SWREG306rs {
    type Ux = u32;
}
///`read()` method returns [`swreg306::R`](R) reader structure
impl crate::Readable for SWREG306rs {}
///`write(|w| ..)` method takes [`swreg306::W`](W) writer structure
impl crate::Writable for SWREG306rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWREG306 to value 0
impl crate::Resettable for SWREG306rs {}
