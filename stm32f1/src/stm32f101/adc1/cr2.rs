///Register `CR2` reader
pub type R = crate::R<CR2rs>;
///Register `CR2` writer
pub type W = crate::W<CR2rs>;
/**A/D converter ON / OFF

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADON {
    ///0: Disable ADC conversion/calibration and go to power down mode
    Disabled = 0,
    ///1: Enable ADC and to start conversion
    Enabled = 1,
}
impl From<ADON> for bool {
    #[inline(always)]
    fn from(variant: ADON) -> Self {
        variant as u8 != 0
    }
}
///Field `ADON` reader - A/D converter ON / OFF
pub type ADON_R = crate::BitReader<ADON>;
impl ADON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADON {
        match self.bits {
            false => ADON::Disabled,
            true => ADON::Enabled,
        }
    }
    ///Disable ADC conversion/calibration and go to power down mode
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADON::Disabled
    }
    ///Enable ADC and to start conversion
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADON::Enabled
    }
}
///Field `ADON` writer - A/D converter ON / OFF
pub type ADON_W<'a, REG> = crate::BitWriter<'a, REG, ADON>;
impl<'a, REG> ADON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable ADC conversion/calibration and go to power down mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ADON::Disabled)
    }
    ///Enable ADC and to start conversion
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ADON::Enabled)
    }
}
/**Continuous conversion

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CONT {
    ///0: Single conversion mode
    Single = 0,
    ///1: Continuous conversion mode
    Continuous = 1,
}
impl From<CONT> for bool {
    #[inline(always)]
    fn from(variant: CONT) -> Self {
        variant as u8 != 0
    }
}
///Field `CONT` reader - Continuous conversion
pub type CONT_R = crate::BitReader<CONT>;
impl CONT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CONT {
        match self.bits {
            false => CONT::Single,
            true => CONT::Continuous,
        }
    }
    ///Single conversion mode
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == CONT::Single
    }
    ///Continuous conversion mode
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == CONT::Continuous
    }
}
///Field `CONT` writer - Continuous conversion
pub type CONT_W<'a, REG> = crate::BitWriter<'a, REG, CONT>;
impl<'a, REG> CONT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Single conversion mode
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(CONT::Single)
    }
    ///Continuous conversion mode
    #[inline(always)]
    pub fn continuous(self) -> &'a mut crate::W<REG> {
        self.variant(CONT::Continuous)
    }
}
/**A/D calibration

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CALR {
    ///0: Calibration completed
    Complete = 0,
    ///1: Calibrating
    NotComplete = 1,
}
impl From<CALR> for bool {
    #[inline(always)]
    fn from(variant: CALR) -> Self {
        variant as u8 != 0
    }
}
///Field `CAL` reader - A/D calibration
pub type CAL_R = crate::BitReader<CALR>;
impl CAL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CALR {
        match self.bits {
            false => CALR::Complete,
            true => CALR::NotComplete,
        }
    }
    ///Calibration completed
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == CALR::Complete
    }
    ///Calibrating
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == CALR::NotComplete
    }
}
/**A/D calibration

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CALW {
    ///1: Enable calibration
    Start = 1,
}
impl From<CALW> for bool {
    #[inline(always)]
    fn from(variant: CALW) -> Self {
        variant as u8 != 0
    }
}
///Field `CAL` writer - A/D calibration
pub type CAL_W<'a, REG> = crate::BitWriter<'a, REG, CALW>;
impl<'a, REG> CAL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Enable calibration
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(CALW::Start)
    }
}
/**Reset calibration

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSTCALR {
    ///0: Calibration register initialized
    Initialized = 0,
    ///1: Initializing calibration register
    NotInitialized = 1,
}
impl From<RSTCALR> for bool {
    #[inline(always)]
    fn from(variant: RSTCALR) -> Self {
        variant as u8 != 0
    }
}
///Field `RSTCAL` reader - Reset calibration
pub type RSTCAL_R = crate::BitReader<RSTCALR>;
impl RSTCAL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RSTCALR {
        match self.bits {
            false => RSTCALR::Initialized,
            true => RSTCALR::NotInitialized,
        }
    }
    ///Calibration register initialized
    #[inline(always)]
    pub fn is_initialized(&self) -> bool {
        *self == RSTCALR::Initialized
    }
    ///Initializing calibration register
    #[inline(always)]
    pub fn is_not_initialized(&self) -> bool {
        *self == RSTCALR::NotInitialized
    }
}
/**Reset calibration

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSTCALW {
    ///1: Initialize calibration register
    Initialize = 1,
}
impl From<RSTCALW> for bool {
    #[inline(always)]
    fn from(variant: RSTCALW) -> Self {
        variant as u8 != 0
    }
}
///Field `RSTCAL` writer - Reset calibration
pub type RSTCAL_W<'a, REG> = crate::BitWriter<'a, REG, RSTCALW>;
impl<'a, REG> RSTCAL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Initialize calibration register
    #[inline(always)]
    pub fn initialize(self) -> &'a mut crate::W<REG> {
        self.variant(RSTCALW::Initialize)
    }
}
/**Direct memory access mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA {
    ///0: DMA mode disabled
    Disabled = 0,
    ///1: DMA mode enabled
    Enabled = 1,
}
impl From<DMA> for bool {
    #[inline(always)]
    fn from(variant: DMA) -> Self {
        variant as u8 != 0
    }
}
///Field `DMA` reader - Direct memory access mode
pub type DMA_R = crate::BitReader<DMA>;
impl DMA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DMA {
        match self.bits {
            false => DMA::Disabled,
            true => DMA::Enabled,
        }
    }
    ///DMA mode disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMA::Disabled
    }
    ///DMA mode enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMA::Enabled
    }
}
///Field `DMA` writer - Direct memory access mode
pub type DMA_W<'a, REG> = crate::BitWriter<'a, REG, DMA>;
impl<'a, REG> DMA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DMA mode disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMA::Disabled)
    }
    ///DMA mode enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMA::Enabled)
    }
}
/**Data alignment

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALIGN {
    ///0: Right Alignment
    Right = 0,
    ///1: Left Alignment
    Left = 1,
}
impl From<ALIGN> for bool {
    #[inline(always)]
    fn from(variant: ALIGN) -> Self {
        variant as u8 != 0
    }
}
///Field `ALIGN` reader - Data alignment
pub type ALIGN_R = crate::BitReader<ALIGN>;
impl ALIGN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ALIGN {
        match self.bits {
            false => ALIGN::Right,
            true => ALIGN::Left,
        }
    }
    ///Right Alignment
    #[inline(always)]
    pub fn is_right(&self) -> bool {
        *self == ALIGN::Right
    }
    ///Left Alignment
    #[inline(always)]
    pub fn is_left(&self) -> bool {
        *self == ALIGN::Left
    }
}
///Field `ALIGN` writer - Data alignment
pub type ALIGN_W<'a, REG> = crate::BitWriter<'a, REG, ALIGN>;
impl<'a, REG> ALIGN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Right Alignment
    #[inline(always)]
    pub fn right(self) -> &'a mut crate::W<REG> {
        self.variant(ALIGN::Right)
    }
    ///Left Alignment
    #[inline(always)]
    pub fn left(self) -> &'a mut crate::W<REG> {
        self.variant(ALIGN::Left)
    }
}
/**External event select for injected group

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum JEXTSEL {
    ///0: Timer 1 TRGO event
    Tim1trgo = 0,
    ///1: Timer 1 CC4 event
    Tim1cc4 = 1,
    ///2: Timer 2 TRGO event
    Tim2trgo = 2,
    ///3: Timer 2 CC1 event
    Tim2cc1 = 3,
    ///4: Timer 3 CC4 event
    Tim3cc4 = 4,
    ///5: Timer 4 TRGO event
    Tim4trgo = 5,
    ///6: EXTI line15/TIM8_CC4 event (TIM8_CC4 is available only in high-density and XL-density devices)
    Exti15 = 6,
    ///7: JSWSTART
    Jswstart = 7,
}
impl From<JEXTSEL> for u8 {
    #[inline(always)]
    fn from(variant: JEXTSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for JEXTSEL {
    type Ux = u8;
}
impl crate::IsEnum for JEXTSEL {}
///Field `JEXTSEL` reader - External event select for injected group
pub type JEXTSEL_R = crate::FieldReader<JEXTSEL>;
impl JEXTSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> JEXTSEL {
        match self.bits {
            0 => JEXTSEL::Tim1trgo,
            1 => JEXTSEL::Tim1cc4,
            2 => JEXTSEL::Tim2trgo,
            3 => JEXTSEL::Tim2cc1,
            4 => JEXTSEL::Tim3cc4,
            5 => JEXTSEL::Tim4trgo,
            6 => JEXTSEL::Exti15,
            7 => JEXTSEL::Jswstart,
            _ => unreachable!(),
        }
    }
    ///Timer 1 TRGO event
    #[inline(always)]
    pub fn is_tim1trgo(&self) -> bool {
        *self == JEXTSEL::Tim1trgo
    }
    ///Timer 1 CC4 event
    #[inline(always)]
    pub fn is_tim1cc4(&self) -> bool {
        *self == JEXTSEL::Tim1cc4
    }
    ///Timer 2 TRGO event
    #[inline(always)]
    pub fn is_tim2trgo(&self) -> bool {
        *self == JEXTSEL::Tim2trgo
    }
    ///Timer 2 CC1 event
    #[inline(always)]
    pub fn is_tim2cc1(&self) -> bool {
        *self == JEXTSEL::Tim2cc1
    }
    ///Timer 3 CC4 event
    #[inline(always)]
    pub fn is_tim3cc4(&self) -> bool {
        *self == JEXTSEL::Tim3cc4
    }
    ///Timer 4 TRGO event
    #[inline(always)]
    pub fn is_tim4trgo(&self) -> bool {
        *self == JEXTSEL::Tim4trgo
    }
    ///EXTI line15/TIM8_CC4 event (TIM8_CC4 is available only in high-density and XL-density devices)
    #[inline(always)]
    pub fn is_exti15(&self) -> bool {
        *self == JEXTSEL::Exti15
    }
    ///JSWSTART
    #[inline(always)]
    pub fn is_jswstart(&self) -> bool {
        *self == JEXTSEL::Jswstart
    }
}
///Field `JEXTSEL` writer - External event select for injected group
pub type JEXTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, JEXTSEL, crate::Safe>;
impl<'a, REG> JEXTSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Timer 1 TRGO event
    #[inline(always)]
    pub fn tim1trgo(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim1trgo)
    }
    ///Timer 1 CC4 event
    #[inline(always)]
    pub fn tim1cc4(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim1cc4)
    }
    ///Timer 2 TRGO event
    #[inline(always)]
    pub fn tim2trgo(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim2trgo)
    }
    ///Timer 2 CC1 event
    #[inline(always)]
    pub fn tim2cc1(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim2cc1)
    }
    ///Timer 3 CC4 event
    #[inline(always)]
    pub fn tim3cc4(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim3cc4)
    }
    ///Timer 4 TRGO event
    #[inline(always)]
    pub fn tim4trgo(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim4trgo)
    }
    ///EXTI line15/TIM8_CC4 event (TIM8_CC4 is available only in high-density and XL-density devices)
    #[inline(always)]
    pub fn exti15(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Exti15)
    }
    ///JSWSTART
    #[inline(always)]
    pub fn jswstart(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Jswstart)
    }
}
/**External trigger conversion mode for injected channels

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JEXTTRIG {
    ///0: Conversion on external event disabled
    Disabled = 0,
    ///1: Conversion on external event enabled
    Enabled = 1,
}
impl From<JEXTTRIG> for bool {
    #[inline(always)]
    fn from(variant: JEXTTRIG) -> Self {
        variant as u8 != 0
    }
}
///Field `JEXTTRIG` reader - External trigger conversion mode for injected channels
pub type JEXTTRIG_R = crate::BitReader<JEXTTRIG>;
impl JEXTTRIG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> JEXTTRIG {
        match self.bits {
            false => JEXTTRIG::Disabled,
            true => JEXTTRIG::Enabled,
        }
    }
    ///Conversion on external event disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JEXTTRIG::Disabled
    }
    ///Conversion on external event enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == JEXTTRIG::Enabled
    }
}
///Field `JEXTTRIG` writer - External trigger conversion mode for injected channels
pub type JEXTTRIG_W<'a, REG> = crate::BitWriter<'a, REG, JEXTTRIG>;
impl<'a, REG> JEXTTRIG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Conversion on external event disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTTRIG::Disabled)
    }
    ///Conversion on external event enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTTRIG::Enabled)
    }
}
/**External event select for regular group

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTSEL {
    ///0: Timer 1 CC1 event
    Tim1cc1 = 0,
    ///1: Timer 1 CC2 event
    Tim1cc2 = 1,
    ///2: Timer 1 CC3 event
    Tim1cc3 = 2,
    ///3: Timer 2 CC2 event
    Tim2cc2 = 3,
    ///4: Timer 3 TRGO event
    Tim3trgo = 4,
    ///5: Timer 4 CC4 event
    Tim4cc4 = 5,
    ///6: EXTI line 11/TIM8_TRGO event (TIM8_TRGO is available only in high-density and XL-density devices)
    Exti11 = 6,
    ///7: SWSTART
    Swstart = 7,
}
impl From<EXTSEL> for u8 {
    #[inline(always)]
    fn from(variant: EXTSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTSEL {
    type Ux = u8;
}
impl crate::IsEnum for EXTSEL {}
///Field `EXTSEL` reader - External event select for regular group
pub type EXTSEL_R = crate::FieldReader<EXTSEL>;
impl EXTSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EXTSEL {
        match self.bits {
            0 => EXTSEL::Tim1cc1,
            1 => EXTSEL::Tim1cc2,
            2 => EXTSEL::Tim1cc3,
            3 => EXTSEL::Tim2cc2,
            4 => EXTSEL::Tim3trgo,
            5 => EXTSEL::Tim4cc4,
            6 => EXTSEL::Exti11,
            7 => EXTSEL::Swstart,
            _ => unreachable!(),
        }
    }
    ///Timer 1 CC1 event
    #[inline(always)]
    pub fn is_tim1cc1(&self) -> bool {
        *self == EXTSEL::Tim1cc1
    }
    ///Timer 1 CC2 event
    #[inline(always)]
    pub fn is_tim1cc2(&self) -> bool {
        *self == EXTSEL::Tim1cc2
    }
    ///Timer 1 CC3 event
    #[inline(always)]
    pub fn is_tim1cc3(&self) -> bool {
        *self == EXTSEL::Tim1cc3
    }
    ///Timer 2 CC2 event
    #[inline(always)]
    pub fn is_tim2cc2(&self) -> bool {
        *self == EXTSEL::Tim2cc2
    }
    ///Timer 3 TRGO event
    #[inline(always)]
    pub fn is_tim3trgo(&self) -> bool {
        *self == EXTSEL::Tim3trgo
    }
    ///Timer 4 CC4 event
    #[inline(always)]
    pub fn is_tim4cc4(&self) -> bool {
        *self == EXTSEL::Tim4cc4
    }
    ///EXTI line 11/TIM8_TRGO event (TIM8_TRGO is available only in high-density and XL-density devices)
    #[inline(always)]
    pub fn is_exti11(&self) -> bool {
        *self == EXTSEL::Exti11
    }
    ///SWSTART
    #[inline(always)]
    pub fn is_swstart(&self) -> bool {
        *self == EXTSEL::Swstart
    }
}
///Field `EXTSEL` writer - External event select for regular group
pub type EXTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, EXTSEL, crate::Safe>;
impl<'a, REG> EXTSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Timer 1 CC1 event
    #[inline(always)]
    pub fn tim1cc1(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim1cc1)
    }
    ///Timer 1 CC2 event
    #[inline(always)]
    pub fn tim1cc2(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim1cc2)
    }
    ///Timer 1 CC3 event
    #[inline(always)]
    pub fn tim1cc3(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim1cc3)
    }
    ///Timer 2 CC2 event
    #[inline(always)]
    pub fn tim2cc2(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim2cc2)
    }
    ///Timer 3 TRGO event
    #[inline(always)]
    pub fn tim3trgo(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim3trgo)
    }
    ///Timer 4 CC4 event
    #[inline(always)]
    pub fn tim4cc4(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim4cc4)
    }
    ///EXTI line 11/TIM8_TRGO event (TIM8_TRGO is available only in high-density and XL-density devices)
    #[inline(always)]
    pub fn exti11(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Exti11)
    }
    ///SWSTART
    #[inline(always)]
    pub fn swstart(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Swstart)
    }
}
/**External trigger conversion mode for regular channels

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXTTRIG {
    ///0: Conversion on external event disabled
    Disabled = 0,
    ///1: Conversion on external event enabled
    Enabled = 1,
}
impl From<EXTTRIG> for bool {
    #[inline(always)]
    fn from(variant: EXTTRIG) -> Self {
        variant as u8 != 0
    }
}
///Field `EXTTRIG` reader - External trigger conversion mode for regular channels
pub type EXTTRIG_R = crate::BitReader<EXTTRIG>;
impl EXTTRIG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EXTTRIG {
        match self.bits {
            false => EXTTRIG::Disabled,
            true => EXTTRIG::Enabled,
        }
    }
    ///Conversion on external event disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EXTTRIG::Disabled
    }
    ///Conversion on external event enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EXTTRIG::Enabled
    }
}
///Field `EXTTRIG` writer - External trigger conversion mode for regular channels
pub type EXTTRIG_W<'a, REG> = crate::BitWriter<'a, REG, EXTTRIG>;
impl<'a, REG> EXTTRIG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Conversion on external event disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EXTTRIG::Disabled)
    }
    ///Conversion on external event enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EXTTRIG::Enabled)
    }
}
/**Start conversion of injected channels

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JSWSTARTR {
    ///0: Reset state
    Started = 0,
    ///1: Starting conversion of injected channels
    NotStarted = 1,
}
impl From<JSWSTARTR> for bool {
    #[inline(always)]
    fn from(variant: JSWSTARTR) -> Self {
        variant as u8 != 0
    }
}
///Field `JSWSTART` reader - Start conversion of injected channels
pub type JSWSTART_R = crate::BitReader<JSWSTARTR>;
impl JSWSTART_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> JSWSTARTR {
        match self.bits {
            false => JSWSTARTR::Started,
            true => JSWSTARTR::NotStarted,
        }
    }
    ///Reset state
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        *self == JSWSTARTR::Started
    }
    ///Starting conversion of injected channels
    #[inline(always)]
    pub fn is_not_started(&self) -> bool {
        *self == JSWSTARTR::NotStarted
    }
}
/**Start conversion of injected channels

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JSWSTARTW {
    ///1: Start conversion of injected channels
    Start = 1,
}
impl From<JSWSTARTW> for bool {
    #[inline(always)]
    fn from(variant: JSWSTARTW) -> Self {
        variant as u8 != 0
    }
}
///Field `JSWSTART` writer - Start conversion of injected channels
pub type JSWSTART_W<'a, REG> = crate::BitWriter<'a, REG, JSWSTARTW>;
impl<'a, REG> JSWSTART_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Start conversion of injected channels
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(JSWSTARTW::Start)
    }
}
/**Start conversion of regular channels

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWSTARTR {
    ///0: Reset state
    Started = 0,
    ///1: Starting conversion of regular channels
    NotStarted = 1,
}
impl From<SWSTARTR> for bool {
    #[inline(always)]
    fn from(variant: SWSTARTR) -> Self {
        variant as u8 != 0
    }
}
///Field `SWSTART` reader - Start conversion of regular channels
pub type SWSTART_R = crate::BitReader<SWSTARTR>;
impl SWSTART_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SWSTARTR {
        match self.bits {
            false => SWSTARTR::Started,
            true => SWSTARTR::NotStarted,
        }
    }
    ///Reset state
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        *self == SWSTARTR::Started
    }
    ///Starting conversion of regular channels
    #[inline(always)]
    pub fn is_not_started(&self) -> bool {
        *self == SWSTARTR::NotStarted
    }
}
/**Start conversion of regular channels

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWSTARTW {
    ///1: Start conversion of regular channels
    Start = 1,
}
impl From<SWSTARTW> for bool {
    #[inline(always)]
    fn from(variant: SWSTARTW) -> Self {
        variant as u8 != 0
    }
}
///Field `SWSTART` writer - Start conversion of regular channels
pub type SWSTART_W<'a, REG> = crate::BitWriter<'a, REG, SWSTARTW>;
impl<'a, REG> SWSTART_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Start conversion of regular channels
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(SWSTARTW::Start)
    }
}
/**Temperature sensor and VREFINT enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSVREFE {
    ///0: Temperature sensor and V_REFINT channel disabled
    Disabled = 0,
    ///1: Temperature sensor and V_REFINT channel enabled
    Enabled = 1,
}
impl From<TSVREFE> for bool {
    #[inline(always)]
    fn from(variant: TSVREFE) -> Self {
        variant as u8 != 0
    }
}
///Field `TSVREFE` reader - Temperature sensor and VREFINT enable
pub type TSVREFE_R = crate::BitReader<TSVREFE>;
impl TSVREFE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TSVREFE {
        match self.bits {
            false => TSVREFE::Disabled,
            true => TSVREFE::Enabled,
        }
    }
    ///Temperature sensor and V_REFINT channel disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TSVREFE::Disabled
    }
    ///Temperature sensor and V_REFINT channel enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TSVREFE::Enabled
    }
}
///Field `TSVREFE` writer - Temperature sensor and VREFINT enable
pub type TSVREFE_W<'a, REG> = crate::BitWriter<'a, REG, TSVREFE>;
impl<'a, REG> TSVREFE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Temperature sensor and V_REFINT channel disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TSVREFE::Disabled)
    }
    ///Temperature sensor and V_REFINT channel enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TSVREFE::Enabled)
    }
}
impl R {
    ///Bit 0 - A/D converter ON / OFF
    #[inline(always)]
    pub fn adon(&self) -> ADON_R {
        ADON_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Continuous conversion
    #[inline(always)]
    pub fn cont(&self) -> CONT_R {
        CONT_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - A/D calibration
    #[inline(always)]
    pub fn cal(&self) -> CAL_R {
        CAL_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Reset calibration
    #[inline(always)]
    pub fn rstcal(&self) -> RSTCAL_R {
        RSTCAL_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 8 - Direct memory access mode
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 11 - Data alignment
    #[inline(always)]
    pub fn align(&self) -> ALIGN_R {
        ALIGN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:14 - External event select for injected group
    #[inline(always)]
    pub fn jextsel(&self) -> JEXTSEL_R {
        JEXTSEL_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bit 15 - External trigger conversion mode for injected channels
    #[inline(always)]
    pub fn jexttrig(&self) -> JEXTTRIG_R {
        JEXTTRIG_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 17:19 - External event select for regular group
    #[inline(always)]
    pub fn extsel(&self) -> EXTSEL_R {
        EXTSEL_R::new(((self.bits >> 17) & 7) as u8)
    }
    ///Bit 20 - External trigger conversion mode for regular channels
    #[inline(always)]
    pub fn exttrig(&self) -> EXTTRIG_R {
        EXTTRIG_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Start conversion of injected channels
    #[inline(always)]
    pub fn jswstart(&self) -> JSWSTART_R {
        JSWSTART_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Start conversion of regular channels
    #[inline(always)]
    pub fn swstart(&self) -> SWSTART_R {
        SWSTART_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Temperature sensor and VREFINT enable
    #[inline(always)]
    pub fn tsvrefe(&self) -> TSVREFE_R {
        TSVREFE_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR2")
            .field("tsvrefe", &self.tsvrefe())
            .field("swstart", &self.swstart())
            .field("jswstart", &self.jswstart())
            .field("exttrig", &self.exttrig())
            .field("extsel", &self.extsel())
            .field("jexttrig", &self.jexttrig())
            .field("jextsel", &self.jextsel())
            .field("align", &self.align())
            .field("dma", &self.dma())
            .field("rstcal", &self.rstcal())
            .field("cal", &self.cal())
            .field("cont", &self.cont())
            .field("adon", &self.adon())
            .finish()
    }
}
impl W {
    ///Bit 0 - A/D converter ON / OFF
    #[inline(always)]
    pub fn adon(&mut self) -> ADON_W<'_, CR2rs> {
        ADON_W::new(self, 0)
    }
    ///Bit 1 - Continuous conversion
    #[inline(always)]
    pub fn cont(&mut self) -> CONT_W<'_, CR2rs> {
        CONT_W::new(self, 1)
    }
    ///Bit 2 - A/D calibration
    #[inline(always)]
    pub fn cal(&mut self) -> CAL_W<'_, CR2rs> {
        CAL_W::new(self, 2)
    }
    ///Bit 3 - Reset calibration
    #[inline(always)]
    pub fn rstcal(&mut self) -> RSTCAL_W<'_, CR2rs> {
        RSTCAL_W::new(self, 3)
    }
    ///Bit 8 - Direct memory access mode
    #[inline(always)]
    pub fn dma(&mut self) -> DMA_W<'_, CR2rs> {
        DMA_W::new(self, 8)
    }
    ///Bit 11 - Data alignment
    #[inline(always)]
    pub fn align(&mut self) -> ALIGN_W<'_, CR2rs> {
        ALIGN_W::new(self, 11)
    }
    ///Bits 12:14 - External event select for injected group
    #[inline(always)]
    pub fn jextsel(&mut self) -> JEXTSEL_W<'_, CR2rs> {
        JEXTSEL_W::new(self, 12)
    }
    ///Bit 15 - External trigger conversion mode for injected channels
    #[inline(always)]
    pub fn jexttrig(&mut self) -> JEXTTRIG_W<'_, CR2rs> {
        JEXTTRIG_W::new(self, 15)
    }
    ///Bits 17:19 - External event select for regular group
    #[inline(always)]
    pub fn extsel(&mut self) -> EXTSEL_W<'_, CR2rs> {
        EXTSEL_W::new(self, 17)
    }
    ///Bit 20 - External trigger conversion mode for regular channels
    #[inline(always)]
    pub fn exttrig(&mut self) -> EXTTRIG_W<'_, CR2rs> {
        EXTTRIG_W::new(self, 20)
    }
    ///Bit 21 - Start conversion of injected channels
    #[inline(always)]
    pub fn jswstart(&mut self) -> JSWSTART_W<'_, CR2rs> {
        JSWSTART_W::new(self, 21)
    }
    ///Bit 22 - Start conversion of regular channels
    #[inline(always)]
    pub fn swstart(&mut self) -> SWSTART_W<'_, CR2rs> {
        SWSTART_W::new(self, 22)
    }
    ///Bit 23 - Temperature sensor and VREFINT enable
    #[inline(always)]
    pub fn tsvrefe(&mut self) -> TSVREFE_W<'_, CR2rs> {
        TSVREFE_W::new(self, 23)
    }
}
/**control register 2

You can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F101.html#ADC1:CR2)*/
pub struct CR2rs;
impl crate::RegisterSpec for CR2rs {
    type Ux = u32;
}
///`read()` method returns [`cr2::R`](R) reader structure
impl crate::Readable for CR2rs {}
///`write(|w| ..)` method takes [`cr2::W`](W) writer structure
impl crate::Writable for CR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR2 to value 0
impl crate::Resettable for CR2rs {}
