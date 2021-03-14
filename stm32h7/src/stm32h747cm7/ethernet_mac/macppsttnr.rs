#[doc = "Reader of register MACPPSTTNR"]
pub type R = crate::R<u32, super::MACPPSTTNR>;
#[doc = "Writer for register MACPPSTTNR"]
pub type W = crate::W<u32, super::MACPPSTTNR>;
#[doc = "Register MACPPSTTNR `reset()`'s with value 0"]
impl crate::ResetValue for super::MACPPSTTNR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TTSL0`"]
pub type TTSL0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TTSL0`"]
pub struct TTSL0_W<'a> {
    w: &'a mut W,
}
impl<'a> TTSL0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff_ffff) | ((value as u32) & 0x7fff_ffff);
        self.w
    }
}
#[doc = "Reader of field `TRGTBUSY0`"]
pub type TRGTBUSY0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRGTBUSY0`"]
pub struct TRGTBUSY0_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGTBUSY0_W<'a> {
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
impl R {
    #[doc = "Bits 0:30 - Target Time Low for PPS Register"]
    #[inline(always)]
    pub fn ttsl0(&self) -> TTSL0_R {
        TTSL0_R::new((self.bits & 0x7fff_ffff) as u32)
    }
    #[doc = "Bit 31 - PPS Target Time Register Busy"]
    #[inline(always)]
    pub fn trgtbusy0(&self) -> TRGTBUSY0_R {
        TRGTBUSY0_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30 - Target Time Low for PPS Register"]
    #[inline(always)]
    pub fn ttsl0(&mut self) -> TTSL0_W {
        TTSL0_W { w: self }
    }
    #[doc = "Bit 31 - PPS Target Time Register Busy"]
    #[inline(always)]
    pub fn trgtbusy0(&mut self) -> TRGTBUSY0_W {
        TRGTBUSY0_W { w: self }
    }
}
