///Register `MPCWM3_NSWMR1` reader
pub type R = crate::R<MPCWM3_NSWMR1rs>;
///Register `MPCWM3_NSWMR1` writer
pub type W = crate::W<MPCWM3_NSWMR1rs>;
///Field `NSWM2STRT` reader - NSWM2STRT
pub type NSWM2STRT_R = crate::FieldReader<u16>;
///Field `NSWM2STRT` writer - NSWM2STRT
pub type NSWM2STRT_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
///Field `NSWM2LGTH` reader - NSWM2LGTH
pub type NSWM2LGTH_R = crate::FieldReader<u16>;
///Field `NSWM2LGTH` writer - NSWM2LGTH
pub type NSWM2LGTH_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    ///Bits 0:10 - NSWM2STRT
    #[inline(always)]
    pub fn nswm2strt(&self) -> NSWM2STRT_R {
        NSWM2STRT_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 16:27 - NSWM2LGTH
    #[inline(always)]
    pub fn nswm2lgth(&self) -> NSWM2LGTH_R {
        NSWM2LGTH_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MPCWM3_NSWMR1")
            .field("nswm2strt", &self.nswm2strt())
            .field("nswm2lgth", &self.nswm2lgth())
            .finish()
    }
}
impl W {
    ///Bits 0:10 - NSWM2STRT
    #[inline(always)]
    pub fn nswm2strt(&mut self) -> NSWM2STRT_W<'_, MPCWM3_NSWMR1rs> {
        NSWM2STRT_W::new(self, 0)
    }
    ///Bits 16:27 - NSWM2LGTH
    #[inline(always)]
    pub fn nswm2lgth(&mut self) -> NSWM2LGTH_W<'_, MPCWM3_NSWMR1rs> {
        NSWM2LGTH_W::new(self, 16)
    }
}
/**TZSC external memory non-secure watermark register 2

You can [`read`](crate::Reg::read) this register and get [`mpcwm3_nswmr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcwm3_nswmr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#GTZC_TZSC:MPCWM3_NSWMR1)*/
pub struct MPCWM3_NSWMR1rs;
impl crate::RegisterSpec for MPCWM3_NSWMR1rs {
    type Ux = u32;
}
///`read()` method returns [`mpcwm3_nswmr1::R`](R) reader structure
impl crate::Readable for MPCWM3_NSWMR1rs {}
///`write(|w| ..)` method takes [`mpcwm3_nswmr1::W`](W) writer structure
impl crate::Writable for MPCWM3_NSWMR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MPCWM3_NSWMR1 to value 0
impl crate::Resettable for MPCWM3_NSWMR1rs {}
