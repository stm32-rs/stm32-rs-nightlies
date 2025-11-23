///Register `SWREG55` reader
pub type R = crate::R<SWREG55rs>;
///Register `SWREG55` writer
pub type W = crate::W<SWREG55rs>;
///Field `SWREG_FIELD` reader - RGB to YUV conversion coefficient F, RGB mask MSB bit position (all format mode)
pub type SWREG_FIELD_R = crate::FieldReader<u32>;
///Field `SWREG_FIELD` writer - RGB to YUV conversion coefficient F, RGB mask MSB bit position (all format mode)
pub type SWREG_FIELD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - RGB to YUV conversion coefficient F, RGB mask MSB bit position (all format mode)
    #[inline(always)]
    pub fn swreg_field(&self) -> SWREG_FIELD_R {
        SWREG_FIELD_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWREG55")
            .field("swreg_field", &self.swreg_field())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - RGB to YUV conversion coefficient F, RGB mask MSB bit position (all format mode)
    #[inline(always)]
    pub fn swreg_field(&mut self) -> SWREG_FIELD_W<'_, SWREG55rs> {
        SWREG_FIELD_W::new(self, 0)
    }
}
/**VENC RGB to YUV conversion coefficient F, RGB mask MSB bit position register

You can [`read`](crate::Reg::read) this register and get [`swreg55::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg55::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#VENC:SWREG55)*/
pub struct SWREG55rs;
impl crate::RegisterSpec for SWREG55rs {
    type Ux = u32;
}
///`read()` method returns [`swreg55::R`](R) reader structure
impl crate::Readable for SWREG55rs {}
///`write(|w| ..)` method takes [`swreg55::W`](W) writer structure
impl crate::Writable for SWREG55rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWREG55 to value 0
impl crate::Resettable for SWREG55rs {}
