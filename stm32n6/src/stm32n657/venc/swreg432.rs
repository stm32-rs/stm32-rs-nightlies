///Register `SWREG432` reader
pub type R = crate::R<SWREG432rs>;
///Register `SWREG432` writer
pub type W = crate::W<SWREG432rs>;
///Field `SWREG_FIELD` reader - high 32 bits of Base address for reference chroma (all format mode)
pub type SWREG_FIELD_R = crate::FieldReader<u32>;
///Field `SWREG_FIELD` writer - high 32 bits of Base address for reference chroma (all format mode)
pub type SWREG_FIELD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - high 32 bits of Base address for reference chroma (all format mode)
    #[inline(always)]
    pub fn swreg_field(&self) -> SWREG_FIELD_R {
        SWREG_FIELD_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWREG432")
            .field("swreg_field", &self.swreg_field())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - high 32 bits of Base address for reference chroma (all format mode)
    #[inline(always)]
    pub fn swreg_field(&mut self) -> SWREG_FIELD_W<'_, SWREG432rs> {
        SWREG_FIELD_W::new(self, 0)
    }
}
/**VENC high 32 bits of base address for reference chroma register

You can [`read`](crate::Reg::read) this register and get [`swreg432::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg432::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#VENC:SWREG432)*/
pub struct SWREG432rs;
impl crate::RegisterSpec for SWREG432rs {
    type Ux = u32;
}
///`read()` method returns [`swreg432::R`](R) reader structure
impl crate::Readable for SWREG432rs {}
///`write(|w| ..)` method takes [`swreg432::W`](W) writer structure
impl crate::Writable for SWREG432rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWREG432 to value 0
impl crate::Resettable for SWREG432rs {}
