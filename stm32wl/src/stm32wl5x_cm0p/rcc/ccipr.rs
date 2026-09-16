///Register `CCIPR` reader
pub type R = crate::R<CCIPRrs>;
///Register `CCIPR` writer
pub type W = crate::W<CCIPRrs>;
/**USART1 clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USART1SEL {
    ///0: PCLK clock selected
    Pclk = 0,
    ///1: SYSCLK clock selected
    Sysclk = 1,
    ///2: HSI16 clock selected
    Hsi16 = 2,
    ///3: LSE clock selected
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
impl crate::IsEnum for USART1SEL {}
///Field `USART1SEL` reader - USART1 clock source selection
pub type USART1SEL_R = crate::FieldReader<USART1SEL>;
impl USART1SEL_R {
    ///Get enumerated values variant
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
    ///PCLK clock selected
    #[inline(always)]
    pub fn is_pclk(&self) -> bool {
        *self == USART1SEL::Pclk
    }
    ///SYSCLK clock selected
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == USART1SEL::Sysclk
    }
    ///HSI16 clock selected
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == USART1SEL::Hsi16
    }
    ///LSE clock selected
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == USART1SEL::Lse
    }
}
///Field `USART1SEL` writer - USART1 clock source selection
pub type USART1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, USART1SEL, crate::Safe>;
impl<'a, REG> USART1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PCLK clock selected
    #[inline(always)]
    pub fn pclk(self) -> &'a mut crate::W<REG> {
        self.variant(USART1SEL::Pclk)
    }
    ///SYSCLK clock selected
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut crate::W<REG> {
        self.variant(USART1SEL::Sysclk)
    }
    ///HSI16 clock selected
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut crate::W<REG> {
        self.variant(USART1SEL::Hsi16)
    }
    ///LSE clock selected
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(USART1SEL::Lse)
    }
}
///Field `USART2SEL` reader - USART2 clock source selection
pub use USART1SEL_R as USART2SEL_R;
///Field `USART2SEL` writer - USART2 clock source selection
pub use USART1SEL_W as USART2SEL_W;
/**SPI2S2 I2S clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPI2S2SEL {
    ///1: PLLQ clock selected
    Pllq = 1,
    ///2: HSI16 clock selected
    Hsi16 = 2,
    ///3: External input I2S_CKIN selected
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
impl crate::IsEnum for SPI2S2SEL {}
///Field `SPI2S2SEL` reader - SPI2S2 I2S clock source selection
pub type SPI2S2SEL_R = crate::FieldReader<SPI2S2SEL>;
impl SPI2S2SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<SPI2S2SEL> {
        match self.bits {
            1 => Some(SPI2S2SEL::Pllq),
            2 => Some(SPI2S2SEL::Hsi16),
            3 => Some(SPI2S2SEL::I2s),
            _ => None,
        }
    }
    ///PLLQ clock selected
    #[inline(always)]
    pub fn is_pllq(&self) -> bool {
        *self == SPI2S2SEL::Pllq
    }
    ///HSI16 clock selected
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == SPI2S2SEL::Hsi16
    }
    ///External input I2S_CKIN selected
    #[inline(always)]
    pub fn is_i2s(&self) -> bool {
        *self == SPI2S2SEL::I2s
    }
}
///Field `SPI2S2SEL` writer - SPI2S2 I2S clock source selection
pub type SPI2S2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SPI2S2SEL>;
impl<'a, REG> SPI2S2SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PLLQ clock selected
    #[inline(always)]
    pub fn pllq(self) -> &'a mut crate::W<REG> {
        self.variant(SPI2S2SEL::Pllq)
    }
    ///HSI16 clock selected
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut crate::W<REG> {
        self.variant(SPI2S2SEL::Hsi16)
    }
    ///External input I2S_CKIN selected
    #[inline(always)]
    pub fn i2s(self) -> &'a mut crate::W<REG> {
        self.variant(SPI2S2SEL::I2s)
    }
}
/**LPUART1 clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LPUART1SEL {
    ///0: PCLK clock selected
    Pclk = 0,
    ///1: SYSCLK clock selected
    Sysclk = 1,
    ///2: HSI16 clock selected
    Hsi16 = 2,
    ///3: LSE clock selected
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
impl crate::IsEnum for LPUART1SEL {}
///Field `LPUART1SEL` reader - LPUART1 clock source selection
pub type LPUART1SEL_R = crate::FieldReader<LPUART1SEL>;
impl LPUART1SEL_R {
    ///Get enumerated values variant
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
    ///PCLK clock selected
    #[inline(always)]
    pub fn is_pclk(&self) -> bool {
        *self == LPUART1SEL::Pclk
    }
    ///SYSCLK clock selected
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == LPUART1SEL::Sysclk
    }
    ///HSI16 clock selected
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == LPUART1SEL::Hsi16
    }
    ///LSE clock selected
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == LPUART1SEL::Lse
    }
}
///Field `LPUART1SEL` writer - LPUART1 clock source selection
pub type LPUART1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, LPUART1SEL, crate::Safe>;
impl<'a, REG> LPUART1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PCLK clock selected
    #[inline(always)]
    pub fn pclk(self) -> &'a mut crate::W<REG> {
        self.variant(LPUART1SEL::Pclk)
    }
    ///SYSCLK clock selected
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut crate::W<REG> {
        self.variant(LPUART1SEL::Sysclk)
    }
    ///HSI16 clock selected
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut crate::W<REG> {
        self.variant(LPUART1SEL::Hsi16)
    }
    ///LSE clock selected
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(LPUART1SEL::Lse)
    }
}
/**I2C1 clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2C1SEL {
    ///0: PCLK clock selected
    Pclk = 0,
    ///1: SYSCLK clock selected
    Sysclk = 1,
    ///2: HSI16 clock selected
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
impl crate::IsEnum for I2C1SEL {}
///Field `I2C1SEL` reader - I2C1 clock source selection
pub type I2C1SEL_R = crate::FieldReader<I2C1SEL>;
impl I2C1SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<I2C1SEL> {
        match self.bits {
            0 => Some(I2C1SEL::Pclk),
            1 => Some(I2C1SEL::Sysclk),
            2 => Some(I2C1SEL::Hsi16),
            _ => None,
        }
    }
    ///PCLK clock selected
    #[inline(always)]
    pub fn is_pclk(&self) -> bool {
        *self == I2C1SEL::Pclk
    }
    ///SYSCLK clock selected
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == I2C1SEL::Sysclk
    }
    ///HSI16 clock selected
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == I2C1SEL::Hsi16
    }
}
///Field `I2C1SEL` writer - I2C1 clock source selection
pub type I2C1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, I2C1SEL>;
impl<'a, REG> I2C1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PCLK clock selected
    #[inline(always)]
    pub fn pclk(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1SEL::Pclk)
    }
    ///SYSCLK clock selected
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1SEL::Sysclk)
    }
    ///HSI16 clock selected
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1SEL::Hsi16)
    }
}
///Field `I2C2SEL` reader - I2C2 clock source selection
pub use I2C1SEL_R as I2C2SEL_R;
///Field `I2C3SEL` reader - I2C3 clock source selection
pub use I2C1SEL_R as I2C3SEL_R;
///Field `I2C2SEL` writer - I2C2 clock source selection
pub use I2C1SEL_W as I2C2SEL_W;
///Field `I2C3SEL` writer - I2C3 clock source selection
pub use I2C1SEL_W as I2C3SEL_W;
/**Low power timer 1 clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LPTIM1SEL {
    ///0: PCLK clock selected
    Pclk = 0,
    ///1: LSI clock selected
    Lsi = 1,
    ///2: HSI16 clock selected
    Hsi16 = 2,
    ///3: LSE clock selected
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
impl crate::IsEnum for LPTIM1SEL {}
///Field `LPTIM1SEL` reader - Low power timer 1 clock source selection
pub type LPTIM1SEL_R = crate::FieldReader<LPTIM1SEL>;
impl LPTIM1SEL_R {
    ///Get enumerated values variant
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
    ///PCLK clock selected
    #[inline(always)]
    pub fn is_pclk(&self) -> bool {
        *self == LPTIM1SEL::Pclk
    }
    ///LSI clock selected
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == LPTIM1SEL::Lsi
    }
    ///HSI16 clock selected
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == LPTIM1SEL::Hsi16
    }
    ///LSE clock selected
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == LPTIM1SEL::Lse
    }
}
///Field `LPTIM1SEL` writer - Low power timer 1 clock source selection
pub type LPTIM1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, LPTIM1SEL, crate::Safe>;
impl<'a, REG> LPTIM1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PCLK clock selected
    #[inline(always)]
    pub fn pclk(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM1SEL::Pclk)
    }
    ///LSI clock selected
    #[inline(always)]
    pub fn lsi(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM1SEL::Lsi)
    }
    ///HSI16 clock selected
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM1SEL::Hsi16)
    }
    ///LSE clock selected
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM1SEL::Lse)
    }
}
///Field `LPTIM2SEL` reader - Low power timer 2 clock source selection
pub use LPTIM1SEL_R as LPTIM2SEL_R;
///Field `LPTIM3SEL` reader - Low power timer 3 clock source selection
pub use LPTIM1SEL_R as LPTIM3SEL_R;
///Field `LPTIM2SEL` writer - Low power timer 2 clock source selection
pub use LPTIM1SEL_W as LPTIM2SEL_W;
///Field `LPTIM3SEL` writer - Low power timer 3 clock source selection
pub use LPTIM1SEL_W as LPTIM3SEL_W;
/**ADC clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADCSEL {
    ///0: No clock selected
    NoClock = 0,
    ///1: HSI16 clock selected
    Hsi16 = 1,
    ///2: PLLP clock selected
    Pllp = 2,
    ///3: SYSCLK clock selected
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
impl crate::IsEnum for ADCSEL {}
///Field `ADCSEL` reader - ADC clock source selection
pub type ADCSEL_R = crate::FieldReader<ADCSEL>;
impl ADCSEL_R {
    ///Get enumerated values variant
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
    ///No clock selected
    #[inline(always)]
    pub fn is_no_clock(&self) -> bool {
        *self == ADCSEL::NoClock
    }
    ///HSI16 clock selected
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == ADCSEL::Hsi16
    }
    ///PLLP clock selected
    #[inline(always)]
    pub fn is_pllp(&self) -> bool {
        *self == ADCSEL::Pllp
    }
    ///SYSCLK clock selected
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == ADCSEL::Sysclk
    }
}
///Field `ADCSEL` writer - ADC clock source selection
pub type ADCSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, ADCSEL, crate::Safe>;
impl<'a, REG> ADCSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No clock selected
    #[inline(always)]
    pub fn no_clock(self) -> &'a mut crate::W<REG> {
        self.variant(ADCSEL::NoClock)
    }
    ///HSI16 clock selected
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut crate::W<REG> {
        self.variant(ADCSEL::Hsi16)
    }
    ///PLLP clock selected
    #[inline(always)]
    pub fn pllp(self) -> &'a mut crate::W<REG> {
        self.variant(ADCSEL::Pllp)
    }
    ///SYSCLK clock selected
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut crate::W<REG> {
        self.variant(ADCSEL::Sysclk)
    }
}
/**RNG clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RNGSEL {
    ///0: PLLQ clock selected
    Pllq = 0,
    ///1: LSI clock selected
    Lsi = 1,
    ///2: LSE clock selected
    Lse = 2,
    ///3: MSI clock selected
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
impl crate::IsEnum for RNGSEL {}
///Field `RNGSEL` reader - RNG clock source selection
pub type RNGSEL_R = crate::FieldReader<RNGSEL>;
impl RNGSEL_R {
    ///Get enumerated values variant
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
    ///PLLQ clock selected
    #[inline(always)]
    pub fn is_pllq(&self) -> bool {
        *self == RNGSEL::Pllq
    }
    ///LSI clock selected
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == RNGSEL::Lsi
    }
    ///LSE clock selected
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == RNGSEL::Lse
    }
    ///MSI clock selected
    #[inline(always)]
    pub fn is_msi(&self) -> bool {
        *self == RNGSEL::Msi
    }
}
///Field `RNGSEL` writer - RNG clock source selection
pub type RNGSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, RNGSEL, crate::Safe>;
impl<'a, REG> RNGSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PLLQ clock selected
    #[inline(always)]
    pub fn pllq(self) -> &'a mut crate::W<REG> {
        self.variant(RNGSEL::Pllq)
    }
    ///LSI clock selected
    #[inline(always)]
    pub fn lsi(self) -> &'a mut crate::W<REG> {
        self.variant(RNGSEL::Lsi)
    }
    ///LSE clock selected
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(RNGSEL::Lse)
    }
    ///MSI clock selected
    #[inline(always)]
    pub fn msi(self) -> &'a mut crate::W<REG> {
        self.variant(RNGSEL::Msi)
    }
}
impl R {
    ///Bits 0:1 - USART1 clock source selection
    #[inline(always)]
    pub fn usart1sel(&self) -> USART1SEL_R {
        USART1SEL_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - USART2 clock source selection
    #[inline(always)]
    pub fn usart2sel(&self) -> USART2SEL_R {
        USART2SEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 8:9 - SPI2S2 I2S clock source selection
    #[inline(always)]
    pub fn spi2s2sel(&self) -> SPI2S2SEL_R {
        SPI2S2SEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - LPUART1 clock source selection
    #[inline(always)]
    pub fn lpuart1sel(&self) -> LPUART1SEL_R {
        LPUART1SEL_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:13 - I2C1 clock source selection
    #[inline(always)]
    pub fn i2c1sel(&self) -> I2C1SEL_R {
        I2C1SEL_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 14:15 - I2C2 clock source selection
    #[inline(always)]
    pub fn i2c2sel(&self) -> I2C2SEL_R {
        I2C2SEL_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bits 16:17 - I2C3 clock source selection
    #[inline(always)]
    pub fn i2c3sel(&self) -> I2C3SEL_R {
        I2C3SEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:19 - Low power timer 1 clock source selection
    #[inline(always)]
    pub fn lptim1sel(&self) -> LPTIM1SEL_R {
        LPTIM1SEL_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bits 20:21 - Low power timer 2 clock source selection
    #[inline(always)]
    pub fn lptim2sel(&self) -> LPTIM2SEL_R {
        LPTIM2SEL_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bits 22:23 - Low power timer 3 clock source selection
    #[inline(always)]
    pub fn lptim3sel(&self) -> LPTIM3SEL_R {
        LPTIM3SEL_R::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bits 28:29 - ADC clock source selection
    #[inline(always)]
    pub fn adcsel(&self) -> ADCSEL_R {
        ADCSEL_R::new(((self.bits >> 28) & 3) as u8)
    }
    ///Bits 30:31 - RNG clock source selection
    #[inline(always)]
    pub fn rngsel(&self) -> RNGSEL_R {
        RNGSEL_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCIPR")
            .field("rngsel", &self.rngsel())
            .field("adcsel", &self.adcsel())
            .field("lptim1sel", &self.lptim1sel())
            .field("lptim3sel", &self.lptim3sel())
            .field("lptim2sel", &self.lptim2sel())
            .field("i2c1sel", &self.i2c1sel())
            .field("i2c3sel", &self.i2c3sel())
            .field("i2c2sel", &self.i2c2sel())
            .field("lpuart1sel", &self.lpuart1sel())
            .field("spi2s2sel", &self.spi2s2sel())
            .field("usart1sel", &self.usart1sel())
            .field("usart2sel", &self.usart2sel())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - USART1 clock source selection
    #[inline(always)]
    pub fn usart1sel(&mut self) -> USART1SEL_W<'_, CCIPRrs> {
        USART1SEL_W::new(self, 0)
    }
    ///Bits 2:3 - USART2 clock source selection
    #[inline(always)]
    pub fn usart2sel(&mut self) -> USART2SEL_W<'_, CCIPRrs> {
        USART2SEL_W::new(self, 2)
    }
    ///Bits 8:9 - SPI2S2 I2S clock source selection
    #[inline(always)]
    pub fn spi2s2sel(&mut self) -> SPI2S2SEL_W<'_, CCIPRrs> {
        SPI2S2SEL_W::new(self, 8)
    }
    ///Bits 10:11 - LPUART1 clock source selection
    #[inline(always)]
    pub fn lpuart1sel(&mut self) -> LPUART1SEL_W<'_, CCIPRrs> {
        LPUART1SEL_W::new(self, 10)
    }
    ///Bits 12:13 - I2C1 clock source selection
    #[inline(always)]
    pub fn i2c1sel(&mut self) -> I2C1SEL_W<'_, CCIPRrs> {
        I2C1SEL_W::new(self, 12)
    }
    ///Bits 14:15 - I2C2 clock source selection
    #[inline(always)]
    pub fn i2c2sel(&mut self) -> I2C2SEL_W<'_, CCIPRrs> {
        I2C2SEL_W::new(self, 14)
    }
    ///Bits 16:17 - I2C3 clock source selection
    #[inline(always)]
    pub fn i2c3sel(&mut self) -> I2C3SEL_W<'_, CCIPRrs> {
        I2C3SEL_W::new(self, 16)
    }
    ///Bits 18:19 - Low power timer 1 clock source selection
    #[inline(always)]
    pub fn lptim1sel(&mut self) -> LPTIM1SEL_W<'_, CCIPRrs> {
        LPTIM1SEL_W::new(self, 18)
    }
    ///Bits 20:21 - Low power timer 2 clock source selection
    #[inline(always)]
    pub fn lptim2sel(&mut self) -> LPTIM2SEL_W<'_, CCIPRrs> {
        LPTIM2SEL_W::new(self, 20)
    }
    ///Bits 22:23 - Low power timer 3 clock source selection
    #[inline(always)]
    pub fn lptim3sel(&mut self) -> LPTIM3SEL_W<'_, CCIPRrs> {
        LPTIM3SEL_W::new(self, 22)
    }
    ///Bits 28:29 - ADC clock source selection
    #[inline(always)]
    pub fn adcsel(&mut self) -> ADCSEL_W<'_, CCIPRrs> {
        ADCSEL_W::new(self, 28)
    }
    ///Bits 30:31 - RNG clock source selection
    #[inline(always)]
    pub fn rngsel(&mut self) -> RNGSEL_W<'_, CCIPRrs> {
        RNGSEL_W::new(self, 30)
    }
}
/**Peripherals independent clock configuration register

You can [`read`](crate::Reg::read) this register and get [`ccipr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#RCC:CCIPR)*/
pub struct CCIPRrs;
impl crate::RegisterSpec for CCIPRrs {
    type Ux = u32;
}
///`read()` method returns [`ccipr::R`](R) reader structure
impl crate::Readable for CCIPRrs {}
///`write(|w| ..)` method takes [`ccipr::W`](W) writer structure
impl crate::Writable for CCIPRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCIPR to value 0
impl crate::Resettable for CCIPRrs {}
