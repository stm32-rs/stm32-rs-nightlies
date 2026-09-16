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
    ///0: Timer 1 CC4 event
    Tim1cc4 = 0,
    ///1: Timer 1 TRGO event
    Tim1trgo = 1,
    ///2: Timer 2 CC1 event
    Tim2cc1 = 2,
    ///3: Timer 2 TRGO event
    Tim2trgo = 3,
    ///4: Timer 3 CC2 event
    Tim3cc2 = 4,
    ///5: Timer 3 CC4 event
    Tim3cc4 = 5,
    ///6: Timer 4 CC1 event
    Tim4cc1 = 6,
    ///7: Timer 4 CC2 event
    Tim4cc2 = 7,
    ///8: Timer 4 CC3 event
    Tim4cc3 = 8,
    ///9: Timer 4 TRGO event
    Tim4trgo = 9,
    ///10: Timer 5 CC4 event
    Tim5cc4 = 10,
    ///11: Timer 5 TRGO event
    Tim5trgo = 11,
    ///12: Timer 8 CC2 event
    Tim8cc2 = 12,
    ///13: Timer 8 CC3 event
    Tim8cc3 = 13,
    ///14: Timer 8 CC4 event
    Tim8cc4 = 14,
    ///15: EXTI line 15
    Exti15 = 15,
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
            0 => JEXTSEL::Tim1cc4,
            1 => JEXTSEL::Tim1trgo,
            2 => JEXTSEL::Tim2cc1,
            3 => JEXTSEL::Tim2trgo,
            4 => JEXTSEL::Tim3cc2,
            5 => JEXTSEL::Tim3cc4,
            6 => JEXTSEL::Tim4cc1,
            7 => JEXTSEL::Tim4cc2,
            8 => JEXTSEL::Tim4cc3,
            9 => JEXTSEL::Tim4trgo,
            10 => JEXTSEL::Tim5cc4,
            11 => JEXTSEL::Tim5trgo,
            12 => JEXTSEL::Tim8cc2,
            13 => JEXTSEL::Tim8cc3,
            14 => JEXTSEL::Tim8cc4,
            15 => JEXTSEL::Exti15,
            _ => unreachable!(),
        }
    }
    ///Timer 1 CC4 event
    #[inline(always)]
    pub fn is_tim1cc4(&self) -> bool {
        *self == JEXTSEL::Tim1cc4
    }
    ///Timer 1 TRGO event
    #[inline(always)]
    pub fn is_tim1trgo(&self) -> bool {
        *self == JEXTSEL::Tim1trgo
    }
    ///Timer 2 CC1 event
    #[inline(always)]
    pub fn is_tim2cc1(&self) -> bool {
        *self == JEXTSEL::Tim2cc1
    }
    ///Timer 2 TRGO event
    #[inline(always)]
    pub fn is_tim2trgo(&self) -> bool {
        *self == JEXTSEL::Tim2trgo
    }
    ///Timer 3 CC2 event
    #[inline(always)]
    pub fn is_tim3cc2(&self) -> bool {
        *self == JEXTSEL::Tim3cc2
    }
    ///Timer 3 CC4 event
    #[inline(always)]
    pub fn is_tim3cc4(&self) -> bool {
        *self == JEXTSEL::Tim3cc4
    }
    ///Timer 4 CC1 event
    #[inline(always)]
    pub fn is_tim4cc1(&self) -> bool {
        *self == JEXTSEL::Tim4cc1
    }
    ///Timer 4 CC2 event
    #[inline(always)]
    pub fn is_tim4cc2(&self) -> bool {
        *self == JEXTSEL::Tim4cc2
    }
    ///Timer 4 CC3 event
    #[inline(always)]
    pub fn is_tim4cc3(&self) -> bool {
        *self == JEXTSEL::Tim4cc3
    }
    ///Timer 4 TRGO event
    #[inline(always)]
    pub fn is_tim4trgo(&self) -> bool {
        *self == JEXTSEL::Tim4trgo
    }
    ///Timer 5 CC4 event
    #[inline(always)]
    pub fn is_tim5cc4(&self) -> bool {
        *self == JEXTSEL::Tim5cc4
    }
    ///Timer 5 TRGO event
    #[inline(always)]
    pub fn is_tim5trgo(&self) -> bool {
        *self == JEXTSEL::Tim5trgo
    }
    ///Timer 8 CC2 event
    #[inline(always)]
    pub fn is_tim8cc2(&self) -> bool {
        *self == JEXTSEL::Tim8cc2
    }
    ///Timer 8 CC3 event
    #[inline(always)]
    pub fn is_tim8cc3(&self) -> bool {
        *self == JEXTSEL::Tim8cc3
    }
    ///Timer 8 CC4 event
    #[inline(always)]
    pub fn is_tim8cc4(&self) -> bool {
        *self == JEXTSEL::Tim8cc4
    }
    ///EXTI line 15
    #[inline(always)]
    pub fn is_exti15(&self) -> bool {
        *self == JEXTSEL::Exti15
    }
}
///Field `JEXTSEL` writer - External event select for injected group
pub type JEXTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4, JEXTSEL, crate::Safe>;
impl<'a, REG> JEXTSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Timer 1 CC4 event
    #[inline(always)]
    pub fn tim1cc4(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim1cc4)
    }
    ///Timer 1 TRGO event
    #[inline(always)]
    pub fn tim1trgo(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim1trgo)
    }
    ///Timer 2 CC1 event
    #[inline(always)]
    pub fn tim2cc1(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim2cc1)
    }
    ///Timer 2 TRGO event
    #[inline(always)]
    pub fn tim2trgo(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim2trgo)
    }
    ///Timer 3 CC2 event
    #[inline(always)]
    pub fn tim3cc2(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim3cc2)
    }
    ///Timer 3 CC4 event
    #[inline(always)]
    pub fn tim3cc4(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim3cc4)
    }
    ///Timer 4 CC1 event
    #[inline(always)]
    pub fn tim4cc1(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim4cc1)
    }
    ///Timer 4 CC2 event
    #[inline(always)]
    pub fn tim4cc2(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim4cc2)
    }
    ///Timer 4 CC3 event
    #[inline(always)]
    pub fn tim4cc3(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim4cc3)
    }
    ///Timer 4 TRGO event
    #[inline(always)]
    pub fn tim4trgo(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim4trgo)
    }
    ///Timer 5 CC4 event
    #[inline(always)]
    pub fn tim5cc4(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim5cc4)
    }
    ///Timer 5 TRGO event
    #[inline(always)]
    pub fn tim5trgo(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim5trgo)
    }
    ///Timer 8 CC2 event
    #[inline(always)]
    pub fn tim8cc2(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim8cc2)
    }
    ///Timer 8 CC3 event
    #[inline(always)]
    pub fn tim8cc3(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim8cc3)
    }
    ///Timer 8 CC4 event
    #[inline(always)]
    pub fn tim8cc4(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim8cc4)
    }
    ///EXTI line 15
    #[inline(always)]
    pub fn exti15(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Exti15)
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
    ///0: Timer 1 CC1 event
    Tim1cc1 = 0,
    ///1: Timer 1 CC2 event
    Tim1cc2 = 1,
    ///2: Timer 1 CC3 event
    Tim1cc3 = 2,
    ///3: Timer 2 CC2 event
    Tim2cc2 = 3,
    ///4: Timer 2 CC3 event
    Tim2cc3 = 4,
    ///5: Timer 2 CC4 event
    Tim2cc4 = 5,
    ///6: Timer 2 TRGO event
    Tim2trgo = 6,
    ///7: Timer 3 CC1 event
    Tim3cc1 = 7,
    ///8: Timer 3 TRGO event
    Tim3trgo = 8,
    ///9: Timer 4 CC4 event
    Tim4cc4 = 9,
    ///10: Timer 5 CC1 event
    Tim5cc1 = 10,
    ///11: Timer 5 CC2 event
    Tim5cc2 = 11,
    ///12: Timer 5 CC3 event
    Tim5cc3 = 12,
    ///13: Timer 8 CC1 event
    Tim8cc1 = 13,
    ///14: Timer 8 TRGO event
    Tim8trgo = 14,
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
    pub const fn variant(&self) -> EXTSEL {
        match self.bits {
            0 => EXTSEL::Tim1cc1,
            1 => EXTSEL::Tim1cc2,
            2 => EXTSEL::Tim1cc3,
            3 => EXTSEL::Tim2cc2,
            4 => EXTSEL::Tim2cc3,
            5 => EXTSEL::Tim2cc4,
            6 => EXTSEL::Tim2trgo,
            7 => EXTSEL::Tim3cc1,
            8 => EXTSEL::Tim3trgo,
            9 => EXTSEL::Tim4cc4,
            10 => EXTSEL::Tim5cc1,
            11 => EXTSEL::Tim5cc2,
            12 => EXTSEL::Tim5cc3,
            13 => EXTSEL::Tim8cc1,
            14 => EXTSEL::Tim8trgo,
            15 => EXTSEL::Exti11,
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
    ///Timer 2 CC3 event
    #[inline(always)]
    pub fn is_tim2cc3(&self) -> bool {
        *self == EXTSEL::Tim2cc3
    }
    ///Timer 2 CC4 event
    #[inline(always)]
    pub fn is_tim2cc4(&self) -> bool {
        *self == EXTSEL::Tim2cc4
    }
    ///Timer 2 TRGO event
    #[inline(always)]
    pub fn is_tim2trgo(&self) -> bool {
        *self == EXTSEL::Tim2trgo
    }
    ///Timer 3 CC1 event
    #[inline(always)]
    pub fn is_tim3cc1(&self) -> bool {
        *self == EXTSEL::Tim3cc1
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
    ///Timer 5 CC1 event
    #[inline(always)]
    pub fn is_tim5cc1(&self) -> bool {
        *self == EXTSEL::Tim5cc1
    }
    ///Timer 5 CC2 event
    #[inline(always)]
    pub fn is_tim5cc2(&self) -> bool {
        *self == EXTSEL::Tim5cc2
    }
    ///Timer 5 CC3 event
    #[inline(always)]
    pub fn is_tim5cc3(&self) -> bool {
        *self == EXTSEL::Tim5cc3
    }
    ///Timer 8 CC1 event
    #[inline(always)]
    pub fn is_tim8cc1(&self) -> bool {
        *self == EXTSEL::Tim8cc1
    }
    ///Timer 8 TRGO event
    #[inline(always)]
    pub fn is_tim8trgo(&self) -> bool {
        *self == EXTSEL::Tim8trgo
    }
    ///EXTI line 11
    #[inline(always)]
    pub fn is_exti11(&self) -> bool {
        *self == EXTSEL::Exti11
    }
}
///Field `EXTSEL` writer - External event select for regular group
pub type EXTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4, EXTSEL, crate::Safe>;
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
    ///Timer 2 CC3 event
    #[inline(always)]
    pub fn tim2cc3(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim2cc3)
    }
    ///Timer 2 CC4 event
    #[inline(always)]
    pub fn tim2cc4(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim2cc4)
    }
    ///Timer 2 TRGO event
    #[inline(always)]
    pub fn tim2trgo(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim2trgo)
    }
    ///Timer 3 CC1 event
    #[inline(always)]
    pub fn tim3cc1(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim3cc1)
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
    ///Timer 5 CC1 event
    #[inline(always)]
    pub fn tim5cc1(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim5cc1)
    }
    ///Timer 5 CC2 event
    #[inline(always)]
    pub fn tim5cc2(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim5cc2)
    }
    ///Timer 5 CC3 event
    #[inline(always)]
    pub fn tim5cc3(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim5cc3)
    }
    ///Timer 8 CC1 event
    #[inline(always)]
    pub fn tim8cc1(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim8cc1)
    }
    ///Timer 8 TRGO event
    #[inline(always)]
    pub fn tim8trgo(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim8trgo)
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

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F217.html#ADC1:CR2)*/
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
