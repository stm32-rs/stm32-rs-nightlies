#[doc = "Register `CFGR1` reader"]
pub struct R(crate::R<CFGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGR1` writer"]
pub struct W(crate::W<CFGR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR1_SPEC>;
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
impl From<crate::W<CFGR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Memory mapping selection bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MEM_MODE_A {
    #[doc = "0: Main Flash memory mapped at 0x0000_0000"]
    MAINFLASH = 0,
    #[doc = "1: System Flash memory mapped at 0x0000_0000"]
    SYSTEMFLASH = 1,
    #[doc = "3: Embedded SRAM mapped at 0x0000_0000"]
    SRAM = 3,
    #[doc = "4: FMC Bank (Only the first  two banks) (Available on STM32F303xD/E only)"]
    FMC = 4,
}
impl From<MEM_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MEM_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MEM_MODE` reader - Memory mapping selection bits"]
pub struct MEM_MODE_R(crate::FieldReader<u8, MEM_MODE_A>);
impl MEM_MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        MEM_MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MEM_MODE_A> {
        match self.bits {
            0 => Some(MEM_MODE_A::MAINFLASH),
            1 => Some(MEM_MODE_A::SYSTEMFLASH),
            3 => Some(MEM_MODE_A::SRAM),
            4 => Some(MEM_MODE_A::FMC),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MAINFLASH`"]
    #[inline(always)]
    pub fn is_main_flash(&self) -> bool {
        **self == MEM_MODE_A::MAINFLASH
    }
    #[doc = "Checks if the value of the field is `SYSTEMFLASH`"]
    #[inline(always)]
    pub fn is_system_flash(&self) -> bool {
        **self == MEM_MODE_A::SYSTEMFLASH
    }
    #[doc = "Checks if the value of the field is `SRAM`"]
    #[inline(always)]
    pub fn is_sram(&self) -> bool {
        **self == MEM_MODE_A::SRAM
    }
    #[doc = "Checks if the value of the field is `FMC`"]
    #[inline(always)]
    pub fn is_fmc(&self) -> bool {
        **self == MEM_MODE_A::FMC
    }
}
impl core::ops::Deref for MEM_MODE_R {
    type Target = crate::FieldReader<u8, MEM_MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM_MODE` writer - Memory mapping selection bits"]
pub struct MEM_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MEM_MODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Main Flash memory mapped at 0x0000_0000"]
    #[inline(always)]
    pub fn main_flash(self) -> &'a mut W {
        self.variant(MEM_MODE_A::MAINFLASH)
    }
    #[doc = "System Flash memory mapped at 0x0000_0000"]
    #[inline(always)]
    pub fn system_flash(self) -> &'a mut W {
        self.variant(MEM_MODE_A::SYSTEMFLASH)
    }
    #[doc = "Embedded SRAM mapped at 0x0000_0000"]
    #[inline(always)]
    pub fn sram(self) -> &'a mut W {
        self.variant(MEM_MODE_A::SRAM)
    }
    #[doc = "FMC Bank (Only the first two banks) (Available on STM32F303xD/E only)"]
    #[inline(always)]
    pub fn fmc(self) -> &'a mut W {
        self.variant(MEM_MODE_A::FMC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "USB interrupt remap\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB_IT_RMP_A {
    #[doc = "0: USB_HP, USB_LP and USB_WAKEUP interrupts are mapped on interrupt lines 19, 20 and 42 respectively"]
    NOTREMAPPED = 0,
    #[doc = "1: USB_HP, USB_LP and USB_WAKEUP interrupts are mapped on interrupt lines 74, 75 and 76 respectively"]
    REMAPPED = 1,
}
impl From<USB_IT_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: USB_IT_RMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USB_IT_RMP` reader - USB interrupt remap"]
pub struct USB_IT_RMP_R(crate::FieldReader<bool, USB_IT_RMP_A>);
impl USB_IT_RMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        USB_IT_RMP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USB_IT_RMP_A {
        match self.bits {
            false => USB_IT_RMP_A::NOTREMAPPED,
            true => USB_IT_RMP_A::REMAPPED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTREMAPPED`"]
    #[inline(always)]
    pub fn is_not_remapped(&self) -> bool {
        **self == USB_IT_RMP_A::NOTREMAPPED
    }
    #[doc = "Checks if the value of the field is `REMAPPED`"]
    #[inline(always)]
    pub fn is_remapped(&self) -> bool {
        **self == USB_IT_RMP_A::REMAPPED
    }
}
impl core::ops::Deref for USB_IT_RMP_R {
    type Target = crate::FieldReader<bool, USB_IT_RMP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB_IT_RMP` writer - USB interrupt remap"]
pub struct USB_IT_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_IT_RMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB_IT_RMP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "USB_HP, USB_LP and USB_WAKEUP interrupts are mapped on interrupt lines 19, 20 and 42 respectively"]
    #[inline(always)]
    pub fn not_remapped(self) -> &'a mut W {
        self.variant(USB_IT_RMP_A::NOTREMAPPED)
    }
    #[doc = "USB_HP, USB_LP and USB_WAKEUP interrupts are mapped on interrupt lines 74, 75 and 76 respectively"]
    #[inline(always)]
    pub fn remapped(self) -> &'a mut W {
        self.variant(USB_IT_RMP_A::REMAPPED)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Timer 1 ITR3 selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIM1_ITR3_RMP_A {
    #[doc = "0: TIM1_ITR3 = TIM4_TRGO in STM32F303xB/C and STM32F358xC devices"]
    NOTREMAPPED = 0,
    #[doc = "1: TIM1_ITR3 = TIM17_OC"]
    REMAPPED = 1,
}
impl From<TIM1_ITR3_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: TIM1_ITR3_RMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM1_ITR3_RMP` reader - Timer 1 ITR3 selection"]
pub struct TIM1_ITR3_RMP_R(crate::FieldReader<bool, TIM1_ITR3_RMP_A>);
impl TIM1_ITR3_RMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM1_ITR3_RMP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM1_ITR3_RMP_A {
        match self.bits {
            false => TIM1_ITR3_RMP_A::NOTREMAPPED,
            true => TIM1_ITR3_RMP_A::REMAPPED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTREMAPPED`"]
    #[inline(always)]
    pub fn is_not_remapped(&self) -> bool {
        **self == TIM1_ITR3_RMP_A::NOTREMAPPED
    }
    #[doc = "Checks if the value of the field is `REMAPPED`"]
    #[inline(always)]
    pub fn is_remapped(&self) -> bool {
        **self == TIM1_ITR3_RMP_A::REMAPPED
    }
}
impl core::ops::Deref for TIM1_ITR3_RMP_R {
    type Target = crate::FieldReader<bool, TIM1_ITR3_RMP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM1_ITR3_RMP` writer - Timer 1 ITR3 selection"]
pub struct TIM1_ITR3_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM1_ITR3_RMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM1_ITR3_RMP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "TIM1_ITR3 = TIM4_TRGO in STM32F303xB/C and STM32F358xC devices"]
    #[inline(always)]
    pub fn not_remapped(self) -> &'a mut W {
        self.variant(TIM1_ITR3_RMP_A::NOTREMAPPED)
    }
    #[doc = "TIM1_ITR3 = TIM17_OC"]
    #[inline(always)]
    pub fn remapped(self) -> &'a mut W {
        self.variant(TIM1_ITR3_RMP_A::REMAPPED)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "DAC trigger remap (when TSEL = 001)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DAC1_TRIG_RMP_A {
    #[doc = "0: DAC trigger is TIM8_TRGO in STM32F303xB/C and STM32F358xC devices"]
    NOTREMAPPED = 0,
    #[doc = "1: DAC trigger is TIM3_TRGO"]
    REMAPPED = 1,
}
impl From<DAC1_TRIG_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: DAC1_TRIG_RMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DAC1_TRIG_RMP` reader - DAC trigger remap (when TSEL = 001)"]
pub struct DAC1_TRIG_RMP_R(crate::FieldReader<bool, DAC1_TRIG_RMP_A>);
impl DAC1_TRIG_RMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DAC1_TRIG_RMP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAC1_TRIG_RMP_A {
        match self.bits {
            false => DAC1_TRIG_RMP_A::NOTREMAPPED,
            true => DAC1_TRIG_RMP_A::REMAPPED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTREMAPPED`"]
    #[inline(always)]
    pub fn is_not_remapped(&self) -> bool {
        **self == DAC1_TRIG_RMP_A::NOTREMAPPED
    }
    #[doc = "Checks if the value of the field is `REMAPPED`"]
    #[inline(always)]
    pub fn is_remapped(&self) -> bool {
        **self == DAC1_TRIG_RMP_A::REMAPPED
    }
}
impl core::ops::Deref for DAC1_TRIG_RMP_R {
    type Target = crate::FieldReader<bool, DAC1_TRIG_RMP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAC1_TRIG_RMP` writer - DAC trigger remap (when TSEL = 001)"]
pub struct DAC1_TRIG_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC1_TRIG_RMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DAC1_TRIG_RMP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DAC trigger is TIM8_TRGO in STM32F303xB/C and STM32F358xC devices"]
    #[inline(always)]
    pub fn not_remapped(self) -> &'a mut W {
        self.variant(DAC1_TRIG_RMP_A::NOTREMAPPED)
    }
    #[doc = "DAC trigger is TIM3_TRGO"]
    #[inline(always)]
    pub fn remapped(self) -> &'a mut W {
        self.variant(DAC1_TRIG_RMP_A::REMAPPED)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "ADC24 DMA remapping bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC2_DMA_RMP_A {
    #[doc = "0: ADC24 DMA requests mapped on DMA2 channels 1 and 2"]
    NOTREMAPPED = 0,
    #[doc = "1: ADC24 DMA requests mapped on DMA2 channels 3 and 4"]
    REMAPPED = 1,
}
impl From<ADC2_DMA_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: ADC2_DMA_RMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC2_DMA_RMP` reader - ADC24 DMA remapping bit"]
pub struct ADC2_DMA_RMP_R(crate::FieldReader<bool, ADC2_DMA_RMP_A>);
impl ADC2_DMA_RMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC2_DMA_RMP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC2_DMA_RMP_A {
        match self.bits {
            false => ADC2_DMA_RMP_A::NOTREMAPPED,
            true => ADC2_DMA_RMP_A::REMAPPED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTREMAPPED`"]
    #[inline(always)]
    pub fn is_not_remapped(&self) -> bool {
        **self == ADC2_DMA_RMP_A::NOTREMAPPED
    }
    #[doc = "Checks if the value of the field is `REMAPPED`"]
    #[inline(always)]
    pub fn is_remapped(&self) -> bool {
        **self == ADC2_DMA_RMP_A::REMAPPED
    }
}
impl core::ops::Deref for ADC2_DMA_RMP_R {
    type Target = crate::FieldReader<bool, ADC2_DMA_RMP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC2_DMA_RMP` writer - ADC24 DMA remapping bit"]
pub struct ADC2_DMA_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC2_DMA_RMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC2_DMA_RMP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "ADC24 DMA requests mapped on DMA2 channels 1 and 2"]
    #[inline(always)]
    pub fn not_remapped(self) -> &'a mut W {
        self.variant(ADC2_DMA_RMP_A::NOTREMAPPED)
    }
    #[doc = "ADC24 DMA requests mapped on DMA2 channels 3 and 4"]
    #[inline(always)]
    pub fn remapped(self) -> &'a mut W {
        self.variant(ADC2_DMA_RMP_A::REMAPPED)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "TIM16 DMA request remapping bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIM16_DMA_RMP_A {
    #[doc = "0: TIM16_CH1 and TIM16_UP DMA requests mapped on DMA channel 3"]
    NOTREMAPPED = 0,
    #[doc = "1: TIM16_CH1 and TIM16_UP DMA requests mapped on DMA channel 4"]
    REMAPPED = 1,
}
impl From<TIM16_DMA_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: TIM16_DMA_RMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM16_DMA_RMP` reader - TIM16 DMA request remapping bit"]
pub struct TIM16_DMA_RMP_R(crate::FieldReader<bool, TIM16_DMA_RMP_A>);
impl TIM16_DMA_RMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM16_DMA_RMP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM16_DMA_RMP_A {
        match self.bits {
            false => TIM16_DMA_RMP_A::NOTREMAPPED,
            true => TIM16_DMA_RMP_A::REMAPPED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTREMAPPED`"]
    #[inline(always)]
    pub fn is_not_remapped(&self) -> bool {
        **self == TIM16_DMA_RMP_A::NOTREMAPPED
    }
    #[doc = "Checks if the value of the field is `REMAPPED`"]
    #[inline(always)]
    pub fn is_remapped(&self) -> bool {
        **self == TIM16_DMA_RMP_A::REMAPPED
    }
}
impl core::ops::Deref for TIM16_DMA_RMP_R {
    type Target = crate::FieldReader<bool, TIM16_DMA_RMP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM16_DMA_RMP` writer - TIM16 DMA request remapping bit"]
pub struct TIM16_DMA_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM16_DMA_RMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM16_DMA_RMP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "TIM16_CH1 and TIM16_UP DMA requests mapped on DMA channel 3"]
    #[inline(always)]
    pub fn not_remapped(self) -> &'a mut W {
        self.variant(TIM16_DMA_RMP_A::NOTREMAPPED)
    }
    #[doc = "TIM16_CH1 and TIM16_UP DMA requests mapped on DMA channel 4"]
    #[inline(always)]
    pub fn remapped(self) -> &'a mut W {
        self.variant(TIM16_DMA_RMP_A::REMAPPED)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "TIM17 DMA request remapping bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIM17_DMA_RMP_A {
    #[doc = "0: TIM17_CH1 and TIM17_UP DMA requests mapped on DMA channel 1"]
    NOTREMAPPED = 0,
    #[doc = "1: TIM17_CH1 and TIM17_UP DMA requests mapped on DMA channel 2"]
    REMAPPED = 1,
}
impl From<TIM17_DMA_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: TIM17_DMA_RMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM17_DMA_RMP` reader - TIM17 DMA request remapping bit"]
pub struct TIM17_DMA_RMP_R(crate::FieldReader<bool, TIM17_DMA_RMP_A>);
impl TIM17_DMA_RMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM17_DMA_RMP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM17_DMA_RMP_A {
        match self.bits {
            false => TIM17_DMA_RMP_A::NOTREMAPPED,
            true => TIM17_DMA_RMP_A::REMAPPED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTREMAPPED`"]
    #[inline(always)]
    pub fn is_not_remapped(&self) -> bool {
        **self == TIM17_DMA_RMP_A::NOTREMAPPED
    }
    #[doc = "Checks if the value of the field is `REMAPPED`"]
    #[inline(always)]
    pub fn is_remapped(&self) -> bool {
        **self == TIM17_DMA_RMP_A::REMAPPED
    }
}
impl core::ops::Deref for TIM17_DMA_RMP_R {
    type Target = crate::FieldReader<bool, TIM17_DMA_RMP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM17_DMA_RMP` writer - TIM17 DMA request remapping bit"]
pub struct TIM17_DMA_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM17_DMA_RMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM17_DMA_RMP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "TIM17_CH1 and TIM17_UP DMA requests mapped on DMA channel 1"]
    #[inline(always)]
    pub fn not_remapped(self) -> &'a mut W {
        self.variant(TIM17_DMA_RMP_A::NOTREMAPPED)
    }
    #[doc = "TIM17_CH1 and TIM17_UP DMA requests mapped on DMA channel 2"]
    #[inline(always)]
    pub fn remapped(self) -> &'a mut W {
        self.variant(TIM17_DMA_RMP_A::REMAPPED)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "TIM6 and DAC1 DMA request remapping bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIM6_DAC1_CH1_DMA_RMP_A {
    #[doc = "0: TIM6_UP and DAC_CH1 DMA requests mapped on DMA2 channel 3 in STM32F303xB/C and STM32F358xC"]
    NOTREMAPPED = 0,
    #[doc = "1: TIM6_UP and DAC_CH1 DMA requests mapped on DMA1 channel 3"]
    REMAPPED = 1,
}
impl From<TIM6_DAC1_CH1_DMA_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: TIM6_DAC1_CH1_DMA_RMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM6_DAC1_CH1_DMA_RMP` reader - TIM6 and DAC1 DMA request remapping bit"]
pub struct TIM6_DAC1_CH1_DMA_RMP_R(crate::FieldReader<bool, TIM6_DAC1_CH1_DMA_RMP_A>);
impl TIM6_DAC1_CH1_DMA_RMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM6_DAC1_CH1_DMA_RMP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM6_DAC1_CH1_DMA_RMP_A {
        match self.bits {
            false => TIM6_DAC1_CH1_DMA_RMP_A::NOTREMAPPED,
            true => TIM6_DAC1_CH1_DMA_RMP_A::REMAPPED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTREMAPPED`"]
    #[inline(always)]
    pub fn is_not_remapped(&self) -> bool {
        **self == TIM6_DAC1_CH1_DMA_RMP_A::NOTREMAPPED
    }
    #[doc = "Checks if the value of the field is `REMAPPED`"]
    #[inline(always)]
    pub fn is_remapped(&self) -> bool {
        **self == TIM6_DAC1_CH1_DMA_RMP_A::REMAPPED
    }
}
impl core::ops::Deref for TIM6_DAC1_CH1_DMA_RMP_R {
    type Target = crate::FieldReader<bool, TIM6_DAC1_CH1_DMA_RMP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM6_DAC1_CH1_DMA_RMP` writer - TIM6 and DAC1 DMA request remapping bit"]
pub struct TIM6_DAC1_CH1_DMA_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM6_DAC1_CH1_DMA_RMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM6_DAC1_CH1_DMA_RMP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "TIM6_UP and DAC_CH1 DMA requests mapped on DMA2 channel 3 in STM32F303xB/C and STM32F358xC"]
    #[inline(always)]
    pub fn not_remapped(self) -> &'a mut W {
        self.variant(TIM6_DAC1_CH1_DMA_RMP_A::NOTREMAPPED)
    }
    #[doc = "TIM6_UP and DAC_CH1 DMA requests mapped on DMA1 channel 3"]
    #[inline(always)]
    pub fn remapped(self) -> &'a mut W {
        self.variant(TIM6_DAC1_CH1_DMA_RMP_A::REMAPPED)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "TIM7 and DAC2 DMA request remapping bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIM7_DAC1_CH2_DMA_RMP_A {
    #[doc = "0: TIM7_UP and DAC_CH2 DMA requests mapped on DMA2 channel 4 in STM32F303xB/C and STM32F358xC devices"]
    NOTREMAPPED = 0,
    #[doc = "1: TIM7_UP and DAC_CH2 DMA requests mapped on DMA1 channel 4"]
    REMAPPED = 1,
}
impl From<TIM7_DAC1_CH2_DMA_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: TIM7_DAC1_CH2_DMA_RMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM7_DAC1_CH2_DMA_RMP` reader - TIM7 and DAC2 DMA request remapping bit"]
pub struct TIM7_DAC1_CH2_DMA_RMP_R(crate::FieldReader<bool, TIM7_DAC1_CH2_DMA_RMP_A>);
impl TIM7_DAC1_CH2_DMA_RMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM7_DAC1_CH2_DMA_RMP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM7_DAC1_CH2_DMA_RMP_A {
        match self.bits {
            false => TIM7_DAC1_CH2_DMA_RMP_A::NOTREMAPPED,
            true => TIM7_DAC1_CH2_DMA_RMP_A::REMAPPED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTREMAPPED`"]
    #[inline(always)]
    pub fn is_not_remapped(&self) -> bool {
        **self == TIM7_DAC1_CH2_DMA_RMP_A::NOTREMAPPED
    }
    #[doc = "Checks if the value of the field is `REMAPPED`"]
    #[inline(always)]
    pub fn is_remapped(&self) -> bool {
        **self == TIM7_DAC1_CH2_DMA_RMP_A::REMAPPED
    }
}
impl core::ops::Deref for TIM7_DAC1_CH2_DMA_RMP_R {
    type Target = crate::FieldReader<bool, TIM7_DAC1_CH2_DMA_RMP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM7_DAC1_CH2_DMA_RMP` writer - TIM7 and DAC2 DMA request remapping bit"]
pub struct TIM7_DAC1_CH2_DMA_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM7_DAC1_CH2_DMA_RMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM7_DAC1_CH2_DMA_RMP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "TIM7_UP and DAC_CH2 DMA requests mapped on DMA2 channel 4 in STM32F303xB/C and STM32F358xC devices"]
    #[inline(always)]
    pub fn not_remapped(self) -> &'a mut W {
        self.variant(TIM7_DAC1_CH2_DMA_RMP_A::NOTREMAPPED)
    }
    #[doc = "TIM7_UP and DAC_CH2 DMA requests mapped on DMA1 channel 4"]
    #[inline(always)]
    pub fn remapped(self) -> &'a mut W {
        self.variant(TIM7_DAC1_CH2_DMA_RMP_A::REMAPPED)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Fast Mode Plus (FM+) driving capability activation bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C_PB6_FMP_A {
    #[doc = "0: PB6 pin operate in standard mode"]
    STANDARD = 0,
    #[doc = "1: I2C FM+ mode enabled on PB6 and the Speed control is bypassed"]
    FMP = 1,
}
impl From<I2C_PB6_FMP_A> for bool {
    #[inline(always)]
    fn from(variant: I2C_PB6_FMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C_PB6_FMP` reader - Fast Mode Plus (FM+) driving capability activation bits."]
pub struct I2C_PB6_FMP_R(crate::FieldReader<bool, I2C_PB6_FMP_A>);
impl I2C_PB6_FMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C_PB6_FMP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C_PB6_FMP_A {
        match self.bits {
            false => I2C_PB6_FMP_A::STANDARD,
            true => I2C_PB6_FMP_A::FMP,
        }
    }
    #[doc = "Checks if the value of the field is `STANDARD`"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        **self == I2C_PB6_FMP_A::STANDARD
    }
    #[doc = "Checks if the value of the field is `FMP`"]
    #[inline(always)]
    pub fn is_fmp(&self) -> bool {
        **self == I2C_PB6_FMP_A::FMP
    }
}
impl core::ops::Deref for I2C_PB6_FMP_R {
    type Target = crate::FieldReader<bool, I2C_PB6_FMP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C_PB6_FMP` writer - Fast Mode Plus (FM+) driving capability activation bits."]
pub struct I2C_PB6_FMP_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_PB6_FMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C_PB6_FMP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PB6 pin operate in standard mode"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(I2C_PB6_FMP_A::STANDARD)
    }
    #[doc = "I2C FM+ mode enabled on PB6 and the Speed control is bypassed"]
    #[inline(always)]
    pub fn fmp(self) -> &'a mut W {
        self.variant(I2C_PB6_FMP_A::FMP)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Fast Mode Plus (FM+) driving capability activation bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C_PB7_FMP_A {
    #[doc = "0: PB7 pin operate in standard mode"]
    STANDARD = 0,
    #[doc = "1: I2C FM+ mode enabled on PB7 and the Speed control is bypassed"]
    FMP = 1,
}
impl From<I2C_PB7_FMP_A> for bool {
    #[inline(always)]
    fn from(variant: I2C_PB7_FMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C_PB7_FMP` reader - Fast Mode Plus (FM+) driving capability activation bits."]
pub struct I2C_PB7_FMP_R(crate::FieldReader<bool, I2C_PB7_FMP_A>);
impl I2C_PB7_FMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C_PB7_FMP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C_PB7_FMP_A {
        match self.bits {
            false => I2C_PB7_FMP_A::STANDARD,
            true => I2C_PB7_FMP_A::FMP,
        }
    }
    #[doc = "Checks if the value of the field is `STANDARD`"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        **self == I2C_PB7_FMP_A::STANDARD
    }
    #[doc = "Checks if the value of the field is `FMP`"]
    #[inline(always)]
    pub fn is_fmp(&self) -> bool {
        **self == I2C_PB7_FMP_A::FMP
    }
}
impl core::ops::Deref for I2C_PB7_FMP_R {
    type Target = crate::FieldReader<bool, I2C_PB7_FMP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C_PB7_FMP` writer - Fast Mode Plus (FM+) driving capability activation bits."]
pub struct I2C_PB7_FMP_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_PB7_FMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C_PB7_FMP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PB7 pin operate in standard mode"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(I2C_PB7_FMP_A::STANDARD)
    }
    #[doc = "I2C FM+ mode enabled on PB7 and the Speed control is bypassed"]
    #[inline(always)]
    pub fn fmp(self) -> &'a mut W {
        self.variant(I2C_PB7_FMP_A::FMP)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Fast Mode Plus (FM+) driving capability activation bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C_PB8_FMP_A {
    #[doc = "0: PB8 pin operate in standard mode"]
    STANDARD = 0,
    #[doc = "1: I2C FM+ mode enabled on PB8 and the Speed control is bypassed"]
    FMP = 1,
}
impl From<I2C_PB8_FMP_A> for bool {
    #[inline(always)]
    fn from(variant: I2C_PB8_FMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C_PB8_FMP` reader - Fast Mode Plus (FM+) driving capability activation bits."]
pub struct I2C_PB8_FMP_R(crate::FieldReader<bool, I2C_PB8_FMP_A>);
impl I2C_PB8_FMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C_PB8_FMP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C_PB8_FMP_A {
        match self.bits {
            false => I2C_PB8_FMP_A::STANDARD,
            true => I2C_PB8_FMP_A::FMP,
        }
    }
    #[doc = "Checks if the value of the field is `STANDARD`"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        **self == I2C_PB8_FMP_A::STANDARD
    }
    #[doc = "Checks if the value of the field is `FMP`"]
    #[inline(always)]
    pub fn is_fmp(&self) -> bool {
        **self == I2C_PB8_FMP_A::FMP
    }
}
impl core::ops::Deref for I2C_PB8_FMP_R {
    type Target = crate::FieldReader<bool, I2C_PB8_FMP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C_PB8_FMP` writer - Fast Mode Plus (FM+) driving capability activation bits."]
pub struct I2C_PB8_FMP_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_PB8_FMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C_PB8_FMP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PB8 pin operate in standard mode"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(I2C_PB8_FMP_A::STANDARD)
    }
    #[doc = "I2C FM+ mode enabled on PB8 and the Speed control is bypassed"]
    #[inline(always)]
    pub fn fmp(self) -> &'a mut W {
        self.variant(I2C_PB8_FMP_A::FMP)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Fast Mode Plus (FM+) driving capability activation bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C_PB9_FMP_A {
    #[doc = "0: PB9 pin operate in standard mode"]
    STANDARD = 0,
    #[doc = "1: I2C FM+ mode enabled on PB9 and the Speed control is bypassed"]
    FMP = 1,
}
impl From<I2C_PB9_FMP_A> for bool {
    #[inline(always)]
    fn from(variant: I2C_PB9_FMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C_PB9_FMP` reader - Fast Mode Plus (FM+) driving capability activation bits."]
pub struct I2C_PB9_FMP_R(crate::FieldReader<bool, I2C_PB9_FMP_A>);
impl I2C_PB9_FMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C_PB9_FMP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C_PB9_FMP_A {
        match self.bits {
            false => I2C_PB9_FMP_A::STANDARD,
            true => I2C_PB9_FMP_A::FMP,
        }
    }
    #[doc = "Checks if the value of the field is `STANDARD`"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        **self == I2C_PB9_FMP_A::STANDARD
    }
    #[doc = "Checks if the value of the field is `FMP`"]
    #[inline(always)]
    pub fn is_fmp(&self) -> bool {
        **self == I2C_PB9_FMP_A::FMP
    }
}
impl core::ops::Deref for I2C_PB9_FMP_R {
    type Target = crate::FieldReader<bool, I2C_PB9_FMP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C_PB9_FMP` writer - Fast Mode Plus (FM+) driving capability activation bits."]
pub struct I2C_PB9_FMP_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_PB9_FMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C_PB9_FMP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PB9 pin operate in standard mode"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(I2C_PB9_FMP_A::STANDARD)
    }
    #[doc = "I2C FM+ mode enabled on PB9 and the Speed control is bypassed"]
    #[inline(always)]
    pub fn fmp(self) -> &'a mut W {
        self.variant(I2C_PB9_FMP_A::FMP)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "I2C1 Fast Mode Plus\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C1_FMP_A {
    #[doc = "0: FM+ mode is controlled by I2C_Pxx_FMP bits only"]
    STANDARD = 0,
    #[doc = "1: FM+ mode is enabled on all I2C1 pins selected through selection through IOPORT control registers AF selection bits"]
    FMP = 1,
}
impl From<I2C1_FMP_A> for bool {
    #[inline(always)]
    fn from(variant: I2C1_FMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C1_FMP` reader - I2C1 Fast Mode Plus"]
pub struct I2C1_FMP_R(crate::FieldReader<bool, I2C1_FMP_A>);
impl I2C1_FMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C1_FMP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C1_FMP_A {
        match self.bits {
            false => I2C1_FMP_A::STANDARD,
            true => I2C1_FMP_A::FMP,
        }
    }
    #[doc = "Checks if the value of the field is `STANDARD`"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        **self == I2C1_FMP_A::STANDARD
    }
    #[doc = "Checks if the value of the field is `FMP`"]
    #[inline(always)]
    pub fn is_fmp(&self) -> bool {
        **self == I2C1_FMP_A::FMP
    }
}
impl core::ops::Deref for I2C1_FMP_R {
    type Target = crate::FieldReader<bool, I2C1_FMP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C1_FMP` writer - I2C1 Fast Mode Plus"]
pub struct I2C1_FMP_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1_FMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C1_FMP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FM+ mode is controlled by I2C_Pxx_FMP bits only"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(I2C1_FMP_A::STANDARD)
    }
    #[doc = "FM+ mode is enabled on all I2C1 pins selected through selection through IOPORT control registers AF selection bits"]
    #[inline(always)]
    pub fn fmp(self) -> &'a mut W {
        self.variant(I2C1_FMP_A::FMP)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "I2C2 Fast Mode Plus\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C2_FMP_A {
    #[doc = "0: FM+ mode is controlled by I2C_Pxx_FMP bits only"]
    STANDARD = 0,
    #[doc = "1: FM+ mode is enabled on all I2C2 pins selected through selection through IOPORT control registers AF selection bits"]
    FMP = 1,
}
impl From<I2C2_FMP_A> for bool {
    #[inline(always)]
    fn from(variant: I2C2_FMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C2_FMP` reader - I2C2 Fast Mode Plus"]
pub struct I2C2_FMP_R(crate::FieldReader<bool, I2C2_FMP_A>);
impl I2C2_FMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C2_FMP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C2_FMP_A {
        match self.bits {
            false => I2C2_FMP_A::STANDARD,
            true => I2C2_FMP_A::FMP,
        }
    }
    #[doc = "Checks if the value of the field is `STANDARD`"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        **self == I2C2_FMP_A::STANDARD
    }
    #[doc = "Checks if the value of the field is `FMP`"]
    #[inline(always)]
    pub fn is_fmp(&self) -> bool {
        **self == I2C2_FMP_A::FMP
    }
}
impl core::ops::Deref for I2C2_FMP_R {
    type Target = crate::FieldReader<bool, I2C2_FMP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C2_FMP` writer - I2C2 Fast Mode Plus"]
pub struct I2C2_FMP_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C2_FMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C2_FMP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FM+ mode is controlled by I2C_Pxx_FMP bits only"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(I2C2_FMP_A::STANDARD)
    }
    #[doc = "FM+ mode is enabled on all I2C2 pins selected through selection through IOPORT control registers AF selection bits"]
    #[inline(always)]
    pub fn fmp(self) -> &'a mut W {
        self.variant(I2C2_FMP_A::FMP)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Encoder mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ENCODER_MODE_A {
    #[doc = "0: No redirection"]
    NOREDIRECTION = 0,
    #[doc = "1: TIM2 IC1 and TIM2 IC2 are connected to TIM15 IC1 and TIM15 IC2 respectively"]
    MAPTIM2TIM15 = 1,
    #[doc = "2: TIM3 IC1 and TIM3 IC2 are connected to TIM15 IC1 and TIM15 IC2 respectively"]
    MAPTIM3TIM15 = 2,
    #[doc = "3: TIM4 IC1 and TIM4 IC2 are connected to TIM15 IC1 and TIM15 IC2 respectively (STM32F303xB/C and STM32F358xC devices only)"]
    MAPTIM4TIM15 = 3,
}
impl From<ENCODER_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: ENCODER_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ENCODER_MODE` reader - Encoder mode"]
pub struct ENCODER_MODE_R(crate::FieldReader<u8, ENCODER_MODE_A>);
impl ENCODER_MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        ENCODER_MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENCODER_MODE_A {
        match self.bits {
            0 => ENCODER_MODE_A::NOREDIRECTION,
            1 => ENCODER_MODE_A::MAPTIM2TIM15,
            2 => ENCODER_MODE_A::MAPTIM3TIM15,
            3 => ENCODER_MODE_A::MAPTIM4TIM15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NOREDIRECTION`"]
    #[inline(always)]
    pub fn is_no_redirection(&self) -> bool {
        **self == ENCODER_MODE_A::NOREDIRECTION
    }
    #[doc = "Checks if the value of the field is `MAPTIM2TIM15`"]
    #[inline(always)]
    pub fn is_map_tim2tim15(&self) -> bool {
        **self == ENCODER_MODE_A::MAPTIM2TIM15
    }
    #[doc = "Checks if the value of the field is `MAPTIM3TIM15`"]
    #[inline(always)]
    pub fn is_map_tim3tim15(&self) -> bool {
        **self == ENCODER_MODE_A::MAPTIM3TIM15
    }
    #[doc = "Checks if the value of the field is `MAPTIM4TIM15`"]
    #[inline(always)]
    pub fn is_map_tim4tim15(&self) -> bool {
        **self == ENCODER_MODE_A::MAPTIM4TIM15
    }
}
impl core::ops::Deref for ENCODER_MODE_R {
    type Target = crate::FieldReader<u8, ENCODER_MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENCODER_MODE` writer - Encoder mode"]
pub struct ENCODER_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENCODER_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENCODER_MODE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "No redirection"]
    #[inline(always)]
    pub fn no_redirection(self) -> &'a mut W {
        self.variant(ENCODER_MODE_A::NOREDIRECTION)
    }
    #[doc = "TIM2 IC1 and TIM2 IC2 are connected to TIM15 IC1 and TIM15 IC2 respectively"]
    #[inline(always)]
    pub fn map_tim2tim15(self) -> &'a mut W {
        self.variant(ENCODER_MODE_A::MAPTIM2TIM15)
    }
    #[doc = "TIM3 IC1 and TIM3 IC2 are connected to TIM15 IC1 and TIM15 IC2 respectively"]
    #[inline(always)]
    pub fn map_tim3tim15(self) -> &'a mut W {
        self.variant(ENCODER_MODE_A::MAPTIM3TIM15)
    }
    #[doc = "TIM4 IC1 and TIM4 IC2 are connected to TIM15 IC1 and TIM15 IC2 respectively (STM32F303xB/C and STM32F358xC devices only)"]
    #[inline(always)]
    pub fn map_tim4tim15(self) -> &'a mut W {
        self.variant(ENCODER_MODE_A::MAPTIM4TIM15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
        self.w
    }
}
#[doc = "Inexact interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FPU_IE5_A {
    #[doc = "0: Inexact interrupt disable"]
    DISABLED = 0,
    #[doc = "1: Inexact interrupt enable"]
    ENABLED = 1,
}
impl From<FPU_IE5_A> for bool {
    #[inline(always)]
    fn from(variant: FPU_IE5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPU_IE5` reader - Inexact interrupt enable"]
pub struct FPU_IE5_R(crate::FieldReader<bool, FPU_IE5_A>);
impl FPU_IE5_R {
    pub(crate) fn new(bits: bool) -> Self {
        FPU_IE5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FPU_IE5_A {
        match self.bits {
            false => FPU_IE5_A::DISABLED,
            true => FPU_IE5_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == FPU_IE5_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == FPU_IE5_A::ENABLED
    }
}
impl core::ops::Deref for FPU_IE5_R {
    type Target = crate::FieldReader<bool, FPU_IE5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPU_IE5` writer - Inexact interrupt enable"]
pub struct FPU_IE5_W<'a> {
    w: &'a mut W,
}
impl<'a> FPU_IE5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FPU_IE5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Inexact interrupt disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FPU_IE5_A::DISABLED)
    }
    #[doc = "Inexact interrupt enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FPU_IE5_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
#[doc = "Input denormal interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FPU_IE4_A {
    #[doc = "0: Input denormal interrupt disable"]
    DISABLED = 0,
    #[doc = "1: Input denormal interrupt enable"]
    ENABLED = 1,
}
impl From<FPU_IE4_A> for bool {
    #[inline(always)]
    fn from(variant: FPU_IE4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPU_IE4` reader - Input denormal interrupt enable"]
pub struct FPU_IE4_R(crate::FieldReader<bool, FPU_IE4_A>);
impl FPU_IE4_R {
    pub(crate) fn new(bits: bool) -> Self {
        FPU_IE4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FPU_IE4_A {
        match self.bits {
            false => FPU_IE4_A::DISABLED,
            true => FPU_IE4_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == FPU_IE4_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == FPU_IE4_A::ENABLED
    }
}
impl core::ops::Deref for FPU_IE4_R {
    type Target = crate::FieldReader<bool, FPU_IE4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPU_IE4` writer - Input denormal interrupt enable"]
pub struct FPU_IE4_W<'a> {
    w: &'a mut W,
}
impl<'a> FPU_IE4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FPU_IE4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Input denormal interrupt disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FPU_IE4_A::DISABLED)
    }
    #[doc = "Input denormal interrupt enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FPU_IE4_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Overflow interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FPU_IE3_A {
    #[doc = "0: Overflow interrupt disable"]
    DISABLED = 0,
    #[doc = "1: Overflow interrupt enable"]
    ENABLED = 1,
}
impl From<FPU_IE3_A> for bool {
    #[inline(always)]
    fn from(variant: FPU_IE3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPU_IE3` reader - Overflow interrupt enable"]
pub struct FPU_IE3_R(crate::FieldReader<bool, FPU_IE3_A>);
impl FPU_IE3_R {
    pub(crate) fn new(bits: bool) -> Self {
        FPU_IE3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FPU_IE3_A {
        match self.bits {
            false => FPU_IE3_A::DISABLED,
            true => FPU_IE3_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == FPU_IE3_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == FPU_IE3_A::ENABLED
    }
}
impl core::ops::Deref for FPU_IE3_R {
    type Target = crate::FieldReader<bool, FPU_IE3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPU_IE3` writer - Overflow interrupt enable"]
pub struct FPU_IE3_W<'a> {
    w: &'a mut W,
}
impl<'a> FPU_IE3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FPU_IE3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Overflow interrupt disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FPU_IE3_A::DISABLED)
    }
    #[doc = "Overflow interrupt enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FPU_IE3_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Underflow interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FPU_IE2_A {
    #[doc = "0: Underflow interrupt disable"]
    DISABLED = 0,
    #[doc = "1: Underflow interrupt enable"]
    ENABLED = 1,
}
impl From<FPU_IE2_A> for bool {
    #[inline(always)]
    fn from(variant: FPU_IE2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPU_IE2` reader - Underflow interrupt enable"]
pub struct FPU_IE2_R(crate::FieldReader<bool, FPU_IE2_A>);
impl FPU_IE2_R {
    pub(crate) fn new(bits: bool) -> Self {
        FPU_IE2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FPU_IE2_A {
        match self.bits {
            false => FPU_IE2_A::DISABLED,
            true => FPU_IE2_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == FPU_IE2_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == FPU_IE2_A::ENABLED
    }
}
impl core::ops::Deref for FPU_IE2_R {
    type Target = crate::FieldReader<bool, FPU_IE2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPU_IE2` writer - Underflow interrupt enable"]
pub struct FPU_IE2_W<'a> {
    w: &'a mut W,
}
impl<'a> FPU_IE2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FPU_IE2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Underflow interrupt disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FPU_IE2_A::DISABLED)
    }
    #[doc = "Underflow interrupt enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FPU_IE2_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Devide-by-zero interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FPU_IE1_A {
    #[doc = "0: Devide-by-zero interrupt disable"]
    DISABLED = 0,
    #[doc = "1: Devide-by-zero interrupt enable"]
    ENABLED = 1,
}
impl From<FPU_IE1_A> for bool {
    #[inline(always)]
    fn from(variant: FPU_IE1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPU_IE1` reader - Devide-by-zero interrupt enable"]
pub struct FPU_IE1_R(crate::FieldReader<bool, FPU_IE1_A>);
impl FPU_IE1_R {
    pub(crate) fn new(bits: bool) -> Self {
        FPU_IE1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FPU_IE1_A {
        match self.bits {
            false => FPU_IE1_A::DISABLED,
            true => FPU_IE1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == FPU_IE1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == FPU_IE1_A::ENABLED
    }
}
impl core::ops::Deref for FPU_IE1_R {
    type Target = crate::FieldReader<bool, FPU_IE1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPU_IE1` writer - Devide-by-zero interrupt enable"]
pub struct FPU_IE1_W<'a> {
    w: &'a mut W,
}
impl<'a> FPU_IE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FPU_IE1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Devide-by-zero interrupt disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FPU_IE1_A::DISABLED)
    }
    #[doc = "Devide-by-zero interrupt enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FPU_IE1_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Invalid operation interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FPU_IE0_A {
    #[doc = "0: Invalid operation interrupt disable"]
    DISABLED = 0,
    #[doc = "1: Invalid operation interrupt enable"]
    ENABLED = 1,
}
impl From<FPU_IE0_A> for bool {
    #[inline(always)]
    fn from(variant: FPU_IE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPU_IE0` reader - Invalid operation interrupt enable"]
pub struct FPU_IE0_R(crate::FieldReader<bool, FPU_IE0_A>);
impl FPU_IE0_R {
    pub(crate) fn new(bits: bool) -> Self {
        FPU_IE0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FPU_IE0_A {
        match self.bits {
            false => FPU_IE0_A::DISABLED,
            true => FPU_IE0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == FPU_IE0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == FPU_IE0_A::ENABLED
    }
}
impl core::ops::Deref for FPU_IE0_R {
    type Target = crate::FieldReader<bool, FPU_IE0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPU_IE0` writer - Invalid operation interrupt enable"]
pub struct FPU_IE0_W<'a> {
    w: &'a mut W,
}
impl<'a> FPU_IE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FPU_IE0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Invalid operation interrupt disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FPU_IE0_A::DISABLED)
    }
    #[doc = "Invalid operation interrupt enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FPU_IE0_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "DAC2 channel1 DMA remap\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DAC2_CH1_DMA_RMP_A {
    #[doc = "0: Not remapped"]
    NOTREMAPPED = 0,
    #[doc = "1: DAC2_CH1 DMA requests mapped on DMA1 channel 5"]
    REMAPPED = 1,
}
impl From<DAC2_CH1_DMA_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: DAC2_CH1_DMA_RMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DAC2_CH1_DMA_RMP` reader - DAC2 channel1 DMA remap"]
pub struct DAC2_CH1_DMA_RMP_R(crate::FieldReader<bool, DAC2_CH1_DMA_RMP_A>);
impl DAC2_CH1_DMA_RMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DAC2_CH1_DMA_RMP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAC2_CH1_DMA_RMP_A {
        match self.bits {
            false => DAC2_CH1_DMA_RMP_A::NOTREMAPPED,
            true => DAC2_CH1_DMA_RMP_A::REMAPPED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTREMAPPED`"]
    #[inline(always)]
    pub fn is_not_remapped(&self) -> bool {
        **self == DAC2_CH1_DMA_RMP_A::NOTREMAPPED
    }
    #[doc = "Checks if the value of the field is `REMAPPED`"]
    #[inline(always)]
    pub fn is_remapped(&self) -> bool {
        **self == DAC2_CH1_DMA_RMP_A::REMAPPED
    }
}
impl core::ops::Deref for DAC2_CH1_DMA_RMP_R {
    type Target = crate::FieldReader<bool, DAC2_CH1_DMA_RMP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAC2_CH1_DMA_RMP` writer - DAC2 channel1 DMA remap"]
pub struct DAC2_CH1_DMA_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC2_CH1_DMA_RMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DAC2_CH1_DMA_RMP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Not remapped"]
    #[inline(always)]
    pub fn not_remapped(self) -> &'a mut W {
        self.variant(DAC2_CH1_DMA_RMP_A::NOTREMAPPED)
    }
    #[doc = "DAC2_CH1 DMA requests mapped on DMA1 channel 5"]
    #[inline(always)]
    pub fn remapped(self) -> &'a mut W {
        self.variant(DAC2_CH1_DMA_RMP_A::REMAPPED)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "I2C3 Fast Mode Plus\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C3_FMP_A {
    #[doc = "0: FM+ mode is controlled by I2C_Pxx_FMP bits only"]
    STANDARD = 0,
    #[doc = "1: FM+ mode is enabled on all I2C3 pins selected through selection through IOPORT control registers AF selection bits"]
    FMP = 1,
}
impl From<I2C3_FMP_A> for bool {
    #[inline(always)]
    fn from(variant: I2C3_FMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C3_FMP` reader - I2C3 Fast Mode Plus"]
pub struct I2C3_FMP_R(crate::FieldReader<bool, I2C3_FMP_A>);
impl I2C3_FMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C3_FMP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C3_FMP_A {
        match self.bits {
            false => I2C3_FMP_A::STANDARD,
            true => I2C3_FMP_A::FMP,
        }
    }
    #[doc = "Checks if the value of the field is `STANDARD`"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        **self == I2C3_FMP_A::STANDARD
    }
    #[doc = "Checks if the value of the field is `FMP`"]
    #[inline(always)]
    pub fn is_fmp(&self) -> bool {
        **self == I2C3_FMP_A::FMP
    }
}
impl core::ops::Deref for I2C3_FMP_R {
    type Target = crate::FieldReader<bool, I2C3_FMP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C3_FMP` writer - I2C3 Fast Mode Plus"]
pub struct I2C3_FMP_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C3_FMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C3_FMP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FM+ mode is controlled by I2C_Pxx_FMP bits only"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(I2C3_FMP_A::STANDARD)
    }
    #[doc = "FM+ mode is enabled on all I2C3 pins selected through selection through IOPORT control registers AF selection bits"]
    #[inline(always)]
    pub fn fmp(self) -> &'a mut W {
        self.variant(I2C3_FMP_A::FMP)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Memory mapping selection bits"]
    #[inline(always)]
    pub fn mem_mode(&self) -> MEM_MODE_R {
        MEM_MODE_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 5 - USB interrupt remap"]
    #[inline(always)]
    pub fn usb_it_rmp(&self) -> USB_IT_RMP_R {
        USB_IT_RMP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Timer 1 ITR3 selection"]
    #[inline(always)]
    pub fn tim1_itr3_rmp(&self) -> TIM1_ITR3_RMP_R {
        TIM1_ITR3_RMP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - DAC trigger remap (when TSEL = 001)"]
    #[inline(always)]
    pub fn dac1_trig_rmp(&self) -> DAC1_TRIG_RMP_R {
        DAC1_TRIG_RMP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - ADC24 DMA remapping bit"]
    #[inline(always)]
    pub fn adc2_dma_rmp(&self) -> ADC2_DMA_RMP_R {
        ADC2_DMA_RMP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 11 - TIM16 DMA request remapping bit"]
    #[inline(always)]
    pub fn tim16_dma_rmp(&self) -> TIM16_DMA_RMP_R {
        TIM16_DMA_RMP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - TIM17 DMA request remapping bit"]
    #[inline(always)]
    pub fn tim17_dma_rmp(&self) -> TIM17_DMA_RMP_R {
        TIM17_DMA_RMP_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - TIM6 and DAC1 DMA request remapping bit"]
    #[inline(always)]
    pub fn tim6_dac1_ch1_dma_rmp(&self) -> TIM6_DAC1_CH1_DMA_RMP_R {
        TIM6_DAC1_CH1_DMA_RMP_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - TIM7 and DAC2 DMA request remapping bit"]
    #[inline(always)]
    pub fn tim7_dac1_ch2_dma_rmp(&self) -> TIM7_DAC1_CH2_DMA_RMP_R {
        TIM7_DAC1_CH2_DMA_RMP_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Fast Mode Plus (FM+) driving capability activation bits."]
    #[inline(always)]
    pub fn i2c_pb6_fmp(&self) -> I2C_PB6_FMP_R {
        I2C_PB6_FMP_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Fast Mode Plus (FM+) driving capability activation bits."]
    #[inline(always)]
    pub fn i2c_pb7_fmp(&self) -> I2C_PB7_FMP_R {
        I2C_PB7_FMP_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Fast Mode Plus (FM+) driving capability activation bits."]
    #[inline(always)]
    pub fn i2c_pb8_fmp(&self) -> I2C_PB8_FMP_R {
        I2C_PB8_FMP_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Fast Mode Plus (FM+) driving capability activation bits."]
    #[inline(always)]
    pub fn i2c_pb9_fmp(&self) -> I2C_PB9_FMP_R {
        I2C_PB9_FMP_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - I2C1 Fast Mode Plus"]
    #[inline(always)]
    pub fn i2c1_fmp(&self) -> I2C1_FMP_R {
        I2C1_FMP_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - I2C2 Fast Mode Plus"]
    #[inline(always)]
    pub fn i2c2_fmp(&self) -> I2C2_FMP_R {
        I2C2_FMP_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bits 22:23 - Encoder mode"]
    #[inline(always)]
    pub fn encoder_mode(&self) -> ENCODER_MODE_R {
        ENCODER_MODE_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bit 31 - Inexact interrupt enable"]
    #[inline(always)]
    pub fn fpu_ie5(&self) -> FPU_IE5_R {
        FPU_IE5_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Input denormal interrupt enable"]
    #[inline(always)]
    pub fn fpu_ie4(&self) -> FPU_IE4_R {
        FPU_IE4_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Overflow interrupt enable"]
    #[inline(always)]
    pub fn fpu_ie3(&self) -> FPU_IE3_R {
        FPU_IE3_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Underflow interrupt enable"]
    #[inline(always)]
    pub fn fpu_ie2(&self) -> FPU_IE2_R {
        FPU_IE2_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Devide-by-zero interrupt enable"]
    #[inline(always)]
    pub fn fpu_ie1(&self) -> FPU_IE1_R {
        FPU_IE1_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Invalid operation interrupt enable"]
    #[inline(always)]
    pub fn fpu_ie0(&self) -> FPU_IE0_R {
        FPU_IE0_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 15 - DAC2 channel1 DMA remap"]
    #[inline(always)]
    pub fn dac2_ch1_dma_rmp(&self) -> DAC2_CH1_DMA_RMP_R {
        DAC2_CH1_DMA_RMP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 24 - I2C3 Fast Mode Plus"]
    #[inline(always)]
    pub fn i2c3_fmp(&self) -> I2C3_FMP_R {
        I2C3_FMP_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Memory mapping selection bits"]
    #[inline(always)]
    pub fn mem_mode(&mut self) -> MEM_MODE_W {
        MEM_MODE_W { w: self }
    }
    #[doc = "Bit 5 - USB interrupt remap"]
    #[inline(always)]
    pub fn usb_it_rmp(&mut self) -> USB_IT_RMP_W {
        USB_IT_RMP_W { w: self }
    }
    #[doc = "Bit 6 - Timer 1 ITR3 selection"]
    #[inline(always)]
    pub fn tim1_itr3_rmp(&mut self) -> TIM1_ITR3_RMP_W {
        TIM1_ITR3_RMP_W { w: self }
    }
    #[doc = "Bit 7 - DAC trigger remap (when TSEL = 001)"]
    #[inline(always)]
    pub fn dac1_trig_rmp(&mut self) -> DAC1_TRIG_RMP_W {
        DAC1_TRIG_RMP_W { w: self }
    }
    #[doc = "Bit 8 - ADC24 DMA remapping bit"]
    #[inline(always)]
    pub fn adc2_dma_rmp(&mut self) -> ADC2_DMA_RMP_W {
        ADC2_DMA_RMP_W { w: self }
    }
    #[doc = "Bit 11 - TIM16 DMA request remapping bit"]
    #[inline(always)]
    pub fn tim16_dma_rmp(&mut self) -> TIM16_DMA_RMP_W {
        TIM16_DMA_RMP_W { w: self }
    }
    #[doc = "Bit 12 - TIM17 DMA request remapping bit"]
    #[inline(always)]
    pub fn tim17_dma_rmp(&mut self) -> TIM17_DMA_RMP_W {
        TIM17_DMA_RMP_W { w: self }
    }
    #[doc = "Bit 13 - TIM6 and DAC1 DMA request remapping bit"]
    #[inline(always)]
    pub fn tim6_dac1_ch1_dma_rmp(&mut self) -> TIM6_DAC1_CH1_DMA_RMP_W {
        TIM6_DAC1_CH1_DMA_RMP_W { w: self }
    }
    #[doc = "Bit 14 - TIM7 and DAC2 DMA request remapping bit"]
    #[inline(always)]
    pub fn tim7_dac1_ch2_dma_rmp(&mut self) -> TIM7_DAC1_CH2_DMA_RMP_W {
        TIM7_DAC1_CH2_DMA_RMP_W { w: self }
    }
    #[doc = "Bit 16 - Fast Mode Plus (FM+) driving capability activation bits."]
    #[inline(always)]
    pub fn i2c_pb6_fmp(&mut self) -> I2C_PB6_FMP_W {
        I2C_PB6_FMP_W { w: self }
    }
    #[doc = "Bit 17 - Fast Mode Plus (FM+) driving capability activation bits."]
    #[inline(always)]
    pub fn i2c_pb7_fmp(&mut self) -> I2C_PB7_FMP_W {
        I2C_PB7_FMP_W { w: self }
    }
    #[doc = "Bit 18 - Fast Mode Plus (FM+) driving capability activation bits."]
    #[inline(always)]
    pub fn i2c_pb8_fmp(&mut self) -> I2C_PB8_FMP_W {
        I2C_PB8_FMP_W { w: self }
    }
    #[doc = "Bit 19 - Fast Mode Plus (FM+) driving capability activation bits."]
    #[inline(always)]
    pub fn i2c_pb9_fmp(&mut self) -> I2C_PB9_FMP_W {
        I2C_PB9_FMP_W { w: self }
    }
    #[doc = "Bit 20 - I2C1 Fast Mode Plus"]
    #[inline(always)]
    pub fn i2c1_fmp(&mut self) -> I2C1_FMP_W {
        I2C1_FMP_W { w: self }
    }
    #[doc = "Bit 21 - I2C2 Fast Mode Plus"]
    #[inline(always)]
    pub fn i2c2_fmp(&mut self) -> I2C2_FMP_W {
        I2C2_FMP_W { w: self }
    }
    #[doc = "Bits 22:23 - Encoder mode"]
    #[inline(always)]
    pub fn encoder_mode(&mut self) -> ENCODER_MODE_W {
        ENCODER_MODE_W { w: self }
    }
    #[doc = "Bit 31 - Inexact interrupt enable"]
    #[inline(always)]
    pub fn fpu_ie5(&mut self) -> FPU_IE5_W {
        FPU_IE5_W { w: self }
    }
    #[doc = "Bit 30 - Input denormal interrupt enable"]
    #[inline(always)]
    pub fn fpu_ie4(&mut self) -> FPU_IE4_W {
        FPU_IE4_W { w: self }
    }
    #[doc = "Bit 29 - Overflow interrupt enable"]
    #[inline(always)]
    pub fn fpu_ie3(&mut self) -> FPU_IE3_W {
        FPU_IE3_W { w: self }
    }
    #[doc = "Bit 28 - Underflow interrupt enable"]
    #[inline(always)]
    pub fn fpu_ie2(&mut self) -> FPU_IE2_W {
        FPU_IE2_W { w: self }
    }
    #[doc = "Bit 27 - Devide-by-zero interrupt enable"]
    #[inline(always)]
    pub fn fpu_ie1(&mut self) -> FPU_IE1_W {
        FPU_IE1_W { w: self }
    }
    #[doc = "Bit 26 - Invalid operation interrupt enable"]
    #[inline(always)]
    pub fn fpu_ie0(&mut self) -> FPU_IE0_W {
        FPU_IE0_W { w: self }
    }
    #[doc = "Bit 15 - DAC2 channel1 DMA remap"]
    #[inline(always)]
    pub fn dac2_ch1_dma_rmp(&mut self) -> DAC2_CH1_DMA_RMP_W {
        DAC2_CH1_DMA_RMP_W { w: self }
    }
    #[doc = "Bit 24 - I2C3 Fast Mode Plus"]
    #[inline(always)]
    pub fn i2c3_fmp(&mut self) -> I2C3_FMP_W {
        I2C3_FMP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr1](index.html) module"]
pub struct CFGR1_SPEC;
impl crate::RegisterSpec for CFGR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgr1::R](R) reader structure"]
impl crate::Readable for CFGR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgr1::W](W) writer structure"]
impl crate::Writable for CFGR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFGR1 to value 0"]
impl crate::Resettable for CFGR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
