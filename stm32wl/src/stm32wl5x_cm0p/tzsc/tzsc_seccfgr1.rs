#[doc = "Reader of register TZSC_SECCFGR1"]
pub type R = crate::R<u32, super::TZSC_SECCFGR1>;
#[doc = "Writer for register TZSC_SECCFGR1"]
pub type W = crate::W<u32, super::TZSC_SECCFGR1>;
#[doc = "Register TZSC_SECCFGR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::TZSC_SECCFGR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AESSEC`"]
pub type AESSEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AESSEC`"]
pub struct AESSEC_W<'a> {
    w: &'a mut W,
}
impl<'a> AESSEC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `RNGSEC`"]
pub type RNGSEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RNGSEC`"]
pub struct RNGSEC_W<'a> {
    w: &'a mut W,
}
impl<'a> RNGSEC_W<'a> {
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
#[doc = "Reader of field `PKASEC`"]
pub type PKASEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PKASEC`"]
pub struct PKASEC_W<'a> {
    w: &'a mut W,
}
impl<'a> PKASEC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - AESSEC"]
    #[inline(always)]
    pub fn aessec(&self) -> AESSEC_R {
        AESSEC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RNGSEC"]
    #[inline(always)]
    pub fn rngsec(&self) -> RNGSEC_R {
        RNGSEC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 13 - PKASEC"]
    #[inline(always)]
    pub fn pkasec(&self) -> PKASEC_R {
        PKASEC_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - AESSEC"]
    #[inline(always)]
    pub fn aessec(&mut self) -> AESSEC_W {
        AESSEC_W { w: self }
    }
    #[doc = "Bit 3 - RNGSEC"]
    #[inline(always)]
    pub fn rngsec(&mut self) -> RNGSEC_W {
        RNGSEC_W { w: self }
    }
    #[doc = "Bit 13 - PKASEC"]
    #[inline(always)]
    pub fn pkasec(&mut self) -> PKASEC_W {
        PKASEC_W { w: self }
    }
}
