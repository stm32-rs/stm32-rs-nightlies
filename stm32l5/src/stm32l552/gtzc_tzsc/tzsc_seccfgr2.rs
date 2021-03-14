#[doc = "Reader of register TZSC_SECCFGR2"]
pub type R = crate::R<u32, super::TZSC_SECCFGR2>;
#[doc = "Writer for register TZSC_SECCFGR2"]
pub type W = crate::W<u32, super::TZSC_SECCFGR2>;
#[doc = "Register TZSC_SECCFGR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::TZSC_SECCFGR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TIM8SEC`"]
pub type TIM8SEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM8SEC`"]
pub struct TIM8SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM8SEC_W<'a> {
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
#[doc = "Reader of field `USART1SEC`"]
pub type USART1SEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USART1SEC`"]
pub struct USART1SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> USART1SEC_W<'a> {
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
#[doc = "Reader of field `TIM15SEC`"]
pub type TIM15SEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM15SEC`"]
pub struct TIM15SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM15SEC_W<'a> {
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
#[doc = "Reader of field `TIM16SEC`"]
pub type TIM16SEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM16SEC`"]
pub struct TIM16SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM16SEC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `TIM17SEC`"]
pub type TIM17SEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM17SEC`"]
pub struct TIM17SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM17SEC_W<'a> {
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
#[doc = "Reader of field `SAI1SEC`"]
pub type SAI1SEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SAI1SEC`"]
pub struct SAI1SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI1SEC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `SAI2SEC`"]
pub type SAI2SEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SAI2SEC`"]
pub struct SAI2SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI2SEC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `DFSDM1SEC`"]
pub type DFSDM1SEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DFSDM1SEC`"]
pub struct DFSDM1SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> DFSDM1SEC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `CRCSEC`"]
pub type CRCSEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRCSEC`"]
pub struct CRCSEC_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCSEC_W<'a> {
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
#[doc = "Reader of field `TSCSEC`"]
pub type TSCSEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSCSEC`"]
pub struct TSCSEC_W<'a> {
    w: &'a mut W,
}
impl<'a> TSCSEC_W<'a> {
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
#[doc = "Reader of field `ICACHESEC`"]
pub type ICACHESEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ICACHESEC`"]
pub struct ICACHESEC_W<'a> {
    w: &'a mut W,
}
impl<'a> ICACHESEC_W<'a> {
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
#[doc = "Reader of field `ADCSEC`"]
pub type ADCSEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADCSEC`"]
pub struct ADCSEC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCSEC_W<'a> {
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
#[doc = "Reader of field `AESSEC`"]
pub type AESSEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AESSEC`"]
pub struct AESSEC_W<'a> {
    w: &'a mut W,
}
impl<'a> AESSEC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `HASHSEC`"]
pub type HASHSEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HASHSEC`"]
pub struct HASHSEC_W<'a> {
    w: &'a mut W,
}
impl<'a> HASHSEC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `RNGSEC`"]
pub type RNGSEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RNGSEC`"]
pub struct RNGSEC_W<'a> {
    w: &'a mut W,
}
impl<'a> RNGSEC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `PKASEC`"]
pub type PKASEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PKASEC`"]
pub struct PKASEC_W<'a> {
    w: &'a mut W,
}
impl<'a> PKASEC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `SDMMC1SEC`"]
pub type SDMMC1SEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SDMMC1SEC`"]
pub struct SDMMC1SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> SDMMC1SEC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `FSMC_REGSEC`"]
pub type FSMC_REGSEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSMC_REGSEC`"]
pub struct FSMC_REGSEC_W<'a> {
    w: &'a mut W,
}
impl<'a> FSMC_REGSEC_W<'a> {
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
#[doc = "Reader of field `OCTOSPI1_REGSEC`"]
pub type OCTOSPI1_REGSEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OCTOSPI1_REGSEC`"]
pub struct OCTOSPI1_REGSEC_W<'a> {
    w: &'a mut W,
}
impl<'a> OCTOSPI1_REGSEC_W<'a> {
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
    #[doc = "Bit 0 - TIM8SEC"]
    #[inline(always)]
    pub fn tim8sec(&self) -> TIM8SEC_R {
        TIM8SEC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - USART1SEC"]
    #[inline(always)]
    pub fn usart1sec(&self) -> USART1SEC_R {
        USART1SEC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TIM15SEC"]
    #[inline(always)]
    pub fn tim15sec(&self) -> TIM15SEC_R {
        TIM15SEC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TIM16SEC"]
    #[inline(always)]
    pub fn tim16sec(&self) -> TIM16SEC_R {
        TIM16SEC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TIM17SEC"]
    #[inline(always)]
    pub fn tim17sec(&self) -> TIM17SEC_R {
        TIM17SEC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - SAI1SEC"]
    #[inline(always)]
    pub fn sai1sec(&self) -> SAI1SEC_R {
        SAI1SEC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SAI2SEC"]
    #[inline(always)]
    pub fn sai2sec(&self) -> SAI2SEC_R {
        SAI2SEC_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - DFSDM1SEC"]
    #[inline(always)]
    pub fn dfsdm1sec(&self) -> DFSDM1SEC_R {
        DFSDM1SEC_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - CRCSEC"]
    #[inline(always)]
    pub fn crcsec(&self) -> CRCSEC_R {
        CRCSEC_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - TSCSEC"]
    #[inline(always)]
    pub fn tscsec(&self) -> TSCSEC_R {
        TSCSEC_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - ICACHESEC"]
    #[inline(always)]
    pub fn icachesec(&self) -> ICACHESEC_R {
        ICACHESEC_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - ADCSEC"]
    #[inline(always)]
    pub fn adcsec(&self) -> ADCSEC_R {
        ADCSEC_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - AESSEC"]
    #[inline(always)]
    pub fn aessec(&self) -> AESSEC_R {
        AESSEC_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - HASHSEC"]
    #[inline(always)]
    pub fn hashsec(&self) -> HASHSEC_R {
        HASHSEC_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - RNGSEC"]
    #[inline(always)]
    pub fn rngsec(&self) -> RNGSEC_R {
        RNGSEC_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - PKASEC"]
    #[inline(always)]
    pub fn pkasec(&self) -> PKASEC_R {
        PKASEC_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - SDMMC1SEC"]
    #[inline(always)]
    pub fn sdmmc1sec(&self) -> SDMMC1SEC_R {
        SDMMC1SEC_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - FSMC_REGSEC"]
    #[inline(always)]
    pub fn fsmc_regsec(&self) -> FSMC_REGSEC_R {
        FSMC_REGSEC_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - OCTOSPI1_REGSEC"]
    #[inline(always)]
    pub fn octospi1_regsec(&self) -> OCTOSPI1_REGSEC_R {
        OCTOSPI1_REGSEC_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM8SEC"]
    #[inline(always)]
    pub fn tim8sec(&mut self) -> TIM8SEC_W {
        TIM8SEC_W { w: self }
    }
    #[doc = "Bit 1 - USART1SEC"]
    #[inline(always)]
    pub fn usart1sec(&mut self) -> USART1SEC_W {
        USART1SEC_W { w: self }
    }
    #[doc = "Bit 2 - TIM15SEC"]
    #[inline(always)]
    pub fn tim15sec(&mut self) -> TIM15SEC_W {
        TIM15SEC_W { w: self }
    }
    #[doc = "Bit 3 - TIM16SEC"]
    #[inline(always)]
    pub fn tim16sec(&mut self) -> TIM16SEC_W {
        TIM16SEC_W { w: self }
    }
    #[doc = "Bit 4 - TIM17SEC"]
    #[inline(always)]
    pub fn tim17sec(&mut self) -> TIM17SEC_W {
        TIM17SEC_W { w: self }
    }
    #[doc = "Bit 5 - SAI1SEC"]
    #[inline(always)]
    pub fn sai1sec(&mut self) -> SAI1SEC_W {
        SAI1SEC_W { w: self }
    }
    #[doc = "Bit 6 - SAI2SEC"]
    #[inline(always)]
    pub fn sai2sec(&mut self) -> SAI2SEC_W {
        SAI2SEC_W { w: self }
    }
    #[doc = "Bit 7 - DFSDM1SEC"]
    #[inline(always)]
    pub fn dfsdm1sec(&mut self) -> DFSDM1SEC_W {
        DFSDM1SEC_W { w: self }
    }
    #[doc = "Bit 8 - CRCSEC"]
    #[inline(always)]
    pub fn crcsec(&mut self) -> CRCSEC_W {
        CRCSEC_W { w: self }
    }
    #[doc = "Bit 9 - TSCSEC"]
    #[inline(always)]
    pub fn tscsec(&mut self) -> TSCSEC_W {
        TSCSEC_W { w: self }
    }
    #[doc = "Bit 10 - ICACHESEC"]
    #[inline(always)]
    pub fn icachesec(&mut self) -> ICACHESEC_W {
        ICACHESEC_W { w: self }
    }
    #[doc = "Bit 11 - ADCSEC"]
    #[inline(always)]
    pub fn adcsec(&mut self) -> ADCSEC_W {
        ADCSEC_W { w: self }
    }
    #[doc = "Bit 12 - AESSEC"]
    #[inline(always)]
    pub fn aessec(&mut self) -> AESSEC_W {
        AESSEC_W { w: self }
    }
    #[doc = "Bit 13 - HASHSEC"]
    #[inline(always)]
    pub fn hashsec(&mut self) -> HASHSEC_W {
        HASHSEC_W { w: self }
    }
    #[doc = "Bit 14 - RNGSEC"]
    #[inline(always)]
    pub fn rngsec(&mut self) -> RNGSEC_W {
        RNGSEC_W { w: self }
    }
    #[doc = "Bit 15 - PKASEC"]
    #[inline(always)]
    pub fn pkasec(&mut self) -> PKASEC_W {
        PKASEC_W { w: self }
    }
    #[doc = "Bit 16 - SDMMC1SEC"]
    #[inline(always)]
    pub fn sdmmc1sec(&mut self) -> SDMMC1SEC_W {
        SDMMC1SEC_W { w: self }
    }
    #[doc = "Bit 17 - FSMC_REGSEC"]
    #[inline(always)]
    pub fn fsmc_regsec(&mut self) -> FSMC_REGSEC_W {
        FSMC_REGSEC_W { w: self }
    }
    #[doc = "Bit 18 - OCTOSPI1_REGSEC"]
    #[inline(always)]
    pub fn octospi1_regsec(&mut self) -> OCTOSPI1_REGSEC_W {
        OCTOSPI1_REGSEC_W { w: self }
    }
}
