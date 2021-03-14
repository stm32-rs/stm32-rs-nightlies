#[doc = "Reader of register OTG_HFIR"]
pub type R = crate::R<u32, super::OTG_HFIR>;
#[doc = "Writer for register OTG_HFIR"]
pub type W = crate::W<u32, super::OTG_HFIR>;
#[doc = "Register OTG_HFIR `reset()`'s with value 0xea60"]
impl crate::ResetValue for super::OTG_HFIR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xea60
    }
}
#[doc = "Reader of field `FRIVL`"]
pub type FRIVL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `FRIVL`"]
pub struct FRIVL_W<'a> {
    w: &'a mut W,
}
impl<'a> FRIVL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `RLDCTRL`"]
pub type RLDCTRL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RLDCTRL`"]
pub struct RLDCTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> RLDCTRL_W<'a> {
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
impl R {
    #[doc = "Bits 0:15 - FRIVL"]
    #[inline(always)]
    pub fn frivl(&self) -> FRIVL_R {
        FRIVL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - RLDCTRL"]
    #[inline(always)]
    pub fn rldctrl(&self) -> RLDCTRL_R {
        RLDCTRL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - FRIVL"]
    #[inline(always)]
    pub fn frivl(&mut self) -> FRIVL_W {
        FRIVL_W { w: self }
    }
    #[doc = "Bit 16 - RLDCTRL"]
    #[inline(always)]
    pub fn rldctrl(&mut self) -> RLDCTRL_W {
        RLDCTRL_W { w: self }
    }
}
