#[doc = "Register `CFGR1` reader"]
pub type R = crate::R<CFGR1rs>;
#[doc = "Register `CFGR1` writer"]
pub type W = crate::W<CFGR1rs>;
#[doc = "system clock and trace clock switch Set and reset by software to select system clock and trace clock sources (sys_ck). Set by hardware in order to: - force the selection of the HSI or CSI (depending on STOPWUCK selection) when leaving a system Stop mode - force the selection of the HSI in case of failure of the HSE when used directly or indirectly as system clock others: reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SW {
    #[doc = "0: HSI selected as system clock"]
    Hsi = 0,
    #[doc = "1: CSI selected as system clock"]
    Csi = 1,
    #[doc = "2: HSE selected as system clock"]
    Hse = 2,
    #[doc = "3: PLL1 selected as system clock"]
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
#[doc = "Field `SW` reader - system clock and trace clock switch Set and reset by software to select system clock and trace clock sources (sys_ck). Set by hardware in order to: - force the selection of the HSI or CSI (depending on STOPWUCK selection) when leaving a system Stop mode - force the selection of the HSI in case of failure of the HSE when used directly or indirectly as system clock others: reserved"]
pub type SW_R = crate::FieldReader<SW>;
impl SW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SW {
        match self.bits {
            0 => SW::Hsi,
            1 => SW::Csi,
            2 => SW::Hse,
            3 => SW::Pll1,
            _ => unreachable!(),
        }
    }
    #[doc = "HSI selected as system clock"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == SW::Hsi
    }
    #[doc = "CSI selected as system clock"]
    #[inline(always)]
    pub fn is_csi(&self) -> bool {
        *self == SW::Csi
    }
    #[doc = "HSE selected as system clock"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == SW::Hse
    }
    #[doc = "PLL1 selected as system clock"]
    #[inline(always)]
    pub fn is_pll1(&self) -> bool {
        *self == SW::Pll1
    }
}
#[doc = "Field `SW` writer - system clock and trace clock switch Set and reset by software to select system clock and trace clock sources (sys_ck). Set by hardware in order to: - force the selection of the HSI or CSI (depending on STOPWUCK selection) when leaving a system Stop mode - force the selection of the HSI in case of failure of the HSE when used directly or indirectly as system clock others: reserved"]
pub type SW_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, SW>;
impl<'a, REG> SW_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "HSI selected as system clock"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut crate::W<REG> {
        self.variant(SW::Hsi)
    }
    #[doc = "CSI selected as system clock"]
    #[inline(always)]
    pub fn csi(self) -> &'a mut crate::W<REG> {
        self.variant(SW::Csi)
    }
    #[doc = "HSE selected as system clock"]
    #[inline(always)]
    pub fn hse(self) -> &'a mut crate::W<REG> {
        self.variant(SW::Hse)
    }
    #[doc = "PLL1 selected as system clock"]
    #[inline(always)]
    pub fn pll1(self) -> &'a mut crate::W<REG> {
        self.variant(SW::Pll1)
    }
}
#[doc = "system clock switch status Set and reset by hardware to indicate which clock source is used as system clock. 000: HSI used as system clock (hsi_ck) (default after reset). others: reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SWSR {
    #[doc = "0: HSI oscillator used as system clock"]
    Hsi = 0,
    #[doc = "1: CSI oscillator used as system clock"]
    Csi = 1,
    #[doc = "2: HSE oscillator used as system clock"]
    Hse = 2,
    #[doc = "3: PLL1 used as system clock"]
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
#[doc = "Field `SWS` reader - system clock switch status Set and reset by hardware to indicate which clock source is used as system clock. 000: HSI used as system clock (hsi_ck) (default after reset). others: reserved"]
pub type SWS_R = crate::FieldReader<SWSR>;
impl SWS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SWSR {
        match self.bits {
            0 => SWSR::Hsi,
            1 => SWSR::Csi,
            2 => SWSR::Hse,
            3 => SWSR::Pll1,
            _ => unreachable!(),
        }
    }
    #[doc = "HSI oscillator used as system clock"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == SWSR::Hsi
    }
    #[doc = "CSI oscillator used as system clock"]
    #[inline(always)]
    pub fn is_csi(&self) -> bool {
        *self == SWSR::Csi
    }
    #[doc = "HSE oscillator used as system clock"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == SWSR::Hse
    }
    #[doc = "PLL1 used as system clock"]
    #[inline(always)]
    pub fn is_pll1(&self) -> bool {
        *self == SWSR::Pll1
    }
}
#[doc = "system clock selection after a wakeup from system Stop Set and reset by software to select the system wakeup clock from system Stop. The selected clock is also used as emergency clock for the clock security system (CSS) on HSE. 0: HSI selected as wakeup clock from system Stop (default after reset) STOPWUCK must not be modified when CSS is enabled (by HSECSSON bit) and the system clock is HSE (SWS = 10) or a switch on HSE is requested (SW =10).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOPWUCK {
    #[doc = "0: HSI selected as wake up clock from system Stop"]
    Hsi = 0,
    #[doc = "1: CSI selected as wake up clock from system Stop"]
    Csi = 1,
}
impl From<STOPWUCK> for bool {
    #[inline(always)]
    fn from(variant: STOPWUCK) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOPWUCK` reader - system clock selection after a wakeup from system Stop Set and reset by software to select the system wakeup clock from system Stop. The selected clock is also used as emergency clock for the clock security system (CSS) on HSE. 0: HSI selected as wakeup clock from system Stop (default after reset) STOPWUCK must not be modified when CSS is enabled (by HSECSSON bit) and the system clock is HSE (SWS = 10) or a switch on HSE is requested (SW =10)."]
pub type STOPWUCK_R = crate::BitReader<STOPWUCK>;
impl STOPWUCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> STOPWUCK {
        match self.bits {
            false => STOPWUCK::Hsi,
            true => STOPWUCK::Csi,
        }
    }
    #[doc = "HSI selected as wake up clock from system Stop"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == STOPWUCK::Hsi
    }
    #[doc = "CSI selected as wake up clock from system Stop"]
    #[inline(always)]
    pub fn is_csi(&self) -> bool {
        *self == STOPWUCK::Csi
    }
}
#[doc = "Field `STOPWUCK` writer - system clock selection after a wakeup from system Stop Set and reset by software to select the system wakeup clock from system Stop. The selected clock is also used as emergency clock for the clock security system (CSS) on HSE. 0: HSI selected as wakeup clock from system Stop (default after reset) STOPWUCK must not be modified when CSS is enabled (by HSECSSON bit) and the system clock is HSE (SWS = 10) or a switch on HSE is requested (SW =10)."]
pub type STOPWUCK_W<'a, REG> = crate::BitWriter<'a, REG, STOPWUCK>;
impl<'a, REG> STOPWUCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HSI selected as wake up clock from system Stop"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut crate::W<REG> {
        self.variant(STOPWUCK::Hsi)
    }
    #[doc = "CSI selected as wake up clock from system Stop"]
    #[inline(always)]
    pub fn csi(self) -> &'a mut crate::W<REG> {
        self.variant(STOPWUCK::Csi)
    }
}
#[doc = "Field `STOPKERWUCK` reader - kernel clock selection after a wakeup from system Stop Set and reset by software to select the kernel wakeup clock from system Stop."]
pub use STOPWUCK_R as STOPKERWUCK_R;
#[doc = "Field `STOPKERWUCK` writer - kernel clock selection after a wakeup from system Stop Set and reset by software to select the kernel wakeup clock from system Stop."]
pub use STOPWUCK_W as STOPKERWUCK_W;
#[doc = "Field `RTCPRE` reader - HSE division factor for RTC clock Set and cleared by software to divide the HSE to generate a clock for RTC. Caution: The software must set these bits correctly to ensure that the clock supplied to the RTC is lower than 1 MHz. These bits must be configured if needed before selecting the RTC clock source. ..."]
pub type RTCPRE_R = crate::FieldReader;
#[doc = "Field `RTCPRE` writer - HSE division factor for RTC clock Set and cleared by software to divide the HSE to generate a clock for RTC. Caution: The software must set these bits correctly to ensure that the clock supplied to the RTC is lower than 1 MHz. These bits must be configured if needed before selecting the RTC clock source. ..."]
pub type RTCPRE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 6>;
#[doc = "timers clocks prescaler selection This bit is set and reset by software to control the clock frequency of all the timers connected to APB1 and APB2 domains.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIMPRE {
    #[doc = "0: Timer kernel clock equal to 2x pclk by default"]
    DefaultX2 = 0,
    #[doc = "1: Timer kernel clock equal to 4x pclk by default"]
    DefaultX4 = 1,
}
impl From<TIMPRE> for bool {
    #[inline(always)]
    fn from(variant: TIMPRE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMPRE` reader - timers clocks prescaler selection This bit is set and reset by software to control the clock frequency of all the timers connected to APB1 and APB2 domains."]
pub type TIMPRE_R = crate::BitReader<TIMPRE>;
impl TIMPRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIMPRE {
        match self.bits {
            false => TIMPRE::DefaultX2,
            true => TIMPRE::DefaultX4,
        }
    }
    #[doc = "Timer kernel clock equal to 2x pclk by default"]
    #[inline(always)]
    pub fn is_default_x2(&self) -> bool {
        *self == TIMPRE::DefaultX2
    }
    #[doc = "Timer kernel clock equal to 4x pclk by default"]
    #[inline(always)]
    pub fn is_default_x4(&self) -> bool {
        *self == TIMPRE::DefaultX4
    }
}
#[doc = "Field `TIMPRE` writer - timers clocks prescaler selection This bit is set and reset by software to control the clock frequency of all the timers connected to APB1 and APB2 domains."]
pub type TIMPRE_W<'a, REG> = crate::BitWriter<'a, REG, TIMPRE>;
impl<'a, REG> TIMPRE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timer kernel clock equal to 2x pclk by default"]
    #[inline(always)]
    pub fn default_x2(self) -> &'a mut crate::W<REG> {
        self.variant(TIMPRE::DefaultX2)
    }
    #[doc = "Timer kernel clock equal to 4x pclk by default"]
    #[inline(always)]
    pub fn default_x4(self) -> &'a mut crate::W<REG> {
        self.variant(TIMPRE::DefaultX4)
    }
}
#[doc = "Field `MCO1PRE` reader - MCO1 prescaler Set and cleared by software to configure the prescaler of the MCO1. Modification of this prescaler may generate glitches on MCO1. It is highly recommended to change this prescaler only after reset, before enabling the external oscillators and the PLLs. ..."]
pub type MCO1PRE_R = crate::FieldReader;
#[doc = "Field `MCO1PRE` writer - MCO1 prescaler Set and cleared by software to configure the prescaler of the MCO1. Modification of this prescaler may generate glitches on MCO1. It is highly recommended to change this prescaler only after reset, before enabling the external oscillators and the PLLs. ..."]
pub type MCO1PRE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4>;
#[doc = "Microcontroller clock output 1 Set and cleared by software. Clock source selection may generate glitches on MCO1. It is highly recommended to configure these bits only after reset, before enabling the external oscillators and the PLLs. others: reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MCO1SEL {
    #[doc = "0: HSI clock selected (hsi_ck)"]
    Hsi = 0,
    #[doc = "1: LSE clock selected (lse_ck)"]
    Lse = 1,
    #[doc = "2: HSE clock selected (hse_ck)"]
    Hse = 2,
    #[doc = "3: PLL1 clock selected (pll1_q_ck)"]
    Pll1Q = 3,
    #[doc = "4: HSI48 clock selected (hsi48_ck)"]
    Hsi48 = 4,
}
impl From<MCO1SEL> for u8 {
    #[inline(always)]
    fn from(variant: MCO1SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MCO1SEL {
    type Ux = u8;
}
#[doc = "Field `MCO1SEL` reader - Microcontroller clock output 1 Set and cleared by software. Clock source selection may generate glitches on MCO1. It is highly recommended to configure these bits only after reset, before enabling the external oscillators and the PLLs. others: reserved"]
pub type MCO1SEL_R = crate::FieldReader<MCO1SEL>;
impl MCO1SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MCO1SEL> {
        match self.bits {
            0 => Some(MCO1SEL::Hsi),
            1 => Some(MCO1SEL::Lse),
            2 => Some(MCO1SEL::Hse),
            3 => Some(MCO1SEL::Pll1Q),
            4 => Some(MCO1SEL::Hsi48),
            _ => None,
        }
    }
    #[doc = "HSI clock selected (hsi_ck)"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == MCO1SEL::Hsi
    }
    #[doc = "LSE clock selected (lse_ck)"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == MCO1SEL::Lse
    }
    #[doc = "HSE clock selected (hse_ck)"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == MCO1SEL::Hse
    }
    #[doc = "PLL1 clock selected (pll1_q_ck)"]
    #[inline(always)]
    pub fn is_pll1_q(&self) -> bool {
        *self == MCO1SEL::Pll1Q
    }
    #[doc = "HSI48 clock selected (hsi48_ck)"]
    #[inline(always)]
    pub fn is_hsi48(&self) -> bool {
        *self == MCO1SEL::Hsi48
    }
}
#[doc = "Field `MCO1SEL` writer - Microcontroller clock output 1 Set and cleared by software. Clock source selection may generate glitches on MCO1. It is highly recommended to configure these bits only after reset, before enabling the external oscillators and the PLLs. others: reserved"]
pub type MCO1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, MCO1SEL>;
impl<'a, REG> MCO1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "HSI clock selected (hsi_ck)"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut crate::W<REG> {
        self.variant(MCO1SEL::Hsi)
    }
    #[doc = "LSE clock selected (lse_ck)"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(MCO1SEL::Lse)
    }
    #[doc = "HSE clock selected (hse_ck)"]
    #[inline(always)]
    pub fn hse(self) -> &'a mut crate::W<REG> {
        self.variant(MCO1SEL::Hse)
    }
    #[doc = "PLL1 clock selected (pll1_q_ck)"]
    #[inline(always)]
    pub fn pll1_q(self) -> &'a mut crate::W<REG> {
        self.variant(MCO1SEL::Pll1Q)
    }
    #[doc = "HSI48 clock selected (hsi48_ck)"]
    #[inline(always)]
    pub fn hsi48(self) -> &'a mut crate::W<REG> {
        self.variant(MCO1SEL::Hsi48)
    }
}
#[doc = "Field `MCO2PRE` reader - MCO2 prescaler Set and cleared by software to configure the prescaler of the MCO2. Modification of this prescaler may generate glitches on MCO2. It is highly recommended to change this prescaler only after reset, before enabling the external oscillators and the PLLs. ..."]
pub type MCO2PRE_R = crate::FieldReader;
#[doc = "Field `MCO2PRE` writer - MCO2 prescaler Set and cleared by software to configure the prescaler of the MCO2. Modification of this prescaler may generate glitches on MCO2. It is highly recommended to change this prescaler only after reset, before enabling the external oscillators and the PLLs. ..."]
pub type MCO2PRE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4>;
#[doc = "microcontroller clock output 2 Set and cleared by software. Clock source selection may generate glitches on MCO2. It is highly recommended to configure these bits only after reset, before enabling the external oscillators and the PLLs. others: reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MCO2SEL {
    #[doc = "0: System clock selected (sys_ck)"]
    Sysclk = 0,
    #[doc = "1: PLL2 oscillator clock selected (pll2_p_ck)"]
    Pll2P = 1,
    #[doc = "2: HSE clock selected (hse_ck)"]
    Hse = 2,
    #[doc = "3: PLL1 clock selected (pll1_p_ck)"]
    Pll1P = 3,
    #[doc = "4: CSI clock selected (csi_ck)"]
    Csi = 4,
    #[doc = "5: LSI clock selected (lsi_ck)"]
    Lsi = 5,
}
impl From<MCO2SEL> for u8 {
    #[inline(always)]
    fn from(variant: MCO2SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MCO2SEL {
    type Ux = u8;
}
#[doc = "Field `MCO2SEL` reader - microcontroller clock output 2 Set and cleared by software. Clock source selection may generate glitches on MCO2. It is highly recommended to configure these bits only after reset, before enabling the external oscillators and the PLLs. others: reserved"]
pub type MCO2SEL_R = crate::FieldReader<MCO2SEL>;
impl MCO2SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MCO2SEL> {
        match self.bits {
            0 => Some(MCO2SEL::Sysclk),
            1 => Some(MCO2SEL::Pll2P),
            2 => Some(MCO2SEL::Hse),
            3 => Some(MCO2SEL::Pll1P),
            4 => Some(MCO2SEL::Csi),
            5 => Some(MCO2SEL::Lsi),
            _ => None,
        }
    }
    #[doc = "System clock selected (sys_ck)"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == MCO2SEL::Sysclk
    }
    #[doc = "PLL2 oscillator clock selected (pll2_p_ck)"]
    #[inline(always)]
    pub fn is_pll2_p(&self) -> bool {
        *self == MCO2SEL::Pll2P
    }
    #[doc = "HSE clock selected (hse_ck)"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == MCO2SEL::Hse
    }
    #[doc = "PLL1 clock selected (pll1_p_ck)"]
    #[inline(always)]
    pub fn is_pll1_p(&self) -> bool {
        *self == MCO2SEL::Pll1P
    }
    #[doc = "CSI clock selected (csi_ck)"]
    #[inline(always)]
    pub fn is_csi(&self) -> bool {
        *self == MCO2SEL::Csi
    }
    #[doc = "LSI clock selected (lsi_ck)"]
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == MCO2SEL::Lsi
    }
}
#[doc = "Field `MCO2SEL` writer - microcontroller clock output 2 Set and cleared by software. Clock source selection may generate glitches on MCO2. It is highly recommended to configure these bits only after reset, before enabling the external oscillators and the PLLs. others: reserved"]
pub type MCO2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, MCO2SEL>;
impl<'a, REG> MCO2SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "System clock selected (sys_ck)"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut crate::W<REG> {
        self.variant(MCO2SEL::Sysclk)
    }
    #[doc = "PLL2 oscillator clock selected (pll2_p_ck)"]
    #[inline(always)]
    pub fn pll2_p(self) -> &'a mut crate::W<REG> {
        self.variant(MCO2SEL::Pll2P)
    }
    #[doc = "HSE clock selected (hse_ck)"]
    #[inline(always)]
    pub fn hse(self) -> &'a mut crate::W<REG> {
        self.variant(MCO2SEL::Hse)
    }
    #[doc = "PLL1 clock selected (pll1_p_ck)"]
    #[inline(always)]
    pub fn pll1_p(self) -> &'a mut crate::W<REG> {
        self.variant(MCO2SEL::Pll1P)
    }
    #[doc = "CSI clock selected (csi_ck)"]
    #[inline(always)]
    pub fn csi(self) -> &'a mut crate::W<REG> {
        self.variant(MCO2SEL::Csi)
    }
    #[doc = "LSI clock selected (lsi_ck)"]
    #[inline(always)]
    pub fn lsi(self) -> &'a mut crate::W<REG> {
        self.variant(MCO2SEL::Lsi)
    }
}
impl R {
    #[doc = "Bits 0:1 - system clock and trace clock switch Set and reset by software to select system clock and trace clock sources (sys_ck). Set by hardware in order to: - force the selection of the HSI or CSI (depending on STOPWUCK selection) when leaving a system Stop mode - force the selection of the HSI in case of failure of the HSE when used directly or indirectly as system clock others: reserved"]
    #[inline(always)]
    pub fn sw(&self) -> SW_R {
        SW_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 3:4 - system clock switch status Set and reset by hardware to indicate which clock source is used as system clock. 000: HSI used as system clock (hsi_ck) (default after reset). others: reserved"]
    #[inline(always)]
    pub fn sws(&self) -> SWS_R {
        SWS_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 6 - system clock selection after a wakeup from system Stop Set and reset by software to select the system wakeup clock from system Stop. The selected clock is also used as emergency clock for the clock security system (CSS) on HSE. 0: HSI selected as wakeup clock from system Stop (default after reset) STOPWUCK must not be modified when CSS is enabled (by HSECSSON bit) and the system clock is HSE (SWS = 10) or a switch on HSE is requested (SW =10)."]
    #[inline(always)]
    pub fn stopwuck(&self) -> STOPWUCK_R {
        STOPWUCK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - kernel clock selection after a wakeup from system Stop Set and reset by software to select the kernel wakeup clock from system Stop."]
    #[inline(always)]
    pub fn stopkerwuck(&self) -> STOPKERWUCK_R {
        STOPKERWUCK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:13 - HSE division factor for RTC clock Set and cleared by software to divide the HSE to generate a clock for RTC. Caution: The software must set these bits correctly to ensure that the clock supplied to the RTC is lower than 1 MHz. These bits must be configured if needed before selecting the RTC clock source. ..."]
    #[inline(always)]
    pub fn rtcpre(&self) -> RTCPRE_R {
        RTCPRE_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 15 - timers clocks prescaler selection This bit is set and reset by software to control the clock frequency of all the timers connected to APB1 and APB2 domains."]
    #[inline(always)]
    pub fn timpre(&self) -> TIMPRE_R {
        TIMPRE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 18:21 - MCO1 prescaler Set and cleared by software to configure the prescaler of the MCO1. Modification of this prescaler may generate glitches on MCO1. It is highly recommended to change this prescaler only after reset, before enabling the external oscillators and the PLLs. ..."]
    #[inline(always)]
    pub fn mco1pre(&self) -> MCO1PRE_R {
        MCO1PRE_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 22:24 - Microcontroller clock output 1 Set and cleared by software. Clock source selection may generate glitches on MCO1. It is highly recommended to configure these bits only after reset, before enabling the external oscillators and the PLLs. others: reserved"]
    #[inline(always)]
    pub fn mco1sel(&self) -> MCO1SEL_R {
        MCO1SEL_R::new(((self.bits >> 22) & 7) as u8)
    }
    #[doc = "Bits 25:28 - MCO2 prescaler Set and cleared by software to configure the prescaler of the MCO2. Modification of this prescaler may generate glitches on MCO2. It is highly recommended to change this prescaler only after reset, before enabling the external oscillators and the PLLs. ..."]
    #[inline(always)]
    pub fn mco2pre(&self) -> MCO2PRE_R {
        MCO2PRE_R::new(((self.bits >> 25) & 0x0f) as u8)
    }
    #[doc = "Bits 29:31 - microcontroller clock output 2 Set and cleared by software. Clock source selection may generate glitches on MCO2. It is highly recommended to configure these bits only after reset, before enabling the external oscillators and the PLLs. others: reserved"]
    #[inline(always)]
    pub fn mco2sel(&self) -> MCO2SEL_R {
        MCO2SEL_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - system clock and trace clock switch Set and reset by software to select system clock and trace clock sources (sys_ck). Set by hardware in order to: - force the selection of the HSI or CSI (depending on STOPWUCK selection) when leaving a system Stop mode - force the selection of the HSI in case of failure of the HSE when used directly or indirectly as system clock others: reserved"]
    #[inline(always)]
    #[must_use]
    pub fn sw(&mut self) -> SW_W<CFGR1rs> {
        SW_W::new(self, 0)
    }
    #[doc = "Bit 6 - system clock selection after a wakeup from system Stop Set and reset by software to select the system wakeup clock from system Stop. The selected clock is also used as emergency clock for the clock security system (CSS) on HSE. 0: HSI selected as wakeup clock from system Stop (default after reset) STOPWUCK must not be modified when CSS is enabled (by HSECSSON bit) and the system clock is HSE (SWS = 10) or a switch on HSE is requested (SW =10)."]
    #[inline(always)]
    #[must_use]
    pub fn stopwuck(&mut self) -> STOPWUCK_W<CFGR1rs> {
        STOPWUCK_W::new(self, 6)
    }
    #[doc = "Bit 7 - kernel clock selection after a wakeup from system Stop Set and reset by software to select the kernel wakeup clock from system Stop."]
    #[inline(always)]
    #[must_use]
    pub fn stopkerwuck(&mut self) -> STOPKERWUCK_W<CFGR1rs> {
        STOPKERWUCK_W::new(self, 7)
    }
    #[doc = "Bits 8:13 - HSE division factor for RTC clock Set and cleared by software to divide the HSE to generate a clock for RTC. Caution: The software must set these bits correctly to ensure that the clock supplied to the RTC is lower than 1 MHz. These bits must be configured if needed before selecting the RTC clock source. ..."]
    #[inline(always)]
    #[must_use]
    pub fn rtcpre(&mut self) -> RTCPRE_W<CFGR1rs> {
        RTCPRE_W::new(self, 8)
    }
    #[doc = "Bit 15 - timers clocks prescaler selection This bit is set and reset by software to control the clock frequency of all the timers connected to APB1 and APB2 domains."]
    #[inline(always)]
    #[must_use]
    pub fn timpre(&mut self) -> TIMPRE_W<CFGR1rs> {
        TIMPRE_W::new(self, 15)
    }
    #[doc = "Bits 18:21 - MCO1 prescaler Set and cleared by software to configure the prescaler of the MCO1. Modification of this prescaler may generate glitches on MCO1. It is highly recommended to change this prescaler only after reset, before enabling the external oscillators and the PLLs. ..."]
    #[inline(always)]
    #[must_use]
    pub fn mco1pre(&mut self) -> MCO1PRE_W<CFGR1rs> {
        MCO1PRE_W::new(self, 18)
    }
    #[doc = "Bits 22:24 - Microcontroller clock output 1 Set and cleared by software. Clock source selection may generate glitches on MCO1. It is highly recommended to configure these bits only after reset, before enabling the external oscillators and the PLLs. others: reserved"]
    #[inline(always)]
    #[must_use]
    pub fn mco1sel(&mut self) -> MCO1SEL_W<CFGR1rs> {
        MCO1SEL_W::new(self, 22)
    }
    #[doc = "Bits 25:28 - MCO2 prescaler Set and cleared by software to configure the prescaler of the MCO2. Modification of this prescaler may generate glitches on MCO2. It is highly recommended to change this prescaler only after reset, before enabling the external oscillators and the PLLs. ..."]
    #[inline(always)]
    #[must_use]
    pub fn mco2pre(&mut self) -> MCO2PRE_W<CFGR1rs> {
        MCO2PRE_W::new(self, 25)
    }
    #[doc = "Bits 29:31 - microcontroller clock output 2 Set and cleared by software. Clock source selection may generate glitches on MCO2. It is highly recommended to configure these bits only after reset, before enabling the external oscillators and the PLLs. others: reserved"]
    #[inline(always)]
    #[must_use]
    pub fn mco2sel(&mut self) -> MCO2SEL_W<CFGR1rs> {
        MCO2SEL_W::new(self, 29)
    }
}
#[doc = "RCC clock configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
