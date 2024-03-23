#[doc = "Register `HWCFGR` reader"]
pub type R = crate::R<HWCFGRrs>;
#[doc = "Field `ALARMB` reader - ALARMB"]
pub type ALARMB_R = crate::FieldReader;
#[doc = "Field `WAKEUP` reader - WAKEUP"]
pub type WAKEUP_R = crate::FieldReader;
#[doc = "Field `SMOOTH_CALIB` reader - SMOOTH_CALIB"]
pub type SMOOTH_CALIB_R = crate::FieldReader;
#[doc = "Field `TIMESTAMP` reader - TIMESTAMP"]
pub type TIMESTAMP_R = crate::FieldReader;
#[doc = "Field `OPTIONREG_OUT` reader - OPTIONREG_OUT"]
pub type OPTIONREG_OUT_R = crate::FieldReader;
#[doc = "Field `TRUST_ZONE` reader - TRUST_ZONE"]
pub type TRUST_ZONE_R = crate::FieldReader;
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
#[doc = "RTC hardware configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwcfgr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HWCFGRrs;
impl crate::RegisterSpec for HWCFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwcfgr::R`](R) reader structure"]
impl crate::Readable for HWCFGRrs {}
#[doc = "`reset()` method sets HWCFGR to value 0x0103_1111"]
impl crate::Resettable for HWCFGRrs {
    const RESET_VALUE: u32 = 0x0103_1111;
}
