#[doc = "Reader of register CFGR1"]
pub type R = crate::R<u32, super::CFGR1>;
#[doc = "Writer for register CFGR1"]
pub type W = crate::W<u32, super::CFGR1>;
#[doc = "Register CFGR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CFGR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
    #[doc = "2: Main Flash memory mapped at 0x0000_0000"]
    MAINFLASH2 = 2,
    #[doc = "3: Embedded SRAM mapped at 0x0000_0000"]
    SRAM = 3,
}
impl From<MEM_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MEM_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MEM_MODE`"]
pub type MEM_MODE_R = crate::R<u8, MEM_MODE_A>;
impl MEM_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MEM_MODE_A {
        match self.bits {
            0 => MEM_MODE_A::MAINFLASH,
            1 => MEM_MODE_A::SYSTEMFLASH,
            2 => MEM_MODE_A::MAINFLASH2,
            3 => MEM_MODE_A::SRAM,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MAINFLASH`"]
    #[inline(always)]
    pub fn is_main_flash(&self) -> bool {
        *self == MEM_MODE_A::MAINFLASH
    }
    #[doc = "Checks if the value of the field is `SYSTEMFLASH`"]
    #[inline(always)]
    pub fn is_system_flash(&self) -> bool {
        *self == MEM_MODE_A::SYSTEMFLASH
    }
    #[doc = "Checks if the value of the field is `MAINFLASH2`"]
    #[inline(always)]
    pub fn is_main_flash2(&self) -> bool {
        *self == MEM_MODE_A::MAINFLASH2
    }
    #[doc = "Checks if the value of the field is `SRAM`"]
    #[inline(always)]
    pub fn is_sram(&self) -> bool {
        *self == MEM_MODE_A::SRAM
    }
}
#[doc = "Write proxy for field `MEM_MODE`"]
pub struct MEM_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MEM_MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
    #[doc = "Main Flash memory mapped at 0x0000_0000"]
    #[inline(always)]
    pub fn main_flash2(self) -> &'a mut W {
        self.variant(MEM_MODE_A::MAINFLASH2)
    }
    #[doc = "Embedded SRAM mapped at 0x0000_0000"]
    #[inline(always)]
    pub fn sram(self) -> &'a mut W {
        self.variant(MEM_MODE_A::SRAM)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "ADC DMA remapping bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_DMA_RMP_A {
    #[doc = "0: ADC DMA request mapped on DMA channel 1"]
    NOTREMAPPED = 0,
    #[doc = "1: ADC DMA request mapped on DMA channel 2"]
    REMAPPED = 1,
}
impl From<ADC_DMA_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_DMA_RMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC_DMA_RMP`"]
pub type ADC_DMA_RMP_R = crate::R<bool, ADC_DMA_RMP_A>;
impl ADC_DMA_RMP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_DMA_RMP_A {
        match self.bits {
            false => ADC_DMA_RMP_A::NOTREMAPPED,
            true => ADC_DMA_RMP_A::REMAPPED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTREMAPPED`"]
    #[inline(always)]
    pub fn is_not_remapped(&self) -> bool {
        *self == ADC_DMA_RMP_A::NOTREMAPPED
    }
    #[doc = "Checks if the value of the field is `REMAPPED`"]
    #[inline(always)]
    pub fn is_remapped(&self) -> bool {
        *self == ADC_DMA_RMP_A::REMAPPED
    }
}
#[doc = "Write proxy for field `ADC_DMA_RMP`"]
pub struct ADC_DMA_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_DMA_RMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_DMA_RMP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ADC DMA request mapped on DMA channel 1"]
    #[inline(always)]
    pub fn not_remapped(self) -> &'a mut W {
        self.variant(ADC_DMA_RMP_A::NOTREMAPPED)
    }
    #[doc = "ADC DMA request mapped on DMA channel 2"]
    #[inline(always)]
    pub fn remapped(self) -> &'a mut W {
        self.variant(ADC_DMA_RMP_A::REMAPPED)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "USART1_TX DMA remapping bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USART1_TX_DMA_RMP_A {
    #[doc = "0: USART1_TX DMA request mapped on DMA channel 2"]
    NOTREMAPPED = 0,
    #[doc = "1: USART1_TX DMA request mapped on DMA channel 4"]
    REMAPPED = 1,
}
impl From<USART1_TX_DMA_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: USART1_TX_DMA_RMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `USART1_TX_DMA_RMP`"]
pub type USART1_TX_DMA_RMP_R = crate::R<bool, USART1_TX_DMA_RMP_A>;
impl USART1_TX_DMA_RMP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USART1_TX_DMA_RMP_A {
        match self.bits {
            false => USART1_TX_DMA_RMP_A::NOTREMAPPED,
            true => USART1_TX_DMA_RMP_A::REMAPPED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTREMAPPED`"]
    #[inline(always)]
    pub fn is_not_remapped(&self) -> bool {
        *self == USART1_TX_DMA_RMP_A::NOTREMAPPED
    }
    #[doc = "Checks if the value of the field is `REMAPPED`"]
    #[inline(always)]
    pub fn is_remapped(&self) -> bool {
        *self == USART1_TX_DMA_RMP_A::REMAPPED
    }
}
#[doc = "Write proxy for field `USART1_TX_DMA_RMP`"]
pub struct USART1_TX_DMA_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> USART1_TX_DMA_RMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USART1_TX_DMA_RMP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "USART1_TX DMA request mapped on DMA channel 2"]
    #[inline(always)]
    pub fn not_remapped(self) -> &'a mut W {
        self.variant(USART1_TX_DMA_RMP_A::NOTREMAPPED)
    }
    #[doc = "USART1_TX DMA request mapped on DMA channel 4"]
    #[inline(always)]
    pub fn remapped(self) -> &'a mut W {
        self.variant(USART1_TX_DMA_RMP_A::REMAPPED)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "USART1_RX DMA request remapping bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USART1_RX_DMA_RMP_A {
    #[doc = "0: USART1_RX DMA request mapped on DMA channel 3"]
    NOTREMAPPED = 0,
    #[doc = "1: USART1_RX DMA request mapped on DMA channel 5"]
    REMAPPED = 1,
}
impl From<USART1_RX_DMA_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: USART1_RX_DMA_RMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `USART1_RX_DMA_RMP`"]
pub type USART1_RX_DMA_RMP_R = crate::R<bool, USART1_RX_DMA_RMP_A>;
impl USART1_RX_DMA_RMP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USART1_RX_DMA_RMP_A {
        match self.bits {
            false => USART1_RX_DMA_RMP_A::NOTREMAPPED,
            true => USART1_RX_DMA_RMP_A::REMAPPED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTREMAPPED`"]
    #[inline(always)]
    pub fn is_not_remapped(&self) -> bool {
        *self == USART1_RX_DMA_RMP_A::NOTREMAPPED
    }
    #[doc = "Checks if the value of the field is `REMAPPED`"]
    #[inline(always)]
    pub fn is_remapped(&self) -> bool {
        *self == USART1_RX_DMA_RMP_A::REMAPPED
    }
}
#[doc = "Write proxy for field `USART1_RX_DMA_RMP`"]
pub struct USART1_RX_DMA_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> USART1_RX_DMA_RMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USART1_RX_DMA_RMP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "USART1_RX DMA request mapped on DMA channel 3"]
    #[inline(always)]
    pub fn not_remapped(self) -> &'a mut W {
        self.variant(USART1_RX_DMA_RMP_A::NOTREMAPPED)
    }
    #[doc = "USART1_RX DMA request mapped on DMA channel 5"]
    #[inline(always)]
    pub fn remapped(self) -> &'a mut W {
        self.variant(USART1_RX_DMA_RMP_A::REMAPPED)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "TIM16 DMA request remapping bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIM16_DMA_RMP_A {
    #[doc = "0: TIM16_CH1 and TIM16_UP DMA request mapped on DMA channel 3"]
    NOTREMAPPED = 0,
    #[doc = "1: TIM16_CH1 and TIM16_UP DMA request mapped on DMA channel 4"]
    REMAPPED = 1,
}
impl From<TIM16_DMA_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: TIM16_DMA_RMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TIM16_DMA_RMP`"]
pub type TIM16_DMA_RMP_R = crate::R<bool, TIM16_DMA_RMP_A>;
impl TIM16_DMA_RMP_R {
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
        *self == TIM16_DMA_RMP_A::NOTREMAPPED
    }
    #[doc = "Checks if the value of the field is `REMAPPED`"]
    #[inline(always)]
    pub fn is_remapped(&self) -> bool {
        *self == TIM16_DMA_RMP_A::REMAPPED
    }
}
#[doc = "Write proxy for field `TIM16_DMA_RMP`"]
pub struct TIM16_DMA_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM16_DMA_RMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM16_DMA_RMP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "TIM16_CH1 and TIM16_UP DMA request mapped on DMA channel 3"]
    #[inline(always)]
    pub fn not_remapped(self) -> &'a mut W {
        self.variant(TIM16_DMA_RMP_A::NOTREMAPPED)
    }
    #[doc = "TIM16_CH1 and TIM16_UP DMA request mapped on DMA channel 4"]
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "TIM17 DMA request remapping bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIM17_DMA_RMP_A {
    #[doc = "0: TIM17_CH1 and TIM17_UP DMA request mapped on DMA channel 1"]
    NOTREMAPPED = 0,
    #[doc = "1: TIM17_CH1 and TIM17_UP DMA request mapped on DMA channel 2"]
    REMAPPED = 1,
}
impl From<TIM17_DMA_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: TIM17_DMA_RMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TIM17_DMA_RMP`"]
pub type TIM17_DMA_RMP_R = crate::R<bool, TIM17_DMA_RMP_A>;
impl TIM17_DMA_RMP_R {
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
        *self == TIM17_DMA_RMP_A::NOTREMAPPED
    }
    #[doc = "Checks if the value of the field is `REMAPPED`"]
    #[inline(always)]
    pub fn is_remapped(&self) -> bool {
        *self == TIM17_DMA_RMP_A::REMAPPED
    }
}
#[doc = "Write proxy for field `TIM17_DMA_RMP`"]
pub struct TIM17_DMA_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM17_DMA_RMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM17_DMA_RMP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "TIM17_CH1 and TIM17_UP DMA request mapped on DMA channel 1"]
    #[inline(always)]
    pub fn not_remapped(self) -> &'a mut W {
        self.variant(TIM17_DMA_RMP_A::NOTREMAPPED)
    }
    #[doc = "TIM17_CH1 and TIM17_UP DMA request mapped on DMA channel 2"]
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Fast Mode Plus (FM plus) driving capability activation bits.\n\nValue on reset: 0"]
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
#[doc = "Reader of field `I2C_PB6_FMP`"]
pub type I2C_PB6_FMP_R = crate::R<bool, I2C_PB6_FMP_A>;
impl I2C_PB6_FMP_R {
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
        *self == I2C_PB6_FMP_A::STANDARD
    }
    #[doc = "Checks if the value of the field is `FMP`"]
    #[inline(always)]
    pub fn is_fmp(&self) -> bool {
        *self == I2C_PB6_FMP_A::FMP
    }
}
#[doc = "Write proxy for field `I2C_PB6_FMP`"]
pub struct I2C_PB6_FMP_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_PB6_FMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C_PB6_FMP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
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
#[doc = "Reader of field `I2C_PB7_FMP`"]
pub type I2C_PB7_FMP_R = crate::R<bool, I2C_PB7_FMP_A>;
impl I2C_PB7_FMP_R {
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
        *self == I2C_PB7_FMP_A::STANDARD
    }
    #[doc = "Checks if the value of the field is `FMP`"]
    #[inline(always)]
    pub fn is_fmp(&self) -> bool {
        *self == I2C_PB7_FMP_A::FMP
    }
}
#[doc = "Write proxy for field `I2C_PB7_FMP`"]
pub struct I2C_PB7_FMP_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_PB7_FMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C_PB7_FMP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
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
#[doc = "Reader of field `I2C_PB8_FMP`"]
pub type I2C_PB8_FMP_R = crate::R<bool, I2C_PB8_FMP_A>;
impl I2C_PB8_FMP_R {
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
        *self == I2C_PB8_FMP_A::STANDARD
    }
    #[doc = "Checks if the value of the field is `FMP`"]
    #[inline(always)]
    pub fn is_fmp(&self) -> bool {
        *self == I2C_PB8_FMP_A::FMP
    }
}
#[doc = "Write proxy for field `I2C_PB8_FMP`"]
pub struct I2C_PB8_FMP_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_PB8_FMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C_PB8_FMP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
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
#[doc = "Reader of field `I2C_PB9_FMP`"]
pub type I2C_PB9_FMP_R = crate::R<bool, I2C_PB9_FMP_A>;
impl I2C_PB9_FMP_R {
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
        *self == I2C_PB9_FMP_A::STANDARD
    }
    #[doc = "Checks if the value of the field is `FMP`"]
    #[inline(always)]
    pub fn is_fmp(&self) -> bool {
        *self == I2C_PB9_FMP_A::FMP
    }
}
#[doc = "Write proxy for field `I2C_PB9_FMP`"]
pub struct I2C_PB9_FMP_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_PB9_FMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C_PB9_FMP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "FM+ driving capability activation for I2C1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C1_FMP_A {
    #[doc = "0: FM+ mode is controlled by I2C_Pxx_FMP bits only"]
    STANDARD = 0,
    #[doc = "1: FM+ mode is enabled on all I2C1 pins selected through selection bits in GPIOx_AFR registers"]
    FMP = 1,
}
impl From<I2C1_FMP_A> for bool {
    #[inline(always)]
    fn from(variant: I2C1_FMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `I2C1_FMP`"]
pub type I2C1_FMP_R = crate::R<bool, I2C1_FMP_A>;
impl I2C1_FMP_R {
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
        *self == I2C1_FMP_A::STANDARD
    }
    #[doc = "Checks if the value of the field is `FMP`"]
    #[inline(always)]
    pub fn is_fmp(&self) -> bool {
        *self == I2C1_FMP_A::FMP
    }
}
#[doc = "Write proxy for field `I2C1_FMP`"]
pub struct I2C1_FMP_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1_FMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C1_FMP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FM+ mode is controlled by I2C_Pxx_FMP bits only"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(I2C1_FMP_A::STANDARD)
    }
    #[doc = "FM+ mode is enabled on all I2C1 pins selected through selection bits in GPIOx_AFR registers"]
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "FM+ driving capability activation for I2C2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C2_FMP_A {
    #[doc = "0: FM+ mode is controlled by I2C_Pxx_FMP bits only"]
    STANDARD = 0,
    #[doc = "1: FM+ mode is enabled on all I2C2 pins selected through selection bits in GPIOx_AFR registers"]
    FMP = 1,
}
impl From<I2C2_FMP_A> for bool {
    #[inline(always)]
    fn from(variant: I2C2_FMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `I2C2_FMP`"]
pub type I2C2_FMP_R = crate::R<bool, I2C2_FMP_A>;
impl I2C2_FMP_R {
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
        *self == I2C2_FMP_A::STANDARD
    }
    #[doc = "Checks if the value of the field is `FMP`"]
    #[inline(always)]
    pub fn is_fmp(&self) -> bool {
        *self == I2C2_FMP_A::FMP
    }
}
#[doc = "Write proxy for field `I2C2_FMP`"]
pub struct I2C2_FMP_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C2_FMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C2_FMP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FM+ mode is controlled by I2C_Pxx_FMP bits only"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(I2C2_FMP_A::STANDARD)
    }
    #[doc = "FM+ mode is enabled on all I2C2 pins selected through selection bits in GPIOx_AFR registers"]
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "SPI2 DMA request remapping bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI2_DMA_RMP_A {
    #[doc = "0: SPI2_RX and SPI2_TX DMA requests mapped on DMA channel 4 and 5 respectively"]
    NOTREMAPPED = 0,
    #[doc = "1: SPI2_RX and SPI2_TX DMA requests mapped on DMA channel 6 and 7 respectively"]
    REMAPPED = 1,
}
impl From<SPI2_DMA_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: SPI2_DMA_RMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPI2_DMA_RMP`"]
pub type SPI2_DMA_RMP_R = crate::R<bool, SPI2_DMA_RMP_A>;
impl SPI2_DMA_RMP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI2_DMA_RMP_A {
        match self.bits {
            false => SPI2_DMA_RMP_A::NOTREMAPPED,
            true => SPI2_DMA_RMP_A::REMAPPED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTREMAPPED`"]
    #[inline(always)]
    pub fn is_not_remapped(&self) -> bool {
        *self == SPI2_DMA_RMP_A::NOTREMAPPED
    }
    #[doc = "Checks if the value of the field is `REMAPPED`"]
    #[inline(always)]
    pub fn is_remapped(&self) -> bool {
        *self == SPI2_DMA_RMP_A::REMAPPED
    }
}
#[doc = "Write proxy for field `SPI2_DMA_RMP`"]
pub struct SPI2_DMA_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI2_DMA_RMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI2_DMA_RMP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SPI2_RX and SPI2_TX DMA requests mapped on DMA channel 4 and 5 respectively"]
    #[inline(always)]
    pub fn not_remapped(self) -> &'a mut W {
        self.variant(SPI2_DMA_RMP_A::NOTREMAPPED)
    }
    #[doc = "SPI2_RX and SPI2_TX DMA requests mapped on DMA channel 6 and 7 respectively"]
    #[inline(always)]
    pub fn remapped(self) -> &'a mut W {
        self.variant(SPI2_DMA_RMP_A::REMAPPED)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "USART2 DMA request remapping bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USART2_DMA_RMP_A {
    #[doc = "0: USART2_RX and USART2_TX DMA requests mapped on DMA channel 5 and 4 respectively"]
    NOTREMAPPED = 0,
    #[doc = "1: USART2_RX and USART2_TX DMA requests mapped on DMA channel 6 and 7 respectively"]
    REMAPPED = 1,
}
impl From<USART2_DMA_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: USART2_DMA_RMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `USART2_DMA_RMP`"]
pub type USART2_DMA_RMP_R = crate::R<bool, USART2_DMA_RMP_A>;
impl USART2_DMA_RMP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USART2_DMA_RMP_A {
        match self.bits {
            false => USART2_DMA_RMP_A::NOTREMAPPED,
            true => USART2_DMA_RMP_A::REMAPPED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTREMAPPED`"]
    #[inline(always)]
    pub fn is_not_remapped(&self) -> bool {
        *self == USART2_DMA_RMP_A::NOTREMAPPED
    }
    #[doc = "Checks if the value of the field is `REMAPPED`"]
    #[inline(always)]
    pub fn is_remapped(&self) -> bool {
        *self == USART2_DMA_RMP_A::REMAPPED
    }
}
#[doc = "Write proxy for field `USART2_DMA_RMP`"]
pub struct USART2_DMA_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> USART2_DMA_RMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USART2_DMA_RMP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "USART2_RX and USART2_TX DMA requests mapped on DMA channel 5 and 4 respectively"]
    #[inline(always)]
    pub fn not_remapped(self) -> &'a mut W {
        self.variant(USART2_DMA_RMP_A::NOTREMAPPED)
    }
    #[doc = "USART2_RX and USART2_TX DMA requests mapped on DMA channel 6 and 7 respectively"]
    #[inline(always)]
    pub fn remapped(self) -> &'a mut W {
        self.variant(USART2_DMA_RMP_A::REMAPPED)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "USART3 DMA request remapping bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USART3_DMA_RMP_A {
    #[doc = "0: USART3_RX and USART3_TX DMA requests mapped on DMA channel 6 and 7 respectively (or simply disabled on STM32F0x0)"]
    NOTREMAPPED = 0,
    #[doc = "1: USART3_RX and USART3_TX DMA requests mapped on DMA channel 3 and 2 respectively"]
    REMAPPED = 1,
}
impl From<USART3_DMA_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: USART3_DMA_RMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `USART3_DMA_RMP`"]
pub type USART3_DMA_RMP_R = crate::R<bool, USART3_DMA_RMP_A>;
impl USART3_DMA_RMP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USART3_DMA_RMP_A {
        match self.bits {
            false => USART3_DMA_RMP_A::NOTREMAPPED,
            true => USART3_DMA_RMP_A::REMAPPED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTREMAPPED`"]
    #[inline(always)]
    pub fn is_not_remapped(&self) -> bool {
        *self == USART3_DMA_RMP_A::NOTREMAPPED
    }
    #[doc = "Checks if the value of the field is `REMAPPED`"]
    #[inline(always)]
    pub fn is_remapped(&self) -> bool {
        *self == USART3_DMA_RMP_A::REMAPPED
    }
}
#[doc = "Write proxy for field `USART3_DMA_RMP`"]
pub struct USART3_DMA_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> USART3_DMA_RMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USART3_DMA_RMP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "USART3_RX and USART3_TX DMA requests mapped on DMA channel 6 and 7 respectively (or simply disabled on STM32F0x0)"]
    #[inline(always)]
    pub fn not_remapped(self) -> &'a mut W {
        self.variant(USART3_DMA_RMP_A::NOTREMAPPED)
    }
    #[doc = "USART3_RX and USART3_TX DMA requests mapped on DMA channel 3 and 2 respectively"]
    #[inline(always)]
    pub fn remapped(self) -> &'a mut W {
        self.variant(USART3_DMA_RMP_A::REMAPPED)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "I2C1 DMA request remapping bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C1_DMA_RMP_A {
    #[doc = "0: I2C1_RX and I2C1_TX DMA requests mapped on DMA channel 3 and 2 respectively"]
    NOTREMAPPED = 0,
    #[doc = "1: I2C1_RX and I2C1_TX DMA requests mapped on DMA channel 7 and 6 respectively"]
    REMAPPED = 1,
}
impl From<I2C1_DMA_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: I2C1_DMA_RMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `I2C1_DMA_RMP`"]
pub type I2C1_DMA_RMP_R = crate::R<bool, I2C1_DMA_RMP_A>;
impl I2C1_DMA_RMP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C1_DMA_RMP_A {
        match self.bits {
            false => I2C1_DMA_RMP_A::NOTREMAPPED,
            true => I2C1_DMA_RMP_A::REMAPPED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTREMAPPED`"]
    #[inline(always)]
    pub fn is_not_remapped(&self) -> bool {
        *self == I2C1_DMA_RMP_A::NOTREMAPPED
    }
    #[doc = "Checks if the value of the field is `REMAPPED`"]
    #[inline(always)]
    pub fn is_remapped(&self) -> bool {
        *self == I2C1_DMA_RMP_A::REMAPPED
    }
}
#[doc = "Write proxy for field `I2C1_DMA_RMP`"]
pub struct I2C1_DMA_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1_DMA_RMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C1_DMA_RMP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "I2C1_RX and I2C1_TX DMA requests mapped on DMA channel 3 and 2 respectively"]
    #[inline(always)]
    pub fn not_remapped(self) -> &'a mut W {
        self.variant(I2C1_DMA_RMP_A::NOTREMAPPED)
    }
    #[doc = "I2C1_RX and I2C1_TX DMA requests mapped on DMA channel 7 and 6 respectively"]
    #[inline(always)]
    pub fn remapped(self) -> &'a mut W {
        self.variant(I2C1_DMA_RMP_A::REMAPPED)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "TIM1 DMA request remapping bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIM1_DMA_RMP_A {
    #[doc = "0: TIM1_CH1, TIM1_CH2 and TIM1_CH3 DMA requests mapped on DMA channel 2, 3 and 4 respectively"]
    NOTREMAPPED = 0,
    #[doc = "1: TIM1_CH1, TIM1_CH2 and TIM1_CH3 DMA requests mapped on DMA channel 6"]
    REMAPPED = 1,
}
impl From<TIM1_DMA_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: TIM1_DMA_RMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TIM1_DMA_RMP`"]
pub type TIM1_DMA_RMP_R = crate::R<bool, TIM1_DMA_RMP_A>;
impl TIM1_DMA_RMP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM1_DMA_RMP_A {
        match self.bits {
            false => TIM1_DMA_RMP_A::NOTREMAPPED,
            true => TIM1_DMA_RMP_A::REMAPPED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTREMAPPED`"]
    #[inline(always)]
    pub fn is_not_remapped(&self) -> bool {
        *self == TIM1_DMA_RMP_A::NOTREMAPPED
    }
    #[doc = "Checks if the value of the field is `REMAPPED`"]
    #[inline(always)]
    pub fn is_remapped(&self) -> bool {
        *self == TIM1_DMA_RMP_A::REMAPPED
    }
}
#[doc = "Write proxy for field `TIM1_DMA_RMP`"]
pub struct TIM1_DMA_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM1_DMA_RMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM1_DMA_RMP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "TIM1_CH1, TIM1_CH2 and TIM1_CH3 DMA requests mapped on DMA channel 2, 3 and 4 respectively"]
    #[inline(always)]
    pub fn not_remapped(self) -> &'a mut W {
        self.variant(TIM1_DMA_RMP_A::NOTREMAPPED)
    }
    #[doc = "TIM1_CH1, TIM1_CH2 and TIM1_CH3 DMA requests mapped on DMA channel 6"]
    #[inline(always)]
    pub fn remapped(self) -> &'a mut W {
        self.variant(TIM1_DMA_RMP_A::REMAPPED)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "TIM2 DMA request remapping bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIM2_DMA_RMP_A {
    #[doc = "0: TIM2_CH2 and TIM2_CH4 DMA requests mapped on DMA channel 3 and 4 respectively"]
    NOTREMAPPED = 0,
    #[doc = "1: TIM2_CH2 and TIM2_CH4 DMA requests mapped on DMA channel 7"]
    REMAPPED = 1,
}
impl From<TIM2_DMA_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: TIM2_DMA_RMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TIM2_DMA_RMP`"]
pub type TIM2_DMA_RMP_R = crate::R<bool, TIM2_DMA_RMP_A>;
impl TIM2_DMA_RMP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM2_DMA_RMP_A {
        match self.bits {
            false => TIM2_DMA_RMP_A::NOTREMAPPED,
            true => TIM2_DMA_RMP_A::REMAPPED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTREMAPPED`"]
    #[inline(always)]
    pub fn is_not_remapped(&self) -> bool {
        *self == TIM2_DMA_RMP_A::NOTREMAPPED
    }
    #[doc = "Checks if the value of the field is `REMAPPED`"]
    #[inline(always)]
    pub fn is_remapped(&self) -> bool {
        *self == TIM2_DMA_RMP_A::REMAPPED
    }
}
#[doc = "Write proxy for field `TIM2_DMA_RMP`"]
pub struct TIM2_DMA_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM2_DMA_RMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM2_DMA_RMP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "TIM2_CH2 and TIM2_CH4 DMA requests mapped on DMA channel 3 and 4 respectively"]
    #[inline(always)]
    pub fn not_remapped(self) -> &'a mut W {
        self.variant(TIM2_DMA_RMP_A::NOTREMAPPED)
    }
    #[doc = "TIM2_CH2 and TIM2_CH4 DMA requests mapped on DMA channel 7"]
    #[inline(always)]
    pub fn remapped(self) -> &'a mut W {
        self.variant(TIM2_DMA_RMP_A::REMAPPED)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "TIM3 DMA request remapping bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIM3_DMA_RMP_A {
    #[doc = "0: TIM3_CH1 and TIM3_TRIG DMA requests mapped on DMA channel 4"]
    NOTREMAPPED = 0,
    #[doc = "1: TIM3_CH1 and TIM3_TRIG DMA requests mapped on DMA channel 6"]
    REMAPPED = 1,
}
impl From<TIM3_DMA_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: TIM3_DMA_RMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TIM3_DMA_RMP`"]
pub type TIM3_DMA_RMP_R = crate::R<bool, TIM3_DMA_RMP_A>;
impl TIM3_DMA_RMP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM3_DMA_RMP_A {
        match self.bits {
            false => TIM3_DMA_RMP_A::NOTREMAPPED,
            true => TIM3_DMA_RMP_A::REMAPPED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTREMAPPED`"]
    #[inline(always)]
    pub fn is_not_remapped(&self) -> bool {
        *self == TIM3_DMA_RMP_A::NOTREMAPPED
    }
    #[doc = "Checks if the value of the field is `REMAPPED`"]
    #[inline(always)]
    pub fn is_remapped(&self) -> bool {
        *self == TIM3_DMA_RMP_A::REMAPPED
    }
}
#[doc = "Write proxy for field `TIM3_DMA_RMP`"]
pub struct TIM3_DMA_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM3_DMA_RMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM3_DMA_RMP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "TIM3_CH1 and TIM3_TRIG DMA requests mapped on DMA channel 4"]
    #[inline(always)]
    pub fn not_remapped(self) -> &'a mut W {
        self.variant(TIM3_DMA_RMP_A::NOTREMAPPED)
    }
    #[doc = "TIM3_CH1 and TIM3_TRIG DMA requests mapped on DMA channel 6"]
    #[inline(always)]
    pub fn remapped(self) -> &'a mut W {
        self.variant(TIM3_DMA_RMP_A::REMAPPED)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "IR Modulation Envelope signal selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IR_MOD_A {
    #[doc = "0: TIM16 selected"]
    TIM16 = 0,
    #[doc = "1: USART1 selected"]
    USART1 = 1,
    #[doc = "2: USART4 selected"]
    USART4 = 2,
}
impl From<IR_MOD_A> for u8 {
    #[inline(always)]
    fn from(variant: IR_MOD_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `IR_MOD`"]
pub type IR_MOD_R = crate::R<u8, IR_MOD_A>;
impl IR_MOD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, IR_MOD_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(IR_MOD_A::TIM16),
            1 => Val(IR_MOD_A::USART1),
            2 => Val(IR_MOD_A::USART4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TIM16`"]
    #[inline(always)]
    pub fn is_tim16(&self) -> bool {
        *self == IR_MOD_A::TIM16
    }
    #[doc = "Checks if the value of the field is `USART1`"]
    #[inline(always)]
    pub fn is_usart1(&self) -> bool {
        *self == IR_MOD_A::USART1
    }
    #[doc = "Checks if the value of the field is `USART4`"]
    #[inline(always)]
    pub fn is_usart4(&self) -> bool {
        *self == IR_MOD_A::USART4
    }
}
#[doc = "Write proxy for field `IR_MOD`"]
pub struct IR_MOD_W<'a> {
    w: &'a mut W,
}
impl<'a> IR_MOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IR_MOD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "TIM16 selected"]
    #[inline(always)]
    pub fn tim16(self) -> &'a mut W {
        self.variant(IR_MOD_A::TIM16)
    }
    #[doc = "USART1 selected"]
    #[inline(always)]
    pub fn usart1(self) -> &'a mut W {
        self.variant(IR_MOD_A::USART1)
    }
    #[doc = "USART4 selected"]
    #[inline(always)]
    pub fn usart4(self) -> &'a mut W {
        self.variant(IR_MOD_A::USART4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "TIM16 alternate DMA request remapping bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIM16_DMA_RMP2_A {
    #[doc = "0: TIM16 DMA request mapped according to TIM16_DMA_RMP bit"]
    NOTALTERNATEREMAPPED = 0,
    #[doc = "1: TIM16_CH1 and TIM16_UP DMA request mapped on DMA channel 6"]
    ALTERNATEREMAPPED = 1,
}
impl From<TIM16_DMA_RMP2_A> for bool {
    #[inline(always)]
    fn from(variant: TIM16_DMA_RMP2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TIM16_DMA_RMP2`"]
pub type TIM16_DMA_RMP2_R = crate::R<bool, TIM16_DMA_RMP2_A>;
impl TIM16_DMA_RMP2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM16_DMA_RMP2_A {
        match self.bits {
            false => TIM16_DMA_RMP2_A::NOTALTERNATEREMAPPED,
            true => TIM16_DMA_RMP2_A::ALTERNATEREMAPPED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTALTERNATEREMAPPED`"]
    #[inline(always)]
    pub fn is_not_alternate_remapped(&self) -> bool {
        *self == TIM16_DMA_RMP2_A::NOTALTERNATEREMAPPED
    }
    #[doc = "Checks if the value of the field is `ALTERNATEREMAPPED`"]
    #[inline(always)]
    pub fn is_alternate_remapped(&self) -> bool {
        *self == TIM16_DMA_RMP2_A::ALTERNATEREMAPPED
    }
}
#[doc = "Write proxy for field `TIM16_DMA_RMP2`"]
pub struct TIM16_DMA_RMP2_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM16_DMA_RMP2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM16_DMA_RMP2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "TIM16 DMA request mapped according to TIM16_DMA_RMP bit"]
    #[inline(always)]
    pub fn not_alternate_remapped(self) -> &'a mut W {
        self.variant(TIM16_DMA_RMP2_A::NOTALTERNATEREMAPPED)
    }
    #[doc = "TIM16_CH1 and TIM16_UP DMA request mapped on DMA channel 6"]
    #[inline(always)]
    pub fn alternate_remapped(self) -> &'a mut W {
        self.variant(TIM16_DMA_RMP2_A::ALTERNATEREMAPPED)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "TIM17 alternate DMA request remapping bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIM17_DMA_RMP2_A {
    #[doc = "0: TIM17 DMA request mapped according to TIM16_DMA_RMP bit"]
    NOTALTERNATEREMAPPED = 0,
    #[doc = "1: TIM17_CH1 and TIM17_UP DMA request mapped on DMA channel 7"]
    ALTERNATEREMAPPED = 1,
}
impl From<TIM17_DMA_RMP2_A> for bool {
    #[inline(always)]
    fn from(variant: TIM17_DMA_RMP2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TIM17_DMA_RMP2`"]
pub type TIM17_DMA_RMP2_R = crate::R<bool, TIM17_DMA_RMP2_A>;
impl TIM17_DMA_RMP2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM17_DMA_RMP2_A {
        match self.bits {
            false => TIM17_DMA_RMP2_A::NOTALTERNATEREMAPPED,
            true => TIM17_DMA_RMP2_A::ALTERNATEREMAPPED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTALTERNATEREMAPPED`"]
    #[inline(always)]
    pub fn is_not_alternate_remapped(&self) -> bool {
        *self == TIM17_DMA_RMP2_A::NOTALTERNATEREMAPPED
    }
    #[doc = "Checks if the value of the field is `ALTERNATEREMAPPED`"]
    #[inline(always)]
    pub fn is_alternate_remapped(&self) -> bool {
        *self == TIM17_DMA_RMP2_A::ALTERNATEREMAPPED
    }
}
#[doc = "Write proxy for field `TIM17_DMA_RMP2`"]
pub struct TIM17_DMA_RMP2_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM17_DMA_RMP2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM17_DMA_RMP2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "TIM17 DMA request mapped according to TIM16_DMA_RMP bit"]
    #[inline(always)]
    pub fn not_alternate_remapped(self) -> &'a mut W {
        self.variant(TIM17_DMA_RMP2_A::NOTALTERNATEREMAPPED)
    }
    #[doc = "TIM17_CH1 and TIM17_UP DMA request mapped on DMA channel 7"]
    #[inline(always)]
    pub fn alternate_remapped(self) -> &'a mut W {
        self.variant(TIM17_DMA_RMP2_A::ALTERNATEREMAPPED)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "PA11 and PA12 remapping bit for small packages (28 and 20 pins)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PA11_PA12_RMP_A {
    #[doc = "0: Pin pair PA9/PA10 mapped on the pins"]
    NOTREMAPPED = 0,
    #[doc = "1: Pin pair PA11/PA12 mapped instead of PA9/PA10"]
    REMAPPED = 1,
}
impl From<PA11_PA12_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: PA11_PA12_RMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PA11_PA12_RMP`"]
pub type PA11_PA12_RMP_R = crate::R<bool, PA11_PA12_RMP_A>;
impl PA11_PA12_RMP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PA11_PA12_RMP_A {
        match self.bits {
            false => PA11_PA12_RMP_A::NOTREMAPPED,
            true => PA11_PA12_RMP_A::REMAPPED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTREMAPPED`"]
    #[inline(always)]
    pub fn is_not_remapped(&self) -> bool {
        *self == PA11_PA12_RMP_A::NOTREMAPPED
    }
    #[doc = "Checks if the value of the field is `REMAPPED`"]
    #[inline(always)]
    pub fn is_remapped(&self) -> bool {
        *self == PA11_PA12_RMP_A::REMAPPED
    }
}
#[doc = "Write proxy for field `PA11_PA12_RMP`"]
pub struct PA11_PA12_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> PA11_PA12_RMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PA11_PA12_RMP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin pair PA9/PA10 mapped on the pins"]
    #[inline(always)]
    pub fn not_remapped(self) -> &'a mut W {
        self.variant(PA11_PA12_RMP_A::NOTREMAPPED)
    }
    #[doc = "Pin pair PA11/PA12 mapped instead of PA9/PA10"]
    #[inline(always)]
    pub fn remapped(self) -> &'a mut W {
        self.variant(PA11_PA12_RMP_A::REMAPPED)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Fast Mode Plus (FM+) driving capability activation bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C_PA9_FMP_A {
    #[doc = "0: PA9 pin operate in standard mode"]
    STANDARD = 0,
    #[doc = "1: I2C FM+ mode enabled on PA9 and the Speed control is bypassed"]
    FMP = 1,
}
impl From<I2C_PA9_FMP_A> for bool {
    #[inline(always)]
    fn from(variant: I2C_PA9_FMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `I2C_PA9_FMP`"]
pub type I2C_PA9_FMP_R = crate::R<bool, I2C_PA9_FMP_A>;
impl I2C_PA9_FMP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C_PA9_FMP_A {
        match self.bits {
            false => I2C_PA9_FMP_A::STANDARD,
            true => I2C_PA9_FMP_A::FMP,
        }
    }
    #[doc = "Checks if the value of the field is `STANDARD`"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == I2C_PA9_FMP_A::STANDARD
    }
    #[doc = "Checks if the value of the field is `FMP`"]
    #[inline(always)]
    pub fn is_fmp(&self) -> bool {
        *self == I2C_PA9_FMP_A::FMP
    }
}
#[doc = "Write proxy for field `I2C_PA9_FMP`"]
pub struct I2C_PA9_FMP_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_PA9_FMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C_PA9_FMP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PA9 pin operate in standard mode"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(I2C_PA9_FMP_A::STANDARD)
    }
    #[doc = "I2C FM+ mode enabled on PA9 and the Speed control is bypassed"]
    #[inline(always)]
    pub fn fmp(self) -> &'a mut W {
        self.variant(I2C_PA9_FMP_A::FMP)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Fast Mode Plus (FM+) driving capability activation bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C_PA10_FMP_A {
    #[doc = "0: PA10 pin operate in standard mode"]
    STANDARD = 0,
    #[doc = "1: I2C FM+ mode enabled on PA10 and the Speed control is bypassed"]
    FMP = 1,
}
impl From<I2C_PA10_FMP_A> for bool {
    #[inline(always)]
    fn from(variant: I2C_PA10_FMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `I2C_PA10_FMP`"]
pub type I2C_PA10_FMP_R = crate::R<bool, I2C_PA10_FMP_A>;
impl I2C_PA10_FMP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C_PA10_FMP_A {
        match self.bits {
            false => I2C_PA10_FMP_A::STANDARD,
            true => I2C_PA10_FMP_A::FMP,
        }
    }
    #[doc = "Checks if the value of the field is `STANDARD`"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == I2C_PA10_FMP_A::STANDARD
    }
    #[doc = "Checks if the value of the field is `FMP`"]
    #[inline(always)]
    pub fn is_fmp(&self) -> bool {
        *self == I2C_PA10_FMP_A::FMP
    }
}
#[doc = "Write proxy for field `I2C_PA10_FMP`"]
pub struct I2C_PA10_FMP_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_PA10_FMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C_PA10_FMP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PA10 pin operate in standard mode"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(I2C_PA10_FMP_A::STANDARD)
    }
    #[doc = "I2C FM+ mode enabled on PA10 and the Speed control is bypassed"]
    #[inline(always)]
    pub fn fmp(self) -> &'a mut W {
        self.variant(I2C_PA10_FMP_A::FMP)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Memory mapping selection bits"]
    #[inline(always)]
    pub fn mem_mode(&self) -> MEM_MODE_R {
        MEM_MODE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 8 - ADC DMA remapping bit"]
    #[inline(always)]
    pub fn adc_dma_rmp(&self) -> ADC_DMA_RMP_R {
        ADC_DMA_RMP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - USART1_TX DMA remapping bit"]
    #[inline(always)]
    pub fn usart1_tx_dma_rmp(&self) -> USART1_TX_DMA_RMP_R {
        USART1_TX_DMA_RMP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - USART1_RX DMA request remapping bit"]
    #[inline(always)]
    pub fn usart1_rx_dma_rmp(&self) -> USART1_RX_DMA_RMP_R {
        USART1_RX_DMA_RMP_R::new(((self.bits >> 10) & 0x01) != 0)
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
    #[doc = "Bit 16 - Fast Mode Plus (FM plus) driving capability activation bits."]
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
    #[doc = "Bit 20 - FM+ driving capability activation for I2C1"]
    #[inline(always)]
    pub fn i2c1_fmp(&self) -> I2C1_FMP_R {
        I2C1_FMP_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - FM+ driving capability activation for I2C2"]
    #[inline(always)]
    pub fn i2c2_fmp(&self) -> I2C2_FMP_R {
        I2C2_FMP_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 24 - SPI2 DMA request remapping bit"]
    #[inline(always)]
    pub fn spi2_dma_rmp(&self) -> SPI2_DMA_RMP_R {
        SPI2_DMA_RMP_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - USART2 DMA request remapping bit"]
    #[inline(always)]
    pub fn usart2_dma_rmp(&self) -> USART2_DMA_RMP_R {
        USART2_DMA_RMP_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - USART3 DMA request remapping bit"]
    #[inline(always)]
    pub fn usart3_dma_rmp(&self) -> USART3_DMA_RMP_R {
        USART3_DMA_RMP_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - I2C1 DMA request remapping bit"]
    #[inline(always)]
    pub fn i2c1_dma_rmp(&self) -> I2C1_DMA_RMP_R {
        I2C1_DMA_RMP_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - TIM1 DMA request remapping bit"]
    #[inline(always)]
    pub fn tim1_dma_rmp(&self) -> TIM1_DMA_RMP_R {
        TIM1_DMA_RMP_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - TIM2 DMA request remapping bit"]
    #[inline(always)]
    pub fn tim2_dma_rmp(&self) -> TIM2_DMA_RMP_R {
        TIM2_DMA_RMP_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - TIM3 DMA request remapping bit"]
    #[inline(always)]
    pub fn tim3_dma_rmp(&self) -> TIM3_DMA_RMP_R {
        TIM3_DMA_RMP_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - IR Modulation Envelope signal selection"]
    #[inline(always)]
    pub fn ir_mod(&self) -> IR_MOD_R {
        IR_MOD_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 13 - TIM16 alternate DMA request remapping bit"]
    #[inline(always)]
    pub fn tim16_dma_rmp2(&self) -> TIM16_DMA_RMP2_R {
        TIM16_DMA_RMP2_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - TIM17 alternate DMA request remapping bit"]
    #[inline(always)]
    pub fn tim17_dma_rmp2(&self) -> TIM17_DMA_RMP2_R {
        TIM17_DMA_RMP2_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PA11 and PA12 remapping bit for small packages (28 and 20 pins)"]
    #[inline(always)]
    pub fn pa11_pa12_rmp(&self) -> PA11_PA12_RMP_R {
        PA11_PA12_RMP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Fast Mode Plus (FM+) driving capability activation bits"]
    #[inline(always)]
    pub fn i2c_pa9_fmp(&self) -> I2C_PA9_FMP_R {
        I2C_PA9_FMP_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Fast Mode Plus (FM+) driving capability activation bits"]
    #[inline(always)]
    pub fn i2c_pa10_fmp(&self) -> I2C_PA10_FMP_R {
        I2C_PA10_FMP_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Memory mapping selection bits"]
    #[inline(always)]
    pub fn mem_mode(&mut self) -> MEM_MODE_W {
        MEM_MODE_W { w: self }
    }
    #[doc = "Bit 8 - ADC DMA remapping bit"]
    #[inline(always)]
    pub fn adc_dma_rmp(&mut self) -> ADC_DMA_RMP_W {
        ADC_DMA_RMP_W { w: self }
    }
    #[doc = "Bit 9 - USART1_TX DMA remapping bit"]
    #[inline(always)]
    pub fn usart1_tx_dma_rmp(&mut self) -> USART1_TX_DMA_RMP_W {
        USART1_TX_DMA_RMP_W { w: self }
    }
    #[doc = "Bit 10 - USART1_RX DMA request remapping bit"]
    #[inline(always)]
    pub fn usart1_rx_dma_rmp(&mut self) -> USART1_RX_DMA_RMP_W {
        USART1_RX_DMA_RMP_W { w: self }
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
    #[doc = "Bit 16 - Fast Mode Plus (FM plus) driving capability activation bits."]
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
    #[doc = "Bit 20 - FM+ driving capability activation for I2C1"]
    #[inline(always)]
    pub fn i2c1_fmp(&mut self) -> I2C1_FMP_W {
        I2C1_FMP_W { w: self }
    }
    #[doc = "Bit 21 - FM+ driving capability activation for I2C2"]
    #[inline(always)]
    pub fn i2c2_fmp(&mut self) -> I2C2_FMP_W {
        I2C2_FMP_W { w: self }
    }
    #[doc = "Bit 24 - SPI2 DMA request remapping bit"]
    #[inline(always)]
    pub fn spi2_dma_rmp(&mut self) -> SPI2_DMA_RMP_W {
        SPI2_DMA_RMP_W { w: self }
    }
    #[doc = "Bit 25 - USART2 DMA request remapping bit"]
    #[inline(always)]
    pub fn usart2_dma_rmp(&mut self) -> USART2_DMA_RMP_W {
        USART2_DMA_RMP_W { w: self }
    }
    #[doc = "Bit 26 - USART3 DMA request remapping bit"]
    #[inline(always)]
    pub fn usart3_dma_rmp(&mut self) -> USART3_DMA_RMP_W {
        USART3_DMA_RMP_W { w: self }
    }
    #[doc = "Bit 27 - I2C1 DMA request remapping bit"]
    #[inline(always)]
    pub fn i2c1_dma_rmp(&mut self) -> I2C1_DMA_RMP_W {
        I2C1_DMA_RMP_W { w: self }
    }
    #[doc = "Bit 28 - TIM1 DMA request remapping bit"]
    #[inline(always)]
    pub fn tim1_dma_rmp(&mut self) -> TIM1_DMA_RMP_W {
        TIM1_DMA_RMP_W { w: self }
    }
    #[doc = "Bit 29 - TIM2 DMA request remapping bit"]
    #[inline(always)]
    pub fn tim2_dma_rmp(&mut self) -> TIM2_DMA_RMP_W {
        TIM2_DMA_RMP_W { w: self }
    }
    #[doc = "Bit 30 - TIM3 DMA request remapping bit"]
    #[inline(always)]
    pub fn tim3_dma_rmp(&mut self) -> TIM3_DMA_RMP_W {
        TIM3_DMA_RMP_W { w: self }
    }
    #[doc = "Bits 6:7 - IR Modulation Envelope signal selection"]
    #[inline(always)]
    pub fn ir_mod(&mut self) -> IR_MOD_W {
        IR_MOD_W { w: self }
    }
    #[doc = "Bit 13 - TIM16 alternate DMA request remapping bit"]
    #[inline(always)]
    pub fn tim16_dma_rmp2(&mut self) -> TIM16_DMA_RMP2_W {
        TIM16_DMA_RMP2_W { w: self }
    }
    #[doc = "Bit 14 - TIM17 alternate DMA request remapping bit"]
    #[inline(always)]
    pub fn tim17_dma_rmp2(&mut self) -> TIM17_DMA_RMP2_W {
        TIM17_DMA_RMP2_W { w: self }
    }
    #[doc = "Bit 4 - PA11 and PA12 remapping bit for small packages (28 and 20 pins)"]
    #[inline(always)]
    pub fn pa11_pa12_rmp(&mut self) -> PA11_PA12_RMP_W {
        PA11_PA12_RMP_W { w: self }
    }
    #[doc = "Bit 22 - Fast Mode Plus (FM+) driving capability activation bits"]
    #[inline(always)]
    pub fn i2c_pa9_fmp(&mut self) -> I2C_PA9_FMP_W {
        I2C_PA9_FMP_W { w: self }
    }
    #[doc = "Bit 23 - Fast Mode Plus (FM+) driving capability activation bits"]
    #[inline(always)]
    pub fn i2c_pa10_fmp(&mut self) -> I2C_PA10_FMP_W {
        I2C_PA10_FMP_W { w: self }
    }
}
