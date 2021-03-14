#[doc = "Reader of register AHB3SMENR"]
pub type R = crate::R<u32, super::AHB3SMENR>;
#[doc = "Writer for register AHB3SMENR"]
pub type W = crate::W<u32, super::AHB3SMENR>;
#[doc = "Register AHB3SMENR `reset()`'s with value 0x0101"]
impl crate::ResetValue for super::AHB3SMENR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0101
    }
}
#[doc = "Reader of field `FMCSMEN`"]
pub type FMCSMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FMCSMEN`"]
pub struct FMCSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FMCSMEN_W<'a> {
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
#[doc = "Reader of field `OSPI1SMEN`"]
pub type OSPI1SMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OSPI1SMEN`"]
pub struct OSPI1SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPI1SMEN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Flexible memory controller clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn fmcsmen(&self) -> FMCSMEN_R {
        FMCSMEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 8 - OSPI1SMEN"]
    #[inline(always)]
    pub fn ospi1smen(&self) -> OSPI1SMEN_R {
        OSPI1SMEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Flexible memory controller clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn fmcsmen(&mut self) -> FMCSMEN_W {
        FMCSMEN_W { w: self }
    }
    #[doc = "Bit 8 - OSPI1SMEN"]
    #[inline(always)]
    pub fn ospi1smen(&mut self) -> OSPI1SMEN_W {
        OSPI1SMEN_W { w: self }
    }
}
