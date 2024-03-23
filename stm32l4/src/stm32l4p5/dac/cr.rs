#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "DAC channel1 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN1 {
    #[doc = "0: DAC Channel X disabled"]
    Disabled = 0,
    #[doc = "1: DAC Channel X enabled"]
    Enabled = 1,
}
impl From<EN1> for bool {
    #[inline(always)]
    fn from(variant: EN1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN1` reader - DAC channel1 enable"]
pub type EN1_R = crate::BitReader<EN1>;
impl EN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EN1 {
        match self.bits {
            false => EN1::Disabled,
            true => EN1::Enabled,
        }
    }
    #[doc = "DAC Channel X disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EN1::Disabled
    }
    #[doc = "DAC Channel X enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EN1::Enabled
    }
}
#[doc = "Field `EN1` writer - DAC channel1 enable"]
pub type EN1_W<'a, REG> = crate::BitWriter<'a, REG, EN1>;
impl<'a, REG> EN1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DAC Channel X disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EN1::Disabled)
    }
    #[doc = "DAC Channel X enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EN1::Enabled)
    }
}
#[doc = "DAC channel1 trigger enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEN1 {
    #[doc = "0: DAC Channel X trigger disabled"]
    Disabled = 0,
    #[doc = "1: DAC Channel X trigger enabled"]
    Enabled = 1,
}
impl From<TEN1> for bool {
    #[inline(always)]
    fn from(variant: TEN1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEN1` reader - DAC channel1 trigger enable"]
pub type TEN1_R = crate::BitReader<TEN1>;
impl TEN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TEN1 {
        match self.bits {
            false => TEN1::Disabled,
            true => TEN1::Enabled,
        }
    }
    #[doc = "DAC Channel X trigger disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TEN1::Disabled
    }
    #[doc = "DAC Channel X trigger enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TEN1::Enabled
    }
}
#[doc = "Field `TEN1` writer - DAC channel1 trigger enable"]
pub type TEN1_W<'a, REG> = crate::BitWriter<'a, REG, TEN1>;
impl<'a, REG> TEN1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DAC Channel X trigger disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TEN1::Disabled)
    }
    #[doc = "DAC Channel X trigger enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TEN1::Enabled)
    }
}
#[doc = "DAC channel1 trigger selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TSEL1 {
    #[doc = "0: TIM6_TRGO event trigger for DAC conversion, if TEN is enabled"]
    Tim6Trgo = 0,
    #[doc = "1: TIM8_TRGO"]
    Tim8Trgo = 1,
    #[doc = "2: TIM7_TRGO (Note: Reserved on STM32L45xxx and STM32L46xxx devices)"]
    Tim7Trgo = 2,
    #[doc = "3: TIM5_TRGO"]
    Tim5Trgo = 3,
    #[doc = "4: TIM2_TRGO"]
    Tim2Trgo = 4,
    #[doc = "5: TIM4_TRGO"]
    Tim4Trgo = 5,
    #[doc = "6: External pin"]
    Exti9 = 6,
    #[doc = "7: Software triger"]
    Swtrig = 7,
}
impl From<TSEL1> for u8 {
    #[inline(always)]
    fn from(variant: TSEL1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TSEL1 {
    type Ux = u8;
}
#[doc = "Field `TSEL1` reader - DAC channel1 trigger selection"]
pub type TSEL1_R = crate::FieldReader<TSEL1>;
impl TSEL1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TSEL1> {
        match self.bits {
            0 => Some(TSEL1::Tim6Trgo),
            1 => Some(TSEL1::Tim8Trgo),
            2 => Some(TSEL1::Tim7Trgo),
            3 => Some(TSEL1::Tim5Trgo),
            4 => Some(TSEL1::Tim2Trgo),
            5 => Some(TSEL1::Tim4Trgo),
            6 => Some(TSEL1::Exti9),
            7 => Some(TSEL1::Swtrig),
            _ => None,
        }
    }
    #[doc = "TIM6_TRGO event trigger for DAC conversion, if TEN is enabled"]
    #[inline(always)]
    pub fn is_tim6_trgo(&self) -> bool {
        *self == TSEL1::Tim6Trgo
    }
    #[doc = "TIM8_TRGO"]
    #[inline(always)]
    pub fn is_tim8_trgo(&self) -> bool {
        *self == TSEL1::Tim8Trgo
    }
    #[doc = "TIM7_TRGO (Note: Reserved on STM32L45xxx and STM32L46xxx devices)"]
    #[inline(always)]
    pub fn is_tim7_trgo(&self) -> bool {
        *self == TSEL1::Tim7Trgo
    }
    #[doc = "TIM5_TRGO"]
    #[inline(always)]
    pub fn is_tim5_trgo(&self) -> bool {
        *self == TSEL1::Tim5Trgo
    }
    #[doc = "TIM2_TRGO"]
    #[inline(always)]
    pub fn is_tim2_trgo(&self) -> bool {
        *self == TSEL1::Tim2Trgo
    }
    #[doc = "TIM4_TRGO"]
    #[inline(always)]
    pub fn is_tim4_trgo(&self) -> bool {
        *self == TSEL1::Tim4Trgo
    }
    #[doc = "External pin"]
    #[inline(always)]
    pub fn is_exti9(&self) -> bool {
        *self == TSEL1::Exti9
    }
    #[doc = "Software triger"]
    #[inline(always)]
    pub fn is_swtrig(&self) -> bool {
        *self == TSEL1::Swtrig
    }
}
#[doc = "Field `TSEL1` writer - DAC channel1 trigger selection"]
pub type TSEL1_W<'a, REG> = crate::FieldWriter<'a, REG, 4, TSEL1>;
impl<'a, REG> TSEL1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TIM6_TRGO event trigger for DAC conversion, if TEN is enabled"]
    #[inline(always)]
    pub fn tim6_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL1::Tim6Trgo)
    }
    #[doc = "TIM8_TRGO"]
    #[inline(always)]
    pub fn tim8_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL1::Tim8Trgo)
    }
    #[doc = "TIM7_TRGO (Note: Reserved on STM32L45xxx and STM32L46xxx devices)"]
    #[inline(always)]
    pub fn tim7_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL1::Tim7Trgo)
    }
    #[doc = "TIM5_TRGO"]
    #[inline(always)]
    pub fn tim5_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL1::Tim5Trgo)
    }
    #[doc = "TIM2_TRGO"]
    #[inline(always)]
    pub fn tim2_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL1::Tim2Trgo)
    }
    #[doc = "TIM4_TRGO"]
    #[inline(always)]
    pub fn tim4_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL1::Tim4Trgo)
    }
    #[doc = "External pin"]
    #[inline(always)]
    pub fn exti9(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL1::Exti9)
    }
    #[doc = "Software triger"]
    #[inline(always)]
    pub fn swtrig(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL1::Swtrig)
    }
}
#[doc = "DAC channel1 noise/triangle wave generation enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WAVE1 {
    #[doc = "0: Wave generation disabled"]
    Disabled = 0,
    #[doc = "1: Noise wave generation enabled"]
    Noise = 1,
    #[doc = "2: Triangle wave generation enabled"]
    Triangle = 2,
}
impl From<WAVE1> for u8 {
    #[inline(always)]
    fn from(variant: WAVE1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WAVE1 {
    type Ux = u8;
}
#[doc = "Field `WAVE1` reader - DAC channel1 noise/triangle wave generation enable"]
pub type WAVE1_R = crate::FieldReader<WAVE1>;
impl WAVE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<WAVE1> {
        match self.bits {
            0 => Some(WAVE1::Disabled),
            1 => Some(WAVE1::Noise),
            2 => Some(WAVE1::Triangle),
            _ => None,
        }
    }
    #[doc = "Wave generation disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WAVE1::Disabled
    }
    #[doc = "Noise wave generation enabled"]
    #[inline(always)]
    pub fn is_noise(&self) -> bool {
        *self == WAVE1::Noise
    }
    #[doc = "Triangle wave generation enabled"]
    #[inline(always)]
    pub fn is_triangle(&self) -> bool {
        *self == WAVE1::Triangle
    }
}
#[doc = "Field `WAVE1` writer - DAC channel1 noise/triangle wave generation enable"]
pub type WAVE1_W<'a, REG> = crate::FieldWriter<'a, REG, 2, WAVE1>;
impl<'a, REG> WAVE1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Wave generation disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(WAVE1::Disabled)
    }
    #[doc = "Noise wave generation enabled"]
    #[inline(always)]
    pub fn noise(self) -> &'a mut crate::W<REG> {
        self.variant(WAVE1::Noise)
    }
    #[doc = "Triangle wave generation enabled"]
    #[inline(always)]
    pub fn triangle(self) -> &'a mut crate::W<REG> {
        self.variant(WAVE1::Triangle)
    }
}
#[doc = "DAC channel1 mask/amplitude selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MAMP1 {
    #[doc = "0: Unmask bit0 of LFSR/ triangle amplitude equal to 1"]
    Amp1 = 0,
    #[doc = "1: Unmask bits\\[1:0\\]
of LFSR/ triangle amplitude equal to 3"]
    Amp3 = 1,
    #[doc = "2: Unmask bits\\[2:0\\]
of LFSR/ triangle amplitude equal to 7"]
    Amp7 = 2,
    #[doc = "3: Unmask bits\\[3:0\\]
of LFSR/ triangle amplitude equal to 15"]
    Amp15 = 3,
    #[doc = "4: Unmask bits\\[4:0\\]
of LFSR/ triangle amplitude equal to 31"]
    Amp31 = 4,
    #[doc = "5: Unmask bits\\[5:0\\]
of LFSR/ triangle amplitude equal 63"]
    Amp63 = 5,
    #[doc = "6: Unmask bits\\[6:0\\]
of LFSR/ triangle amplitude equal to 127"]
    Amp127 = 6,
    #[doc = "7: Unmask bits\\[7:0\\]
of LFSR/ triangle amplitude equal to 255"]
    Amp255 = 7,
    #[doc = "8: Unmask bits\\[8:0\\]
of LFSR/ triangle amplitude equal to 511"]
    Amp511 = 8,
    #[doc = "9: Unmask bits\\[9:0\\]
of LFSR/ triangle amplitude equal to 1023"]
    Amp1023 = 9,
    #[doc = "10: Unmask bits\\[10:0\\]
of LFSR/ triangle amplitude equal to 2047"]
    Amp2047 = 10,
    #[doc = "11: Unmask bits\\[11:0\\]
of LFSR/ triangle amplitude equal to 4095"]
    Amp4095 = 11,
}
impl From<MAMP1> for u8 {
    #[inline(always)]
    fn from(variant: MAMP1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MAMP1 {
    type Ux = u8;
}
#[doc = "Field `MAMP1` reader - DAC channel1 mask/amplitude selector"]
pub type MAMP1_R = crate::FieldReader<MAMP1>;
impl MAMP1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MAMP1> {
        match self.bits {
            0 => Some(MAMP1::Amp1),
            1 => Some(MAMP1::Amp3),
            2 => Some(MAMP1::Amp7),
            3 => Some(MAMP1::Amp15),
            4 => Some(MAMP1::Amp31),
            5 => Some(MAMP1::Amp63),
            6 => Some(MAMP1::Amp127),
            7 => Some(MAMP1::Amp255),
            8 => Some(MAMP1::Amp511),
            9 => Some(MAMP1::Amp1023),
            10 => Some(MAMP1::Amp2047),
            11 => Some(MAMP1::Amp4095),
            _ => None,
        }
    }
    #[doc = "Unmask bit0 of LFSR/ triangle amplitude equal to 1"]
    #[inline(always)]
    pub fn is_amp1(&self) -> bool {
        *self == MAMP1::Amp1
    }
    #[doc = "Unmask bits\\[1:0\\]
of LFSR/ triangle amplitude equal to 3"]
    #[inline(always)]
    pub fn is_amp3(&self) -> bool {
        *self == MAMP1::Amp3
    }
    #[doc = "Unmask bits\\[2:0\\]
of LFSR/ triangle amplitude equal to 7"]
    #[inline(always)]
    pub fn is_amp7(&self) -> bool {
        *self == MAMP1::Amp7
    }
    #[doc = "Unmask bits\\[3:0\\]
of LFSR/ triangle amplitude equal to 15"]
    #[inline(always)]
    pub fn is_amp15(&self) -> bool {
        *self == MAMP1::Amp15
    }
    #[doc = "Unmask bits\\[4:0\\]
of LFSR/ triangle amplitude equal to 31"]
    #[inline(always)]
    pub fn is_amp31(&self) -> bool {
        *self == MAMP1::Amp31
    }
    #[doc = "Unmask bits\\[5:0\\]
of LFSR/ triangle amplitude equal 63"]
    #[inline(always)]
    pub fn is_amp63(&self) -> bool {
        *self == MAMP1::Amp63
    }
    #[doc = "Unmask bits\\[6:0\\]
of LFSR/ triangle amplitude equal to 127"]
    #[inline(always)]
    pub fn is_amp127(&self) -> bool {
        *self == MAMP1::Amp127
    }
    #[doc = "Unmask bits\\[7:0\\]
of LFSR/ triangle amplitude equal to 255"]
    #[inline(always)]
    pub fn is_amp255(&self) -> bool {
        *self == MAMP1::Amp255
    }
    #[doc = "Unmask bits\\[8:0\\]
of LFSR/ triangle amplitude equal to 511"]
    #[inline(always)]
    pub fn is_amp511(&self) -> bool {
        *self == MAMP1::Amp511
    }
    #[doc = "Unmask bits\\[9:0\\]
of LFSR/ triangle amplitude equal to 1023"]
    #[inline(always)]
    pub fn is_amp1023(&self) -> bool {
        *self == MAMP1::Amp1023
    }
    #[doc = "Unmask bits\\[10:0\\]
of LFSR/ triangle amplitude equal to 2047"]
    #[inline(always)]
    pub fn is_amp2047(&self) -> bool {
        *self == MAMP1::Amp2047
    }
    #[doc = "Unmask bits\\[11:0\\]
of LFSR/ triangle amplitude equal to 4095"]
    #[inline(always)]
    pub fn is_amp4095(&self) -> bool {
        *self == MAMP1::Amp4095
    }
}
#[doc = "Field `MAMP1` writer - DAC channel1 mask/amplitude selector"]
pub type MAMP1_W<'a, REG> = crate::FieldWriter<'a, REG, 4, MAMP1>;
impl<'a, REG> MAMP1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Unmask bit0 of LFSR/ triangle amplitude equal to 1"]
    #[inline(always)]
    pub fn amp1(self) -> &'a mut crate::W<REG> {
        self.variant(MAMP1::Amp1)
    }
    #[doc = "Unmask bits\\[1:0\\]
of LFSR/ triangle amplitude equal to 3"]
    #[inline(always)]
    pub fn amp3(self) -> &'a mut crate::W<REG> {
        self.variant(MAMP1::Amp3)
    }
    #[doc = "Unmask bits\\[2:0\\]
of LFSR/ triangle amplitude equal to 7"]
    #[inline(always)]
    pub fn amp7(self) -> &'a mut crate::W<REG> {
        self.variant(MAMP1::Amp7)
    }
    #[doc = "Unmask bits\\[3:0\\]
of LFSR/ triangle amplitude equal to 15"]
    #[inline(always)]
    pub fn amp15(self) -> &'a mut crate::W<REG> {
        self.variant(MAMP1::Amp15)
    }
    #[doc = "Unmask bits\\[4:0\\]
of LFSR/ triangle amplitude equal to 31"]
    #[inline(always)]
    pub fn amp31(self) -> &'a mut crate::W<REG> {
        self.variant(MAMP1::Amp31)
    }
    #[doc = "Unmask bits\\[5:0\\]
of LFSR/ triangle amplitude equal 63"]
    #[inline(always)]
    pub fn amp63(self) -> &'a mut crate::W<REG> {
        self.variant(MAMP1::Amp63)
    }
    #[doc = "Unmask bits\\[6:0\\]
of LFSR/ triangle amplitude equal to 127"]
    #[inline(always)]
    pub fn amp127(self) -> &'a mut crate::W<REG> {
        self.variant(MAMP1::Amp127)
    }
    #[doc = "Unmask bits\\[7:0\\]
of LFSR/ triangle amplitude equal to 255"]
    #[inline(always)]
    pub fn amp255(self) -> &'a mut crate::W<REG> {
        self.variant(MAMP1::Amp255)
    }
    #[doc = "Unmask bits\\[8:0\\]
of LFSR/ triangle amplitude equal to 511"]
    #[inline(always)]
    pub fn amp511(self) -> &'a mut crate::W<REG> {
        self.variant(MAMP1::Amp511)
    }
    #[doc = "Unmask bits\\[9:0\\]
of LFSR/ triangle amplitude equal to 1023"]
    #[inline(always)]
    pub fn amp1023(self) -> &'a mut crate::W<REG> {
        self.variant(MAMP1::Amp1023)
    }
    #[doc = "Unmask bits\\[10:0\\]
of LFSR/ triangle amplitude equal to 2047"]
    #[inline(always)]
    pub fn amp2047(self) -> &'a mut crate::W<REG> {
        self.variant(MAMP1::Amp2047)
    }
    #[doc = "Unmask bits\\[11:0\\]
of LFSR/ triangle amplitude equal to 4095"]
    #[inline(always)]
    pub fn amp4095(self) -> &'a mut crate::W<REG> {
        self.variant(MAMP1::Amp4095)
    }
}
#[doc = "DAC channel1 DMA enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAEN1 {
    #[doc = "0: DAC Channel X DMA mode disabled"]
    Disabled = 0,
    #[doc = "1: DAC Channel X DMA mode enabled"]
    Enabled = 1,
}
impl From<DMAEN1> for bool {
    #[inline(always)]
    fn from(variant: DMAEN1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAEN1` reader - DAC channel1 DMA enable"]
pub type DMAEN1_R = crate::BitReader<DMAEN1>;
impl DMAEN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMAEN1 {
        match self.bits {
            false => DMAEN1::Disabled,
            true => DMAEN1::Enabled,
        }
    }
    #[doc = "DAC Channel X DMA mode disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAEN1::Disabled
    }
    #[doc = "DAC Channel X DMA mode enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAEN1::Enabled
    }
}
#[doc = "Field `DMAEN1` writer - DAC channel1 DMA enable"]
pub type DMAEN1_W<'a, REG> = crate::BitWriter<'a, REG, DMAEN1>;
impl<'a, REG> DMAEN1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DAC Channel X DMA mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAEN1::Disabled)
    }
    #[doc = "DAC Channel X DMA mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAEN1::Enabled)
    }
}
#[doc = "DAC channel1 DMA Underrun Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAUDRIE1 {
    #[doc = "0: DAC Channel X DMA Underrun Interrupt disabled"]
    Disabled = 0,
    #[doc = "1: DAC Channel X DMA Underrun Interrupt enabled"]
    Enabled = 1,
}
impl From<DMAUDRIE1> for bool {
    #[inline(always)]
    fn from(variant: DMAUDRIE1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAUDRIE1` reader - DAC channel1 DMA Underrun Interrupt enable"]
pub type DMAUDRIE1_R = crate::BitReader<DMAUDRIE1>;
impl DMAUDRIE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMAUDRIE1 {
        match self.bits {
            false => DMAUDRIE1::Disabled,
            true => DMAUDRIE1::Enabled,
        }
    }
    #[doc = "DAC Channel X DMA Underrun Interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAUDRIE1::Disabled
    }
    #[doc = "DAC Channel X DMA Underrun Interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAUDRIE1::Enabled
    }
}
#[doc = "Field `DMAUDRIE1` writer - DAC channel1 DMA Underrun Interrupt enable"]
pub type DMAUDRIE1_W<'a, REG> = crate::BitWriter<'a, REG, DMAUDRIE1>;
impl<'a, REG> DMAUDRIE1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DAC Channel X DMA Underrun Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAUDRIE1::Disabled)
    }
    #[doc = "DAC Channel X DMA Underrun Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAUDRIE1::Enabled)
    }
}
#[doc = "DAC Channel 1 calibration enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CEN1 {
    #[doc = "0: DAC Channel X Normal operating mode"]
    Normal = 0,
    #[doc = "1: DAC Channel X calibration mode"]
    Calibration = 1,
}
impl From<CEN1> for bool {
    #[inline(always)]
    fn from(variant: CEN1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEN1` reader - DAC Channel 1 calibration enable"]
pub type CEN1_R = crate::BitReader<CEN1>;
impl CEN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CEN1 {
        match self.bits {
            false => CEN1::Normal,
            true => CEN1::Calibration,
        }
    }
    #[doc = "DAC Channel X Normal operating mode"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == CEN1::Normal
    }
    #[doc = "DAC Channel X calibration mode"]
    #[inline(always)]
    pub fn is_calibration(&self) -> bool {
        *self == CEN1::Calibration
    }
}
#[doc = "Field `CEN1` writer - DAC Channel 1 calibration enable"]
pub type CEN1_W<'a, REG> = crate::BitWriter<'a, REG, CEN1>;
impl<'a, REG> CEN1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DAC Channel X Normal operating mode"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(CEN1::Normal)
    }
    #[doc = "DAC Channel X calibration mode"]
    #[inline(always)]
    pub fn calibration(self) -> &'a mut crate::W<REG> {
        self.variant(CEN1::Calibration)
    }
}
#[doc = "High frequency interface mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HFSEL {
    #[doc = "0: High frequency interface mode disabled"]
    Disabled = 0,
    #[doc = "1: High frequency interface mode enabled"]
    Enabled = 1,
}
impl From<HFSEL> for bool {
    #[inline(always)]
    fn from(variant: HFSEL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HFSEL` reader - High frequency interface mode enable"]
pub type HFSEL_R = crate::BitReader<HFSEL>;
impl HFSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HFSEL {
        match self.bits {
            false => HFSEL::Disabled,
            true => HFSEL::Enabled,
        }
    }
    #[doc = "High frequency interface mode disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HFSEL::Disabled
    }
    #[doc = "High frequency interface mode enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HFSEL::Enabled
    }
}
#[doc = "Field `HFSEL` writer - High frequency interface mode enable"]
pub type HFSEL_W<'a, REG> = crate::BitWriter<'a, REG, HFSEL>;
impl<'a, REG> HFSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "High frequency interface mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(HFSEL::Disabled)
    }
    #[doc = "High frequency interface mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(HFSEL::Enabled)
    }
}
#[doc = "Field `CEN2` reader - DAC Channel 2 calibration enable"]
pub use CEN1_R as CEN2_R;
#[doc = "Field `CEN2` writer - DAC Channel 2 calibration enable"]
pub use CEN1_W as CEN2_W;
#[doc = "Field `DMAEN2` reader - DAC channel2 DMA enable"]
pub use DMAEN1_R as DMAEN2_R;
#[doc = "Field `DMAEN2` writer - DAC channel2 DMA enable"]
pub use DMAEN1_W as DMAEN2_W;
#[doc = "Field `DMAUDRIE2` reader - DAC channel2 DMA underrun interrupt enable"]
pub use DMAUDRIE1_R as DMAUDRIE2_R;
#[doc = "Field `DMAUDRIE2` writer - DAC channel2 DMA underrun interrupt enable"]
pub use DMAUDRIE1_W as DMAUDRIE2_W;
#[doc = "Field `EN2` reader - DAC channel2 enable"]
pub use EN1_R as EN2_R;
#[doc = "Field `EN2` writer - DAC channel2 enable"]
pub use EN1_W as EN2_W;
#[doc = "Field `MAMP2` reader - DAC channel2 mask/amplitude selector"]
pub use MAMP1_R as MAMP2_R;
#[doc = "Field `MAMP2` writer - DAC channel2 mask/amplitude selector"]
pub use MAMP1_W as MAMP2_W;
#[doc = "Field `TEN2` reader - DAC channel2 trigger enable"]
pub use TEN1_R as TEN2_R;
#[doc = "Field `TEN2` writer - DAC channel2 trigger enable"]
pub use TEN1_W as TEN2_W;
#[doc = "Field `TSEL2` reader - DAC channel2 trigger selection"]
pub use TSEL1_R as TSEL2_R;
#[doc = "Field `TSEL2` writer - DAC channel2 trigger selection"]
pub use TSEL1_W as TSEL2_W;
#[doc = "Field `WAVE2` reader - DAC channel2 noise/triangle wave generation enable"]
pub use WAVE1_R as WAVE2_R;
#[doc = "Field `WAVE2` writer - DAC channel2 noise/triangle wave generation enable"]
pub use WAVE1_W as WAVE2_W;
impl R {
    #[doc = "Bit 0 - DAC channel1 enable"]
    #[inline(always)]
    pub fn en1(&self) -> EN1_R {
        EN1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DAC channel1 trigger enable"]
    #[inline(always)]
    pub fn ten1(&self) -> TEN1_R {
        TEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5 - DAC channel1 trigger selection"]
    #[inline(always)]
    pub fn tsel1(&self) -> TSEL1_R {
        TSEL1_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 6:7 - DAC channel1 noise/triangle wave generation enable"]
    #[inline(always)]
    pub fn wave1(&self) -> WAVE1_R {
        WAVE1_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:11 - DAC channel1 mask/amplitude selector"]
    #[inline(always)]
    pub fn mamp1(&self) -> MAMP1_R {
        MAMP1_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - DAC channel1 DMA enable"]
    #[inline(always)]
    pub fn dmaen1(&self) -> DMAEN1_R {
        DMAEN1_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DAC channel1 DMA Underrun Interrupt enable"]
    #[inline(always)]
    pub fn dmaudrie1(&self) -> DMAUDRIE1_R {
        DMAUDRIE1_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - DAC Channel 1 calibration enable"]
    #[inline(always)]
    pub fn cen1(&self) -> CEN1_R {
        CEN1_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - High frequency interface mode enable"]
    #[inline(always)]
    pub fn hfsel(&self) -> HFSEL_R {
        HFSEL_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - DAC channel2 enable"]
    #[inline(always)]
    pub fn en2(&self) -> EN2_R {
        EN2_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DAC channel2 trigger enable"]
    #[inline(always)]
    pub fn ten2(&self) -> TEN2_R {
        TEN2_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:21 - DAC channel2 trigger selection"]
    #[inline(always)]
    pub fn tsel2(&self) -> TSEL2_R {
        TSEL2_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 22:23 - DAC channel2 noise/triangle wave generation enable"]
    #[inline(always)]
    pub fn wave2(&self) -> WAVE2_R {
        WAVE2_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:27 - DAC channel2 mask/amplitude selector"]
    #[inline(always)]
    pub fn mamp2(&self) -> MAMP2_R {
        MAMP2_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - DAC channel2 DMA enable"]
    #[inline(always)]
    pub fn dmaen2(&self) -> DMAEN2_R {
        DMAEN2_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DAC channel2 DMA underrun interrupt enable"]
    #[inline(always)]
    pub fn dmaudrie2(&self) -> DMAUDRIE2_R {
        DMAUDRIE2_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - DAC Channel 2 calibration enable"]
    #[inline(always)]
    pub fn cen2(&self) -> CEN2_R {
        CEN2_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DAC channel1 enable"]
    #[inline(always)]
    #[must_use]
    pub fn en1(&mut self) -> EN1_W<CRrs> {
        EN1_W::new(self, 0)
    }
    #[doc = "Bit 1 - DAC channel1 trigger enable"]
    #[inline(always)]
    #[must_use]
    pub fn ten1(&mut self) -> TEN1_W<CRrs> {
        TEN1_W::new(self, 1)
    }
    #[doc = "Bits 2:5 - DAC channel1 trigger selection"]
    #[inline(always)]
    #[must_use]
    pub fn tsel1(&mut self) -> TSEL1_W<CRrs> {
        TSEL1_W::new(self, 2)
    }
    #[doc = "Bits 6:7 - DAC channel1 noise/triangle wave generation enable"]
    #[inline(always)]
    #[must_use]
    pub fn wave1(&mut self) -> WAVE1_W<CRrs> {
        WAVE1_W::new(self, 6)
    }
    #[doc = "Bits 8:11 - DAC channel1 mask/amplitude selector"]
    #[inline(always)]
    #[must_use]
    pub fn mamp1(&mut self) -> MAMP1_W<CRrs> {
        MAMP1_W::new(self, 8)
    }
    #[doc = "Bit 12 - DAC channel1 DMA enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen1(&mut self) -> DMAEN1_W<CRrs> {
        DMAEN1_W::new(self, 12)
    }
    #[doc = "Bit 13 - DAC channel1 DMA Underrun Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaudrie1(&mut self) -> DMAUDRIE1_W<CRrs> {
        DMAUDRIE1_W::new(self, 13)
    }
    #[doc = "Bit 14 - DAC Channel 1 calibration enable"]
    #[inline(always)]
    #[must_use]
    pub fn cen1(&mut self) -> CEN1_W<CRrs> {
        CEN1_W::new(self, 14)
    }
    #[doc = "Bit 15 - High frequency interface mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn hfsel(&mut self) -> HFSEL_W<CRrs> {
        HFSEL_W::new(self, 15)
    }
    #[doc = "Bit 16 - DAC channel2 enable"]
    #[inline(always)]
    #[must_use]
    pub fn en2(&mut self) -> EN2_W<CRrs> {
        EN2_W::new(self, 16)
    }
    #[doc = "Bit 17 - DAC channel2 trigger enable"]
    #[inline(always)]
    #[must_use]
    pub fn ten2(&mut self) -> TEN2_W<CRrs> {
        TEN2_W::new(self, 17)
    }
    #[doc = "Bits 18:21 - DAC channel2 trigger selection"]
    #[inline(always)]
    #[must_use]
    pub fn tsel2(&mut self) -> TSEL2_W<CRrs> {
        TSEL2_W::new(self, 18)
    }
    #[doc = "Bits 22:23 - DAC channel2 noise/triangle wave generation enable"]
    #[inline(always)]
    #[must_use]
    pub fn wave2(&mut self) -> WAVE2_W<CRrs> {
        WAVE2_W::new(self, 22)
    }
    #[doc = "Bits 24:27 - DAC channel2 mask/amplitude selector"]
    #[inline(always)]
    #[must_use]
    pub fn mamp2(&mut self) -> MAMP2_W<CRrs> {
        MAMP2_W::new(self, 24)
    }
    #[doc = "Bit 28 - DAC channel2 DMA enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen2(&mut self) -> DMAEN2_W<CRrs> {
        DMAEN2_W::new(self, 28)
    }
    #[doc = "Bit 29 - DAC channel2 DMA underrun interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaudrie2(&mut self) -> DMAUDRIE2_W<CRrs> {
        DMAUDRIE2_W::new(self, 29)
    }
    #[doc = "Bit 30 - DAC Channel 2 calibration enable"]
    #[inline(always)]
    #[must_use]
    pub fn cen2(&mut self) -> CEN2_W<CRrs> {
        CEN2_W::new(self, 30)
    }
}
#[doc = "DAC control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CRrs {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0;
}
