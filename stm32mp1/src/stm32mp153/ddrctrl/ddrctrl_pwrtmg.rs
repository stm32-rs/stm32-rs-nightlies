#[doc = "Register `DDRCTRL_PWRTMG` reader"]
pub type R = crate::R<DDRCTRL_PWRTMGrs>;
#[doc = "Register `DDRCTRL_PWRTMG` writer"]
pub type W = crate::W<DDRCTRL_PWRTMGrs>;
#[doc = "Field `POWERDOWN_TO_X32` reader - POWERDOWN_TO_X32"]
pub type POWERDOWN_TO_X32_R = crate::FieldReader;
#[doc = "Field `POWERDOWN_TO_X32` writer - POWERDOWN_TO_X32"]
pub type POWERDOWN_TO_X32_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `T_DPD_X4096` reader - T_DPD_X4096"]
pub type T_DPD_X4096_R = crate::FieldReader;
#[doc = "Field `T_DPD_X4096` writer - T_DPD_X4096"]
pub type T_DPD_X4096_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SELFREF_TO_X32` reader - SELFREF_TO_X32"]
pub type SELFREF_TO_X32_R = crate::FieldReader;
#[doc = "Field `SELFREF_TO_X32` writer - SELFREF_TO_X32"]
pub type SELFREF_TO_X32_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:4 - POWERDOWN_TO_X32"]
    #[inline(always)]
    pub fn powerdown_to_x32(&self) -> POWERDOWN_TO_X32_R {
        POWERDOWN_TO_X32_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:15 - T_DPD_X4096"]
    #[inline(always)]
    pub fn t_dpd_x4096(&self) -> T_DPD_X4096_R {
        T_DPD_X4096_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - SELFREF_TO_X32"]
    #[inline(always)]
    pub fn selfref_to_x32(&self) -> SELFREF_TO_X32_R {
        SELFREF_TO_X32_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - POWERDOWN_TO_X32"]
    #[inline(always)]
    #[must_use]
    pub fn powerdown_to_x32(&mut self) -> POWERDOWN_TO_X32_W<DDRCTRL_PWRTMGrs> {
        POWERDOWN_TO_X32_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - T_DPD_X4096"]
    #[inline(always)]
    #[must_use]
    pub fn t_dpd_x4096(&mut self) -> T_DPD_X4096_W<DDRCTRL_PWRTMGrs> {
        T_DPD_X4096_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - SELFREF_TO_X32"]
    #[inline(always)]
    #[must_use]
    pub fn selfref_to_x32(&mut self) -> SELFREF_TO_X32_W<DDRCTRL_PWRTMGrs> {
        SELFREF_TO_X32_W::new(self, 16)
    }
}
#[doc = "DDRCTRL low power timing register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_pwrtmg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_pwrtmg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRCTRL_PWRTMGrs;
impl crate::RegisterSpec for DDRCTRL_PWRTMGrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrctrl_pwrtmg::R`](R) reader structure"]
impl crate::Readable for DDRCTRL_PWRTMGrs {}
#[doc = "`write(|w| ..)` method takes [`ddrctrl_pwrtmg::W`](W) writer structure"]
impl crate::Writable for DDRCTRL_PWRTMGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRCTRL_PWRTMG to value 0x0040_2010"]
impl crate::Resettable for DDRCTRL_PWRTMGrs {
    const RESET_VALUE: u32 = 0x0040_2010;
}
