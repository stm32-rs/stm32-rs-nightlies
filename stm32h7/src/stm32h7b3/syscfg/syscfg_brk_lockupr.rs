#[doc = "Reader of register SYSCFG_BRK_LOCKUPR"]
pub type R = crate::R<u32, super::SYSCFG_BRK_LOCKUPR>;
#[doc = "Writer for register SYSCFG_BRK_LOCKUPR"]
pub type W = crate::W<u32, super::SYSCFG_BRK_LOCKUPR>;
#[doc = "Register SYSCFG_BRK_LOCKUPR `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSCFG_BRK_LOCKUPR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PVDL`"]
pub type PVDL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PVDL`"]
pub struct PVDL_W<'a> {
    w: &'a mut W,
}
impl<'a> PVDL_W<'a> {
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
#[doc = "Reader of field `FLASHL`"]
pub type FLASHL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLASHL`"]
pub struct FLASHL_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASHL_W<'a> {
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
#[doc = "Reader of field `CM7L`"]
pub type CM7L_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CM7L`"]
pub struct CM7L_W<'a> {
    w: &'a mut W,
}
impl<'a> CM7L_W<'a> {
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
#[doc = "Reader of field `DTCML`"]
pub type DTCML_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DTCML`"]
pub struct DTCML_W<'a> {
    w: &'a mut W,
}
impl<'a> DTCML_W<'a> {
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
#[doc = "Reader of field `ITCML`"]
pub type ITCML_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ITCML`"]
pub struct ITCML_W<'a> {
    w: &'a mut W,
}
impl<'a> ITCML_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - PVD lock enable bit."]
    #[inline(always)]
    pub fn pvdl(&self) -> PVDL_R {
        PVDL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Flash double ECC error lock bit"]
    #[inline(always)]
    pub fn flashl(&self) -> FLASHL_R {
        FLASHL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 6 - CortexÃ‚Â®-M7 LOCKUP (HardFault) output enable bit"]
    #[inline(always)]
    pub fn cm7l(&self) -> CM7L_R {
        CM7L_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 13 - D1TCM or D0TCM double ECC error signal lock"]
    #[inline(always)]
    pub fn dtcml(&self) -> DTCML_R {
        DTCML_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - ITCM double ECC error signal lock"]
    #[inline(always)]
    pub fn itcml(&self) -> ITCML_R {
        ITCML_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - PVD lock enable bit."]
    #[inline(always)]
    pub fn pvdl(&mut self) -> PVDL_W {
        PVDL_W { w: self }
    }
    #[doc = "Bit 3 - Flash double ECC error lock bit"]
    #[inline(always)]
    pub fn flashl(&mut self) -> FLASHL_W {
        FLASHL_W { w: self }
    }
    #[doc = "Bit 6 - CortexÃ‚Â®-M7 LOCKUP (HardFault) output enable bit"]
    #[inline(always)]
    pub fn cm7l(&mut self) -> CM7L_W {
        CM7L_W { w: self }
    }
    #[doc = "Bit 13 - D1TCM or D0TCM double ECC error signal lock"]
    #[inline(always)]
    pub fn dtcml(&mut self) -> DTCML_W {
        DTCML_W { w: self }
    }
    #[doc = "Bit 14 - ITCM double ECC error signal lock"]
    #[inline(always)]
    pub fn itcml(&mut self) -> ITCML_W {
        ITCML_W { w: self }
    }
}
