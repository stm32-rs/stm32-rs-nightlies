#[doc = "Register `CFGR1` reader"]
pub type R = crate::R<CFGR1rs>;
#[doc = "Register `CFGR1` writer"]
pub type W = crate::W<CFGR1rs>;
#[doc = "Direct memory access enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAEN {
    #[doc = "0: DMA mode disabled"]
    Disabled = 0,
    #[doc = "1: DMA mode enabled"]
    Enabled = 1,
}
impl From<DMAEN> for bool {
    #[inline(always)]
    fn from(variant: DMAEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAEN` reader - Direct memory access enable"]
pub type DMAEN_R = crate::BitReader<DMAEN>;
impl DMAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMAEN {
        match self.bits {
            false => DMAEN::Disabled,
            true => DMAEN::Enabled,
        }
    }
    #[doc = "DMA mode disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAEN::Disabled
    }
    #[doc = "DMA mode enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAEN::Enabled
    }
}
#[doc = "Field `DMAEN` writer - Direct memory access enable"]
pub type DMAEN_W<'a, REG> = crate::BitWriter<'a, REG, DMAEN>;
impl<'a, REG> DMAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAEN::Disabled)
    }
    #[doc = "DMA mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAEN::Enabled)
    }
}
#[doc = "Direct memery access configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMACFG {
    #[doc = "0: DMA one shot mode"]
    OneShot = 0,
    #[doc = "1: DMA circular mode"]
    Circular = 1,
}
impl From<DMACFG> for bool {
    #[inline(always)]
    fn from(variant: DMACFG) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMACFG` reader - Direct memery access configuration"]
pub type DMACFG_R = crate::BitReader<DMACFG>;
impl DMACFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMACFG {
        match self.bits {
            false => DMACFG::OneShot,
            true => DMACFG::Circular,
        }
    }
    #[doc = "DMA one shot mode"]
    #[inline(always)]
    pub fn is_one_shot(&self) -> bool {
        *self == DMACFG::OneShot
    }
    #[doc = "DMA circular mode"]
    #[inline(always)]
    pub fn is_circular(&self) -> bool {
        *self == DMACFG::Circular
    }
}
#[doc = "Field `DMACFG` writer - Direct memery access configuration"]
pub type DMACFG_W<'a, REG> = crate::BitWriter<'a, REG, DMACFG>;
impl<'a, REG> DMACFG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA one shot mode"]
    #[inline(always)]
    pub fn one_shot(self) -> &'a mut crate::W<REG> {
        self.variant(DMACFG::OneShot)
    }
    #[doc = "DMA circular mode"]
    #[inline(always)]
    pub fn circular(self) -> &'a mut crate::W<REG> {
        self.variant(DMACFG::Circular)
    }
}
#[doc = "Scan sequence direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCANDIR {
    #[doc = "0: Upward scan (from CHSEL0 to CHSEL18)"]
    Upward = 0,
    #[doc = "1: Backward scan (from CHSEL18 to CHSEL0)"]
    Backward = 1,
}
impl From<SCANDIR> for bool {
    #[inline(always)]
    fn from(variant: SCANDIR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCANDIR` reader - Scan sequence direction"]
pub type SCANDIR_R = crate::BitReader<SCANDIR>;
impl SCANDIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SCANDIR {
        match self.bits {
            false => SCANDIR::Upward,
            true => SCANDIR::Backward,
        }
    }
    #[doc = "Upward scan (from CHSEL0 to CHSEL18)"]
    #[inline(always)]
    pub fn is_upward(&self) -> bool {
        *self == SCANDIR::Upward
    }
    #[doc = "Backward scan (from CHSEL18 to CHSEL0)"]
    #[inline(always)]
    pub fn is_backward(&self) -> bool {
        *self == SCANDIR::Backward
    }
}
#[doc = "Field `SCANDIR` writer - Scan sequence direction"]
pub type SCANDIR_W<'a, REG> = crate::BitWriter<'a, REG, SCANDIR>;
impl<'a, REG> SCANDIR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Upward scan (from CHSEL0 to CHSEL18)"]
    #[inline(always)]
    pub fn upward(self) -> &'a mut crate::W<REG> {
        self.variant(SCANDIR::Upward)
    }
    #[doc = "Backward scan (from CHSEL18 to CHSEL0)"]
    #[inline(always)]
    pub fn backward(self) -> &'a mut crate::W<REG> {
        self.variant(SCANDIR::Backward)
    }
}
#[doc = "Data resolution\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RES {
    #[doc = "0: 12-bit (14 ADCCLK cycles)"]
    TwelveBit = 0,
    #[doc = "1: 10-bit (13 ADCCLK cycles)"]
    TenBit = 1,
    #[doc = "2: 8-bit (11 ADCCLK cycles)"]
    EightBit = 2,
    #[doc = "3: 6-bit (9 ADCCLK cycles)"]
    SixBit = 3,
}
impl From<RES> for u8 {
    #[inline(always)]
    fn from(variant: RES) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RES {
    type Ux = u8;
}
#[doc = "Field `RES` reader - Data resolution"]
pub type RES_R = crate::FieldReader<RES>;
impl RES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RES {
        match self.bits {
            0 => RES::TwelveBit,
            1 => RES::TenBit,
            2 => RES::EightBit,
            3 => RES::SixBit,
            _ => unreachable!(),
        }
    }
    #[doc = "12-bit (14 ADCCLK cycles)"]
    #[inline(always)]
    pub fn is_twelve_bit(&self) -> bool {
        *self == RES::TwelveBit
    }
    #[doc = "10-bit (13 ADCCLK cycles)"]
    #[inline(always)]
    pub fn is_ten_bit(&self) -> bool {
        *self == RES::TenBit
    }
    #[doc = "8-bit (11 ADCCLK cycles)"]
    #[inline(always)]
    pub fn is_eight_bit(&self) -> bool {
        *self == RES::EightBit
    }
    #[doc = "6-bit (9 ADCCLK cycles)"]
    #[inline(always)]
    pub fn is_six_bit(&self) -> bool {
        *self == RES::SixBit
    }
}
#[doc = "Field `RES` writer - Data resolution"]
pub type RES_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, RES>;
impl<'a, REG> RES_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "12-bit (14 ADCCLK cycles)"]
    #[inline(always)]
    pub fn twelve_bit(self) -> &'a mut crate::W<REG> {
        self.variant(RES::TwelveBit)
    }
    #[doc = "10-bit (13 ADCCLK cycles)"]
    #[inline(always)]
    pub fn ten_bit(self) -> &'a mut crate::W<REG> {
        self.variant(RES::TenBit)
    }
    #[doc = "8-bit (11 ADCCLK cycles)"]
    #[inline(always)]
    pub fn eight_bit(self) -> &'a mut crate::W<REG> {
        self.variant(RES::EightBit)
    }
    #[doc = "6-bit (9 ADCCLK cycles)"]
    #[inline(always)]
    pub fn six_bit(self) -> &'a mut crate::W<REG> {
        self.variant(RES::SixBit)
    }
}
#[doc = "Data alignment\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALIGN {
    #[doc = "0: Right alignment"]
    Right = 0,
    #[doc = "1: Left alignment"]
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
    #[doc = "Right alignment"]
    #[inline(always)]
    pub fn is_right(&self) -> bool {
        *self == ALIGN::Right
    }
    #[doc = "Left alignment"]
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
    #[doc = "Right alignment"]
    #[inline(always)]
    pub fn right(self) -> &'a mut crate::W<REG> {
        self.variant(ALIGN::Right)
    }
    #[doc = "Left alignment"]
    #[inline(always)]
    pub fn left(self) -> &'a mut crate::W<REG> {
        self.variant(ALIGN::Left)
    }
}
#[doc = "External trigger selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTSEL {
    #[doc = "0: Timer 1 TRGO Event"]
    Tim1Trgo = 0,
    #[doc = "1: Timer 1 CC4 event"]
    Tim1Cc4 = 1,
    #[doc = "2: Timer 2 TRGO event"]
    Tim2Trgo = 2,
    #[doc = "3: Timer 3 TRGO event"]
    Tim3Trgo = 3,
    #[doc = "4: Timer 15 TRGO event"]
    Tim15Trgo = 4,
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
#[doc = "Field `EXTSEL` reader - External trigger selection"]
pub type EXTSEL_R = crate::FieldReader<EXTSEL>;
impl EXTSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTSEL> {
        match self.bits {
            0 => Some(EXTSEL::Tim1Trgo),
            1 => Some(EXTSEL::Tim1Cc4),
            2 => Some(EXTSEL::Tim2Trgo),
            3 => Some(EXTSEL::Tim3Trgo),
            4 => Some(EXTSEL::Tim15Trgo),
            _ => None,
        }
    }
    #[doc = "Timer 1 TRGO Event"]
    #[inline(always)]
    pub fn is_tim1_trgo(&self) -> bool {
        *self == EXTSEL::Tim1Trgo
    }
    #[doc = "Timer 1 CC4 event"]
    #[inline(always)]
    pub fn is_tim1_cc4(&self) -> bool {
        *self == EXTSEL::Tim1Cc4
    }
    #[doc = "Timer 2 TRGO event"]
    #[inline(always)]
    pub fn is_tim2_trgo(&self) -> bool {
        *self == EXTSEL::Tim2Trgo
    }
    #[doc = "Timer 3 TRGO event"]
    #[inline(always)]
    pub fn is_tim3_trgo(&self) -> bool {
        *self == EXTSEL::Tim3Trgo
    }
    #[doc = "Timer 15 TRGO event"]
    #[inline(always)]
    pub fn is_tim15_trgo(&self) -> bool {
        *self == EXTSEL::Tim15Trgo
    }
}
#[doc = "Field `EXTSEL` writer - External trigger selection"]
pub type EXTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, EXTSEL>;
impl<'a, REG> EXTSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timer 1 TRGO Event"]
    #[inline(always)]
    pub fn tim1_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim1Trgo)
    }
    #[doc = "Timer 1 CC4 event"]
    #[inline(always)]
    pub fn tim1_cc4(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim1Cc4)
    }
    #[doc = "Timer 2 TRGO event"]
    #[inline(always)]
    pub fn tim2_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim2Trgo)
    }
    #[doc = "Timer 3 TRGO event"]
    #[inline(always)]
    pub fn tim3_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim3Trgo)
    }
    #[doc = "Timer 15 TRGO event"]
    #[inline(always)]
    pub fn tim15_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim15Trgo)
    }
}
#[doc = "External trigger enable and polarity selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTEN {
    #[doc = "0: Trigger detection disabled"]
    Disabled = 0,
    #[doc = "1: Trigger detection on the rising edge"]
    RisingEdge = 1,
    #[doc = "2: Trigger detection on the falling edge"]
    FallingEdge = 2,
    #[doc = "3: Trigger detection on both the rising and falling edges"]
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
#[doc = "Field `EXTEN` reader - External trigger enable and polarity selection"]
pub type EXTEN_R = crate::FieldReader<EXTEN>;
impl EXTEN_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "Trigger detection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EXTEN::Disabled
    }
    #[doc = "Trigger detection on the rising edge"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == EXTEN::RisingEdge
    }
    #[doc = "Trigger detection on the falling edge"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == EXTEN::FallingEdge
    }
    #[doc = "Trigger detection on both the rising and falling edges"]
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        *self == EXTEN::BothEdges
    }
}
#[doc = "Field `EXTEN` writer - External trigger enable and polarity selection"]
pub type EXTEN_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, EXTEN>;
impl<'a, REG> EXTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Trigger detection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EXTEN::Disabled)
    }
    #[doc = "Trigger detection on the rising edge"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(EXTEN::RisingEdge)
    }
    #[doc = "Trigger detection on the falling edge"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(EXTEN::FallingEdge)
    }
    #[doc = "Trigger detection on both the rising and falling edges"]
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut crate::W<REG> {
        self.variant(EXTEN::BothEdges)
    }
}
#[doc = "Overrun management mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRMOD {
    #[doc = "0: ADC_DR register is preserved with the old data when an overrun is detected"]
    Preserved = 0,
    #[doc = "1: ADC_DR register is overwritten with the last conversion result when an overrun is detected"]
    Overwritten = 1,
}
impl From<OVRMOD> for bool {
    #[inline(always)]
    fn from(variant: OVRMOD) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVRMOD` reader - Overrun management mode"]
pub type OVRMOD_R = crate::BitReader<OVRMOD>;
impl OVRMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OVRMOD {
        match self.bits {
            false => OVRMOD::Preserved,
            true => OVRMOD::Overwritten,
        }
    }
    #[doc = "ADC_DR register is preserved with the old data when an overrun is detected"]
    #[inline(always)]
    pub fn is_preserved(&self) -> bool {
        *self == OVRMOD::Preserved
    }
    #[doc = "ADC_DR register is overwritten with the last conversion result when an overrun is detected"]
    #[inline(always)]
    pub fn is_overwritten(&self) -> bool {
        *self == OVRMOD::Overwritten
    }
}
#[doc = "Field `OVRMOD` writer - Overrun management mode"]
pub type OVRMOD_W<'a, REG> = crate::BitWriter<'a, REG, OVRMOD>;
impl<'a, REG> OVRMOD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC_DR register is preserved with the old data when an overrun is detected"]
    #[inline(always)]
    pub fn preserved(self) -> &'a mut crate::W<REG> {
        self.variant(OVRMOD::Preserved)
    }
    #[doc = "ADC_DR register is overwritten with the last conversion result when an overrun is detected"]
    #[inline(always)]
    pub fn overwritten(self) -> &'a mut crate::W<REG> {
        self.variant(OVRMOD::Overwritten)
    }
}
#[doc = "Single / continuous conversion mode\n\nValue on reset: 0"]
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
#[doc = "Field `CONT` reader - Single / continuous conversion mode"]
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
#[doc = "Field `CONT` writer - Single / continuous conversion mode"]
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
#[doc = "Wait conversion mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAIT {
    #[doc = "0: Wait conversion mode off"]
    Disabled = 0,
    #[doc = "1: Wait conversion mode on"]
    Enabled = 1,
}
impl From<WAIT> for bool {
    #[inline(always)]
    fn from(variant: WAIT) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAIT` reader - Wait conversion mode"]
pub type WAIT_R = crate::BitReader<WAIT>;
impl WAIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WAIT {
        match self.bits {
            false => WAIT::Disabled,
            true => WAIT::Enabled,
        }
    }
    #[doc = "Wait conversion mode off"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WAIT::Disabled
    }
    #[doc = "Wait conversion mode on"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WAIT::Enabled
    }
}
#[doc = "Field `WAIT` writer - Wait conversion mode"]
pub type WAIT_W<'a, REG> = crate::BitWriter<'a, REG, WAIT>;
impl<'a, REG> WAIT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wait conversion mode off"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(WAIT::Disabled)
    }
    #[doc = "Wait conversion mode on"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(WAIT::Enabled)
    }
}
#[doc = "Auto-off mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUTOFF {
    #[doc = "0: Auto-off mode disabled"]
    Disabled = 0,
    #[doc = "1: Auto-off mode enabled"]
    Enabled = 1,
}
impl From<AUTOFF> for bool {
    #[inline(always)]
    fn from(variant: AUTOFF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUTOFF` reader - Auto-off mode"]
pub type AUTOFF_R = crate::BitReader<AUTOFF>;
impl AUTOFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AUTOFF {
        match self.bits {
            false => AUTOFF::Disabled,
            true => AUTOFF::Enabled,
        }
    }
    #[doc = "Auto-off mode disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AUTOFF::Disabled
    }
    #[doc = "Auto-off mode enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AUTOFF::Enabled
    }
}
#[doc = "Field `AUTOFF` writer - Auto-off mode"]
pub type AUTOFF_W<'a, REG> = crate::BitWriter<'a, REG, AUTOFF>;
impl<'a, REG> AUTOFF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Auto-off mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(AUTOFF::Disabled)
    }
    #[doc = "Auto-off mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(AUTOFF::Enabled)
    }
}
#[doc = "Discontinuous mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DISCEN {
    #[doc = "0: Discontinuous mode on regular channels disabled"]
    Disabled = 0,
    #[doc = "1: Discontinuous mode on regular channels enabled"]
    Enabled = 1,
}
impl From<DISCEN> for bool {
    #[inline(always)]
    fn from(variant: DISCEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DISCEN` reader - Discontinuous mode"]
pub type DISCEN_R = crate::BitReader<DISCEN>;
impl DISCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DISCEN {
        match self.bits {
            false => DISCEN::Disabled,
            true => DISCEN::Enabled,
        }
    }
    #[doc = "Discontinuous mode on regular channels disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DISCEN::Disabled
    }
    #[doc = "Discontinuous mode on regular channels enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DISCEN::Enabled
    }
}
#[doc = "Field `DISCEN` writer - Discontinuous mode"]
pub type DISCEN_W<'a, REG> = crate::BitWriter<'a, REG, DISCEN>;
impl<'a, REG> DISCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Discontinuous mode on regular channels disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DISCEN::Disabled)
    }
    #[doc = "Discontinuous mode on regular channels enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DISCEN::Enabled)
    }
}
#[doc = "Enable the watchdog on a single channel or on all channels\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWDSGL {
    #[doc = "0: Analog watchdog enabled on all channels"]
    AllChannels = 0,
    #[doc = "1: Analog watchdog enabled on a single channel"]
    SingleChannel = 1,
}
impl From<AWDSGL> for bool {
    #[inline(always)]
    fn from(variant: AWDSGL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWDSGL` reader - Enable the watchdog on a single channel or on all channels"]
pub type AWDSGL_R = crate::BitReader<AWDSGL>;
impl AWDSGL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AWDSGL {
        match self.bits {
            false => AWDSGL::AllChannels,
            true => AWDSGL::SingleChannel,
        }
    }
    #[doc = "Analog watchdog enabled on all channels"]
    #[inline(always)]
    pub fn is_all_channels(&self) -> bool {
        *self == AWDSGL::AllChannels
    }
    #[doc = "Analog watchdog enabled on a single channel"]
    #[inline(always)]
    pub fn is_single_channel(&self) -> bool {
        *self == AWDSGL::SingleChannel
    }
}
#[doc = "Field `AWDSGL` writer - Enable the watchdog on a single channel or on all channels"]
pub type AWDSGL_W<'a, REG> = crate::BitWriter<'a, REG, AWDSGL>;
impl<'a, REG> AWDSGL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Analog watchdog enabled on all channels"]
    #[inline(always)]
    pub fn all_channels(self) -> &'a mut crate::W<REG> {
        self.variant(AWDSGL::AllChannels)
    }
    #[doc = "Analog watchdog enabled on a single channel"]
    #[inline(always)]
    pub fn single_channel(self) -> &'a mut crate::W<REG> {
        self.variant(AWDSGL::SingleChannel)
    }
}
#[doc = "Analog watchdog enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWDEN {
    #[doc = "0: Analog watchdog disabled on regular channels"]
    Disabled = 0,
    #[doc = "1: Analog watchdog enabled on regular channels"]
    Enabled = 1,
}
impl From<AWDEN> for bool {
    #[inline(always)]
    fn from(variant: AWDEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWDEN` reader - Analog watchdog enable"]
pub type AWDEN_R = crate::BitReader<AWDEN>;
impl AWDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AWDEN {
        match self.bits {
            false => AWDEN::Disabled,
            true => AWDEN::Enabled,
        }
    }
    #[doc = "Analog watchdog disabled on regular channels"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AWDEN::Disabled
    }
    #[doc = "Analog watchdog enabled on regular channels"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AWDEN::Enabled
    }
}
#[doc = "Field `AWDEN` writer - Analog watchdog enable"]
pub type AWDEN_W<'a, REG> = crate::BitWriter<'a, REG, AWDEN>;
impl<'a, REG> AWDEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Analog watchdog disabled on regular channels"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(AWDEN::Disabled)
    }
    #[doc = "Analog watchdog enabled on regular channels"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(AWDEN::Enabled)
    }
}
#[doc = "Field `AWDCH` reader - Analog watchdog channel selection"]
pub type AWDCH_R = crate::FieldReader;
#[doc = "Field `AWDCH` writer - Analog watchdog channel selection"]
pub type AWDCH_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - Direct memory access enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Direct memery access configuration"]
    #[inline(always)]
    pub fn dmacfg(&self) -> DMACFG_R {
        DMACFG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Scan sequence direction"]
    #[inline(always)]
    pub fn scandir(&self) -> SCANDIR_R {
        SCANDIR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Data resolution"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - Data alignment"]
    #[inline(always)]
    pub fn align(&self) -> ALIGN_R {
        ALIGN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:8 - External trigger selection"]
    #[inline(always)]
    pub fn extsel(&self) -> EXTSEL_R {
        EXTSEL_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 10:11 - External trigger enable and polarity selection"]
    #[inline(always)]
    pub fn exten(&self) -> EXTEN_R {
        EXTEN_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Overrun management mode"]
    #[inline(always)]
    pub fn ovrmod(&self) -> OVRMOD_R {
        OVRMOD_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Single / continuous conversion mode"]
    #[inline(always)]
    pub fn cont(&self) -> CONT_R {
        CONT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Wait conversion mode"]
    #[inline(always)]
    pub fn wait(&self) -> WAIT_R {
        WAIT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Auto-off mode"]
    #[inline(always)]
    pub fn autoff(&self) -> AUTOFF_R {
        AUTOFF_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Discontinuous mode"]
    #[inline(always)]
    pub fn discen(&self) -> DISCEN_R {
        DISCEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable the watchdog on a single channel or on all channels"]
    #[inline(always)]
    pub fn awdsgl(&self) -> AWDSGL_R {
        AWDSGL_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Analog watchdog enable"]
    #[inline(always)]
    pub fn awden(&self) -> AWDEN_R {
        AWDEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 26:30 - Analog watchdog channel selection"]
    #[inline(always)]
    pub fn awdch(&self) -> AWDCH_R {
        AWDCH_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Direct memory access enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<CFGR1rs> {
        DMAEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Direct memery access configuration"]
    #[inline(always)]
    #[must_use]
    pub fn dmacfg(&mut self) -> DMACFG_W<CFGR1rs> {
        DMACFG_W::new(self, 1)
    }
    #[doc = "Bit 2 - Scan sequence direction"]
    #[inline(always)]
    #[must_use]
    pub fn scandir(&mut self) -> SCANDIR_W<CFGR1rs> {
        SCANDIR_W::new(self, 2)
    }
    #[doc = "Bits 3:4 - Data resolution"]
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> RES_W<CFGR1rs> {
        RES_W::new(self, 3)
    }
    #[doc = "Bit 5 - Data alignment"]
    #[inline(always)]
    #[must_use]
    pub fn align(&mut self) -> ALIGN_W<CFGR1rs> {
        ALIGN_W::new(self, 5)
    }
    #[doc = "Bits 6:8 - External trigger selection"]
    #[inline(always)]
    #[must_use]
    pub fn extsel(&mut self) -> EXTSEL_W<CFGR1rs> {
        EXTSEL_W::new(self, 6)
    }
    #[doc = "Bits 10:11 - External trigger enable and polarity selection"]
    #[inline(always)]
    #[must_use]
    pub fn exten(&mut self) -> EXTEN_W<CFGR1rs> {
        EXTEN_W::new(self, 10)
    }
    #[doc = "Bit 12 - Overrun management mode"]
    #[inline(always)]
    #[must_use]
    pub fn ovrmod(&mut self) -> OVRMOD_W<CFGR1rs> {
        OVRMOD_W::new(self, 12)
    }
    #[doc = "Bit 13 - Single / continuous conversion mode"]
    #[inline(always)]
    #[must_use]
    pub fn cont(&mut self) -> CONT_W<CFGR1rs> {
        CONT_W::new(self, 13)
    }
    #[doc = "Bit 14 - Wait conversion mode"]
    #[inline(always)]
    #[must_use]
    pub fn wait(&mut self) -> WAIT_W<CFGR1rs> {
        WAIT_W::new(self, 14)
    }
    #[doc = "Bit 15 - Auto-off mode"]
    #[inline(always)]
    #[must_use]
    pub fn autoff(&mut self) -> AUTOFF_W<CFGR1rs> {
        AUTOFF_W::new(self, 15)
    }
    #[doc = "Bit 16 - Discontinuous mode"]
    #[inline(always)]
    #[must_use]
    pub fn discen(&mut self) -> DISCEN_W<CFGR1rs> {
        DISCEN_W::new(self, 16)
    }
    #[doc = "Bit 22 - Enable the watchdog on a single channel or on all channels"]
    #[inline(always)]
    #[must_use]
    pub fn awdsgl(&mut self) -> AWDSGL_W<CFGR1rs> {
        AWDSGL_W::new(self, 22)
    }
    #[doc = "Bit 23 - Analog watchdog enable"]
    #[inline(always)]
    #[must_use]
    pub fn awden(&mut self) -> AWDEN_W<CFGR1rs> {
        AWDEN_W::new(self, 23)
    }
    #[doc = "Bits 26:30 - Analog watchdog channel selection"]
    #[inline(always)]
    #[must_use]
    pub fn awdch(&mut self) -> AWDCH_W<CFGR1rs> {
        AWDCH_W::new(self, 26)
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
