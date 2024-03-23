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
    #[doc = "3: Embedded SRAM mapped at 0x0000_0000"]
    Sram = 3,
    #[doc = "4: FMC Bank (Only the first two banks) (Available on STM32F303xD/E only)"]
    Fmc = 4,
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
    pub const fn variant(&self) -> Option<MEM_MODE> {
        match self.bits {
            0 => Some(MEM_MODE::MainFlash),
            1 => Some(MEM_MODE::SystemFlash),
            3 => Some(MEM_MODE::Sram),
            4 => Some(MEM_MODE::Fmc),
            _ => None,
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
    #[doc = "Embedded SRAM mapped at 0x0000_0000"]
    #[inline(always)]
    pub fn is_sram(&self) -> bool {
        *self == MEM_MODE::Sram
    }
    #[doc = "FMC Bank (Only the first two banks) (Available on STM32F303xD/E only)"]
    #[inline(always)]
    pub fn is_fmc(&self) -> bool {
        *self == MEM_MODE::Fmc
    }
}
#[doc = "Field `MEM_MODE` writer - Memory mapping selection bits"]
pub type MEM_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3, MEM_MODE>;
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
    #[doc = "Embedded SRAM mapped at 0x0000_0000"]
    #[inline(always)]
    pub fn sram(self) -> &'a mut crate::W<REG> {
        self.variant(MEM_MODE::Sram)
    }
    #[doc = "FMC Bank (Only the first two banks) (Available on STM32F303xD/E only)"]
    #[inline(always)]
    pub fn fmc(self) -> &'a mut crate::W<REG> {
        self.variant(MEM_MODE::Fmc)
    }
}
#[doc = "USB interrupt remap\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USB_IT_RMP {
    #[doc = "0: USB_HP, USB_LP and USB_WAKEUP interrupts are mapped on interrupt lines 19, 20 and 42 respectively"]
    NotRemapped = 0,
    #[doc = "1: USB_HP, USB_LP and USB_WAKEUP interrupts are mapped on interrupt lines 74, 75 and 76 respectively"]
    Remapped = 1,
}
impl From<USB_IT_RMP> for bool {
    #[inline(always)]
    fn from(variant: USB_IT_RMP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USB_IT_RMP` reader - USB interrupt remap"]
pub type USB_IT_RMP_R = crate::BitReader<USB_IT_RMP>;
impl USB_IT_RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USB_IT_RMP {
        match self.bits {
            false => USB_IT_RMP::NotRemapped,
            true => USB_IT_RMP::Remapped,
        }
    }
    #[doc = "USB_HP, USB_LP and USB_WAKEUP interrupts are mapped on interrupt lines 19, 20 and 42 respectively"]
    #[inline(always)]
    pub fn is_not_remapped(&self) -> bool {
        *self == USB_IT_RMP::NotRemapped
    }
    #[doc = "USB_HP, USB_LP and USB_WAKEUP interrupts are mapped on interrupt lines 74, 75 and 76 respectively"]
    #[inline(always)]
    pub fn is_remapped(&self) -> bool {
        *self == USB_IT_RMP::Remapped
    }
}
#[doc = "Field `USB_IT_RMP` writer - USB interrupt remap"]
pub type USB_IT_RMP_W<'a, REG> = crate::BitWriter<'a, REG, USB_IT_RMP>;
impl<'a, REG> USB_IT_RMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "USB_HP, USB_LP and USB_WAKEUP interrupts are mapped on interrupt lines 19, 20 and 42 respectively"]
    #[inline(always)]
    pub fn not_remapped(self) -> &'a mut crate::W<REG> {
        self.variant(USB_IT_RMP::NotRemapped)
    }
    #[doc = "USB_HP, USB_LP and USB_WAKEUP interrupts are mapped on interrupt lines 74, 75 and 76 respectively"]
    #[inline(always)]
    pub fn remapped(self) -> &'a mut crate::W<REG> {
        self.variant(USB_IT_RMP::Remapped)
    }
}
#[doc = "Timer 1 ITR3 selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM1_ITR3_RMP {
    #[doc = "0: TIM1_ITR3 = TIM4_TRGO in STM32F303xB/C and STM32F358xC devices"]
    NotRemapped = 0,
    #[doc = "1: TIM1_ITR3 = TIM17_OC"]
    Remapped = 1,
}
impl From<TIM1_ITR3_RMP> for bool {
    #[inline(always)]
    fn from(variant: TIM1_ITR3_RMP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM1_ITR3_RMP` reader - Timer 1 ITR3 selection"]
pub type TIM1_ITR3_RMP_R = crate::BitReader<TIM1_ITR3_RMP>;
impl TIM1_ITR3_RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIM1_ITR3_RMP {
        match self.bits {
            false => TIM1_ITR3_RMP::NotRemapped,
            true => TIM1_ITR3_RMP::Remapped,
        }
    }
    #[doc = "TIM1_ITR3 = TIM4_TRGO in STM32F303xB/C and STM32F358xC devices"]
    #[inline(always)]
    pub fn is_not_remapped(&self) -> bool {
        *self == TIM1_ITR3_RMP::NotRemapped
    }
    #[doc = "TIM1_ITR3 = TIM17_OC"]
    #[inline(always)]
    pub fn is_remapped(&self) -> bool {
        *self == TIM1_ITR3_RMP::Remapped
    }
}
#[doc = "Field `TIM1_ITR3_RMP` writer - Timer 1 ITR3 selection"]
pub type TIM1_ITR3_RMP_W<'a, REG> = crate::BitWriter<'a, REG, TIM1_ITR3_RMP>;
impl<'a, REG> TIM1_ITR3_RMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TIM1_ITR3 = TIM4_TRGO in STM32F303xB/C and STM32F358xC devices"]
    #[inline(always)]
    pub fn not_remapped(self) -> &'a mut crate::W<REG> {
        self.variant(TIM1_ITR3_RMP::NotRemapped)
    }
    #[doc = "TIM1_ITR3 = TIM17_OC"]
    #[inline(always)]
    pub fn remapped(self) -> &'a mut crate::W<REG> {
        self.variant(TIM1_ITR3_RMP::Remapped)
    }
}
#[doc = "DAC trigger remap (when TSEL = 001)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAC1_TRIG_RMP {
    #[doc = "0: DAC trigger is TIM8_TRGO in STM32F303xB/C and STM32F358xC devices"]
    NotRemapped = 0,
    #[doc = "1: DAC trigger is TIM3_TRGO"]
    Remapped = 1,
}
impl From<DAC1_TRIG_RMP> for bool {
    #[inline(always)]
    fn from(variant: DAC1_TRIG_RMP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DAC1_TRIG_RMP` reader - DAC trigger remap (when TSEL = 001)"]
pub type DAC1_TRIG_RMP_R = crate::BitReader<DAC1_TRIG_RMP>;
impl DAC1_TRIG_RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DAC1_TRIG_RMP {
        match self.bits {
            false => DAC1_TRIG_RMP::NotRemapped,
            true => DAC1_TRIG_RMP::Remapped,
        }
    }
    #[doc = "DAC trigger is TIM8_TRGO in STM32F303xB/C and STM32F358xC devices"]
    #[inline(always)]
    pub fn is_not_remapped(&self) -> bool {
        *self == DAC1_TRIG_RMP::NotRemapped
    }
    #[doc = "DAC trigger is TIM3_TRGO"]
    #[inline(always)]
    pub fn is_remapped(&self) -> bool {
        *self == DAC1_TRIG_RMP::Remapped
    }
}
#[doc = "Field `DAC1_TRIG_RMP` writer - DAC trigger remap (when TSEL = 001)"]
pub type DAC1_TRIG_RMP_W<'a, REG> = crate::BitWriter<'a, REG, DAC1_TRIG_RMP>;
impl<'a, REG> DAC1_TRIG_RMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DAC trigger is TIM8_TRGO in STM32F303xB/C and STM32F358xC devices"]
    #[inline(always)]
    pub fn not_remapped(self) -> &'a mut crate::W<REG> {
        self.variant(DAC1_TRIG_RMP::NotRemapped)
    }
    #[doc = "DAC trigger is TIM3_TRGO"]
    #[inline(always)]
    pub fn remapped(self) -> &'a mut crate::W<REG> {
        self.variant(DAC1_TRIG_RMP::Remapped)
    }
}
#[doc = "ADC24 DMA remapping bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC2_DMA_RMP {
    #[doc = "0: ADC24 DMA requests mapped on DMA2 channels 1 and 2"]
    NotRemapped = 0,
    #[doc = "1: ADC24 DMA requests mapped on DMA2 channels 3 and 4"]
    Remapped = 1,
}
impl From<ADC2_DMA_RMP> for bool {
    #[inline(always)]
    fn from(variant: ADC2_DMA_RMP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC2_DMA_RMP` reader - ADC24 DMA remapping bit"]
pub type ADC2_DMA_RMP_R = crate::BitReader<ADC2_DMA_RMP>;
impl ADC2_DMA_RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADC2_DMA_RMP {
        match self.bits {
            false => ADC2_DMA_RMP::NotRemapped,
            true => ADC2_DMA_RMP::Remapped,
        }
    }
    #[doc = "ADC24 DMA requests mapped on DMA2 channels 1 and 2"]
    #[inline(always)]
    pub fn is_not_remapped(&self) -> bool {
        *self == ADC2_DMA_RMP::NotRemapped
    }
    #[doc = "ADC24 DMA requests mapped on DMA2 channels 3 and 4"]
    #[inline(always)]
    pub fn is_remapped(&self) -> bool {
        *self == ADC2_DMA_RMP::Remapped
    }
}
#[doc = "Field `ADC2_DMA_RMP` writer - ADC24 DMA remapping bit"]
pub type ADC2_DMA_RMP_W<'a, REG> = crate::BitWriter<'a, REG, ADC2_DMA_RMP>;
impl<'a, REG> ADC2_DMA_RMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC24 DMA requests mapped on DMA2 channels 1 and 2"]
    #[inline(always)]
    pub fn not_remapped(self) -> &'a mut crate::W<REG> {
        self.variant(ADC2_DMA_RMP::NotRemapped)
    }
    #[doc = "ADC24 DMA requests mapped on DMA2 channels 3 and 4"]
    #[inline(always)]
    pub fn remapped(self) -> &'a mut crate::W<REG> {
        self.variant(ADC2_DMA_RMP::Remapped)
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
pub enum TIM6_DAC1_CH1_DMA_RMP {
    #[doc = "0: TIM6_UP and DAC_CH1 DMA requests mapped on DMA2 channel 3 in STM32F303xB/C and STM32F358xC"]
    NotRemapped = 0,
    #[doc = "1: TIM6_UP and DAC_CH1 DMA requests mapped on DMA1 channel 3"]
    Remapped = 1,
}
impl From<TIM6_DAC1_CH1_DMA_RMP> for bool {
    #[inline(always)]
    fn from(variant: TIM6_DAC1_CH1_DMA_RMP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM6_DAC1_CH1_DMA_RMP` reader - TIM6 and DAC1 DMA request remapping bit"]
pub type TIM6_DAC1_CH1_DMA_RMP_R = crate::BitReader<TIM6_DAC1_CH1_DMA_RMP>;
impl TIM6_DAC1_CH1_DMA_RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIM6_DAC1_CH1_DMA_RMP {
        match self.bits {
            false => TIM6_DAC1_CH1_DMA_RMP::NotRemapped,
            true => TIM6_DAC1_CH1_DMA_RMP::Remapped,
        }
    }
    #[doc = "TIM6_UP and DAC_CH1 DMA requests mapped on DMA2 channel 3 in STM32F303xB/C and STM32F358xC"]
    #[inline(always)]
    pub fn is_not_remapped(&self) -> bool {
        *self == TIM6_DAC1_CH1_DMA_RMP::NotRemapped
    }
    #[doc = "TIM6_UP and DAC_CH1 DMA requests mapped on DMA1 channel 3"]
    #[inline(always)]
    pub fn is_remapped(&self) -> bool {
        *self == TIM6_DAC1_CH1_DMA_RMP::Remapped
    }
}
#[doc = "Field `TIM6_DAC1_CH1_DMA_RMP` writer - TIM6 and DAC1 DMA request remapping bit"]
pub type TIM6_DAC1_CH1_DMA_RMP_W<'a, REG> = crate::BitWriter<'a, REG, TIM6_DAC1_CH1_DMA_RMP>;
impl<'a, REG> TIM6_DAC1_CH1_DMA_RMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TIM6_UP and DAC_CH1 DMA requests mapped on DMA2 channel 3 in STM32F303xB/C and STM32F358xC"]
    #[inline(always)]
    pub fn not_remapped(self) -> &'a mut crate::W<REG> {
        self.variant(TIM6_DAC1_CH1_DMA_RMP::NotRemapped)
    }
    #[doc = "TIM6_UP and DAC_CH1 DMA requests mapped on DMA1 channel 3"]
    #[inline(always)]
    pub fn remapped(self) -> &'a mut crate::W<REG> {
        self.variant(TIM6_DAC1_CH1_DMA_RMP::Remapped)
    }
}
#[doc = "TIM7 and DAC2 DMA request remapping bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM7_DAC1_CH2_DMA_RMP {
    #[doc = "0: TIM7_UP and DAC_CH2 DMA requests mapped on DMA2 channel 4 in STM32F303xB/C and STM32F358xC devices"]
    NotRemapped = 0,
    #[doc = "1: TIM7_UP and DAC_CH2 DMA requests mapped on DMA1 channel 4"]
    Remapped = 1,
}
impl From<TIM7_DAC1_CH2_DMA_RMP> for bool {
    #[inline(always)]
    fn from(variant: TIM7_DAC1_CH2_DMA_RMP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM7_DAC1_CH2_DMA_RMP` reader - TIM7 and DAC2 DMA request remapping bit"]
pub type TIM7_DAC1_CH2_DMA_RMP_R = crate::BitReader<TIM7_DAC1_CH2_DMA_RMP>;
impl TIM7_DAC1_CH2_DMA_RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIM7_DAC1_CH2_DMA_RMP {
        match self.bits {
            false => TIM7_DAC1_CH2_DMA_RMP::NotRemapped,
            true => TIM7_DAC1_CH2_DMA_RMP::Remapped,
        }
    }
    #[doc = "TIM7_UP and DAC_CH2 DMA requests mapped on DMA2 channel 4 in STM32F303xB/C and STM32F358xC devices"]
    #[inline(always)]
    pub fn is_not_remapped(&self) -> bool {
        *self == TIM7_DAC1_CH2_DMA_RMP::NotRemapped
    }
    #[doc = "TIM7_UP and DAC_CH2 DMA requests mapped on DMA1 channel 4"]
    #[inline(always)]
    pub fn is_remapped(&self) -> bool {
        *self == TIM7_DAC1_CH2_DMA_RMP::Remapped
    }
}
#[doc = "Field `TIM7_DAC1_CH2_DMA_RMP` writer - TIM7 and DAC2 DMA request remapping bit"]
pub type TIM7_DAC1_CH2_DMA_RMP_W<'a, REG> = crate::BitWriter<'a, REG, TIM7_DAC1_CH2_DMA_RMP>;
impl<'a, REG> TIM7_DAC1_CH2_DMA_RMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TIM7_UP and DAC_CH2 DMA requests mapped on DMA2 channel 4 in STM32F303xB/C and STM32F358xC devices"]
    #[inline(always)]
    pub fn not_remapped(self) -> &'a mut crate::W<REG> {
        self.variant(TIM7_DAC1_CH2_DMA_RMP::NotRemapped)
    }
    #[doc = "TIM7_UP and DAC_CH2 DMA requests mapped on DMA1 channel 4"]
    #[inline(always)]
    pub fn remapped(self) -> &'a mut crate::W<REG> {
        self.variant(TIM7_DAC1_CH2_DMA_RMP::Remapped)
    }
}
#[doc = "DAC2 channel1 DMA remap\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAC2_CH1_DMA_RMP {
    #[doc = "0: Not remapped"]
    NotRemapped = 0,
    #[doc = "1: DAC2_CH1 DMA requests mapped on DMA1 channel 5"]
    Remapped = 1,
}
impl From<DAC2_CH1_DMA_RMP> for bool {
    #[inline(always)]
    fn from(variant: DAC2_CH1_DMA_RMP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DAC2_CH1_DMA_RMP` reader - DAC2 channel1 DMA remap"]
pub type DAC2_CH1_DMA_RMP_R = crate::BitReader<DAC2_CH1_DMA_RMP>;
impl DAC2_CH1_DMA_RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DAC2_CH1_DMA_RMP {
        match self.bits {
            false => DAC2_CH1_DMA_RMP::NotRemapped,
            true => DAC2_CH1_DMA_RMP::Remapped,
        }
    }
    #[doc = "Not remapped"]
    #[inline(always)]
    pub fn is_not_remapped(&self) -> bool {
        *self == DAC2_CH1_DMA_RMP::NotRemapped
    }
    #[doc = "DAC2_CH1 DMA requests mapped on DMA1 channel 5"]
    #[inline(always)]
    pub fn is_remapped(&self) -> bool {
        *self == DAC2_CH1_DMA_RMP::Remapped
    }
}
#[doc = "Field `DAC2_CH1_DMA_RMP` writer - DAC2 channel1 DMA remap"]
pub type DAC2_CH1_DMA_RMP_W<'a, REG> = crate::BitWriter<'a, REG, DAC2_CH1_DMA_RMP>;
impl<'a, REG> DAC2_CH1_DMA_RMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not remapped"]
    #[inline(always)]
    pub fn not_remapped(self) -> &'a mut crate::W<REG> {
        self.variant(DAC2_CH1_DMA_RMP::NotRemapped)
    }
    #[doc = "DAC2_CH1 DMA requests mapped on DMA1 channel 5"]
    #[inline(always)]
    pub fn remapped(self) -> &'a mut crate::W<REG> {
        self.variant(DAC2_CH1_DMA_RMP::Remapped)
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
#[doc = "Encoder mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ENCODER_MODE {
    #[doc = "0: No redirection"]
    NoRedirection = 0,
    #[doc = "1: TIM2 IC1 and TIM2 IC2 are connected to TIM15 IC1 and TIM15 IC2 respectively"]
    MapTim2tim15 = 1,
    #[doc = "2: TIM3 IC1 and TIM3 IC2 are connected to TIM15 IC1 and TIM15 IC2 respectively"]
    MapTim3tim15 = 2,
    #[doc = "3: TIM4 IC1 and TIM4 IC2 are connected to TIM15 IC1 and TIM15 IC2 respectively (STM32F303xB/C and STM32F358xC devices only)"]
    MapTim4tim15 = 3,
}
impl From<ENCODER_MODE> for u8 {
    #[inline(always)]
    fn from(variant: ENCODER_MODE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ENCODER_MODE {
    type Ux = u8;
}
#[doc = "Field `ENCODER_MODE` reader - Encoder mode"]
pub type ENCODER_MODE_R = crate::FieldReader<ENCODER_MODE>;
impl ENCODER_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ENCODER_MODE {
        match self.bits {
            0 => ENCODER_MODE::NoRedirection,
            1 => ENCODER_MODE::MapTim2tim15,
            2 => ENCODER_MODE::MapTim3tim15,
            3 => ENCODER_MODE::MapTim4tim15,
            _ => unreachable!(),
        }
    }
    #[doc = "No redirection"]
    #[inline(always)]
    pub fn is_no_redirection(&self) -> bool {
        *self == ENCODER_MODE::NoRedirection
    }
    #[doc = "TIM2 IC1 and TIM2 IC2 are connected to TIM15 IC1 and TIM15 IC2 respectively"]
    #[inline(always)]
    pub fn is_map_tim2tim15(&self) -> bool {
        *self == ENCODER_MODE::MapTim2tim15
    }
    #[doc = "TIM3 IC1 and TIM3 IC2 are connected to TIM15 IC1 and TIM15 IC2 respectively"]
    #[inline(always)]
    pub fn is_map_tim3tim15(&self) -> bool {
        *self == ENCODER_MODE::MapTim3tim15
    }
    #[doc = "TIM4 IC1 and TIM4 IC2 are connected to TIM15 IC1 and TIM15 IC2 respectively (STM32F303xB/C and STM32F358xC devices only)"]
    #[inline(always)]
    pub fn is_map_tim4tim15(&self) -> bool {
        *self == ENCODER_MODE::MapTim4tim15
    }
}
#[doc = "Field `ENCODER_MODE` writer - Encoder mode"]
pub type ENCODER_MODE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, ENCODER_MODE>;
impl<'a, REG> ENCODER_MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No redirection"]
    #[inline(always)]
    pub fn no_redirection(self) -> &'a mut crate::W<REG> {
        self.variant(ENCODER_MODE::NoRedirection)
    }
    #[doc = "TIM2 IC1 and TIM2 IC2 are connected to TIM15 IC1 and TIM15 IC2 respectively"]
    #[inline(always)]
    pub fn map_tim2tim15(self) -> &'a mut crate::W<REG> {
        self.variant(ENCODER_MODE::MapTim2tim15)
    }
    #[doc = "TIM3 IC1 and TIM3 IC2 are connected to TIM15 IC1 and TIM15 IC2 respectively"]
    #[inline(always)]
    pub fn map_tim3tim15(self) -> &'a mut crate::W<REG> {
        self.variant(ENCODER_MODE::MapTim3tim15)
    }
    #[doc = "TIM4 IC1 and TIM4 IC2 are connected to TIM15 IC1 and TIM15 IC2 respectively (STM32F303xB/C and STM32F358xC devices only)"]
    #[inline(always)]
    pub fn map_tim4tim15(self) -> &'a mut crate::W<REG> {
        self.variant(ENCODER_MODE::MapTim4tim15)
    }
}
#[doc = "I2C3 Fast Mode Plus\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C3_FMP {
    #[doc = "0: FM+ mode is controlled by I2C_Pxx_FMP bits only"]
    Standard = 0,
    #[doc = "1: FM+ mode is enabled on all I2C3 pins selected through selection through IOPORT control registers AF selection bits"]
    Fmp = 1,
}
impl From<I2C3_FMP> for bool {
    #[inline(always)]
    fn from(variant: I2C3_FMP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C3_FMP` reader - I2C3 Fast Mode Plus"]
pub type I2C3_FMP_R = crate::BitReader<I2C3_FMP>;
impl I2C3_FMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2C3_FMP {
        match self.bits {
            false => I2C3_FMP::Standard,
            true => I2C3_FMP::Fmp,
        }
    }
    #[doc = "FM+ mode is controlled by I2C_Pxx_FMP bits only"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == I2C3_FMP::Standard
    }
    #[doc = "FM+ mode is enabled on all I2C3 pins selected through selection through IOPORT control registers AF selection bits"]
    #[inline(always)]
    pub fn is_fmp(&self) -> bool {
        *self == I2C3_FMP::Fmp
    }
}
#[doc = "Field `I2C3_FMP` writer - I2C3 Fast Mode Plus"]
pub type I2C3_FMP_W<'a, REG> = crate::BitWriter<'a, REG, I2C3_FMP>;
impl<'a, REG> I2C3_FMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FM+ mode is controlled by I2C_Pxx_FMP bits only"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(I2C3_FMP::Standard)
    }
    #[doc = "FM+ mode is enabled on all I2C3 pins selected through selection through IOPORT control registers AF selection bits"]
    #[inline(always)]
    pub fn fmp(self) -> &'a mut crate::W<REG> {
        self.variant(I2C3_FMP::Fmp)
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
    #[doc = "Bits 0:2 - Memory mapping selection bits"]
    #[inline(always)]
    pub fn mem_mode(&self) -> MEM_MODE_R {
        MEM_MODE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 5 - USB interrupt remap"]
    #[inline(always)]
    pub fn usb_it_rmp(&self) -> USB_IT_RMP_R {
        USB_IT_RMP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Timer 1 ITR3 selection"]
    #[inline(always)]
    pub fn tim1_itr3_rmp(&self) -> TIM1_ITR3_RMP_R {
        TIM1_ITR3_RMP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DAC trigger remap (when TSEL = 001)"]
    #[inline(always)]
    pub fn dac1_trig_rmp(&self) -> DAC1_TRIG_RMP_R {
        DAC1_TRIG_RMP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ADC24 DMA remapping bit"]
    #[inline(always)]
    pub fn adc2_dma_rmp(&self) -> ADC2_DMA_RMP_R {
        ADC2_DMA_RMP_R::new(((self.bits >> 8) & 1) != 0)
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
    pub fn tim6_dac1_ch1_dma_rmp(&self) -> TIM6_DAC1_CH1_DMA_RMP_R {
        TIM6_DAC1_CH1_DMA_RMP_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - TIM7 and DAC2 DMA request remapping bit"]
    #[inline(always)]
    pub fn tim7_dac1_ch2_dma_rmp(&self) -> TIM7_DAC1_CH2_DMA_RMP_R {
        TIM7_DAC1_CH2_DMA_RMP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - DAC2 channel1 DMA remap"]
    #[inline(always)]
    pub fn dac2_ch1_dma_rmp(&self) -> DAC2_CH1_DMA_RMP_R {
        DAC2_CH1_DMA_RMP_R::new(((self.bits >> 15) & 1) != 0)
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
    #[doc = "Bits 22:23 - Encoder mode"]
    #[inline(always)]
    pub fn encoder_mode(&self) -> ENCODER_MODE_R {
        ENCODER_MODE_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24 - I2C3 Fast Mode Plus"]
    #[inline(always)]
    pub fn i2c3_fmp(&self) -> I2C3_FMP_R {
        I2C3_FMP_R::new(((self.bits >> 24) & 1) != 0)
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
    #[doc = "Bits 0:2 - Memory mapping selection bits"]
    #[inline(always)]
    #[must_use]
    pub fn mem_mode(&mut self) -> MEM_MODE_W<CFGR1rs> {
        MEM_MODE_W::new(self, 0)
    }
    #[doc = "Bit 5 - USB interrupt remap"]
    #[inline(always)]
    #[must_use]
    pub fn usb_it_rmp(&mut self) -> USB_IT_RMP_W<CFGR1rs> {
        USB_IT_RMP_W::new(self, 5)
    }
    #[doc = "Bit 6 - Timer 1 ITR3 selection"]
    #[inline(always)]
    #[must_use]
    pub fn tim1_itr3_rmp(&mut self) -> TIM1_ITR3_RMP_W<CFGR1rs> {
        TIM1_ITR3_RMP_W::new(self, 6)
    }
    #[doc = "Bit 7 - DAC trigger remap (when TSEL = 001)"]
    #[inline(always)]
    #[must_use]
    pub fn dac1_trig_rmp(&mut self) -> DAC1_TRIG_RMP_W<CFGR1rs> {
        DAC1_TRIG_RMP_W::new(self, 7)
    }
    #[doc = "Bit 8 - ADC24 DMA remapping bit"]
    #[inline(always)]
    #[must_use]
    pub fn adc2_dma_rmp(&mut self) -> ADC2_DMA_RMP_W<CFGR1rs> {
        ADC2_DMA_RMP_W::new(self, 8)
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
    pub fn tim6_dac1_ch1_dma_rmp(&mut self) -> TIM6_DAC1_CH1_DMA_RMP_W<CFGR1rs> {
        TIM6_DAC1_CH1_DMA_RMP_W::new(self, 13)
    }
    #[doc = "Bit 14 - TIM7 and DAC2 DMA request remapping bit"]
    #[inline(always)]
    #[must_use]
    pub fn tim7_dac1_ch2_dma_rmp(&mut self) -> TIM7_DAC1_CH2_DMA_RMP_W<CFGR1rs> {
        TIM7_DAC1_CH2_DMA_RMP_W::new(self, 14)
    }
    #[doc = "Bit 15 - DAC2 channel1 DMA remap"]
    #[inline(always)]
    #[must_use]
    pub fn dac2_ch1_dma_rmp(&mut self) -> DAC2_CH1_DMA_RMP_W<CFGR1rs> {
        DAC2_CH1_DMA_RMP_W::new(self, 15)
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
    #[doc = "Bits 22:23 - Encoder mode"]
    #[inline(always)]
    #[must_use]
    pub fn encoder_mode(&mut self) -> ENCODER_MODE_W<CFGR1rs> {
        ENCODER_MODE_W::new(self, 22)
    }
    #[doc = "Bit 24 - I2C3 Fast Mode Plus"]
    #[inline(always)]
    #[must_use]
    pub fn i2c3_fmp(&mut self) -> I2C3_FMP_W<CFGR1rs> {
        I2C3_FMP_W::new(self, 24)
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
