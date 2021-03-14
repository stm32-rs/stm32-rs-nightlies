#[doc = "Reader of register IMR2"]
pub type R = crate::R<u32, super::IMR2>;
#[doc = "Writer for register IMR2"]
pub type W = crate::W<u32, super::IMR2>;
#[doc = "Register IMR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::IMR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PVM3IM`"]
pub type PVM3IM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PVM3IM`"]
pub struct PVM3IM_W<'a> {
    w: &'a mut W,
}
impl<'a> PVM3IM_W<'a> {
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
#[doc = "Reader of field `PVM1IM`"]
pub type PVM1IM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PVM1IM`"]
pub struct PVM1IM_W<'a> {
    w: &'a mut W,
}
impl<'a> PVM1IM_W<'a> {
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
#[doc = "Reader of field `PVDIM`"]
pub type PVDIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PVDIM`"]
pub struct PVDIM_W<'a> {
    w: &'a mut W,
}
impl<'a> PVDIM_W<'a> {
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
impl R {
    #[doc = "Bit 18 - Peripheral PVM3 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn pvm3im(&self) -> PVM3IM_R {
        PVM3IM_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Peripheral PVM1 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn pvm1im(&self) -> PVM1IM_R {
        PVM1IM_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Peripheral PVD interrupt mask to CPU1"]
    #[inline(always)]
    pub fn pvdim(&self) -> PVDIM_R {
        PVDIM_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 18 - Peripheral PVM3 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn pvm3im(&mut self) -> PVM3IM_W {
        PVM3IM_W { w: self }
    }
    #[doc = "Bit 16 - Peripheral PVM1 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn pvm1im(&mut self) -> PVM1IM_W {
        PVM1IM_W { w: self }
    }
    #[doc = "Bit 20 - Peripheral PVD interrupt mask to CPU1"]
    #[inline(always)]
    pub fn pvdim(&mut self) -> PVDIM_W {
        PVDIM_W { w: self }
    }
}
