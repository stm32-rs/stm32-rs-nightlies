#[doc = "Reader of register DDRPHYC_DDR3_MR2"]
pub type R = crate::R<u16, super::DDRPHYC_DDR3_MR2>;
#[doc = "Writer for register DDRPHYC_DDR3_MR2"]
pub type W = crate::W<u16, super::DDRPHYC_DDR3_MR2>;
#[doc = "Register DDRPHYC_DDR3_MR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::DDRPHYC_DDR3_MR2 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PASR`"]
pub type PASR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PASR`"]
pub struct PASR_W<'a> {
    w: &'a mut W,
}
impl<'a> PASR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u16) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `CWL`"]
pub type CWL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CWL`"]
pub struct CWL_W<'a> {
    w: &'a mut W,
}
impl<'a> CWL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u16) & 0x07) << 3);
        self.w
    }
}
#[doc = "Reader of field `ASR`"]
pub type ASR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ASR`"]
pub struct ASR_W<'a> {
    w: &'a mut W,
}
impl<'a> ASR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u16) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `SRT`"]
pub type SRT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SRT`"]
pub struct SRT_W<'a> {
    w: &'a mut W,
}
impl<'a> SRT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u16) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `RTTWR`"]
pub type RTTWR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTTWR`"]
pub struct RTTWR_W<'a> {
    w: &'a mut W,
}
impl<'a> RTTWR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | (((value as u16) & 0x03) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - PASR"]
    #[inline(always)]
    pub fn pasr(&self) -> PASR_R {
        PASR_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:5 - CWL"]
    #[inline(always)]
    pub fn cwl(&self) -> CWL_R {
        CWL_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bit 6 - ASR"]
    #[inline(always)]
    pub fn asr(&self) -> ASR_R {
        ASR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - SRT"]
    #[inline(always)]
    pub fn srt(&self) -> SRT_R {
        SRT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 9:10 - RTTWR"]
    #[inline(always)]
    pub fn rttwr(&self) -> RTTWR_R {
        RTTWR_R::new(((self.bits >> 9) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - PASR"]
    #[inline(always)]
    pub fn pasr(&mut self) -> PASR_W {
        PASR_W { w: self }
    }
    #[doc = "Bits 3:5 - CWL"]
    #[inline(always)]
    pub fn cwl(&mut self) -> CWL_W {
        CWL_W { w: self }
    }
    #[doc = "Bit 6 - ASR"]
    #[inline(always)]
    pub fn asr(&mut self) -> ASR_W {
        ASR_W { w: self }
    }
    #[doc = "Bit 7 - SRT"]
    #[inline(always)]
    pub fn srt(&mut self) -> SRT_W {
        SRT_W { w: self }
    }
    #[doc = "Bits 9:10 - RTTWR"]
    #[inline(always)]
    pub fn rttwr(&mut self) -> RTTWR_W {
        RTTWR_W { w: self }
    }
}
