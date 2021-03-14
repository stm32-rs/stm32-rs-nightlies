#[doc = "Reader of register FMC_CSQCFGR1"]
pub type R = crate::R<u32, super::FMC_CSQCFGR1>;
#[doc = "Writer for register FMC_CSQCFGR1"]
pub type W = crate::W<u32, super::FMC_CSQCFGR1>;
#[doc = "Register FMC_CSQCFGR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::FMC_CSQCFGR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CMD2EN`"]
pub type CMD2EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMD2EN`"]
pub struct CMD2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD2EN_W<'a> {
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
#[doc = "Reader of field `DMADEN`"]
pub type DMADEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMADEN`"]
pub struct DMADEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMADEN_W<'a> {
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
#[doc = "Reader of field `ACYNBR`"]
pub type ACYNBR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ACYNBR`"]
pub struct ACYNBR_W<'a> {
    w: &'a mut W,
}
impl<'a> ACYNBR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `CMD1`"]
pub type CMD1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CMD1`"]
pub struct CMD1_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `CMD2`"]
pub type CMD2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CMD2`"]
pub struct CMD2_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `CMD1T`"]
pub type CMD1T_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMD1T`"]
pub struct CMD1T_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD1T_W<'a> {
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
#[doc = "Reader of field `CMD2T`"]
pub type CMD2T_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMD2T`"]
pub struct CMD2T_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD2T_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - CMD2EN"]
    #[inline(always)]
    pub fn cmd2en(&self) -> CMD2EN_R {
        CMD2EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DMADEN"]
    #[inline(always)]
    pub fn dmaden(&self) -> DMADEN_R {
        DMADEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - ACYNBR"]
    #[inline(always)]
    pub fn acynbr(&self) -> ACYNBR_R {
        ACYNBR_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 8:15 - CMD1"]
    #[inline(always)]
    pub fn cmd1(&self) -> CMD1_R {
        CMD1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - CMD2"]
    #[inline(always)]
    pub fn cmd2(&self) -> CMD2_R {
        CMD2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - CMD1T"]
    #[inline(always)]
    pub fn cmd1t(&self) -> CMD1T_R {
        CMD1T_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - CMD2T"]
    #[inline(always)]
    pub fn cmd2t(&self) -> CMD2T_R {
        CMD2T_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - CMD2EN"]
    #[inline(always)]
    pub fn cmd2en(&mut self) -> CMD2EN_W {
        CMD2EN_W { w: self }
    }
    #[doc = "Bit 2 - DMADEN"]
    #[inline(always)]
    pub fn dmaden(&mut self) -> DMADEN_W {
        DMADEN_W { w: self }
    }
    #[doc = "Bits 4:6 - ACYNBR"]
    #[inline(always)]
    pub fn acynbr(&mut self) -> ACYNBR_W {
        ACYNBR_W { w: self }
    }
    #[doc = "Bits 8:15 - CMD1"]
    #[inline(always)]
    pub fn cmd1(&mut self) -> CMD1_W {
        CMD1_W { w: self }
    }
    #[doc = "Bits 16:23 - CMD2"]
    #[inline(always)]
    pub fn cmd2(&mut self) -> CMD2_W {
        CMD2_W { w: self }
    }
    #[doc = "Bit 24 - CMD1T"]
    #[inline(always)]
    pub fn cmd1t(&mut self) -> CMD1T_W {
        CMD1T_W { w: self }
    }
    #[doc = "Bit 25 - CMD2T"]
    #[inline(always)]
    pub fn cmd2t(&mut self) -> CMD2T_W {
        CMD2T_W { w: self }
    }
}
