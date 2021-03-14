#[doc = "Reader of register CSR"]
pub type R = crate::R<u32, super::CSR>;
#[doc = "Writer for register CSR"]
pub type W = crate::W<u32, super::CSR>;
#[doc = "Register CSR `reset()`'s with value 0"]
impl crate::ResetValue for super::CSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TSUSP`"]
pub type TSUSP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSUSP`"]
pub struct TSUSP_W<'a> {
    w: &'a mut W,
}
impl<'a> TSUSP_W<'a> {
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
#[doc = "Reader of field `CAIF`"]
pub type CAIF_R = crate::R<bool, bool>;
#[doc = "Reader of field `CAIE`"]
pub type CAIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAIE`"]
pub struct CAIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CAIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `RCH13`"]
pub type RCH13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RCH13`"]
pub struct RCH13_W<'a> {
    w: &'a mut W,
}
impl<'a> RCH13_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `FCH8`"]
pub type FCH8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FCH8`"]
pub struct FCH8_W<'a> {
    w: &'a mut W,
}
impl<'a> FCH8_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `FCH3`"]
pub type FCH3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FCH3`"]
pub struct FCH3_W<'a> {
    w: &'a mut W,
}
impl<'a> FCH3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `OUTSEL`"]
pub type OUTSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OUTSEL`"]
pub struct OUTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 21)) | (((value as u32) & 0x07) << 21);
        self.w
    }
}
#[doc = "Reader of field `INSEL`"]
pub type INSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INSEL`"]
pub struct INSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> INSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | (((value as u32) & 0x07) << 18);
        self.w
    }
}
#[doc = "Reader of field `WNDWE`"]
pub type WNDWE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WNDWE`"]
pub struct WNDWE_W<'a> {
    w: &'a mut W,
}
impl<'a> WNDWE_W<'a> {
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
#[doc = "Reader of field `VREFOUTEN`"]
pub type VREFOUTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VREFOUTEN`"]
pub struct VREFOUTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> VREFOUTEN_W<'a> {
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
#[doc = "Reader of field `CMP2OUT`"]
pub type CMP2OUT_R = crate::R<bool, bool>;
#[doc = "Reader of field `SPEED`"]
pub type SPEED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPEED`"]
pub struct SPEED_W<'a> {
    w: &'a mut W,
}
impl<'a> SPEED_W<'a> {
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
#[doc = "Reader of field `CMP1OUT`"]
pub type CMP1OUT_R = crate::R<bool, bool>;
#[doc = "Reader of field `SW1`"]
pub type SW1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SW1`"]
pub struct SW1_W<'a> {
    w: &'a mut W,
}
impl<'a> SW1_W<'a> {
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
#[doc = "Reader of field `CMP1EN`"]
pub type CMP1EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMP1EN`"]
pub struct CMP1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP1EN_W<'a> {
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
#[doc = "Reader of field `PD400K`"]
pub type PD400K_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PD400K`"]
pub struct PD400K_W<'a> {
    w: &'a mut W,
}
impl<'a> PD400K_W<'a> {
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
#[doc = "Reader of field `PD10K`"]
pub type PD10K_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PD10K`"]
pub struct PD10K_W<'a> {
    w: &'a mut W,
}
impl<'a> PD10K_W<'a> {
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
#[doc = "Reader of field `PU400K`"]
pub type PU400K_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PU400K`"]
pub struct PU400K_W<'a> {
    w: &'a mut W,
}
impl<'a> PU400K_W<'a> {
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
#[doc = "Reader of field `PU10K`"]
pub type PU10K_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PU10K`"]
pub struct PU10K_W<'a> {
    w: &'a mut W,
}
impl<'a> PU10K_W<'a> {
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
impl R {
    #[doc = "Bit 31 - Suspend Timer Mode"]
    #[inline(always)]
    pub fn tsusp(&self) -> TSUSP_R {
        TSUSP_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Channel acquisition interrupt flag"]
    #[inline(always)]
    pub fn caif(&self) -> CAIF_R {
        CAIF_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Channel Acquisition Interrupt Enable / Clear"]
    #[inline(always)]
    pub fn caie(&self) -> CAIE_R {
        CAIE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Select GPIO port PC3 as re-routed ADC input channel CH13."]
    #[inline(always)]
    pub fn rch13(&self) -> RCH13_R {
        RCH13_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Select GPIO port PB0 as fast ADC input channel CH8."]
    #[inline(always)]
    pub fn fch8(&self) -> FCH8_R {
        FCH8_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Select GPIO port PA3 as fast ADC input channel CH3."]
    #[inline(always)]
    pub fn fch3(&self) -> FCH3_R {
        FCH3_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bits 21:23 - Comparator 2 output selection"]
    #[inline(always)]
    pub fn outsel(&self) -> OUTSEL_R {
        OUTSEL_R::new(((self.bits >> 21) & 0x07) as u8)
    }
    #[doc = "Bits 18:20 - Inverted input selection"]
    #[inline(always)]
    pub fn insel(&self) -> INSEL_R {
        INSEL_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bit 17 - Window mode enable"]
    #[inline(always)]
    pub fn wndwe(&self) -> WNDWE_R {
        WNDWE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - VREFINT output enable"]
    #[inline(always)]
    pub fn vrefouten(&self) -> VREFOUTEN_R {
        VREFOUTEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Comparator 2 output"]
    #[inline(always)]
    pub fn cmp2out(&self) -> CMP2OUT_R {
        CMP2OUT_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Comparator 2 speed mode"]
    #[inline(always)]
    pub fn speed(&self) -> SPEED_R {
        SPEED_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Comparator 1 output"]
    #[inline(always)]
    pub fn cmp1out(&self) -> CMP1OUT_R {
        CMP1OUT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 5 - SW1 analog switch enable"]
    #[inline(always)]
    pub fn sw1(&self) -> SW1_R {
        SW1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Comparator 1 enable"]
    #[inline(always)]
    pub fn cmp1en(&self) -> CMP1EN_R {
        CMP1EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 400 kO pull-down resistor"]
    #[inline(always)]
    pub fn pd400k(&self) -> PD400K_R {
        PD400K_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 10 kO pull-down resistor"]
    #[inline(always)]
    pub fn pd10k(&self) -> PD10K_R {
        PD10K_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 400 kO pull-up resistor"]
    #[inline(always)]
    pub fn pu400k(&self) -> PU400K_R {
        PU400K_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 10 kO pull-up resistor"]
    #[inline(always)]
    pub fn pu10k(&self) -> PU10K_R {
        PU10K_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Suspend Timer Mode"]
    #[inline(always)]
    pub fn tsusp(&mut self) -> TSUSP_W {
        TSUSP_W { w: self }
    }
    #[doc = "Bit 29 - Channel Acquisition Interrupt Enable / Clear"]
    #[inline(always)]
    pub fn caie(&mut self) -> CAIE_W {
        CAIE_W { w: self }
    }
    #[doc = "Bit 28 - Select GPIO port PC3 as re-routed ADC input channel CH13."]
    #[inline(always)]
    pub fn rch13(&mut self) -> RCH13_W {
        RCH13_W { w: self }
    }
    #[doc = "Bit 27 - Select GPIO port PB0 as fast ADC input channel CH8."]
    #[inline(always)]
    pub fn fch8(&mut self) -> FCH8_W {
        FCH8_W { w: self }
    }
    #[doc = "Bit 26 - Select GPIO port PA3 as fast ADC input channel CH3."]
    #[inline(always)]
    pub fn fch3(&mut self) -> FCH3_W {
        FCH3_W { w: self }
    }
    #[doc = "Bits 21:23 - Comparator 2 output selection"]
    #[inline(always)]
    pub fn outsel(&mut self) -> OUTSEL_W {
        OUTSEL_W { w: self }
    }
    #[doc = "Bits 18:20 - Inverted input selection"]
    #[inline(always)]
    pub fn insel(&mut self) -> INSEL_W {
        INSEL_W { w: self }
    }
    #[doc = "Bit 17 - Window mode enable"]
    #[inline(always)]
    pub fn wndwe(&mut self) -> WNDWE_W {
        WNDWE_W { w: self }
    }
    #[doc = "Bit 16 - VREFINT output enable"]
    #[inline(always)]
    pub fn vrefouten(&mut self) -> VREFOUTEN_W {
        VREFOUTEN_W { w: self }
    }
    #[doc = "Bit 12 - Comparator 2 speed mode"]
    #[inline(always)]
    pub fn speed(&mut self) -> SPEED_W {
        SPEED_W { w: self }
    }
    #[doc = "Bit 5 - SW1 analog switch enable"]
    #[inline(always)]
    pub fn sw1(&mut self) -> SW1_W {
        SW1_W { w: self }
    }
    #[doc = "Bit 4 - Comparator 1 enable"]
    #[inline(always)]
    pub fn cmp1en(&mut self) -> CMP1EN_W {
        CMP1EN_W { w: self }
    }
    #[doc = "Bit 3 - 400 kO pull-down resistor"]
    #[inline(always)]
    pub fn pd400k(&mut self) -> PD400K_W {
        PD400K_W { w: self }
    }
    #[doc = "Bit 2 - 10 kO pull-down resistor"]
    #[inline(always)]
    pub fn pd10k(&mut self) -> PD10K_W {
        PD10K_W { w: self }
    }
    #[doc = "Bit 1 - 400 kO pull-up resistor"]
    #[inline(always)]
    pub fn pu400k(&mut self) -> PU400K_W {
        PU400K_W { w: self }
    }
    #[doc = "Bit 0 - 10 kO pull-up resistor"]
    #[inline(always)]
    pub fn pu10k(&mut self) -> PU10K_W {
        PU10K_W { w: self }
    }
}
