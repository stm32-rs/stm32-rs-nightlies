#[doc = "Reader of register HWCFGR"]
pub type R = crate::R<u32, super::HWCFGR>;
#[doc = "Writer for register HWCFGR"]
pub type W = crate::W<u32, super::HWCFGR>;
#[doc = "Register HWCFGR `reset()`'s with value 0"]
impl crate::ResetValue for super::HWCFGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ALARMB`"]
pub type ALARMB_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ALARMB`"]
pub struct ALARMB_W<'a> {
    w: &'a mut W,
}
impl<'a> ALARMB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `WAKEUP`"]
pub type WAKEUP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WAKEUP`"]
pub struct WAKEUP_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `SMOOTH_CALIB`"]
pub type SMOOTH_CALIB_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SMOOTH_CALIB`"]
pub struct SMOOTH_CALIB_W<'a> {
    w: &'a mut W,
}
impl<'a> SMOOTH_CALIB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `TIMESTAMP`"]
pub type TIMESTAMP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TIMESTAMP`"]
pub struct TIMESTAMP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMESTAMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `OPTIONREG_OUT`"]
pub type OPTIONREG_OUT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OPTIONREG_OUT`"]
pub struct OPTIONREG_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> OPTIONREG_OUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `TRUST_ZONE`"]
pub type TRUST_ZONE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRUST_ZONE`"]
pub struct TRUST_ZONE_W<'a> {
    w: &'a mut W,
}
impl<'a> TRUST_ZONE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
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
    pub fn alarmb(&mut self) -> ALARMB_W {
        ALARMB_W { w: self }
    }
    #[doc = "Bits 4:7 - WAKEUP"]
    #[inline(always)]
    pub fn wakeup(&mut self) -> WAKEUP_W {
        WAKEUP_W { w: self }
    }
    #[doc = "Bits 8:11 - SMOOTH_CALIB"]
    #[inline(always)]
    pub fn smooth_calib(&mut self) -> SMOOTH_CALIB_W {
        SMOOTH_CALIB_W { w: self }
    }
    #[doc = "Bits 12:15 - TIMESTAMP"]
    #[inline(always)]
    pub fn timestamp(&mut self) -> TIMESTAMP_W {
        TIMESTAMP_W { w: self }
    }
    #[doc = "Bits 16:23 - OPTIONREG_OUT"]
    #[inline(always)]
    pub fn optionreg_out(&mut self) -> OPTIONREG_OUT_W {
        OPTIONREG_OUT_W { w: self }
    }
    #[doc = "Bits 24:27 - TRUST_ZONE"]
    #[inline(always)]
    pub fn trust_zone(&mut self) -> TRUST_ZONE_W {
        TRUST_ZONE_W { w: self }
    }
}
