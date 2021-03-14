#[doc = "Reader of register SRCR"]
pub type R = crate::R<u32, super::SRCR>;
#[doc = "Writer for register SRCR"]
pub type W = crate::W<u32, super::SRCR>;
#[doc = "Register SRCR `reset()`'s with value 0"]
impl crate::ResetValue for super::SRCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Vertical Blanking Reload\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VBR_A {
    #[doc = "1: The shadow registers are reloaded during the vertical blanking period (at the beginning of the first line after the active display area)."]
    RELOAD = 1,
    #[doc = "0: This bit is set by software and cleared only by hardware after reload (it cannot be cleared through register write once it is set)"]
    NOEFFECT = 0,
}
impl From<VBR_A> for bool {
    #[inline(always)]
    fn from(variant: VBR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VBR`"]
pub type VBR_R = crate::R<bool, VBR_A>;
impl VBR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VBR_A {
        match self.bits {
            true => VBR_A::RELOAD,
            false => VBR_A::NOEFFECT,
        }
    }
    #[doc = "Checks if the value of the field is `RELOAD`"]
    #[inline(always)]
    pub fn is_reload(&self) -> bool {
        *self == VBR_A::RELOAD
    }
    #[doc = "Checks if the value of the field is `NOEFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == VBR_A::NOEFFECT
    }
}
#[doc = "Write proxy for field `VBR`"]
pub struct VBR_W<'a> {
    w: &'a mut W,
}
impl<'a> VBR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VBR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The shadow registers are reloaded during the vertical blanking period (at the beginning of the first line after the active display area)."]
    #[inline(always)]
    pub fn reload(self) -> &'a mut W {
        self.variant(VBR_A::RELOAD)
    }
    #[doc = "This bit is set by software and cleared only by hardware after reload (it cannot be cleared through register write once it is set)"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(VBR_A::NOEFFECT)
    }
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
#[doc = "Immediate Reload\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IMR_A {
    #[doc = "1: The shadow registers are reloaded immediately. This bit is set by software and cleared only by hardware after reload"]
    RELOAD = 1,
    #[doc = "0: This bit is set by software and cleared only by hardware after reload (it cannot be cleared through register write once it is set)"]
    NOEFFECT = 0,
}
impl From<IMR_A> for bool {
    #[inline(always)]
    fn from(variant: IMR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IMR`"]
pub type IMR_R = crate::R<bool, IMR_A>;
impl IMR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IMR_A {
        match self.bits {
            true => IMR_A::RELOAD,
            false => IMR_A::NOEFFECT,
        }
    }
    #[doc = "Checks if the value of the field is `RELOAD`"]
    #[inline(always)]
    pub fn is_reload(&self) -> bool {
        *self == IMR_A::RELOAD
    }
    #[doc = "Checks if the value of the field is `NOEFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == IMR_A::NOEFFECT
    }
}
#[doc = "Write proxy for field `IMR`"]
pub struct IMR_W<'a> {
    w: &'a mut W,
}
impl<'a> IMR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IMR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The shadow registers are reloaded immediately. This bit is set by software and cleared only by hardware after reload"]
    #[inline(always)]
    pub fn reload(self) -> &'a mut W {
        self.variant(IMR_A::RELOAD)
    }
    #[doc = "This bit is set by software and cleared only by hardware after reload (it cannot be cleared through register write once it is set)"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(IMR_A::NOEFFECT)
    }
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
impl R {
    #[doc = "Bit 1 - Vertical Blanking Reload"]
    #[inline(always)]
    pub fn vbr(&self) -> VBR_R {
        VBR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Immediate Reload"]
    #[inline(always)]
    pub fn imr(&self) -> IMR_R {
        IMR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Vertical Blanking Reload"]
    #[inline(always)]
    pub fn vbr(&mut self) -> VBR_W {
        VBR_W { w: self }
    }
    #[doc = "Bit 0 - Immediate Reload"]
    #[inline(always)]
    pub fn imr(&mut self) -> IMR_W {
        IMR_W { w: self }
    }
}
