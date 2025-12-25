///Register `SWREG294` reader
pub type R = crate::R<SWREG294rs>;
///Register `SWREG294` writer
pub type W = crate::W<SWREG294rs>;
///Field `SWREG_FIELD` reader - Mb boost (all format mode)
pub type SWREG_FIELD_R = crate::FieldReader<u32>;
///Field `SWREG_FIELD` writer - Mb boost (all format mode)
pub type SWREG_FIELD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Mb boost (all format mode)
    #[inline(always)]
    pub fn swreg_field(&self) -> SWREG_FIELD_R {
        SWREG_FIELD_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWREG294")
            .field("swreg_field", &self.swreg_field())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Mb boost (all format mode)
    #[inline(always)]
    pub fn swreg_field(&mut self) -> SWREG_FIELD_W<'_, SWREG294rs> {
        SWREG_FIELD_W::new(self, 0)
    }
}
/**VENC Mb boost register

You can [`read`](crate::Reg::read) this register and get [`swreg294::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg294::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG294)*/
pub struct SWREG294rs;
impl crate::RegisterSpec for SWREG294rs {
    type Ux = u32;
}
///`read()` method returns [`swreg294::R`](R) reader structure
impl crate::Readable for SWREG294rs {}
///`write(|w| ..)` method takes [`swreg294::W`](W) writer structure
impl crate::Writable for SWREG294rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWREG294 to value 0
impl crate::Resettable for SWREG294rs {}
