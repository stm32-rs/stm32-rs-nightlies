///Register `SWREG95` reader
pub type R = crate::R<SWREG95rs>;
///Register `SWREG95` writer
pub type W = crate::W<SWREG95rs>;
///Field `SWREG_FIELD` reader - JPEG chroma quantization 16 / segment3 parameter (all format mode)
pub type SWREG_FIELD_R = crate::FieldReader<u32>;
///Field `SWREG_FIELD` writer - JPEG chroma quantization 16 / segment3 parameter (all format mode)
pub type SWREG_FIELD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - JPEG chroma quantization 16 / segment3 parameter (all format mode)
    #[inline(always)]
    pub fn swreg_field(&self) -> SWREG_FIELD_R {
        SWREG_FIELD_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWREG95")
            .field("swreg_field", &self.swreg_field())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - JPEG chroma quantization 16 / segment3 parameter (all format mode)
    #[inline(always)]
    pub fn swreg_field(&mut self) -> SWREG_FIELD_W<'_, SWREG95rs> {
        SWREG_FIELD_W::new(self, 0)
    }
}
/**VENC JPEG chroma quantization 16/segment3 parameter register

You can [`read`](crate::Reg::read) this register and get [`swreg95::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg95::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG95)*/
pub struct SWREG95rs;
impl crate::RegisterSpec for SWREG95rs {
    type Ux = u32;
}
///`read()` method returns [`swreg95::R`](R) reader structure
impl crate::Readable for SWREG95rs {}
///`write(|w| ..)` method takes [`swreg95::W`](W) writer structure
impl crate::Writable for SWREG95rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWREG95 to value 0
impl crate::Resettable for SWREG95rs {}
