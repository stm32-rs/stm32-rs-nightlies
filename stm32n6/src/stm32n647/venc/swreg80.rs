///Register `SWREG80` reader
pub type R = crate::R<SWREG80rs>;
///Register `SWREG80` writer
pub type W = crate::W<SWREG80rs>;
///Field `SWREG_FIELD` reader - JPEG chroma quantization 1 / segment2 parameter (all format mode)
pub type SWREG_FIELD_R = crate::FieldReader<u32>;
///Field `SWREG_FIELD` writer - JPEG chroma quantization 1 / segment2 parameter (all format mode)
pub type SWREG_FIELD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - JPEG chroma quantization 1 / segment2 parameter (all format mode)
    #[inline(always)]
    pub fn swreg_field(&self) -> SWREG_FIELD_R {
        SWREG_FIELD_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWREG80")
            .field("swreg_field", &self.swreg_field())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - JPEG chroma quantization 1 / segment2 parameter (all format mode)
    #[inline(always)]
    pub fn swreg_field(&mut self) -> SWREG_FIELD_W<'_, SWREG80rs> {
        SWREG_FIELD_W::new(self, 0)
    }
}
/**VENC JPEG chroma quantization 1/segment2 parameter register

You can [`read`](crate::Reg::read) this register and get [`swreg80::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg80::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#VENC:SWREG80)*/
pub struct SWREG80rs;
impl crate::RegisterSpec for SWREG80rs {
    type Ux = u32;
}
///`read()` method returns [`swreg80::R`](R) reader structure
impl crate::Readable for SWREG80rs {}
///`write(|w| ..)` method takes [`swreg80::W`](W) writer structure
impl crate::Writable for SWREG80rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWREG80 to value 0
impl crate::Resettable for SWREG80rs {}
