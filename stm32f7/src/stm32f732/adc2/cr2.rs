///Register `CR2` reader
pub type R = crate::R<CR2rs>;
///Register `CR2` writer
pub type W = crate::W<CR2rs>;
/**A/D Converter ON / OFF

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADON {
    ///0: Disable ADC conversion and go to power down mode
    Disabled = 0,
    ///1: Enable ADC
    Enabled = 1,
}
impl From<ADON> for bool {
    #[inline(always)]
    fn from(variant: ADON) -> Self {
        variant as u8 != 0
    }
}
///Field `ADON` reader - A/D Converter ON / OFF
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
    ///Disable ADC conversion and go to power down mode
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADON::Disabled
    }
    ///Enable ADC
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADON::Enabled
    }
}
///Field `ADON` writer - A/D Converter ON / OFF
pub type ADON_W<'a, REG> = crate::BitWriter<'a, REG, ADON>;
impl<'a, REG> ADON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable ADC conversion and go to power down mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ADON::Disabled)
    }
    ///Enable ADC
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
/**Direct memory access mode (for single ADC mode)

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
///Field `DMA` reader - Direct memory access mode (for single ADC mode)
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
///Field `DMA` writer - Direct memory access mode (for single ADC mode)
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
/**DMA disable selection (for single ADC mode)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DDS {
    ///0: No new DMA request is issued after the last transfer
    Single = 0,
    ///1: DMA requests are issued as long as data are converted and DMA=1
    Continuous = 1,
}
impl From<DDS> for bool {
    #[inline(always)]
    fn from(variant: DDS) -> Self {
        variant as u8 != 0
    }
}
///Field `DDS` reader - DMA disable selection (for single ADC mode)
pub type DDS_R = crate::BitReader<DDS>;
impl DDS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DDS {
        match self.bits {
            false => DDS::Single,
            true => DDS::Continuous,
        }
    }
    ///No new DMA request is issued after the last transfer
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == DDS::Single
    }
    ///DMA requests are issued as long as data are converted and DMA=1
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == DDS::Continuous
    }
}
///Field `DDS` writer - DMA disable selection (for single ADC mode)
pub type DDS_W<'a, REG> = crate::BitWriter<'a, REG, DDS>;
impl<'a, REG> DDS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No new DMA request is issued after the last transfer
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(DDS::Single)
    }
    ///DMA requests are issued as long as data are converted and DMA=1
    #[inline(always)]
    pub fn continuous(self) -> &'a mut crate::W<REG> {
        self.variant(DDS::Continuous)
    }
}
/**End of conversion selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOCS {
    ///0: The EOC bit is set at the end of each sequence of regular conversions
    EachSequence = 0,
    ///1: The EOC bit is set at the end of each regular conversion
    EachConversion = 1,
}
impl From<EOCS> for bool {
    #[inline(always)]
    fn from(variant: EOCS) -> Self {
        variant as u8 != 0
    }
}
///Field `EOCS` reader - End of conversion selection
pub type EOCS_R = crate::BitReader<EOCS>;
impl EOCS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EOCS {
        match self.bits {
            false => EOCS::EachSequence,
            true => EOCS::EachConversion,
        }
    }
    ///The EOC bit is set at the end of each sequence of regular conversions
    #[inline(always)]
    pub fn is_each_sequence(&self) -> bool {
        *self == EOCS::EachSequence
    }
    ///The EOC bit is set at the end of each regular conversion
    #[inline(always)]
    pub fn is_each_conversion(&self) -> bool {
        *self == EOCS::EachConversion
    }
}
///Field `EOCS` writer - End of conversion selection
pub type EOCS_W<'a, REG> = crate::BitWriter<'a, REG, EOCS>;
impl<'a, REG> EOCS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The EOC bit is set at the end of each sequence of regular conversions
    #[inline(always)]
    pub fn each_sequence(self) -> &'a mut crate::W<REG> {
        self.variant(EOCS::EachSequence)
    }
    ///The EOC bit is set at the end of each regular conversion
    #[inline(always)]
    pub fn each_conversion(self) -> &'a mut crate::W<REG> {
        self.variant(EOCS::EachConversion)
    }
}
/**Data alignment

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALIGN {
    ///0: Right alignment
    Right = 0,
    ///1: Left alignment
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
    ///Right alignment
    #[inline(always)]
    pub fn is_right(&self) -> bool {
        *self == ALIGN::Right
    }
    ///Left alignment
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
    ///Right alignment
    #[inline(always)]
    pub fn right(self) -> &'a mut crate::W<REG> {
        self.variant(ALIGN::Right)
    }
    ///Left alignment
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
    ///0: Timer 1 TRGO
    Tim1trgo = 0,
    ///1: Timer 1 CH4
    Tim1ch4 = 1,
    ///2: Timer 2 TRGO
    Tim2trgo = 2,
    ///3: Timer 2 CH1
    Tim2ch1 = 3,
    ///4: Timer 3 CH4
    Tim3ch4 = 4,
    ///5: Timer 4 TRGO
    Tim4trgo = 5,
    ///7: Timer 8 CH4
    Tim8ch4 = 7,
    ///8: Timer 1 TRGO(2)
    Tim1trgo2 = 8,
    ///9: Timer 8 TRGO
    Tim8trgo = 9,
    ///10: Timer 8 TRGO(2)
    Tim8trgo2 = 10,
    ///11: Timer 3 CH3
    Tim3ch3 = 11,
    ///12: Timer 5 TRGO
    Tim5trgo = 12,
    ///13: Timer 3 CH1
    Tim3ch1 = 13,
    ///14: Timer 6 TRGO
    Tim6trgo = 14,
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
    pub const fn variant(&self) -> Option<JEXTSEL> {
        match self.bits {
            0 => Some(JEXTSEL::Tim1trgo),
            1 => Some(JEXTSEL::Tim1ch4),
            2 => Some(JEXTSEL::Tim2trgo),
            3 => Some(JEXTSEL::Tim2ch1),
            4 => Some(JEXTSEL::Tim3ch4),
            5 => Some(JEXTSEL::Tim4trgo),
            7 => Some(JEXTSEL::Tim8ch4),
            8 => Some(JEXTSEL::Tim1trgo2),
            9 => Some(JEXTSEL::Tim8trgo),
            10 => Some(JEXTSEL::Tim8trgo2),
            11 => Some(JEXTSEL::Tim3ch3),
            12 => Some(JEXTSEL::Tim5trgo),
            13 => Some(JEXTSEL::Tim3ch1),
            14 => Some(JEXTSEL::Tim6trgo),
            _ => None,
        }
    }
    ///Timer 1 TRGO
    #[inline(always)]
    pub fn is_tim1trgo(&self) -> bool {
        *self == JEXTSEL::Tim1trgo
    }
    ///Timer 1 CH4
    #[inline(always)]
    pub fn is_tim1ch4(&self) -> bool {
        *self == JEXTSEL::Tim1ch4
    }
    ///Timer 2 TRGO
    #[inline(always)]
    pub fn is_tim2trgo(&self) -> bool {
        *self == JEXTSEL::Tim2trgo
    }
    ///Timer 2 CH1
    #[inline(always)]
    pub fn is_tim2ch1(&self) -> bool {
        *self == JEXTSEL::Tim2ch1
    }
    ///Timer 3 CH4
    #[inline(always)]
    pub fn is_tim3ch4(&self) -> bool {
        *self == JEXTSEL::Tim3ch4
    }
    ///Timer 4 TRGO
    #[inline(always)]
    pub fn is_tim4trgo(&self) -> bool {
        *self == JEXTSEL::Tim4trgo
    }
    ///Timer 8 CH4
    #[inline(always)]
    pub fn is_tim8ch4(&self) -> bool {
        *self == JEXTSEL::Tim8ch4
    }
    ///Timer 1 TRGO(2)
    #[inline(always)]
    pub fn is_tim1trgo2(&self) -> bool {
        *self == JEXTSEL::Tim1trgo2
    }
    ///Timer 8 TRGO
    #[inline(always)]
    pub fn is_tim8trgo(&self) -> bool {
        *self == JEXTSEL::Tim8trgo
    }
    ///Timer 8 TRGO(2)
    #[inline(always)]
    pub fn is_tim8trgo2(&self) -> bool {
        *self == JEXTSEL::Tim8trgo2
    }
    ///Timer 3 CH3
    #[inline(always)]
    pub fn is_tim3ch3(&self) -> bool {
        *self == JEXTSEL::Tim3ch3
    }
    ///Timer 5 TRGO
    #[inline(always)]
    pub fn is_tim5trgo(&self) -> bool {
        *self == JEXTSEL::Tim5trgo
    }
    ///Timer 3 CH1
    #[inline(always)]
    pub fn is_tim3ch1(&self) -> bool {
        *self == JEXTSEL::Tim3ch1
    }
    ///Timer 6 TRGO
    #[inline(always)]
    pub fn is_tim6trgo(&self) -> bool {
        *self == JEXTSEL::Tim6trgo
    }
}
///Field `JEXTSEL` writer - External event select for injected group
pub type JEXTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4, JEXTSEL>;
impl<'a, REG> JEXTSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Timer 1 TRGO
    #[inline(always)]
    pub fn tim1trgo(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim1trgo)
    }
    ///Timer 1 CH4
    #[inline(always)]
    pub fn tim1ch4(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim1ch4)
    }
    ///Timer 2 TRGO
    #[inline(always)]
    pub fn tim2trgo(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim2trgo)
    }
    ///Timer 2 CH1
    #[inline(always)]
    pub fn tim2ch1(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim2ch1)
    }
    ///Timer 3 CH4
    #[inline(always)]
    pub fn tim3ch4(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim3ch4)
    }
    ///Timer 4 TRGO
    #[inline(always)]
    pub fn tim4trgo(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim4trgo)
    }
    ///Timer 8 CH4
    #[inline(always)]
    pub fn tim8ch4(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim8ch4)
    }
    ///Timer 1 TRGO(2)
    #[inline(always)]
    pub fn tim1trgo2(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim1trgo2)
    }
    ///Timer 8 TRGO
    #[inline(always)]
    pub fn tim8trgo(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim8trgo)
    }
    ///Timer 8 TRGO(2)
    #[inline(always)]
    pub fn tim8trgo2(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim8trgo2)
    }
    ///Timer 3 CH3
    #[inline(always)]
    pub fn tim3ch3(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim3ch3)
    }
    ///Timer 5 TRGO
    #[inline(always)]
    pub fn tim5trgo(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim5trgo)
    }
    ///Timer 3 CH1
    #[inline(always)]
    pub fn tim3ch1(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim3ch1)
    }
    ///Timer 6 TRGO
    #[inline(always)]
    pub fn tim6trgo(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim6trgo)
    }
}
/**External trigger enable for injected channels

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum JEXTEN {
    ///0: Trigger detection disabled
    Disabled = 0,
    ///1: Trigger detection on the rising edge
    RisingEdge = 1,
    ///2: Trigger detection on the falling edge
    FallingEdge = 2,
    ///3: Trigger detection on both the rising and falling edges
    BothEdges = 3,
}
impl From<JEXTEN> for u8 {
    #[inline(always)]
    fn from(variant: JEXTEN) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for JEXTEN {
    type Ux = u8;
}
impl crate::IsEnum for JEXTEN {}
///Field `JEXTEN` reader - External trigger enable for injected channels
pub type JEXTEN_R = crate::FieldReader<JEXTEN>;
impl JEXTEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> JEXTEN {
        match self.bits {
            0 => JEXTEN::Disabled,
            1 => JEXTEN::RisingEdge,
            2 => JEXTEN::FallingEdge,
            3 => JEXTEN::BothEdges,
            _ => unreachable!(),
        }
    }
    ///Trigger detection disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JEXTEN::Disabled
    }
    ///Trigger detection on the rising edge
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == JEXTEN::RisingEdge
    }
    ///Trigger detection on the falling edge
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == JEXTEN::FallingEdge
    }
    ///Trigger detection on both the rising and falling edges
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        *self == JEXTEN::BothEdges
    }
}
///Field `JEXTEN` writer - External trigger enable for injected channels
pub type JEXTEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2, JEXTEN, crate::Safe>;
impl<'a, REG> JEXTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Trigger detection disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTEN::Disabled)
    }
    ///Trigger detection on the rising edge
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTEN::RisingEdge)
    }
    ///Trigger detection on the falling edge
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTEN::FallingEdge)
    }
    ///Trigger detection on both the rising and falling edges
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTEN::BothEdges)
    }
}
/**Start conversion of injected channels

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JSWSTARTW {
    ///1: Starts conversion of injected channels
    Start = 1,
}
impl From<JSWSTARTW> for bool {
    #[inline(always)]
    fn from(variant: JSWSTARTW) -> Self {
        variant as u8 != 0
    }
}
///Field `JSWSTART` reader - Start conversion of injected channels
pub type JSWSTART_R = crate::BitReader<JSWSTARTW>;
impl JSWSTART_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<JSWSTARTW> {
        match self.bits {
            true => Some(JSWSTARTW::Start),
            _ => None,
        }
    }
    ///Starts conversion of injected channels
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == JSWSTARTW::Start
    }
}
///Field `JSWSTART` writer - Start conversion of injected channels
pub type JSWSTART_W<'a, REG> = crate::BitWriter<'a, REG, JSWSTARTW>;
impl<'a, REG> JSWSTART_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Starts conversion of injected channels
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(JSWSTARTW::Start)
    }
}
/**External event select for regular group

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTSEL {
    ///0: Timer 1 CH1
    Tim1ch1 = 0,
    ///1: Timer 1 CH2
    Tim1ch2 = 1,
    ///2: Timer 1 CH3
    Tim1ch3 = 2,
    ///3: Timer 2 CH2
    Tim2ch2 = 3,
    ///4: Timer 5 TRGO
    Tim5trgo = 4,
    ///5: Timer 4 CH4
    Tim4ch4 = 5,
    ///6: Timer 3 CH4
    Tim3ch4 = 6,
    ///7: Timer 8 TRGO
    Tim8trgo = 7,
    ///8: Timer 8 TRGO(2)
    Tim8trgo2 = 8,
    ///9: Timer 1 TRGO
    Tim1trgo = 9,
    ///10: Timer 1 TRGO(2)
    Tim1trgo2 = 10,
    ///11: Timer 2 TRGO
    Tim2trgo = 11,
    ///12: Timer 4 TRGO
    Tim4trgo = 12,
    ///13: Timer 6 TRGO
    Tim6trgo = 13,
    ///15: EXTI line 11
    Exti11 = 15,
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
    pub const fn variant(&self) -> Option<EXTSEL> {
        match self.bits {
            0 => Some(EXTSEL::Tim1ch1),
            1 => Some(EXTSEL::Tim1ch2),
            2 => Some(EXTSEL::Tim1ch3),
            3 => Some(EXTSEL::Tim2ch2),
            4 => Some(EXTSEL::Tim5trgo),
            5 => Some(EXTSEL::Tim4ch4),
            6 => Some(EXTSEL::Tim3ch4),
            7 => Some(EXTSEL::Tim8trgo),
            8 => Some(EXTSEL::Tim8trgo2),
            9 => Some(EXTSEL::Tim1trgo),
            10 => Some(EXTSEL::Tim1trgo2),
            11 => Some(EXTSEL::Tim2trgo),
            12 => Some(EXTSEL::Tim4trgo),
            13 => Some(EXTSEL::Tim6trgo),
            15 => Some(EXTSEL::Exti11),
            _ => None,
        }
    }
    ///Timer 1 CH1
    #[inline(always)]
    pub fn is_tim1ch1(&self) -> bool {
        *self == EXTSEL::Tim1ch1
    }
    ///Timer 1 CH2
    #[inline(always)]
    pub fn is_tim1ch2(&self) -> bool {
        *self == EXTSEL::Tim1ch2
    }
    ///Timer 1 CH3
    #[inline(always)]
    pub fn is_tim1ch3(&self) -> bool {
        *self == EXTSEL::Tim1ch3
    }
    ///Timer 2 CH2
    #[inline(always)]
    pub fn is_tim2ch2(&self) -> bool {
        *self == EXTSEL::Tim2ch2
    }
    ///Timer 5 TRGO
    #[inline(always)]
    pub fn is_tim5trgo(&self) -> bool {
        *self == EXTSEL::Tim5trgo
    }
    ///Timer 4 CH4
    #[inline(always)]
    pub fn is_tim4ch4(&self) -> bool {
        *self == EXTSEL::Tim4ch4
    }
    ///Timer 3 CH4
    #[inline(always)]
    pub fn is_tim3ch4(&self) -> bool {
        *self == EXTSEL::Tim3ch4
    }
    ///Timer 8 TRGO
    #[inline(always)]
    pub fn is_tim8trgo(&self) -> bool {
        *self == EXTSEL::Tim8trgo
    }
    ///Timer 8 TRGO(2)
    #[inline(always)]
    pub fn is_tim8trgo2(&self) -> bool {
        *self == EXTSEL::Tim8trgo2
    }
    ///Timer 1 TRGO
    #[inline(always)]
    pub fn is_tim1trgo(&self) -> bool {
        *self == EXTSEL::Tim1trgo
    }
    ///Timer 1 TRGO(2)
    #[inline(always)]
    pub fn is_tim1trgo2(&self) -> bool {
        *self == EXTSEL::Tim1trgo2
    }
    ///Timer 2 TRGO
    #[inline(always)]
    pub fn is_tim2trgo(&self) -> bool {
        *self == EXTSEL::Tim2trgo
    }
    ///Timer 4 TRGO
    #[inline(always)]
    pub fn is_tim4trgo(&self) -> bool {
        *self == EXTSEL::Tim4trgo
    }
    ///Timer 6 TRGO
    #[inline(always)]
    pub fn is_tim6trgo(&self) -> bool {
        *self == EXTSEL::Tim6trgo
    }
    ///EXTI line 11
    #[inline(always)]
    pub fn is_exti11(&self) -> bool {
        *self == EXTSEL::Exti11
    }
}
///Field `EXTSEL` writer - External event select for regular group
pub type EXTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4, EXTSEL>;
impl<'a, REG> EXTSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Timer 1 CH1
    #[inline(always)]
    pub fn tim1ch1(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim1ch1)
    }
    ///Timer 1 CH2
    #[inline(always)]
    pub fn tim1ch2(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim1ch2)
    }
    ///Timer 1 CH3
    #[inline(always)]
    pub fn tim1ch3(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim1ch3)
    }
    ///Timer 2 CH2
    #[inline(always)]
    pub fn tim2ch2(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim2ch2)
    }
    ///Timer 5 TRGO
    #[inline(always)]
    pub fn tim5trgo(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim5trgo)
    }
    ///Timer 4 CH4
    #[inline(always)]
    pub fn tim4ch4(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim4ch4)
    }
    ///Timer 3 CH4
    #[inline(always)]
    pub fn tim3ch4(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim3ch4)
    }
    ///Timer 8 TRGO
    #[inline(always)]
    pub fn tim8trgo(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim8trgo)
    }
    ///Timer 8 TRGO(2)
    #[inline(always)]
    pub fn tim8trgo2(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim8trgo2)
    }
    ///Timer 1 TRGO
    #[inline(always)]
    pub fn tim1trgo(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim1trgo)
    }
    ///Timer 1 TRGO(2)
    #[inline(always)]
    pub fn tim1trgo2(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim1trgo2)
    }
    ///Timer 2 TRGO
    #[inline(always)]
    pub fn tim2trgo(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim2trgo)
    }
    ///Timer 4 TRGO
    #[inline(always)]
    pub fn tim4trgo(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim4trgo)
    }
    ///Timer 6 TRGO
    #[inline(always)]
    pub fn tim6trgo(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim6trgo)
    }
    ///EXTI line 11
    #[inline(always)]
    pub fn exti11(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Exti11)
    }
}
/**External trigger enable for regular channels

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTEN {
    ///0: Trigger detection disabled
    Disabled = 0,
    ///1: Trigger detection on the rising edge
    RisingEdge = 1,
    ///2: Trigger detection on the falling edge
    FallingEdge = 2,
    ///3: Trigger detection on both the rising and falling edges
    BothEdges = 3,
}
impl From<EXTEN> for u8 {
    #[inline(always)]
    fn from(variant: EXTEN) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTEN {
    type Ux = u8;
}
impl crate::IsEnum for EXTEN {}
///Field `EXTEN` reader - External trigger enable for regular channels
pub type EXTEN_R = crate::FieldReader<EXTEN>;
impl EXTEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EXTEN {
        match self.bits {
            0 => EXTEN::Disabled,
            1 => EXTEN::RisingEdge,
            2 => EXTEN::FallingEdge,
            3 => EXTEN::BothEdges,
            _ => unreachable!(),
        }
    }
    ///Trigger detection disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EXTEN::Disabled
    }
    ///Trigger detection on the rising edge
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == EXTEN::RisingEdge
    }
    ///Trigger detection on the falling edge
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == EXTEN::FallingEdge
    }
    ///Trigger detection on both the rising and falling edges
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        *self == EXTEN::BothEdges
    }
}
///Field `EXTEN` writer - External trigger enable for regular channels
pub type EXTEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2, EXTEN, crate::Safe>;
impl<'a, REG> EXTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Trigger detection disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EXTEN::Disabled)
    }
    ///Trigger detection on the rising edge
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(EXTEN::RisingEdge)
    }
    ///Trigger detection on the falling edge
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(EXTEN::FallingEdge)
    }
    ///Trigger detection on both the rising and falling edges
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut crate::W<REG> {
        self.variant(EXTEN::BothEdges)
    }
}
/**Start conversion of regular channels

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWSTARTW {
    ///1: Starts conversion of regular channels
    Start = 1,
}
impl From<SWSTARTW> for bool {
    #[inline(always)]
    fn from(variant: SWSTARTW) -> Self {
        variant as u8 != 0
    }
}
///Field `SWSTART` reader - Start conversion of regular channels
pub type SWSTART_R = crate::BitReader<SWSTARTW>;
impl SWSTART_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<SWSTARTW> {
        match self.bits {
            true => Some(SWSTARTW::Start),
            _ => None,
        }
    }
    ///Starts conversion of regular channels
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == SWSTARTW::Start
    }
}
///Field `SWSTART` writer - Start conversion of regular channels
pub type SWSTART_W<'a, REG> = crate::BitWriter<'a, REG, SWSTARTW>;
impl<'a, REG> SWSTART_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Starts conversion of regular channels
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(SWSTARTW::Start)
    }
}
impl R {
    ///Bit 0 - A/D Converter ON / OFF
    #[inline(always)]
    pub fn adon(&self) -> ADON_R {
        ADON_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Continuous conversion
    #[inline(always)]
    pub fn cont(&self) -> CONT_R {
        CONT_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 8 - Direct memory access mode (for single ADC mode)
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - DMA disable selection (for single ADC mode)
    #[inline(always)]
    pub fn dds(&self) -> DDS_R {
        DDS_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - End of conversion selection
    #[inline(always)]
    pub fn eocs(&self) -> EOCS_R {
        EOCS_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Data alignment
    #[inline(always)]
    pub fn align(&self) -> ALIGN_R {
        ALIGN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 16:19 - External event select for injected group
    #[inline(always)]
    pub fn jextsel(&self) -> JEXTSEL_R {
        JEXTSEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:21 - External trigger enable for injected channels
    #[inline(always)]
    pub fn jexten(&self) -> JEXTEN_R {
        JEXTEN_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bit 22 - Start conversion of injected channels
    #[inline(always)]
    pub fn jswstart(&self) -> JSWSTART_R {
        JSWSTART_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bits 24:27 - External event select for regular group
    #[inline(always)]
    pub fn extsel(&self) -> EXTSEL_R {
        EXTSEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:29 - External trigger enable for regular channels
    #[inline(always)]
    pub fn exten(&self) -> EXTEN_R {
        EXTEN_R::new(((self.bits >> 28) & 3) as u8)
    }
    ///Bit 30 - Start conversion of regular channels
    #[inline(always)]
    pub fn swstart(&self) -> SWSTART_R {
        SWSTART_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR2")
            .field("swstart", &self.swstart())
            .field("exten", &self.exten())
            .field("extsel", &self.extsel())
            .field("jswstart", &self.jswstart())
            .field("jexten", &self.jexten())
            .field("jextsel", &self.jextsel())
            .field("align", &self.align())
            .field("eocs", &self.eocs())
            .field("dds", &self.dds())
            .field("dma", &self.dma())
            .field("cont", &self.cont())
            .field("adon", &self.adon())
            .finish()
    }
}
impl W {
    ///Bit 0 - A/D Converter ON / OFF
    #[inline(always)]
    pub fn adon(&mut self) -> ADON_W<'_, CR2rs> {
        ADON_W::new(self, 0)
    }
    ///Bit 1 - Continuous conversion
    #[inline(always)]
    pub fn cont(&mut self) -> CONT_W<'_, CR2rs> {
        CONT_W::new(self, 1)
    }
    ///Bit 8 - Direct memory access mode (for single ADC mode)
    #[inline(always)]
    pub fn dma(&mut self) -> DMA_W<'_, CR2rs> {
        DMA_W::new(self, 8)
    }
    ///Bit 9 - DMA disable selection (for single ADC mode)
    #[inline(always)]
    pub fn dds(&mut self) -> DDS_W<'_, CR2rs> {
        DDS_W::new(self, 9)
    }
    ///Bit 10 - End of conversion selection
    #[inline(always)]
    pub fn eocs(&mut self) -> EOCS_W<'_, CR2rs> {
        EOCS_W::new(self, 10)
    }
    ///Bit 11 - Data alignment
    #[inline(always)]
    pub fn align(&mut self) -> ALIGN_W<'_, CR2rs> {
        ALIGN_W::new(self, 11)
    }
    ///Bits 16:19 - External event select for injected group
    #[inline(always)]
    pub fn jextsel(&mut self) -> JEXTSEL_W<'_, CR2rs> {
        JEXTSEL_W::new(self, 16)
    }
    ///Bits 20:21 - External trigger enable for injected channels
    #[inline(always)]
    pub fn jexten(&mut self) -> JEXTEN_W<'_, CR2rs> {
        JEXTEN_W::new(self, 20)
    }
    ///Bit 22 - Start conversion of injected channels
    #[inline(always)]
    pub fn jswstart(&mut self) -> JSWSTART_W<'_, CR2rs> {
        JSWSTART_W::new(self, 22)
    }
    ///Bits 24:27 - External event select for regular group
    #[inline(always)]
    pub fn extsel(&mut self) -> EXTSEL_W<'_, CR2rs> {
        EXTSEL_W::new(self, 24)
    }
    ///Bits 28:29 - External trigger enable for regular channels
    #[inline(always)]
    pub fn exten(&mut self) -> EXTEN_W<'_, CR2rs> {
        EXTEN_W::new(self, 28)
    }
    ///Bit 30 - Start conversion of regular channels
    #[inline(always)]
    pub fn swstart(&mut self) -> SWSTART_W<'_, CR2rs> {
        SWSTART_W::new(self, 30)
    }
}
/**control register 2

You can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F732.html#ADC2:CR2)*/
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
