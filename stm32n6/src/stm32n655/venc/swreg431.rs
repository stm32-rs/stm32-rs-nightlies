///Register `SWREG431` reader
pub type R = crate::R<SWREG431rs>;
///Register `SWREG431` writer
pub type W = crate::W<SWREG431rs>;
///Field `SWREG_FIELD` reader - high 32 bits of Base address for reference luma (all format mode)
pub type SWREG_FIELD_R = crate::FieldReader<u32>;
///Field `SWREG_FIELD` writer - high 32 bits of Base address for reference luma (all format mode)
pub type SWREG_FIELD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - high 32 bits of Base address for reference luma (all format mode)
    #[inline(always)]
    pub fn swreg_field(&self) -> SWREG_FIELD_R {
        SWREG_FIELD_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWREG431")
            .field("swreg_field", &self.swreg_field())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - high 32 bits of Base address for reference luma (all format mode)
    #[inline(always)]
    pub fn swreg_field(&mut self) -> SWREG_FIELD_W<'_, SWREG431rs> {
        SWREG_FIELD_W::new(self, 0)
    }
}
/**VENC high 32 bits of base address for reference luma register

You can [`read`](crate::Reg::read) this register and get [`swreg431::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg431::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG431)*/
pub struct SWREG431rs;
impl crate::RegisterSpec for SWREG431rs {
    type Ux = u32;
}
///`read()` method returns [`swreg431::R`](R) reader structure
impl crate::Readable for SWREG431rs {}
///`write(|w| ..)` method takes [`swreg431::W`](W) writer structure
impl crate::Writable for SWREG431rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWREG431 to value 0
impl crate::Resettable for SWREG431rs {}
