#[doc = "Reader of register FPR2"]
pub type R = crate::R<u32, super::FPR2>;
#[doc = "Writer for register FPR2"]
pub type W = crate::W<u32, super::FPR2>;
#[doc = "Register FPR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::FPR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FPIF35`"]
pub type FPIF35_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FPIF35`"]
pub struct FPIF35_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIF35_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `FPIF36`"]
pub type FPIF36_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FPIF36`"]
pub struct FPIF36_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIF36_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `FPIF37`"]
pub type FPIF37_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FPIF37`"]
pub struct FPIF37_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIF37_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `FPIF38`"]
pub type FPIF38_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FPIF38`"]
pub struct FPIF38_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIF38_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bit 3 - FPIF35"]
    #[inline(always)]
    pub fn fpif35(&self) -> FPIF35_R {
        FPIF35_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - FPIF36"]
    #[inline(always)]
    pub fn fpif36(&self) -> FPIF36_R {
        FPIF36_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - FPIF37"]
    #[inline(always)]
    pub fn fpif37(&self) -> FPIF37_R {
        FPIF37_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - FPIF38"]
    #[inline(always)]
    pub fn fpif38(&self) -> FPIF38_R {
        FPIF38_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - FPIF35"]
    #[inline(always)]
    pub fn fpif35(&mut self) -> FPIF35_W {
        FPIF35_W { w: self }
    }
    #[doc = "Bit 4 - FPIF36"]
    #[inline(always)]
    pub fn fpif36(&mut self) -> FPIF36_W {
        FPIF36_W { w: self }
    }
    #[doc = "Bit 5 - FPIF37"]
    #[inline(always)]
    pub fn fpif37(&mut self) -> FPIF37_W {
        FPIF37_W { w: self }
    }
    #[doc = "Bit 6 - FPIF38"]
    #[inline(always)]
    pub fn fpif38(&mut self) -> FPIF38_W {
        FPIF38_W { w: self }
    }
}
