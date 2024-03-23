#[doc = "Register `HWCFGR` reader"]
pub type R = crate::R<HWCFGRrs>;
#[doc = "Register `HWCFGR` writer"]
pub type W = crate::W<HWCFGRrs>;
#[doc = "Field `ALARMB` reader - ALARMB"]
pub type ALARMB_R = crate::FieldReader;
#[doc = "Field `ALARMB` writer - ALARMB"]
pub type ALARMB_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WAKEUP` reader - WAKEUP"]
pub type WAKEUP_R = crate::FieldReader;
#[doc = "Field `WAKEUP` writer - WAKEUP"]
pub type WAKEUP_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SMOOTH_CALIB` reader - SMOOTH_CALIB"]
pub type SMOOTH_CALIB_R = crate::FieldReader;
#[doc = "Field `SMOOTH_CALIB` writer - SMOOTH_CALIB"]
pub type SMOOTH_CALIB_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TIMESTAMP` reader - TIMESTAMP"]
pub type TIMESTAMP_R = crate::FieldReader;
#[doc = "Field `TIMESTAMP` writer - TIMESTAMP"]
pub type TIMESTAMP_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `OPTIONREG_OUT` reader - OPTIONREG_OUT"]
pub type OPTIONREG_OUT_R = crate::FieldReader;
#[doc = "Field `OPTIONREG_OUT` writer - OPTIONREG_OUT"]
pub type OPTIONREG_OUT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TRUST_ZONE` reader - TRUST_ZONE"]
pub type TRUST_ZONE_R = crate::FieldReader;
#[doc = "Field `TRUST_ZONE` writer - TRUST_ZONE"]
pub type TRUST_ZONE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - ALARMB"]
    #[inline(always)]
    pub fn alarmb(&self) -> ALARMB_R {
        ALARMB_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - WAKEUP"]
    #[inline(always)]
    pub fn wakeup(&self) -> WAKEUP_R {
        WAKEUP_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - SMOOTH_CALIB"]
    #[inline(always)]
    pub fn smooth_calib(&self) -> SMOOTH_CALIB_R {
        SMOOTH_CALIB_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - TIMESTAMP"]
    #[inline(always)]
    pub fn timestamp(&self) -> TIMESTAMP_R {
        TIMESTAMP_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - OPTIONREG_OUT"]
    #[inline(always)]
    pub fn optionreg_out(&self) -> OPTIONREG_OUT_R {
        OPTIONREG_OUT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:27 - TRUST_ZONE"]
    #[inline(always)]
    pub fn trust_zone(&self) -> TRUST_ZONE_R {
        TRUST_ZONE_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - ALARMB"]
    #[inline(always)]
    #[must_use]
    pub fn alarmb(&mut self) -> ALARMB_W<HWCFGRrs> {
        ALARMB_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - WAKEUP"]
    #[inline(always)]
    #[must_use]
    pub fn wakeup(&mut self) -> WAKEUP_W<HWCFGRrs> {
        WAKEUP_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - SMOOTH_CALIB"]
    #[inline(always)]
    #[must_use]
    pub fn smooth_calib(&mut self) -> SMOOTH_CALIB_W<HWCFGRrs> {
        SMOOTH_CALIB_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - TIMESTAMP"]
    #[inline(always)]
    #[must_use]
    pub fn timestamp(&mut self) -> TIMESTAMP_W<HWCFGRrs> {
        TIMESTAMP_W::new(self, 12)
    }
    #[doc = "Bits 16:23 - OPTIONREG_OUT"]
    #[inline(always)]
    #[must_use]
    pub fn optionreg_out(&mut self) -> OPTIONREG_OUT_W<HWCFGRrs> {
        OPTIONREG_OUT_W::new(self, 16)
    }
    #[doc = "Bits 24:27 - TRUST_ZONE"]
    #[inline(always)]
    #[must_use]
    pub fn trust_zone(&mut self) -> TRUST_ZONE_W<HWCFGRrs> {
        TRUST_ZONE_W::new(self, 24)
    }
}
#[doc = "hardware configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwcfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hwcfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HWCFGRrs;
impl crate::RegisterSpec for HWCFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwcfgr::R`](R) reader structure"]
impl crate::Readable for HWCFGRrs {}
#[doc = "`write(|w| ..)` method takes [`hwcfgr::W`](W) writer structure"]
impl crate::Writable for HWCFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HWCFGR to value 0"]
impl crate::Resettable for HWCFGRrs {
    const RESET_VALUE: u32 = 0;
}
