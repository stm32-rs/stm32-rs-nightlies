#[doc = "Reader of register DDRCTRL_PWRTMG"]
pub type R = crate::R<u32, super::DDRCTRL_PWRTMG>;
#[doc = "Writer for register DDRCTRL_PWRTMG"]
pub type W = crate::W<u32, super::DDRCTRL_PWRTMG>;
#[doc = "Register DDRCTRL_PWRTMG `reset()`'s with value 0x0040_2010"]
impl crate::ResetValue for super::DDRCTRL_PWRTMG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0040_2010
    }
}
#[doc = "Reader of field `POWERDOWN_TO_X32`"]
pub type POWERDOWN_TO_X32_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `POWERDOWN_TO_X32`"]
pub struct POWERDOWN_TO_X32_W<'a> {
    w: &'a mut W,
}
impl<'a> POWERDOWN_TO_X32_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `T_DPD_X4096`"]
pub type T_DPD_X4096_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `T_DPD_X4096`"]
pub struct T_DPD_X4096_W<'a> {
    w: &'a mut W,
}
impl<'a> T_DPD_X4096_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `SELFREF_TO_X32`"]
pub type SELFREF_TO_X32_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SELFREF_TO_X32`"]
pub struct SELFREF_TO_X32_W<'a> {
    w: &'a mut W,
}
impl<'a> SELFREF_TO_X32_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
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
    pub fn powerdown_to_x32(&mut self) -> POWERDOWN_TO_X32_W {
        POWERDOWN_TO_X32_W { w: self }
    }
    #[doc = "Bits 8:15 - T_DPD_X4096"]
    #[inline(always)]
    pub fn t_dpd_x4096(&mut self) -> T_DPD_X4096_W {
        T_DPD_X4096_W { w: self }
    }
    #[doc = "Bits 16:23 - SELFREF_TO_X32"]
    #[inline(always)]
    pub fn selfref_to_x32(&mut self) -> SELFREF_TO_X32_W {
        SELFREF_TO_X32_W { w: self }
    }
}
