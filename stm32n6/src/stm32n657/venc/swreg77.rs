///Register `SWREG77` reader
pub type R = crate::R<SWREG77rs>;
///Register `SWREG77` writer
pub type W = crate::W<SWREG77rs>;
///Field `SWREG_FIELD` reader - JPEG luma quantization 14 / segment1 parameter (all format mode)
pub type SWREG_FIELD_R = crate::FieldReader<u32>;
///Field `SWREG_FIELD` writer - JPEG luma quantization 14 / segment1 parameter (all format mode)
pub type SWREG_FIELD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - JPEG luma quantization 14 / segment1 parameter (all format mode)
    #[inline(always)]
    pub fn swreg_field(&self) -> SWREG_FIELD_R {
        SWREG_FIELD_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWREG77")
            .field("swreg_field", &self.swreg_field())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - JPEG luma quantization 14 / segment1 parameter (all format mode)
    #[inline(always)]
    pub fn swreg_field(&mut self) -> SWREG_FIELD_W<SWREG77rs> {
        SWREG_FIELD_W::new(self, 0)
    }
}
/**VENC JPEG luma quantization 14/segment1 parameter register

You can [`read`](crate::Reg::read) this register and get [`swreg77::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg77::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#VENC:SWREG77)*/
pub struct SWREG77rs;
impl crate::RegisterSpec for SWREG77rs {
    type Ux = u32;
}
///`read()` method returns [`swreg77::R`](R) reader structure
impl crate::Readable for SWREG77rs {}
///`write(|w| ..)` method takes [`swreg77::W`](W) writer structure
impl crate::Writable for SWREG77rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWREG77 to value 0
impl crate::Resettable for SWREG77rs {}
