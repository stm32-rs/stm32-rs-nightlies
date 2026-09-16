///Register `SWREG64` reader
pub type R = crate::R<SWREG64rs>;
///Register `SWREG64` writer
pub type W = crate::W<SWREG64rs>;
///Field `SWREG_FIELD` reader - JPEG luma quantization 1 / intra 16x16 mode 0-1 penalty (all format mode)
pub type SWREG_FIELD_R = crate::FieldReader<u32>;
///Field `SWREG_FIELD` writer - JPEG luma quantization 1 / intra 16x16 mode 0-1 penalty (all format mode)
pub type SWREG_FIELD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - JPEG luma quantization 1 / intra 16x16 mode 0-1 penalty (all format mode)
    #[inline(always)]
    pub fn swreg_field(&self) -> SWREG_FIELD_R {
        SWREG_FIELD_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWREG64")
            .field("swreg_field", &self.swreg_field())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - JPEG luma quantization 1 / intra 16x16 mode 0-1 penalty (all format mode)
    #[inline(always)]
    pub fn swreg_field(&mut self) -> SWREG_FIELD_W<'_, SWREG64rs> {
        SWREG_FIELD_W::new(self, 0)
    }
}
/**VENC JPEG luma quantization 1/intra 16x16 mode 0-1 penalty register

You can [`read`](crate::Reg::read) this register and get [`swreg64::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg64::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#VENC:SWREG64)*/
pub struct SWREG64rs;
impl crate::RegisterSpec for SWREG64rs {
    type Ux = u32;
}
///`read()` method returns [`swreg64::R`](R) reader structure
impl crate::Readable for SWREG64rs {}
///`write(|w| ..)` method takes [`swreg64::W`](W) writer structure
impl crate::Writable for SWREG64rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWREG64 to value 0
impl crate::Resettable for SWREG64rs {}
