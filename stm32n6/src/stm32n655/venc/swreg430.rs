///Register `SWREG430` reader
pub type R = crate::R<SWREG430rs>;
///Register `SWREG430` writer
pub type W = crate::W<SWREG430rs>;
///Field `SWREG_FIELD` reader - high 32 bits of Base address for output control data (all format mode)
pub type SWREG_FIELD_R = crate::FieldReader<u32>;
///Field `SWREG_FIELD` writer - high 32 bits of Base address for output control data (all format mode)
pub type SWREG_FIELD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - high 32 bits of Base address for output control data (all format mode)
    #[inline(always)]
    pub fn swreg_field(&self) -> SWREG_FIELD_R {
        SWREG_FIELD_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWREG430")
            .field("swreg_field", &self.swreg_field())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - high 32 bits of Base address for output control data (all format mode)
    #[inline(always)]
    pub fn swreg_field(&mut self) -> SWREG_FIELD_W<'_, SWREG430rs> {
        SWREG_FIELD_W::new(self, 0)
    }
}
/**VENC high 32 bits of base address for output control data register

You can [`read`](crate::Reg::read) this register and get [`swreg430::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg430::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG430)*/
pub struct SWREG430rs;
impl crate::RegisterSpec for SWREG430rs {
    type Ux = u32;
}
///`read()` method returns [`swreg430::R`](R) reader structure
impl crate::Readable for SWREG430rs {}
///`write(|w| ..)` method takes [`swreg430::W`](W) writer structure
impl crate::Writable for SWREG430rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWREG430 to value 0
impl crate::Resettable for SWREG430rs {}
