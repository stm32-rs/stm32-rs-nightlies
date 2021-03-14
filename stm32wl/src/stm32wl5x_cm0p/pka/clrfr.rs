#[doc = "Writer for register CLRFR"]
pub type W = crate::W<u32, super::CLRFR>;
#[doc = "Register CLRFR `reset()`'s with value 0"]
impl crate::ResetValue for super::CLRFR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `ADDRERRFC`"]
pub struct ADDRERRFC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRERRFC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Write proxy for field `RAMERRFC`"]
pub struct RAMERRFC_W<'a> {
    w: &'a mut W,
}
impl<'a> RAMERRFC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Write proxy for field `PROCENDFC`"]
pub struct PROCENDFC_W<'a> {
    w: &'a mut W,
}
impl<'a> PROCENDFC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
impl W {
    #[doc = "Bit 20 - Clear Address error flag"]
    #[inline(always)]
    pub fn addrerrfc(&mut self) -> ADDRERRFC_W {
        ADDRERRFC_W { w: self }
    }
    #[doc = "Bit 19 - Clear PKA RAM error flag"]
    #[inline(always)]
    pub fn ramerrfc(&mut self) -> RAMERRFC_W {
        RAMERRFC_W { w: self }
    }
    #[doc = "Bit 17 - Clear PKA End of Operation flag"]
    #[inline(always)]
    pub fn procendfc(&mut self) -> PROCENDFC_W {
        PROCENDFC_W { w: self }
    }
}
