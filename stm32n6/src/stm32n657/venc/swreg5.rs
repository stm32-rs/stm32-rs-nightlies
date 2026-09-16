///Register `SWREG5` reader
pub type R = crate::R<SWREG5rs>;
///Register `SWREG5` writer
pub type W = crate::W<SWREG5rs>;
///Field `SWREG_FIELD` reader - Base address for output stream data (all format mode)
pub type SWREG_FIELD_R = crate::FieldReader<u32>;
///Field `SWREG_FIELD` writer - Base address for output stream data (all format mode)
pub type SWREG_FIELD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Base address for output stream data (all format mode)
    #[inline(always)]
    pub fn swreg_field(&self) -> SWREG_FIELD_R {
        SWREG_FIELD_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWREG5")
            .field("swreg_field", &self.swreg_field())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Base address for output stream data (all format mode)
    #[inline(always)]
    pub fn swreg_field(&mut self) -> SWREG_FIELD_W<'_, SWREG5rs> {
        SWREG_FIELD_W::new(self, 0)
    }
}
/**VENC base address for output stream data register

You can [`read`](crate::Reg::read) this register and get [`swreg5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#VENC:SWREG5)*/
pub struct SWREG5rs;
impl crate::RegisterSpec for SWREG5rs {
    type Ux = u32;
}
///`read()` method returns [`swreg5::R`](R) reader structure
impl crate::Readable for SWREG5rs {}
///`write(|w| ..)` method takes [`swreg5::W`](W) writer structure
impl crate::Writable for SWREG5rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWREG5 to value 0
impl crate::Resettable for SWREG5rs {}
