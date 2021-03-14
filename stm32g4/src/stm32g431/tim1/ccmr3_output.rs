#[doc = "Reader of register CCMR3_Output"]
pub type R = crate::R<u32, super::CCMR3_OUTPUT>;
#[doc = "Writer for register CCMR3_Output"]
pub type W = crate::W<u32, super::CCMR3_OUTPUT>;
#[doc = "Register CCMR3_Output `reset()`'s with value 0"]
impl crate::ResetValue for super::CCMR3_OUTPUT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OC6M_bit3`"]
pub type OC6M_BIT3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OC6M_bit3`"]
pub struct OC6M_BIT3_W<'a> {
    w: &'a mut W,
}
impl<'a> OC6M_BIT3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `OC5M_bit3`"]
pub type OC5M_BIT3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OC5M_bit3`"]
pub struct OC5M_BIT3_W<'a> {
    w: &'a mut W,
}
impl<'a> OC5M_BIT3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Reader of field `OC6CE`"]
pub type OC6CE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OC6CE`"]
pub struct OC6CE_W<'a> {
    w: &'a mut W,
}
impl<'a> OC6CE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `OC6M`"]
pub type OC6M_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OC6M`"]
pub struct OC6M_W<'a> {
    w: &'a mut W,
}
impl<'a> OC6M_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Reader of field `OC6PE`"]
pub type OC6PE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OC6PE`"]
pub struct OC6PE_W<'a> {
    w: &'a mut W,
}
impl<'a> OC6PE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `OC6FE`"]
pub type OC6FE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OC6FE`"]
pub struct OC6FE_W<'a> {
    w: &'a mut W,
}
impl<'a> OC6FE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `OC5CE`"]
pub type OC5CE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OC5CE`"]
pub struct OC5CE_W<'a> {
    w: &'a mut W,
}
impl<'a> OC5CE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `OC5M`"]
pub type OC5M_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OC5M`"]
pub struct OC5M_W<'a> {
    w: &'a mut W,
}
impl<'a> OC5M_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `OC5PE`"]
pub type OC5PE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OC5PE`"]
pub struct OC5PE_W<'a> {
    w: &'a mut W,
}
impl<'a> OC5PE_W<'a> {
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
#[doc = "Reader of field `OC5FE`"]
pub type OC5FE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OC5FE`"]
pub struct OC5FE_W<'a> {
    w: &'a mut W,
}
impl<'a> OC5FE_W<'a> {
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
impl R {
    #[doc = "Bit 24 - Output Compare 6 mode bit 3"]
    #[inline(always)]
    pub fn oc6m_bit3(&self) -> OC6M_BIT3_R {
        OC6M_BIT3_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 16:18 - Output Compare 5 mode bit 3"]
    #[inline(always)]
    pub fn oc5m_bit3(&self) -> OC5M_BIT3_R {
        OC5M_BIT3_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 15 - Output compare 6 clear enable"]
    #[inline(always)]
    pub fn oc6ce(&self) -> OC6CE_R {
        OC6CE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 12:14 - Output compare 6 mode"]
    #[inline(always)]
    pub fn oc6m(&self) -> OC6M_R {
        OC6M_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bit 11 - Output compare 6 preload enable"]
    #[inline(always)]
    pub fn oc6pe(&self) -> OC6PE_R {
        OC6PE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Output compare 6 fast enable"]
    #[inline(always)]
    pub fn oc6fe(&self) -> OC6FE_R {
        OC6FE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Output compare 5 clear enable"]
    #[inline(always)]
    pub fn oc5ce(&self) -> OC5CE_R {
        OC5CE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - Output compare 5 mode"]
    #[inline(always)]
    pub fn oc5m(&self) -> OC5M_R {
        OC5M_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 3 - Output compare 5 preload enable"]
    #[inline(always)]
    pub fn oc5pe(&self) -> OC5PE_R {
        OC5PE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Output compare 5 fast enable"]
    #[inline(always)]
    pub fn oc5fe(&self) -> OC5FE_R {
        OC5FE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - Output Compare 6 mode bit 3"]
    #[inline(always)]
    pub fn oc6m_bit3(&mut self) -> OC6M_BIT3_W {
        OC6M_BIT3_W { w: self }
    }
    #[doc = "Bits 16:18 - Output Compare 5 mode bit 3"]
    #[inline(always)]
    pub fn oc5m_bit3(&mut self) -> OC5M_BIT3_W {
        OC5M_BIT3_W { w: self }
    }
    #[doc = "Bit 15 - Output compare 6 clear enable"]
    #[inline(always)]
    pub fn oc6ce(&mut self) -> OC6CE_W {
        OC6CE_W { w: self }
    }
    #[doc = "Bits 12:14 - Output compare 6 mode"]
    #[inline(always)]
    pub fn oc6m(&mut self) -> OC6M_W {
        OC6M_W { w: self }
    }
    #[doc = "Bit 11 - Output compare 6 preload enable"]
    #[inline(always)]
    pub fn oc6pe(&mut self) -> OC6PE_W {
        OC6PE_W { w: self }
    }
    #[doc = "Bit 10 - Output compare 6 fast enable"]
    #[inline(always)]
    pub fn oc6fe(&mut self) -> OC6FE_W {
        OC6FE_W { w: self }
    }
    #[doc = "Bit 7 - Output compare 5 clear enable"]
    #[inline(always)]
    pub fn oc5ce(&mut self) -> OC5CE_W {
        OC5CE_W { w: self }
    }
    #[doc = "Bits 4:6 - Output compare 5 mode"]
    #[inline(always)]
    pub fn oc5m(&mut self) -> OC5M_W {
        OC5M_W { w: self }
    }
    #[doc = "Bit 3 - Output compare 5 preload enable"]
    #[inline(always)]
    pub fn oc5pe(&mut self) -> OC5PE_W {
        OC5PE_W { w: self }
    }
    #[doc = "Bit 2 - Output compare 5 fast enable"]
    #[inline(always)]
    pub fn oc5fe(&mut self) -> OC5FE_W {
        OC5FE_W { w: self }
    }
}
