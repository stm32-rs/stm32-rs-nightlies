#[doc = "Reader of register MACPPSCR"]
pub type R = crate::R<u32, super::MACPPSCR>;
#[doc = "Writer for register MACPPSCR"]
pub type W = crate::W<u32, super::MACPPSCR>;
#[doc = "Register MACPPSCR `reset()`'s with value 0"]
impl crate::ResetValue for super::MACPPSCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PPSCTRL`"]
pub type PPSCTRL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PPSCTRL`"]
pub struct PPSCTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> PPSCTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `PPSEN0`"]
pub type PPSEN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PPSEN0`"]
pub struct PPSEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> PPSEN0_W<'a> {
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
#[doc = "Reader of field `TRGTMODSEL0`"]
pub type TRGTMODSEL0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRGTMODSEL0`"]
pub struct TRGTMODSEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGTMODSEL0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Flexible PPS Output (ptp_pps_o\\[0\\]) Control or PPSCTRL PPS Output Frequency Control if PPSEN0 is cleared"]
    #[inline(always)]
    pub fn ppsctrl(&self) -> PPSCTRL_R {
        PPSCTRL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Flexible PPS Output Mode Enable"]
    #[inline(always)]
    pub fn ppsen0(&self) -> PPSEN0_R {
        PPSEN0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - Target Time Register Mode for PPS Output"]
    #[inline(always)]
    pub fn trgtmodsel0(&self) -> TRGTMODSEL0_R {
        TRGTMODSEL0_R::new(((self.bits >> 5) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Flexible PPS Output (ptp_pps_o\\[0\\]) Control or PPSCTRL PPS Output Frequency Control if PPSEN0 is cleared"]
    #[inline(always)]
    pub fn ppsctrl(&mut self) -> PPSCTRL_W {
        PPSCTRL_W { w: self }
    }
    #[doc = "Bit 4 - Flexible PPS Output Mode Enable"]
    #[inline(always)]
    pub fn ppsen0(&mut self) -> PPSEN0_W {
        PPSEN0_W { w: self }
    }
    #[doc = "Bits 5:6 - Target Time Register Mode for PPS Output"]
    #[inline(always)]
    pub fn trgtmodsel0(&mut self) -> TRGTMODSEL0_W {
        TRGTMODSEL0_W { w: self }
    }
}
