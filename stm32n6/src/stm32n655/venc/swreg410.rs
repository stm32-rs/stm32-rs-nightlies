///Register `SWREG410` reader
pub type R = crate::R<SWREG410rs>;
///Register `SWREG410` writer
pub type W = crate::W<SWREG410rs>;
///Field `SWREG_FIELD` reader - MBRC control (QP, offset, enable) (all format mode)
pub type SWREG_FIELD_R = crate::FieldReader<u32>;
///Field `SWREG_FIELD` writer - MBRC control (QP, offset, enable) (all format mode)
pub type SWREG_FIELD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - MBRC control (QP, offset, enable) (all format mode)
    #[inline(always)]
    pub fn swreg_field(&self) -> SWREG_FIELD_R {
        SWREG_FIELD_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWREG410")
            .field("swreg_field", &self.swreg_field())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - MBRC control (QP, offset, enable) (all format mode)
    #[inline(always)]
    pub fn swreg_field(&mut self) -> SWREG_FIELD_W<'_, SWREG410rs> {
        SWREG_FIELD_W::new(self, 0)
    }
}
/**VENC MBRC control, QP, offset, enable register

You can [`read`](crate::Reg::read) this register and get [`swreg410::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg410::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG410)*/
pub struct SWREG410rs;
impl crate::RegisterSpec for SWREG410rs {
    type Ux = u32;
}
///`read()` method returns [`swreg410::R`](R) reader structure
impl crate::Readable for SWREG410rs {}
///`write(|w| ..)` method takes [`swreg410::W`](W) writer structure
impl crate::Writable for SWREG410rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWREG410 to value 0
impl crate::Resettable for SWREG410rs {}
