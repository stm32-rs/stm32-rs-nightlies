#[doc = "Register `OR1` reader"]
pub struct R(crate::R<OR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OR1` writer"]
pub struct W(crate::W<OR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<OR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Input Capture 1 remap\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TI1_RMP_A {
    #[doc = "0: TIM1 input capture 1 is connected to I/O"]
    IO = 0,
    #[doc = "1: TIM1 input capture 1 is connected to COMP1 output"]
    COMP1 = 1,
}
impl From<TI1_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: TI1_RMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TI1_RMP` reader - Input Capture 1 remap"]
pub struct TI1_RMP_R(crate::FieldReader<bool, TI1_RMP_A>);
impl TI1_RMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        TI1_RMP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TI1_RMP_A {
        match self.bits {
            false => TI1_RMP_A::IO,
            true => TI1_RMP_A::COMP1,
        }
    }
    #[doc = "Checks if the value of the field is `IO`"]
    #[inline(always)]
    pub fn is_io(&self) -> bool {
        **self == TI1_RMP_A::IO
    }
    #[doc = "Checks if the value of the field is `COMP1`"]
    #[inline(always)]
    pub fn is_comp1(&self) -> bool {
        **self == TI1_RMP_A::COMP1
    }
}
impl core::ops::Deref for TI1_RMP_R {
    type Target = crate::FieldReader<bool, TI1_RMP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TI1_RMP` writer - Input Capture 1 remap"]
pub struct TI1_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> TI1_RMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TI1_RMP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "TIM1 input capture 1 is connected to I/O"]
    #[inline(always)]
    pub fn io(self) -> &'a mut W {
        self.variant(TI1_RMP_A::IO)
    }
    #[doc = "TIM1 input capture 1 is connected to COMP1 output"]
    #[inline(always)]
    pub fn comp1(self) -> &'a mut W {
        self.variant(TI1_RMP_A::COMP1)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "TIM1_ETR_ADC1 remapping capability\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TIM1_ETR_ADC1_RMP_A {
    #[doc = "0: TIM1_ETR is not connected to ADC AWDx (must be selected when the ETR comes from the ETR input pin)"]
    SELECT = 0,
    #[doc = "1: TIM1_ETR is connected to ADC AWD1"]
    ADC_AWD1 = 1,
    #[doc = "2: TIM1_ETR is connected to ADC AWD2"]
    ADC_AWD2 = 2,
    #[doc = "3: TIM1_ETR is connected to ADC AWD3"]
    ADC_AWD3 = 3,
}
impl From<TIM1_ETR_ADC1_RMP_A> for u8 {
    #[inline(always)]
    fn from(variant: TIM1_ETR_ADC1_RMP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TIM1_ETR_ADC1_RMP` reader - TIM1_ETR_ADC1 remapping capability"]
pub struct TIM1_ETR_ADC1_RMP_R(crate::FieldReader<u8, TIM1_ETR_ADC1_RMP_A>);
impl TIM1_ETR_ADC1_RMP_R {
    pub(crate) fn new(bits: u8) -> Self {
        TIM1_ETR_ADC1_RMP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM1_ETR_ADC1_RMP_A {
        match self.bits {
            0 => TIM1_ETR_ADC1_RMP_A::SELECT,
            1 => TIM1_ETR_ADC1_RMP_A::ADC_AWD1,
            2 => TIM1_ETR_ADC1_RMP_A::ADC_AWD2,
            3 => TIM1_ETR_ADC1_RMP_A::ADC_AWD3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SELECT`"]
    #[inline(always)]
    pub fn is_select(&self) -> bool {
        **self == TIM1_ETR_ADC1_RMP_A::SELECT
    }
    #[doc = "Checks if the value of the field is `ADC_AWD1`"]
    #[inline(always)]
    pub fn is_adc_awd1(&self) -> bool {
        **self == TIM1_ETR_ADC1_RMP_A::ADC_AWD1
    }
    #[doc = "Checks if the value of the field is `ADC_AWD2`"]
    #[inline(always)]
    pub fn is_adc_awd2(&self) -> bool {
        **self == TIM1_ETR_ADC1_RMP_A::ADC_AWD2
    }
    #[doc = "Checks if the value of the field is `ADC_AWD3`"]
    #[inline(always)]
    pub fn is_adc_awd3(&self) -> bool {
        **self == TIM1_ETR_ADC1_RMP_A::ADC_AWD3
    }
}
impl core::ops::Deref for TIM1_ETR_ADC1_RMP_R {
    type Target = crate::FieldReader<u8, TIM1_ETR_ADC1_RMP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM1_ETR_ADC1_RMP` writer - TIM1_ETR_ADC1 remapping capability"]
pub struct TIM1_ETR_ADC1_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM1_ETR_ADC1_RMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM1_ETR_ADC1_RMP_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "TIM1_ETR is not connected to ADC AWDx (must be selected when the ETR comes from the ETR input pin)"]
    #[inline(always)]
    pub fn select(self) -> &'a mut W {
        self.variant(TIM1_ETR_ADC1_RMP_A::SELECT)
    }
    #[doc = "TIM1_ETR is connected to ADC AWD1"]
    #[inline(always)]
    pub fn adc_awd1(self) -> &'a mut W {
        self.variant(TIM1_ETR_ADC1_RMP_A::ADC_AWD1)
    }
    #[doc = "TIM1_ETR is connected to ADC AWD2"]
    #[inline(always)]
    pub fn adc_awd2(self) -> &'a mut W {
        self.variant(TIM1_ETR_ADC1_RMP_A::ADC_AWD2)
    }
    #[doc = "TIM1_ETR is connected to ADC AWD3"]
    #[inline(always)]
    pub fn adc_awd3(self) -> &'a mut W {
        self.variant(TIM1_ETR_ADC1_RMP_A::ADC_AWD3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bit 4 - Input Capture 1 remap"]
    #[inline(always)]
    pub fn ti1_rmp(&self) -> TI1_RMP_R {
        TI1_RMP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 0:1 - TIM1_ETR_ADC1 remapping capability"]
    #[inline(always)]
    pub fn tim1_etr_adc1_rmp(&self) -> TIM1_ETR_ADC1_RMP_R {
        TIM1_ETR_ADC1_RMP_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 4 - Input Capture 1 remap"]
    #[inline(always)]
    pub fn ti1_rmp(&mut self) -> TI1_RMP_W {
        TI1_RMP_W { w: self }
    }
    #[doc = "Bits 0:1 - TIM1_ETR_ADC1 remapping capability"]
    #[inline(always)]
    pub fn tim1_etr_adc1_rmp(&mut self) -> TIM1_ETR_ADC1_RMP_W {
        TIM1_ETR_ADC1_RMP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "option register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [or1](index.html) module"]
pub struct OR1_SPEC;
impl crate::RegisterSpec for OR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [or1::R](R) reader structure"]
impl crate::Readable for OR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [or1::W](W) writer structure"]
impl crate::Writable for OR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OR1 to value 0"]
impl crate::Resettable for OR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
