///Register `SWREG21` reader
pub type R = crate::R<SWREG21rs>;
///Register `SWREG21` writer
pub type W = crate::W<SWREG21rs>;
///Field `SWREG_FIELD` reader - Control of H264 (all format mode)
pub type SWREG_FIELD_R = crate::FieldReader<u32>;
///Field `SWREG_FIELD` writer - Control of H264 (all format mode)
pub type SWREG_FIELD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Control of H264 (all format mode)
    #[inline(always)]
    pub fn swreg_field(&self) -> SWREG_FIELD_R {
        SWREG_FIELD_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWREG21")
            .field("swreg_field", &self.swreg_field())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Control of H264 (all format mode)
    #[inline(always)]
    pub fn swreg_field(&mut self) -> SWREG_FIELD_W<'_, SWREG21rs> {
        SWREG_FIELD_W::new(self, 0)
    }
}
/**VENC encoder control register 7

You can [`read`](crate::Reg::read) this register and get [`swreg21::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg21::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#VENC:SWREG21)*/
pub struct SWREG21rs;
impl crate::RegisterSpec for SWREG21rs {
    type Ux = u32;
}
///`read()` method returns [`swreg21::R`](R) reader structure
impl crate::Readable for SWREG21rs {}
///`write(|w| ..)` method takes [`swreg21::W`](W) writer structure
impl crate::Writable for SWREG21rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWREG21 to value 0
impl crate::Resettable for SWREG21rs {}
