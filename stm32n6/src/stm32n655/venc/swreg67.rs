///Register `SWREG67` reader
pub type R = crate::R<SWREG67rs>;
///Register `SWREG67` writer
pub type W = crate::W<SWREG67rs>;
///Field `SWREG_FIELD` reader - JPEG luma quantization 4 / intra 4x4 mode 2-3 penalty (all format mode)
pub type SWREG_FIELD_R = crate::FieldReader<u32>;
///Field `SWREG_FIELD` writer - JPEG luma quantization 4 / intra 4x4 mode 2-3 penalty (all format mode)
pub type SWREG_FIELD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - JPEG luma quantization 4 / intra 4x4 mode 2-3 penalty (all format mode)
    #[inline(always)]
    pub fn swreg_field(&self) -> SWREG_FIELD_R {
        SWREG_FIELD_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWREG67")
            .field("swreg_field", &self.swreg_field())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - JPEG luma quantization 4 / intra 4x4 mode 2-3 penalty (all format mode)
    #[inline(always)]
    pub fn swreg_field(&mut self) -> SWREG_FIELD_W<'_, SWREG67rs> {
        SWREG_FIELD_W::new(self, 0)
    }
}
/**VENC JPEG luma quantization 4/intra 4x4 mode 2-3 penalty register

You can [`read`](crate::Reg::read) this register and get [`swreg67::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg67::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG67)*/
pub struct SWREG67rs;
impl crate::RegisterSpec for SWREG67rs {
    type Ux = u32;
}
///`read()` method returns [`swreg67::R`](R) reader structure
impl crate::Readable for SWREG67rs {}
///`write(|w| ..)` method takes [`swreg67::W`](W) writer structure
impl crate::Writable for SWREG67rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWREG67 to value 0
impl crate::Resettable for SWREG67rs {}
