#[doc = "Register `CCIPR` reader"]
pub type R = crate::R<CCIPRrs>;
#[doc = "Register `CCIPR` writer"]
pub type W = crate::W<CCIPRrs>;
#[doc = "USART1 clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USART1SEL {
    #[doc = "0: PCLK clock selected"]
    Pclk = 0,
    #[doc = "1: SYSCLK clock selected"]
    Sysclk = 1,
    #[doc = "2: HSI16 clock selected"]
    Hsi16 = 2,
    #[doc = "3: LSE clock selected"]
    Lse = 3,
}
impl From<USART1SEL> for u8 {
    #[inline(always)]
    fn from(variant: USART1SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for USART1SEL {
    type Ux = u8;
}
#[doc = "Field `USART1SEL` reader - USART1 clock source selection"]
pub type USART1SEL_R = crate::FieldReader<USART1SEL>;
impl USART1SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USART1SEL {
        match self.bits {
            0 => USART1SEL::Pclk,
            1 => USART1SEL::Sysclk,
            2 => USART1SEL::Hsi16,
            3 => USART1SEL::Lse,
            _ => unreachable!(),
        }
    }
    #[doc = "PCLK clock selected"]
    #[inline(always)]
    pub fn is_pclk(&self) -> bool {
        *self == USART1SEL::Pclk
    }
    #[doc = "SYSCLK clock selected"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == USART1SEL::Sysclk
    }
    #[doc = "HSI16 clock selected"]
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == USART1SEL::Hsi16
    }
    #[doc = "LSE clock selected"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == USART1SEL::Lse
    }
}
#[doc = "Field `USART1SEL` writer - USART1 clock source selection"]
pub type USART1SEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, USART1SEL>;
impl<'a, REG> USART1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PCLK clock selected"]
    #[inline(always)]
    pub fn pclk(self) -> &'a mut crate::W<REG> {
        self.variant(USART1SEL::Pclk)
    }
    #[doc = "SYSCLK clock selected"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut crate::W<REG> {
        self.variant(USART1SEL::Sysclk)
    }
    #[doc = "HSI16 clock selected"]
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut crate::W<REG> {
        self.variant(USART1SEL::Hsi16)
    }
    #[doc = "LSE clock selected"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(USART1SEL::Lse)
    }
}
#[doc = "Field `USART2SEL` reader - USART2 clock source selection"]
pub use USART1SEL_R as USART2SEL_R;
#[doc = "Field `USART2SEL` writer - USART2 clock source selection"]
pub use USART1SEL_W as USART2SEL_W;
#[doc = "SPI2S2 I2S clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPI2S2SEL {
    #[doc = "1: PLLQ clock selected"]
    Pllq = 1,
    #[doc = "2: HSI16 clock selected"]
    Hsi16 = 2,
    #[doc = "3: External input I2S_CKIN selected"]
    I2s = 3,
}
impl From<SPI2S2SEL> for u8 {
    #[inline(always)]
    fn from(variant: SPI2S2SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SPI2S2SEL {
    type Ux = u8;
}
#[doc = "Field `SPI2S2SEL` reader - SPI2S2 I2S clock source selection"]
pub type SPI2S2SEL_R = crate::FieldReader<SPI2S2SEL>;
impl SPI2S2SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SPI2S2SEL> {
        match self.bits {
            1 => Some(SPI2S2SEL::Pllq),
            2 => Some(SPI2S2SEL::Hsi16),
            3 => Some(SPI2S2SEL::I2s),
            _ => None,
        }
    }
    #[doc = "PLLQ clock selected"]
    #[inline(always)]
    pub fn is_pllq(&self) -> bool {
        *self == SPI2S2SEL::Pllq
    }
    #[doc = "HSI16 clock selected"]
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == SPI2S2SEL::Hsi16
    }
    #[doc = "External input I2S_CKIN selected"]
    #[inline(always)]
    pub fn is_i2s(&self) -> bool {
        *self == SPI2S2SEL::I2s
    }
}
#[doc = "Field `SPI2S2SEL` writer - SPI2S2 I2S clock source selection"]
pub type SPI2S2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SPI2S2SEL>;
impl<'a, REG> SPI2S2SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PLLQ clock selected"]
    #[inline(always)]
    pub fn pllq(self) -> &'a mut crate::W<REG> {
        self.variant(SPI2S2SEL::Pllq)
    }
    #[doc = "HSI16 clock selected"]
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut crate::W<REG> {
        self.variant(SPI2S2SEL::Hsi16)
    }
    #[doc = "External input I2S_CKIN selected"]
    #[inline(always)]
    pub fn i2s(self) -> &'a mut crate::W<REG> {
        self.variant(SPI2S2SEL::I2s)
    }
}
#[doc = "LPUART1 clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LPUART1SEL {
    #[doc = "0: PCLK clock selected"]
    Pclk = 0,
    #[doc = "1: SYSCLK clock selected"]
    Sysclk = 1,
    #[doc = "2: HSI16 clock selected"]
    Hsi16 = 2,
    #[doc = "3: LSE clock selected"]
    Lse = 3,
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
#[doc = "Field `LPUART1SEL` reader - LPUART1 clock source selection"]
pub type LPUART1SEL_R = crate::FieldReader<LPUART1SEL>;
impl LPUART1SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LPUART1SEL {
        match self.bits {
            0 => LPUART1SEL::Pclk,
            1 => LPUART1SEL::Sysclk,
            2 => LPUART1SEL::Hsi16,
            3 => LPUART1SEL::Lse,
            _ => unreachable!(),
        }
    }
    #[doc = "PCLK clock selected"]
    #[inline(always)]
    pub fn is_pclk(&self) -> bool {
        *self == LPUART1SEL::Pclk
    }
    #[doc = "SYSCLK clock selected"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == LPUART1SEL::Sysclk
    }
    #[doc = "HSI16 clock selected"]
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == LPUART1SEL::Hsi16
    }
    #[doc = "LSE clock selected"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == LPUART1SEL::Lse
    }
}
#[doc = "Field `LPUART1SEL` writer - LPUART1 clock source selection"]
pub type LPUART1SEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, LPUART1SEL>;
impl<'a, REG> LPUART1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PCLK clock selected"]
    #[inline(always)]
    pub fn pclk(self) -> &'a mut crate::W<REG> {
        self.variant(LPUART1SEL::Pclk)
    }
    #[doc = "SYSCLK clock selected"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut crate::W<REG> {
        self.variant(LPUART1SEL::Sysclk)
    }
    #[doc = "HSI16 clock selected"]
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut crate::W<REG> {
        self.variant(LPUART1SEL::Hsi16)
    }
    #[doc = "LSE clock selected"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(LPUART1SEL::Lse)
    }
}
#[doc = "I2C1 clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2C1SEL {
    #[doc = "0: PCLK clock selected"]
    Pclk = 0,
    #[doc = "1: SYSCLK clock selected"]
    Sysclk = 1,
    #[doc = "2: HSI16 clock selected"]
    Hsi16 = 2,
}
impl From<I2C1SEL> for u8 {
    #[inline(always)]
    fn from(variant: I2C1SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for I2C1SEL {
    type Ux = u8;
}
#[doc = "Field `I2C1SEL` reader - I2C1 clock source selection"]
pub type I2C1SEL_R = crate::FieldReader<I2C1SEL>;
impl I2C1SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<I2C1SEL> {
        match self.bits {
            0 => Some(I2C1SEL::Pclk),
            1 => Some(I2C1SEL::Sysclk),
            2 => Some(I2C1SEL::Hsi16),
            _ => None,
        }
    }
    #[doc = "PCLK clock selected"]
    #[inline(always)]
    pub fn is_pclk(&self) -> bool {
        *self == I2C1SEL::Pclk
    }
    #[doc = "SYSCLK clock selected"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == I2C1SEL::Sysclk
    }
    #[doc = "HSI16 clock selected"]
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == I2C1SEL::Hsi16
    }
}
#[doc = "Field `I2C1SEL` writer - I2C1 clock source selection"]
pub type I2C1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, I2C1SEL>;
impl<'a, REG> I2C1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PCLK clock selected"]
    #[inline(always)]
    pub fn pclk(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1SEL::Pclk)
    }
    #[doc = "SYSCLK clock selected"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1SEL::Sysclk)
    }
    #[doc = "HSI16 clock selected"]
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1SEL::Hsi16)
    }
}
#[doc = "Field `I2C2SEL` reader - I2C2 clock source selection"]
pub use I2C1SEL_R as I2C2SEL_R;
#[doc = "Field `I2C3SEL` reader - I2C3 clock source selection"]
pub use I2C1SEL_R as I2C3SEL_R;
#[doc = "Field `I2C2SEL` writer - I2C2 clock source selection"]
pub use I2C1SEL_W as I2C2SEL_W;
#[doc = "Field `I2C3SEL` writer - I2C3 clock source selection"]
pub use I2C1SEL_W as I2C3SEL_W;
#[doc = "Low power timer 1 clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LPTIM1SEL {
    #[doc = "0: PCLK clock selected"]
    Pclk = 0,
    #[doc = "1: LSI clock selected"]
    Lsi = 1,
    #[doc = "2: HSI16 clock selected"]
    Hsi16 = 2,
    #[doc = "3: LSE clock selected"]
    Lse = 3,
}
impl From<LPTIM1SEL> for u8 {
    #[inline(always)]
    fn from(variant: LPTIM1SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LPTIM1SEL {
    type Ux = u8;
}
#[doc = "Field `LPTIM1SEL` reader - Low power timer 1 clock source selection"]
pub type LPTIM1SEL_R = crate::FieldReader<LPTIM1SEL>;
impl LPTIM1SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LPTIM1SEL {
        match self.bits {
            0 => LPTIM1SEL::Pclk,
            1 => LPTIM1SEL::Lsi,
            2 => LPTIM1SEL::Hsi16,
            3 => LPTIM1SEL::Lse,
            _ => unreachable!(),
        }
    }
    #[doc = "PCLK clock selected"]
    #[inline(always)]
    pub fn is_pclk(&self) -> bool {
        *self == LPTIM1SEL::Pclk
    }
    #[doc = "LSI clock selected"]
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == LPTIM1SEL::Lsi
    }
    #[doc = "HSI16 clock selected"]
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == LPTIM1SEL::Hsi16
    }
    #[doc = "LSE clock selected"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == LPTIM1SEL::Lse
    }
}
#[doc = "Field `LPTIM1SEL` writer - Low power timer 1 clock source selection"]
pub type LPTIM1SEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, LPTIM1SEL>;
impl<'a, REG> LPTIM1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PCLK clock selected"]
    #[inline(always)]
    pub fn pclk(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM1SEL::Pclk)
    }
    #[doc = "LSI clock selected"]
    #[inline(always)]
    pub fn lsi(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM1SEL::Lsi)
    }
    #[doc = "HSI16 clock selected"]
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM1SEL::Hsi16)
    }
    #[doc = "LSE clock selected"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM1SEL::Lse)
    }
}
#[doc = "Field `LPTIM2SEL` reader - Low power timer 2 clock source selection"]
pub use LPTIM1SEL_R as LPTIM2SEL_R;
#[doc = "Field `LPTIM3SEL` reader - Low power timer 3 clock source selection"]
pub use LPTIM1SEL_R as LPTIM3SEL_R;
#[doc = "Field `LPTIM2SEL` writer - Low power timer 2 clock source selection"]
pub use LPTIM1SEL_W as LPTIM2SEL_W;
#[doc = "Field `LPTIM3SEL` writer - Low power timer 3 clock source selection"]
pub use LPTIM1SEL_W as LPTIM3SEL_W;
#[doc = "ADC clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADCSEL {
    #[doc = "0: No clock selected"]
    NoClock = 0,
    #[doc = "1: HSI16 clock selected"]
    Hsi16 = 1,
    #[doc = "2: PLLP clock selected"]
    Pllp = 2,
    #[doc = "3: SYSCLK clock selected"]
    Sysclk = 3,
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
#[doc = "Field `ADCSEL` reader - ADC clock source selection"]
pub type ADCSEL_R = crate::FieldReader<ADCSEL>;
impl ADCSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADCSEL {
        match self.bits {
            0 => ADCSEL::NoClock,
            1 => ADCSEL::Hsi16,
            2 => ADCSEL::Pllp,
            3 => ADCSEL::Sysclk,
            _ => unreachable!(),
        }
    }
    #[doc = "No clock selected"]
    #[inline(always)]
    pub fn is_no_clock(&self) -> bool {
        *self == ADCSEL::NoClock
    }
    #[doc = "HSI16 clock selected"]
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == ADCSEL::Hsi16
    }
    #[doc = "PLLP clock selected"]
    #[inline(always)]
    pub fn is_pllp(&self) -> bool {
        *self == ADCSEL::Pllp
    }
    #[doc = "SYSCLK clock selected"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == ADCSEL::Sysclk
    }
}
#[doc = "Field `ADCSEL` writer - ADC clock source selection"]
pub type ADCSEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, ADCSEL>;
impl<'a, REG> ADCSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No clock selected"]
    #[inline(always)]
    pub fn no_clock(self) -> &'a mut crate::W<REG> {
        self.variant(ADCSEL::NoClock)
    }
    #[doc = "HSI16 clock selected"]
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut crate::W<REG> {
        self.variant(ADCSEL::Hsi16)
    }
    #[doc = "PLLP clock selected"]
    #[inline(always)]
    pub fn pllp(self) -> &'a mut crate::W<REG> {
        self.variant(ADCSEL::Pllp)
    }
    #[doc = "SYSCLK clock selected"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut crate::W<REG> {
        self.variant(ADCSEL::Sysclk)
    }
}
#[doc = "RNG clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RNGSEL {
    #[doc = "0: PLLQ clock selected"]
    Pllq = 0,
    #[doc = "1: LSI clock selected"]
    Lsi = 1,
    #[doc = "2: LSE clock selected"]
    Lse = 2,
    #[doc = "3: MSI clock selected"]
    Msi = 3,
}
impl From<RNGSEL> for u8 {
    #[inline(always)]
    fn from(variant: RNGSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RNGSEL {
    type Ux = u8;
}
#[doc = "Field `RNGSEL` reader - RNG clock source selection"]
pub type RNGSEL_R = crate::FieldReader<RNGSEL>;
impl RNGSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RNGSEL {
        match self.bits {
            0 => RNGSEL::Pllq,
            1 => RNGSEL::Lsi,
            2 => RNGSEL::Lse,
            3 => RNGSEL::Msi,
            _ => unreachable!(),
        }
    }
    #[doc = "PLLQ clock selected"]
    #[inline(always)]
    pub fn is_pllq(&self) -> bool {
        *self == RNGSEL::Pllq
    }
    #[doc = "LSI clock selected"]
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == RNGSEL::Lsi
    }
    #[doc = "LSE clock selected"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == RNGSEL::Lse
    }
    #[doc = "MSI clock selected"]
    #[inline(always)]
    pub fn is_msi(&self) -> bool {
        *self == RNGSEL::Msi
    }
}
#[doc = "Field `RNGSEL` writer - RNG clock source selection"]
pub type RNGSEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, RNGSEL>;
impl<'a, REG> RNGSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PLLQ clock selected"]
    #[inline(always)]
    pub fn pllq(self) -> &'a mut crate::W<REG> {
        self.variant(RNGSEL::Pllq)
    }
    #[doc = "LSI clock selected"]
    #[inline(always)]
    pub fn lsi(self) -> &'a mut crate::W<REG> {
        self.variant(RNGSEL::Lsi)
    }
    #[doc = "LSE clock selected"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(RNGSEL::Lse)
    }
    #[doc = "MSI clock selected"]
    #[inline(always)]
    pub fn msi(self) -> &'a mut crate::W<REG> {
        self.variant(RNGSEL::Msi)
    }
}
impl R {
    #[doc = "Bits 0:1 - USART1 clock source selection"]
    #[inline(always)]
    pub fn usart1sel(&self) -> USART1SEL_R {
        USART1SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - USART2 clock source selection"]
    #[inline(always)]
    pub fn usart2sel(&self) -> USART2SEL_R {
        USART2SEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 8:9 - SPI2S2 I2S clock source selection"]
    #[inline(always)]
    pub fn spi2s2sel(&self) -> SPI2S2SEL_R {
        SPI2S2SEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - LPUART1 clock source selection"]
    #[inline(always)]
    pub fn lpuart1sel(&self) -> LPUART1SEL_R {
        LPUART1SEL_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - I2C1 clock source selection"]
    #[inline(always)]
    pub fn i2c1sel(&self) -> I2C1SEL_R {
        I2C1SEL_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - I2C2 clock source selection"]
    #[inline(always)]
    pub fn i2c2sel(&self) -> I2C2SEL_R {
        I2C2SEL_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - I2C3 clock source selection"]
    #[inline(always)]
    pub fn i2c3sel(&self) -> I2C3SEL_R {
        I2C3SEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Low power timer 1 clock source selection"]
    #[inline(always)]
    pub fn lptim1sel(&self) -> LPTIM1SEL_R {
        LPTIM1SEL_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Low power timer 2 clock source selection"]
    #[inline(always)]
    pub fn lptim2sel(&self) -> LPTIM2SEL_R {
        LPTIM2SEL_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Low power timer 3 clock source selection"]
    #[inline(always)]
    pub fn lptim3sel(&self) -> LPTIM3SEL_R {
        LPTIM3SEL_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 28:29 - ADC clock source selection"]
    #[inline(always)]
    pub fn adcsel(&self) -> ADCSEL_R {
        ADCSEL_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - RNG clock source selection"]
    #[inline(always)]
    pub fn rngsel(&self) -> RNGSEL_R {
        RNGSEL_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - USART1 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn usart1sel(&mut self) -> USART1SEL_W<CCIPRrs> {
        USART1SEL_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - USART2 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn usart2sel(&mut self) -> USART2SEL_W<CCIPRrs> {
        USART2SEL_W::new(self, 2)
    }
    #[doc = "Bits 8:9 - SPI2S2 I2S clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn spi2s2sel(&mut self) -> SPI2S2SEL_W<CCIPRrs> {
        SPI2S2SEL_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - LPUART1 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1sel(&mut self) -> LPUART1SEL_W<CCIPRrs> {
        LPUART1SEL_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - I2C1 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1sel(&mut self) -> I2C1SEL_W<CCIPRrs> {
        I2C1SEL_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - I2C2 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2sel(&mut self) -> I2C2SEL_W<CCIPRrs> {
        I2C2SEL_W::new(self, 14)
    }
    #[doc = "Bits 16:17 - I2C3 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn i2c3sel(&mut self) -> I2C3SEL_W<CCIPRrs> {
        I2C3SEL_W::new(self, 16)
    }
    #[doc = "Bits 18:19 - Low power timer 1 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn lptim1sel(&mut self) -> LPTIM1SEL_W<CCIPRrs> {
        LPTIM1SEL_W::new(self, 18)
    }
    #[doc = "Bits 20:21 - Low power timer 2 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn lptim2sel(&mut self) -> LPTIM2SEL_W<CCIPRrs> {
        LPTIM2SEL_W::new(self, 20)
    }
    #[doc = "Bits 22:23 - Low power timer 3 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn lptim3sel(&mut self) -> LPTIM3SEL_W<CCIPRrs> {
        LPTIM3SEL_W::new(self, 22)
    }
    #[doc = "Bits 28:29 - ADC clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn adcsel(&mut self) -> ADCSEL_W<CCIPRrs> {
        ADCSEL_W::new(self, 28)
    }
    #[doc = "Bits 30:31 - RNG clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn rngsel(&mut self) -> RNGSEL_W<CCIPRrs> {
        RNGSEL_W::new(self, 30)
    }
}
#[doc = "Peripherals independent clock configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccipr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccipr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCIPRrs;
impl crate::RegisterSpec for CCIPRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccipr::R`](R) reader structure"]
impl crate::Readable for CCIPRrs {}
#[doc = "`write(|w| ..)` method takes [`ccipr::W`](W) writer structure"]
impl crate::Writable for CCIPRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCIPR to value 0"]
impl crate::Resettable for CCIPRrs {
    const RESET_VALUE: u32 = 0;
}
