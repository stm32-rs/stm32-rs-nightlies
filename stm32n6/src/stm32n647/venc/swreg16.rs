///Register `SWREG16` reader
pub type R = crate::R<SWREG16rs>;
///Register `SWREG16` writer
pub type W = crate::W<SWREG16rs>;
///Field `SWREG_FIELD` reader - Base address for second reference luma (H264 control) (all format mode)
pub type SWREG_FIELD_R = crate::FieldReader<u32>;
///Field `SWREG_FIELD` writer - Base address for second reference luma (H264 control) (all format mode)
pub type SWREG_FIELD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Base address for second reference luma (H264 control) (all format mode)
    #[inline(always)]
    pub fn swreg_field(&self) -> SWREG_FIELD_R {
        SWREG_FIELD_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWREG16")
            .field("swreg_field", &self.swreg_field())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Base address for second reference luma (H264 control) (all format mode)
    #[inline(always)]
    pub fn swreg_field(&mut self) -> SWREG_FIELD_W<'_, SWREG16rs> {
        SWREG_FIELD_W::new(self, 0)
    }
}
/**VENC encoder control register 2

You can [`read`](crate::Reg::read) this register and get [`swreg16::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg16::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#VENC:SWREG16)*/
pub struct SWREG16rs;
impl crate::RegisterSpec for SWREG16rs {
    type Ux = u32;
}
///`read()` method returns [`swreg16::R`](R) reader structure
impl crate::Readable for SWREG16rs {}
///`write(|w| ..)` method takes [`swreg16::W`](W) writer structure
impl crate::Writable for SWREG16rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWREG16 to value 0
impl crate::Resettable for SWREG16rs {}
