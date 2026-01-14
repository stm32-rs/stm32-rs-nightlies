///Register `SWREG280` reader
pub type R = crate::R<SWREG280rs>;
///Register `SWREG280` writer
pub type W = crate::W<SWREG280rs>;
///Field `SWREG_FIELD` reader - segment 3: intra 16x16 mode 0-2 penalty (all format mode)
pub type SWREG_FIELD_R = crate::FieldReader<u32>;
///Field `SWREG_FIELD` writer - segment 3: intra 16x16 mode 0-2 penalty (all format mode)
pub type SWREG_FIELD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - segment 3: intra 16x16 mode 0-2 penalty (all format mode)
    #[inline(always)]
    pub fn swreg_field(&self) -> SWREG_FIELD_R {
        SWREG_FIELD_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWREG280")
            .field("swreg_field", &self.swreg_field())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - segment 3: intra 16x16 mode 0-2 penalty (all format mode)
    #[inline(always)]
    pub fn swreg_field(&mut self) -> SWREG_FIELD_W<'_, SWREG280rs> {
        SWREG_FIELD_W::new(self, 0)
    }
}
/**VENC segment 3: intra 16x16 mode 0-2 penalty register

You can [`read`](crate::Reg::read) this register and get [`swreg280::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg280::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG280)*/
pub struct SWREG280rs;
impl crate::RegisterSpec for SWREG280rs {
    type Ux = u32;
}
///`read()` method returns [`swreg280::R`](R) reader structure
impl crate::Readable for SWREG280rs {}
///`write(|w| ..)` method takes [`swreg280::W`](W) writer structure
impl crate::Writable for SWREG280rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWREG280 to value 0
impl crate::Resettable for SWREG280rs {}
