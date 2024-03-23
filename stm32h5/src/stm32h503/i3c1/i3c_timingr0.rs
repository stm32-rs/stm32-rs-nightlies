#[doc = "Register `I3C_TIMINGR0` reader"]
pub type R = crate::R<I3C_TIMINGR0rs>;
#[doc = "Register `I3C_TIMINGR0` writer"]
pub type W = crate::W<I3C_TIMINGR0rs>;
#[doc = "Field `SCLL_PP` reader - SCL low duration in I3C push-pull phases, in number of kernel clocks cycles: tSCLL_PP = (SCLL_PP + 1) x tI3CCLK SCLL_PP is used to generate tLOW (I3C) timing."]
pub type SCLL_PP_R = crate::FieldReader;
#[doc = "Field `SCLL_PP` writer - SCL low duration in I3C push-pull phases, in number of kernel clocks cycles: tSCLL_PP = (SCLL_PP + 1) x tI3CCLK SCLL_PP is used to generate tLOW (I3C) timing."]
pub type SCLL_PP_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SCLH_I3C` reader - SCL high duration, used for I3C messages (both in push-pull and open-drain phases), in number of kernel clocks cycles: tSCLH_I3C = (SCLH_I3C + 1) x tI3CCLK SCLH_I3C is used to generate both tHIGH (I3C) and tHIGH_MIXED timings."]
pub type SCLH_I3C_R = crate::FieldReader;
#[doc = "Field `SCLH_I3C` writer - SCL high duration, used for I3C messages (both in push-pull and open-drain phases), in number of kernel clocks cycles: tSCLH_I3C = (SCLH_I3C + 1) x tI3CCLK SCLH_I3C is used to generate both tHIGH (I3C) and tHIGH_MIXED timings."]
pub type SCLH_I3C_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SCLL_OD` reader - SCL low duration in open-drain phases, used for legacy I2C commands and for I3C open-drain phases (address header phase following a START, not a Repeated START), in number of kernel clocks cycles: tSCLL_OD = (SCLL_OD + 1) x tI3CCLK SCLL_OD is used to generate both tLOW (I2C) and tLOW_OD timings (max. of the two)."]
pub type SCLL_OD_R = crate::FieldReader;
#[doc = "Field `SCLL_OD` writer - SCL low duration in open-drain phases, used for legacy I2C commands and for I3C open-drain phases (address header phase following a START, not a Repeated START), in number of kernel clocks cycles: tSCLL_OD = (SCLL_OD + 1) x tI3CCLK SCLL_OD is used to generate both tLOW (I2C) and tLOW_OD timings (max. of the two)."]
pub type SCLL_OD_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SCLH_I2C` reader - SCL high duration, used for legacy I2C commands, in number of kernel clocks cycles: tSCLH_I2C = (SCLH_I2C + 1) x tI3CCLK SCLH_I2C is used to generate tHIGH (I2C) timing."]
pub type SCLH_I2C_R = crate::FieldReader;
#[doc = "Field `SCLH_I2C` writer - SCL high duration, used for legacy I2C commands, in number of kernel clocks cycles: tSCLH_I2C = (SCLH_I2C + 1) x tI3CCLK SCLH_I2C is used to generate tHIGH (I2C) timing."]
pub type SCLH_I2C_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - SCL low duration in I3C push-pull phases, in number of kernel clocks cycles: tSCLL_PP = (SCLL_PP + 1) x tI3CCLK SCLL_PP is used to generate tLOW (I3C) timing."]
    #[inline(always)]
    pub fn scll_pp(&self) -> SCLL_PP_R {
        SCLL_PP_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - SCL high duration, used for I3C messages (both in push-pull and open-drain phases), in number of kernel clocks cycles: tSCLH_I3C = (SCLH_I3C + 1) x tI3CCLK SCLH_I3C is used to generate both tHIGH (I3C) and tHIGH_MIXED timings."]
    #[inline(always)]
    pub fn sclh_i3c(&self) -> SCLH_I3C_R {
        SCLH_I3C_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - SCL low duration in open-drain phases, used for legacy I2C commands and for I3C open-drain phases (address header phase following a START, not a Repeated START), in number of kernel clocks cycles: tSCLL_OD = (SCLL_OD + 1) x tI3CCLK SCLL_OD is used to generate both tLOW (I2C) and tLOW_OD timings (max. of the two)."]
    #[inline(always)]
    pub fn scll_od(&self) -> SCLL_OD_R {
        SCLL_OD_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - SCL high duration, used for legacy I2C commands, in number of kernel clocks cycles: tSCLH_I2C = (SCLH_I2C + 1) x tI3CCLK SCLH_I2C is used to generate tHIGH (I2C) timing."]
    #[inline(always)]
    pub fn sclh_i2c(&self) -> SCLH_I2C_R {
        SCLH_I2C_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SCL low duration in I3C push-pull phases, in number of kernel clocks cycles: tSCLL_PP = (SCLL_PP + 1) x tI3CCLK SCLL_PP is used to generate tLOW (I3C) timing."]
    #[inline(always)]
    #[must_use]
    pub fn scll_pp(&mut self) -> SCLL_PP_W<I3C_TIMINGR0rs> {
        SCLL_PP_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - SCL high duration, used for I3C messages (both in push-pull and open-drain phases), in number of kernel clocks cycles: tSCLH_I3C = (SCLH_I3C + 1) x tI3CCLK SCLH_I3C is used to generate both tHIGH (I3C) and tHIGH_MIXED timings."]
    #[inline(always)]
    #[must_use]
    pub fn sclh_i3c(&mut self) -> SCLH_I3C_W<I3C_TIMINGR0rs> {
        SCLH_I3C_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - SCL low duration in open-drain phases, used for legacy I2C commands and for I3C open-drain phases (address header phase following a START, not a Repeated START), in number of kernel clocks cycles: tSCLL_OD = (SCLL_OD + 1) x tI3CCLK SCLL_OD is used to generate both tLOW (I2C) and tLOW_OD timings (max. of the two)."]
    #[inline(always)]
    #[must_use]
    pub fn scll_od(&mut self) -> SCLL_OD_W<I3C_TIMINGR0rs> {
        SCLL_OD_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - SCL high duration, used for legacy I2C commands, in number of kernel clocks cycles: tSCLH_I2C = (SCLH_I2C + 1) x tI3CCLK SCLH_I2C is used to generate tHIGH (I2C) timing."]
    #[inline(always)]
    #[must_use]
    pub fn sclh_i2c(&mut self) -> SCLH_I2C_W<I3C_TIMINGR0rs> {
        SCLH_I2C_W::new(self, 24)
    }
}
#[doc = "I3C timing register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i3c_timingr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i3c_timingr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3C_TIMINGR0rs;
impl crate::RegisterSpec for I3C_TIMINGR0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3c_timingr0::R`](R) reader structure"]
impl crate::Readable for I3C_TIMINGR0rs {}
#[doc = "`write(|w| ..)` method takes [`i3c_timingr0::W`](W) writer structure"]
impl crate::Writable for I3C_TIMINGR0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets I3C_TIMINGR0 to value 0"]
impl crate::Resettable for I3C_TIMINGR0rs {
    const RESET_VALUE: u32 = 0;
}
