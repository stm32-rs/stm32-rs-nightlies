///Register `SWREG442` reader
pub type R = crate::R<SWREG442rs>;
///Register `SWREG442` writer
pub type W = crate::W<SWREG442rs>;
///Field `SWREG_FIELD` reader - high 32 bits of Base address for next pic luminance (all format mode)
pub type SWREG_FIELD_R = crate::FieldReader<u32>;
///Field `SWREG_FIELD` writer - high 32 bits of Base address for next pic luminance (all format mode)
pub type SWREG_FIELD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - high 32 bits of Base address for next pic luminance (all format mode)
    #[inline(always)]
    pub fn swreg_field(&self) -> SWREG_FIELD_R {
        SWREG_FIELD_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWREG442")
            .field("swreg_field", &self.swreg_field())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - high 32 bits of Base address for next pic luminance (all format mode)
    #[inline(always)]
    pub fn swreg_field(&mut self) -> SWREG_FIELD_W<'_, SWREG442rs> {
        SWREG_FIELD_W::new(self, 0)
    }
}
/**VENC high 32 bits of base address for next pic luminance register

You can [`read`](crate::Reg::read) this register and get [`swreg442::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg442::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#VENC:SWREG442)*/
pub struct SWREG442rs;
impl crate::RegisterSpec for SWREG442rs {
    type Ux = u32;
}
///`read()` method returns [`swreg442::R`](R) reader structure
impl crate::Readable for SWREG442rs {}
///`write(|w| ..)` method takes [`swreg442::W`](W) writer structure
impl crate::Writable for SWREG442rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWREG442 to value 0
impl crate::Resettable for SWREG442rs {}
