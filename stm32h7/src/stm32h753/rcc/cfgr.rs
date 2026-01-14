///Register `CFGR` reader
pub type R = crate::R<CFGRrs>;
///Register `CFGR` writer
pub type W = crate::W<CFGRrs>;
/**System clock switch

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SW {
    ///0: HSI selected as system clock
    Hsi = 0,
    ///1: CSI selected as system clock
    Csi = 1,
    ///2: HSE selected as system clock
    Hse = 2,
    ///3: PLL1 selected as system clock
    Pll1 = 3,
}
impl From<SW> for u8 {
    #[inline(always)]
    fn from(variant: SW) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SW {
    type Ux = u8;
}
impl crate::IsEnum for SW {}
///Field `SW` reader - System clock switch
pub type SW_R = crate::FieldReader<SW>;
impl SW_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<SW> {
        match self.bits {
            0 => Some(SW::Hsi),
            1 => Some(SW::Csi),
            2 => Some(SW::Hse),
            3 => Some(SW::Pll1),
            _ => None,
        }
    }
    ///HSI selected as system clock
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == SW::Hsi
    }
    ///CSI selected as system clock
    #[inline(always)]
    pub fn is_csi(&self) -> bool {
        *self == SW::Csi
    }
    ///HSE selected as system clock
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == SW::Hse
    }
    ///PLL1 selected as system clock
    #[inline(always)]
    pub fn is_pll1(&self) -> bool {
        *self == SW::Pll1
    }
}
///Field `SW` writer - System clock switch
pub type SW_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SW>;
impl<'a, REG> SW_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///HSI selected as system clock
    #[inline(always)]
    pub fn hsi(self) -> &'a mut crate::W<REG> {
        self.variant(SW::Hsi)
    }
    ///CSI selected as system clock
    #[inline(always)]
    pub fn csi(self) -> &'a mut crate::W<REG> {
        self.variant(SW::Csi)
    }
    ///HSE selected as system clock
    #[inline(always)]
    pub fn hse(self) -> &'a mut crate::W<REG> {
        self.variant(SW::Hse)
    }
    ///PLL1 selected as system clock
    #[inline(always)]
    pub fn pll1(self) -> &'a mut crate::W<REG> {
        self.variant(SW::Pll1)
    }
}
/**System clock switch status

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SWSR {
    ///0: HSI oscillator used as system clock
    Hsi = 0,
    ///1: CSI oscillator used as system clock
    Csi = 1,
    ///2: HSE oscillator used as system clock
    Hse = 2,
    ///3: PLL1 used as system clock
    Pll1 = 3,
}
impl From<SWSR> for u8 {
    #[inline(always)]
    fn from(variant: SWSR) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SWSR {
    type Ux = u8;
}
impl crate::IsEnum for SWSR {}
///Field `SWS` reader - System clock switch status
pub type SWS_R = crate::FieldReader<SWSR>;
impl SWS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<SWSR> {
        match self.bits {
            0 => Some(SWSR::Hsi),
            1 => Some(SWSR::Csi),
            2 => Some(SWSR::Hse),
            3 => Some(SWSR::Pll1),
            _ => None,
        }
    }
    ///HSI oscillator used as system clock
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == SWSR::Hsi
    }
    ///CSI oscillator used as system clock
    #[inline(always)]
    pub fn is_csi(&self) -> bool {
        *self == SWSR::Csi
    }
    ///HSE oscillator used as system clock
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == SWSR::Hse
    }
    ///PLL1 used as system clock
    #[inline(always)]
    pub fn is_pll1(&self) -> bool {
        *self == SWSR::Pll1
    }
}
///Field `SWS` writer - System clock switch status
pub type SWS_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SWSR>;
impl<'a, REG> SWS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///HSI oscillator used as system clock
    #[inline(always)]
    pub fn hsi(self) -> &'a mut crate::W<REG> {
        self.variant(SWSR::Hsi)
    }
    ///CSI oscillator used as system clock
    #[inline(always)]
    pub fn csi(self) -> &'a mut crate::W<REG> {
        self.variant(SWSR::Csi)
    }
    ///HSE oscillator used as system clock
    #[inline(always)]
    pub fn hse(self) -> &'a mut crate::W<REG> {
        self.variant(SWSR::Hse)
    }
    ///PLL1 used as system clock
    #[inline(always)]
    pub fn pll1(self) -> &'a mut crate::W<REG> {
        self.variant(SWSR::Pll1)
    }
}
/**System clock selection after a wake up from system Stop

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOPWUCK {
    ///0: HSI selected as wake up clock from system Stop
    Hsi = 0,
    ///1: CSI selected as wake up clock from system Stop
    Csi = 1,
}
impl From<STOPWUCK> for bool {
    #[inline(always)]
    fn from(variant: STOPWUCK) -> Self {
        variant as u8 != 0
    }
}
///Field `STOPWUCK` reader - System clock selection after a wake up from system Stop
pub type STOPWUCK_R = crate::BitReader<STOPWUCK>;
impl STOPWUCK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> STOPWUCK {
        match self.bits {
            false => STOPWUCK::Hsi,
            true => STOPWUCK::Csi,
        }
    }
    ///HSI selected as wake up clock from system Stop
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == STOPWUCK::Hsi
    }
    ///CSI selected as wake up clock from system Stop
    #[inline(always)]
    pub fn is_csi(&self) -> bool {
        *self == STOPWUCK::Csi
    }
}
///Field `STOPWUCK` writer - System clock selection after a wake up from system Stop
pub type STOPWUCK_W<'a, REG> = crate::BitWriter<'a, REG, STOPWUCK>;
impl<'a, REG> STOPWUCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///HSI selected as wake up clock from system Stop
    #[inline(always)]
    pub fn hsi(self) -> &'a mut crate::W<REG> {
        self.variant(STOPWUCK::Hsi)
    }
    ///CSI selected as wake up clock from system Stop
    #[inline(always)]
    pub fn csi(self) -> &'a mut crate::W<REG> {
        self.variant(STOPWUCK::Csi)
    }
}
///Field `STOPKERWUCK` reader - Kernel clock selection after a wake up from system Stop
pub use STOPWUCK_R as STOPKERWUCK_R;
///Field `STOPKERWUCK` writer - Kernel clock selection after a wake up from system Stop
pub use STOPWUCK_W as STOPKERWUCK_W;
///Field `RTCPRE` reader - HSE division factor for RTC clock
pub type RTCPRE_R = crate::FieldReader;
///Field `RTCPRE` writer - HSE division factor for RTC clock
pub type RTCPRE_W<'a, REG> = crate::FieldWriter<'a, REG, 6, u8, crate::Safe>;
/**High Resolution Timer clock prescaler selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HRTIMSEL {
    ///0: The HRTIM prescaler clock source is the same as other timers (rcc_timy_ker_ck)
    TimyKer = 0,
    ///1: The HRTIM prescaler clock source is the CPU clock (c_ck)
    CCk = 1,
}
impl From<HRTIMSEL> for bool {
    #[inline(always)]
    fn from(variant: HRTIMSEL) -> Self {
        variant as u8 != 0
    }
}
///Field `HRTIMSEL` reader - High Resolution Timer clock prescaler selection
pub type HRTIMSEL_R = crate::BitReader<HRTIMSEL>;
impl HRTIMSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HRTIMSEL {
        match self.bits {
            false => HRTIMSEL::TimyKer,
            true => HRTIMSEL::CCk,
        }
    }
    ///The HRTIM prescaler clock source is the same as other timers (rcc_timy_ker_ck)
    #[inline(always)]
    pub fn is_timy_ker(&self) -> bool {
        *self == HRTIMSEL::TimyKer
    }
    ///The HRTIM prescaler clock source is the CPU clock (c_ck)
    #[inline(always)]
    pub fn is_c_ck(&self) -> bool {
        *self == HRTIMSEL::CCk
    }
}
///Field `HRTIMSEL` writer - High Resolution Timer clock prescaler selection
pub type HRTIMSEL_W<'a, REG> = crate::BitWriter<'a, REG, HRTIMSEL>;
impl<'a, REG> HRTIMSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The HRTIM prescaler clock source is the same as other timers (rcc_timy_ker_ck)
    #[inline(always)]
    pub fn timy_ker(self) -> &'a mut crate::W<REG> {
        self.variant(HRTIMSEL::TimyKer)
    }
    ///The HRTIM prescaler clock source is the CPU clock (c_ck)
    #[inline(always)]
    pub fn c_ck(self) -> &'a mut crate::W<REG> {
        self.variant(HRTIMSEL::CCk)
    }
}
/**Timers clocks prescaler selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIMPRE {
    ///0: Timer kernel clock equal to 2x pclk by default
    DefaultX2 = 0,
    ///1: Timer kernel clock equal to 4x pclk by default
    DefaultX4 = 1,
}
impl From<TIMPRE> for bool {
    #[inline(always)]
    fn from(variant: TIMPRE) -> Self {
        variant as u8 != 0
    }
}
///Field `TIMPRE` reader - Timers clocks prescaler selection
pub type TIMPRE_R = crate::BitReader<TIMPRE>;
impl TIMPRE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIMPRE {
        match self.bits {
            false => TIMPRE::DefaultX2,
            true => TIMPRE::DefaultX4,
        }
    }
    ///Timer kernel clock equal to 2x pclk by default
    #[inline(always)]
    pub fn is_default_x2(&self) -> bool {
        *self == TIMPRE::DefaultX2
    }
    ///Timer kernel clock equal to 4x pclk by default
    #[inline(always)]
    pub fn is_default_x4(&self) -> bool {
        *self == TIMPRE::DefaultX4
    }
}
///Field `TIMPRE` writer - Timers clocks prescaler selection
pub type TIMPRE_W<'a, REG> = crate::BitWriter<'a, REG, TIMPRE>;
impl<'a, REG> TIMPRE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Timer kernel clock equal to 2x pclk by default
    #[inline(always)]
    pub fn default_x2(self) -> &'a mut crate::W<REG> {
        self.variant(TIMPRE::DefaultX2)
    }
    ///Timer kernel clock equal to 4x pclk by default
    #[inline(always)]
    pub fn default_x4(self) -> &'a mut crate::W<REG> {
        self.variant(TIMPRE::DefaultX4)
    }
}
///Field `MCO1PRE` reader - MCO1 prescaler
pub type MCO1PRE_R = crate::FieldReader;
///Field `MCO1PRE` writer - MCO1 prescaler
pub type MCO1PRE_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
/**Micro-controller clock output 1

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MCO1 {
    ///0: HSI selected for micro-controller clock output
    Hsi = 0,
    ///1: LSE selected for micro-controller clock output
    Lse = 1,
    ///2: HSE selected for micro-controller clock output
    Hse = 2,
    ///3: pll1_q selected for micro-controller clock output
    Pll1Q = 3,
    ///4: HSI48 selected for micro-controller clock output
    Hsi48 = 4,
}
impl From<MCO1> for u8 {
    #[inline(always)]
    fn from(variant: MCO1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MCO1 {
    type Ux = u8;
}
impl crate::IsEnum for MCO1 {}
///Field `MCO1` reader - Micro-controller clock output 1
pub type MCO1_R = crate::FieldReader<MCO1>;
impl MCO1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<MCO1> {
        match self.bits {
            0 => Some(MCO1::Hsi),
            1 => Some(MCO1::Lse),
            2 => Some(MCO1::Hse),
            3 => Some(MCO1::Pll1Q),
            4 => Some(MCO1::Hsi48),
            _ => None,
        }
    }
    ///HSI selected for micro-controller clock output
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == MCO1::Hsi
    }
    ///LSE selected for micro-controller clock output
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == MCO1::Lse
    }
    ///HSE selected for micro-controller clock output
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == MCO1::Hse
    }
    ///pll1_q selected for micro-controller clock output
    #[inline(always)]
    pub fn is_pll1_q(&self) -> bool {
        *self == MCO1::Pll1Q
    }
    ///HSI48 selected for micro-controller clock output
    #[inline(always)]
    pub fn is_hsi48(&self) -> bool {
        *self == MCO1::Hsi48
    }
}
///Field `MCO1` writer - Micro-controller clock output 1
pub type MCO1_W<'a, REG> = crate::FieldWriter<'a, REG, 3, MCO1>;
impl<'a, REG> MCO1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///HSI selected for micro-controller clock output
    #[inline(always)]
    pub fn hsi(self) -> &'a mut crate::W<REG> {
        self.variant(MCO1::Hsi)
    }
    ///LSE selected for micro-controller clock output
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(MCO1::Lse)
    }
    ///HSE selected for micro-controller clock output
    #[inline(always)]
    pub fn hse(self) -> &'a mut crate::W<REG> {
        self.variant(MCO1::Hse)
    }
    ///pll1_q selected for micro-controller clock output
    #[inline(always)]
    pub fn pll1_q(self) -> &'a mut crate::W<REG> {
        self.variant(MCO1::Pll1Q)
    }
    ///HSI48 selected for micro-controller clock output
    #[inline(always)]
    pub fn hsi48(self) -> &'a mut crate::W<REG> {
        self.variant(MCO1::Hsi48)
    }
}
///Field `MCO2PRE` reader - MCO2 prescaler
pub type MCO2PRE_R = crate::FieldReader;
///Field `MCO2PRE` writer - MCO2 prescaler
pub type MCO2PRE_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
/**Micro-controller clock output 2

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MCO2 {
    ///0: System clock selected for micro-controller clock output
    Sysclk = 0,
    ///1: pll2_p selected for micro-controller clock output
    Pll2P = 1,
    ///2: HSE selected for micro-controller clock output
    Hse = 2,
    ///3: pll1_p selected for micro-controller clock output
    Pll1P = 3,
    ///4: CSI selected for micro-controller clock output
    Csi = 4,
    ///5: LSI selected for micro-controller clock output
    Lsi = 5,
}
impl From<MCO2> for u8 {
    #[inline(always)]
    fn from(variant: MCO2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MCO2 {
    type Ux = u8;
}
impl crate::IsEnum for MCO2 {}
///Field `MCO2` reader - Micro-controller clock output 2
pub type MCO2_R = crate::FieldReader<MCO2>;
impl MCO2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<MCO2> {
        match self.bits {
            0 => Some(MCO2::Sysclk),
            1 => Some(MCO2::Pll2P),
            2 => Some(MCO2::Hse),
            3 => Some(MCO2::Pll1P),
            4 => Some(MCO2::Csi),
            5 => Some(MCO2::Lsi),
            _ => None,
        }
    }
    ///System clock selected for micro-controller clock output
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == MCO2::Sysclk
    }
    ///pll2_p selected for micro-controller clock output
    #[inline(always)]
    pub fn is_pll2_p(&self) -> bool {
        *self == MCO2::Pll2P
    }
    ///HSE selected for micro-controller clock output
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == MCO2::Hse
    }
    ///pll1_p selected for micro-controller clock output
    #[inline(always)]
    pub fn is_pll1_p(&self) -> bool {
        *self == MCO2::Pll1P
    }
    ///CSI selected for micro-controller clock output
    #[inline(always)]
    pub fn is_csi(&self) -> bool {
        *self == MCO2::Csi
    }
    ///LSI selected for micro-controller clock output
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == MCO2::Lsi
    }
}
///Field `MCO2` writer - Micro-controller clock output 2
pub type MCO2_W<'a, REG> = crate::FieldWriter<'a, REG, 3, MCO2>;
impl<'a, REG> MCO2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///System clock selected for micro-controller clock output
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut crate::W<REG> {
        self.variant(MCO2::Sysclk)
    }
    ///pll2_p selected for micro-controller clock output
    #[inline(always)]
    pub fn pll2_p(self) -> &'a mut crate::W<REG> {
        self.variant(MCO2::Pll2P)
    }
    ///HSE selected for micro-controller clock output
    #[inline(always)]
    pub fn hse(self) -> &'a mut crate::W<REG> {
        self.variant(MCO2::Hse)
    }
    ///pll1_p selected for micro-controller clock output
    #[inline(always)]
    pub fn pll1_p(self) -> &'a mut crate::W<REG> {
        self.variant(MCO2::Pll1P)
    }
    ///CSI selected for micro-controller clock output
    #[inline(always)]
    pub fn csi(self) -> &'a mut crate::W<REG> {
        self.variant(MCO2::Csi)
    }
    ///LSI selected for micro-controller clock output
    #[inline(always)]
    pub fn lsi(self) -> &'a mut crate::W<REG> {
        self.variant(MCO2::Lsi)
    }
}
impl R {
    ///Bits 0:2 - System clock switch
    #[inline(always)]
    pub fn sw(&self) -> SW_R {
        SW_R::new((self.bits & 7) as u8)
    }
    ///Bits 3:5 - System clock switch status
    #[inline(always)]
    pub fn sws(&self) -> SWS_R {
        SWS_R::new(((self.bits >> 3) & 7) as u8)
    }
    ///Bit 6 - System clock selection after a wake up from system Stop
    #[inline(always)]
    pub fn stopwuck(&self) -> STOPWUCK_R {
        STOPWUCK_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Kernel clock selection after a wake up from system Stop
    #[inline(always)]
    pub fn stopkerwuck(&self) -> STOPKERWUCK_R {
        STOPKERWUCK_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:13 - HSE division factor for RTC clock
    #[inline(always)]
    pub fn rtcpre(&self) -> RTCPRE_R {
        RTCPRE_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    ///Bit 14 - High Resolution Timer clock prescaler selection
    #[inline(always)]
    pub fn hrtimsel(&self) -> HRTIMSEL_R {
        HRTIMSEL_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Timers clocks prescaler selection
    #[inline(always)]
    pub fn timpre(&self) -> TIMPRE_R {
        TIMPRE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 18:21 - MCO1 prescaler
    #[inline(always)]
    pub fn mco1pre(&self) -> MCO1PRE_R {
        MCO1PRE_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    ///Bits 22:24 - Micro-controller clock output 1
    #[inline(always)]
    pub fn mco1(&self) -> MCO1_R {
        MCO1_R::new(((self.bits >> 22) & 7) as u8)
    }
    ///Bits 25:28 - MCO2 prescaler
    #[inline(always)]
    pub fn mco2pre(&self) -> MCO2PRE_R {
        MCO2PRE_R::new(((self.bits >> 25) & 0x0f) as u8)
    }
    ///Bits 29:31 - Micro-controller clock output 2
    #[inline(always)]
    pub fn mco2(&self) -> MCO2_R {
        MCO2_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR")
            .field("sw", &self.sw())
            .field("sws", &self.sws())
            .field("stopwuck", &self.stopwuck())
            .field("stopkerwuck", &self.stopkerwuck())
            .field("rtcpre", &self.rtcpre())
            .field("hrtimsel", &self.hrtimsel())
            .field("timpre", &self.timpre())
            .field("mco1pre", &self.mco1pre())
            .field("mco1", &self.mco1())
            .field("mco2pre", &self.mco2pre())
            .field("mco2", &self.mco2())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - System clock switch
    #[inline(always)]
    pub fn sw(&mut self) -> SW_W<'_, CFGRrs> {
        SW_W::new(self, 0)
    }
    ///Bits 3:5 - System clock switch status
    #[inline(always)]
    pub fn sws(&mut self) -> SWS_W<'_, CFGRrs> {
        SWS_W::new(self, 3)
    }
    ///Bit 6 - System clock selection after a wake up from system Stop
    #[inline(always)]
    pub fn stopwuck(&mut self) -> STOPWUCK_W<'_, CFGRrs> {
        STOPWUCK_W::new(self, 6)
    }
    ///Bit 7 - Kernel clock selection after a wake up from system Stop
    #[inline(always)]
    pub fn stopkerwuck(&mut self) -> STOPKERWUCK_W<'_, CFGRrs> {
        STOPKERWUCK_W::new(self, 7)
    }
    ///Bits 8:13 - HSE division factor for RTC clock
    #[inline(always)]
    pub fn rtcpre(&mut self) -> RTCPRE_W<'_, CFGRrs> {
        RTCPRE_W::new(self, 8)
    }
    ///Bit 14 - High Resolution Timer clock prescaler selection
    #[inline(always)]
    pub fn hrtimsel(&mut self) -> HRTIMSEL_W<'_, CFGRrs> {
        HRTIMSEL_W::new(self, 14)
    }
    ///Bit 15 - Timers clocks prescaler selection
    #[inline(always)]
    pub fn timpre(&mut self) -> TIMPRE_W<'_, CFGRrs> {
        TIMPRE_W::new(self, 15)
    }
    ///Bits 18:21 - MCO1 prescaler
    #[inline(always)]
    pub fn mco1pre(&mut self) -> MCO1PRE_W<'_, CFGRrs> {
        MCO1PRE_W::new(self, 18)
    }
    ///Bits 22:24 - Micro-controller clock output 1
    #[inline(always)]
    pub fn mco1(&mut self) -> MCO1_W<'_, CFGRrs> {
        MCO1_W::new(self, 22)
    }
    ///Bits 25:28 - MCO2 prescaler
    #[inline(always)]
    pub fn mco2pre(&mut self) -> MCO2PRE_W<'_, CFGRrs> {
        MCO2PRE_W::new(self, 25)
    }
    ///Bits 29:31 - Micro-controller clock output 2
    #[inline(always)]
    pub fn mco2(&mut self) -> MCO2_W<'_, CFGRrs> {
        MCO2_W::new(self, 29)
    }
}
/**RCC Clock Configuration Register

You can [`read`](crate::Reg::read) this register and get [`cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H753.html#RCC:CFGR)*/
pub struct CFGRrs;
impl crate::RegisterSpec for CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`cfgr::R`](R) reader structure
impl crate::Readable for CFGRrs {}
///`write(|w| ..)` method takes [`cfgr::W`](W) writer structure
impl crate::Writable for CFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFGR to value 0
impl crate::Resettable for CFGRrs {}
