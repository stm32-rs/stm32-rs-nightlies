///Register `SWREG53` reader
pub type R = crate::R<SWREG53rs>;
///Register `SWREG53` writer
pub type W = crate::W<SWREG53rs>;
///Field `SWREG_FIELD` reader - RGB to YUV conversion coefficient A - B (all format mode)
pub type SWREG_FIELD_R = crate::FieldReader<u32>;
///Field `SWREG_FIELD` writer - RGB to YUV conversion coefficient A - B (all format mode)
pub type SWREG_FIELD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - RGB to YUV conversion coefficient A - B (all format mode)
    #[inline(always)]
    pub fn swreg_field(&self) -> SWREG_FIELD_R {
        SWREG_FIELD_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWREG53")
            .field("swreg_field", &self.swreg_field())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - RGB to YUV conversion coefficient A - B (all format mode)
    #[inline(always)]
    pub fn swreg_field(&mut self) -> SWREG_FIELD_W<'_, SWREG53rs> {
        SWREG_FIELD_W::new(self, 0)
    }
}
/**VENC RGB to YUV conversion coefficient A - B register

You can [`read`](crate::Reg::read) this register and get [`swreg53::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg53::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#VENC:SWREG53)*/
pub struct SWREG53rs;
impl crate::RegisterSpec for SWREG53rs {
    type Ux = u32;
}
///`read()` method returns [`swreg53::R`](R) reader structure
impl crate::Readable for SWREG53rs {}
///`write(|w| ..)` method takes [`swreg53::W`](W) writer structure
impl crate::Writable for SWREG53rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWREG53 to value 0
impl crate::Resettable for SWREG53rs {}
