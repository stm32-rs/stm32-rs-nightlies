#[doc = "Reader of register SECCFGR"]
pub type R = crate::R<u32, super::SECCFGR>;
#[doc = "Writer for register SECCFGR"]
pub type W = crate::W<u32, super::SECCFGR>;
#[doc = "Register SECCFGR `reset()`'s with value 0"]
impl crate::ResetValue for super::SECCFGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SRAM2SEC`"]
pub type SRAM2SEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SRAM2SEC`"]
pub struct SRAM2SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM2SEC_W<'a> {
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
#[doc = "Reader of field `CLASSBSEC`"]
pub type CLASSBSEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLASSBSEC`"]
pub struct CLASSBSEC_W<'a> {
    w: &'a mut W,
}
impl<'a> CLASSBSEC_W<'a> {
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
#[doc = "Reader of field `SYSCFGSEC`"]
pub type SYSCFGSEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCFGSEC`"]
pub struct SYSCFGSEC_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCFGSEC_W<'a> {
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
#[doc = "Reader of field `FPUSEC`"]
pub type FPUSEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FPUSEC`"]
pub struct FPUSEC_W<'a> {
    w: &'a mut W,
}
impl<'a> FPUSEC_W<'a> {
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
impl R {
    #[doc = "Bit 2 - SRAM2 security"]
    #[inline(always)]
    pub fn sram2sec(&self) -> SRAM2SEC_R {
        SRAM2SEC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - ClassB security"]
    #[inline(always)]
    pub fn classbsec(&self) -> CLASSBSEC_R {
        CLASSBSEC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - SYSCFG clock control security"]
    #[inline(always)]
    pub fn syscfgsec(&self) -> SYSCFGSEC_R {
        SYSCFGSEC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 3 - FPUSEC"]
    #[inline(always)]
    pub fn fpusec(&self) -> FPUSEC_R {
        FPUSEC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - SRAM2 security"]
    #[inline(always)]
    pub fn sram2sec(&mut self) -> SRAM2SEC_W {
        SRAM2SEC_W { w: self }
    }
    #[doc = "Bit 1 - ClassB security"]
    #[inline(always)]
    pub fn classbsec(&mut self) -> CLASSBSEC_W {
        CLASSBSEC_W { w: self }
    }
    #[doc = "Bit 0 - SYSCFG clock control security"]
    #[inline(always)]
    pub fn syscfgsec(&mut self) -> SYSCFGSEC_W {
        SYSCFGSEC_W { w: self }
    }
    #[doc = "Bit 3 - FPUSEC"]
    #[inline(always)]
    pub fn fpusec(&mut self) -> FPUSEC_W {
        FPUSEC_W { w: self }
    }
}
