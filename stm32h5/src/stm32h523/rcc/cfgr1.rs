///Register `CFGR1` reader
pub type R = crate::R<CFGR1rs>;
///Register `CFGR1` writer
pub type W = crate::W<CFGR1rs>;
/**system clock and trace clock switch

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
///Field `SW` reader - system clock and trace clock switch
pub type SW_R = crate::FieldReader<SW>;
impl SW_R {
    ///Get enumerated values variant
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
///Field `SW` writer - system clock and trace clock switch
pub type SW_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SW, crate::Safe>;
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
/**system clock switch status

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
///Field `SWS` reader - system clock switch status
pub type SWS_R = crate::FieldReader<SWSR>;
impl SWS_R {
    ///Get enumerated values variant
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
/**system clock selection after a wakeup from system Stop

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
///Field `STOPWUCK` reader - system clock selection after a wakeup from system Stop
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
///Field `STOPWUCK` writer - system clock selection after a wakeup from system Stop
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
///Field `STOPKERWUCK` reader - kernel clock selection after a wakeup from system Stop
pub use STOPWUCK_R as STOPKERWUCK_R;
///Field `STOPKERWUCK` writer - kernel clock selection after a wakeup from system Stop
pub use STOPWUCK_W as STOPKERWUCK_W;
///Field `RTCPRE` reader - HSE division factor for RTC clock
pub type RTCPRE_R = crate::FieldReader;
///Field `RTCPRE` writer - HSE division factor for RTC clock
pub type RTCPRE_W<'a, REG> = crate::FieldWriter<'a, REG, 6, u8, crate::Safe>;
/**timers clocks prescaler selection

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
///Field `TIMPRE` reader - timers clocks prescaler selection
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
///Field `TIMPRE` writer - timers clocks prescaler selection
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
/**Microcontroller clock output 1

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MCO1SEL {
    ///0: HSI clock selected (hsi_ck)
    Hsi = 0,
    ///1: LSE clock selected (lse_ck)
    Lse = 1,
    ///2: HSE clock selected (hse_ck)
    Hse = 2,
    ///3: PLL1 clock selected (pll1_q_ck)
    Pll1Q = 3,
    ///4: HSI48 clock selected (hsi48_ck)
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
impl crate::IsEnum for MCO1SEL {}
///Field `MCO1SEL` reader - Microcontroller clock output 1
pub type MCO1SEL_R = crate::FieldReader<MCO1SEL>;
impl MCO1SEL_R {
    ///Get enumerated values variant
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
    ///HSI clock selected (hsi_ck)
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == MCO1SEL::Hsi
    }
    ///LSE clock selected (lse_ck)
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == MCO1SEL::Lse
    }
    ///HSE clock selected (hse_ck)
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == MCO1SEL::Hse
    }
    ///PLL1 clock selected (pll1_q_ck)
    #[inline(always)]
    pub fn is_pll1_q(&self) -> bool {
        *self == MCO1SEL::Pll1Q
    }
    ///HSI48 clock selected (hsi48_ck)
    #[inline(always)]
    pub fn is_hsi48(&self) -> bool {
        *self == MCO1SEL::Hsi48
    }
}
///Field `MCO1SEL` writer - Microcontroller clock output 1
pub type MCO1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, MCO1SEL>;
impl<'a, REG> MCO1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///HSI clock selected (hsi_ck)
    #[inline(always)]
    pub fn hsi(self) -> &'a mut crate::W<REG> {
        self.variant(MCO1SEL::Hsi)
    }
    ///LSE clock selected (lse_ck)
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(MCO1SEL::Lse)
    }
    ///HSE clock selected (hse_ck)
    #[inline(always)]
    pub fn hse(self) -> &'a mut crate::W<REG> {
        self.variant(MCO1SEL::Hse)
    }
    ///PLL1 clock selected (pll1_q_ck)
    #[inline(always)]
    pub fn pll1_q(self) -> &'a mut crate::W<REG> {
        self.variant(MCO1SEL::Pll1Q)
    }
    ///HSI48 clock selected (hsi48_ck)
    #[inline(always)]
    pub fn hsi48(self) -> &'a mut crate::W<REG> {
        self.variant(MCO1SEL::Hsi48)
    }
}
///Field `MCO2PRE` reader - MCO2 prescaler
pub type MCO2PRE_R = crate::FieldReader;
///Field `MCO2PRE` writer - MCO2 prescaler
pub type MCO2PRE_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
/**microcontroller clock output 2

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MCO2SEL {
    ///0: System clock selected (sys_ck)
    Sysclk = 0,
    ///1: PLL2 oscillator clock selected (pll2_p_ck)
    Pll2P = 1,
    ///2: HSE clock selected (hse_ck)
    Hse = 2,
    ///3: PLL1 clock selected (pll1_p_ck)
    Pll1P = 3,
    ///4: CSI clock selected (csi_ck)
    Csi = 4,
    ///5: LSI clock selected (lsi_ck)
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
impl crate::IsEnum for MCO2SEL {}
///Field `MCO2SEL` reader - microcontroller clock output 2
pub type MCO2SEL_R = crate::FieldReader<MCO2SEL>;
impl MCO2SEL_R {
    ///Get enumerated values variant
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
    ///System clock selected (sys_ck)
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == MCO2SEL::Sysclk
    }
    ///PLL2 oscillator clock selected (pll2_p_ck)
    #[inline(always)]
    pub fn is_pll2_p(&self) -> bool {
        *self == MCO2SEL::Pll2P
    }
    ///HSE clock selected (hse_ck)
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == MCO2SEL::Hse
    }
    ///PLL1 clock selected (pll1_p_ck)
    #[inline(always)]
    pub fn is_pll1_p(&self) -> bool {
        *self == MCO2SEL::Pll1P
    }
    ///CSI clock selected (csi_ck)
    #[inline(always)]
    pub fn is_csi(&self) -> bool {
        *self == MCO2SEL::Csi
    }
    ///LSI clock selected (lsi_ck)
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == MCO2SEL::Lsi
    }
}
///Field `MCO2SEL` writer - microcontroller clock output 2
pub type MCO2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, MCO2SEL>;
impl<'a, REG> MCO2SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///System clock selected (sys_ck)
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut crate::W<REG> {
        self.variant(MCO2SEL::Sysclk)
    }
    ///PLL2 oscillator clock selected (pll2_p_ck)
    #[inline(always)]
    pub fn pll2_p(self) -> &'a mut crate::W<REG> {
        self.variant(MCO2SEL::Pll2P)
    }
    ///HSE clock selected (hse_ck)
    #[inline(always)]
    pub fn hse(self) -> &'a mut crate::W<REG> {
        self.variant(MCO2SEL::Hse)
    }
    ///PLL1 clock selected (pll1_p_ck)
    #[inline(always)]
    pub fn pll1_p(self) -> &'a mut crate::W<REG> {
        self.variant(MCO2SEL::Pll1P)
    }
    ///CSI clock selected (csi_ck)
    #[inline(always)]
    pub fn csi(self) -> &'a mut crate::W<REG> {
        self.variant(MCO2SEL::Csi)
    }
    ///LSI clock selected (lsi_ck)
    #[inline(always)]
    pub fn lsi(self) -> &'a mut crate::W<REG> {
        self.variant(MCO2SEL::Lsi)
    }
}
impl R {
    ///Bits 0:1 - system clock and trace clock switch
    #[inline(always)]
    pub fn sw(&self) -> SW_R {
        SW_R::new((self.bits & 3) as u8)
    }
    ///Bits 3:4 - system clock switch status
    #[inline(always)]
    pub fn sws(&self) -> SWS_R {
        SWS_R::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bit 6 - system clock selection after a wakeup from system Stop
    #[inline(always)]
    pub fn stopwuck(&self) -> STOPWUCK_R {
        STOPWUCK_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - kernel clock selection after a wakeup from system Stop
    #[inline(always)]
    pub fn stopkerwuck(&self) -> STOPKERWUCK_R {
        STOPKERWUCK_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:13 - HSE division factor for RTC clock
    #[inline(always)]
    pub fn rtcpre(&self) -> RTCPRE_R {
        RTCPRE_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    ///Bit 15 - timers clocks prescaler selection
    #[inline(always)]
    pub fn timpre(&self) -> TIMPRE_R {
        TIMPRE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 18:21 - MCO1 prescaler
    #[inline(always)]
    pub fn mco1pre(&self) -> MCO1PRE_R {
        MCO1PRE_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    ///Bits 22:24 - Microcontroller clock output 1
    #[inline(always)]
    pub fn mco1sel(&self) -> MCO1SEL_R {
        MCO1SEL_R::new(((self.bits >> 22) & 7) as u8)
    }
    ///Bits 25:28 - MCO2 prescaler
    #[inline(always)]
    pub fn mco2pre(&self) -> MCO2PRE_R {
        MCO2PRE_R::new(((self.bits >> 25) & 0x0f) as u8)
    }
    ///Bits 29:31 - microcontroller clock output 2
    #[inline(always)]
    pub fn mco2sel(&self) -> MCO2SEL_R {
        MCO2SEL_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR1")
            .field("sw", &self.sw())
            .field("sws", &self.sws())
            .field("stopwuck", &self.stopwuck())
            .field("stopkerwuck", &self.stopkerwuck())
            .field("rtcpre", &self.rtcpre())
            .field("timpre", &self.timpre())
            .field("mco1pre", &self.mco1pre())
            .field("mco1sel", &self.mco1sel())
            .field("mco2pre", &self.mco2pre())
            .field("mco2sel", &self.mco2sel())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - system clock and trace clock switch
    #[inline(always)]
    pub fn sw(&mut self) -> SW_W<CFGR1rs> {
        SW_W::new(self, 0)
    }
    ///Bit 6 - system clock selection after a wakeup from system Stop
    #[inline(always)]
    pub fn stopwuck(&mut self) -> STOPWUCK_W<CFGR1rs> {
        STOPWUCK_W::new(self, 6)
    }
    ///Bit 7 - kernel clock selection after a wakeup from system Stop
    #[inline(always)]
    pub fn stopkerwuck(&mut self) -> STOPKERWUCK_W<CFGR1rs> {
        STOPKERWUCK_W::new(self, 7)
    }
    ///Bits 8:13 - HSE division factor for RTC clock
    #[inline(always)]
    pub fn rtcpre(&mut self) -> RTCPRE_W<CFGR1rs> {
        RTCPRE_W::new(self, 8)
    }
    ///Bit 15 - timers clocks prescaler selection
    #[inline(always)]
    pub fn timpre(&mut self) -> TIMPRE_W<CFGR1rs> {
        TIMPRE_W::new(self, 15)
    }
    ///Bits 18:21 - MCO1 prescaler
    #[inline(always)]
    pub fn mco1pre(&mut self) -> MCO1PRE_W<CFGR1rs> {
        MCO1PRE_W::new(self, 18)
    }
    ///Bits 22:24 - Microcontroller clock output 1
    #[inline(always)]
    pub fn mco1sel(&mut self) -> MCO1SEL_W<CFGR1rs> {
        MCO1SEL_W::new(self, 22)
    }
    ///Bits 25:28 - MCO2 prescaler
    #[inline(always)]
    pub fn mco2pre(&mut self) -> MCO2PRE_W<CFGR1rs> {
        MCO2PRE_W::new(self, 25)
    }
    ///Bits 29:31 - microcontroller clock output 2
    #[inline(always)]
    pub fn mco2sel(&mut self) -> MCO2SEL_W<CFGR1rs> {
        MCO2SEL_W::new(self, 29)
    }
}
/**RCC clock configuration register1

You can [`read`](crate::Reg::read) this register and get [`cfgr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#RCC:CFGR1)*/
pub struct CFGR1rs;
impl crate::RegisterSpec for CFGR1rs {
    type Ux = u32;
}
///`read()` method returns [`cfgr1::R`](R) reader structure
impl crate::Readable for CFGR1rs {}
///`write(|w| ..)` method takes [`cfgr1::W`](W) writer structure
impl crate::Writable for CFGR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFGR1 to value 0
impl crate::Resettable for CFGR1rs {}
