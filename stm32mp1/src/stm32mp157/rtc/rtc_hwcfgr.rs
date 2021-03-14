#[doc = "Reader of register RTC_HWCFGR"]
pub type R = crate::R<u32, super::RTC_HWCFGR>;
#[doc = "Reader of field `ALARMB`"]
pub type ALARMB_R = crate::R<u8, u8>;
#[doc = "Reader of field `WAKEUP`"]
pub type WAKEUP_R = crate::R<u8, u8>;
#[doc = "Reader of field `SMOOTH_CALIB`"]
pub type SMOOTH_CALIB_R = crate::R<u8, u8>;
#[doc = "Reader of field `TIMESTAMP`"]
pub type TIMESTAMP_R = crate::R<u8, u8>;
#[doc = "Reader of field `OPTIONREG_OUT`"]
pub type OPTIONREG_OUT_R = crate::R<u8, u8>;
#[doc = "Reader of field `TRUST_ZONE`"]
pub type TRUST_ZONE_R = crate::R<u8, u8>;
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
