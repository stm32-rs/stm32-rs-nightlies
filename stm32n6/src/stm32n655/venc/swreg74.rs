///Register `SWREG74` reader
pub type R = crate::R<SWREG74rs>;
///Register `SWREG74` writer
pub type W = crate::W<SWREG74rs>;
///Field `SWREG_FIELD` reader - JPEG luma quantization 11 / segment1 parameter (all format mode)
pub type SWREG_FIELD_R = crate::FieldReader<u32>;
///Field `SWREG_FIELD` writer - JPEG luma quantization 11 / segment1 parameter (all format mode)
pub type SWREG_FIELD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - JPEG luma quantization 11 / segment1 parameter (all format mode)
    #[inline(always)]
    pub fn swreg_field(&self) -> SWREG_FIELD_R {
        SWREG_FIELD_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWREG74")
            .field("swreg_field", &self.swreg_field())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - JPEG luma quantization 11 / segment1 parameter (all format mode)
    #[inline(always)]
    pub fn swreg_field(&mut self) -> SWREG_FIELD_W<'_, SWREG74rs> {
        SWREG_FIELD_W::new(self, 0)
    }
}
/**VENC JPEG luma quantization 11/segment1 parameter register

You can [`read`](crate::Reg::read) this register and get [`swreg74::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg74::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG74)*/
pub struct SWREG74rs;
impl crate::RegisterSpec for SWREG74rs {
    type Ux = u32;
}
///`read()` method returns [`swreg74::R`](R) reader structure
impl crate::Readable for SWREG74rs {}
///`write(|w| ..)` method takes [`swreg74::W`](W) writer structure
impl crate::Writable for SWREG74rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWREG74 to value 0
impl crate::Resettable for SWREG74rs {}
