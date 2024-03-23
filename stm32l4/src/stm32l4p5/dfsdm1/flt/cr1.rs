#[doc = "Register `CR1` reader"]
pub type R = crate::R<CR1rs>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<CR1rs>;
#[doc = "DFSDM enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DFEN {
    #[doc = "0: DFSDM_FLTx is disabled. All conversions of given DFSDM_FLTx are stopped immediately and all DFSDM_FLTx functions are stopped"]
    Disabled = 0,
    #[doc = "1: DFSDM_FLTx is enabled. If DFSDM_FLTx is enabled, then DFSDM_FLTx starts operating according to its setting"]
    Enabled = 1,
}
impl From<DFEN> for bool {
    #[inline(always)]
    fn from(variant: DFEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DFEN` reader - DFSDM enable"]
pub type DFEN_R = crate::BitReader<DFEN>;
impl DFEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DFEN {
        match self.bits {
            false => DFEN::Disabled,
            true => DFEN::Enabled,
        }
    }
    #[doc = "DFSDM_FLTx is disabled. All conversions of given DFSDM_FLTx are stopped immediately and all DFSDM_FLTx functions are stopped"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DFEN::Disabled
    }
    #[doc = "DFSDM_FLTx is enabled. If DFSDM_FLTx is enabled, then DFSDM_FLTx starts operating according to its setting"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DFEN::Enabled
    }
}
#[doc = "Field `DFEN` writer - DFSDM enable"]
pub type DFEN_W<'a, REG> = crate::BitWriter<'a, REG, DFEN>;
impl<'a, REG> DFEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DFSDM_FLTx is disabled. All conversions of given DFSDM_FLTx are stopped immediately and all DFSDM_FLTx functions are stopped"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DFEN::Disabled)
    }
    #[doc = "DFSDM_FLTx is enabled. If DFSDM_FLTx is enabled, then DFSDM_FLTx starts operating according to its setting"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DFEN::Enabled)
    }
}
#[doc = "Start a conversion of the injected group of channels\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JSWSTARTW {
    #[doc = "1: Writing ‘1’ makes a request to convert the channels in the injected conversion group, causing JCIP to become ‘1’ at the same time. If JCIP=1 already, then writing to JSWSTART has no effect. Writing ‘1’ has no effect if JSYNC=1"]
    Start = 1,
}
impl From<JSWSTARTW> for bool {
    #[inline(always)]
    fn from(variant: JSWSTARTW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JSWSTART` reader - Start a conversion of the injected group of channels"]
pub type JSWSTART_R = crate::BitReader<JSWSTARTW>;
impl JSWSTART_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<JSWSTARTW> {
        match self.bits {
            true => Some(JSWSTARTW::Start),
            _ => None,
        }
    }
    #[doc = "Writing ‘1’ makes a request to convert the channels in the injected conversion group, causing JCIP to become ‘1’ at the same time. If JCIP=1 already, then writing to JSWSTART has no effect. Writing ‘1’ has no effect if JSYNC=1"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == JSWSTARTW::Start
    }
}
#[doc = "Field `JSWSTART` writer - Start a conversion of the injected group of channels"]
pub type JSWSTART_W<'a, REG> = crate::BitWriter<'a, REG, JSWSTARTW>;
impl<'a, REG> JSWSTART_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing ‘1’ makes a request to convert the channels in the injected conversion group, causing JCIP to become ‘1’ at the same time. If JCIP=1 already, then writing to JSWSTART has no effect. Writing ‘1’ has no effect if JSYNC=1"]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(JSWSTARTW::Start)
    }
}
#[doc = "Launch an injected conversion synchronously with the DFSDM0 JSWSTART trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JSYNC {
    #[doc = "0: Do not launch an injected conversion synchronously with DFSDM_FLT0"]
    Disabled = 0,
    #[doc = "1: Launch an injected conversion in this DFSDM_FLTx at the very moment when an injected conversion is launched in DFSDM_FLT0 by its JSWSTART trigger"]
    Enabled = 1,
}
impl From<JSYNC> for bool {
    #[inline(always)]
    fn from(variant: JSYNC) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JSYNC` reader - Launch an injected conversion synchronously with the DFSDM0 JSWSTART trigger"]
pub type JSYNC_R = crate::BitReader<JSYNC>;
impl JSYNC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> JSYNC {
        match self.bits {
            false => JSYNC::Disabled,
            true => JSYNC::Enabled,
        }
    }
    #[doc = "Do not launch an injected conversion synchronously with DFSDM_FLT0"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JSYNC::Disabled
    }
    #[doc = "Launch an injected conversion in this DFSDM_FLTx at the very moment when an injected conversion is launched in DFSDM_FLT0 by its JSWSTART trigger"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == JSYNC::Enabled
    }
}
#[doc = "Field `JSYNC` writer - Launch an injected conversion synchronously with the DFSDM0 JSWSTART trigger"]
pub type JSYNC_W<'a, REG> = crate::BitWriter<'a, REG, JSYNC>;
impl<'a, REG> JSYNC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not launch an injected conversion synchronously with DFSDM_FLT0"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(JSYNC::Disabled)
    }
    #[doc = "Launch an injected conversion in this DFSDM_FLTx at the very moment when an injected conversion is launched in DFSDM_FLT0 by its JSWSTART trigger"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(JSYNC::Enabled)
    }
}
#[doc = "Scanning conversion mode for injected conversions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JSCAN {
    #[doc = "0: One channel conversion is performed from the injected channel group and next the selected channel from this group is selected"]
    Single = 0,
    #[doc = "1: The series of conversions for the injected group channels is executed, starting over with the lowest selected channel"]
    Series = 1,
}
impl From<JSCAN> for bool {
    #[inline(always)]
    fn from(variant: JSCAN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JSCAN` reader - Scanning conversion mode for injected conversions"]
pub type JSCAN_R = crate::BitReader<JSCAN>;
impl JSCAN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> JSCAN {
        match self.bits {
            false => JSCAN::Single,
            true => JSCAN::Series,
        }
    }
    #[doc = "One channel conversion is performed from the injected channel group and next the selected channel from this group is selected"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == JSCAN::Single
    }
    #[doc = "The series of conversions for the injected group channels is executed, starting over with the lowest selected channel"]
    #[inline(always)]
    pub fn is_series(&self) -> bool {
        *self == JSCAN::Series
    }
}
#[doc = "Field `JSCAN` writer - Scanning conversion mode for injected conversions"]
pub type JSCAN_W<'a, REG> = crate::BitWriter<'a, REG, JSCAN>;
impl<'a, REG> JSCAN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "One channel conversion is performed from the injected channel group and next the selected channel from this group is selected"]
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(JSCAN::Single)
    }
    #[doc = "The series of conversions for the injected group channels is executed, starting over with the lowest selected channel"]
    #[inline(always)]
    pub fn series(self) -> &'a mut crate::W<REG> {
        self.variant(JSCAN::Series)
    }
}
#[doc = "DMA channel enabled to read data for the injected channel group\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JDMAEN {
    #[doc = "0: The DMA channel is not enabled to read injected data"]
    Disabled = 0,
    #[doc = "1: The DMA channel is enabled to read injected data"]
    Enabled = 1,
}
impl From<JDMAEN> for bool {
    #[inline(always)]
    fn from(variant: JDMAEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JDMAEN` reader - DMA channel enabled to read data for the injected channel group"]
pub type JDMAEN_R = crate::BitReader<JDMAEN>;
impl JDMAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> JDMAEN {
        match self.bits {
            false => JDMAEN::Disabled,
            true => JDMAEN::Enabled,
        }
    }
    #[doc = "The DMA channel is not enabled to read injected data"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JDMAEN::Disabled
    }
    #[doc = "The DMA channel is enabled to read injected data"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == JDMAEN::Enabled
    }
}
#[doc = "Field `JDMAEN` writer - DMA channel enabled to read data for the injected channel group"]
pub type JDMAEN_W<'a, REG> = crate::BitWriter<'a, REG, JDMAEN>;
impl<'a, REG> JDMAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The DMA channel is not enabled to read injected data"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(JDMAEN::Disabled)
    }
    #[doc = "The DMA channel is enabled to read injected data"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(JDMAEN::Enabled)
    }
}
#[doc = "Field `JEXTSEL` reader - Trigger signal selection for launching injected conversions"]
pub type JEXTSEL_R = crate::FieldReader;
#[doc = "Field `JEXTSEL` writer - Trigger signal selection for launching injected conversions"]
pub type JEXTSEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 5>;
#[doc = "Trigger enable and trigger edge selection for injected conversions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum JEXTEN {
    #[doc = "0: Trigger detection is disabled"]
    Disabled = 0,
    #[doc = "1: Each rising edge on the selected trigger makes a request to launch an injected conversion"]
    RisingEdge = 1,
    #[doc = "2: Each falling edge on the selected trigger makes a request to launch an injected conversion"]
    FallingEdge = 2,
    #[doc = "3: Both rising edges and falling edges on the selected trigger make requests to launch injected conversions"]
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
#[doc = "Field `JEXTEN` reader - Trigger enable and trigger edge selection for injected conversions"]
pub type JEXTEN_R = crate::FieldReader<JEXTEN>;
impl JEXTEN_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "Trigger detection is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JEXTEN::Disabled
    }
    #[doc = "Each rising edge on the selected trigger makes a request to launch an injected conversion"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == JEXTEN::RisingEdge
    }
    #[doc = "Each falling edge on the selected trigger makes a request to launch an injected conversion"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == JEXTEN::FallingEdge
    }
    #[doc = "Both rising edges and falling edges on the selected trigger make requests to launch injected conversions"]
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        *self == JEXTEN::BothEdges
    }
}
#[doc = "Field `JEXTEN` writer - Trigger enable and trigger edge selection for injected conversions"]
pub type JEXTEN_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, JEXTEN>;
impl<'a, REG> JEXTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Trigger detection is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTEN::Disabled)
    }
    #[doc = "Each rising edge on the selected trigger makes a request to launch an injected conversion"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTEN::RisingEdge)
    }
    #[doc = "Each falling edge on the selected trigger makes a request to launch an injected conversion"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTEN::FallingEdge)
    }
    #[doc = "Both rising edges and falling edges on the selected trigger make requests to launch injected conversions"]
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTEN::BothEdges)
    }
}
#[doc = "Software start of a conversion on the regular channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSWSTARTW {
    #[doc = "1: Writing ‘1’ makes a request to start a conversion on the regular channel and causes RCIP to become ‘1’. If RCIP=1 already, writing to RSWSTART has no effect. Writing ‘1’ has no effect if RSYNC=1"]
    Start = 1,
}
impl From<RSWSTARTW> for bool {
    #[inline(always)]
    fn from(variant: RSWSTARTW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSWSTART` reader - Software start of a conversion on the regular channel"]
pub type RSWSTART_R = crate::BitReader<RSWSTARTW>;
impl RSWSTART_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RSWSTARTW> {
        match self.bits {
            true => Some(RSWSTARTW::Start),
            _ => None,
        }
    }
    #[doc = "Writing ‘1’ makes a request to start a conversion on the regular channel and causes RCIP to become ‘1’. If RCIP=1 already, writing to RSWSTART has no effect. Writing ‘1’ has no effect if RSYNC=1"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == RSWSTARTW::Start
    }
}
#[doc = "Field `RSWSTART` writer - Software start of a conversion on the regular channel"]
pub type RSWSTART_W<'a, REG> = crate::BitWriter<'a, REG, RSWSTARTW>;
impl<'a, REG> RSWSTART_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing ‘1’ makes a request to start a conversion on the regular channel and causes RCIP to become ‘1’. If RCIP=1 already, writing to RSWSTART has no effect. Writing ‘1’ has no effect if RSYNC=1"]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(RSWSTARTW::Start)
    }
}
#[doc = "Continuous mode selection for regular conversions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RCONT {
    #[doc = "0: The regular channel is converted just once for each conversion request"]
    Once = 0,
    #[doc = "1: The regular channel is converted repeatedly after each conversion request"]
    Continuous = 1,
}
impl From<RCONT> for bool {
    #[inline(always)]
    fn from(variant: RCONT) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RCONT` reader - Continuous mode selection for regular conversions"]
pub type RCONT_R = crate::BitReader<RCONT>;
impl RCONT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RCONT {
        match self.bits {
            false => RCONT::Once,
            true => RCONT::Continuous,
        }
    }
    #[doc = "The regular channel is converted just once for each conversion request"]
    #[inline(always)]
    pub fn is_once(&self) -> bool {
        *self == RCONT::Once
    }
    #[doc = "The regular channel is converted repeatedly after each conversion request"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == RCONT::Continuous
    }
}
#[doc = "Field `RCONT` writer - Continuous mode selection for regular conversions"]
pub type RCONT_W<'a, REG> = crate::BitWriter<'a, REG, RCONT>;
impl<'a, REG> RCONT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The regular channel is converted just once for each conversion request"]
    #[inline(always)]
    pub fn once(self) -> &'a mut crate::W<REG> {
        self.variant(RCONT::Once)
    }
    #[doc = "The regular channel is converted repeatedly after each conversion request"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut crate::W<REG> {
        self.variant(RCONT::Continuous)
    }
}
#[doc = "Launch regular conversion synchronously with DFSDM0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSYNC {
    #[doc = "0: Do not launch a regular conversion synchronously with DFSDM_FLT0"]
    NoLaunch = 0,
    #[doc = "1: Launch a regular conversion in this DFSDM_FLTx at the very moment when a regular conversion is launched in DFSDM_FLT0"]
    Launch = 1,
}
impl From<RSYNC> for bool {
    #[inline(always)]
    fn from(variant: RSYNC) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSYNC` reader - Launch regular conversion synchronously with DFSDM0"]
pub type RSYNC_R = crate::BitReader<RSYNC>;
impl RSYNC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RSYNC {
        match self.bits {
            false => RSYNC::NoLaunch,
            true => RSYNC::Launch,
        }
    }
    #[doc = "Do not launch a regular conversion synchronously with DFSDM_FLT0"]
    #[inline(always)]
    pub fn is_no_launch(&self) -> bool {
        *self == RSYNC::NoLaunch
    }
    #[doc = "Launch a regular conversion in this DFSDM_FLTx at the very moment when a regular conversion is launched in DFSDM_FLT0"]
    #[inline(always)]
    pub fn is_launch(&self) -> bool {
        *self == RSYNC::Launch
    }
}
#[doc = "Field `RSYNC` writer - Launch regular conversion synchronously with DFSDM0"]
pub type RSYNC_W<'a, REG> = crate::BitWriter<'a, REG, RSYNC>;
impl<'a, REG> RSYNC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not launch a regular conversion synchronously with DFSDM_FLT0"]
    #[inline(always)]
    pub fn no_launch(self) -> &'a mut crate::W<REG> {
        self.variant(RSYNC::NoLaunch)
    }
    #[doc = "Launch a regular conversion in this DFSDM_FLTx at the very moment when a regular conversion is launched in DFSDM_FLT0"]
    #[inline(always)]
    pub fn launch(self) -> &'a mut crate::W<REG> {
        self.variant(RSYNC::Launch)
    }
}
#[doc = "DMA channel enabled to read data for the regular conversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDMAEN {
    #[doc = "0: The DMA channel is not enabled to read regular data"]
    Disabled = 0,
    #[doc = "1: The DMA channel is enabled to read regular data"]
    Enabled = 1,
}
impl From<RDMAEN> for bool {
    #[inline(always)]
    fn from(variant: RDMAEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDMAEN` reader - DMA channel enabled to read data for the regular conversion"]
pub type RDMAEN_R = crate::BitReader<RDMAEN>;
impl RDMAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RDMAEN {
        match self.bits {
            false => RDMAEN::Disabled,
            true => RDMAEN::Enabled,
        }
    }
    #[doc = "The DMA channel is not enabled to read regular data"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RDMAEN::Disabled
    }
    #[doc = "The DMA channel is enabled to read regular data"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RDMAEN::Enabled
    }
}
#[doc = "Field `RDMAEN` writer - DMA channel enabled to read data for the regular conversion"]
pub type RDMAEN_W<'a, REG> = crate::BitWriter<'a, REG, RDMAEN>;
impl<'a, REG> RDMAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The DMA channel is not enabled to read regular data"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RDMAEN::Disabled)
    }
    #[doc = "The DMA channel is enabled to read regular data"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RDMAEN::Enabled)
    }
}
#[doc = "Regular channel selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RCH {
    #[doc = "0: Channel 0 is selected as regular channel"]
    Channel0 = 0,
    #[doc = "1: Channel 1 is selected as regular channel"]
    Channel1 = 1,
    #[doc = "2: Channel 2 is selected as regular channel"]
    Channel2 = 2,
    #[doc = "3: Channel 3 is selected as regular channel"]
    Channel3 = 3,
    #[doc = "4: Channel 4 is selected as regular channel"]
    Channel4 = 4,
    #[doc = "5: Channel 5 is selected as regular channel"]
    Channel5 = 5,
    #[doc = "6: Channel 6 is selected as regular channel"]
    Channel6 = 6,
    #[doc = "7: Channel 7 is selected as regular channel"]
    Channel7 = 7,
}
impl From<RCH> for u8 {
    #[inline(always)]
    fn from(variant: RCH) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RCH {
    type Ux = u8;
}
#[doc = "Field `RCH` reader - Regular channel selection"]
pub type RCH_R = crate::FieldReader<RCH>;
impl RCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RCH {
        match self.bits {
            0 => RCH::Channel0,
            1 => RCH::Channel1,
            2 => RCH::Channel2,
            3 => RCH::Channel3,
            4 => RCH::Channel4,
            5 => RCH::Channel5,
            6 => RCH::Channel6,
            7 => RCH::Channel7,
            _ => unreachable!(),
        }
    }
    #[doc = "Channel 0 is selected as regular channel"]
    #[inline(always)]
    pub fn is_channel0(&self) -> bool {
        *self == RCH::Channel0
    }
    #[doc = "Channel 1 is selected as regular channel"]
    #[inline(always)]
    pub fn is_channel1(&self) -> bool {
        *self == RCH::Channel1
    }
    #[doc = "Channel 2 is selected as regular channel"]
    #[inline(always)]
    pub fn is_channel2(&self) -> bool {
        *self == RCH::Channel2
    }
    #[doc = "Channel 3 is selected as regular channel"]
    #[inline(always)]
    pub fn is_channel3(&self) -> bool {
        *self == RCH::Channel3
    }
    #[doc = "Channel 4 is selected as regular channel"]
    #[inline(always)]
    pub fn is_channel4(&self) -> bool {
        *self == RCH::Channel4
    }
    #[doc = "Channel 5 is selected as regular channel"]
    #[inline(always)]
    pub fn is_channel5(&self) -> bool {
        *self == RCH::Channel5
    }
    #[doc = "Channel 6 is selected as regular channel"]
    #[inline(always)]
    pub fn is_channel6(&self) -> bool {
        *self == RCH::Channel6
    }
    #[doc = "Channel 7 is selected as regular channel"]
    #[inline(always)]
    pub fn is_channel7(&self) -> bool {
        *self == RCH::Channel7
    }
}
#[doc = "Field `RCH` writer - Regular channel selection"]
pub type RCH_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, RCH>;
impl<'a, REG> RCH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Channel 0 is selected as regular channel"]
    #[inline(always)]
    pub fn channel0(self) -> &'a mut crate::W<REG> {
        self.variant(RCH::Channel0)
    }
    #[doc = "Channel 1 is selected as regular channel"]
    #[inline(always)]
    pub fn channel1(self) -> &'a mut crate::W<REG> {
        self.variant(RCH::Channel1)
    }
    #[doc = "Channel 2 is selected as regular channel"]
    #[inline(always)]
    pub fn channel2(self) -> &'a mut crate::W<REG> {
        self.variant(RCH::Channel2)
    }
    #[doc = "Channel 3 is selected as regular channel"]
    #[inline(always)]
    pub fn channel3(self) -> &'a mut crate::W<REG> {
        self.variant(RCH::Channel3)
    }
    #[doc = "Channel 4 is selected as regular channel"]
    #[inline(always)]
    pub fn channel4(self) -> &'a mut crate::W<REG> {
        self.variant(RCH::Channel4)
    }
    #[doc = "Channel 5 is selected as regular channel"]
    #[inline(always)]
    pub fn channel5(self) -> &'a mut crate::W<REG> {
        self.variant(RCH::Channel5)
    }
    #[doc = "Channel 6 is selected as regular channel"]
    #[inline(always)]
    pub fn channel6(self) -> &'a mut crate::W<REG> {
        self.variant(RCH::Channel6)
    }
    #[doc = "Channel 7 is selected as regular channel"]
    #[inline(always)]
    pub fn channel7(self) -> &'a mut crate::W<REG> {
        self.variant(RCH::Channel7)
    }
}
#[doc = "Fast conversion mode selection for regular conversions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FAST {
    #[doc = "0: Fast conversion mode disabled"]
    Disabled = 0,
    #[doc = "1: Fast conversion mode enabled"]
    Enabled = 1,
}
impl From<FAST> for bool {
    #[inline(always)]
    fn from(variant: FAST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FAST` reader - Fast conversion mode selection for regular conversions"]
pub type FAST_R = crate::BitReader<FAST>;
impl FAST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FAST {
        match self.bits {
            false => FAST::Disabled,
            true => FAST::Enabled,
        }
    }
    #[doc = "Fast conversion mode disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FAST::Disabled
    }
    #[doc = "Fast conversion mode enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FAST::Enabled
    }
}
#[doc = "Field `FAST` writer - Fast conversion mode selection for regular conversions"]
pub type FAST_W<'a, REG> = crate::BitWriter<'a, REG, FAST>;
impl<'a, REG> FAST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Fast conversion mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FAST::Disabled)
    }
    #[doc = "Fast conversion mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FAST::Enabled)
    }
}
#[doc = "Analog watchdog fast mode select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWFSEL {
    #[doc = "0: Analog watchdog on data output value (after the digital filter). The comparison is done after offset correction and shift"]
    Output = 0,
    #[doc = "1: Analog watchdog on channel transceivers value (after watchdog filter)"]
    Transceiver = 1,
}
impl From<AWFSEL> for bool {
    #[inline(always)]
    fn from(variant: AWFSEL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWFSEL` reader - Analog watchdog fast mode select"]
pub type AWFSEL_R = crate::BitReader<AWFSEL>;
impl AWFSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AWFSEL {
        match self.bits {
            false => AWFSEL::Output,
            true => AWFSEL::Transceiver,
        }
    }
    #[doc = "Analog watchdog on data output value (after the digital filter). The comparison is done after offset correction and shift"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == AWFSEL::Output
    }
    #[doc = "Analog watchdog on channel transceivers value (after watchdog filter)"]
    #[inline(always)]
    pub fn is_transceiver(&self) -> bool {
        *self == AWFSEL::Transceiver
    }
}
#[doc = "Field `AWFSEL` writer - Analog watchdog fast mode select"]
pub type AWFSEL_W<'a, REG> = crate::BitWriter<'a, REG, AWFSEL>;
impl<'a, REG> AWFSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Analog watchdog on data output value (after the digital filter). The comparison is done after offset correction and shift"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(AWFSEL::Output)
    }
    #[doc = "Analog watchdog on channel transceivers value (after watchdog filter)"]
    #[inline(always)]
    pub fn transceiver(self) -> &'a mut crate::W<REG> {
        self.variant(AWFSEL::Transceiver)
    }
}
impl R {
    #[doc = "Bit 0 - DFSDM enable"]
    #[inline(always)]
    pub fn dfen(&self) -> DFEN_R {
        DFEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Start a conversion of the injected group of channels"]
    #[inline(always)]
    pub fn jswstart(&self) -> JSWSTART_R {
        JSWSTART_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Launch an injected conversion synchronously with the DFSDM0 JSWSTART trigger"]
    #[inline(always)]
    pub fn jsync(&self) -> JSYNC_R {
        JSYNC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Scanning conversion mode for injected conversions"]
    #[inline(always)]
    pub fn jscan(&self) -> JSCAN_R {
        JSCAN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DMA channel enabled to read data for the injected channel group"]
    #[inline(always)]
    pub fn jdmaen(&self) -> JDMAEN_R {
        JDMAEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:12 - Trigger signal selection for launching injected conversions"]
    #[inline(always)]
    pub fn jextsel(&self) -> JEXTSEL_R {
        JEXTSEL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:14 - Trigger enable and trigger edge selection for injected conversions"]
    #[inline(always)]
    pub fn jexten(&self) -> JEXTEN_R {
        JEXTEN_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 17 - Software start of a conversion on the regular channel"]
    #[inline(always)]
    pub fn rswstart(&self) -> RSWSTART_R {
        RSWSTART_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Continuous mode selection for regular conversions"]
    #[inline(always)]
    pub fn rcont(&self) -> RCONT_R {
        RCONT_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Launch regular conversion synchronously with DFSDM0"]
    #[inline(always)]
    pub fn rsync(&self) -> RSYNC_R {
        RSYNC_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - DMA channel enabled to read data for the regular conversion"]
    #[inline(always)]
    pub fn rdmaen(&self) -> RDMAEN_R {
        RDMAEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Regular channel selection"]
    #[inline(always)]
    pub fn rch(&self) -> RCH_R {
        RCH_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 29 - Fast conversion mode selection for regular conversions"]
    #[inline(always)]
    pub fn fast(&self) -> FAST_R {
        FAST_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Analog watchdog fast mode select"]
    #[inline(always)]
    pub fn awfsel(&self) -> AWFSEL_R {
        AWFSEL_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DFSDM enable"]
    #[inline(always)]
    #[must_use]
    pub fn dfen(&mut self) -> DFEN_W<CR1rs> {
        DFEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Start a conversion of the injected group of channels"]
    #[inline(always)]
    #[must_use]
    pub fn jswstart(&mut self) -> JSWSTART_W<CR1rs> {
        JSWSTART_W::new(self, 1)
    }
    #[doc = "Bit 3 - Launch an injected conversion synchronously with the DFSDM0 JSWSTART trigger"]
    #[inline(always)]
    #[must_use]
    pub fn jsync(&mut self) -> JSYNC_W<CR1rs> {
        JSYNC_W::new(self, 3)
    }
    #[doc = "Bit 4 - Scanning conversion mode for injected conversions"]
    #[inline(always)]
    #[must_use]
    pub fn jscan(&mut self) -> JSCAN_W<CR1rs> {
        JSCAN_W::new(self, 4)
    }
    #[doc = "Bit 5 - DMA channel enabled to read data for the injected channel group"]
    #[inline(always)]
    #[must_use]
    pub fn jdmaen(&mut self) -> JDMAEN_W<CR1rs> {
        JDMAEN_W::new(self, 5)
    }
    #[doc = "Bits 8:12 - Trigger signal selection for launching injected conversions"]
    #[inline(always)]
    #[must_use]
    pub fn jextsel(&mut self) -> JEXTSEL_W<CR1rs> {
        JEXTSEL_W::new(self, 8)
    }
    #[doc = "Bits 13:14 - Trigger enable and trigger edge selection for injected conversions"]
    #[inline(always)]
    #[must_use]
    pub fn jexten(&mut self) -> JEXTEN_W<CR1rs> {
        JEXTEN_W::new(self, 13)
    }
    #[doc = "Bit 17 - Software start of a conversion on the regular channel"]
    #[inline(always)]
    #[must_use]
    pub fn rswstart(&mut self) -> RSWSTART_W<CR1rs> {
        RSWSTART_W::new(self, 17)
    }
    #[doc = "Bit 18 - Continuous mode selection for regular conversions"]
    #[inline(always)]
    #[must_use]
    pub fn rcont(&mut self) -> RCONT_W<CR1rs> {
        RCONT_W::new(self, 18)
    }
    #[doc = "Bit 19 - Launch regular conversion synchronously with DFSDM0"]
    #[inline(always)]
    #[must_use]
    pub fn rsync(&mut self) -> RSYNC_W<CR1rs> {
        RSYNC_W::new(self, 19)
    }
    #[doc = "Bit 21 - DMA channel enabled to read data for the regular conversion"]
    #[inline(always)]
    #[must_use]
    pub fn rdmaen(&mut self) -> RDMAEN_W<CR1rs> {
        RDMAEN_W::new(self, 21)
    }
    #[doc = "Bits 24:26 - Regular channel selection"]
    #[inline(always)]
    #[must_use]
    pub fn rch(&mut self) -> RCH_W<CR1rs> {
        RCH_W::new(self, 24)
    }
    #[doc = "Bit 29 - Fast conversion mode selection for regular conversions"]
    #[inline(always)]
    #[must_use]
    pub fn fast(&mut self) -> FAST_W<CR1rs> {
        FAST_W::new(self, 29)
    }
    #[doc = "Bit 30 - Analog watchdog fast mode select"]
    #[inline(always)]
    #[must_use]
    pub fn awfsel(&mut self) -> AWFSEL_W<CR1rs> {
        AWFSEL_W::new(self, 30)
    }
}
#[doc = "control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR1rs;
impl crate::RegisterSpec for CR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for CR1rs {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for CR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for CR1rs {
    const RESET_VALUE: u32 = 0;
}
