///Register `SWREG13` reader
pub type R = crate::R<SWREG13rs>;
///Register `SWREG13` writer
pub type W = crate::W<SWREG13rs>;
///Field `SWREG_FIELD` reader - Base address for input picture cr (all format mode)
pub type SWREG_FIELD_R = crate::FieldReader<u32>;
///Field `SWREG_FIELD` writer - Base address for input picture cr (all format mode)
pub type SWREG_FIELD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Base address for input picture cr (all format mode)
    #[inline(always)]
    pub fn swreg_field(&self) -> SWREG_FIELD_R {
        SWREG_FIELD_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWREG13")
            .field("swreg_field", &self.swreg_field())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Base address for input picture cr (all format mode)
    #[inline(always)]
    pub fn swreg_field(&mut self) -> SWREG_FIELD_W<'_, SWREG13rs> {
        SWREG_FIELD_W::new(self, 0)
    }
}
/**VENC base address for input picture cr register

You can [`read`](crate::Reg::read) this register and get [`swreg13::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg13::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#VENC:SWREG13)*/
pub struct SWREG13rs;
impl crate::RegisterSpec for SWREG13rs {
    type Ux = u32;
}
///`read()` method returns [`swreg13::R`](R) reader structure
impl crate::Readable for SWREG13rs {}
///`write(|w| ..)` method takes [`swreg13::W`](W) writer structure
impl crate::Writable for SWREG13rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWREG13 to value 0
impl crate::Resettable for SWREG13rs {}
