#[doc = "Reader of register FCCAN_CCU_IE"]
pub type R = crate::R<u32, super::FCCAN_CCU_IE>;
#[doc = "Writer for register FCCAN_CCU_IE"]
pub type W = crate::W<u32, super::FCCAN_CCU_IE>;
#[doc = "Register FCCAN_CCU_IE `reset()`'s with value 0"]
impl crate::ResetValue for super::FCCAN_CCU_IE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CWEE`"]
pub type CWEE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CWEE`"]
pub struct CWEE_W<'a> {
    w: &'a mut W,
}
impl<'a> CWEE_W<'a> {
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
#[doc = "Reader of field `CSCE`"]
pub type CSCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CSCE`"]
pub struct CSCE_W<'a> {
    w: &'a mut W,
}
impl<'a> CSCE_W<'a> {
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
impl R {
    #[doc = "Bit 0 - CWEE"]
    #[inline(always)]
    pub fn cwee(&self) -> CWEE_R {
        CWEE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CSCE"]
    #[inline(always)]
    pub fn csce(&self) -> CSCE_R {
        CSCE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CWEE"]
    #[inline(always)]
    pub fn cwee(&mut self) -> CWEE_W {
        CWEE_W { w: self }
    }
    #[doc = "Bit 1 - CSCE"]
    #[inline(always)]
    pub fn csce(&mut self) -> CSCE_W {
        CSCE_W { w: self }
    }
}
