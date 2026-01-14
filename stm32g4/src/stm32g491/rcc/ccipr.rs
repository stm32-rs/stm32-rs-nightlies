///Register `CCIPR` reader
pub type R = crate::R<CCIPRrs>;
///Register `CCIPR` writer
pub type W = crate::W<CCIPRrs>;
///Field `USART1SEL` reader - USART1 clock source selection
pub type USART1SEL_R = crate::FieldReader;
///Field `USART1SEL` writer - USART1 clock source selection
pub type USART1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `USART2SEL` reader - USART2 clock source selection
pub type USART2SEL_R = crate::FieldReader;
///Field `USART2SEL` writer - USART2 clock source selection
pub type USART2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `USART3SEL` reader - USART3 clock source selection
pub type USART3SEL_R = crate::FieldReader;
///Field `USART3SEL` writer - USART3 clock source selection
pub type USART3SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
/**UART4 clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UART4SEL {
    ///0: PCLK clock selected as UART clock
    Pclk = 0,
    ///1: System clock (SYSCLK) selected as UART clock
    System = 1,
    ///2: HSI16 clock selected as UART clock
    Hsi16 = 2,
    ///3: LSE clock selected as UART clock
    Lse = 3,
}
impl From<UART4SEL> for u8 {
    #[inline(always)]
    fn from(variant: UART4SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for UART4SEL {
    type Ux = u8;
}
impl crate::IsEnum for UART4SEL {}
///Field `UART4SEL` reader - UART4 clock source selection
pub type UART4SEL_R = crate::FieldReader<UART4SEL>;
impl UART4SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> UART4SEL {
        match self.bits {
            0 => UART4SEL::Pclk,
            1 => UART4SEL::System,
            2 => UART4SEL::Hsi16,
            3 => UART4SEL::Lse,
            _ => unreachable!(),
        }
    }
    ///PCLK clock selected as UART clock
    #[inline(always)]
    pub fn is_pclk(&self) -> bool {
        *self == UART4SEL::Pclk
    }
    ///System clock (SYSCLK) selected as UART clock
    #[inline(always)]
    pub fn is_system(&self) -> bool {
        *self == UART4SEL::System
    }
    ///HSI16 clock selected as UART clock
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == UART4SEL::Hsi16
    }
    ///LSE clock selected as UART clock
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == UART4SEL::Lse
    }
}
///Field `UART4SEL` writer - UART4 clock source selection
pub type UART4SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, UART4SEL, crate::Safe>;
impl<'a, REG> UART4SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PCLK clock selected as UART clock
    #[inline(always)]
    pub fn pclk(self) -> &'a mut crate::W<REG> {
        self.variant(UART4SEL::Pclk)
    }
    ///System clock (SYSCLK) selected as UART clock
    #[inline(always)]
    pub fn system(self) -> &'a mut crate::W<REG> {
        self.variant(UART4SEL::System)
    }
    ///HSI16 clock selected as UART clock
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut crate::W<REG> {
        self.variant(UART4SEL::Hsi16)
    }
    ///LSE clock selected as UART clock
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(UART4SEL::Lse)
    }
}
///Field `UART5SEL` reader - UART5 clock source selection
pub use UART4SEL_R as UART5SEL_R;
///Field `LPUART1SEL` reader - LPUART1 clock source selection
pub use UART4SEL_R as LPUART1SEL_R;
///Field `UART5SEL` writer - UART5 clock source selection
pub use UART4SEL_W as UART5SEL_W;
///Field `LPUART1SEL` writer - LPUART1 clock source selection
pub use UART4SEL_W as LPUART1SEL_W;
/**I2C1 clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2C1SEL {
    ///0: PCLK clock selected as I2C clock
    Pclk = 0,
    ///1: System clock (SYSCLK) selected as I2C clock
    System = 1,
    ///2: HSI16 clock selected as I2C clock
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
            1 => Some(I2C1SEL::System),
            2 => Some(I2C1SEL::Hsi16),
            _ => None,
        }
    }
    ///PCLK clock selected as I2C clock
    #[inline(always)]
    pub fn is_pclk(&self) -> bool {
        *self == I2C1SEL::Pclk
    }
    ///System clock (SYSCLK) selected as I2C clock
    #[inline(always)]
    pub fn is_system(&self) -> bool {
        *self == I2C1SEL::System
    }
    ///HSI16 clock selected as I2C clock
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
    ///PCLK clock selected as I2C clock
    #[inline(always)]
    pub fn pclk(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1SEL::Pclk)
    }
    ///System clock (SYSCLK) selected as I2C clock
    #[inline(always)]
    pub fn system(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1SEL::System)
    }
    ///HSI16 clock selected as I2C clock
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
    ///0: PCLK clock selected as LPTIM1 clock
    Pclk = 0,
    ///1: LSI clock selected as LPTIM1 clock
    Lsi = 1,
    ///2: HSI16 clock selected as LPTIM1 clock
    Hsi16 = 2,
    ///3: LSE clock selected as LPTIM1 clock
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
    ///PCLK clock selected as LPTIM1 clock
    #[inline(always)]
    pub fn is_pclk(&self) -> bool {
        *self == LPTIM1SEL::Pclk
    }
    ///LSI clock selected as LPTIM1 clock
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == LPTIM1SEL::Lsi
    }
    ///HSI16 clock selected as LPTIM1 clock
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == LPTIM1SEL::Hsi16
    }
    ///LSE clock selected as LPTIM1 clock
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
    ///PCLK clock selected as LPTIM1 clock
    #[inline(always)]
    pub fn pclk(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM1SEL::Pclk)
    }
    ///LSI clock selected as LPTIM1 clock
    #[inline(always)]
    pub fn lsi(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM1SEL::Lsi)
    }
    ///HSI16 clock selected as LPTIM1 clock
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM1SEL::Hsi16)
    }
    ///LSE clock selected as LPTIM1 clock
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM1SEL::Lse)
    }
}
/**clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SAI1SEL {
    ///0: System clock selected as SAI clock
    System = 0,
    ///1: PLL 'Q' clock selected as SAI clock
    Pllq = 1,
    ///2: Clock provided on I2S_CKIN pin is selected as SAI clock
    I2sCkin = 2,
    ///3: HSI16 clock selected as SAI clock
    Hsi16 = 3,
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
impl crate::IsEnum for SAI1SEL {}
///Field `SAI1SEL` reader - clock source selection
pub type SAI1SEL_R = crate::FieldReader<SAI1SEL>;
impl SAI1SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SAI1SEL {
        match self.bits {
            0 => SAI1SEL::System,
            1 => SAI1SEL::Pllq,
            2 => SAI1SEL::I2sCkin,
            3 => SAI1SEL::Hsi16,
            _ => unreachable!(),
        }
    }
    ///System clock selected as SAI clock
    #[inline(always)]
    pub fn is_system(&self) -> bool {
        *self == SAI1SEL::System
    }
    ///PLL 'Q' clock selected as SAI clock
    #[inline(always)]
    pub fn is_pllq(&self) -> bool {
        *self == SAI1SEL::Pllq
    }
    ///Clock provided on I2S_CKIN pin is selected as SAI clock
    #[inline(always)]
    pub fn is_i2s_ckin(&self) -> bool {
        *self == SAI1SEL::I2sCkin
    }
    ///HSI16 clock selected as SAI clock
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == SAI1SEL::Hsi16
    }
}
///Field `SAI1SEL` writer - clock source selection
pub type SAI1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SAI1SEL, crate::Safe>;
impl<'a, REG> SAI1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///System clock selected as SAI clock
    #[inline(always)]
    pub fn system(self) -> &'a mut crate::W<REG> {
        self.variant(SAI1SEL::System)
    }
    ///PLL 'Q' clock selected as SAI clock
    #[inline(always)]
    pub fn pllq(self) -> &'a mut crate::W<REG> {
        self.variant(SAI1SEL::Pllq)
    }
    ///Clock provided on I2S_CKIN pin is selected as SAI clock
    #[inline(always)]
    pub fn i2s_ckin(self) -> &'a mut crate::W<REG> {
        self.variant(SAI1SEL::I2sCkin)
    }
    ///HSI16 clock selected as SAI clock
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut crate::W<REG> {
        self.variant(SAI1SEL::Hsi16)
    }
}
/**clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2S23SEL {
    ///0: System clock selected as I2S23 clock
    System = 0,
    ///1: PLL 'Q' clock selected as I2S23 clock
    Pllq = 1,
    ///2: Clock provided on I2S_CKIN pin is selected as I2S23 clock
    I2sCkin = 2,
    ///3: HSI16 clock selected as I2S23 clock
    Hsi16 = 3,
}
impl From<I2S23SEL> for u8 {
    #[inline(always)]
    fn from(variant: I2S23SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for I2S23SEL {
    type Ux = u8;
}
impl crate::IsEnum for I2S23SEL {}
///Field `I2S23SEL` reader - clock source selection
pub type I2S23SEL_R = crate::FieldReader<I2S23SEL>;
impl I2S23SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> I2S23SEL {
        match self.bits {
            0 => I2S23SEL::System,
            1 => I2S23SEL::Pllq,
            2 => I2S23SEL::I2sCkin,
            3 => I2S23SEL::Hsi16,
            _ => unreachable!(),
        }
    }
    ///System clock selected as I2S23 clock
    #[inline(always)]
    pub fn is_system(&self) -> bool {
        *self == I2S23SEL::System
    }
    ///PLL 'Q' clock selected as I2S23 clock
    #[inline(always)]
    pub fn is_pllq(&self) -> bool {
        *self == I2S23SEL::Pllq
    }
    ///Clock provided on I2S_CKIN pin is selected as I2S23 clock
    #[inline(always)]
    pub fn is_i2s_ckin(&self) -> bool {
        *self == I2S23SEL::I2sCkin
    }
    ///HSI16 clock selected as I2S23 clock
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == I2S23SEL::Hsi16
    }
}
///Field `I2S23SEL` writer - clock source selection
pub type I2S23SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, I2S23SEL, crate::Safe>;
impl<'a, REG> I2S23SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///System clock selected as I2S23 clock
    #[inline(always)]
    pub fn system(self) -> &'a mut crate::W<REG> {
        self.variant(I2S23SEL::System)
    }
    ///PLL 'Q' clock selected as I2S23 clock
    #[inline(always)]
    pub fn pllq(self) -> &'a mut crate::W<REG> {
        self.variant(I2S23SEL::Pllq)
    }
    ///Clock provided on I2S_CKIN pin is selected as I2S23 clock
    #[inline(always)]
    pub fn i2s_ckin(self) -> &'a mut crate::W<REG> {
        self.variant(I2S23SEL::I2sCkin)
    }
    ///HSI16 clock selected as I2S23 clock
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut crate::W<REG> {
        self.variant(I2S23SEL::Hsi16)
    }
}
/**None

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FDCANSEL {
    ///0: HSE clock selected as FDCAN clock
    Hse = 0,
    ///1: PLL 'Q' clock selected as FDCAN clock
    Pllq = 1,
    ///2: PCLK clock selected as FDCAN clock
    Pclk = 2,
}
impl From<FDCANSEL> for u8 {
    #[inline(always)]
    fn from(variant: FDCANSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FDCANSEL {
    type Ux = u8;
}
impl crate::IsEnum for FDCANSEL {}
///Field `FDCANSEL` reader - None
pub type FDCANSEL_R = crate::FieldReader<FDCANSEL>;
impl FDCANSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<FDCANSEL> {
        match self.bits {
            0 => Some(FDCANSEL::Hse),
            1 => Some(FDCANSEL::Pllq),
            2 => Some(FDCANSEL::Pclk),
            _ => None,
        }
    }
    ///HSE clock selected as FDCAN clock
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == FDCANSEL::Hse
    }
    ///PLL 'Q' clock selected as FDCAN clock
    #[inline(always)]
    pub fn is_pllq(&self) -> bool {
        *self == FDCANSEL::Pllq
    }
    ///PCLK clock selected as FDCAN clock
    #[inline(always)]
    pub fn is_pclk(&self) -> bool {
        *self == FDCANSEL::Pclk
    }
}
///Field `FDCANSEL` writer - None
pub type FDCANSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, FDCANSEL>;
impl<'a, REG> FDCANSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///HSE clock selected as FDCAN clock
    #[inline(always)]
    pub fn hse(self) -> &'a mut crate::W<REG> {
        self.variant(FDCANSEL::Hse)
    }
    ///PLL 'Q' clock selected as FDCAN clock
    #[inline(always)]
    pub fn pllq(self) -> &'a mut crate::W<REG> {
        self.variant(FDCANSEL::Pllq)
    }
    ///PCLK clock selected as FDCAN clock
    #[inline(always)]
    pub fn pclk(self) -> &'a mut crate::W<REG> {
        self.variant(FDCANSEL::Pclk)
    }
}
/**48 MHz clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLK48SEL {
    ///0: HSI48 clock selected as 48MHz clock
    Hsi48 = 0,
    ///2: PLL 'Q' (PLL48M1CLK) clock selected as 48MHz clock
    Pllq = 2,
}
impl From<CLK48SEL> for u8 {
    #[inline(always)]
    fn from(variant: CLK48SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CLK48SEL {
    type Ux = u8;
}
impl crate::IsEnum for CLK48SEL {}
///Field `CLK48SEL` reader - 48 MHz clock source selection
pub type CLK48SEL_R = crate::FieldReader<CLK48SEL>;
impl CLK48SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CLK48SEL> {
        match self.bits {
            0 => Some(CLK48SEL::Hsi48),
            2 => Some(CLK48SEL::Pllq),
            _ => None,
        }
    }
    ///HSI48 clock selected as 48MHz clock
    #[inline(always)]
    pub fn is_hsi48(&self) -> bool {
        *self == CLK48SEL::Hsi48
    }
    ///PLL 'Q' (PLL48M1CLK) clock selected as 48MHz clock
    #[inline(always)]
    pub fn is_pllq(&self) -> bool {
        *self == CLK48SEL::Pllq
    }
}
///Field `CLK48SEL` writer - 48 MHz clock source selection
pub type CLK48SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CLK48SEL>;
impl<'a, REG> CLK48SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///HSI48 clock selected as 48MHz clock
    #[inline(always)]
    pub fn hsi48(self) -> &'a mut crate::W<REG> {
        self.variant(CLK48SEL::Hsi48)
    }
    ///PLL 'Q' (PLL48M1CLK) clock selected as 48MHz clock
    #[inline(always)]
    pub fn pllq(self) -> &'a mut crate::W<REG> {
        self.variant(CLK48SEL::Pllq)
    }
}
/**ADC1/2 clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADC12SEL {
    ///0: No clock selected for ADC
    None = 0,
    ///1: PLL 'P' clock selected for ADC
    Pllp = 1,
    ///2: System clock selected for ADC
    System = 2,
}
impl From<ADC12SEL> for u8 {
    #[inline(always)]
    fn from(variant: ADC12SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ADC12SEL {
    type Ux = u8;
}
impl crate::IsEnum for ADC12SEL {}
///Field `ADC12SEL` reader - ADC1/2 clock source selection
pub type ADC12SEL_R = crate::FieldReader<ADC12SEL>;
impl ADC12SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<ADC12SEL> {
        match self.bits {
            0 => Some(ADC12SEL::None),
            1 => Some(ADC12SEL::Pllp),
            2 => Some(ADC12SEL::System),
            _ => None,
        }
    }
    ///No clock selected for ADC
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == ADC12SEL::None
    }
    ///PLL 'P' clock selected for ADC
    #[inline(always)]
    pub fn is_pllp(&self) -> bool {
        *self == ADC12SEL::Pllp
    }
    ///System clock selected for ADC
    #[inline(always)]
    pub fn is_system(&self) -> bool {
        *self == ADC12SEL::System
    }
}
///Field `ADC12SEL` writer - ADC1/2 clock source selection
pub type ADC12SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, ADC12SEL>;
impl<'a, REG> ADC12SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No clock selected for ADC
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12SEL::None)
    }
    ///PLL 'P' clock selected for ADC
    #[inline(always)]
    pub fn pllp(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12SEL::Pllp)
    }
    ///System clock selected for ADC
    #[inline(always)]
    pub fn system(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12SEL::System)
    }
}
///Field `ADC345SEL` reader - ADC3/4/5 clock source selection
pub use ADC12SEL_R as ADC345SEL_R;
///Field `ADC345SEL` writer - ADC3/4/5 clock source selection
pub use ADC12SEL_W as ADC345SEL_W;
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
    ///Bits 4:5 - USART3 clock source selection
    #[inline(always)]
    pub fn usart3sel(&self) -> USART3SEL_R {
        USART3SEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 6:7 - UART4 clock source selection
    #[inline(always)]
    pub fn uart4sel(&self) -> UART4SEL_R {
        UART4SEL_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:9 - UART5 clock source selection
    #[inline(always)]
    pub fn uart5sel(&self) -> UART5SEL_R {
        UART5SEL_R::new(((self.bits >> 8) & 3) as u8)
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
    ///Bits 20:21 - clock source selection
    #[inline(always)]
    pub fn sai1sel(&self) -> SAI1SEL_R {
        SAI1SEL_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bits 22:23 - clock source selection
    #[inline(always)]
    pub fn i2s23sel(&self) -> I2S23SEL_R {
        I2S23SEL_R::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bits 24:25 - None
    #[inline(always)]
    pub fn fdcansel(&self) -> FDCANSEL_R {
        FDCANSEL_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bits 26:27 - 48 MHz clock source selection
    #[inline(always)]
    pub fn clk48sel(&self) -> CLK48SEL_R {
        CLK48SEL_R::new(((self.bits >> 26) & 3) as u8)
    }
    ///Bits 28:29 - ADC1/2 clock source selection
    #[inline(always)]
    pub fn adc12sel(&self) -> ADC12SEL_R {
        ADC12SEL_R::new(((self.bits >> 28) & 3) as u8)
    }
    ///Bits 30:31 - ADC3/4/5 clock source selection
    #[inline(always)]
    pub fn adc345sel(&self) -> ADC345SEL_R {
        ADC345SEL_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCIPR")
            .field("usart1sel", &self.usart1sel())
            .field("usart2sel", &self.usart2sel())
            .field("usart3sel", &self.usart3sel())
            .field("uart4sel", &self.uart4sel())
            .field("uart5sel", &self.uart5sel())
            .field("lpuart1sel", &self.lpuart1sel())
            .field("i2c1sel", &self.i2c1sel())
            .field("i2c2sel", &self.i2c2sel())
            .field("i2c3sel", &self.i2c3sel())
            .field("lptim1sel", &self.lptim1sel())
            .field("sai1sel", &self.sai1sel())
            .field("i2s23sel", &self.i2s23sel())
            .field("fdcansel", &self.fdcansel())
            .field("clk48sel", &self.clk48sel())
            .field("adc12sel", &self.adc12sel())
            .field("adc345sel", &self.adc345sel())
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
    ///Bits 4:5 - USART3 clock source selection
    #[inline(always)]
    pub fn usart3sel(&mut self) -> USART3SEL_W<'_, CCIPRrs> {
        USART3SEL_W::new(self, 4)
    }
    ///Bits 6:7 - UART4 clock source selection
    #[inline(always)]
    pub fn uart4sel(&mut self) -> UART4SEL_W<'_, CCIPRrs> {
        UART4SEL_W::new(self, 6)
    }
    ///Bits 8:9 - UART5 clock source selection
    #[inline(always)]
    pub fn uart5sel(&mut self) -> UART5SEL_W<'_, CCIPRrs> {
        UART5SEL_W::new(self, 8)
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
    ///Bits 20:21 - clock source selection
    #[inline(always)]
    pub fn sai1sel(&mut self) -> SAI1SEL_W<'_, CCIPRrs> {
        SAI1SEL_W::new(self, 20)
    }
    ///Bits 22:23 - clock source selection
    #[inline(always)]
    pub fn i2s23sel(&mut self) -> I2S23SEL_W<'_, CCIPRrs> {
        I2S23SEL_W::new(self, 22)
    }
    ///Bits 24:25 - None
    #[inline(always)]
    pub fn fdcansel(&mut self) -> FDCANSEL_W<'_, CCIPRrs> {
        FDCANSEL_W::new(self, 24)
    }
    ///Bits 26:27 - 48 MHz clock source selection
    #[inline(always)]
    pub fn clk48sel(&mut self) -> CLK48SEL_W<'_, CCIPRrs> {
        CLK48SEL_W::new(self, 26)
    }
    ///Bits 28:29 - ADC1/2 clock source selection
    #[inline(always)]
    pub fn adc12sel(&mut self) -> ADC12SEL_W<'_, CCIPRrs> {
        ADC12SEL_W::new(self, 28)
    }
    ///Bits 30:31 - ADC3/4/5 clock source selection
    #[inline(always)]
    pub fn adc345sel(&mut self) -> ADC345SEL_W<'_, CCIPRrs> {
        ADC345SEL_W::new(self, 30)
    }
}
/**Peripherals independent clock configuration register

You can [`read`](crate::Reg::read) this register and get [`ccipr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G491.html#RCC:CCIPR)*/
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
