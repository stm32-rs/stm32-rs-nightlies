#[doc = "Reader of register FMC_CSQCFGR3"]
pub type R = crate::R<u32, super::FMC_CSQCFGR3>;
#[doc = "Writer for register FMC_CSQCFGR3"]
pub type W = crate::W<u32, super::FMC_CSQCFGR3>;
#[doc = "Register FMC_CSQCFGR3 `reset()`'s with value 0"]
impl crate::ResetValue for super::FMC_CSQCFGR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SNBR`"]
pub type SNBR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SNBR`"]
pub struct SNBR_W<'a> {
    w: &'a mut W,
}
impl<'a> SNBR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "Reader of field `AC1T`"]
pub type AC1T_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AC1T`"]
pub struct AC1T_W<'a> {
    w: &'a mut W,
}
impl<'a> AC1T_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `AC2T`"]
pub type AC2T_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AC2T`"]
pub struct AC2T_W<'a> {
    w: &'a mut W,
}
impl<'a> AC2T_W<'a> {
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
#[doc = "Reader of field `AC3T`"]
pub type AC3T_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AC3T`"]
pub struct AC3T_W<'a> {
    w: &'a mut W,
}
impl<'a> AC3T_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `AC4T`"]
pub type AC4T_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AC4T`"]
pub struct AC4T_W<'a> {
    w: &'a mut W,
}
impl<'a> AC4T_W<'a> {
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
#[doc = "Reader of field `AC5T`"]
pub type AC5T_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AC5T`"]
pub struct AC5T_W<'a> {
    w: &'a mut W,
}
impl<'a> AC5T_W<'a> {
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
#[doc = "Reader of field `SDT`"]
pub type SDT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SDT`"]
pub struct SDT_W<'a> {
    w: &'a mut W,
}
impl<'a> SDT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `RAC1T`"]
pub type RAC1T_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RAC1T`"]
pub struct RAC1T_W<'a> {
    w: &'a mut W,
}
impl<'a> RAC1T_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `RAC2T`"]
pub type RAC2T_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RAC2T`"]
pub struct RAC2T_W<'a> {
    w: &'a mut W,
}
impl<'a> RAC2T_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:13 - SNBR"]
    #[inline(always)]
    pub fn snbr(&self) -> SNBR_R {
        SNBR_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - AC1T"]
    #[inline(always)]
    pub fn ac1t(&self) -> AC1T_R {
        AC1T_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - AC2T"]
    #[inline(always)]
    pub fn ac2t(&self) -> AC2T_R {
        AC2T_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - AC3T"]
    #[inline(always)]
    pub fn ac3t(&self) -> AC3T_R {
        AC3T_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - AC4T"]
    #[inline(always)]
    pub fn ac4t(&self) -> AC4T_R {
        AC4T_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - AC5T"]
    #[inline(always)]
    pub fn ac5t(&self) -> AC5T_R {
        AC5T_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - SDT"]
    #[inline(always)]
    pub fn sdt(&self) -> SDT_R {
        SDT_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - RAC1T"]
    #[inline(always)]
    pub fn rac1t(&self) -> RAC1T_R {
        RAC1T_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - RAC2T"]
    #[inline(always)]
    pub fn rac2t(&self) -> RAC2T_R {
        RAC2T_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 8:13 - SNBR"]
    #[inline(always)]
    pub fn snbr(&mut self) -> SNBR_W {
        SNBR_W { w: self }
    }
    #[doc = "Bit 16 - AC1T"]
    #[inline(always)]
    pub fn ac1t(&mut self) -> AC1T_W {
        AC1T_W { w: self }
    }
    #[doc = "Bit 17 - AC2T"]
    #[inline(always)]
    pub fn ac2t(&mut self) -> AC2T_W {
        AC2T_W { w: self }
    }
    #[doc = "Bit 18 - AC3T"]
    #[inline(always)]
    pub fn ac3t(&mut self) -> AC3T_W {
        AC3T_W { w: self }
    }
    #[doc = "Bit 19 - AC4T"]
    #[inline(always)]
    pub fn ac4t(&mut self) -> AC4T_W {
        AC4T_W { w: self }
    }
    #[doc = "Bit 20 - AC5T"]
    #[inline(always)]
    pub fn ac5t(&mut self) -> AC5T_W {
        AC5T_W { w: self }
    }
    #[doc = "Bit 21 - SDT"]
    #[inline(always)]
    pub fn sdt(&mut self) -> SDT_W {
        SDT_W { w: self }
    }
    #[doc = "Bit 22 - RAC1T"]
    #[inline(always)]
    pub fn rac1t(&mut self) -> RAC1T_W {
        RAC1T_W { w: self }
    }
    #[doc = "Bit 23 - RAC2T"]
    #[inline(always)]
    pub fn rac2t(&mut self) -> RAC2T_W {
        RAC2T_W { w: self }
    }
}
