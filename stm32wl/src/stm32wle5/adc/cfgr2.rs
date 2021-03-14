#[doc = "Reader of register CFGR2"]
pub type R = crate::R<u32, super::CFGR2>;
#[doc = "Writer for register CFGR2"]
pub type W = crate::W<u32, super::CFGR2>;
#[doc = "Register CFGR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::CFGR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OVSE`"]
pub type OVSE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OVSE`"]
pub struct OVSE_W<'a> {
    w: &'a mut W,
}
impl<'a> OVSE_W<'a> {
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
#[doc = "Reader of field `OVSR0`"]
pub type OVSR0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OVSR0`"]
pub struct OVSR0_W<'a> {
    w: &'a mut W,
}
impl<'a> OVSR0_W<'a> {
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
#[doc = "Reader of field `OVSR1`"]
pub type OVSR1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OVSR1`"]
pub struct OVSR1_W<'a> {
    w: &'a mut W,
}
impl<'a> OVSR1_W<'a> {
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
#[doc = "Reader of field `OVSR2`"]
pub type OVSR2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OVSR2`"]
pub struct OVSR2_W<'a> {
    w: &'a mut W,
}
impl<'a> OVSR2_W<'a> {
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
#[doc = "Reader of field `OVSS0`"]
pub type OVSS0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OVSS0`"]
pub struct OVSS0_W<'a> {
    w: &'a mut W,
}
impl<'a> OVSS0_W<'a> {
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
#[doc = "Reader of field `OVSS1`"]
pub type OVSS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OVSS1`"]
pub struct OVSS1_W<'a> {
    w: &'a mut W,
}
impl<'a> OVSS1_W<'a> {
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
#[doc = "Reader of field `OVSS2`"]
pub type OVSS2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OVSS2`"]
pub struct OVSS2_W<'a> {
    w: &'a mut W,
}
impl<'a> OVSS2_W<'a> {
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
#[doc = "Reader of field `OVSS3`"]
pub type OVSS3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OVSS3`"]
pub struct OVSS3_W<'a> {
    w: &'a mut W,
}
impl<'a> OVSS3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `TOVS`"]
pub type TOVS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TOVS`"]
pub struct TOVS_W<'a> {
    w: &'a mut W,
}
impl<'a> TOVS_W<'a> {
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
#[doc = "Reader of field `LFTRIG`"]
pub type LFTRIG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LFTRIG`"]
pub struct LFTRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> LFTRIG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `CKMODE`"]
pub type CKMODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CKMODE`"]
pub struct CKMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CKMODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - OVSE"]
    #[inline(always)]
    pub fn ovse(&self) -> OVSE_R {
        OVSE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - OVSR0"]
    #[inline(always)]
    pub fn ovsr0(&self) -> OVSR0_R {
        OVSR0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - OVSR1"]
    #[inline(always)]
    pub fn ovsr1(&self) -> OVSR1_R {
        OVSR1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - OVSR2"]
    #[inline(always)]
    pub fn ovsr2(&self) -> OVSR2_R {
        OVSR2_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - OVSS0"]
    #[inline(always)]
    pub fn ovss0(&self) -> OVSS0_R {
        OVSS0_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - OVSS1"]
    #[inline(always)]
    pub fn ovss1(&self) -> OVSS1_R {
        OVSS1_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - OVSS2"]
    #[inline(always)]
    pub fn ovss2(&self) -> OVSS2_R {
        OVSS2_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - OVSS3"]
    #[inline(always)]
    pub fn ovss3(&self) -> OVSS3_R {
        OVSS3_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - TOVS"]
    #[inline(always)]
    pub fn tovs(&self) -> TOVS_R {
        TOVS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 29 - LFTRIG"]
    #[inline(always)]
    pub fn lftrig(&self) -> LFTRIG_R {
        LFTRIG_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bits 30:31 - CKMODE"]
    #[inline(always)]
    pub fn ckmode(&self) -> CKMODE_R {
        CKMODE_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - OVSE"]
    #[inline(always)]
    pub fn ovse(&mut self) -> OVSE_W {
        OVSE_W { w: self }
    }
    #[doc = "Bit 2 - OVSR0"]
    #[inline(always)]
    pub fn ovsr0(&mut self) -> OVSR0_W {
        OVSR0_W { w: self }
    }
    #[doc = "Bit 3 - OVSR1"]
    #[inline(always)]
    pub fn ovsr1(&mut self) -> OVSR1_W {
        OVSR1_W { w: self }
    }
    #[doc = "Bit 4 - OVSR2"]
    #[inline(always)]
    pub fn ovsr2(&mut self) -> OVSR2_W {
        OVSR2_W { w: self }
    }
    #[doc = "Bit 5 - OVSS0"]
    #[inline(always)]
    pub fn ovss0(&mut self) -> OVSS0_W {
        OVSS0_W { w: self }
    }
    #[doc = "Bit 6 - OVSS1"]
    #[inline(always)]
    pub fn ovss1(&mut self) -> OVSS1_W {
        OVSS1_W { w: self }
    }
    #[doc = "Bit 7 - OVSS2"]
    #[inline(always)]
    pub fn ovss2(&mut self) -> OVSS2_W {
        OVSS2_W { w: self }
    }
    #[doc = "Bit 8 - OVSS3"]
    #[inline(always)]
    pub fn ovss3(&mut self) -> OVSS3_W {
        OVSS3_W { w: self }
    }
    #[doc = "Bit 9 - TOVS"]
    #[inline(always)]
    pub fn tovs(&mut self) -> TOVS_W {
        TOVS_W { w: self }
    }
    #[doc = "Bit 29 - LFTRIG"]
    #[inline(always)]
    pub fn lftrig(&mut self) -> LFTRIG_W {
        LFTRIG_W { w: self }
    }
    #[doc = "Bits 30:31 - CKMODE"]
    #[inline(always)]
    pub fn ckmode(&mut self) -> CKMODE_W {
        CKMODE_W { w: self }
    }
}
