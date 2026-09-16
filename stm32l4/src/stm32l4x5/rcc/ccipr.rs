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
///Field `USART3SEL` reader - USART3 clock source selection
pub use USART1SEL_R as USART3SEL_R;
///Field `USART2SEL` writer - USART2 clock source selection
pub use USART1SEL_W as USART2SEL_W;
///Field `USART3SEL` writer - USART3 clock source selection
pub use USART1SEL_W as USART3SEL_W;
///Field `UART4SEL` reader - UART4 clock source selection
pub type UART4SEL_R = crate::FieldReader;
///Field `UART4SEL` writer - UART4 clock source selection
pub type UART4SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `UART5SEL` reader - UART5 clock source selection
pub type UART5SEL_R = crate::FieldReader;
///Field `UART5SEL` writer - UART5 clock source selection
pub type UART5SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
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
///Field `LPTIM2SEL` writer - Low power timer 2 clock source selection
pub use LPTIM1SEL_W as LPTIM2SEL_W;
///Field `SAI1SEL` reader - SAI1 clock source selection
pub type SAI1SEL_R = crate::FieldReader;
///Field `SAI1SEL` writer - SAI1 clock source selection
pub type SAI1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SAI2SEL` reader - SAI2 clock source selection
pub type SAI2SEL_R = crate::FieldReader;
///Field `SAI2SEL` writer - SAI2 clock source selection
pub type SAI2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
/**48 MHz clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLK48SEL {
    ///0: HSI48 clock selected (only for STM32L41x/L42x/L43x/L44x/L45x/L46x/L49x/L4Ax devices, otherwise no clock selected)
    Hsi48 = 0,
    ///1: PLLSAI1 clock selected
    Pllsai1 = 1,
    ///2: PLL clock selected
    Pll = 2,
    ///3: MSI clock selected
    Msi = 3,
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
    pub const fn variant(&self) -> CLK48SEL {
        match self.bits {
            0 => CLK48SEL::Hsi48,
            1 => CLK48SEL::Pllsai1,
            2 => CLK48SEL::Pll,
            3 => CLK48SEL::Msi,
            _ => unreachable!(),
        }
    }
    ///HSI48 clock selected (only for STM32L41x/L42x/L43x/L44x/L45x/L46x/L49x/L4Ax devices, otherwise no clock selected)
    #[inline(always)]
    pub fn is_hsi48(&self) -> bool {
        *self == CLK48SEL::Hsi48
    }
    ///PLLSAI1 clock selected
    #[inline(always)]
    pub fn is_pllsai1(&self) -> bool {
        *self == CLK48SEL::Pllsai1
    }
    ///PLL clock selected
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == CLK48SEL::Pll
    }
    ///MSI clock selected
    #[inline(always)]
    pub fn is_msi(&self) -> bool {
        *self == CLK48SEL::Msi
    }
}
///Field `CLK48SEL` writer - 48 MHz clock source selection
pub type CLK48SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CLK48SEL, crate::Safe>;
impl<'a, REG> CLK48SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///HSI48 clock selected (only for STM32L41x/L42x/L43x/L44x/L45x/L46x/L49x/L4Ax devices, otherwise no clock selected)
    #[inline(always)]
    pub fn hsi48(self) -> &'a mut crate::W<REG> {
        self.variant(CLK48SEL::Hsi48)
    }
    ///PLLSAI1 clock selected
    #[inline(always)]
    pub fn pllsai1(self) -> &'a mut crate::W<REG> {
        self.variant(CLK48SEL::Pllsai1)
    }
    ///PLL clock selected
    #[inline(always)]
    pub fn pll(self) -> &'a mut crate::W<REG> {
        self.variant(CLK48SEL::Pll)
    }
    ///MSI clock selected
    #[inline(always)]
    pub fn msi(self) -> &'a mut crate::W<REG> {
        self.variant(CLK48SEL::Msi)
    }
}
/**ADCs clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADCSEL {
    ///0: No clock selected
    NoClock = 0,
    ///1: PLLSAI1 clock selected
    Pllsai1 = 1,
    ///2: PLLSAI2 clock selected (only for STM32L47x/L48x/L49x/L4Ax devices)
    Pllsai2 = 2,
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
///Field `ADCSEL` reader - ADCs clock source selection
pub type ADCSEL_R = crate::FieldReader<ADCSEL>;
impl ADCSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADCSEL {
        match self.bits {
            0 => ADCSEL::NoClock,
            1 => ADCSEL::Pllsai1,
            2 => ADCSEL::Pllsai2,
            3 => ADCSEL::Sysclk,
            _ => unreachable!(),
        }
    }
    ///No clock selected
    #[inline(always)]
    pub fn is_no_clock(&self) -> bool {
        *self == ADCSEL::NoClock
    }
    ///PLLSAI1 clock selected
    #[inline(always)]
    pub fn is_pllsai1(&self) -> bool {
        *self == ADCSEL::Pllsai1
    }
    ///PLLSAI2 clock selected (only for STM32L47x/L48x/L49x/L4Ax devices)
    #[inline(always)]
    pub fn is_pllsai2(&self) -> bool {
        *self == ADCSEL::Pllsai2
    }
    ///SYSCLK clock selected
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == ADCSEL::Sysclk
    }
}
///Field `ADCSEL` writer - ADCs clock source selection
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
    ///PLLSAI1 clock selected
    #[inline(always)]
    pub fn pllsai1(self) -> &'a mut crate::W<REG> {
        self.variant(ADCSEL::Pllsai1)
    }
    ///PLLSAI2 clock selected (only for STM32L47x/L48x/L49x/L4Ax devices)
    #[inline(always)]
    pub fn pllsai2(self) -> &'a mut crate::W<REG> {
        self.variant(ADCSEL::Pllsai2)
    }
    ///SYSCLK clock selected
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut crate::W<REG> {
        self.variant(ADCSEL::Sysclk)
    }
}
///Field `SWPMI1SEL` reader - SWPMI1 clock source selection
pub type SWPMI1SEL_R = crate::BitReader;
///Field `SWPMI1SEL` writer - SWPMI1 clock source selection
pub type SWPMI1SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DFSDMSEL` reader - DFSDM clock source selection
pub type DFSDMSEL_R = crate::BitReader;
///Field `DFSDMSEL` writer - DFSDM clock source selection
pub type DFSDMSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    ///Bits 20:21 - Low power timer 2 clock source selection
    #[inline(always)]
    pub fn lptim2sel(&self) -> LPTIM2SEL_R {
        LPTIM2SEL_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bits 22:23 - SAI1 clock source selection
    #[inline(always)]
    pub fn sai1sel(&self) -> SAI1SEL_R {
        SAI1SEL_R::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bits 24:25 - SAI2 clock source selection
    #[inline(always)]
    pub fn sai2sel(&self) -> SAI2SEL_R {
        SAI2SEL_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bits 26:27 - 48 MHz clock source selection
    #[inline(always)]
    pub fn clk48sel(&self) -> CLK48SEL_R {
        CLK48SEL_R::new(((self.bits >> 26) & 3) as u8)
    }
    ///Bits 28:29 - ADCs clock source selection
    #[inline(always)]
    pub fn adcsel(&self) -> ADCSEL_R {
        ADCSEL_R::new(((self.bits >> 28) & 3) as u8)
    }
    ///Bit 30 - SWPMI1 clock source selection
    #[inline(always)]
    pub fn swpmi1sel(&self) -> SWPMI1SEL_R {
        SWPMI1SEL_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - DFSDM clock source selection
    #[inline(always)]
    pub fn dfsdmsel(&self) -> DFSDMSEL_R {
        DFSDMSEL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCIPR")
            .field("dfsdmsel", &self.dfsdmsel())
            .field("swpmi1sel", &self.swpmi1sel())
            .field("adcsel", &self.adcsel())
            .field("clk48sel", &self.clk48sel())
            .field("sai2sel", &self.sai2sel())
            .field("sai1sel", &self.sai1sel())
            .field("lptim1sel", &self.lptim1sel())
            .field("lptim2sel", &self.lptim2sel())
            .field("i2c1sel", &self.i2c1sel())
            .field("i2c3sel", &self.i2c3sel())
            .field("i2c2sel", &self.i2c2sel())
            .field("lpuart1sel", &self.lpuart1sel())
            .field("uart5sel", &self.uart5sel())
            .field("uart4sel", &self.uart4sel())
            .field("usart1sel", &self.usart1sel())
            .field("usart3sel", &self.usart3sel())
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
    ///Bits 20:21 - Low power timer 2 clock source selection
    #[inline(always)]
    pub fn lptim2sel(&mut self) -> LPTIM2SEL_W<'_, CCIPRrs> {
        LPTIM2SEL_W::new(self, 20)
    }
    ///Bits 22:23 - SAI1 clock source selection
    #[inline(always)]
    pub fn sai1sel(&mut self) -> SAI1SEL_W<'_, CCIPRrs> {
        SAI1SEL_W::new(self, 22)
    }
    ///Bits 24:25 - SAI2 clock source selection
    #[inline(always)]
    pub fn sai2sel(&mut self) -> SAI2SEL_W<'_, CCIPRrs> {
        SAI2SEL_W::new(self, 24)
    }
    ///Bits 26:27 - 48 MHz clock source selection
    #[inline(always)]
    pub fn clk48sel(&mut self) -> CLK48SEL_W<'_, CCIPRrs> {
        CLK48SEL_W::new(self, 26)
    }
    ///Bits 28:29 - ADCs clock source selection
    #[inline(always)]
    pub fn adcsel(&mut self) -> ADCSEL_W<'_, CCIPRrs> {
        ADCSEL_W::new(self, 28)
    }
    ///Bit 30 - SWPMI1 clock source selection
    #[inline(always)]
    pub fn swpmi1sel(&mut self) -> SWPMI1SEL_W<'_, CCIPRrs> {
        SWPMI1SEL_W::new(self, 30)
    }
    ///Bit 31 - DFSDM clock source selection
    #[inline(always)]
    pub fn dfsdmsel(&mut self) -> DFSDMSEL_W<'_, CCIPRrs> {
        DFSDMSEL_W::new(self, 31)
    }
}
/**CCIPR

You can [`read`](crate::Reg::read) this register and get [`ccipr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x5.html#RCC:CCIPR)*/
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
