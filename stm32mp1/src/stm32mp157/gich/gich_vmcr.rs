#[doc = "Reader of register GICH_VMCR"]
pub type R = crate::R<u32, super::GICH_VMCR>;
#[doc = "Writer for register GICH_VMCR"]
pub type W = crate::W<u32, super::GICH_VMCR>;
#[doc = "Register GICH_VMCR `reset()`'s with value 0x004d_0000"]
impl crate::ResetValue for super::GICH_VMCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x004d_0000
    }
}
#[doc = "Reader of field `VMGRP0EN`"]
pub type VMGRP0EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VMGRP0EN`"]
pub struct VMGRP0EN_W<'a> {
    w: &'a mut W,
}
impl<'a> VMGRP0EN_W<'a> {
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
#[doc = "Reader of field `VMGRP1EN`"]
pub type VMGRP1EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VMGRP1EN`"]
pub struct VMGRP1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> VMGRP1EN_W<'a> {
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
#[doc = "Reader of field `VMACKCTL`"]
pub type VMACKCTL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VMACKCTL`"]
pub struct VMACKCTL_W<'a> {
    w: &'a mut W,
}
impl<'a> VMACKCTL_W<'a> {
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
#[doc = "Reader of field `VMFIQEN`"]
pub type VMFIQEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VMFIQEN`"]
pub struct VMFIQEN_W<'a> {
    w: &'a mut W,
}
impl<'a> VMFIQEN_W<'a> {
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
#[doc = "Reader of field `VMCBPR`"]
pub type VMCBPR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VMCBPR`"]
pub struct VMCBPR_W<'a> {
    w: &'a mut W,
}
impl<'a> VMCBPR_W<'a> {
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
#[doc = "Reader of field `VEM`"]
pub type VEM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VEM`"]
pub struct VEM_W<'a> {
    w: &'a mut W,
}
impl<'a> VEM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `VMABP`"]
pub type VMABP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VMABP`"]
pub struct VMABP_W<'a> {
    w: &'a mut W,
}
impl<'a> VMABP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | (((value as u32) & 0x07) << 18);
        self.w
    }
}
#[doc = "Reader of field `VMBP`"]
pub type VMBP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VMBP`"]
pub struct VMBP_W<'a> {
    w: &'a mut W,
}
impl<'a> VMBP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 21)) | (((value as u32) & 0x07) << 21);
        self.w
    }
}
#[doc = "Reader of field `VMPRIMASK`"]
pub type VMPRIMASK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VMPRIMASK`"]
pub struct VMPRIMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> VMPRIMASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 27)) | (((value as u32) & 0x1f) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - VMGRP0EN"]
    #[inline(always)]
    pub fn vmgrp0en(&self) -> VMGRP0EN_R {
        VMGRP0EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - VMGRP1EN"]
    #[inline(always)]
    pub fn vmgrp1en(&self) -> VMGRP1EN_R {
        VMGRP1EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - VMACKCTL"]
    #[inline(always)]
    pub fn vmackctl(&self) -> VMACKCTL_R {
        VMACKCTL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - VMFIQEN"]
    #[inline(always)]
    pub fn vmfiqen(&self) -> VMFIQEN_R {
        VMFIQEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - VMCBPR"]
    #[inline(always)]
    pub fn vmcbpr(&self) -> VMCBPR_R {
        VMCBPR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 9 - VEM"]
    #[inline(always)]
    pub fn vem(&self) -> VEM_R {
        VEM_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 18:20 - VMABP"]
    #[inline(always)]
    pub fn vmabp(&self) -> VMABP_R {
        VMABP_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bits 21:23 - VMBP"]
    #[inline(always)]
    pub fn vmbp(&self) -> VMBP_R {
        VMBP_R::new(((self.bits >> 21) & 0x07) as u8)
    }
    #[doc = "Bits 27:31 - VMPRIMASK"]
    #[inline(always)]
    pub fn vmprimask(&self) -> VMPRIMASK_R {
        VMPRIMASK_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - VMGRP0EN"]
    #[inline(always)]
    pub fn vmgrp0en(&mut self) -> VMGRP0EN_W {
        VMGRP0EN_W { w: self }
    }
    #[doc = "Bit 1 - VMGRP1EN"]
    #[inline(always)]
    pub fn vmgrp1en(&mut self) -> VMGRP1EN_W {
        VMGRP1EN_W { w: self }
    }
    #[doc = "Bit 2 - VMACKCTL"]
    #[inline(always)]
    pub fn vmackctl(&mut self) -> VMACKCTL_W {
        VMACKCTL_W { w: self }
    }
    #[doc = "Bit 3 - VMFIQEN"]
    #[inline(always)]
    pub fn vmfiqen(&mut self) -> VMFIQEN_W {
        VMFIQEN_W { w: self }
    }
    #[doc = "Bit 4 - VMCBPR"]
    #[inline(always)]
    pub fn vmcbpr(&mut self) -> VMCBPR_W {
        VMCBPR_W { w: self }
    }
    #[doc = "Bit 9 - VEM"]
    #[inline(always)]
    pub fn vem(&mut self) -> VEM_W {
        VEM_W { w: self }
    }
    #[doc = "Bits 18:20 - VMABP"]
    #[inline(always)]
    pub fn vmabp(&mut self) -> VMABP_W {
        VMABP_W { w: self }
    }
    #[doc = "Bits 21:23 - VMBP"]
    #[inline(always)]
    pub fn vmbp(&mut self) -> VMBP_W {
        VMBP_W { w: self }
    }
    #[doc = "Bits 27:31 - VMPRIMASK"]
    #[inline(always)]
    pub fn vmprimask(&mut self) -> VMPRIMASK_W {
        VMPRIMASK_W { w: self }
    }
}
