#[doc = "Register `CCIPR2` reader"]
pub type R = crate::R<CCIPR2rs>;
#[doc = "Register `CCIPR2` writer"]
pub type W = crate::W<CCIPR2rs>;
#[doc = "I2C4 clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2C4SEL {
    #[doc = "0: PCLK clock selected"]
    Pclk = 0,
    #[doc = "1: SYSCLK clock selected"]
    Sysclk = 1,
    #[doc = "2: HSI16 clock selected"]
    Hsi16 = 2,
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
#[doc = "Field `I2C4SEL` reader - I2C4 clock source selection"]
pub type I2C4SEL_R = crate::FieldReader<I2C4SEL>;
impl I2C4SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<I2C4SEL> {
        match self.bits {
            0 => Some(I2C4SEL::Pclk),
            1 => Some(I2C4SEL::Sysclk),
            2 => Some(I2C4SEL::Hsi16),
            _ => None,
        }
    }
    #[doc = "PCLK clock selected"]
    #[inline(always)]
    pub fn is_pclk(&self) -> bool {
        *self == I2C4SEL::Pclk
    }
    #[doc = "SYSCLK clock selected"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == I2C4SEL::Sysclk
    }
    #[doc = "HSI16 clock selected"]
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == I2C4SEL::Hsi16
    }
}
#[doc = "Field `I2C4SEL` writer - I2C4 clock source selection"]
pub type I2C4SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, I2C4SEL>;
impl<'a, REG> I2C4SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PCLK clock selected"]
    #[inline(always)]
    pub fn pclk(self) -> &'a mut crate::W<REG> {
        self.variant(I2C4SEL::Pclk)
    }
    #[doc = "SYSCLK clock selected"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut crate::W<REG> {
        self.variant(I2C4SEL::Sysclk)
    }
    #[doc = "HSI16 clock selected"]
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut crate::W<REG> {
        self.variant(I2C4SEL::Hsi16)
    }
}
#[doc = "Digital filter for sigma delta modulator kernel clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DFSDMSEL {
    #[doc = "0: APB2 clock (PCLK2) selected as DFSDM kernel clock"]
    Pclk2 = 0,
    #[doc = "1: System clock selected as DFSDM kernel clock"]
    Sysclk = 1,
}
impl From<DFSDMSEL> for bool {
    #[inline(always)]
    fn from(variant: DFSDMSEL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DFSDMSEL` reader - Digital filter for sigma delta modulator kernel clock source selection"]
pub type DFSDMSEL_R = crate::BitReader<DFSDMSEL>;
impl DFSDMSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DFSDMSEL {
        match self.bits {
            false => DFSDMSEL::Pclk2,
            true => DFSDMSEL::Sysclk,
        }
    }
    #[doc = "APB2 clock (PCLK2) selected as DFSDM kernel clock"]
    #[inline(always)]
    pub fn is_pclk2(&self) -> bool {
        *self == DFSDMSEL::Pclk2
    }
    #[doc = "System clock selected as DFSDM kernel clock"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == DFSDMSEL::Sysclk
    }
}
#[doc = "Field `DFSDMSEL` writer - Digital filter for sigma delta modulator kernel clock source selection"]
pub type DFSDMSEL_W<'a, REG> = crate::BitWriter<'a, REG, DFSDMSEL>;
impl<'a, REG> DFSDMSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "APB2 clock (PCLK2) selected as DFSDM kernel clock"]
    #[inline(always)]
    pub fn pclk2(self) -> &'a mut crate::W<REG> {
        self.variant(DFSDMSEL::Pclk2)
    }
    #[doc = "System clock selected as DFSDM kernel clock"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut crate::W<REG> {
        self.variant(DFSDMSEL::Sysclk)
    }
}
#[doc = "Digital filter for sigma delta modulator audio clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADFSDMSEL {
    #[doc = "0: SAI1clock selected as DFSDM audio clock"]
    Sai1 = 0,
    #[doc = "1: HSI clock selected as DFSDM audio clock"]
    Hsi = 1,
    #[doc = "2: MSI clock selected as DFSDM audio clock"]
    Msi = 2,
}
impl From<ADFSDMSEL> for u8 {
    #[inline(always)]
    fn from(variant: ADFSDMSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ADFSDMSEL {
    type Ux = u8;
}
#[doc = "Field `ADFSDMSEL` reader - Digital filter for sigma delta modulator audio clock source selection"]
pub type ADFSDMSEL_R = crate::FieldReader<ADFSDMSEL>;
impl ADFSDMSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ADFSDMSEL> {
        match self.bits {
            0 => Some(ADFSDMSEL::Sai1),
            1 => Some(ADFSDMSEL::Hsi),
            2 => Some(ADFSDMSEL::Msi),
            _ => None,
        }
    }
    #[doc = "SAI1clock selected as DFSDM audio clock"]
    #[inline(always)]
    pub fn is_sai1(&self) -> bool {
        *self == ADFSDMSEL::Sai1
    }
    #[doc = "HSI clock selected as DFSDM audio clock"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == ADFSDMSEL::Hsi
    }
    #[doc = "MSI clock selected as DFSDM audio clock"]
    #[inline(always)]
    pub fn is_msi(&self) -> bool {
        *self == ADFSDMSEL::Msi
    }
}
#[doc = "Field `ADFSDMSEL` writer - Digital filter for sigma delta modulator audio clock source selection"]
pub type ADFSDMSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, ADFSDMSEL>;
impl<'a, REG> ADFSDMSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SAI1clock selected as DFSDM audio clock"]
    #[inline(always)]
    pub fn sai1(self) -> &'a mut crate::W<REG> {
        self.variant(ADFSDMSEL::Sai1)
    }
    #[doc = "HSI clock selected as DFSDM audio clock"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut crate::W<REG> {
        self.variant(ADFSDMSEL::Hsi)
    }
    #[doc = "MSI clock selected as DFSDM audio clock"]
    #[inline(always)]
    pub fn msi(self) -> &'a mut crate::W<REG> {
        self.variant(ADFSDMSEL::Msi)
    }
}
#[doc = "SAI1 clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SAI1SEL {
    #[doc = "0: PLLSAI1CLK clock is selected as SAIx clock"]
    Pllsai1clk = 0,
    #[doc = "1: PLLSAI2CLK clock is selected as SAIx clock"]
    Pllsai2clk = 1,
    #[doc = "2: PLLSAI3CLK clock is selected as SAIx clock"]
    Pllsai3clk = 2,
    #[doc = "3: External clock SAIx_EXTCLK clock selected as SAIx clock"]
    Sai2Extclk = 3,
    #[doc = "4: HSI clock selected as SAIx clock"]
    Hsi = 4,
}
impl From<SAI1SEL> for u8 {
    #[inline(always)]
    fn from(variant: SAI1SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SAI1SEL {
    type Ux = u8;
}
#[doc = "Field `SAI1SEL` reader - SAI1 clock source selection"]
pub type SAI1SEL_R = crate::FieldReader<SAI1SEL>;
impl SAI1SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SAI1SEL> {
        match self.bits {
            0 => Some(SAI1SEL::Pllsai1clk),
            1 => Some(SAI1SEL::Pllsai2clk),
            2 => Some(SAI1SEL::Pllsai3clk),
            3 => Some(SAI1SEL::Sai2Extclk),
            4 => Some(SAI1SEL::Hsi),
            _ => None,
        }
    }
    #[doc = "PLLSAI1CLK clock is selected as SAIx clock"]
    #[inline(always)]
    pub fn is_pllsai1clk(&self) -> bool {
        *self == SAI1SEL::Pllsai1clk
    }
    #[doc = "PLLSAI2CLK clock is selected as SAIx clock"]
    #[inline(always)]
    pub fn is_pllsai2clk(&self) -> bool {
        *self == SAI1SEL::Pllsai2clk
    }
    #[doc = "PLLSAI3CLK clock is selected as SAIx clock"]
    #[inline(always)]
    pub fn is_pllsai3clk(&self) -> bool {
        *self == SAI1SEL::Pllsai3clk
    }
    #[doc = "External clock SAIx_EXTCLK clock selected as SAIx clock"]
    #[inline(always)]
    pub fn is_sai2_extclk(&self) -> bool {
        *self == SAI1SEL::Sai2Extclk
    }
    #[doc = "HSI clock selected as SAIx clock"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == SAI1SEL::Hsi
    }
}
#[doc = "Field `SAI1SEL` writer - SAI1 clock source selection"]
pub type SAI1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SAI1SEL>;
impl<'a, REG> SAI1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PLLSAI1CLK clock is selected as SAIx clock"]
    #[inline(always)]
    pub fn pllsai1clk(self) -> &'a mut crate::W<REG> {
        self.variant(SAI1SEL::Pllsai1clk)
    }
    #[doc = "PLLSAI2CLK clock is selected as SAIx clock"]
    #[inline(always)]
    pub fn pllsai2clk(self) -> &'a mut crate::W<REG> {
        self.variant(SAI1SEL::Pllsai2clk)
    }
    #[doc = "PLLSAI3CLK clock is selected as SAIx clock"]
    #[inline(always)]
    pub fn pllsai3clk(self) -> &'a mut crate::W<REG> {
        self.variant(SAI1SEL::Pllsai3clk)
    }
    #[doc = "External clock SAIx_EXTCLK clock selected as SAIx clock"]
    #[inline(always)]
    pub fn sai2_extclk(self) -> &'a mut crate::W<REG> {
        self.variant(SAI1SEL::Sai2Extclk)
    }
    #[doc = "HSI clock selected as SAIx clock"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut crate::W<REG> {
        self.variant(SAI1SEL::Hsi)
    }
}
#[doc = "Field `SAI2SEL` reader - SAI2 clock source selection"]
pub use SAI1SEL_R as SAI2SEL_R;
#[doc = "Field `SAI2SEL` writer - SAI2 clock source selection"]
pub use SAI1SEL_W as SAI2SEL_W;
#[doc = "clock selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSISEL {
    #[doc = "0: DSI-PHY is selected as DSI byte lane clock source (usual case)"]
    Dsiphy = 0,
    #[doc = "1: PLLDSICLK is selected as DSI byte lane clock source, used in case DSI PLL and DSIPHY are off (low-power mode)"]
    Plldsiclk = 1,
}
impl From<DSISEL> for bool {
    #[inline(always)]
    fn from(variant: DSISEL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSISEL` reader - clock selection"]
pub type DSISEL_R = crate::BitReader<DSISEL>;
impl DSISEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DSISEL {
        match self.bits {
            false => DSISEL::Dsiphy,
            true => DSISEL::Plldsiclk,
        }
    }
    #[doc = "DSI-PHY is selected as DSI byte lane clock source (usual case)"]
    #[inline(always)]
    pub fn is_dsiphy(&self) -> bool {
        *self == DSISEL::Dsiphy
    }
    #[doc = "PLLDSICLK is selected as DSI byte lane clock source, used in case DSI PLL and DSIPHY are off (low-power mode)"]
    #[inline(always)]
    pub fn is_plldsiclk(&self) -> bool {
        *self == DSISEL::Plldsiclk
    }
}
#[doc = "Field `DSISEL` writer - clock selection"]
pub type DSISEL_W<'a, REG> = crate::BitWriter<'a, REG, DSISEL>;
impl<'a, REG> DSISEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DSI-PHY is selected as DSI byte lane clock source (usual case)"]
    #[inline(always)]
    pub fn dsiphy(self) -> &'a mut crate::W<REG> {
        self.variant(DSISEL::Dsiphy)
    }
    #[doc = "PLLDSICLK is selected as DSI byte lane clock source, used in case DSI PLL and DSIPHY are off (low-power mode)"]
    #[inline(always)]
    pub fn plldsiclk(self) -> &'a mut crate::W<REG> {
        self.variant(DSISEL::Plldsiclk)
    }
}
#[doc = "SDMMC clock selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDMMCSEL {
    #[doc = "0: 48 MHz clock is selected as SDMMC kernel clock"]
    Hsi48 = 0,
    #[doc = "1: PLLSAI3CLK is selected as SDMMC kernel clock, used in case higher frequency than 48MHz is needed (for SDR50 mode)"]
    Pllsai3clk = 1,
}
impl From<SDMMCSEL> for bool {
    #[inline(always)]
    fn from(variant: SDMMCSEL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDMMCSEL` reader - SDMMC clock selection"]
pub type SDMMCSEL_R = crate::BitReader<SDMMCSEL>;
impl SDMMCSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SDMMCSEL {
        match self.bits {
            false => SDMMCSEL::Hsi48,
            true => SDMMCSEL::Pllsai3clk,
        }
    }
    #[doc = "48 MHz clock is selected as SDMMC kernel clock"]
    #[inline(always)]
    pub fn is_hsi48(&self) -> bool {
        *self == SDMMCSEL::Hsi48
    }
    #[doc = "PLLSAI3CLK is selected as SDMMC kernel clock, used in case higher frequency than 48MHz is needed (for SDR50 mode)"]
    #[inline(always)]
    pub fn is_pllsai3clk(&self) -> bool {
        *self == SDMMCSEL::Pllsai3clk
    }
}
#[doc = "Field `SDMMCSEL` writer - SDMMC clock selection"]
pub type SDMMCSEL_W<'a, REG> = crate::BitWriter<'a, REG, SDMMCSEL>;
impl<'a, REG> SDMMCSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "48 MHz clock is selected as SDMMC kernel clock"]
    #[inline(always)]
    pub fn hsi48(self) -> &'a mut crate::W<REG> {
        self.variant(SDMMCSEL::Hsi48)
    }
    #[doc = "PLLSAI3CLK is selected as SDMMC kernel clock, used in case higher frequency than 48MHz is needed (for SDR50 mode)"]
    #[inline(always)]
    pub fn pllsai3clk(self) -> &'a mut crate::W<REG> {
        self.variant(SDMMCSEL::Pllsai3clk)
    }
}
#[doc = "division factor for LTDC clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLSAI2DIVR {
    #[doc = "0: PLLSAI2DIVR = /2"]
    Div2 = 0,
    #[doc = "1: PLLSAI2DIVR = /4"]
    Div4 = 1,
    #[doc = "2: PLLSAI2DIVR = /8"]
    Div8 = 2,
    #[doc = "3: PLLSAI2DIVR = /16"]
    Div16 = 3,
}
impl From<PLLSAI2DIVR> for u8 {
    #[inline(always)]
    fn from(variant: PLLSAI2DIVR) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLLSAI2DIVR {
    type Ux = u8;
}
#[doc = "Field `PLLSAI2DIVR` reader - division factor for LTDC clock"]
pub type PLLSAI2DIVR_R = crate::FieldReader<PLLSAI2DIVR>;
impl PLLSAI2DIVR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PLLSAI2DIVR {
        match self.bits {
            0 => PLLSAI2DIVR::Div2,
            1 => PLLSAI2DIVR::Div4,
            2 => PLLSAI2DIVR::Div8,
            3 => PLLSAI2DIVR::Div16,
            _ => unreachable!(),
        }
    }
    #[doc = "PLLSAI2DIVR = /2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLSAI2DIVR::Div2
    }
    #[doc = "PLLSAI2DIVR = /4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PLLSAI2DIVR::Div4
    }
    #[doc = "PLLSAI2DIVR = /8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PLLSAI2DIVR::Div8
    }
    #[doc = "PLLSAI2DIVR = /16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PLLSAI2DIVR::Div16
    }
}
#[doc = "Field `PLLSAI2DIVR` writer - division factor for LTDC clock"]
pub type PLLSAI2DIVR_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, PLLSAI2DIVR>;
impl<'a, REG> PLLSAI2DIVR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PLLSAI2DIVR = /2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI2DIVR::Div2)
    }
    #[doc = "PLLSAI2DIVR = /4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI2DIVR::Div4)
    }
    #[doc = "PLLSAI2DIVR = /8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI2DIVR::Div8)
    }
    #[doc = "PLLSAI2DIVR = /16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI2DIVR::Div16)
    }
}
#[doc = "Octospi clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OSPISEL {
    #[doc = "0: System clock selected as OctoSPI kernel clock"]
    Sysclk = 0,
    #[doc = "1: MSI clock selected as OctoSPI kernel clock"]
    Msi = 1,
    #[doc = "2: PLL48M1CLK clock selected as OctoSPI kernel clock"]
    Pll48m1clk = 2,
}
impl From<OSPISEL> for u8 {
    #[inline(always)]
    fn from(variant: OSPISEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OSPISEL {
    type Ux = u8;
}
#[doc = "Field `OSPISEL` reader - Octospi clock source selection"]
pub type OSPISEL_R = crate::FieldReader<OSPISEL>;
impl OSPISEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<OSPISEL> {
        match self.bits {
            0 => Some(OSPISEL::Sysclk),
            1 => Some(OSPISEL::Msi),
            2 => Some(OSPISEL::Pll48m1clk),
            _ => None,
        }
    }
    #[doc = "System clock selected as OctoSPI kernel clock"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == OSPISEL::Sysclk
    }
    #[doc = "MSI clock selected as OctoSPI kernel clock"]
    #[inline(always)]
    pub fn is_msi(&self) -> bool {
        *self == OSPISEL::Msi
    }
    #[doc = "PLL48M1CLK clock selected as OctoSPI kernel clock"]
    #[inline(always)]
    pub fn is_pll48m1clk(&self) -> bool {
        *self == OSPISEL::Pll48m1clk
    }
}
#[doc = "Field `OSPISEL` writer - Octospi clock source selection"]
pub type OSPISEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, OSPISEL>;
impl<'a, REG> OSPISEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "System clock selected as OctoSPI kernel clock"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut crate::W<REG> {
        self.variant(OSPISEL::Sysclk)
    }
    #[doc = "MSI clock selected as OctoSPI kernel clock"]
    #[inline(always)]
    pub fn msi(self) -> &'a mut crate::W<REG> {
        self.variant(OSPISEL::Msi)
    }
    #[doc = "PLL48M1CLK clock selected as OctoSPI kernel clock"]
    #[inline(always)]
    pub fn pll48m1clk(self) -> &'a mut crate::W<REG> {
        self.variant(OSPISEL::Pll48m1clk)
    }
}
impl R {
    #[doc = "Bits 0:1 - I2C4 clock source selection"]
    #[inline(always)]
    pub fn i2c4sel(&self) -> I2C4SEL_R {
        I2C4SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Digital filter for sigma delta modulator kernel clock source selection"]
    #[inline(always)]
    pub fn dfsdmsel(&self) -> DFSDMSEL_R {
        DFSDMSEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Digital filter for sigma delta modulator audio clock source selection"]
    #[inline(always)]
    pub fn adfsdmsel(&self) -> ADFSDMSEL_R {
        ADFSDMSEL_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:7 - SAI1 clock source selection"]
    #[inline(always)]
    pub fn sai1sel(&self) -> SAI1SEL_R {
        SAI1SEL_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:10 - SAI2 clock source selection"]
    #[inline(always)]
    pub fn sai2sel(&self) -> SAI2SEL_R {
        SAI2SEL_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 12 - clock selection"]
    #[inline(always)]
    pub fn dsisel(&self) -> DSISEL_R {
        DSISEL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - SDMMC clock selection"]
    #[inline(always)]
    pub fn sdmmcsel(&self) -> SDMMCSEL_R {
        SDMMCSEL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:17 - division factor for LTDC clock"]
    #[inline(always)]
    pub fn pllsai2divr(&self) -> PLLSAI2DIVR_R {
        PLLSAI2DIVR_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Octospi clock source selection"]
    #[inline(always)]
    pub fn ospisel(&self) -> OSPISEL_R {
        OSPISEL_R::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - I2C4 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn i2c4sel(&mut self) -> I2C4SEL_W<CCIPR2rs> {
        I2C4SEL_W::new(self, 0)
    }
    #[doc = "Bit 2 - Digital filter for sigma delta modulator kernel clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn dfsdmsel(&mut self) -> DFSDMSEL_W<CCIPR2rs> {
        DFSDMSEL_W::new(self, 2)
    }
    #[doc = "Bits 3:4 - Digital filter for sigma delta modulator audio clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn adfsdmsel(&mut self) -> ADFSDMSEL_W<CCIPR2rs> {
        ADFSDMSEL_W::new(self, 3)
    }
    #[doc = "Bits 5:7 - SAI1 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn sai1sel(&mut self) -> SAI1SEL_W<CCIPR2rs> {
        SAI1SEL_W::new(self, 5)
    }
    #[doc = "Bits 8:10 - SAI2 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn sai2sel(&mut self) -> SAI2SEL_W<CCIPR2rs> {
        SAI2SEL_W::new(self, 8)
    }
    #[doc = "Bit 12 - clock selection"]
    #[inline(always)]
    #[must_use]
    pub fn dsisel(&mut self) -> DSISEL_W<CCIPR2rs> {
        DSISEL_W::new(self, 12)
    }
    #[doc = "Bit 14 - SDMMC clock selection"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmcsel(&mut self) -> SDMMCSEL_W<CCIPR2rs> {
        SDMMCSEL_W::new(self, 14)
    }
    #[doc = "Bits 16:17 - division factor for LTDC clock"]
    #[inline(always)]
    #[must_use]
    pub fn pllsai2divr(&mut self) -> PLLSAI2DIVR_W<CCIPR2rs> {
        PLLSAI2DIVR_W::new(self, 16)
    }
    #[doc = "Bits 20:21 - Octospi clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn ospisel(&mut self) -> OSPISEL_W<CCIPR2rs> {
        OSPISEL_W::new(self, 20)
    }
}
#[doc = "Peripherals independent clock configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccipr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccipr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCIPR2rs;
impl crate::RegisterSpec for CCIPR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccipr2::R`](R) reader structure"]
impl crate::Readable for CCIPR2rs {}
#[doc = "`write(|w| ..)` method takes [`ccipr2::W`](W) writer structure"]
impl crate::Writable for CCIPR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCIPR2 to value 0"]
impl crate::Resettable for CCIPR2rs {
    const RESET_VALUE: u32 = 0;
}
