///Register `CCIPR1` reader
pub type R = crate::R<CCIPR1rs>;
///Register `CCIPR1` writer
pub type W = crate::W<CCIPR1rs>;
/**FMC kernel clock source selection Set and reset by software.

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
///Field `FMCSEL` reader - FMC kernel clock source selection Set and reset by software.
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
///Field `FMCSEL` writer - FMC kernel clock source selection Set and reset by software.
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
/**SDMMC1 and SDMMC2 kernel clock source selection Set and reset by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDMMCSEL {
    ///0: pll1_s selected as peripheral clock
    Pll2S = 0,
    ///1: pll2_t selected as peripheral clock
    Pll2T = 1,
}
impl From<SDMMCSEL> for bool {
    #[inline(always)]
    fn from(variant: SDMMCSEL) -> Self {
        variant as u8 != 0
    }
}
///Field `SDMMCSEL` reader - SDMMC1 and SDMMC2 kernel clock source selection Set and reset by software.
pub type SDMMCSEL_R = crate::BitReader<SDMMCSEL>;
impl SDMMCSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SDMMCSEL {
        match self.bits {
            false => SDMMCSEL::Pll2S,
            true => SDMMCSEL::Pll2T,
        }
    }
    ///pll1_s selected as peripheral clock
    #[inline(always)]
    pub fn is_pll2_s(&self) -> bool {
        *self == SDMMCSEL::Pll2S
    }
    ///pll2_t selected as peripheral clock
    #[inline(always)]
    pub fn is_pll2_t(&self) -> bool {
        *self == SDMMCSEL::Pll2T
    }
}
///Field `SDMMCSEL` writer - SDMMC1 and SDMMC2 kernel clock source selection Set and reset by software.
pub type SDMMCSEL_W<'a, REG> = crate::BitWriter<'a, REG, SDMMCSEL>;
impl<'a, REG> SDMMCSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///pll1_s selected as peripheral clock
    #[inline(always)]
    pub fn pll2_s(self) -> &'a mut crate::W<REG> {
        self.variant(SDMMCSEL::Pll2S)
    }
    ///pll2_t selected as peripheral clock
    #[inline(always)]
    pub fn pll2_t(self) -> &'a mut crate::W<REG> {
        self.variant(SDMMCSEL::Pll2T)
    }
}
/**XSPI1 kernel clock source selection Set and reset by software. 1x: pll2_t_ck selected as kernel peripheral clock

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OCTOSPI1SEL {
    ///0: hclk5 selected as peripheral clock
    RccHclk5 = 0,
    ///1: pll2_s_ck selected as peripheral clock
    Pll2S = 1,
    ///2: pll2_t_ck selected as peripheral clock
    Pll2T = 2,
}
impl From<OCTOSPI1SEL> for u8 {
    #[inline(always)]
    fn from(variant: OCTOSPI1SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OCTOSPI1SEL {
    type Ux = u8;
}
impl crate::IsEnum for OCTOSPI1SEL {}
///Field `OCTOSPI1SEL` reader - XSPI1 kernel clock source selection Set and reset by software. 1x: pll2_t_ck selected as kernel peripheral clock
pub type OCTOSPI1SEL_R = crate::FieldReader<OCTOSPI1SEL>;
impl OCTOSPI1SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<OCTOSPI1SEL> {
        match self.bits {
            0 => Some(OCTOSPI1SEL::RccHclk5),
            1 => Some(OCTOSPI1SEL::Pll2S),
            2 => Some(OCTOSPI1SEL::Pll2T),
            _ => None,
        }
    }
    ///hclk5 selected as peripheral clock
    #[inline(always)]
    pub fn is_rcc_hclk5(&self) -> bool {
        *self == OCTOSPI1SEL::RccHclk5
    }
    ///pll2_s_ck selected as peripheral clock
    #[inline(always)]
    pub fn is_pll2_s(&self) -> bool {
        *self == OCTOSPI1SEL::Pll2S
    }
    ///pll2_t_ck selected as peripheral clock
    #[inline(always)]
    pub fn is_pll2_t(&self) -> bool {
        *self == OCTOSPI1SEL::Pll2T
    }
}
///Field `OCTOSPI1SEL` writer - XSPI1 kernel clock source selection Set and reset by software. 1x: pll2_t_ck selected as kernel peripheral clock
pub type OCTOSPI1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, OCTOSPI1SEL>;
impl<'a, REG> OCTOSPI1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///hclk5 selected as peripheral clock
    #[inline(always)]
    pub fn rcc_hclk5(self) -> &'a mut crate::W<REG> {
        self.variant(OCTOSPI1SEL::RccHclk5)
    }
    ///pll2_s_ck selected as peripheral clock
    #[inline(always)]
    pub fn pll2_s(self) -> &'a mut crate::W<REG> {
        self.variant(OCTOSPI1SEL::Pll2S)
    }
    ///pll2_t_ck selected as peripheral clock
    #[inline(always)]
    pub fn pll2_t(self) -> &'a mut crate::W<REG> {
        self.variant(OCTOSPI1SEL::Pll2T)
    }
}
/**XSPI2 kernel clock source selection Set and reset by software. 1x: pll2_t_ck selected as kernel peripheral clock

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OCTOSPI2SEL {
    ///0: hclk5 selected as peripheral clock
    RccHclk5 = 0,
    ///1: pll2_s_ck selected as peripheral clock
    Pll2S = 1,
    ///2: pll2_t_ck selected as peripheral clock
    Pll2T = 2,
}
impl From<OCTOSPI2SEL> for u8 {
    #[inline(always)]
    fn from(variant: OCTOSPI2SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OCTOSPI2SEL {
    type Ux = u8;
}
impl crate::IsEnum for OCTOSPI2SEL {}
///Field `OCTOSPI2SEL` reader - XSPI2 kernel clock source selection Set and reset by software. 1x: pll2_t_ck selected as kernel peripheral clock
pub type OCTOSPI2SEL_R = crate::FieldReader<OCTOSPI2SEL>;
impl OCTOSPI2SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<OCTOSPI2SEL> {
        match self.bits {
            0 => Some(OCTOSPI2SEL::RccHclk5),
            1 => Some(OCTOSPI2SEL::Pll2S),
            2 => Some(OCTOSPI2SEL::Pll2T),
            _ => None,
        }
    }
    ///hclk5 selected as peripheral clock
    #[inline(always)]
    pub fn is_rcc_hclk5(&self) -> bool {
        *self == OCTOSPI2SEL::RccHclk5
    }
    ///pll2_s_ck selected as peripheral clock
    #[inline(always)]
    pub fn is_pll2_s(&self) -> bool {
        *self == OCTOSPI2SEL::Pll2S
    }
    ///pll2_t_ck selected as peripheral clock
    #[inline(always)]
    pub fn is_pll2_t(&self) -> bool {
        *self == OCTOSPI2SEL::Pll2T
    }
}
///Field `OCTOSPI2SEL` writer - XSPI2 kernel clock source selection Set and reset by software. 1x: pll2_t_ck selected as kernel peripheral clock
pub type OCTOSPI2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, OCTOSPI2SEL>;
impl<'a, REG> OCTOSPI2SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///hclk5 selected as peripheral clock
    #[inline(always)]
    pub fn rcc_hclk5(self) -> &'a mut crate::W<REG> {
        self.variant(OCTOSPI2SEL::RccHclk5)
    }
    ///pll2_s_ck selected as peripheral clock
    #[inline(always)]
    pub fn pll2_s(self) -> &'a mut crate::W<REG> {
        self.variant(OCTOSPI2SEL::Pll2S)
    }
    ///pll2_t_ck selected as peripheral clock
    #[inline(always)]
    pub fn pll2_t(self) -> &'a mut crate::W<REG> {
        self.variant(OCTOSPI2SEL::Pll2T)
    }
}
/**USBPHYC kernel clock frequency selection Set and reset by software. This field is used to indicate to the USBPHYC, the frequency of the reference kernel clock provided to the USBPHYC. others: reserved

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
///Field `USBREFCKSEL` reader - USBPHYC kernel clock frequency selection Set and reset by software. This field is used to indicate to the USBPHYC, the frequency of the reference kernel clock provided to the USBPHYC. others: reserved
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
///Field `USBREFCKSEL` writer - USBPHYC kernel clock frequency selection Set and reset by software. This field is used to indicate to the USBPHYC, the frequency of the reference kernel clock provided to the USBPHYC. others: reserved
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
/**USBPHYC kernel clock source selection Set and reset by software.

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
///Field `USBPHYCSEL` reader - USBPHYC kernel clock source selection Set and reset by software.
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
///Field `USBPHYCSEL` writer - USBPHYC kernel clock source selection Set and reset by software.
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
/**OTGFS kernel clock source selection Set and reset by software.

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
///Field `OTGFSSEL` reader - OTGFS kernel clock source selection Set and reset by software.
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
///Field `OTGFSSEL` writer - OTGFS kernel clock source selection Set and reset by software.
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
/**Ethernet reference clock source selection Set and reset by software. others: reserved, the kernel clock is disabled

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ETH1_REF_CLK_SEL {
    ///0: ETH_RMII_REF selected as peripheral clock
    EthRmiiRef = 0,
    ///1: hse_ker selected as peripheral clock
    HseKer = 1,
    ///2: eth_clk_fb selected as peripheral clock
    EthClkFb = 2,
}
impl From<ETH1_REF_CLK_SEL> for u8 {
    #[inline(always)]
    fn from(variant: ETH1_REF_CLK_SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ETH1_REF_CLK_SEL {
    type Ux = u8;
}
impl crate::IsEnum for ETH1_REF_CLK_SEL {}
///Field `ETH1_REF_CLK_SEL` reader - Ethernet reference clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
pub type ETH1_REF_CLK_SEL_R = crate::FieldReader<ETH1_REF_CLK_SEL>;
impl ETH1_REF_CLK_SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<ETH1_REF_CLK_SEL> {
        match self.bits {
            0 => Some(ETH1_REF_CLK_SEL::EthRmiiRef),
            1 => Some(ETH1_REF_CLK_SEL::HseKer),
            2 => Some(ETH1_REF_CLK_SEL::EthClkFb),
            _ => None,
        }
    }
    ///ETH_RMII_REF selected as peripheral clock
    #[inline(always)]
    pub fn is_eth_rmii_ref(&self) -> bool {
        *self == ETH1_REF_CLK_SEL::EthRmiiRef
    }
    ///hse_ker selected as peripheral clock
    #[inline(always)]
    pub fn is_hse_ker(&self) -> bool {
        *self == ETH1_REF_CLK_SEL::HseKer
    }
    ///eth_clk_fb selected as peripheral clock
    #[inline(always)]
    pub fn is_eth_clk_fb(&self) -> bool {
        *self == ETH1_REF_CLK_SEL::EthClkFb
    }
}
///Field `ETH1_REF_CLK_SEL` writer - Ethernet reference clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
pub type ETH1_REF_CLK_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, ETH1_REF_CLK_SEL>;
impl<'a, REG> ETH1_REF_CLK_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///ETH_RMII_REF selected as peripheral clock
    #[inline(always)]
    pub fn eth_rmii_ref(self) -> &'a mut crate::W<REG> {
        self.variant(ETH1_REF_CLK_SEL::EthRmiiRef)
    }
    ///hse_ker selected as peripheral clock
    #[inline(always)]
    pub fn hse_ker(self) -> &'a mut crate::W<REG> {
        self.variant(ETH1_REF_CLK_SEL::HseKer)
    }
    ///eth_clk_fb selected as peripheral clock
    #[inline(always)]
    pub fn eth_clk_fb(self) -> &'a mut crate::W<REG> {
        self.variant(ETH1_REF_CLK_SEL::EthClkFb)
    }
}
/**Clock source selection for external Ethernet PHY Set and reset by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ETHPHY_CLK_SEL {
    ///0: hse_ker selected as clock source
    HseKer = 0,
    ///1: pll3_s selected clock source
    Pll3S = 1,
}
impl From<ETHPHY_CLK_SEL> for bool {
    #[inline(always)]
    fn from(variant: ETHPHY_CLK_SEL) -> Self {
        variant as u8 != 0
    }
}
///Field `ETHPHY_CLK_SEL` reader - Clock source selection for external Ethernet PHY Set and reset by software.
pub type ETHPHY_CLK_SEL_R = crate::BitReader<ETHPHY_CLK_SEL>;
impl ETHPHY_CLK_SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ETHPHY_CLK_SEL {
        match self.bits {
            false => ETHPHY_CLK_SEL::HseKer,
            true => ETHPHY_CLK_SEL::Pll3S,
        }
    }
    ///hse_ker selected as clock source
    #[inline(always)]
    pub fn is_hse_ker(&self) -> bool {
        *self == ETHPHY_CLK_SEL::HseKer
    }
    ///pll3_s selected clock source
    #[inline(always)]
    pub fn is_pll3_s(&self) -> bool {
        *self == ETHPHY_CLK_SEL::Pll3S
    }
}
///Field `ETHPHY_CLK_SEL` writer - Clock source selection for external Ethernet PHY Set and reset by software.
pub type ETHPHY_CLK_SEL_W<'a, REG> = crate::BitWriter<'a, REG, ETHPHY_CLK_SEL>;
impl<'a, REG> ETHPHY_CLK_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///hse_ker selected as clock source
    #[inline(always)]
    pub fn hse_ker(self) -> &'a mut crate::W<REG> {
        self.variant(ETHPHY_CLK_SEL::HseKer)
    }
    ///pll3_s selected clock source
    #[inline(always)]
    pub fn pll3_s(self) -> &'a mut crate::W<REG> {
        self.variant(ETHPHY_CLK_SEL::Pll3S)
    }
}
/**ADF kernel clock source selection Set and reset by software. Note: I2S_CKIN is an external clock taken from a pin.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADFSEL {
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
impl From<ADFSEL> for u8 {
    #[inline(always)]
    fn from(variant: ADFSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ADFSEL {
    type Ux = u8;
}
impl crate::IsEnum for ADFSEL {}
///Field `ADFSEL` reader - ADF kernel clock source selection Set and reset by software. Note: I2S_CKIN is an external clock taken from a pin.
pub type ADFSEL_R = crate::FieldReader<ADFSEL>;
impl ADFSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<ADFSEL> {
        match self.bits {
            0 => Some(ADFSEL::Hclk1),
            1 => Some(ADFSEL::Pll2P),
            2 => Some(ADFSEL::Pll3P),
            3 => Some(ADFSEL::I2sClkin),
            4 => Some(ADFSEL::CsiKer),
            5 => Some(ADFSEL::HsiKer),
            _ => None,
        }
    }
    ///hclk1 selected as ADF clock
    #[inline(always)]
    pub fn is_hclk1(&self) -> bool {
        *self == ADFSEL::Hclk1
    }
    ///pll2_p_ck selected as ADF clock
    #[inline(always)]
    pub fn is_pll2_p(&self) -> bool {
        *self == ADFSEL::Pll2P
    }
    ///pll3_p_ck selected as ADF clock
    #[inline(always)]
    pub fn is_pll3_p(&self) -> bool {
        *self == ADFSEL::Pll3P
    }
    ///I2S_CKIN selected as ADF clock
    #[inline(always)]
    pub fn is_i2s_clkin(&self) -> bool {
        *self == ADFSEL::I2sClkin
    }
    ///csi_ker_ck selected as ADF clock
    #[inline(always)]
    pub fn is_csi_ker(&self) -> bool {
        *self == ADFSEL::CsiKer
    }
    ///hsi_ker_ck selected as ADF clock
    #[inline(always)]
    pub fn is_hsi_ker(&self) -> bool {
        *self == ADFSEL::HsiKer
    }
}
///Field `ADFSEL` writer - ADF kernel clock source selection Set and reset by software. Note: I2S_CKIN is an external clock taken from a pin.
pub type ADFSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, ADFSEL>;
impl<'a, REG> ADFSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///hclk1 selected as ADF clock
    #[inline(always)]
    pub fn hclk1(self) -> &'a mut crate::W<REG> {
        self.variant(ADFSEL::Hclk1)
    }
    ///pll2_p_ck selected as ADF clock
    #[inline(always)]
    pub fn pll2_p(self) -> &'a mut crate::W<REG> {
        self.variant(ADFSEL::Pll2P)
    }
    ///pll3_p_ck selected as ADF clock
    #[inline(always)]
    pub fn pll3_p(self) -> &'a mut crate::W<REG> {
        self.variant(ADFSEL::Pll3P)
    }
    ///I2S_CKIN selected as ADF clock
    #[inline(always)]
    pub fn i2s_clkin(self) -> &'a mut crate::W<REG> {
        self.variant(ADFSEL::I2sClkin)
    }
    ///csi_ker_ck selected as ADF clock
    #[inline(always)]
    pub fn csi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(ADFSEL::CsiKer)
    }
    ///hsi_ker_ck selected as ADF clock
    #[inline(always)]
    pub fn hsi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(ADFSEL::HsiKer)
    }
}
/**SAR ADC kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled

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
///Field `ADCSEL` reader - SAR ADC kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
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
///Field `ADCSEL` writer - SAR ADC kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
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
/**PSSI kernel clock source selection Set and reset by software.

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
///Field `PSSISEL` reader - PSSI kernel clock source selection Set and reset by software.
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
///Field `PSSISEL` writer - PSSI kernel clock source selection Set and reset by software.
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
    ///Bits 0:1 - FMC kernel clock source selection Set and reset by software.
    #[inline(always)]
    pub fn fmcsel(&self) -> FMCSEL_R {
        FMCSEL_R::new((self.bits & 3) as u8)
    }
    ///Bit 2 - SDMMC1 and SDMMC2 kernel clock source selection Set and reset by software.
    #[inline(always)]
    pub fn sdmmcsel(&self) -> SDMMCSEL_R {
        SDMMCSEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 4:5 - XSPI1 kernel clock source selection Set and reset by software. 1x: pll2_t_ck selected as kernel peripheral clock
    #[inline(always)]
    pub fn octospi1sel(&self) -> OCTOSPI1SEL_R {
        OCTOSPI1SEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 6:7 - XSPI2 kernel clock source selection Set and reset by software. 1x: pll2_t_ck selected as kernel peripheral clock
    #[inline(always)]
    pub fn octospi2sel(&self) -> OCTOSPI2SEL_R {
        OCTOSPI2SEL_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:11 - USBPHYC kernel clock frequency selection Set and reset by software. This field is used to indicate to the USBPHYC, the frequency of the reference kernel clock provided to the USBPHYC. others: reserved
    #[inline(always)]
    pub fn usbrefcksel(&self) -> USBREFCKSEL_R {
        USBREFCKSEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:13 - USBPHYC kernel clock source selection Set and reset by software.
    #[inline(always)]
    pub fn usbphycsel(&self) -> USBPHYCSEL_R {
        USBPHYCSEL_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 14:15 - OTGFS kernel clock source selection Set and reset by software.
    #[inline(always)]
    pub fn otgfssel(&self) -> OTGFSSEL_R {
        OTGFSSEL_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bits 16:17 - Ethernet reference clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
    #[inline(always)]
    pub fn eth1_ref_clk_sel(&self) -> ETH1_REF_CLK_SEL_R {
        ETH1_REF_CLK_SEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bit 18 - Clock source selection for external Ethernet PHY Set and reset by software.
    #[inline(always)]
    pub fn ethphy_clk_sel(&self) -> ETHPHY_CLK_SEL_R {
        ETHPHY_CLK_SEL_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bits 20:22 - ADF kernel clock source selection Set and reset by software. Note: I2S_CKIN is an external clock taken from a pin.
    #[inline(always)]
    pub fn adfsel(&self) -> ADFSEL_R {
        ADFSEL_R::new(((self.bits >> 20) & 7) as u8)
    }
    ///Bits 24:25 - SAR ADC kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
    #[inline(always)]
    pub fn adcsel(&self) -> ADCSEL_R {
        ADCSEL_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bit 27 - PSSI kernel clock source selection Set and reset by software.
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
            .field("sdmmcsel", &self.sdmmcsel())
            .field("octospi1sel", &self.octospi1sel())
            .field("octospi2sel", &self.octospi2sel())
            .field("usbrefcksel", &self.usbrefcksel())
            .field("usbphycsel", &self.usbphycsel())
            .field("otgfssel", &self.otgfssel())
            .field("eth1_ref_clk_sel", &self.eth1_ref_clk_sel())
            .field("ethphy_clk_sel", &self.ethphy_clk_sel())
            .field("adfsel", &self.adfsel())
            .field("adcsel", &self.adcsel())
            .field("pssisel", &self.pssisel())
            .field("ckpersel", &self.ckpersel())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - FMC kernel clock source selection Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn fmcsel(&mut self) -> FMCSEL_W<CCIPR1rs> {
        FMCSEL_W::new(self, 0)
    }
    ///Bit 2 - SDMMC1 and SDMMC2 kernel clock source selection Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn sdmmcsel(&mut self) -> SDMMCSEL_W<CCIPR1rs> {
        SDMMCSEL_W::new(self, 2)
    }
    ///Bits 4:5 - XSPI1 kernel clock source selection Set and reset by software. 1x: pll2_t_ck selected as kernel peripheral clock
    #[inline(always)]
    #[must_use]
    pub fn octospi1sel(&mut self) -> OCTOSPI1SEL_W<CCIPR1rs> {
        OCTOSPI1SEL_W::new(self, 4)
    }
    ///Bits 6:7 - XSPI2 kernel clock source selection Set and reset by software. 1x: pll2_t_ck selected as kernel peripheral clock
    #[inline(always)]
    #[must_use]
    pub fn octospi2sel(&mut self) -> OCTOSPI2SEL_W<CCIPR1rs> {
        OCTOSPI2SEL_W::new(self, 6)
    }
    ///Bits 8:11 - USBPHYC kernel clock frequency selection Set and reset by software. This field is used to indicate to the USBPHYC, the frequency of the reference kernel clock provided to the USBPHYC. others: reserved
    #[inline(always)]
    #[must_use]
    pub fn usbrefcksel(&mut self) -> USBREFCKSEL_W<CCIPR1rs> {
        USBREFCKSEL_W::new(self, 8)
    }
    ///Bits 12:13 - USBPHYC kernel clock source selection Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn usbphycsel(&mut self) -> USBPHYCSEL_W<CCIPR1rs> {
        USBPHYCSEL_W::new(self, 12)
    }
    ///Bits 14:15 - OTGFS kernel clock source selection Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn otgfssel(&mut self) -> OTGFSSEL_W<CCIPR1rs> {
        OTGFSSEL_W::new(self, 14)
    }
    ///Bits 16:17 - Ethernet reference clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
    #[inline(always)]
    #[must_use]
    pub fn eth1_ref_clk_sel(&mut self) -> ETH1_REF_CLK_SEL_W<CCIPR1rs> {
        ETH1_REF_CLK_SEL_W::new(self, 16)
    }
    ///Bit 18 - Clock source selection for external Ethernet PHY Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn ethphy_clk_sel(&mut self) -> ETHPHY_CLK_SEL_W<CCIPR1rs> {
        ETHPHY_CLK_SEL_W::new(self, 18)
    }
    ///Bits 20:22 - ADF kernel clock source selection Set and reset by software. Note: I2S_CKIN is an external clock taken from a pin.
    #[inline(always)]
    #[must_use]
    pub fn adfsel(&mut self) -> ADFSEL_W<CCIPR1rs> {
        ADFSEL_W::new(self, 20)
    }
    ///Bits 24:25 - SAR ADC kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
    #[inline(always)]
    #[must_use]
    pub fn adcsel(&mut self) -> ADCSEL_W<CCIPR1rs> {
        ADCSEL_W::new(self, 24)
    }
    ///Bit 27 - PSSI kernel clock source selection Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn pssisel(&mut self) -> PSSISEL_W<CCIPR1rs> {
        PSSISEL_W::new(self, 27)
    }
    ///Bits 28:29 - per_ck clock source selection
    #[inline(always)]
    #[must_use]
    pub fn ckpersel(&mut self) -> CKPERSEL_W<CCIPR1rs> {
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CCIPR1 to value 0x0a00
impl crate::Resettable for CCIPR1rs {
    const RESET_VALUE: u32 = 0x0a00;
}
