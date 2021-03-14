#[doc = "Reader of register FMC_CSQAR2"]
pub type R = crate::R<u32, super::FMC_CSQAR2>;
#[doc = "Writer for register FMC_CSQAR2"]
pub type W = crate::W<u32, super::FMC_CSQAR2>;
#[doc = "Register FMC_CSQAR2 `reset()`'s with value 0x0002_0000"]
impl crate::ResetValue for super::FMC_CSQAR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0002_0000
    }
}
#[doc = "Reader of field `ADDC5`"]
pub type ADDC5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADDC5`"]
pub struct ADDC5_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDC5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `NANDCEN0`"]
pub type NANDCEN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NANDCEN0`"]
pub struct NANDCEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> NANDCEN0_W<'a> {
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
#[doc = "Reader of field `NANDCEN1`"]
pub type NANDCEN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NANDCEN1`"]
pub struct NANDCEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> NANDCEN1_W<'a> {
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
#[doc = "Reader of field `SAO`"]
pub type SAO_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SAO`"]
pub struct SAO_W<'a> {
    w: &'a mut W,
}
impl<'a> SAO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - ADDC5"]
    #[inline(always)]
    pub fn addc5(&self) -> ADDC5_R {
        ADDC5_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 10 - NANDCEN0"]
    #[inline(always)]
    pub fn nandcen0(&self) -> NANDCEN0_R {
        NANDCEN0_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - NANDCEN1"]
    #[inline(always)]
    pub fn nandcen1(&self) -> NANDCEN1_R {
        NANDCEN1_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 16:31 - SAO"]
    #[inline(always)]
    pub fn sao(&self) -> SAO_R {
        SAO_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - ADDC5"]
    #[inline(always)]
    pub fn addc5(&mut self) -> ADDC5_W {
        ADDC5_W { w: self }
    }
    #[doc = "Bit 10 - NANDCEN0"]
    #[inline(always)]
    pub fn nandcen0(&mut self) -> NANDCEN0_W {
        NANDCEN0_W { w: self }
    }
    #[doc = "Bit 11 - NANDCEN1"]
    #[inline(always)]
    pub fn nandcen1(&mut self) -> NANDCEN1_W {
        NANDCEN1_W { w: self }
    }
    #[doc = "Bits 16:31 - SAO"]
    #[inline(always)]
    pub fn sao(&mut self) -> SAO_W {
        SAO_W { w: self }
    }
}
