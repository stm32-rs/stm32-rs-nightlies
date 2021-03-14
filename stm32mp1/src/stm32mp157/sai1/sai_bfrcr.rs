#[doc = "Reader of register SAI_BFRCR"]
pub type R = crate::R<u32, super::SAI_BFRCR>;
#[doc = "Writer for register SAI_BFRCR"]
pub type W = crate::W<u32, super::SAI_BFRCR>;
#[doc = "Register SAI_BFRCR `reset()`'s with value 0x07"]
impl crate::ResetValue for super::SAI_BFRCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x07
    }
}
#[doc = "Reader of field `FRL`"]
pub type FRL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FRL`"]
pub struct FRL_W<'a> {
    w: &'a mut W,
}
impl<'a> FRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `FSALL`"]
pub type FSALL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FSALL`"]
pub struct FSALL_W<'a> {
    w: &'a mut W,
}
impl<'a> FSALL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u32) & 0x7f) << 8);
        self.w
    }
}
#[doc = "Reader of field `FSDEF`"]
pub type FSDEF_R = crate::R<bool, bool>;
#[doc = "Reader of field `FSPOL`"]
pub type FSPOL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSPOL`"]
pub struct FSPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> FSPOL_W<'a> {
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
#[doc = "Reader of field `FSOFF`"]
pub type FSOFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSOFF`"]
pub struct FSOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> FSOFF_W<'a> {
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
impl R {
    #[doc = "Bits 0:7 - FRL"]
    #[inline(always)]
    pub fn frl(&self) -> FRL_R {
        FRL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:14 - FSALL"]
    #[inline(always)]
    pub fn fsall(&self) -> FSALL_R {
        FSALL_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - FSDEF"]
    #[inline(always)]
    pub fn fsdef(&self) -> FSDEF_R {
        FSDEF_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - FSPOL"]
    #[inline(always)]
    pub fn fspol(&self) -> FSPOL_R {
        FSPOL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - FSOFF"]
    #[inline(always)]
    pub fn fsoff(&self) -> FSOFF_R {
        FSOFF_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - FRL"]
    #[inline(always)]
    pub fn frl(&mut self) -> FRL_W {
        FRL_W { w: self }
    }
    #[doc = "Bits 8:14 - FSALL"]
    #[inline(always)]
    pub fn fsall(&mut self) -> FSALL_W {
        FSALL_W { w: self }
    }
    #[doc = "Bit 17 - FSPOL"]
    #[inline(always)]
    pub fn fspol(&mut self) -> FSPOL_W {
        FSPOL_W { w: self }
    }
    #[doc = "Bit 18 - FSOFF"]
    #[inline(always)]
    pub fn fsoff(&mut self) -> FSOFF_W {
        FSOFF_W { w: self }
    }
}
