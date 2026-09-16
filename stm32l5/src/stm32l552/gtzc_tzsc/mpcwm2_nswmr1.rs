///Register `MPCWM2_NSWMR1` reader
pub type R = crate::R<MPCWM2_NSWMR1rs>;
///Register `MPCWM2_NSWMR1` writer
pub type W = crate::W<MPCWM2_NSWMR1rs>;
///Field `NSWM1STRT` reader - NSWM1STRT
pub type NSWM1STRT_R = crate::FieldReader<u16>;
///Field `NSWM1STRT` writer - NSWM1STRT
pub type NSWM1STRT_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
///Field `NSWM1LGTH` reader - NSWM1LGTH
pub type NSWM1LGTH_R = crate::FieldReader<u16>;
///Field `NSWM1LGTH` writer - NSWM1LGTH
pub type NSWM1LGTH_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    ///Bits 0:10 - NSWM1STRT
    #[inline(always)]
    pub fn nswm1strt(&self) -> NSWM1STRT_R {
        NSWM1STRT_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 16:27 - NSWM1LGTH
    #[inline(always)]
    pub fn nswm1lgth(&self) -> NSWM1LGTH_R {
        NSWM1LGTH_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MPCWM2_NSWMR1")
            .field("nswm1strt", &self.nswm1strt())
            .field("nswm1lgth", &self.nswm1lgth())
            .finish()
    }
}
impl W {
    ///Bits 0:10 - NSWM1STRT
    #[inline(always)]
    pub fn nswm1strt(&mut self) -> NSWM1STRT_W<'_, MPCWM2_NSWMR1rs> {
        NSWM1STRT_W::new(self, 0)
    }
    ///Bits 16:27 - NSWM1LGTH
    #[inline(always)]
    pub fn nswm1lgth(&mut self) -> NSWM1LGTH_W<'_, MPCWM2_NSWMR1rs> {
        NSWM1LGTH_W::new(self, 16)
    }
}
/**TZSC external memory non-secure watermark register 1

You can [`read`](crate::Reg::read) this register and get [`mpcwm2_nswmr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcwm2_nswmr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#GTZC_TZSC:MPCWM2_NSWMR1)*/
pub struct MPCWM2_NSWMR1rs;
impl crate::RegisterSpec for MPCWM2_NSWMR1rs {
    type Ux = u32;
}
///`read()` method returns [`mpcwm2_nswmr1::R`](R) reader structure
impl crate::Readable for MPCWM2_NSWMR1rs {}
///`write(|w| ..)` method takes [`mpcwm2_nswmr1::W`](W) writer structure
impl crate::Writable for MPCWM2_NSWMR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MPCWM2_NSWMR1 to value 0
impl crate::Resettable for MPCWM2_NSWMR1rs {}
