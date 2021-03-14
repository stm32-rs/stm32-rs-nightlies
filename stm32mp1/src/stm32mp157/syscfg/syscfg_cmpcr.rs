#[doc = "Reader of register SYSCFG_CMPCR"]
pub type R = crate::R<u32, super::SYSCFG_CMPCR>;
#[doc = "Writer for register SYSCFG_CMPCR"]
pub type W = crate::W<u32, super::SYSCFG_CMPCR>;
#[doc = "Register SYSCFG_CMPCR `reset()`'s with value 0x0087_0000"]
impl crate::ResetValue for super::SYSCFG_CMPCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0087_0000
    }
}
#[doc = "Reader of field `SW_CTRL`"]
pub type SW_CTRL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SW_CTRL`"]
pub struct SW_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_CTRL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `READY`"]
pub type READY_R = crate::R<bool, bool>;
#[doc = "Reader of field `RANSRC`"]
pub type RANSRC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RANSRC`"]
pub struct RANSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> RANSRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `RAPSRC`"]
pub type RAPSRC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RAPSRC`"]
pub struct RAPSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> RAPSRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `ANSRC`"]
pub type ANSRC_R = crate::R<u8, u8>;
#[doc = "Reader of field `APSRC`"]
pub type APSRC_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 1 - SW_CTRL"]
    #[inline(always)]
    pub fn sw_ctrl(&self) -> SW_CTRL_R {
        SW_CTRL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 8 - READY"]
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - RANSRC"]
    #[inline(always)]
    pub fn ransrc(&self) -> RANSRC_R {
        RANSRC_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - RAPSRC"]
    #[inline(always)]
    pub fn rapsrc(&self) -> RAPSRC_R {
        RAPSRC_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - ANSRC"]
    #[inline(always)]
    pub fn ansrc(&self) -> ANSRC_R {
        ANSRC_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - APSRC"]
    #[inline(always)]
    pub fn apsrc(&self) -> APSRC_R {
        APSRC_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - SW_CTRL"]
    #[inline(always)]
    pub fn sw_ctrl(&mut self) -> SW_CTRL_W {
        SW_CTRL_W { w: self }
    }
    #[doc = "Bits 16:19 - RANSRC"]
    #[inline(always)]
    pub fn ransrc(&mut self) -> RANSRC_W {
        RANSRC_W { w: self }
    }
    #[doc = "Bits 20:23 - RAPSRC"]
    #[inline(always)]
    pub fn rapsrc(&mut self) -> RAPSRC_W {
        RAPSRC_W { w: self }
    }
}
