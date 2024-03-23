#[doc = "Register `SRDCCIPR` reader"]
pub type R = crate::R<SRDCCIPRrs>;
#[doc = "Register `SRDCCIPR` writer"]
pub type W = crate::W<SRDCCIPRrs>;
#[doc = "LPUART1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LPUART1SEL {
    #[doc = "0: rcc_pclk_d3 selected as peripheral clock"]
    RccPclkD3 = 0,
    #[doc = "1: pll2_q selected as peripheral clock"]
    Pll2Q = 1,
    #[doc = "2: pll3_q selected as peripheral clock"]
    Pll3Q = 2,
    #[doc = "3: hsi_ker selected as peripheral clock"]
    HsiKer = 3,
    #[doc = "4: csi_ker selected as peripheral clock"]
    CsiKer = 4,
    #[doc = "5: LSE selected as peripheral clock"]
    Lse = 5,
}
impl From<LPUART1SEL> for u8 {
    #[inline(always)]
    fn from(variant: LPUART1SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LPUART1SEL {
    type Ux = u8;
}
#[doc = "Field `LPUART1SEL` reader - LPUART1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
pub type LPUART1SEL_R = crate::FieldReader<LPUART1SEL>;
impl LPUART1SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LPUART1SEL> {
        match self.bits {
            0 => Some(LPUART1SEL::RccPclkD3),
            1 => Some(LPUART1SEL::Pll2Q),
            2 => Some(LPUART1SEL::Pll3Q),
            3 => Some(LPUART1SEL::HsiKer),
            4 => Some(LPUART1SEL::CsiKer),
            5 => Some(LPUART1SEL::Lse),
            _ => None,
        }
    }
    #[doc = "rcc_pclk_d3 selected as peripheral clock"]
    #[inline(always)]
    pub fn is_rcc_pclk_d3(&self) -> bool {
        *self == LPUART1SEL::RccPclkD3
    }
    #[doc = "pll2_q selected as peripheral clock"]
    #[inline(always)]
    pub fn is_pll2_q(&self) -> bool {
        *self == LPUART1SEL::Pll2Q
    }
    #[doc = "pll3_q selected as peripheral clock"]
    #[inline(always)]
    pub fn is_pll3_q(&self) -> bool {
        *self == LPUART1SEL::Pll3Q
    }
    #[doc = "hsi_ker selected as peripheral clock"]
    #[inline(always)]
    pub fn is_hsi_ker(&self) -> bool {
        *self == LPUART1SEL::HsiKer
    }
    #[doc = "csi_ker selected as peripheral clock"]
    #[inline(always)]
    pub fn is_csi_ker(&self) -> bool {
        *self == LPUART1SEL::CsiKer
    }
    #[doc = "LSE selected as peripheral clock"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == LPUART1SEL::Lse
    }
}
#[doc = "Field `LPUART1SEL` writer - LPUART1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
pub type LPUART1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, LPUART1SEL>;
impl<'a, REG> LPUART1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "rcc_pclk_d3 selected as peripheral clock"]
    #[inline(always)]
    pub fn rcc_pclk_d3(self) -> &'a mut crate::W<REG> {
        self.variant(LPUART1SEL::RccPclkD3)
    }
    #[doc = "pll2_q selected as peripheral clock"]
    #[inline(always)]
    pub fn pll2_q(self) -> &'a mut crate::W<REG> {
        self.variant(LPUART1SEL::Pll2Q)
    }
    #[doc = "pll3_q selected as peripheral clock"]
    #[inline(always)]
    pub fn pll3_q(self) -> &'a mut crate::W<REG> {
        self.variant(LPUART1SEL::Pll3Q)
    }
    #[doc = "hsi_ker selected as peripheral clock"]
    #[inline(always)]
    pub fn hsi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(LPUART1SEL::HsiKer)
    }
    #[doc = "csi_ker selected as peripheral clock"]
    #[inline(always)]
    pub fn csi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(LPUART1SEL::CsiKer)
    }
    #[doc = "LSE selected as peripheral clock"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(LPUART1SEL::Lse)
    }
}
#[doc = "I2C4 kernel clock source selection Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2C4SEL {
    #[doc = "0: rcc_pclk4 selected as peripheral clock"]
    RccPclk4 = 0,
    #[doc = "1: pll3_r selected as peripheral clock"]
    Pll3R = 1,
    #[doc = "2: hsi_ker selected as peripheral clock"]
    HsiKer = 2,
    #[doc = "3: csi_ker selected as peripheral clock"]
    CsiKer = 3,
}
impl From<I2C4SEL> for u8 {
    #[inline(always)]
    fn from(variant: I2C4SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for I2C4SEL {
    type Ux = u8;
}
#[doc = "Field `I2C4SEL` reader - I2C4 kernel clock source selection Set and reset by software."]
pub type I2C4SEL_R = crate::FieldReader<I2C4SEL>;
impl I2C4SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2C4SEL {
        match self.bits {
            0 => I2C4SEL::RccPclk4,
            1 => I2C4SEL::Pll3R,
            2 => I2C4SEL::HsiKer,
            3 => I2C4SEL::CsiKer,
            _ => unreachable!(),
        }
    }
    #[doc = "rcc_pclk4 selected as peripheral clock"]
    #[inline(always)]
    pub fn is_rcc_pclk4(&self) -> bool {
        *self == I2C4SEL::RccPclk4
    }
    #[doc = "pll3_r selected as peripheral clock"]
    #[inline(always)]
    pub fn is_pll3_r(&self) -> bool {
        *self == I2C4SEL::Pll3R
    }
    #[doc = "hsi_ker selected as peripheral clock"]
    #[inline(always)]
    pub fn is_hsi_ker(&self) -> bool {
        *self == I2C4SEL::HsiKer
    }
    #[doc = "csi_ker selected as peripheral clock"]
    #[inline(always)]
    pub fn is_csi_ker(&self) -> bool {
        *self == I2C4SEL::CsiKer
    }
}
#[doc = "Field `I2C4SEL` writer - I2C4 kernel clock source selection Set and reset by software."]
pub type I2C4SEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, I2C4SEL>;
impl<'a, REG> I2C4SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "rcc_pclk4 selected as peripheral clock"]
    #[inline(always)]
    pub fn rcc_pclk4(self) -> &'a mut crate::W<REG> {
        self.variant(I2C4SEL::RccPclk4)
    }
    #[doc = "pll3_r selected as peripheral clock"]
    #[inline(always)]
    pub fn pll3_r(self) -> &'a mut crate::W<REG> {
        self.variant(I2C4SEL::Pll3R)
    }
    #[doc = "hsi_ker selected as peripheral clock"]
    #[inline(always)]
    pub fn hsi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(I2C4SEL::HsiKer)
    }
    #[doc = "csi_ker selected as peripheral clock"]
    #[inline(always)]
    pub fn csi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(I2C4SEL::CsiKer)
    }
}
#[doc = "LPTIM2 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LPTIM2SEL {
    #[doc = "0: rcc_pclk4 selected as peripheral clock"]
    RccPclk4 = 0,
    #[doc = "1: pll2_p selected as peripheral clock"]
    Pll2P = 1,
    #[doc = "2: pll3_r selected as peripheral clock"]
    Pll3R = 2,
    #[doc = "3: LSE selected as peripheral clock"]
    Lse = 3,
    #[doc = "4: LSI selected as peripheral clock"]
    Lsi = 4,
    #[doc = "5: PER selected as peripheral clock"]
    Per = 5,
}
impl From<LPTIM2SEL> for u8 {
    #[inline(always)]
    fn from(variant: LPTIM2SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LPTIM2SEL {
    type Ux = u8;
}
#[doc = "Field `LPTIM2SEL` reader - LPTIM2 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
pub type LPTIM2SEL_R = crate::FieldReader<LPTIM2SEL>;
impl LPTIM2SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LPTIM2SEL> {
        match self.bits {
            0 => Some(LPTIM2SEL::RccPclk4),
            1 => Some(LPTIM2SEL::Pll2P),
            2 => Some(LPTIM2SEL::Pll3R),
            3 => Some(LPTIM2SEL::Lse),
            4 => Some(LPTIM2SEL::Lsi),
            5 => Some(LPTIM2SEL::Per),
            _ => None,
        }
    }
    #[doc = "rcc_pclk4 selected as peripheral clock"]
    #[inline(always)]
    pub fn is_rcc_pclk4(&self) -> bool {
        *self == LPTIM2SEL::RccPclk4
    }
    #[doc = "pll2_p selected as peripheral clock"]
    #[inline(always)]
    pub fn is_pll2_p(&self) -> bool {
        *self == LPTIM2SEL::Pll2P
    }
    #[doc = "pll3_r selected as peripheral clock"]
    #[inline(always)]
    pub fn is_pll3_r(&self) -> bool {
        *self == LPTIM2SEL::Pll3R
    }
    #[doc = "LSE selected as peripheral clock"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == LPTIM2SEL::Lse
    }
    #[doc = "LSI selected as peripheral clock"]
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == LPTIM2SEL::Lsi
    }
    #[doc = "PER selected as peripheral clock"]
    #[inline(always)]
    pub fn is_per(&self) -> bool {
        *self == LPTIM2SEL::Per
    }
}
#[doc = "Field `LPTIM2SEL` writer - LPTIM2 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
pub type LPTIM2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, LPTIM2SEL>;
impl<'a, REG> LPTIM2SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "rcc_pclk4 selected as peripheral clock"]
    #[inline(always)]
    pub fn rcc_pclk4(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM2SEL::RccPclk4)
    }
    #[doc = "pll2_p selected as peripheral clock"]
    #[inline(always)]
    pub fn pll2_p(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM2SEL::Pll2P)
    }
    #[doc = "pll3_r selected as peripheral clock"]
    #[inline(always)]
    pub fn pll3_r(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM2SEL::Pll3R)
    }
    #[doc = "LSE selected as peripheral clock"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM2SEL::Lse)
    }
    #[doc = "LSI selected as peripheral clock"]
    #[inline(always)]
    pub fn lsi(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM2SEL::Lsi)
    }
    #[doc = "PER selected as peripheral clock"]
    #[inline(always)]
    pub fn per(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM2SEL::Per)
    }
}
#[doc = "Field `LPTIM3SEL` reader - LPTIM3 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
pub type LPTIM3SEL_R = crate::FieldReader;
#[doc = "Field `LPTIM3SEL` writer - LPTIM3 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
pub type LPTIM3SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "SAR ADC kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADCSEL {
    #[doc = "0: pll2_p selected as peripheral clock"]
    Pll2P = 0,
    #[doc = "1: pll3_r selected as peripheral clock"]
    Pll3R = 1,
    #[doc = "2: PER selected as peripheral clock"]
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
#[doc = "Field `ADCSEL` reader - SAR ADC kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
pub type ADCSEL_R = crate::FieldReader<ADCSEL>;
impl ADCSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ADCSEL> {
        match self.bits {
            0 => Some(ADCSEL::Pll2P),
            1 => Some(ADCSEL::Pll3R),
            2 => Some(ADCSEL::Per),
            _ => None,
        }
    }
    #[doc = "pll2_p selected as peripheral clock"]
    #[inline(always)]
    pub fn is_pll2_p(&self) -> bool {
        *self == ADCSEL::Pll2P
    }
    #[doc = "pll3_r selected as peripheral clock"]
    #[inline(always)]
    pub fn is_pll3_r(&self) -> bool {
        *self == ADCSEL::Pll3R
    }
    #[doc = "PER selected as peripheral clock"]
    #[inline(always)]
    pub fn is_per(&self) -> bool {
        *self == ADCSEL::Per
    }
}
#[doc = "Field `ADCSEL` writer - SAR ADC kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
pub type ADCSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, ADCSEL>;
impl<'a, REG> ADCSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "pll2_p selected as peripheral clock"]
    #[inline(always)]
    pub fn pll2_p(self) -> &'a mut crate::W<REG> {
        self.variant(ADCSEL::Pll2P)
    }
    #[doc = "pll3_r selected as peripheral clock"]
    #[inline(always)]
    pub fn pll3_r(self) -> &'a mut crate::W<REG> {
        self.variant(ADCSEL::Pll3R)
    }
    #[doc = "PER selected as peripheral clock"]
    #[inline(always)]
    pub fn per(self) -> &'a mut crate::W<REG> {
        self.variant(ADCSEL::Per)
    }
}
#[doc = "Field `DFSDM2SEL` reader - DFSDM2 kernel Clk clock source selection Set and reset by software. Note: The DFSDM2 Aclk clock source selection is done by SPI6SEL (see and )."]
pub type DFSDM2SEL_R = crate::BitReader;
#[doc = "Field `DFSDM2SEL` writer - DFSDM2 kernel Clk clock source selection Set and reset by software. Note: The DFSDM2 Aclk clock source selection is done by SPI6SEL (see and )."]
pub type DFSDM2SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "SPI6 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPI6SEL {
    #[doc = "0: rcc_pclk4 selected as peripheral clock"]
    RccPclk4 = 0,
    #[doc = "1: pll2_q selected as peripheral clock"]
    Pll2Q = 1,
    #[doc = "2: pll3_q selected as peripheral clock"]
    Pll3Q = 2,
    #[doc = "3: hsi_ker selected as peripheral clock"]
    HsiKer = 3,
    #[doc = "4: csi_ker selected as peripheral clock"]
    CsiKer = 4,
    #[doc = "5: HSE selected as peripheral clock"]
    Hse = 5,
}
impl From<SPI6SEL> for u8 {
    #[inline(always)]
    fn from(variant: SPI6SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SPI6SEL {
    type Ux = u8;
}
#[doc = "Field `SPI6SEL` reader - SPI6 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
pub type SPI6SEL_R = crate::FieldReader<SPI6SEL>;
impl SPI6SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SPI6SEL> {
        match self.bits {
            0 => Some(SPI6SEL::RccPclk4),
            1 => Some(SPI6SEL::Pll2Q),
            2 => Some(SPI6SEL::Pll3Q),
            3 => Some(SPI6SEL::HsiKer),
            4 => Some(SPI6SEL::CsiKer),
            5 => Some(SPI6SEL::Hse),
            _ => None,
        }
    }
    #[doc = "rcc_pclk4 selected as peripheral clock"]
    #[inline(always)]
    pub fn is_rcc_pclk4(&self) -> bool {
        *self == SPI6SEL::RccPclk4
    }
    #[doc = "pll2_q selected as peripheral clock"]
    #[inline(always)]
    pub fn is_pll2_q(&self) -> bool {
        *self == SPI6SEL::Pll2Q
    }
    #[doc = "pll3_q selected as peripheral clock"]
    #[inline(always)]
    pub fn is_pll3_q(&self) -> bool {
        *self == SPI6SEL::Pll3Q
    }
    #[doc = "hsi_ker selected as peripheral clock"]
    #[inline(always)]
    pub fn is_hsi_ker(&self) -> bool {
        *self == SPI6SEL::HsiKer
    }
    #[doc = "csi_ker selected as peripheral clock"]
    #[inline(always)]
    pub fn is_csi_ker(&self) -> bool {
        *self == SPI6SEL::CsiKer
    }
    #[doc = "HSE selected as peripheral clock"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == SPI6SEL::Hse
    }
}
#[doc = "Field `SPI6SEL` writer - SPI6 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
pub type SPI6SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SPI6SEL>;
impl<'a, REG> SPI6SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "rcc_pclk4 selected as peripheral clock"]
    #[inline(always)]
    pub fn rcc_pclk4(self) -> &'a mut crate::W<REG> {
        self.variant(SPI6SEL::RccPclk4)
    }
    #[doc = "pll2_q selected as peripheral clock"]
    #[inline(always)]
    pub fn pll2_q(self) -> &'a mut crate::W<REG> {
        self.variant(SPI6SEL::Pll2Q)
    }
    #[doc = "pll3_q selected as peripheral clock"]
    #[inline(always)]
    pub fn pll3_q(self) -> &'a mut crate::W<REG> {
        self.variant(SPI6SEL::Pll3Q)
    }
    #[doc = "hsi_ker selected as peripheral clock"]
    #[inline(always)]
    pub fn hsi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(SPI6SEL::HsiKer)
    }
    #[doc = "csi_ker selected as peripheral clock"]
    #[inline(always)]
    pub fn csi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(SPI6SEL::CsiKer)
    }
    #[doc = "HSE selected as peripheral clock"]
    #[inline(always)]
    pub fn hse(self) -> &'a mut crate::W<REG> {
        self.variant(SPI6SEL::Hse)
    }
}
impl R {
    #[doc = "Bits 0:2 - LPUART1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn lpuart1sel(&self) -> LPUART1SEL_R {
        LPUART1SEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:9 - I2C4 kernel clock source selection Set and reset by software."]
    #[inline(always)]
    pub fn i2c4sel(&self) -> I2C4SEL_R {
        I2C4SEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:12 - LPTIM2 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn lptim2sel(&self) -> LPTIM2SEL_R {
        LPTIM2SEL_R::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bits 13:15 - LPTIM3 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn lptim3sel(&self) -> LPTIM3SEL_R {
        LPTIM3SEL_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:17 - SAR ADC kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn adcsel(&self) -> ADCSEL_R {
        ADCSEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 27 - DFSDM2 kernel Clk clock source selection Set and reset by software. Note: The DFSDM2 Aclk clock source selection is done by SPI6SEL (see and )."]
    #[inline(always)]
    pub fn dfsdm2sel(&self) -> DFSDM2SEL_R {
        DFSDM2SEL_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - SPI6 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn spi6sel(&self) -> SPI6SEL_R {
        SPI6SEL_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - LPUART1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1sel(&mut self) -> LPUART1SEL_W<SRDCCIPRrs> {
        LPUART1SEL_W::new(self, 0)
    }
    #[doc = "Bits 8:9 - I2C4 kernel clock source selection Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn i2c4sel(&mut self) -> I2C4SEL_W<SRDCCIPRrs> {
        I2C4SEL_W::new(self, 8)
    }
    #[doc = "Bits 10:12 - LPTIM2 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn lptim2sel(&mut self) -> LPTIM2SEL_W<SRDCCIPRrs> {
        LPTIM2SEL_W::new(self, 10)
    }
    #[doc = "Bits 13:15 - LPTIM3 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn lptim3sel(&mut self) -> LPTIM3SEL_W<SRDCCIPRrs> {
        LPTIM3SEL_W::new(self, 13)
    }
    #[doc = "Bits 16:17 - SAR ADC kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn adcsel(&mut self) -> ADCSEL_W<SRDCCIPRrs> {
        ADCSEL_W::new(self, 16)
    }
    #[doc = "Bit 27 - DFSDM2 kernel Clk clock source selection Set and reset by software. Note: The DFSDM2 Aclk clock source selection is done by SPI6SEL (see and )."]
    #[inline(always)]
    #[must_use]
    pub fn dfsdm2sel(&mut self) -> DFSDM2SEL_W<SRDCCIPRrs> {
        DFSDM2SEL_W::new(self, 27)
    }
    #[doc = "Bits 28:30 - SPI6 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn spi6sel(&mut self) -> SPI6SEL_W<SRDCCIPRrs> {
        SPI6SEL_W::new(self, 28)
    }
}
#[doc = "RCC SmartRun domain kernel clock configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srdccipr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srdccipr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRDCCIPRrs;
impl crate::RegisterSpec for SRDCCIPRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srdccipr::R`](R) reader structure"]
impl crate::Readable for SRDCCIPRrs {}
#[doc = "`write(|w| ..)` method takes [`srdccipr::W`](W) writer structure"]
impl crate::Writable for SRDCCIPRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRDCCIPR to value 0"]
impl crate::Resettable for SRDCCIPRrs {
    const RESET_VALUE: u32 = 0;
}
