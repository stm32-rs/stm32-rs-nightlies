///Register `SWREG337` reader
pub type R = crate::R<SWREG337rs>;
///Register `SWREG337` writer
pub type W = crate::W<SWREG337rs>;
///Field `SWREG_FIELD` reader - segment 13: penalty value (all format mode)
pub type SWREG_FIELD_R = crate::FieldReader<u32>;
///Field `SWREG_FIELD` writer - segment 13: penalty value (all format mode)
pub type SWREG_FIELD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - segment 13: penalty value (all format mode)
    #[inline(always)]
    pub fn swreg_field(&self) -> SWREG_FIELD_R {
        SWREG_FIELD_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWREG337")
            .field("swreg_field", &self.swreg_field())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - segment 13: penalty value (all format mode)
    #[inline(always)]
    pub fn swreg_field(&mut self) -> SWREG_FIELD_W<'_, SWREG337rs> {
        SWREG_FIELD_W::new(self, 0)
    }
}
/**VENC segment 13: penalty value register

You can [`read`](crate::Reg::read) this register and get [`swreg337::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg337::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#VENC:SWREG337)*/
pub struct SWREG337rs;
impl crate::RegisterSpec for SWREG337rs {
    type Ux = u32;
}
///`read()` method returns [`swreg337::R`](R) reader structure
impl crate::Readable for SWREG337rs {}
///`write(|w| ..)` method takes [`swreg337::W`](W) writer structure
impl crate::Writable for SWREG337rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWREG337 to value 0
impl crate::Resettable for SWREG337rs {}
