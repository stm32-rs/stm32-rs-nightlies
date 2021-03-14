#[doc = "Reader of register CPUCR"]
pub type R = crate::R<u32, super::CPUCR>;
#[doc = "Writer for register CPUCR"]
pub type W = crate::W<u32, super::CPUCR>;
#[doc = "Register CPUCR `reset()`'s with value 0"]
impl crate::ResetValue for super::CPUCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PDDS_D1`"]
pub type PDDS_D1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PDDS_D1`"]
pub struct PDDS_D1_W<'a> {
    w: &'a mut W,
}
impl<'a> PDDS_D1_W<'a> {
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
#[doc = "Reader of field `PDDS_D2`"]
pub type PDDS_D2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PDDS_D2`"]
pub struct PDDS_D2_W<'a> {
    w: &'a mut W,
}
impl<'a> PDDS_D2_W<'a> {
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
#[doc = "Reader of field `PDDS_D3`"]
pub type PDDS_D3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PDDS_D3`"]
pub struct PDDS_D3_W<'a> {
    w: &'a mut W,
}
impl<'a> PDDS_D3_W<'a> {
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
#[doc = "Reader of field `STOPF`"]
pub type STOPF_R = crate::R<bool, bool>;
#[doc = "Reader of field `SBF`"]
pub type SBF_R = crate::R<bool, bool>;
#[doc = "Reader of field `SBF_D1`"]
pub type SBF_D1_R = crate::R<bool, bool>;
#[doc = "Reader of field `SBF_D2`"]
pub type SBF_D2_R = crate::R<bool, bool>;
#[doc = "Reader of field `CSSF`"]
pub type CSSF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CSSF`"]
pub struct CSSF_W<'a> {
    w: &'a mut W,
}
impl<'a> CSSF_W<'a> {
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
#[doc = "Reader of field `RUN_D3`"]
pub type RUN_D3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RUN_D3`"]
pub struct RUN_D3_W<'a> {
    w: &'a mut W,
}
impl<'a> RUN_D3_W<'a> {
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
impl R {
    #[doc = "Bit 0 - D1 domain Power Down Deepsleep selection. This bit allows CPU1 to define the Deepsleep mode for D1 domain."]
    #[inline(always)]
    pub fn pdds_d1(&self) -> PDDS_D1_R {
        PDDS_D1_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - D2 domain Power Down Deepsleep. This bit allows CPU1 to define the Deepsleep mode for D2 domain."]
    #[inline(always)]
    pub fn pdds_d2(&self) -> PDDS_D2_R {
        PDDS_D2_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - System D3 domain Power Down Deepsleep. This bit allows CPU1 to define the Deepsleep mode for System D3 domain."]
    #[inline(always)]
    pub fn pdds_d3(&self) -> PDDS_D3_R {
        PDDS_D3_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 5 - STOP flag This bit is set by hardware and cleared only by any reset or by setting the CPU1 CSSF bit."]
    #[inline(always)]
    pub fn stopf(&self) -> STOPF_R {
        STOPF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - System Standby flag This bit is set by hardware and cleared only by a POR (Power-on Reset) or by setting the CPU1 CSSF bit"]
    #[inline(always)]
    pub fn sbf(&self) -> SBF_R {
        SBF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - D1 domain DStandby flag This bit is set by hardware and cleared by any system reset or by setting the CPU1 CSSF bit. Once set, this bit can be cleared only when the D1 domain is no longer in DStandby mode."]
    #[inline(always)]
    pub fn sbf_d1(&self) -> SBF_D1_R {
        SBF_D1_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - D2 domain DStandby flag This bit is set by hardware and cleared by any system reset or by setting the CPU1 CSSF bit. Once set, this bit can be cleared only when the D2 domain is no longer in DStandby mode."]
    #[inline(always)]
    pub fn sbf_d2(&self) -> SBF_D2_R {
        SBF_D2_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Clear D1 domain CPU1 Standby, Stop and HOLD flags (always read as 0) This bit is cleared to 0 by hardware."]
    #[inline(always)]
    pub fn cssf(&self) -> CSSF_R {
        CSSF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Keep system D3 domain in Run mode regardless of the CPU sub-systems modes"]
    #[inline(always)]
    pub fn run_d3(&self) -> RUN_D3_R {
        RUN_D3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - D1 domain Power Down Deepsleep selection. This bit allows CPU1 to define the Deepsleep mode for D1 domain."]
    #[inline(always)]
    pub fn pdds_d1(&mut self) -> PDDS_D1_W {
        PDDS_D1_W { w: self }
    }
    #[doc = "Bit 1 - D2 domain Power Down Deepsleep. This bit allows CPU1 to define the Deepsleep mode for D2 domain."]
    #[inline(always)]
    pub fn pdds_d2(&mut self) -> PDDS_D2_W {
        PDDS_D2_W { w: self }
    }
    #[doc = "Bit 2 - System D3 domain Power Down Deepsleep. This bit allows CPU1 to define the Deepsleep mode for System D3 domain."]
    #[inline(always)]
    pub fn pdds_d3(&mut self) -> PDDS_D3_W {
        PDDS_D3_W { w: self }
    }
    #[doc = "Bit 9 - Clear D1 domain CPU1 Standby, Stop and HOLD flags (always read as 0) This bit is cleared to 0 by hardware."]
    #[inline(always)]
    pub fn cssf(&mut self) -> CSSF_W {
        CSSF_W { w: self }
    }
    #[doc = "Bit 11 - Keep system D3 domain in Run mode regardless of the CPU sub-systems modes"]
    #[inline(always)]
    pub fn run_d3(&mut self) -> RUN_D3_W {
        RUN_D3_W { w: self }
    }
}
