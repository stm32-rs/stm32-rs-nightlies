///Register `SWREG2` reader
pub type R = crate::R<SWREG2rs>;
///Register `SWREG2` writer
pub type W = crate::W<SWREG2rs>;
///Field `SWREG_FIELD` reader - Bus interface configuration register (all format mode)
pub type SWREG_FIELD_R = crate::FieldReader<u32>;
///Field `SWREG_FIELD` writer - Bus interface configuration register (all format mode)
pub type SWREG_FIELD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Bus interface configuration register (all format mode)
    #[inline(always)]
    pub fn swreg_field(&self) -> SWREG_FIELD_R {
        SWREG_FIELD_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWREG2")
            .field("swreg_field", &self.swreg_field())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Bus interface configuration register (all format mode)
    #[inline(always)]
    pub fn swreg_field(&mut self) -> SWREG_FIELD_W<'_, SWREG2rs> {
        SWREG_FIELD_W::new(self, 0)
    }
}
/**VENC bus interface configuration register

You can [`read`](crate::Reg::read) this register and get [`swreg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG2)*/
pub struct SWREG2rs;
impl crate::RegisterSpec for SWREG2rs {
    type Ux = u32;
}
///`read()` method returns [`swreg2::R`](R) reader structure
impl crate::Readable for SWREG2rs {}
///`write(|w| ..)` method takes [`swreg2::W`](W) writer structure
impl crate::Writable for SWREG2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWREG2 to value 0x10
impl crate::Resettable for SWREG2rs {
    const RESET_VALUE: u32 = 0x10;
}
