#[doc = "Reader of register PRAR_PRG"]
pub type R = crate::R<u32, super::PRAR_PRG>;
#[doc = "Writer for register PRAR_PRG"]
pub type W = crate::W<u32, super::PRAR_PRG>;
#[doc = "Register PRAR_PRG `reset()`'s with value 0"]
impl crate::ResetValue for super::PRAR_PRG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DMEP`"]
pub type DMEP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMEP`"]
pub struct DMEP_W<'a> {
    w: &'a mut W,
}
impl<'a> DMEP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `PROT_AREA_END`"]
pub type PROT_AREA_END_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PROT_AREA_END`"]
pub struct PROT_AREA_END_W<'a> {
    w: &'a mut W,
}
impl<'a> PROT_AREA_END_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | (((value as u32) & 0x0fff) << 16);
        self.w
    }
}
#[doc = "Reader of field `PROT_AREA_START`"]
pub type PROT_AREA_START_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PROT_AREA_START`"]
pub struct PROT_AREA_START_W<'a> {
    w: &'a mut W,
}
impl<'a> PROT_AREA_START_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - Bank 1 PCROP protected erase enable option configuration bit"]
    #[inline(always)]
    pub fn dmep(&self) -> DMEP_R {
        DMEP_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 16:27 - Bank 1 PCROP area end configuration bits"]
    #[inline(always)]
    pub fn prot_area_end(&self) -> PROT_AREA_END_R {
        PROT_AREA_END_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 0:11 - Bank 1 PCROP area start configuration bits"]
    #[inline(always)]
    pub fn prot_area_start(&self) -> PROT_AREA_START_R {
        PROT_AREA_START_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 31 - Bank 1 PCROP protected erase enable option configuration bit"]
    #[inline(always)]
    pub fn dmep(&mut self) -> DMEP_W {
        DMEP_W { w: self }
    }
    #[doc = "Bits 16:27 - Bank 1 PCROP area end configuration bits"]
    #[inline(always)]
    pub fn prot_area_end(&mut self) -> PROT_AREA_END_W {
        PROT_AREA_END_W { w: self }
    }
    #[doc = "Bits 0:11 - Bank 1 PCROP area start configuration bits"]
    #[inline(always)]
    pub fn prot_area_start(&mut self) -> PROT_AREA_START_W {
        PROT_AREA_START_W { w: self }
    }
}
