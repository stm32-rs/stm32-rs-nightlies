///Register `SWREG18` reader
pub type R = crate::R<SWREG18rs>;
///Register `SWREG18` writer
pub type W = crate::W<SWREG18rs>;
///Field `SWREG_FIELD` reader - Encoder control register 4 (deblock filter mode, H264 control) (all format mode)
pub type SWREG_FIELD_R = crate::FieldReader<u32>;
///Field `SWREG_FIELD` writer - Encoder control register 4 (deblock filter mode, H264 control) (all format mode)
pub type SWREG_FIELD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Encoder control register 4 (deblock filter mode, H264 control) (all format mode)
    #[inline(always)]
    pub fn swreg_field(&self) -> SWREG_FIELD_R {
        SWREG_FIELD_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWREG18")
            .field("swreg_field", &self.swreg_field())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Encoder control register 4 (deblock filter mode, H264 control) (all format mode)
    #[inline(always)]
    pub fn swreg_field(&mut self) -> SWREG_FIELD_W<SWREG18rs> {
        SWREG_FIELD_W::new(self, 0)
    }
}
/**VENC encoder control register 4

You can [`read`](crate::Reg::read) this register and get [`swreg18::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg18::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#VENC:SWREG18)*/
pub struct SWREG18rs;
impl crate::RegisterSpec for SWREG18rs {
    type Ux = u32;
}
///`read()` method returns [`swreg18::R`](R) reader structure
impl crate::Readable for SWREG18rs {}
///`write(|w| ..)` method takes [`swreg18::W`](W) writer structure
impl crate::Writable for SWREG18rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWREG18 to value 0
impl crate::Resettable for SWREG18rs {}
