#[doc = "Register `CFGR1` reader"]
pub type R = crate::R<CFGR1rs>;
#[doc = "Register `CFGR1` writer"]
pub type W = crate::W<CFGR1rs>;
#[doc = "Memory mapping selection bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MEM_MODE {
    #[doc = "0: Main Flash memory mapped at 0x0000_0000"]
    MainFlash = 0,
    #[doc = "1: System Flash memory mapped at 0x0000_0000"]
    SystemFlash = 1,
    #[doc = "2: Main Flash memory mapped at 0x0000_0000"]
    MainFlash2 = 2,
    #[doc = "3: Embedded SRAM mapped at 0x0000_0000"]
    Sram = 3,
}
impl From<MEM_MODE> for u8 {
    #[inline(always)]
    fn from(variant: MEM_MODE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MEM_MODE {
    type Ux = u8;
}
#[doc = "Field `MEM_MODE` reader - Memory mapping selection bits"]
pub type MEM_MODE_R = crate::FieldReader<MEM_MODE>;
impl MEM_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MEM_MODE {
        match self.bits {
            0 => MEM_MODE::MainFlash,
            1 => MEM_MODE::SystemFlash,
            2 => MEM_MODE::MainFlash2,
            3 => MEM_MODE::Sram,
            _ => unreachable!(),
        }
    }
    #[doc = "Main Flash memory mapped at 0x0000_0000"]
    #[inline(always)]
    pub fn is_main_flash(&self) -> bool {
        *self == MEM_MODE::MainFlash
    }
    #[doc = "System Flash memory mapped at 0x0000_0000"]
    #[inline(always)]
    pub fn is_system_flash(&self) -> bool {
        *self == MEM_MODE::SystemFlash
    }
    #[doc = "Main Flash memory mapped at 0x0000_0000"]
    #[inline(always)]
    pub fn is_main_flash2(&self) -> bool {
        *self == MEM_MODE::MainFlash2
    }
    #[doc = "Embedded SRAM mapped at 0x0000_0000"]
    #[inline(always)]
    pub fn is_sram(&self) -> bool {
        *self == MEM_MODE::Sram
    }
}
#[doc = "Field `MEM_MODE` writer - Memory mapping selection bits"]
pub type MEM_MODE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, MEM_MODE>;
impl<'a, REG> MEM_MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Main Flash memory mapped at 0x0000_0000"]
    #[inline(always)]
    pub fn main_flash(self) -> &'a mut crate::W<REG> {
        self.variant(MEM_MODE::MainFlash)
    }
    #[doc = "System Flash memory mapped at 0x0000_0000"]
    #[inline(always)]
    pub fn system_flash(self) -> &'a mut crate::W<REG> {
        self.variant(MEM_MODE::SystemFlash)
    }
    #[doc = "Main Flash memory mapped at 0x0000_0000"]
    #[inline(always)]
    pub fn main_flash2(self) -> &'a mut crate::W<REG> {
        self.variant(MEM_MODE::MainFlash2)
    }
    #[doc = "Embedded SRAM mapped at 0x0000_0000"]
    #[inline(always)]
    pub fn sram(self) -> &'a mut crate::W<REG> {
        self.variant(MEM_MODE::Sram)
    }
}
#[doc = "TIM16 DMA request remapping bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM16_DMA_RMP {
    #[doc = "0: TIM16_CH1 and TIM16_UP DMA requests mapped on DMA channel 3"]
    NotRemapped = 0,
    #[doc = "1: TIM16_CH1 and TIM16_UP DMA requests mapped on DMA channel 4"]
    Remapped = 1,
}
impl From<TIM16_DMA_RMP> for bool {
    #[inline(always)]
    fn from(variant: TIM16_DMA_RMP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM16_DMA_RMP` reader - TIM16 DMA request remapping bit"]
pub type TIM16_DMA_RMP_R = crate::BitReader<TIM16_DMA_RMP>;
impl TIM16_DMA_RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIM16_DMA_RMP {
        match self.bits {
            false => TIM16_DMA_RMP::NotRemapped,
            true => TIM16_DMA_RMP::Remapped,
        }
    }
    #[doc = "TIM16_CH1 and TIM16_UP DMA requests mapped on DMA channel 3"]
    #[inline(always)]
    pub fn is_not_remapped(&self) -> bool {
        *self == TIM16_DMA_RMP::NotRemapped
    }
    #[doc = "TIM16_CH1 and TIM16_UP DMA requests mapped on DMA channel 4"]
    #[inline(always)]
    pub fn is_remapped(&self) -> bool {
        *self == TIM16_DMA_RMP::Remapped
    }
}
#[doc = "Field `TIM16_DMA_RMP` writer - TIM16 DMA request remapping bit"]
pub type TIM16_DMA_RMP_W<'a, REG> = crate::BitWriter<'a, REG, TIM16_DMA_RMP>;
impl<'a, REG> TIM16_DMA_RMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TIM16_CH1 and TIM16_UP DMA requests mapped on DMA channel 3"]
    #[inline(always)]
    pub fn not_remapped(self) -> &'a mut crate::W<REG> {
        self.variant(TIM16_DMA_RMP::NotRemapped)
    }
    #[doc = "TIM16_CH1 and TIM16_UP DMA requests mapped on DMA channel 4"]
    #[inline(always)]
    pub fn remapped(self) -> &'a mut crate::W<REG> {
        self.variant(TIM16_DMA_RMP::Remapped)
    }
}
#[doc = "TIM17 DMA request remapping bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM17_DMA_RMP {
    #[doc = "0: TIM17_CH1 and TIM17_UP DMA requests mapped on DMA channel 1"]
    NotRemapped = 0,
    #[doc = "1: TIM17_CH1 and TIM17_UP DMA requests mapped on DMA channel 2"]
    Remapped = 1,
}
impl From<TIM17_DMA_RMP> for bool {
    #[inline(always)]
    fn from(variant: TIM17_DMA_RMP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM17_DMA_RMP` reader - TIM17 DMA request remapping bit"]
pub type TIM17_DMA_RMP_R = crate::BitReader<TIM17_DMA_RMP>;
impl TIM17_DMA_RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIM17_DMA_RMP {
        match self.bits {
            false => TIM17_DMA_RMP::NotRemapped,
            true => TIM17_DMA_RMP::Remapped,
        }
    }
    #[doc = "TIM17_CH1 and TIM17_UP DMA requests mapped on DMA channel 1"]
    #[inline(always)]
    pub fn is_not_remapped(&self) -> bool {
        *self == TIM17_DMA_RMP::NotRemapped
    }
    #[doc = "TIM17_CH1 and TIM17_UP DMA requests mapped on DMA channel 2"]
    #[inline(always)]
    pub fn is_remapped(&self) -> bool {
        *self == TIM17_DMA_RMP::Remapped
    }
}
#[doc = "Field `TIM17_DMA_RMP` writer - TIM17 DMA request remapping bit"]
pub type TIM17_DMA_RMP_W<'a, REG> = crate::BitWriter<'a, REG, TIM17_DMA_RMP>;
impl<'a, REG> TIM17_DMA_RMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TIM17_CH1 and TIM17_UP DMA requests mapped on DMA channel 1"]
    #[inline(always)]
    pub fn not_remapped(self) -> &'a mut crate::W<REG> {
        self.variant(TIM17_DMA_RMP::NotRemapped)
    }
    #[doc = "TIM17_CH1 and TIM17_UP DMA requests mapped on DMA channel 2"]
    #[inline(always)]
    pub fn remapped(self) -> &'a mut crate::W<REG> {
        self.variant(TIM17_DMA_RMP::Remapped)
    }
}
#[doc = "TIM6 and DAC1 DMA request remapping bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM6_DAC1_OUT1_DMA_RMP {
    #[doc = "0: TIM7 and DAC1_OUT1 DMA requests mapped on DMA2 channel 3"]
    NotRemapped = 0,
    #[doc = "1: TIM7 and DAC1_OUT1 DMA requests mapped on DMA1 channel 3"]
    Remapped = 1,
}
impl From<TIM6_DAC1_OUT1_DMA_RMP> for bool {
    #[inline(always)]
    fn from(variant: TIM6_DAC1_OUT1_DMA_RMP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM6_DAC1_OUT1_DMA_RMP` reader - TIM6 and DAC1 DMA request remapping bit"]
pub type TIM6_DAC1_OUT1_DMA_RMP_R = crate::BitReader<TIM6_DAC1_OUT1_DMA_RMP>;
impl TIM6_DAC1_OUT1_DMA_RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIM6_DAC1_OUT1_DMA_RMP {
        match self.bits {
            false => TIM6_DAC1_OUT1_DMA_RMP::NotRemapped,
            true => TIM6_DAC1_OUT1_DMA_RMP::Remapped,
        }
    }
    #[doc = "TIM7 and DAC1_OUT1 DMA requests mapped on DMA2 channel 3"]
    #[inline(always)]
    pub fn is_not_remapped(&self) -> bool {
        *self == TIM6_DAC1_OUT1_DMA_RMP::NotRemapped
    }
    #[doc = "TIM7 and DAC1_OUT1 DMA requests mapped on DMA1 channel 3"]
    #[inline(always)]
    pub fn is_remapped(&self) -> bool {
        *self == TIM6_DAC1_OUT1_DMA_RMP::Remapped
    }
}
#[doc = "Field `TIM6_DAC1_OUT1_DMA_RMP` writer - TIM6 and DAC1 DMA request remapping bit"]
pub type TIM6_DAC1_OUT1_DMA_RMP_W<'a, REG> = crate::BitWriter<'a, REG, TIM6_DAC1_OUT1_DMA_RMP>;
impl<'a, REG> TIM6_DAC1_OUT1_DMA_RMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TIM7 and DAC1_OUT1 DMA requests mapped on DMA2 channel 3"]
    #[inline(always)]
    pub fn not_remapped(self) -> &'a mut crate::W<REG> {
        self.variant(TIM6_DAC1_OUT1_DMA_RMP::NotRemapped)
    }
    #[doc = "TIM7 and DAC1_OUT1 DMA requests mapped on DMA1 channel 3"]
    #[inline(always)]
    pub fn remapped(self) -> &'a mut crate::W<REG> {
        self.variant(TIM6_DAC1_OUT1_DMA_RMP::Remapped)
    }
}
#[doc = "TIM7 and DAC2 DMA request remapping bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM7_DAC1_OUT2_DMA_RMP {
    #[doc = "0: TIM7 and DAC1_OUT2 DMA requests mapped on DMA2 channel 4"]
    NotRemapped = 0,
    #[doc = "1: TIM7 and DAC1_OUT2 DMA requests mapped on DMA1 channel 4"]
    Remapped = 1,
}
impl From<TIM7_DAC1_OUT2_DMA_RMP> for bool {
    #[inline(always)]
    fn from(variant: TIM7_DAC1_OUT2_DMA_RMP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM7_DAC1_OUT2_DMA_RMP` reader - TIM7 and DAC2 DMA request remapping bit"]
pub type TIM7_DAC1_OUT2_DMA_RMP_R = crate::BitReader<TIM7_DAC1_OUT2_DMA_RMP>;
impl TIM7_DAC1_OUT2_DMA_RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIM7_DAC1_OUT2_DMA_RMP {
        match self.bits {
            false => TIM7_DAC1_OUT2_DMA_RMP::NotRemapped,
            true => TIM7_DAC1_OUT2_DMA_RMP::Remapped,
        }
    }
    #[doc = "TIM7 and DAC1_OUT2 DMA requests mapped on DMA2 channel 4"]
    #[inline(always)]
    pub fn is_not_remapped(&self) -> bool {
        *self == TIM7_DAC1_OUT2_DMA_RMP::NotRemapped
    }
    #[doc = "TIM7 and DAC1_OUT2 DMA requests mapped on DMA1 channel 4"]
    #[inline(always)]
    pub fn is_remapped(&self) -> bool {
        *self == TIM7_DAC1_OUT2_DMA_RMP::Remapped
    }
}
#[doc = "Field `TIM7_DAC1_OUT2_DMA_RMP` writer - TIM7 and DAC2 DMA request remapping bit"]
pub type TIM7_DAC1_OUT2_DMA_RMP_W<'a, REG> = crate::BitWriter<'a, REG, TIM7_DAC1_OUT2_DMA_RMP>;
impl<'a, REG> TIM7_DAC1_OUT2_DMA_RMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TIM7 and DAC1_OUT2 DMA requests mapped on DMA2 channel 4"]
    #[inline(always)]
    pub fn not_remapped(self) -> &'a mut crate::W<REG> {
        self.variant(TIM7_DAC1_OUT2_DMA_RMP::NotRemapped)
    }
    #[doc = "TIM7 and DAC1_OUT2 DMA requests mapped on DMA1 channel 4"]
    #[inline(always)]
    pub fn remapped(self) -> &'a mut crate::W<REG> {
        self.variant(TIM7_DAC1_OUT2_DMA_RMP::Remapped)
    }
}
#[doc = "TIM18 and DAC2_OUT1 DMA request remapping bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM18_DAC2_OUT1_DMA_RMP {
    #[doc = "0: TIM18 and DAC2_OUT1 DMA requests mapped on DMA2 channel 5"]
    NotRemapped = 0,
    #[doc = "1: TIM18 and DAC2_OUT1 DMA requests mapped on DMA1 channel 5"]
    Remapped = 1,
}
impl From<TIM18_DAC2_OUT1_DMA_RMP> for bool {
    #[inline(always)]
    fn from(variant: TIM18_DAC2_OUT1_DMA_RMP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM18_DAC2_OUT1_DMA_RMP` reader - TIM18 and DAC2_OUT1 DMA request remapping bit"]
pub type TIM18_DAC2_OUT1_DMA_RMP_R = crate::BitReader<TIM18_DAC2_OUT1_DMA_RMP>;
impl TIM18_DAC2_OUT1_DMA_RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIM18_DAC2_OUT1_DMA_RMP {
        match self.bits {
            false => TIM18_DAC2_OUT1_DMA_RMP::NotRemapped,
            true => TIM18_DAC2_OUT1_DMA_RMP::Remapped,
        }
    }
    #[doc = "TIM18 and DAC2_OUT1 DMA requests mapped on DMA2 channel 5"]
    #[inline(always)]
    pub fn is_not_remapped(&self) -> bool {
        *self == TIM18_DAC2_OUT1_DMA_RMP::NotRemapped
    }
    #[doc = "TIM18 and DAC2_OUT1 DMA requests mapped on DMA1 channel 5"]
    #[inline(always)]
    pub fn is_remapped(&self) -> bool {
        *self == TIM18_DAC2_OUT1_DMA_RMP::Remapped
    }
}
#[doc = "Field `TIM18_DAC2_OUT1_DMA_RMP` writer - TIM18 and DAC2_OUT1 DMA request remapping bit"]
pub type TIM18_DAC2_OUT1_DMA_RMP_W<'a, REG> = crate::BitWriter<'a, REG, TIM18_DAC2_OUT1_DMA_RMP>;
impl<'a, REG> TIM18_DAC2_OUT1_DMA_RMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TIM18 and DAC2_OUT1 DMA requests mapped on DMA2 channel 5"]
    #[inline(always)]
    pub fn not_remapped(self) -> &'a mut crate::W<REG> {
        self.variant(TIM18_DAC2_OUT1_DMA_RMP::NotRemapped)
    }
    #[doc = "TIM18 and DAC2_OUT1 DMA requests mapped on DMA1 channel 5"]
    #[inline(always)]
    pub fn remapped(self) -> &'a mut crate::W<REG> {
        self.variant(TIM18_DAC2_OUT1_DMA_RMP::Remapped)
    }
}
#[doc = "Fast Mode Plus (FM+) driving capability activation bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C_PB6_FMP {
    #[doc = "0: PB6 pin operate in standard mode"]
    Standard = 0,
    #[doc = "1: I2C FM+ mode enabled on PB6 and the Speed control is bypassed"]
    Fmp = 1,
}
impl From<I2C_PB6_FMP> for bool {
    #[inline(always)]
    fn from(variant: I2C_PB6_FMP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C_PB6_FMP` reader - Fast Mode Plus (FM+) driving capability activation bits."]
pub type I2C_PB6_FMP_R = crate::BitReader<I2C_PB6_FMP>;
impl I2C_PB6_FMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2C_PB6_FMP {
        match self.bits {
            false => I2C_PB6_FMP::Standard,
            true => I2C_PB6_FMP::Fmp,
        }
    }
    #[doc = "PB6 pin operate in standard mode"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == I2C_PB6_FMP::Standard
    }
    #[doc = "I2C FM+ mode enabled on PB6 and the Speed control is bypassed"]
    #[inline(always)]
    pub fn is_fmp(&self) -> bool {
        *self == I2C_PB6_FMP::Fmp
    }
}
#[doc = "Field `I2C_PB6_FMP` writer - Fast Mode Plus (FM+) driving capability activation bits."]
pub type I2C_PB6_FMP_W<'a, REG> = crate::BitWriter<'a, REG, I2C_PB6_FMP>;
impl<'a, REG> I2C_PB6_FMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PB6 pin operate in standard mode"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(I2C_PB6_FMP::Standard)
    }
    #[doc = "I2C FM+ mode enabled on PB6 and the Speed control is bypassed"]
    #[inline(always)]
    pub fn fmp(self) -> &'a mut crate::W<REG> {
        self.variant(I2C_PB6_FMP::Fmp)
    }
}
#[doc = "Fast Mode Plus (FM+) driving capability activation bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C_PB7_FMP {
    #[doc = "0: PB7 pin operate in standard mode"]
    Standard = 0,
    #[doc = "1: I2C FM+ mode enabled on PB7 and the Speed control is bypassed"]
    Fmp = 1,
}
impl From<I2C_PB7_FMP> for bool {
    #[inline(always)]
    fn from(variant: I2C_PB7_FMP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C_PB7_FMP` reader - Fast Mode Plus (FM+) driving capability activation bits."]
pub type I2C_PB7_FMP_R = crate::BitReader<I2C_PB7_FMP>;
impl I2C_PB7_FMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2C_PB7_FMP {
        match self.bits {
            false => I2C_PB7_FMP::Standard,
            true => I2C_PB7_FMP::Fmp,
        }
    }
    #[doc = "PB7 pin operate in standard mode"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == I2C_PB7_FMP::Standard
    }
    #[doc = "I2C FM+ mode enabled on PB7 and the Speed control is bypassed"]
    #[inline(always)]
    pub fn is_fmp(&self) -> bool {
        *self == I2C_PB7_FMP::Fmp
    }
}
#[doc = "Field `I2C_PB7_FMP` writer - Fast Mode Plus (FM+) driving capability activation bits."]
pub type I2C_PB7_FMP_W<'a, REG> = crate::BitWriter<'a, REG, I2C_PB7_FMP>;
impl<'a, REG> I2C_PB7_FMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PB7 pin operate in standard mode"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(I2C_PB7_FMP::Standard)
    }
    #[doc = "I2C FM+ mode enabled on PB7 and the Speed control is bypassed"]
    #[inline(always)]
    pub fn fmp(self) -> &'a mut crate::W<REG> {
        self.variant(I2C_PB7_FMP::Fmp)
    }
}
#[doc = "Fast Mode Plus (FM+) driving capability activation bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C_PB8_FMP {
    #[doc = "0: PB8 pin operate in standard mode"]
    Standard = 0,
    #[doc = "1: I2C FM+ mode enabled on PB8 and the Speed control is bypassed"]
    Fmp = 1,
}
impl From<I2C_PB8_FMP> for bool {
    #[inline(always)]
    fn from(variant: I2C_PB8_FMP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C_PB8_FMP` reader - Fast Mode Plus (FM+) driving capability activation bits."]
pub type I2C_PB8_FMP_R = crate::BitReader<I2C_PB8_FMP>;
impl I2C_PB8_FMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2C_PB8_FMP {
        match self.bits {
            false => I2C_PB8_FMP::Standard,
            true => I2C_PB8_FMP::Fmp,
        }
    }
    #[doc = "PB8 pin operate in standard mode"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == I2C_PB8_FMP::Standard
    }
    #[doc = "I2C FM+ mode enabled on PB8 and the Speed control is bypassed"]
    #[inline(always)]
    pub fn is_fmp(&self) -> bool {
        *self == I2C_PB8_FMP::Fmp
    }
}
#[doc = "Field `I2C_PB8_FMP` writer - Fast Mode Plus (FM+) driving capability activation bits."]
pub type I2C_PB8_FMP_W<'a, REG> = crate::BitWriter<'a, REG, I2C_PB8_FMP>;
impl<'a, REG> I2C_PB8_FMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PB8 pin operate in standard mode"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(I2C_PB8_FMP::Standard)
    }
    #[doc = "I2C FM+ mode enabled on PB8 and the Speed control is bypassed"]
    #[inline(always)]
    pub fn fmp(self) -> &'a mut crate::W<REG> {
        self.variant(I2C_PB8_FMP::Fmp)
    }
}
#[doc = "Fast Mode Plus (FM+) driving capability activation bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C_PB9_FMP {
    #[doc = "0: PB9 pin operate in standard mode"]
    Standard = 0,
    #[doc = "1: I2C FM+ mode enabled on PB9 and the Speed control is bypassed"]
    Fmp = 1,
}
impl From<I2C_PB9_FMP> for bool {
    #[inline(always)]
    fn from(variant: I2C_PB9_FMP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C_PB9_FMP` reader - Fast Mode Plus (FM+) driving capability activation bits."]
pub type I2C_PB9_FMP_R = crate::BitReader<I2C_PB9_FMP>;
impl I2C_PB9_FMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2C_PB9_FMP {
        match self.bits {
            false => I2C_PB9_FMP::Standard,
            true => I2C_PB9_FMP::Fmp,
        }
    }
    #[doc = "PB9 pin operate in standard mode"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == I2C_PB9_FMP::Standard
    }
    #[doc = "I2C FM+ mode enabled on PB9 and the Speed control is bypassed"]
    #[inline(always)]
    pub fn is_fmp(&self) -> bool {
        *self == I2C_PB9_FMP::Fmp
    }
}
#[doc = "Field `I2C_PB9_FMP` writer - Fast Mode Plus (FM+) driving capability activation bits."]
pub type I2C_PB9_FMP_W<'a, REG> = crate::BitWriter<'a, REG, I2C_PB9_FMP>;
impl<'a, REG> I2C_PB9_FMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PB9 pin operate in standard mode"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(I2C_PB9_FMP::Standard)
    }
    #[doc = "I2C FM+ mode enabled on PB9 and the Speed control is bypassed"]
    #[inline(always)]
    pub fn fmp(self) -> &'a mut crate::W<REG> {
        self.variant(I2C_PB9_FMP::Fmp)
    }
}
#[doc = "I2C1 Fast Mode Plus\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C1_FMP {
    #[doc = "0: FM+ mode is controlled by I2C_Pxx_FMP bits only"]
    Standard = 0,
    #[doc = "1: FM+ mode is enabled on all I2C1 pins selected through selection through IOPORT control registers AF selection bits"]
    Fmp = 1,
}
impl From<I2C1_FMP> for bool {
    #[inline(always)]
    fn from(variant: I2C1_FMP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C1_FMP` reader - I2C1 Fast Mode Plus"]
pub type I2C1_FMP_R = crate::BitReader<I2C1_FMP>;
impl I2C1_FMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2C1_FMP {
        match self.bits {
            false => I2C1_FMP::Standard,
            true => I2C1_FMP::Fmp,
        }
    }
    #[doc = "FM+ mode is controlled by I2C_Pxx_FMP bits only"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == I2C1_FMP::Standard
    }
    #[doc = "FM+ mode is enabled on all I2C1 pins selected through selection through IOPORT control registers AF selection bits"]
    #[inline(always)]
    pub fn is_fmp(&self) -> bool {
        *self == I2C1_FMP::Fmp
    }
}
#[doc = "Field `I2C1_FMP` writer - I2C1 Fast Mode Plus"]
pub type I2C1_FMP_W<'a, REG> = crate::BitWriter<'a, REG, I2C1_FMP>;
impl<'a, REG> I2C1_FMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FM+ mode is controlled by I2C_Pxx_FMP bits only"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1_FMP::Standard)
    }
    #[doc = "FM+ mode is enabled on all I2C1 pins selected through selection through IOPORT control registers AF selection bits"]
    #[inline(always)]
    pub fn fmp(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1_FMP::Fmp)
    }
}
#[doc = "I2C2 Fast Mode Plus\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C2_FMP {
    #[doc = "0: FM+ mode is controlled by I2C_Pxx_FMP bits only"]
    Standard = 0,
    #[doc = "1: FM+ mode is enabled on all I2C2 pins selected through selection through IOPORT control registers AF selection bits"]
    Fmp = 1,
}
impl From<I2C2_FMP> for bool {
    #[inline(always)]
    fn from(variant: I2C2_FMP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C2_FMP` reader - I2C2 Fast Mode Plus"]
pub type I2C2_FMP_R = crate::BitReader<I2C2_FMP>;
impl I2C2_FMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2C2_FMP {
        match self.bits {
            false => I2C2_FMP::Standard,
            true => I2C2_FMP::Fmp,
        }
    }
    #[doc = "FM+ mode is controlled by I2C_Pxx_FMP bits only"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == I2C2_FMP::Standard
    }
    #[doc = "FM+ mode is enabled on all I2C2 pins selected through selection through IOPORT control registers AF selection bits"]
    #[inline(always)]
    pub fn is_fmp(&self) -> bool {
        *self == I2C2_FMP::Fmp
    }
}
#[doc = "Field `I2C2_FMP` writer - I2C2 Fast Mode Plus"]
pub type I2C2_FMP_W<'a, REG> = crate::BitWriter<'a, REG, I2C2_FMP>;
impl<'a, REG> I2C2_FMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FM+ mode is controlled by I2C_Pxx_FMP bits only"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(I2C2_FMP::Standard)
    }
    #[doc = "FM+ mode is enabled on all I2C2 pins selected through selection through IOPORT control registers AF selection bits"]
    #[inline(always)]
    pub fn fmp(self) -> &'a mut crate::W<REG> {
        self.variant(I2C2_FMP::Fmp)
    }
}
#[doc = "VBAT monitoring enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBAT_MON {
    #[doc = "0: Disable the power switch to not deliver VBAT voltage on ADC channel 18 input"]
    Disable = 0,
    #[doc = "1: Enable the power switch to deliver VBAT voltage on ADC channel 18 input"]
    Enable = 1,
}
impl From<VBAT_MON> for bool {
    #[inline(always)]
    fn from(variant: VBAT_MON) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBAT_MON` reader - VBAT monitoring enable"]
pub type VBAT_MON_R = crate::BitReader<VBAT_MON>;
impl VBAT_MON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VBAT_MON {
        match self.bits {
            false => VBAT_MON::Disable,
            true => VBAT_MON::Enable,
        }
    }
    #[doc = "Disable the power switch to not deliver VBAT voltage on ADC channel 18 input"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == VBAT_MON::Disable
    }
    #[doc = "Enable the power switch to deliver VBAT voltage on ADC channel 18 input"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == VBAT_MON::Enable
    }
}
#[doc = "Field `VBAT_MON` writer - VBAT monitoring enable"]
pub type VBAT_MON_W<'a, REG> = crate::BitWriter<'a, REG, VBAT_MON>;
impl<'a, REG> VBAT_MON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the power switch to not deliver VBAT voltage on ADC channel 18 input"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(VBAT_MON::Disable)
    }
    #[doc = "Enable the power switch to deliver VBAT voltage on ADC channel 18 input"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(VBAT_MON::Enable)
    }
}
#[doc = "Invalid operation interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPU_IE0 {
    #[doc = "0: Invalid operation interrupt disable"]
    Disabled = 0,
    #[doc = "1: Invalid operation interrupt enable"]
    Enabled = 1,
}
impl From<FPU_IE0> for bool {
    #[inline(always)]
    fn from(variant: FPU_IE0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPU_IE0` reader - Invalid operation interrupt enable"]
pub type FPU_IE0_R = crate::BitReader<FPU_IE0>;
impl FPU_IE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FPU_IE0 {
        match self.bits {
            false => FPU_IE0::Disabled,
            true => FPU_IE0::Enabled,
        }
    }
    #[doc = "Invalid operation interrupt disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FPU_IE0::Disabled
    }
    #[doc = "Invalid operation interrupt enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FPU_IE0::Enabled
    }
}
#[doc = "Field `FPU_IE0` writer - Invalid operation interrupt enable"]
pub type FPU_IE0_W<'a, REG> = crate::BitWriter<'a, REG, FPU_IE0>;
impl<'a, REG> FPU_IE0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Invalid operation interrupt disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FPU_IE0::Disabled)
    }
    #[doc = "Invalid operation interrupt enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FPU_IE0::Enabled)
    }
}
#[doc = "Devide-by-zero interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPU_IE1 {
    #[doc = "0: Devide-by-zero interrupt disable"]
    Disabled = 0,
    #[doc = "1: Devide-by-zero interrupt enable"]
    Enabled = 1,
}
impl From<FPU_IE1> for bool {
    #[inline(always)]
    fn from(variant: FPU_IE1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPU_IE1` reader - Devide-by-zero interrupt enable"]
pub type FPU_IE1_R = crate::BitReader<FPU_IE1>;
impl FPU_IE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FPU_IE1 {
        match self.bits {
            false => FPU_IE1::Disabled,
            true => FPU_IE1::Enabled,
        }
    }
    #[doc = "Devide-by-zero interrupt disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FPU_IE1::Disabled
    }
    #[doc = "Devide-by-zero interrupt enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FPU_IE1::Enabled
    }
}
#[doc = "Field `FPU_IE1` writer - Devide-by-zero interrupt enable"]
pub type FPU_IE1_W<'a, REG> = crate::BitWriter<'a, REG, FPU_IE1>;
impl<'a, REG> FPU_IE1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Devide-by-zero interrupt disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FPU_IE1::Disabled)
    }
    #[doc = "Devide-by-zero interrupt enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FPU_IE1::Enabled)
    }
}
#[doc = "Underflow interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPU_IE2 {
    #[doc = "0: Underflow interrupt disable"]
    Disabled = 0,
    #[doc = "1: Underflow interrupt enable"]
    Enabled = 1,
}
impl From<FPU_IE2> for bool {
    #[inline(always)]
    fn from(variant: FPU_IE2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPU_IE2` reader - Underflow interrupt enable"]
pub type FPU_IE2_R = crate::BitReader<FPU_IE2>;
impl FPU_IE2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FPU_IE2 {
        match self.bits {
            false => FPU_IE2::Disabled,
            true => FPU_IE2::Enabled,
        }
    }
    #[doc = "Underflow interrupt disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FPU_IE2::Disabled
    }
    #[doc = "Underflow interrupt enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FPU_IE2::Enabled
    }
}
#[doc = "Field `FPU_IE2` writer - Underflow interrupt enable"]
pub type FPU_IE2_W<'a, REG> = crate::BitWriter<'a, REG, FPU_IE2>;
impl<'a, REG> FPU_IE2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Underflow interrupt disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FPU_IE2::Disabled)
    }
    #[doc = "Underflow interrupt enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FPU_IE2::Enabled)
    }
}
#[doc = "Overflow interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPU_IE3 {
    #[doc = "0: Overflow interrupt disable"]
    Disabled = 0,
    #[doc = "1: Overflow interrupt enable"]
    Enabled = 1,
}
impl From<FPU_IE3> for bool {
    #[inline(always)]
    fn from(variant: FPU_IE3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPU_IE3` reader - Overflow interrupt enable"]
pub type FPU_IE3_R = crate::BitReader<FPU_IE3>;
impl FPU_IE3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FPU_IE3 {
        match self.bits {
            false => FPU_IE3::Disabled,
            true => FPU_IE3::Enabled,
        }
    }
    #[doc = "Overflow interrupt disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FPU_IE3::Disabled
    }
    #[doc = "Overflow interrupt enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FPU_IE3::Enabled
    }
}
#[doc = "Field `FPU_IE3` writer - Overflow interrupt enable"]
pub type FPU_IE3_W<'a, REG> = crate::BitWriter<'a, REG, FPU_IE3>;
impl<'a, REG> FPU_IE3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Overflow interrupt disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FPU_IE3::Disabled)
    }
    #[doc = "Overflow interrupt enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FPU_IE3::Enabled)
    }
}
#[doc = "Input denormal interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPU_IE4 {
    #[doc = "0: Input denormal interrupt disable"]
    Disabled = 0,
    #[doc = "1: Input denormal interrupt enable"]
    Enabled = 1,
}
impl From<FPU_IE4> for bool {
    #[inline(always)]
    fn from(variant: FPU_IE4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPU_IE4` reader - Input denormal interrupt enable"]
pub type FPU_IE4_R = crate::BitReader<FPU_IE4>;
impl FPU_IE4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FPU_IE4 {
        match self.bits {
            false => FPU_IE4::Disabled,
            true => FPU_IE4::Enabled,
        }
    }
    #[doc = "Input denormal interrupt disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FPU_IE4::Disabled
    }
    #[doc = "Input denormal interrupt enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FPU_IE4::Enabled
    }
}
#[doc = "Field `FPU_IE4` writer - Input denormal interrupt enable"]
pub type FPU_IE4_W<'a, REG> = crate::BitWriter<'a, REG, FPU_IE4>;
impl<'a, REG> FPU_IE4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input denormal interrupt disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FPU_IE4::Disabled)
    }
    #[doc = "Input denormal interrupt enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FPU_IE4::Enabled)
    }
}
#[doc = "Inexact interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPU_IE5 {
    #[doc = "0: Inexact interrupt disable"]
    Disabled = 0,
    #[doc = "1: Inexact interrupt enable"]
    Enabled = 1,
}
impl From<FPU_IE5> for bool {
    #[inline(always)]
    fn from(variant: FPU_IE5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPU_IE5` reader - Inexact interrupt enable"]
pub type FPU_IE5_R = crate::BitReader<FPU_IE5>;
impl FPU_IE5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FPU_IE5 {
        match self.bits {
            false => FPU_IE5::Disabled,
            true => FPU_IE5::Enabled,
        }
    }
    #[doc = "Inexact interrupt disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FPU_IE5::Disabled
    }
    #[doc = "Inexact interrupt enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FPU_IE5::Enabled
    }
}
#[doc = "Field `FPU_IE5` writer - Inexact interrupt enable"]
pub type FPU_IE5_W<'a, REG> = crate::BitWriter<'a, REG, FPU_IE5>;
impl<'a, REG> FPU_IE5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Inexact interrupt disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FPU_IE5::Disabled)
    }
    #[doc = "Inexact interrupt enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FPU_IE5::Enabled)
    }
}
impl R {
    #[doc = "Bits 0:1 - Memory mapping selection bits"]
    #[inline(always)]
    pub fn mem_mode(&self) -> MEM_MODE_R {
        MEM_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 11 - TIM16 DMA request remapping bit"]
    #[inline(always)]
    pub fn tim16_dma_rmp(&self) -> TIM16_DMA_RMP_R {
        TIM16_DMA_RMP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TIM17 DMA request remapping bit"]
    #[inline(always)]
    pub fn tim17_dma_rmp(&self) -> TIM17_DMA_RMP_R {
        TIM17_DMA_RMP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TIM6 and DAC1 DMA request remapping bit"]
    #[inline(always)]
    pub fn tim6_dac1_out1_dma_rmp(&self) -> TIM6_DAC1_OUT1_DMA_RMP_R {
        TIM6_DAC1_OUT1_DMA_RMP_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - TIM7 and DAC2 DMA request remapping bit"]
    #[inline(always)]
    pub fn tim7_dac1_out2_dma_rmp(&self) -> TIM7_DAC1_OUT2_DMA_RMP_R {
        TIM7_DAC1_OUT2_DMA_RMP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - TIM18 and DAC2_OUT1 DMA request remapping bit"]
    #[inline(always)]
    pub fn tim18_dac2_out1_dma_rmp(&self) -> TIM18_DAC2_OUT1_DMA_RMP_R {
        TIM18_DAC2_OUT1_DMA_RMP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Fast Mode Plus (FM+) driving capability activation bits."]
    #[inline(always)]
    pub fn i2c_pb6_fmp(&self) -> I2C_PB6_FMP_R {
        I2C_PB6_FMP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Fast Mode Plus (FM+) driving capability activation bits."]
    #[inline(always)]
    pub fn i2c_pb7_fmp(&self) -> I2C_PB7_FMP_R {
        I2C_PB7_FMP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Fast Mode Plus (FM+) driving capability activation bits."]
    #[inline(always)]
    pub fn i2c_pb8_fmp(&self) -> I2C_PB8_FMP_R {
        I2C_PB8_FMP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Fast Mode Plus (FM+) driving capability activation bits."]
    #[inline(always)]
    pub fn i2c_pb9_fmp(&self) -> I2C_PB9_FMP_R {
        I2C_PB9_FMP_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - I2C1 Fast Mode Plus"]
    #[inline(always)]
    pub fn i2c1_fmp(&self) -> I2C1_FMP_R {
        I2C1_FMP_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C2 Fast Mode Plus"]
    #[inline(always)]
    pub fn i2c2_fmp(&self) -> I2C2_FMP_R {
        I2C2_FMP_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - VBAT monitoring enable"]
    #[inline(always)]
    pub fn vbat_mon(&self) -> VBAT_MON_R {
        VBAT_MON_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - Invalid operation interrupt enable"]
    #[inline(always)]
    pub fn fpu_ie0(&self) -> FPU_IE0_R {
        FPU_IE0_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Devide-by-zero interrupt enable"]
    #[inline(always)]
    pub fn fpu_ie1(&self) -> FPU_IE1_R {
        FPU_IE1_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Underflow interrupt enable"]
    #[inline(always)]
    pub fn fpu_ie2(&self) -> FPU_IE2_R {
        FPU_IE2_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Overflow interrupt enable"]
    #[inline(always)]
    pub fn fpu_ie3(&self) -> FPU_IE3_R {
        FPU_IE3_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Input denormal interrupt enable"]
    #[inline(always)]
    pub fn fpu_ie4(&self) -> FPU_IE4_R {
        FPU_IE4_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Inexact interrupt enable"]
    #[inline(always)]
    pub fn fpu_ie5(&self) -> FPU_IE5_R {
        FPU_IE5_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Memory mapping selection bits"]
    #[inline(always)]
    #[must_use]
    pub fn mem_mode(&mut self) -> MEM_MODE_W<CFGR1rs> {
        MEM_MODE_W::new(self, 0)
    }
    #[doc = "Bit 11 - TIM16 DMA request remapping bit"]
    #[inline(always)]
    #[must_use]
    pub fn tim16_dma_rmp(&mut self) -> TIM16_DMA_RMP_W<CFGR1rs> {
        TIM16_DMA_RMP_W::new(self, 11)
    }
    #[doc = "Bit 12 - TIM17 DMA request remapping bit"]
    #[inline(always)]
    #[must_use]
    pub fn tim17_dma_rmp(&mut self) -> TIM17_DMA_RMP_W<CFGR1rs> {
        TIM17_DMA_RMP_W::new(self, 12)
    }
    #[doc = "Bit 13 - TIM6 and DAC1 DMA request remapping bit"]
    #[inline(always)]
    #[must_use]
    pub fn tim6_dac1_out1_dma_rmp(&mut self) -> TIM6_DAC1_OUT1_DMA_RMP_W<CFGR1rs> {
        TIM6_DAC1_OUT1_DMA_RMP_W::new(self, 13)
    }
    #[doc = "Bit 14 - TIM7 and DAC2 DMA request remapping bit"]
    #[inline(always)]
    #[must_use]
    pub fn tim7_dac1_out2_dma_rmp(&mut self) -> TIM7_DAC1_OUT2_DMA_RMP_W<CFGR1rs> {
        TIM7_DAC1_OUT2_DMA_RMP_W::new(self, 14)
    }
    #[doc = "Bit 15 - TIM18 and DAC2_OUT1 DMA request remapping bit"]
    #[inline(always)]
    #[must_use]
    pub fn tim18_dac2_out1_dma_rmp(&mut self) -> TIM18_DAC2_OUT1_DMA_RMP_W<CFGR1rs> {
        TIM18_DAC2_OUT1_DMA_RMP_W::new(self, 15)
    }
    #[doc = "Bit 16 - Fast Mode Plus (FM+) driving capability activation bits."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_pb6_fmp(&mut self) -> I2C_PB6_FMP_W<CFGR1rs> {
        I2C_PB6_FMP_W::new(self, 16)
    }
    #[doc = "Bit 17 - Fast Mode Plus (FM+) driving capability activation bits."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_pb7_fmp(&mut self) -> I2C_PB7_FMP_W<CFGR1rs> {
        I2C_PB7_FMP_W::new(self, 17)
    }
    #[doc = "Bit 18 - Fast Mode Plus (FM+) driving capability activation bits."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_pb8_fmp(&mut self) -> I2C_PB8_FMP_W<CFGR1rs> {
        I2C_PB8_FMP_W::new(self, 18)
    }
    #[doc = "Bit 19 - Fast Mode Plus (FM+) driving capability activation bits."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_pb9_fmp(&mut self) -> I2C_PB9_FMP_W<CFGR1rs> {
        I2C_PB9_FMP_W::new(self, 19)
    }
    #[doc = "Bit 20 - I2C1 Fast Mode Plus"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1_fmp(&mut self) -> I2C1_FMP_W<CFGR1rs> {
        I2C1_FMP_W::new(self, 20)
    }
    #[doc = "Bit 21 - I2C2 Fast Mode Plus"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2_fmp(&mut self) -> I2C2_FMP_W<CFGR1rs> {
        I2C2_FMP_W::new(self, 21)
    }
    #[doc = "Bit 24 - VBAT monitoring enable"]
    #[inline(always)]
    #[must_use]
    pub fn vbat_mon(&mut self) -> VBAT_MON_W<CFGR1rs> {
        VBAT_MON_W::new(self, 24)
    }
    #[doc = "Bit 26 - Invalid operation interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn fpu_ie0(&mut self) -> FPU_IE0_W<CFGR1rs> {
        FPU_IE0_W::new(self, 26)
    }
    #[doc = "Bit 27 - Devide-by-zero interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn fpu_ie1(&mut self) -> FPU_IE1_W<CFGR1rs> {
        FPU_IE1_W::new(self, 27)
    }
    #[doc = "Bit 28 - Underflow interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn fpu_ie2(&mut self) -> FPU_IE2_W<CFGR1rs> {
        FPU_IE2_W::new(self, 28)
    }
    #[doc = "Bit 29 - Overflow interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn fpu_ie3(&mut self) -> FPU_IE3_W<CFGR1rs> {
        FPU_IE3_W::new(self, 29)
    }
    #[doc = "Bit 30 - Input denormal interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn fpu_ie4(&mut self) -> FPU_IE4_W<CFGR1rs> {
        FPU_IE4_W::new(self, 30)
    }
    #[doc = "Bit 31 - Inexact interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn fpu_ie5(&mut self) -> FPU_IE5_W<CFGR1rs> {
        FPU_IE5_W::new(self, 31)
    }
}
#[doc = "configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGR1rs;
impl crate::RegisterSpec for CFGR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr1::R`](R) reader structure"]
impl crate::Readable for CFGR1rs {}
#[doc = "`write(|w| ..)` method takes [`cfgr1::W`](W) writer structure"]
impl crate::Writable for CFGR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFGR1 to value 0"]
impl crate::Resettable for CFGR1rs {
    const RESET_VALUE: u32 = 0;
}
