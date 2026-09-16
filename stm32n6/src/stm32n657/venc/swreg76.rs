///Register `SWREG76` reader
pub type R = crate::R<SWREG76rs>;
///Register `SWREG76` writer
pub type W = crate::W<SWREG76rs>;
///Field `SWREG_FIELD` reader - JPEG luma quantization 13 / segment1 parameter (all format mode)
pub type SWREG_FIELD_R = crate::FieldReader<u32>;
///Field `SWREG_FIELD` writer - JPEG luma quantization 13 / segment1 parameter (all format mode)
pub type SWREG_FIELD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - JPEG luma quantization 13 / segment1 parameter (all format mode)
    #[inline(always)]
    pub fn swreg_field(&self) -> SWREG_FIELD_R {
        SWREG_FIELD_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWREG76")
            .field("swreg_field", &self.swreg_field())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - JPEG luma quantization 13 / segment1 parameter (all format mode)
    #[inline(always)]
    pub fn swreg_field(&mut self) -> SWREG_FIELD_W<'_, SWREG76rs> {
        SWREG_FIELD_W::new(self, 0)
    }
}
/**VENC JPEG luma quantization 13/segment1 parameter register

You can [`read`](crate::Reg::read) this register and get [`swreg76::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg76::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#VENC:SWREG76)*/
pub struct SWREG76rs;
impl crate::RegisterSpec for SWREG76rs {
    type Ux = u32;
}
///`read()` method returns [`swreg76::R`](R) reader structure
impl crate::Readable for SWREG76rs {}
///`write(|w| ..)` method takes [`swreg76::W`](W) writer structure
impl crate::Writable for SWREG76rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWREG76 to value 0
impl crate::Resettable for SWREG76rs {}
