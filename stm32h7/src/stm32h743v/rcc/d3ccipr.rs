#[doc = "Register `D3CCIPR` reader"]
pub type R = crate::R<D3CCIPRrs>;
#[doc = "Register `D3CCIPR` writer"]
pub type W = crate::W<D3CCIPRrs>;
#[doc = "LPUART1 kernel clock source selection\n\nValue on reset: 0"]
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
#[doc = "Field `LPUART1SEL` reader - LPUART1 kernel clock source selection"]
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
#[doc = "Field `LPUART1SEL` writer - LPUART1 kernel clock source selection"]
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
#[doc = "I2C4 kernel clock source selection\n\nValue on reset: 0"]
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
#[doc = "Field `I2C4SEL` reader - I2C4 kernel clock source selection"]
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
#[doc = "Field `I2C4SEL` writer - I2C4 kernel clock source selection"]
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
#[doc = "LPTIM2 kernel clock source selection\n\nValue on reset: 0"]
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
#[doc = "Field `LPTIM2SEL` reader - LPTIM2 kernel clock source selection"]
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
#[doc = "Field `LPTIM2SEL` writer - LPTIM2 kernel clock source selection"]
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
#[doc = "Field `LPTIM345SEL` reader - LPTIM3,4,5 kernel clock source selection"]
pub use LPTIM2SEL_R as LPTIM345SEL_R;
#[doc = "Field `LPTIM345SEL` writer - LPTIM3,4,5 kernel clock source selection"]
pub use LPTIM2SEL_W as LPTIM345SEL_W;
#[doc = "SAR ADC kernel clock source selection\n\nValue on reset: 0"]
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
#[doc = "Field `ADCSEL` reader - SAR ADC kernel clock source selection"]
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
#[doc = "Field `ADCSEL` writer - SAR ADC kernel clock source selection"]
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
#[doc = "Sub-Block A of SAI4 kernel clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SAI4ASEL {
    #[doc = "0: pll1_q selected as peripheral clock"]
    Pll1Q = 0,
    #[doc = "1: pll2_p selected as peripheral clock"]
    Pll2P = 1,
    #[doc = "2: pll3_p selected as peripheral clock"]
    Pll3P = 2,
    #[doc = "3: i2s_ckin selected as peripheral clock"]
    I2sCkin = 3,
    #[doc = "4: PER selected as peripheral clock"]
    Per = 4,
}
impl From<SAI4ASEL> for u8 {
    #[inline(always)]
    fn from(variant: SAI4ASEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SAI4ASEL {
    type Ux = u8;
}
#[doc = "Field `SAI4ASEL` reader - Sub-Block A of SAI4 kernel clock source selection"]
pub type SAI4ASEL_R = crate::FieldReader<SAI4ASEL>;
impl SAI4ASEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SAI4ASEL> {
        match self.bits {
            0 => Some(SAI4ASEL::Pll1Q),
            1 => Some(SAI4ASEL::Pll2P),
            2 => Some(SAI4ASEL::Pll3P),
            3 => Some(SAI4ASEL::I2sCkin),
            4 => Some(SAI4ASEL::Per),
            _ => None,
        }
    }
    #[doc = "pll1_q selected as peripheral clock"]
    #[inline(always)]
    pub fn is_pll1_q(&self) -> bool {
        *self == SAI4ASEL::Pll1Q
    }
    #[doc = "pll2_p selected as peripheral clock"]
    #[inline(always)]
    pub fn is_pll2_p(&self) -> bool {
        *self == SAI4ASEL::Pll2P
    }
    #[doc = "pll3_p selected as peripheral clock"]
    #[inline(always)]
    pub fn is_pll3_p(&self) -> bool {
        *self == SAI4ASEL::Pll3P
    }
    #[doc = "i2s_ckin selected as peripheral clock"]
    #[inline(always)]
    pub fn is_i2s_ckin(&self) -> bool {
        *self == SAI4ASEL::I2sCkin
    }
    #[doc = "PER selected as peripheral clock"]
    #[inline(always)]
    pub fn is_per(&self) -> bool {
        *self == SAI4ASEL::Per
    }
}
#[doc = "Field `SAI4ASEL` writer - Sub-Block A of SAI4 kernel clock source selection"]
pub type SAI4ASEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SAI4ASEL>;
impl<'a, REG> SAI4ASEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "pll1_q selected as peripheral clock"]
    #[inline(always)]
    pub fn pll1_q(self) -> &'a mut crate::W<REG> {
        self.variant(SAI4ASEL::Pll1Q)
    }
    #[doc = "pll2_p selected as peripheral clock"]
    #[inline(always)]
    pub fn pll2_p(self) -> &'a mut crate::W<REG> {
        self.variant(SAI4ASEL::Pll2P)
    }
    #[doc = "pll3_p selected as peripheral clock"]
    #[inline(always)]
    pub fn pll3_p(self) -> &'a mut crate::W<REG> {
        self.variant(SAI4ASEL::Pll3P)
    }
    #[doc = "i2s_ckin selected as peripheral clock"]
    #[inline(always)]
    pub fn i2s_ckin(self) -> &'a mut crate::W<REG> {
        self.variant(SAI4ASEL::I2sCkin)
    }
    #[doc = "PER selected as peripheral clock"]
    #[inline(always)]
    pub fn per(self) -> &'a mut crate::W<REG> {
        self.variant(SAI4ASEL::Per)
    }
}
#[doc = "Field `SAI4BSEL` reader - Sub-Block B of SAI4 kernel clock source selection"]
pub use SAI4ASEL_R as SAI4BSEL_R;
#[doc = "Field `SAI4BSEL` writer - Sub-Block B of SAI4 kernel clock source selection"]
pub use SAI4ASEL_W as SAI4BSEL_W;
#[doc = "SPI6 kernel clock source selection\n\nValue on reset: 0"]
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
#[doc = "Field `SPI6SEL` reader - SPI6 kernel clock source selection"]
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
#[doc = "Field `SPI6SEL` writer - SPI6 kernel clock source selection"]
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
    #[doc = "Bits 0:2 - LPUART1 kernel clock source selection"]
    #[inline(always)]
    pub fn lpuart1sel(&self) -> LPUART1SEL_R {
        LPUART1SEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:9 - I2C4 kernel clock source selection"]
    #[inline(always)]
    pub fn i2c4sel(&self) -> I2C4SEL_R {
        I2C4SEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:12 - LPTIM2 kernel clock source selection"]
    #[inline(always)]
    pub fn lptim2sel(&self) -> LPTIM2SEL_R {
        LPTIM2SEL_R::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bits 13:15 - LPTIM3,4,5 kernel clock source selection"]
    #[inline(always)]
    pub fn lptim345sel(&self) -> LPTIM345SEL_R {
        LPTIM345SEL_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:17 - SAR ADC kernel clock source selection"]
    #[inline(always)]
    pub fn adcsel(&self) -> ADCSEL_R {
        ADCSEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 21:23 - Sub-Block A of SAI4 kernel clock source selection"]
    #[inline(always)]
    pub fn sai4asel(&self) -> SAI4ASEL_R {
        SAI4ASEL_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Sub-Block B of SAI4 kernel clock source selection"]
    #[inline(always)]
    pub fn sai4bsel(&self) -> SAI4BSEL_R {
        SAI4BSEL_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - SPI6 kernel clock source selection"]
    #[inline(always)]
    pub fn spi6sel(&self) -> SPI6SEL_R {
        SPI6SEL_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - LPUART1 kernel clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1sel(&mut self) -> LPUART1SEL_W<D3CCIPRrs> {
        LPUART1SEL_W::new(self, 0)
    }
    #[doc = "Bits 8:9 - I2C4 kernel clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn i2c4sel(&mut self) -> I2C4SEL_W<D3CCIPRrs> {
        I2C4SEL_W::new(self, 8)
    }
    #[doc = "Bits 10:12 - LPTIM2 kernel clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn lptim2sel(&mut self) -> LPTIM2SEL_W<D3CCIPRrs> {
        LPTIM2SEL_W::new(self, 10)
    }
    #[doc = "Bits 13:15 - LPTIM3,4,5 kernel clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn lptim345sel(&mut self) -> LPTIM345SEL_W<D3CCIPRrs> {
        LPTIM345SEL_W::new(self, 13)
    }
    #[doc = "Bits 16:17 - SAR ADC kernel clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn adcsel(&mut self) -> ADCSEL_W<D3CCIPRrs> {
        ADCSEL_W::new(self, 16)
    }
    #[doc = "Bits 21:23 - Sub-Block A of SAI4 kernel clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn sai4asel(&mut self) -> SAI4ASEL_W<D3CCIPRrs> {
        SAI4ASEL_W::new(self, 21)
    }
    #[doc = "Bits 24:26 - Sub-Block B of SAI4 kernel clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn sai4bsel(&mut self) -> SAI4BSEL_W<D3CCIPRrs> {
        SAI4BSEL_W::new(self, 24)
    }
    #[doc = "Bits 28:30 - SPI6 kernel clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn spi6sel(&mut self) -> SPI6SEL_W<D3CCIPRrs> {
        SPI6SEL_W::new(self, 28)
    }
}
#[doc = "RCC Domain 3 Kernel Clock Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d3ccipr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d3ccipr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D3CCIPRrs;
impl crate::RegisterSpec for D3CCIPRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d3ccipr::R`](R) reader structure"]
impl crate::Readable for D3CCIPRrs {}
#[doc = "`write(|w| ..)` method takes [`d3ccipr::W`](W) writer structure"]
impl crate::Writable for D3CCIPRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets D3CCIPR to value 0"]
impl crate::Resettable for D3CCIPRrs {
    const RESET_VALUE: u32 = 0;
}
