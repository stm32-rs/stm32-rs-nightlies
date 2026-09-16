///Register `SWREG26` reader
pub type R = crate::R<SWREG26rs>;
///Register `SWREG26` writer
pub type W = crate::W<SWREG26rs>;
///Field `SWREG_FIELD` reader - intra-slice bitmap for probability updates (all format mode)
pub type SWREG_FIELD_R = crate::FieldReader<u32>;
///Field `SWREG_FIELD` writer - intra-slice bitmap for probability updates (all format mode)
pub type SWREG_FIELD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - intra-slice bitmap for probability updates (all format mode)
    #[inline(always)]
    pub fn swreg_field(&self) -> SWREG_FIELD_R {
        SWREG_FIELD_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWREG26")
            .field("swreg_field", &self.swreg_field())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - intra-slice bitmap for probability updates (all format mode)
    #[inline(always)]
    pub fn swreg_field(&mut self) -> SWREG_FIELD_W<'_, SWREG26rs> {
        SWREG_FIELD_W::new(self, 0)
    }
}
/**VENC intra-slice bitmap register

You can [`read`](crate::Reg::read) this register and get [`swreg26::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg26::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG26)*/
pub struct SWREG26rs;
impl crate::RegisterSpec for SWREG26rs {
    type Ux = u32;
}
///`read()` method returns [`swreg26::R`](R) reader structure
impl crate::Readable for SWREG26rs {}
///`write(|w| ..)` method takes [`swreg26::W`](W) writer structure
impl crate::Writable for SWREG26rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWREG26 to value 0
impl crate::Resettable for SWREG26rs {}
