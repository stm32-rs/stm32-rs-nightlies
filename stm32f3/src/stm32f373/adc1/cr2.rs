#[doc = "Register `CR2` reader"]
pub type R = crate::R<CR2rs>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<CR2rs>;
#[doc = "A/D converter ON / OFF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADON {
    #[doc = "0: Disable ADC conversion/calibration and go to power down mode"]
    Disabled = 0,
    #[doc = "1: Enable ADC and to start conversion"]
    Enabled = 1,
}
impl From<ADON> for bool {
    #[inline(always)]
    fn from(variant: ADON) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADON` reader - A/D converter ON / OFF"]
pub type ADON_R = crate::BitReader<ADON>;
impl ADON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADON {
        match self.bits {
            false => ADON::Disabled,
            true => ADON::Enabled,
        }
    }
    #[doc = "Disable ADC conversion/calibration and go to power down mode"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADON::Disabled
    }
    #[doc = "Enable ADC and to start conversion"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADON::Enabled
    }
}
#[doc = "Field `ADON` writer - A/D converter ON / OFF"]
pub type ADON_W<'a, REG> = crate::BitWriter<'a, REG, ADON>;
impl<'a, REG> ADON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable ADC conversion/calibration and go to power down mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ADON::Disabled)
    }
    #[doc = "Enable ADC and to start conversion"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ADON::Enabled)
    }
}
#[doc = "Continuous conversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CONT {
    #[doc = "0: Single conversion mode"]
    Single = 0,
    #[doc = "1: Continuous conversion mode"]
    Continuous = 1,
}
impl From<CONT> for bool {
    #[inline(always)]
    fn from(variant: CONT) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CONT` reader - Continuous conversion"]
pub type CONT_R = crate::BitReader<CONT>;
impl CONT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CONT {
        match self.bits {
            false => CONT::Single,
            true => CONT::Continuous,
        }
    }
    #[doc = "Single conversion mode"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == CONT::Single
    }
    #[doc = "Continuous conversion mode"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == CONT::Continuous
    }
}
#[doc = "Field `CONT` writer - Continuous conversion"]
pub type CONT_W<'a, REG> = crate::BitWriter<'a, REG, CONT>;
impl<'a, REG> CONT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Single conversion mode"]
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(CONT::Single)
    }
    #[doc = "Continuous conversion mode"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut crate::W<REG> {
        self.variant(CONT::Continuous)
    }
}
#[doc = "A/D calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CALR {
    #[doc = "0: Calibration completed"]
    Complete = 0,
    #[doc = "1: Calibrating"]
    NotComplete = 1,
}
impl From<CALR> for bool {
    #[inline(always)]
    fn from(variant: CALR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAL` reader - A/D calibration"]
pub type CAL_R = crate::BitReader<CALR>;
impl CAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CALR {
        match self.bits {
            false => CALR::Complete,
            true => CALR::NotComplete,
        }
    }
    #[doc = "Calibration completed"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == CALR::Complete
    }
    #[doc = "Calibrating"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == CALR::NotComplete
    }
}
#[doc = "A/D calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CALW {
    #[doc = "1: Enable calibration"]
    Start = 1,
}
impl From<CALW> for bool {
    #[inline(always)]
    fn from(variant: CALW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAL` writer - A/D calibration"]
pub type CAL_W<'a, REG> = crate::BitWriter<'a, REG, CALW>;
impl<'a, REG> CAL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable calibration"]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(CALW::Start)
    }
}
#[doc = "Reset calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSTCALR {
    #[doc = "0: Calibration register initialized"]
    Initialized = 0,
    #[doc = "1: Initializing calibration register"]
    NotInitialized = 1,
}
impl From<RSTCALR> for bool {
    #[inline(always)]
    fn from(variant: RSTCALR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSTCAL` reader - Reset calibration"]
pub type RSTCAL_R = crate::BitReader<RSTCALR>;
impl RSTCAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RSTCALR {
        match self.bits {
            false => RSTCALR::Initialized,
            true => RSTCALR::NotInitialized,
        }
    }
    #[doc = "Calibration register initialized"]
    #[inline(always)]
    pub fn is_initialized(&self) -> bool {
        *self == RSTCALR::Initialized
    }
    #[doc = "Initializing calibration register"]
    #[inline(always)]
    pub fn is_not_initialized(&self) -> bool {
        *self == RSTCALR::NotInitialized
    }
}
#[doc = "Reset calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSTCALW {
    #[doc = "1: Initialize calibration register"]
    Initialize = 1,
}
impl From<RSTCALW> for bool {
    #[inline(always)]
    fn from(variant: RSTCALW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSTCAL` writer - Reset calibration"]
pub type RSTCAL_W<'a, REG> = crate::BitWriter<'a, REG, RSTCALW>;
impl<'a, REG> RSTCAL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Initialize calibration register"]
    #[inline(always)]
    pub fn initialize(self) -> &'a mut crate::W<REG> {
        self.variant(RSTCALW::Initialize)
    }
}
#[doc = "Direct memory access mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA {
    #[doc = "0: DMA mode disabled"]
    Disabled = 0,
    #[doc = "1: DMA mode enabled"]
    Enabled = 1,
}
impl From<DMA> for bool {
    #[inline(always)]
    fn from(variant: DMA) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA` reader - Direct memory access mode"]
pub type DMA_R = crate::BitReader<DMA>;
impl DMA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMA {
        match self.bits {
            false => DMA::Disabled,
            true => DMA::Enabled,
        }
    }
    #[doc = "DMA mode disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMA::Disabled
    }
    #[doc = "DMA mode enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMA::Enabled
    }
}
#[doc = "Field `DMA` writer - Direct memory access mode"]
pub type DMA_W<'a, REG> = crate::BitWriter<'a, REG, DMA>;
impl<'a, REG> DMA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMA::Disabled)
    }
    #[doc = "DMA mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMA::Enabled)
    }
}
#[doc = "Data alignment\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALIGN {
    #[doc = "0: Right Alignment"]
    Right = 0,
    #[doc = "1: Left Alignment"]
    Left = 1,
}
impl From<ALIGN> for bool {
    #[inline(always)]
    fn from(variant: ALIGN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALIGN` reader - Data alignment"]
pub type ALIGN_R = crate::BitReader<ALIGN>;
impl ALIGN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ALIGN {
        match self.bits {
            false => ALIGN::Right,
            true => ALIGN::Left,
        }
    }
    #[doc = "Right Alignment"]
    #[inline(always)]
    pub fn is_right(&self) -> bool {
        *self == ALIGN::Right
    }
    #[doc = "Left Alignment"]
    #[inline(always)]
    pub fn is_left(&self) -> bool {
        *self == ALIGN::Left
    }
}
#[doc = "Field `ALIGN` writer - Data alignment"]
pub type ALIGN_W<'a, REG> = crate::BitWriter<'a, REG, ALIGN>;
impl<'a, REG> ALIGN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Right Alignment"]
    #[inline(always)]
    pub fn right(self) -> &'a mut crate::W<REG> {
        self.variant(ALIGN::Right)
    }
    #[doc = "Left Alignment"]
    #[inline(always)]
    pub fn left(self) -> &'a mut crate::W<REG> {
        self.variant(ALIGN::Left)
    }
}
#[doc = "External event select for injected group\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum JEXTSEL {
    #[doc = "0: Timer 19 CC1 event"]
    Tim19cc1 = 0,
    #[doc = "1: Timer 19 CC2 event"]
    Tim19cc2 = 1,
    #[doc = "2: Timer 2 TRGO event"]
    Tim2trgo = 2,
    #[doc = "3: Timer 2 CC1 event"]
    Tim2cc1 = 3,
    #[doc = "4: Timer 3 CC4 event"]
    Tim3cc4 = 4,
    #[doc = "5: Timer 4 TRGO event"]
    Tim4trgo = 5,
    #[doc = "6: EXTI line15"]
    Exti15 = 6,
    #[doc = "7: JSWSTART"]
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
#[doc = "Field `JEXTSEL` reader - External event select for injected group"]
pub type JEXTSEL_R = crate::FieldReader<JEXTSEL>;
impl JEXTSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> JEXTSEL {
        match self.bits {
            0 => JEXTSEL::Tim19cc1,
            1 => JEXTSEL::Tim19cc2,
            2 => JEXTSEL::Tim2trgo,
            3 => JEXTSEL::Tim2cc1,
            4 => JEXTSEL::Tim3cc4,
            5 => JEXTSEL::Tim4trgo,
            6 => JEXTSEL::Exti15,
            7 => JEXTSEL::Jswstart,
            _ => unreachable!(),
        }
    }
    #[doc = "Timer 19 CC1 event"]
    #[inline(always)]
    pub fn is_tim19cc1(&self) -> bool {
        *self == JEXTSEL::Tim19cc1
    }
    #[doc = "Timer 19 CC2 event"]
    #[inline(always)]
    pub fn is_tim19cc2(&self) -> bool {
        *self == JEXTSEL::Tim19cc2
    }
    #[doc = "Timer 2 TRGO event"]
    #[inline(always)]
    pub fn is_tim2trgo(&self) -> bool {
        *self == JEXTSEL::Tim2trgo
    }
    #[doc = "Timer 2 CC1 event"]
    #[inline(always)]
    pub fn is_tim2cc1(&self) -> bool {
        *self == JEXTSEL::Tim2cc1
    }
    #[doc = "Timer 3 CC4 event"]
    #[inline(always)]
    pub fn is_tim3cc4(&self) -> bool {
        *self == JEXTSEL::Tim3cc4
    }
    #[doc = "Timer 4 TRGO event"]
    #[inline(always)]
    pub fn is_tim4trgo(&self) -> bool {
        *self == JEXTSEL::Tim4trgo
    }
    #[doc = "EXTI line15"]
    #[inline(always)]
    pub fn is_exti15(&self) -> bool {
        *self == JEXTSEL::Exti15
    }
    #[doc = "JSWSTART"]
    #[inline(always)]
    pub fn is_jswstart(&self) -> bool {
        *self == JEXTSEL::Jswstart
    }
}
#[doc = "Field `JEXTSEL` writer - External event select for injected group"]
pub type JEXTSEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, JEXTSEL>;
impl<'a, REG> JEXTSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timer 19 CC1 event"]
    #[inline(always)]
    pub fn tim19cc1(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim19cc1)
    }
    #[doc = "Timer 19 CC2 event"]
    #[inline(always)]
    pub fn tim19cc2(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim19cc2)
    }
    #[doc = "Timer 2 TRGO event"]
    #[inline(always)]
    pub fn tim2trgo(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim2trgo)
    }
    #[doc = "Timer 2 CC1 event"]
    #[inline(always)]
    pub fn tim2cc1(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim2cc1)
    }
    #[doc = "Timer 3 CC4 event"]
    #[inline(always)]
    pub fn tim3cc4(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim3cc4)
    }
    #[doc = "Timer 4 TRGO event"]
    #[inline(always)]
    pub fn tim4trgo(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim4trgo)
    }
    #[doc = "EXTI line15"]
    #[inline(always)]
    pub fn exti15(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Exti15)
    }
    #[doc = "JSWSTART"]
    #[inline(always)]
    pub fn jswstart(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Jswstart)
    }
}
#[doc = "External trigger conversion mode for injected channels\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JEXTTRIG {
    #[doc = "0: Conversion on external event disabled"]
    Disabled = 0,
    #[doc = "1: Conversion on external event enabled"]
    Enabled = 1,
}
impl From<JEXTTRIG> for bool {
    #[inline(always)]
    fn from(variant: JEXTTRIG) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JEXTTRIG` reader - External trigger conversion mode for injected channels"]
pub type JEXTTRIG_R = crate::BitReader<JEXTTRIG>;
impl JEXTTRIG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> JEXTTRIG {
        match self.bits {
            false => JEXTTRIG::Disabled,
            true => JEXTTRIG::Enabled,
        }
    }
    #[doc = "Conversion on external event disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JEXTTRIG::Disabled
    }
    #[doc = "Conversion on external event enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == JEXTTRIG::Enabled
    }
}
#[doc = "Field `JEXTTRIG` writer - External trigger conversion mode for injected channels"]
pub type JEXTTRIG_W<'a, REG> = crate::BitWriter<'a, REG, JEXTTRIG>;
impl<'a, REG> JEXTTRIG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Conversion on external event disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTTRIG::Disabled)
    }
    #[doc = "Conversion on external event enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTTRIG::Enabled)
    }
}
#[doc = "External event select for regular group\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTSEL {
    #[doc = "0: Timer 19 TRGO event"]
    Tim19trgo = 0,
    #[doc = "1: Timer 19 CC3 event"]
    Tim19cc3 = 1,
    #[doc = "2: Timer 19 CC4 event"]
    Tim19cc4 = 2,
    #[doc = "3: Timer 2 CC2 event"]
    Tim2cc2 = 3,
    #[doc = "4: Timer 3 TRGO event"]
    Tim3trgo = 4,
    #[doc = "5: Timer 4 CC4 event"]
    Tim4cc4 = 5,
    #[doc = "6: EXTI line 11"]
    Exti11 = 6,
    #[doc = "7: SWSTART"]
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
#[doc = "Field `EXTSEL` reader - External event select for regular group"]
pub type EXTSEL_R = crate::FieldReader<EXTSEL>;
impl EXTSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EXTSEL {
        match self.bits {
            0 => EXTSEL::Tim19trgo,
            1 => EXTSEL::Tim19cc3,
            2 => EXTSEL::Tim19cc4,
            3 => EXTSEL::Tim2cc2,
            4 => EXTSEL::Tim3trgo,
            5 => EXTSEL::Tim4cc4,
            6 => EXTSEL::Exti11,
            7 => EXTSEL::Swstart,
            _ => unreachable!(),
        }
    }
    #[doc = "Timer 19 TRGO event"]
    #[inline(always)]
    pub fn is_tim19trgo(&self) -> bool {
        *self == EXTSEL::Tim19trgo
    }
    #[doc = "Timer 19 CC3 event"]
    #[inline(always)]
    pub fn is_tim19cc3(&self) -> bool {
        *self == EXTSEL::Tim19cc3
    }
    #[doc = "Timer 19 CC4 event"]
    #[inline(always)]
    pub fn is_tim19cc4(&self) -> bool {
        *self == EXTSEL::Tim19cc4
    }
    #[doc = "Timer 2 CC2 event"]
    #[inline(always)]
    pub fn is_tim2cc2(&self) -> bool {
        *self == EXTSEL::Tim2cc2
    }
    #[doc = "Timer 3 TRGO event"]
    #[inline(always)]
    pub fn is_tim3trgo(&self) -> bool {
        *self == EXTSEL::Tim3trgo
    }
    #[doc = "Timer 4 CC4 event"]
    #[inline(always)]
    pub fn is_tim4cc4(&self) -> bool {
        *self == EXTSEL::Tim4cc4
    }
    #[doc = "EXTI line 11"]
    #[inline(always)]
    pub fn is_exti11(&self) -> bool {
        *self == EXTSEL::Exti11
    }
    #[doc = "SWSTART"]
    #[inline(always)]
    pub fn is_swstart(&self) -> bool {
        *self == EXTSEL::Swstart
    }
}
#[doc = "Field `EXTSEL` writer - External event select for regular group"]
pub type EXTSEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, EXTSEL>;
impl<'a, REG> EXTSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timer 19 TRGO event"]
    #[inline(always)]
    pub fn tim19trgo(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim19trgo)
    }
    #[doc = "Timer 19 CC3 event"]
    #[inline(always)]
    pub fn tim19cc3(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim19cc3)
    }
    #[doc = "Timer 19 CC4 event"]
    #[inline(always)]
    pub fn tim19cc4(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim19cc4)
    }
    #[doc = "Timer 2 CC2 event"]
    #[inline(always)]
    pub fn tim2cc2(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim2cc2)
    }
    #[doc = "Timer 3 TRGO event"]
    #[inline(always)]
    pub fn tim3trgo(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim3trgo)
    }
    #[doc = "Timer 4 CC4 event"]
    #[inline(always)]
    pub fn tim4cc4(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim4cc4)
    }
    #[doc = "EXTI line 11"]
    #[inline(always)]
    pub fn exti11(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Exti11)
    }
    #[doc = "SWSTART"]
    #[inline(always)]
    pub fn swstart(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Swstart)
    }
}
#[doc = "External trigger conversion mode for regular channels\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXTTRIG {
    #[doc = "0: Conversion on external event disabled"]
    Disabled = 0,
    #[doc = "1: Conversion on external event enabled"]
    Enabled = 1,
}
impl From<EXTTRIG> for bool {
    #[inline(always)]
    fn from(variant: EXTTRIG) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTTRIG` reader - External trigger conversion mode for regular channels"]
pub type EXTTRIG_R = crate::BitReader<EXTTRIG>;
impl EXTTRIG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EXTTRIG {
        match self.bits {
            false => EXTTRIG::Disabled,
            true => EXTTRIG::Enabled,
        }
    }
    #[doc = "Conversion on external event disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EXTTRIG::Disabled
    }
    #[doc = "Conversion on external event enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EXTTRIG::Enabled
    }
}
#[doc = "Field `EXTTRIG` writer - External trigger conversion mode for regular channels"]
pub type EXTTRIG_W<'a, REG> = crate::BitWriter<'a, REG, EXTTRIG>;
impl<'a, REG> EXTTRIG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Conversion on external event disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EXTTRIG::Disabled)
    }
    #[doc = "Conversion on external event enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EXTTRIG::Enabled)
    }
}
#[doc = "Start conversion of injected channels\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JSWSTARTR {
    #[doc = "0: Reset state"]
    Started = 0,
    #[doc = "1: Starting conversion of injected channels"]
    NotStarted = 1,
}
impl From<JSWSTARTR> for bool {
    #[inline(always)]
    fn from(variant: JSWSTARTR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JSWSTART` reader - Start conversion of injected channels"]
pub type JSWSTART_R = crate::BitReader<JSWSTARTR>;
impl JSWSTART_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> JSWSTARTR {
        match self.bits {
            false => JSWSTARTR::Started,
            true => JSWSTARTR::NotStarted,
        }
    }
    #[doc = "Reset state"]
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        *self == JSWSTARTR::Started
    }
    #[doc = "Starting conversion of injected channels"]
    #[inline(always)]
    pub fn is_not_started(&self) -> bool {
        *self == JSWSTARTR::NotStarted
    }
}
#[doc = "Start conversion of injected channels\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JSWSTARTW {
    #[doc = "1: Start conversion of injected channels"]
    Start = 1,
}
impl From<JSWSTARTW> for bool {
    #[inline(always)]
    fn from(variant: JSWSTARTW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JSWSTART` writer - Start conversion of injected channels"]
pub type JSWSTART_W<'a, REG> = crate::BitWriter<'a, REG, JSWSTARTW>;
impl<'a, REG> JSWSTART_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Start conversion of injected channels"]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(JSWSTARTW::Start)
    }
}
#[doc = "Start conversion of regular channels\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWSTARTR {
    #[doc = "0: Reset state"]
    Started = 0,
    #[doc = "1: Starting conversion of regular channels"]
    NotStarted = 1,
}
impl From<SWSTARTR> for bool {
    #[inline(always)]
    fn from(variant: SWSTARTR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWSTART` reader - Start conversion of regular channels"]
pub type SWSTART_R = crate::BitReader<SWSTARTR>;
impl SWSTART_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SWSTARTR {
        match self.bits {
            false => SWSTARTR::Started,
            true => SWSTARTR::NotStarted,
        }
    }
    #[doc = "Reset state"]
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        *self == SWSTARTR::Started
    }
    #[doc = "Starting conversion of regular channels"]
    #[inline(always)]
    pub fn is_not_started(&self) -> bool {
        *self == SWSTARTR::NotStarted
    }
}
#[doc = "Start conversion of regular channels\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWSTARTW {
    #[doc = "1: Start conversion of regular channels"]
    Start = 1,
}
impl From<SWSTARTW> for bool {
    #[inline(always)]
    fn from(variant: SWSTARTW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWSTART` writer - Start conversion of regular channels"]
pub type SWSTART_W<'a, REG> = crate::BitWriter<'a, REG, SWSTARTW>;
impl<'a, REG> SWSTART_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Start conversion of regular channels"]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(SWSTARTW::Start)
    }
}
#[doc = "Temperature sensor and VREFINT enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSVREFE {
    #[doc = "0: Temperature sensor and V_REFINT channel disabled"]
    Disabled = 0,
    #[doc = "1: Temperature sensor and V_REFINT channel enabled"]
    Enabled = 1,
}
impl From<TSVREFE> for bool {
    #[inline(always)]
    fn from(variant: TSVREFE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSVREFE` reader - Temperature sensor and VREFINT enable"]
pub type TSVREFE_R = crate::BitReader<TSVREFE>;
impl TSVREFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TSVREFE {
        match self.bits {
            false => TSVREFE::Disabled,
            true => TSVREFE::Enabled,
        }
    }
    #[doc = "Temperature sensor and V_REFINT channel disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TSVREFE::Disabled
    }
    #[doc = "Temperature sensor and V_REFINT channel enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TSVREFE::Enabled
    }
}
#[doc = "Field `TSVREFE` writer - Temperature sensor and VREFINT enable"]
pub type TSVREFE_W<'a, REG> = crate::BitWriter<'a, REG, TSVREFE>;
impl<'a, REG> TSVREFE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Temperature sensor and V_REFINT channel disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TSVREFE::Disabled)
    }
    #[doc = "Temperature sensor and V_REFINT channel enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TSVREFE::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - A/D converter ON / OFF"]
    #[inline(always)]
    pub fn adon(&self) -> ADON_R {
        ADON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Continuous conversion"]
    #[inline(always)]
    pub fn cont(&self) -> CONT_R {
        CONT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - A/D calibration"]
    #[inline(always)]
    pub fn cal(&self) -> CAL_R {
        CAL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reset calibration"]
    #[inline(always)]
    pub fn rstcal(&self) -> RSTCAL_R {
        RSTCAL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Direct memory access mode"]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - Data alignment"]
    #[inline(always)]
    pub fn align(&self) -> ALIGN_R {
        ALIGN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - External event select for injected group"]
    #[inline(always)]
    pub fn jextsel(&self) -> JEXTSEL_R {
        JEXTSEL_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - External trigger conversion mode for injected channels"]
    #[inline(always)]
    pub fn jexttrig(&self) -> JEXTTRIG_R {
        JEXTTRIG_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 17:19 - External event select for regular group"]
    #[inline(always)]
    pub fn extsel(&self) -> EXTSEL_R {
        EXTSEL_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bit 20 - External trigger conversion mode for regular channels"]
    #[inline(always)]
    pub fn exttrig(&self) -> EXTTRIG_R {
        EXTTRIG_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Start conversion of injected channels"]
    #[inline(always)]
    pub fn jswstart(&self) -> JSWSTART_R {
        JSWSTART_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Start conversion of regular channels"]
    #[inline(always)]
    pub fn swstart(&self) -> SWSTART_R {
        SWSTART_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Temperature sensor and VREFINT enable"]
    #[inline(always)]
    pub fn tsvrefe(&self) -> TSVREFE_R {
        TSVREFE_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - A/D converter ON / OFF"]
    #[inline(always)]
    #[must_use]
    pub fn adon(&mut self) -> ADON_W<CR2rs> {
        ADON_W::new(self, 0)
    }
    #[doc = "Bit 1 - Continuous conversion"]
    #[inline(always)]
    #[must_use]
    pub fn cont(&mut self) -> CONT_W<CR2rs> {
        CONT_W::new(self, 1)
    }
    #[doc = "Bit 2 - A/D calibration"]
    #[inline(always)]
    #[must_use]
    pub fn cal(&mut self) -> CAL_W<CR2rs> {
        CAL_W::new(self, 2)
    }
    #[doc = "Bit 3 - Reset calibration"]
    #[inline(always)]
    #[must_use]
    pub fn rstcal(&mut self) -> RSTCAL_W<CR2rs> {
        RSTCAL_W::new(self, 3)
    }
    #[doc = "Bit 8 - Direct memory access mode"]
    #[inline(always)]
    #[must_use]
    pub fn dma(&mut self) -> DMA_W<CR2rs> {
        DMA_W::new(self, 8)
    }
    #[doc = "Bit 11 - Data alignment"]
    #[inline(always)]
    #[must_use]
    pub fn align(&mut self) -> ALIGN_W<CR2rs> {
        ALIGN_W::new(self, 11)
    }
    #[doc = "Bits 12:14 - External event select for injected group"]
    #[inline(always)]
    #[must_use]
    pub fn jextsel(&mut self) -> JEXTSEL_W<CR2rs> {
        JEXTSEL_W::new(self, 12)
    }
    #[doc = "Bit 15 - External trigger conversion mode for injected channels"]
    #[inline(always)]
    #[must_use]
    pub fn jexttrig(&mut self) -> JEXTTRIG_W<CR2rs> {
        JEXTTRIG_W::new(self, 15)
    }
    #[doc = "Bits 17:19 - External event select for regular group"]
    #[inline(always)]
    #[must_use]
    pub fn extsel(&mut self) -> EXTSEL_W<CR2rs> {
        EXTSEL_W::new(self, 17)
    }
    #[doc = "Bit 20 - External trigger conversion mode for regular channels"]
    #[inline(always)]
    #[must_use]
    pub fn exttrig(&mut self) -> EXTTRIG_W<CR2rs> {
        EXTTRIG_W::new(self, 20)
    }
    #[doc = "Bit 21 - Start conversion of injected channels"]
    #[inline(always)]
    #[must_use]
    pub fn jswstart(&mut self) -> JSWSTART_W<CR2rs> {
        JSWSTART_W::new(self, 21)
    }
    #[doc = "Bit 22 - Start conversion of regular channels"]
    #[inline(always)]
    #[must_use]
    pub fn swstart(&mut self) -> SWSTART_W<CR2rs> {
        SWSTART_W::new(self, 22)
    }
    #[doc = "Bit 23 - Temperature sensor and VREFINT enable"]
    #[inline(always)]
    #[must_use]
    pub fn tsvrefe(&mut self) -> TSVREFE_W<CR2rs> {
        TSVREFE_W::new(self, 23)
    }
}
#[doc = "control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR2rs;
impl crate::RegisterSpec for CR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for CR2rs {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for CR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for CR2rs {
    const RESET_VALUE: u32 = 0;
}
