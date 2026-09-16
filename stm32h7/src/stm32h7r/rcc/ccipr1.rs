///Register `CCIPR1` reader
pub type R = crate::R<CCIPR1rs>;
///Register `CCIPR1` writer
pub type W = crate::W<CCIPR1rs>;
/**FMC kernel clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FMCSEL {
    ///0: hclk5 selected as peripheral clock
    RccHclk5 = 0,
    ///1: pll1_q selected as peripheral clock
    Pll1Q = 1,
    ///2: pll2_r selected as peripheral clock
    Pll2R = 2,
    ///3: hsi_ker selected as peripheral clock
    HsiKer = 3,
}
impl From<FMCSEL> for u8 {
    #[inline(always)]
    fn from(variant: FMCSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FMCSEL {
    type Ux = u8;
}
impl crate::IsEnum for FMCSEL {}
///Field `FMCSEL` reader - FMC kernel clock source selection
pub type FMCSEL_R = crate::FieldReader<FMCSEL>;
impl FMCSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FMCSEL {
        match self.bits {
            0 => FMCSEL::RccHclk5,
            1 => FMCSEL::Pll1Q,
            2 => FMCSEL::Pll2R,
            3 => FMCSEL::HsiKer,
            _ => unreachable!(),
        }
    }
    ///hclk5 selected as peripheral clock
    #[inline(always)]
    pub fn is_rcc_hclk5(&self) -> bool {
        *self == FMCSEL::RccHclk5
    }
    ///pll1_q selected as peripheral clock
    #[inline(always)]
    pub fn is_pll1_q(&self) -> bool {
        *self == FMCSEL::Pll1Q
    }
    ///pll2_r selected as peripheral clock
    #[inline(always)]
    pub fn is_pll2_r(&self) -> bool {
        *self == FMCSEL::Pll2R
    }
    ///hsi_ker selected as peripheral clock
    #[inline(always)]
    pub fn is_hsi_ker(&self) -> bool {
        *self == FMCSEL::HsiKer
    }
}
///Field `FMCSEL` writer - FMC kernel clock source selection
pub type FMCSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, FMCSEL, crate::Safe>;
impl<'a, REG> FMCSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///hclk5 selected as peripheral clock
    #[inline(always)]
    pub fn rcc_hclk5(self) -> &'a mut crate::W<REG> {
        self.variant(FMCSEL::RccHclk5)
    }
    ///pll1_q selected as peripheral clock
    #[inline(always)]
    pub fn pll1_q(self) -> &'a mut crate::W<REG> {
        self.variant(FMCSEL::Pll1Q)
    }
    ///pll2_r selected as peripheral clock
    #[inline(always)]
    pub fn pll2_r(self) -> &'a mut crate::W<REG> {
        self.variant(FMCSEL::Pll2R)
    }
    ///hsi_ker selected as peripheral clock
    #[inline(always)]
    pub fn hsi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(FMCSEL::HsiKer)
    }
}
/**SDMMC1 and SDMMC2 kernel clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDMMC12SEL {
    ///0: pll1_s selected as peripheral clock
    Pll2S = 0,
    ///1: pll2_t selected as peripheral clock
    Pll2T = 1,
}
impl From<SDMMC12SEL> for bool {
    #[inline(always)]
    fn from(variant: SDMMC12SEL) -> Self {
        variant as u8 != 0
    }
}
///Field `SDMMC12SEL` reader - SDMMC1 and SDMMC2 kernel clock source selection
pub type SDMMC12SEL_R = crate::BitReader<SDMMC12SEL>;
impl SDMMC12SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SDMMC12SEL {
        match self.bits {
            false => SDMMC12SEL::Pll2S,
            true => SDMMC12SEL::Pll2T,
        }
    }
    ///pll1_s selected as peripheral clock
    #[inline(always)]
    pub fn is_pll2_s(&self) -> bool {
        *self == SDMMC12SEL::Pll2S
    }
    ///pll2_t selected as peripheral clock
    #[inline(always)]
    pub fn is_pll2_t(&self) -> bool {
        *self == SDMMC12SEL::Pll2T
    }
}
///Field `SDMMC12SEL` writer - SDMMC1 and SDMMC2 kernel clock source selection
pub type SDMMC12SEL_W<'a, REG> = crate::BitWriter<'a, REG, SDMMC12SEL>;
impl<'a, REG> SDMMC12SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///pll1_s selected as peripheral clock
    #[inline(always)]
    pub fn pll2_s(self) -> &'a mut crate::W<REG> {
        self.variant(SDMMC12SEL::Pll2S)
    }
    ///pll2_t selected as peripheral clock
    #[inline(always)]
    pub fn pll2_t(self) -> &'a mut crate::W<REG> {
        self.variant(SDMMC12SEL::Pll2T)
    }
}
/**XSPI1 kernel clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum XSPI1SEL {
    ///0: hclk5 selected as peripheral clock
    RccHclk5 = 0,
    ///1: pll2_s_ck selected as peripheral clock
    Pll2S = 1,
    ///2: pll2_t_ck selected as peripheral clock
    Pll2T = 2,
}
impl From<XSPI1SEL> for u8 {
    #[inline(always)]
    fn from(variant: XSPI1SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for XSPI1SEL {
    type Ux = u8;
}
impl crate::IsEnum for XSPI1SEL {}
///Field `XSPI1SEL` reader - XSPI1 kernel clock source selection
pub type XSPI1SEL_R = crate::FieldReader<XSPI1SEL>;
impl XSPI1SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<XSPI1SEL> {
        match self.bits {
            0 => Some(XSPI1SEL::RccHclk5),
            1 => Some(XSPI1SEL::Pll2S),
            2 => Some(XSPI1SEL::Pll2T),
            _ => None,
        }
    }
    ///hclk5 selected as peripheral clock
    #[inline(always)]
    pub fn is_rcc_hclk5(&self) -> bool {
        *self == XSPI1SEL::RccHclk5
    }
    ///pll2_s_ck selected as peripheral clock
    #[inline(always)]
    pub fn is_pll2_s(&self) -> bool {
        *self == XSPI1SEL::Pll2S
    }
    ///pll2_t_ck selected as peripheral clock
    #[inline(always)]
    pub fn is_pll2_t(&self) -> bool {
        *self == XSPI1SEL::Pll2T
    }
}
///Field `XSPI1SEL` writer - XSPI1 kernel clock source selection
pub type XSPI1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, XSPI1SEL>;
impl<'a, REG> XSPI1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///hclk5 selected as peripheral clock
    #[inline(always)]
    pub fn rcc_hclk5(self) -> &'a mut crate::W<REG> {
        self.variant(XSPI1SEL::RccHclk5)
    }
    ///pll2_s_ck selected as peripheral clock
    #[inline(always)]
    pub fn pll2_s(self) -> &'a mut crate::W<REG> {
        self.variant(XSPI1SEL::Pll2S)
    }
    ///pll2_t_ck selected as peripheral clock
    #[inline(always)]
    pub fn pll2_t(self) -> &'a mut crate::W<REG> {
        self.variant(XSPI1SEL::Pll2T)
    }
}
/**XSPI2 kernel clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum XSPI2SEL {
    ///0: hclk5 selected as peripheral clock
    RccHclk5 = 0,
    ///1: pll2_s_ck selected as peripheral clock
    Pll2S = 1,
    ///2: pll2_t_ck selected as peripheral clock
    Pll2T = 2,
}
impl From<XSPI2SEL> for u8 {
    #[inline(always)]
    fn from(variant: XSPI2SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for XSPI2SEL {
    type Ux = u8;
}
impl crate::IsEnum for XSPI2SEL {}
///Field `XSPI2SEL` reader - XSPI2 kernel clock source selection
pub type XSPI2SEL_R = crate::FieldReader<XSPI2SEL>;
impl XSPI2SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<XSPI2SEL> {
        match self.bits {
            0 => Some(XSPI2SEL::RccHclk5),
            1 => Some(XSPI2SEL::Pll2S),
            2 => Some(XSPI2SEL::Pll2T),
            _ => None,
        }
    }
    ///hclk5 selected as peripheral clock
    #[inline(always)]
    pub fn is_rcc_hclk5(&self) -> bool {
        *self == XSPI2SEL::RccHclk5
    }
    ///pll2_s_ck selected as peripheral clock
    #[inline(always)]
    pub fn is_pll2_s(&self) -> bool {
        *self == XSPI2SEL::Pll2S
    }
    ///pll2_t_ck selected as peripheral clock
    #[inline(always)]
    pub fn is_pll2_t(&self) -> bool {
        *self == XSPI2SEL::Pll2T
    }
}
///Field `XSPI2SEL` writer - XSPI2 kernel clock source selection
pub type XSPI2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, XSPI2SEL>;
impl<'a, REG> XSPI2SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///hclk5 selected as peripheral clock
    #[inline(always)]
    pub fn rcc_hclk5(self) -> &'a mut crate::W<REG> {
        self.variant(XSPI2SEL::RccHclk5)
    }
    ///pll2_s_ck selected as peripheral clock
    #[inline(always)]
    pub fn pll2_s(self) -> &'a mut crate::W<REG> {
        self.variant(XSPI2SEL::Pll2S)
    }
    ///pll2_t_ck selected as peripheral clock
    #[inline(always)]
    pub fn pll2_t(self) -> &'a mut crate::W<REG> {
        self.variant(XSPI2SEL::Pll2T)
    }
}
/**USBPHYC kernel clock frequency selection

Value on reset: 10*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USBREFCKSEL {
    ///3: The clock frequency provided to the USBPHYC is 16 MHz
    Mhz16 = 3,
    ///8: The clock frequency provided to the USBPHYC is 19.2 MHz
    Mhz19 = 8,
    ///9: The clock frequency provided to the USBPHYC is 20MHz
    Mhz20 = 9,
    ///10: The clock frequency provided to the USBPHYC is 24 MHz
    Mhz24 = 10,
    ///11: The clock frequency provided to the USBPHYC is 32 MHz
    Mhz32 = 11,
    ///14: The clock frequency provided to the USBPHYC is 26 MHz
    Mhz26 = 14,
}
impl From<USBREFCKSEL> for u8 {
    #[inline(always)]
    fn from(variant: USBREFCKSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for USBREFCKSEL {
    type Ux = u8;
}
impl crate::IsEnum for USBREFCKSEL {}
///Field `USBREFCKSEL` reader - USBPHYC kernel clock frequency selection
pub type USBREFCKSEL_R = crate::FieldReader<USBREFCKSEL>;
impl USBREFCKSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<USBREFCKSEL> {
        match self.bits {
            3 => Some(USBREFCKSEL::Mhz16),
            8 => Some(USBREFCKSEL::Mhz19),
            9 => Some(USBREFCKSEL::Mhz20),
            10 => Some(USBREFCKSEL::Mhz24),
            11 => Some(USBREFCKSEL::Mhz32),
            14 => Some(USBREFCKSEL::Mhz26),
            _ => None,
        }
    }
    ///The clock frequency provided to the USBPHYC is 16 MHz
    #[inline(always)]
    pub fn is_mhz16(&self) -> bool {
        *self == USBREFCKSEL::Mhz16
    }
    ///The clock frequency provided to the USBPHYC is 19.2 MHz
    #[inline(always)]
    pub fn is_mhz19(&self) -> bool {
        *self == USBREFCKSEL::Mhz19
    }
    ///The clock frequency provided to the USBPHYC is 20MHz
    #[inline(always)]
    pub fn is_mhz20(&self) -> bool {
        *self == USBREFCKSEL::Mhz20
    }
    ///The clock frequency provided to the USBPHYC is 24 MHz
    #[inline(always)]
    pub fn is_mhz24(&self) -> bool {
        *self == USBREFCKSEL::Mhz24
    }
    ///The clock frequency provided to the USBPHYC is 32 MHz
    #[inline(always)]
    pub fn is_mhz32(&self) -> bool {
        *self == USBREFCKSEL::Mhz32
    }
    ///The clock frequency provided to the USBPHYC is 26 MHz
    #[inline(always)]
    pub fn is_mhz26(&self) -> bool {
        *self == USBREFCKSEL::Mhz26
    }
}
///Field `USBREFCKSEL` writer - USBPHYC kernel clock frequency selection
pub type USBREFCKSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4, USBREFCKSEL>;
impl<'a, REG> USBREFCKSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///The clock frequency provided to the USBPHYC is 16 MHz
    #[inline(always)]
    pub fn mhz16(self) -> &'a mut crate::W<REG> {
        self.variant(USBREFCKSEL::Mhz16)
    }
    ///The clock frequency provided to the USBPHYC is 19.2 MHz
    #[inline(always)]
    pub fn mhz19(self) -> &'a mut crate::W<REG> {
        self.variant(USBREFCKSEL::Mhz19)
    }
    ///The clock frequency provided to the USBPHYC is 20MHz
    #[inline(always)]
    pub fn mhz20(self) -> &'a mut crate::W<REG> {
        self.variant(USBREFCKSEL::Mhz20)
    }
    ///The clock frequency provided to the USBPHYC is 24 MHz
    #[inline(always)]
    pub fn mhz24(self) -> &'a mut crate::W<REG> {
        self.variant(USBREFCKSEL::Mhz24)
    }
    ///The clock frequency provided to the USBPHYC is 32 MHz
    #[inline(always)]
    pub fn mhz32(self) -> &'a mut crate::W<REG> {
        self.variant(USBREFCKSEL::Mhz32)
    }
    ///The clock frequency provided to the USBPHYC is 26 MHz
    #[inline(always)]
    pub fn mhz26(self) -> &'a mut crate::W<REG> {
        self.variant(USBREFCKSEL::Mhz26)
    }
}
/**USBPHYC kernel clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USBPHYCSEL {
    ///0: hse_ker_ck
    HseKer = 0,
    ///1: hse_ker_ck / 2
    HseKerDiv2 = 1,
    ///2: pll3_q_ck
    Pll3Q = 2,
}
impl From<USBPHYCSEL> for u8 {
    #[inline(always)]
    fn from(variant: USBPHYCSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for USBPHYCSEL {
    type Ux = u8;
}
impl crate::IsEnum for USBPHYCSEL {}
///Field `USBPHYCSEL` reader - USBPHYC kernel clock source selection
pub type USBPHYCSEL_R = crate::FieldReader<USBPHYCSEL>;
impl USBPHYCSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<USBPHYCSEL> {
        match self.bits {
            0 => Some(USBPHYCSEL::HseKer),
            1 => Some(USBPHYCSEL::HseKerDiv2),
            2 => Some(USBPHYCSEL::Pll3Q),
            _ => None,
        }
    }
    ///hse_ker_ck
    #[inline(always)]
    pub fn is_hse_ker(&self) -> bool {
        *self == USBPHYCSEL::HseKer
    }
    ///hse_ker_ck / 2
    #[inline(always)]
    pub fn is_hse_ker_div2(&self) -> bool {
        *self == USBPHYCSEL::HseKerDiv2
    }
    ///pll3_q_ck
    #[inline(always)]
    pub fn is_pll3_q(&self) -> bool {
        *self == USBPHYCSEL::Pll3Q
    }
}
///Field `USBPHYCSEL` writer - USBPHYC kernel clock source selection
pub type USBPHYCSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, USBPHYCSEL>;
impl<'a, REG> USBPHYCSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///hse_ker_ck
    #[inline(always)]
    pub fn hse_ker(self) -> &'a mut crate::W<REG> {
        self.variant(USBPHYCSEL::HseKer)
    }
    ///hse_ker_ck / 2
    #[inline(always)]
    pub fn hse_ker_div2(self) -> &'a mut crate::W<REG> {
        self.variant(USBPHYCSEL::HseKerDiv2)
    }
    ///pll3_q_ck
    #[inline(always)]
    pub fn pll3_q(self) -> &'a mut crate::W<REG> {
        self.variant(USBPHYCSEL::Pll3Q)
    }
}
/**OTGFS kernel clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OTGFSSEL {
    ///0: hsi48_ker_ck
    Hsi48Ker = 0,
    ///1: pll3_q_ck
    Pll3Q = 1,
    ///2: hse_ker_ck
    HseKer = 2,
    ///3: clk48mohci
    Clk48 = 3,
}
impl From<OTGFSSEL> for u8 {
    #[inline(always)]
    fn from(variant: OTGFSSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OTGFSSEL {
    type Ux = u8;
}
impl crate::IsEnum for OTGFSSEL {}
///Field `OTGFSSEL` reader - OTGFS kernel clock source selection
pub type OTGFSSEL_R = crate::FieldReader<OTGFSSEL>;
impl OTGFSSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OTGFSSEL {
        match self.bits {
            0 => OTGFSSEL::Hsi48Ker,
            1 => OTGFSSEL::Pll3Q,
            2 => OTGFSSEL::HseKer,
            3 => OTGFSSEL::Clk48,
            _ => unreachable!(),
        }
    }
    ///hsi48_ker_ck
    #[inline(always)]
    pub fn is_hsi48_ker(&self) -> bool {
        *self == OTGFSSEL::Hsi48Ker
    }
    ///pll3_q_ck
    #[inline(always)]
    pub fn is_pll3_q(&self) -> bool {
        *self == OTGFSSEL::Pll3Q
    }
    ///hse_ker_ck
    #[inline(always)]
    pub fn is_hse_ker(&self) -> bool {
        *self == OTGFSSEL::HseKer
    }
    ///clk48mohci
    #[inline(always)]
    pub fn is_clk48(&self) -> bool {
        *self == OTGFSSEL::Clk48
    }
}
///Field `OTGFSSEL` writer - OTGFS kernel clock source selection
pub type OTGFSSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, OTGFSSEL, crate::Safe>;
impl<'a, REG> OTGFSSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///hsi48_ker_ck
    #[inline(always)]
    pub fn hsi48_ker(self) -> &'a mut crate::W<REG> {
        self.variant(OTGFSSEL::Hsi48Ker)
    }
    ///pll3_q_ck
    #[inline(always)]
    pub fn pll3_q(self) -> &'a mut crate::W<REG> {
        self.variant(OTGFSSEL::Pll3Q)
    }
    ///hse_ker_ck
    #[inline(always)]
    pub fn hse_ker(self) -> &'a mut crate::W<REG> {
        self.variant(OTGFSSEL::HseKer)
    }
    ///clk48mohci
    #[inline(always)]
    pub fn clk48(self) -> &'a mut crate::W<REG> {
        self.variant(OTGFSSEL::Clk48)
    }
}
/**Ethernet reference clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ETH1REFCKSEL {
    ///0: ETH_RMII_REF selected as peripheral clock
    EthRmiiRef = 0,
    ///1: hse_ker selected as peripheral clock
    HseKer = 1,
    ///2: eth_clk_fb selected as peripheral clock
    EthClkFb = 2,
}
impl From<ETH1REFCKSEL> for u8 {
    #[inline(always)]
    fn from(variant: ETH1REFCKSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ETH1REFCKSEL {
    type Ux = u8;
}
impl crate::IsEnum for ETH1REFCKSEL {}
///Field `ETH1REFCKSEL` reader - Ethernet reference clock source selection
pub type ETH1REFCKSEL_R = crate::FieldReader<ETH1REFCKSEL>;
impl ETH1REFCKSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<ETH1REFCKSEL> {
        match self.bits {
            0 => Some(ETH1REFCKSEL::EthRmiiRef),
            1 => Some(ETH1REFCKSEL::HseKer),
            2 => Some(ETH1REFCKSEL::EthClkFb),
            _ => None,
        }
    }
    ///ETH_RMII_REF selected as peripheral clock
    #[inline(always)]
    pub fn is_eth_rmii_ref(&self) -> bool {
        *self == ETH1REFCKSEL::EthRmiiRef
    }
    ///hse_ker selected as peripheral clock
    #[inline(always)]
    pub fn is_hse_ker(&self) -> bool {
        *self == ETH1REFCKSEL::HseKer
    }
    ///eth_clk_fb selected as peripheral clock
    #[inline(always)]
    pub fn is_eth_clk_fb(&self) -> bool {
        *self == ETH1REFCKSEL::EthClkFb
    }
}
///Field `ETH1REFCKSEL` writer - Ethernet reference clock source selection
pub type ETH1REFCKSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, ETH1REFCKSEL>;
impl<'a, REG> ETH1REFCKSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///ETH_RMII_REF selected as peripheral clock
    #[inline(always)]
    pub fn eth_rmii_ref(self) -> &'a mut crate::W<REG> {
        self.variant(ETH1REFCKSEL::EthRmiiRef)
    }
    ///hse_ker selected as peripheral clock
    #[inline(always)]
    pub fn hse_ker(self) -> &'a mut crate::W<REG> {
        self.variant(ETH1REFCKSEL::HseKer)
    }
    ///eth_clk_fb selected as peripheral clock
    #[inline(always)]
    pub fn eth_clk_fb(self) -> &'a mut crate::W<REG> {
        self.variant(ETH1REFCKSEL::EthClkFb)
    }
}
/**Clock source selection for external Ethernet PHY

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ETH1PHYCKSEL {
    ///0: hse_ker selected as clock source
    HseKer = 0,
    ///1: pll3_s selected clock source
    Pll3S = 1,
}
impl From<ETH1PHYCKSEL> for bool {
    #[inline(always)]
    fn from(variant: ETH1PHYCKSEL) -> Self {
        variant as u8 != 0
    }
}
///Field `ETH1PHYCKSEL` reader - Clock source selection for external Ethernet PHY
pub type ETH1PHYCKSEL_R = crate::BitReader<ETH1PHYCKSEL>;
impl ETH1PHYCKSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ETH1PHYCKSEL {
        match self.bits {
            false => ETH1PHYCKSEL::HseKer,
            true => ETH1PHYCKSEL::Pll3S,
        }
    }
    ///hse_ker selected as clock source
    #[inline(always)]
    pub fn is_hse_ker(&self) -> bool {
        *self == ETH1PHYCKSEL::HseKer
    }
    ///pll3_s selected clock source
    #[inline(always)]
    pub fn is_pll3_s(&self) -> bool {
        *self == ETH1PHYCKSEL::Pll3S
    }
}
///Field `ETH1PHYCKSEL` writer - Clock source selection for external Ethernet PHY
pub type ETH1PHYCKSEL_W<'a, REG> = crate::BitWriter<'a, REG, ETH1PHYCKSEL>;
impl<'a, REG> ETH1PHYCKSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///hse_ker selected as clock source
    #[inline(always)]
    pub fn hse_ker(self) -> &'a mut crate::W<REG> {
        self.variant(ETH1PHYCKSEL::HseKer)
    }
    ///pll3_s selected clock source
    #[inline(always)]
    pub fn pll3_s(self) -> &'a mut crate::W<REG> {
        self.variant(ETH1PHYCKSEL::Pll3S)
    }
}
/**ADF kernel clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADF1SEL {
    ///0: hclk1 selected as ADF clock
    Hclk1 = 0,
    ///1: pll2_p_ck selected as ADF clock
    Pll2P = 1,
    ///2: pll3_p_ck selected as ADF clock
    Pll3P = 2,
    ///3: I2S_CKIN selected as ADF clock
    I2sClkin = 3,
    ///4: csi_ker_ck selected as ADF clock
    CsiKer = 4,
    ///5: hsi_ker_ck selected as ADF clock
    HsiKer = 5,
}
impl From<ADF1SEL> for u8 {
    #[inline(always)]
    fn from(variant: ADF1SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ADF1SEL {
    type Ux = u8;
}
impl crate::IsEnum for ADF1SEL {}
///Field `ADF1SEL` reader - ADF kernel clock source selection
pub type ADF1SEL_R = crate::FieldReader<ADF1SEL>;
impl ADF1SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<ADF1SEL> {
        match self.bits {
            0 => Some(ADF1SEL::Hclk1),
            1 => Some(ADF1SEL::Pll2P),
            2 => Some(ADF1SEL::Pll3P),
            3 => Some(ADF1SEL::I2sClkin),
            4 => Some(ADF1SEL::CsiKer),
            5 => Some(ADF1SEL::HsiKer),
            _ => None,
        }
    }
    ///hclk1 selected as ADF clock
    #[inline(always)]
    pub fn is_hclk1(&self) -> bool {
        *self == ADF1SEL::Hclk1
    }
    ///pll2_p_ck selected as ADF clock
    #[inline(always)]
    pub fn is_pll2_p(&self) -> bool {
        *self == ADF1SEL::Pll2P
    }
    ///pll3_p_ck selected as ADF clock
    #[inline(always)]
    pub fn is_pll3_p(&self) -> bool {
        *self == ADF1SEL::Pll3P
    }
    ///I2S_CKIN selected as ADF clock
    #[inline(always)]
    pub fn is_i2s_clkin(&self) -> bool {
        *self == ADF1SEL::I2sClkin
    }
    ///csi_ker_ck selected as ADF clock
    #[inline(always)]
    pub fn is_csi_ker(&self) -> bool {
        *self == ADF1SEL::CsiKer
    }
    ///hsi_ker_ck selected as ADF clock
    #[inline(always)]
    pub fn is_hsi_ker(&self) -> bool {
        *self == ADF1SEL::HsiKer
    }
}
///Field `ADF1SEL` writer - ADF kernel clock source selection
pub type ADF1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, ADF1SEL>;
impl<'a, REG> ADF1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///hclk1 selected as ADF clock
    #[inline(always)]
    pub fn hclk1(self) -> &'a mut crate::W<REG> {
        self.variant(ADF1SEL::Hclk1)
    }
    ///pll2_p_ck selected as ADF clock
    #[inline(always)]
    pub fn pll2_p(self) -> &'a mut crate::W<REG> {
        self.variant(ADF1SEL::Pll2P)
    }
    ///pll3_p_ck selected as ADF clock
    #[inline(always)]
    pub fn pll3_p(self) -> &'a mut crate::W<REG> {
        self.variant(ADF1SEL::Pll3P)
    }
    ///I2S_CKIN selected as ADF clock
    #[inline(always)]
    pub fn i2s_clkin(self) -> &'a mut crate::W<REG> {
        self.variant(ADF1SEL::I2sClkin)
    }
    ///csi_ker_ck selected as ADF clock
    #[inline(always)]
    pub fn csi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(ADF1SEL::CsiKer)
    }
    ///hsi_ker_ck selected as ADF clock
    #[inline(always)]
    pub fn hsi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(ADF1SEL::HsiKer)
    }
}
/**SAR ADC kernel clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADCSEL {
    ///0: pll2_p selected as peripheral clock
    Pll2P = 0,
    ///1: pll3_r selected as peripheral clock
    Pll3R = 1,
    ///2: per selected as peripheral clock
    Per = 2,
}
impl From<ADCSEL> for u8 {
    #[inline(always)]
    fn from(variant: ADCSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ADCSEL {
    type Ux = u8;
}
impl crate::IsEnum for ADCSEL {}
///Field `ADCSEL` reader - SAR ADC kernel clock source selection
pub type ADCSEL_R = crate::FieldReader<ADCSEL>;
impl ADCSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<ADCSEL> {
        match self.bits {
            0 => Some(ADCSEL::Pll2P),
            1 => Some(ADCSEL::Pll3R),
            2 => Some(ADCSEL::Per),
            _ => None,
        }
    }
    ///pll2_p selected as peripheral clock
    #[inline(always)]
    pub fn is_pll2_p(&self) -> bool {
        *self == ADCSEL::Pll2P
    }
    ///pll3_r selected as peripheral clock
    #[inline(always)]
    pub fn is_pll3_r(&self) -> bool {
        *self == ADCSEL::Pll3R
    }
    ///per selected as peripheral clock
    #[inline(always)]
    pub fn is_per(&self) -> bool {
        *self == ADCSEL::Per
    }
}
///Field `ADCSEL` writer - SAR ADC kernel clock source selection
pub type ADCSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, ADCSEL>;
impl<'a, REG> ADCSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///pll2_p selected as peripheral clock
    #[inline(always)]
    pub fn pll2_p(self) -> &'a mut crate::W<REG> {
        self.variant(ADCSEL::Pll2P)
    }
    ///pll3_r selected as peripheral clock
    #[inline(always)]
    pub fn pll3_r(self) -> &'a mut crate::W<REG> {
        self.variant(ADCSEL::Pll3R)
    }
    ///per selected as peripheral clock
    #[inline(always)]
    pub fn per(self) -> &'a mut crate::W<REG> {
        self.variant(ADCSEL::Per)
    }
}
/**PSSI kernel clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSSISEL {
    ///0: pll3_r selected as peripheral clock
    Pll3R = 0,
    ///1: per selected as kernel peripheral clock
    Per = 1,
}
impl From<PSSISEL> for bool {
    #[inline(always)]
    fn from(variant: PSSISEL) -> Self {
        variant as u8 != 0
    }
}
///Field `PSSISEL` reader - PSSI kernel clock source selection
pub type PSSISEL_R = crate::BitReader<PSSISEL>;
impl PSSISEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PSSISEL {
        match self.bits {
            false => PSSISEL::Pll3R,
            true => PSSISEL::Per,
        }
    }
    ///pll3_r selected as peripheral clock
    #[inline(always)]
    pub fn is_pll3_r(&self) -> bool {
        *self == PSSISEL::Pll3R
    }
    ///per selected as kernel peripheral clock
    #[inline(always)]
    pub fn is_per(&self) -> bool {
        *self == PSSISEL::Per
    }
}
///Field `PSSISEL` writer - PSSI kernel clock source selection
pub type PSSISEL_W<'a, REG> = crate::BitWriter<'a, REG, PSSISEL>;
impl<'a, REG> PSSISEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///pll3_r selected as peripheral clock
    #[inline(always)]
    pub fn pll3_r(self) -> &'a mut crate::W<REG> {
        self.variant(PSSISEL::Pll3R)
    }
    ///per selected as kernel peripheral clock
    #[inline(always)]
    pub fn per(self) -> &'a mut crate::W<REG> {
        self.variant(PSSISEL::Per)
    }
}
/**per_ck clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CKPERSEL {
    ///0: HSI selected as peripheral clock
    Hsi = 0,
    ///1: CSI selected as peripheral clock
    Csi = 1,
    ///2: HSE selected as peripheral clock
    Hse = 2,
}
impl From<CKPERSEL> for u8 {
    #[inline(always)]
    fn from(variant: CKPERSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CKPERSEL {
    type Ux = u8;
}
impl crate::IsEnum for CKPERSEL {}
///Field `CKPERSEL` reader - per_ck clock source selection
pub type CKPERSEL_R = crate::FieldReader<CKPERSEL>;
impl CKPERSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CKPERSEL> {
        match self.bits {
            0 => Some(CKPERSEL::Hsi),
            1 => Some(CKPERSEL::Csi),
            2 => Some(CKPERSEL::Hse),
            _ => None,
        }
    }
    ///HSI selected as peripheral clock
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == CKPERSEL::Hsi
    }
    ///CSI selected as peripheral clock
    #[inline(always)]
    pub fn is_csi(&self) -> bool {
        *self == CKPERSEL::Csi
    }
    ///HSE selected as peripheral clock
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == CKPERSEL::Hse
    }
}
///Field `CKPERSEL` writer - per_ck clock source selection
pub type CKPERSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CKPERSEL>;
impl<'a, REG> CKPERSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///HSI selected as peripheral clock
    #[inline(always)]
    pub fn hsi(self) -> &'a mut crate::W<REG> {
        self.variant(CKPERSEL::Hsi)
    }
    ///CSI selected as peripheral clock
    #[inline(always)]
    pub fn csi(self) -> &'a mut crate::W<REG> {
        self.variant(CKPERSEL::Csi)
    }
    ///HSE selected as peripheral clock
    #[inline(always)]
    pub fn hse(self) -> &'a mut crate::W<REG> {
        self.variant(CKPERSEL::Hse)
    }
}
impl R {
    ///Bits 0:1 - FMC kernel clock source selection
    #[inline(always)]
    pub fn fmcsel(&self) -> FMCSEL_R {
        FMCSEL_R::new((self.bits & 3) as u8)
    }
    ///Bit 2 - SDMMC1 and SDMMC2 kernel clock source selection
    #[inline(always)]
    pub fn sdmmc12sel(&self) -> SDMMC12SEL_R {
        SDMMC12SEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 4:5 - XSPI1 kernel clock source selection
    #[inline(always)]
    pub fn xspi1sel(&self) -> XSPI1SEL_R {
        XSPI1SEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 6:7 - XSPI2 kernel clock source selection
    #[inline(always)]
    pub fn xspi2sel(&self) -> XSPI2SEL_R {
        XSPI2SEL_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:11 - USBPHYC kernel clock frequency selection
    #[inline(always)]
    pub fn usbrefcksel(&self) -> USBREFCKSEL_R {
        USBREFCKSEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:13 - USBPHYC kernel clock source selection
    #[inline(always)]
    pub fn usbphycsel(&self) -> USBPHYCSEL_R {
        USBPHYCSEL_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 14:15 - OTGFS kernel clock source selection
    #[inline(always)]
    pub fn otgfssel(&self) -> OTGFSSEL_R {
        OTGFSSEL_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bits 16:17 - Ethernet reference clock source selection
    #[inline(always)]
    pub fn eth1refcksel(&self) -> ETH1REFCKSEL_R {
        ETH1REFCKSEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bit 18 - Clock source selection for external Ethernet PHY
    #[inline(always)]
    pub fn eth1phycksel(&self) -> ETH1PHYCKSEL_R {
        ETH1PHYCKSEL_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bits 20:22 - ADF kernel clock source selection
    #[inline(always)]
    pub fn adf1sel(&self) -> ADF1SEL_R {
        ADF1SEL_R::new(((self.bits >> 20) & 7) as u8)
    }
    ///Bits 24:25 - SAR ADC kernel clock source selection
    #[inline(always)]
    pub fn adcsel(&self) -> ADCSEL_R {
        ADCSEL_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bit 27 - PSSI kernel clock source selection
    #[inline(always)]
    pub fn pssisel(&self) -> PSSISEL_R {
        PSSISEL_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bits 28:29 - per_ck clock source selection
    #[inline(always)]
    pub fn ckpersel(&self) -> CKPERSEL_R {
        CKPERSEL_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCIPR1")
            .field("fmcsel", &self.fmcsel())
            .field("sdmmc12sel", &self.sdmmc12sel())
            .field("xspi1sel", &self.xspi1sel())
            .field("xspi2sel", &self.xspi2sel())
            .field("usbrefcksel", &self.usbrefcksel())
            .field("usbphycsel", &self.usbphycsel())
            .field("otgfssel", &self.otgfssel())
            .field("eth1refcksel", &self.eth1refcksel())
            .field("eth1phycksel", &self.eth1phycksel())
            .field("adf1sel", &self.adf1sel())
            .field("adcsel", &self.adcsel())
            .field("pssisel", &self.pssisel())
            .field("ckpersel", &self.ckpersel())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - FMC kernel clock source selection
    #[inline(always)]
    pub fn fmcsel(&mut self) -> FMCSEL_W<'_, CCIPR1rs> {
        FMCSEL_W::new(self, 0)
    }
    ///Bit 2 - SDMMC1 and SDMMC2 kernel clock source selection
    #[inline(always)]
    pub fn sdmmc12sel(&mut self) -> SDMMC12SEL_W<'_, CCIPR1rs> {
        SDMMC12SEL_W::new(self, 2)
    }
    ///Bits 4:5 - XSPI1 kernel clock source selection
    #[inline(always)]
    pub fn xspi1sel(&mut self) -> XSPI1SEL_W<'_, CCIPR1rs> {
        XSPI1SEL_W::new(self, 4)
    }
    ///Bits 6:7 - XSPI2 kernel clock source selection
    #[inline(always)]
    pub fn xspi2sel(&mut self) -> XSPI2SEL_W<'_, CCIPR1rs> {
        XSPI2SEL_W::new(self, 6)
    }
    ///Bits 8:11 - USBPHYC kernel clock frequency selection
    #[inline(always)]
    pub fn usbrefcksel(&mut self) -> USBREFCKSEL_W<'_, CCIPR1rs> {
        USBREFCKSEL_W::new(self, 8)
    }
    ///Bits 12:13 - USBPHYC kernel clock source selection
    #[inline(always)]
    pub fn usbphycsel(&mut self) -> USBPHYCSEL_W<'_, CCIPR1rs> {
        USBPHYCSEL_W::new(self, 12)
    }
    ///Bits 14:15 - OTGFS kernel clock source selection
    #[inline(always)]
    pub fn otgfssel(&mut self) -> OTGFSSEL_W<'_, CCIPR1rs> {
        OTGFSSEL_W::new(self, 14)
    }
    ///Bits 16:17 - Ethernet reference clock source selection
    #[inline(always)]
    pub fn eth1refcksel(&mut self) -> ETH1REFCKSEL_W<'_, CCIPR1rs> {
        ETH1REFCKSEL_W::new(self, 16)
    }
    ///Bit 18 - Clock source selection for external Ethernet PHY
    #[inline(always)]
    pub fn eth1phycksel(&mut self) -> ETH1PHYCKSEL_W<'_, CCIPR1rs> {
        ETH1PHYCKSEL_W::new(self, 18)
    }
    ///Bits 20:22 - ADF kernel clock source selection
    #[inline(always)]
    pub fn adf1sel(&mut self) -> ADF1SEL_W<'_, CCIPR1rs> {
        ADF1SEL_W::new(self, 20)
    }
    ///Bits 24:25 - SAR ADC kernel clock source selection
    #[inline(always)]
    pub fn adcsel(&mut self) -> ADCSEL_W<'_, CCIPR1rs> {
        ADCSEL_W::new(self, 24)
    }
    ///Bit 27 - PSSI kernel clock source selection
    #[inline(always)]
    pub fn pssisel(&mut self) -> PSSISEL_W<'_, CCIPR1rs> {
        PSSISEL_W::new(self, 27)
    }
    ///Bits 28:29 - per_ck clock source selection
    #[inline(always)]
    pub fn ckpersel(&mut self) -> CKPERSEL_W<'_, CCIPR1rs> {
        CKPERSEL_W::new(self, 28)
    }
}
/**RCC AHB peripheral kernel clock selection register

You can [`read`](crate::Reg::read) this register and get [`ccipr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:CCIPR1)*/
pub struct CCIPR1rs;
impl crate::RegisterSpec for CCIPR1rs {
    type Ux = u32;
}
///`read()` method returns [`ccipr1::R`](R) reader structure
impl crate::Readable for CCIPR1rs {}
///`write(|w| ..)` method takes [`ccipr1::W`](W) writer structure
impl crate::Writable for CCIPR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCIPR1 to value 0x0a00
impl crate::Resettable for CCIPR1rs {
    const RESET_VALUE: u32 = 0x0a00;
}
