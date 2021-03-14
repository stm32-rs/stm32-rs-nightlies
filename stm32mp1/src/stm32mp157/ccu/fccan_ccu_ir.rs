#[doc = "Reader of register FCCAN_CCU_IR"]
pub type R = crate::R<u32, super::FCCAN_CCU_IR>;
#[doc = "Writer for register FCCAN_CCU_IR"]
pub type W = crate::W<u32, super::FCCAN_CCU_IR>;
#[doc = "Register FCCAN_CCU_IR `reset()`'s with value 0"]
impl crate::ResetValue for super::FCCAN_CCU_IR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CWE`"]
pub type CWE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CWE`"]
pub struct CWE_W<'a> {
    w: &'a mut W,
}
impl<'a> CWE_W<'a> {
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
#[doc = "Reader of field `CSC`"]
pub type CSC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CSC`"]
pub struct CSC_W<'a> {
    w: &'a mut W,
}
impl<'a> CSC_W<'a> {
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
    #[doc = "Bit 0 - CWE"]
    #[inline(always)]
    pub fn cwe(&self) -> CWE_R {
        CWE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CSC"]
    #[inline(always)]
    pub fn csc(&self) -> CSC_R {
        CSC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CWE"]
    #[inline(always)]
    pub fn cwe(&mut self) -> CWE_W {
        CWE_W { w: self }
    }
    #[doc = "Bit 1 - CSC"]
    #[inline(always)]
    pub fn csc(&mut self) -> CSC_W {
        CSC_W { w: self }
    }
}
