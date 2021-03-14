#[doc = "Reader of register RTC_CFGR"]
pub type R = crate::R<u32, super::RTC_CFGR>;
#[doc = "Writer for register RTC_CFGR"]
pub type W = crate::W<u32, super::RTC_CFGR>;
#[doc = "Register RTC_CFGR `reset()`'s with value 0"]
impl crate::ResetValue for super::RTC_CFGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OUT2_RMP`"]
pub type OUT2_RMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OUT2_RMP`"]
pub struct OUT2_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT2_RMP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `LSCOEN`"]
pub type LSCOEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LSCOEN`"]
pub struct LSCOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LSCOEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - OUT2_RMP"]
    #[inline(always)]
    pub fn out2_rmp(&self) -> OUT2_RMP_R {
        OUT2_RMP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - LSCOEN"]
    #[inline(always)]
    pub fn lscoen(&self) -> LSCOEN_R {
        LSCOEN_R::new(((self.bits >> 1) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - OUT2_RMP"]
    #[inline(always)]
    pub fn out2_rmp(&mut self) -> OUT2_RMP_W {
        OUT2_RMP_W { w: self }
    }
    #[doc = "Bits 1:2 - LSCOEN"]
    #[inline(always)]
    pub fn lscoen(&mut self) -> LSCOEN_W {
        LSCOEN_W { w: self }
    }
}
