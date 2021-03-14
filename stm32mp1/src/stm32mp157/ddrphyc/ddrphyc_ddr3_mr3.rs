#[doc = "Reader of register DDRPHYC_DDR3_MR3"]
pub type R = crate::R<u8, super::DDRPHYC_DDR3_MR3>;
#[doc = "Writer for register DDRPHYC_DDR3_MR3"]
pub type W = crate::W<u8, super::DDRPHYC_DDR3_MR3>;
#[doc = "Register DDRPHYC_DDR3_MR3 `reset()`'s with value 0"]
impl crate::ResetValue for super::DDRPHYC_DDR3_MR3 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MPRLOC`"]
pub type MPRLOC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MPRLOC`"]
pub struct MPRLOC_W<'a> {
    w: &'a mut W,
}
impl<'a> MPRLOC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u8) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `MPR`"]
pub type MPR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPR`"]
pub struct MPR_W<'a> {
    w: &'a mut W,
}
impl<'a> MPR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u8) & 0x01) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - MPRLOC"]
    #[inline(always)]
    pub fn mprloc(&self) -> MPRLOC_R {
        MPRLOC_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - MPR"]
    #[inline(always)]
    pub fn mpr(&self) -> MPR_R {
        MPR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - MPRLOC"]
    #[inline(always)]
    pub fn mprloc(&mut self) -> MPRLOC_W {
        MPRLOC_W { w: self }
    }
    #[doc = "Bit 2 - MPR"]
    #[inline(always)]
    pub fn mpr(&mut self) -> MPR_W {
        MPR_W { w: self }
    }
}
