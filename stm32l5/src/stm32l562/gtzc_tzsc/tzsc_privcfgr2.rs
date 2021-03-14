#[doc = "Reader of register TZSC_PRIVCFGR2"]
pub type R = crate::R<u32, super::TZSC_PRIVCFGR2>;
#[doc = "Writer for register TZSC_PRIVCFGR2"]
pub type W = crate::W<u32, super::TZSC_PRIVCFGR2>;
#[doc = "Register TZSC_PRIVCFGR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::TZSC_PRIVCFGR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TIM8PRIV`"]
pub type TIM8PRIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM8PRIV`"]
pub struct TIM8PRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM8PRIV_W<'a> {
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
#[doc = "Reader of field `USART1PRIV`"]
pub type USART1PRIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USART1PRIV`"]
pub struct USART1PRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> USART1PRIV_W<'a> {
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
#[doc = "Reader of field `TIM15PRIV`"]
pub type TIM15PRIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM15PRIV`"]
pub struct TIM15PRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM15PRIV_W<'a> {
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
#[doc = "Reader of field `TIM16PRIV`"]
pub type TIM16PRIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM16PRIV`"]
pub struct TIM16PRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM16PRIV_W<'a> {
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
#[doc = "Reader of field `TIM17PRIV`"]
pub type TIM17PRIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM17PRIV`"]
pub struct TIM17PRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM17PRIV_W<'a> {
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
#[doc = "Reader of field `SAI1PRIV`"]
pub type SAI1PRIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SAI1PRIV`"]
pub struct SAI1PRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI1PRIV_W<'a> {
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
#[doc = "Reader of field `SAI2PRIV`"]
pub type SAI2PRIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SAI2PRIV`"]
pub struct SAI2PRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI2PRIV_W<'a> {
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
#[doc = "Reader of field `DFSDM1PRIV`"]
pub type DFSDM1PRIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DFSDM1PRIV`"]
pub struct DFSDM1PRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> DFSDM1PRIV_W<'a> {
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
#[doc = "Reader of field `CRCPRIV`"]
pub type CRCPRIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRCPRIV`"]
pub struct CRCPRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCPRIV_W<'a> {
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
#[doc = "Reader of field `TSCPRIV`"]
pub type TSCPRIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSCPRIV`"]
pub struct TSCPRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> TSCPRIV_W<'a> {
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
#[doc = "Reader of field `ICACHEPRIV`"]
pub type ICACHEPRIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ICACHEPRIV`"]
pub struct ICACHEPRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> ICACHEPRIV_W<'a> {
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
#[doc = "Reader of field `ADCPRIV`"]
pub type ADCPRIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADCPRIV`"]
pub struct ADCPRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCPRIV_W<'a> {
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
#[doc = "Reader of field `AESPRIV`"]
pub type AESPRIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AESPRIV`"]
pub struct AESPRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> AESPRIV_W<'a> {
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
#[doc = "Reader of field `HASHPRIV`"]
pub type HASHPRIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HASHPRIV`"]
pub struct HASHPRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> HASHPRIV_W<'a> {
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
#[doc = "Reader of field `RNGPRIV`"]
pub type RNGPRIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RNGPRIV`"]
pub struct RNGPRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> RNGPRIV_W<'a> {
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
#[doc = "Reader of field `PKAPRIV`"]
pub type PKAPRIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PKAPRIV`"]
pub struct PKAPRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> PKAPRIV_W<'a> {
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
#[doc = "Reader of field `SDMMC1PRIV`"]
pub type SDMMC1PRIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SDMMC1PRIV`"]
pub struct SDMMC1PRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> SDMMC1PRIV_W<'a> {
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
#[doc = "Reader of field `FSMC_REGPRIV`"]
pub type FSMC_REGPRIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSMC_REGPRIV`"]
pub struct FSMC_REGPRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> FSMC_REGPRIV_W<'a> {
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
#[doc = "Reader of field `OCTOSPI1_REGPRIV`"]
pub type OCTOSPI1_REGPRIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OCTOSPI1_REGPRIV`"]
pub struct OCTOSPI1_REGPRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> OCTOSPI1_REGPRIV_W<'a> {
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
    #[doc = "Bit 0 - TIM8PRIV"]
    #[inline(always)]
    pub fn tim8priv(&self) -> TIM8PRIV_R {
        TIM8PRIV_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - USART1PRIV"]
    #[inline(always)]
    pub fn usart1priv(&self) -> USART1PRIV_R {
        USART1PRIV_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TIM15PRIV"]
    #[inline(always)]
    pub fn tim15priv(&self) -> TIM15PRIV_R {
        TIM15PRIV_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TIM16PRIV"]
    #[inline(always)]
    pub fn tim16priv(&self) -> TIM16PRIV_R {
        TIM16PRIV_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TIM17PRIV"]
    #[inline(always)]
    pub fn tim17priv(&self) -> TIM17PRIV_R {
        TIM17PRIV_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - SAI1PRIV"]
    #[inline(always)]
    pub fn sai1priv(&self) -> SAI1PRIV_R {
        SAI1PRIV_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SAI2PRIV"]
    #[inline(always)]
    pub fn sai2priv(&self) -> SAI2PRIV_R {
        SAI2PRIV_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - DFSDM1PRIV"]
    #[inline(always)]
    pub fn dfsdm1priv(&self) -> DFSDM1PRIV_R {
        DFSDM1PRIV_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - CRCPRIV"]
    #[inline(always)]
    pub fn crcpriv(&self) -> CRCPRIV_R {
        CRCPRIV_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - TSCPRIV"]
    #[inline(always)]
    pub fn tscpriv(&self) -> TSCPRIV_R {
        TSCPRIV_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - ICACHEPRIV"]
    #[inline(always)]
    pub fn icachepriv(&self) -> ICACHEPRIV_R {
        ICACHEPRIV_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - ADCPRIV"]
    #[inline(always)]
    pub fn adcpriv(&self) -> ADCPRIV_R {
        ADCPRIV_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - AESPRIV"]
    #[inline(always)]
    pub fn aespriv(&self) -> AESPRIV_R {
        AESPRIV_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - HASHPRIV"]
    #[inline(always)]
    pub fn hashpriv(&self) -> HASHPRIV_R {
        HASHPRIV_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - RNGPRIV"]
    #[inline(always)]
    pub fn rngpriv(&self) -> RNGPRIV_R {
        RNGPRIV_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - PKAPRIV"]
    #[inline(always)]
    pub fn pkapriv(&self) -> PKAPRIV_R {
        PKAPRIV_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - SDMMC1PRIV"]
    #[inline(always)]
    pub fn sdmmc1priv(&self) -> SDMMC1PRIV_R {
        SDMMC1PRIV_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - FSMC_REGPRIV"]
    #[inline(always)]
    pub fn fsmc_regpriv(&self) -> FSMC_REGPRIV_R {
        FSMC_REGPRIV_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - OCTOSPI1_REGRIV"]
    #[inline(always)]
    pub fn octospi1_regpriv(&self) -> OCTOSPI1_REGPRIV_R {
        OCTOSPI1_REGPRIV_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM8PRIV"]
    #[inline(always)]
    pub fn tim8priv(&mut self) -> TIM8PRIV_W {
        TIM8PRIV_W { w: self }
    }
    #[doc = "Bit 1 - USART1PRIV"]
    #[inline(always)]
    pub fn usart1priv(&mut self) -> USART1PRIV_W {
        USART1PRIV_W { w: self }
    }
    #[doc = "Bit 2 - TIM15PRIV"]
    #[inline(always)]
    pub fn tim15priv(&mut self) -> TIM15PRIV_W {
        TIM15PRIV_W { w: self }
    }
    #[doc = "Bit 3 - TIM16PRIV"]
    #[inline(always)]
    pub fn tim16priv(&mut self) -> TIM16PRIV_W {
        TIM16PRIV_W { w: self }
    }
    #[doc = "Bit 4 - TIM17PRIV"]
    #[inline(always)]
    pub fn tim17priv(&mut self) -> TIM17PRIV_W {
        TIM17PRIV_W { w: self }
    }
    #[doc = "Bit 5 - SAI1PRIV"]
    #[inline(always)]
    pub fn sai1priv(&mut self) -> SAI1PRIV_W {
        SAI1PRIV_W { w: self }
    }
    #[doc = "Bit 6 - SAI2PRIV"]
    #[inline(always)]
    pub fn sai2priv(&mut self) -> SAI2PRIV_W {
        SAI2PRIV_W { w: self }
    }
    #[doc = "Bit 7 - DFSDM1PRIV"]
    #[inline(always)]
    pub fn dfsdm1priv(&mut self) -> DFSDM1PRIV_W {
        DFSDM1PRIV_W { w: self }
    }
    #[doc = "Bit 8 - CRCPRIV"]
    #[inline(always)]
    pub fn crcpriv(&mut self) -> CRCPRIV_W {
        CRCPRIV_W { w: self }
    }
    #[doc = "Bit 9 - TSCPRIV"]
    #[inline(always)]
    pub fn tscpriv(&mut self) -> TSCPRIV_W {
        TSCPRIV_W { w: self }
    }
    #[doc = "Bit 10 - ICACHEPRIV"]
    #[inline(always)]
    pub fn icachepriv(&mut self) -> ICACHEPRIV_W {
        ICACHEPRIV_W { w: self }
    }
    #[doc = "Bit 11 - ADCPRIV"]
    #[inline(always)]
    pub fn adcpriv(&mut self) -> ADCPRIV_W {
        ADCPRIV_W { w: self }
    }
    #[doc = "Bit 12 - AESPRIV"]
    #[inline(always)]
    pub fn aespriv(&mut self) -> AESPRIV_W {
        AESPRIV_W { w: self }
    }
    #[doc = "Bit 13 - HASHPRIV"]
    #[inline(always)]
    pub fn hashpriv(&mut self) -> HASHPRIV_W {
        HASHPRIV_W { w: self }
    }
    #[doc = "Bit 14 - RNGPRIV"]
    #[inline(always)]
    pub fn rngpriv(&mut self) -> RNGPRIV_W {
        RNGPRIV_W { w: self }
    }
    #[doc = "Bit 15 - PKAPRIV"]
    #[inline(always)]
    pub fn pkapriv(&mut self) -> PKAPRIV_W {
        PKAPRIV_W { w: self }
    }
    #[doc = "Bit 16 - SDMMC1PRIV"]
    #[inline(always)]
    pub fn sdmmc1priv(&mut self) -> SDMMC1PRIV_W {
        SDMMC1PRIV_W { w: self }
    }
    #[doc = "Bit 17 - FSMC_REGPRIV"]
    #[inline(always)]
    pub fn fsmc_regpriv(&mut self) -> FSMC_REGPRIV_W {
        FSMC_REGPRIV_W { w: self }
    }
    #[doc = "Bit 18 - OCTOSPI1_REGRIV"]
    #[inline(always)]
    pub fn octospi1_regpriv(&mut self) -> OCTOSPI1_REGPRIV_W {
        OCTOSPI1_REGPRIV_W { w: self }
    }
}
