#[doc = "Reader of register M1SR"]
pub type R = crate::R<u32, super::M1SR>;
#[doc = "Writer for register M1SR"]
pub type W = crate::W<u32, super::M1SR>;
#[doc = "Register M1SR `reset()`'s with value 0"]
impl crate::ResetValue for super::M1SR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ECCSEIE`"]
pub type ECCSEIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ECCSEIE`"]
pub struct ECCSEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ECCSEIE_W<'a> {
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
#[doc = "Reader of field `ECCDEIE`"]
pub type ECCDEIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ECCDEIE`"]
pub struct ECCDEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ECCDEIE_W<'a> {
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
#[doc = "Reader of field `ECCDEBWIE`"]
pub type ECCDEBWIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ECCDEBWIE`"]
pub struct ECCDEBWIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ECCDEBWIE_W<'a> {
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
#[doc = "Reader of field `ECCELEN`"]
pub type ECCELEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ECCELEN`"]
pub struct ECCELEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ECCELEN_W<'a> {
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
impl R {
    #[doc = "Bit 2 - ECC single error interrupt enable"]
    #[inline(always)]
    pub fn eccseie(&self) -> ECCSEIE_R {
        ECCSEIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ECC double error interrupt enable"]
    #[inline(always)]
    pub fn eccdeie(&self) -> ECCDEIE_R {
        ECCDEIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ECC double error on byte write (BW) interrupt enable"]
    #[inline(always)]
    pub fn eccdebwie(&self) -> ECCDEBWIE_R {
        ECCDEBWIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ECC error latching enable"]
    #[inline(always)]
    pub fn eccelen(&self) -> ECCELEN_R {
        ECCELEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - ECC single error interrupt enable"]
    #[inline(always)]
    pub fn eccseie(&mut self) -> ECCSEIE_W {
        ECCSEIE_W { w: self }
    }
    #[doc = "Bit 3 - ECC double error interrupt enable"]
    #[inline(always)]
    pub fn eccdeie(&mut self) -> ECCDEIE_W {
        ECCDEIE_W { w: self }
    }
    #[doc = "Bit 4 - ECC double error on byte write (BW) interrupt enable"]
    #[inline(always)]
    pub fn eccdebwie(&mut self) -> ECCDEBWIE_W {
        ECCDEBWIE_W { w: self }
    }
    #[doc = "Bit 5 - ECC error latching enable"]
    #[inline(always)]
    pub fn eccelen(&mut self) -> ECCELEN_W {
        ECCELEN_W { w: self }
    }
}
