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
/**HDMI CEC clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CECSEL {
    ///0: HSI16 clock divided by 488 selected
    Hsi16 = 0,
    ///1: LSE clock selected
    Lse = 1,
}
impl From<CECSEL> for bool {
    #[inline(always)]
    fn from(variant: CECSEL) -> Self {
        variant as u8 != 0
    }
}
///Field `CECSEL` reader - HDMI CEC clock source selection
pub type CECSEL_R = crate::BitReader<CECSEL>;
impl CECSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CECSEL {
        match self.bits {
            false => CECSEL::Hsi16,
            true => CECSEL::Lse,
        }
    }
    ///HSI16 clock divided by 488 selected
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == CECSEL::Hsi16
    }
    ///LSE clock selected
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == CECSEL::Lse
    }
}
///Field `CECSEL` writer - HDMI CEC clock source selection
pub type CECSEL_W<'a, REG> = crate::BitWriter<'a, REG, CECSEL>;
impl<'a, REG> CECSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///HSI16 clock divided by 488 selected
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut crate::W<REG> {
        self.variant(CECSEL::Hsi16)
    }
    ///LSE clock selected
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(CECSEL::Lse)
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
/**I2S1 clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2S1SEL {
    ///0: SYSCLK clock selected
    Sysclk = 0,
    ///1: PLLPCLK clock selected
    Pllp = 1,
    ///2: HSI16 clock selected
    Hsi16 = 2,
    ///3: I2S_CKIN clock selected
    Ckin = 3,
}
impl From<I2S1SEL> for u8 {
    #[inline(always)]
    fn from(variant: I2S1SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for I2S1SEL {
    type Ux = u8;
}
impl crate::IsEnum for I2S1SEL {}
///Field `I2S1SEL` reader - I2S1 clock source selection
pub type I2S1SEL_R = crate::FieldReader<I2S1SEL>;
impl I2S1SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> I2S1SEL {
        match self.bits {
            0 => I2S1SEL::Sysclk,
            1 => I2S1SEL::Pllp,
            2 => I2S1SEL::Hsi16,
            3 => I2S1SEL::Ckin,
            _ => unreachable!(),
        }
    }
    ///SYSCLK clock selected
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == I2S1SEL::Sysclk
    }
    ///PLLPCLK clock selected
    #[inline(always)]
    pub fn is_pllp(&self) -> bool {
        *self == I2S1SEL::Pllp
    }
    ///HSI16 clock selected
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == I2S1SEL::Hsi16
    }
    ///I2S_CKIN clock selected
    #[inline(always)]
    pub fn is_ckin(&self) -> bool {
        *self == I2S1SEL::Ckin
    }
}
///Field `I2S1SEL` writer - I2S1 clock source selection
pub type I2S1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, I2S1SEL, crate::Safe>;
impl<'a, REG> I2S1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///SYSCLK clock selected
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut crate::W<REG> {
        self.variant(I2S1SEL::Sysclk)
    }
    ///PLLPCLK clock selected
    #[inline(always)]
    pub fn pllp(self) -> &'a mut crate::W<REG> {
        self.variant(I2S1SEL::Pllp)
    }
    ///HSI16 clock selected
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut crate::W<REG> {
        self.variant(I2S1SEL::Hsi16)
    }
    ///I2S_CKIN clock selected
    #[inline(always)]
    pub fn ckin(self) -> &'a mut crate::W<REG> {
        self.variant(I2S1SEL::Ckin)
    }
}
/**LPTIM1 clock source selection

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
///Field `LPTIM1SEL` reader - LPTIM1 clock source selection
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
///Field `LPTIM1SEL` writer - LPTIM1 clock source selection
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
///Field `LPTIM2SEL` reader - LPTIM2 clock source selection
pub use LPTIM1SEL_R as LPTIM2SEL_R;
///Field `LPTIM2SEL` writer - LPTIM2 clock source selection
pub use LPTIM1SEL_W as LPTIM2SEL_W;
/**TIM1 clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM1SEL {
    ///0: TIMPCLK clock selected
    Timp = 0,
    ///1: PLLQCLK clock selected
    Pllq = 1,
}
impl From<TIM1SEL> for bool {
    #[inline(always)]
    fn from(variant: TIM1SEL) -> Self {
        variant as u8 != 0
    }
}
///Field `TIM1SEL` reader - TIM1 clock source selection
pub type TIM1SEL_R = crate::BitReader<TIM1SEL>;
impl TIM1SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIM1SEL {
        match self.bits {
            false => TIM1SEL::Timp,
            true => TIM1SEL::Pllq,
        }
    }
    ///TIMPCLK clock selected
    #[inline(always)]
    pub fn is_timp(&self) -> bool {
        *self == TIM1SEL::Timp
    }
    ///PLLQCLK clock selected
    #[inline(always)]
    pub fn is_pllq(&self) -> bool {
        *self == TIM1SEL::Pllq
    }
}
///Field `TIM1SEL` writer - TIM1 clock source selection
pub type TIM1SEL_W<'a, REG> = crate::BitWriter<'a, REG, TIM1SEL>;
impl<'a, REG> TIM1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///TIMPCLK clock selected
    #[inline(always)]
    pub fn timp(self) -> &'a mut crate::W<REG> {
        self.variant(TIM1SEL::Timp)
    }
    ///PLLQCLK clock selected
    #[inline(always)]
    pub fn pllq(self) -> &'a mut crate::W<REG> {
        self.variant(TIM1SEL::Pllq)
    }
}
///Field `TIM15SEL` reader - TIM15 clock source selection
pub use TIM1SEL_R as TIM15SEL_R;
///Field `TIM15SEL` writer - TIM15 clock source selection
pub use TIM1SEL_W as TIM15SEL_W;
/**ADCs clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADCSEL {
    ///0: System clock selected
    Sysclk = 0,
    ///1: PLLPCLK clock selected
    Pllp = 1,
    ///2: HSI16 clock selected
    Hsi16 = 2,
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
    pub const fn variant(&self) -> Option<ADCSEL> {
        match self.bits {
            0 => Some(ADCSEL::Sysclk),
            1 => Some(ADCSEL::Pllp),
            2 => Some(ADCSEL::Hsi16),
            _ => None,
        }
    }
    ///System clock selected
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == ADCSEL::Sysclk
    }
    ///PLLPCLK clock selected
    #[inline(always)]
    pub fn is_pllp(&self) -> bool {
        *self == ADCSEL::Pllp
    }
    ///HSI16 clock selected
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == ADCSEL::Hsi16
    }
}
///Field `ADCSEL` writer - ADCs clock source selection
pub type ADCSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, ADCSEL>;
impl<'a, REG> ADCSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///System clock selected
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut crate::W<REG> {
        self.variant(ADCSEL::Sysclk)
    }
    ///PLLPCLK clock selected
    #[inline(always)]
    pub fn pllp(self) -> &'a mut crate::W<REG> {
        self.variant(ADCSEL::Pllp)
    }
    ///HSI16 clock selected
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut crate::W<REG> {
        self.variant(ADCSEL::Hsi16)
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
    ///Bit 6 - HDMI CEC clock source selection
    #[inline(always)]
    pub fn cecsel(&self) -> CECSEL_R {
        CECSEL_R::new(((self.bits >> 6) & 1) != 0)
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
    ///Bits 14:15 - I2S1 clock source selection
    #[inline(always)]
    pub fn i2s1sel(&self) -> I2S1SEL_R {
        I2S1SEL_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bits 18:19 - LPTIM1 clock source selection
    #[inline(always)]
    pub fn lptim1sel(&self) -> LPTIM1SEL_R {
        LPTIM1SEL_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bits 20:21 - LPTIM2 clock source selection
    #[inline(always)]
    pub fn lptim2sel(&self) -> LPTIM2SEL_R {
        LPTIM2SEL_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bit 22 - TIM1 clock source selection
    #[inline(always)]
    pub fn tim1sel(&self) -> TIM1SEL_R {
        TIM1SEL_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 24 - TIM15 clock source selection
    #[inline(always)]
    pub fn tim15sel(&self) -> TIM15SEL_R {
        TIM15SEL_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bits 30:31 - ADCs clock source selection
    #[inline(always)]
    pub fn adcsel(&self) -> ADCSEL_R {
        ADCSEL_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCIPR")
            .field("usart1sel", &self.usart1sel())
            .field("usart2sel", &self.usart2sel())
            .field("cecsel", &self.cecsel())
            .field("lpuart1sel", &self.lpuart1sel())
            .field("i2c1sel", &self.i2c1sel())
            .field("i2s1sel", &self.i2s1sel())
            .field("lptim1sel", &self.lptim1sel())
            .field("lptim2sel", &self.lptim2sel())
            .field("tim1sel", &self.tim1sel())
            .field("tim15sel", &self.tim15sel())
            .field("adcsel", &self.adcsel())
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
    ///Bit 6 - HDMI CEC clock source selection
    #[inline(always)]
    pub fn cecsel(&mut self) -> CECSEL_W<'_, CCIPRrs> {
        CECSEL_W::new(self, 6)
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
    ///Bits 14:15 - I2S1 clock source selection
    #[inline(always)]
    pub fn i2s1sel(&mut self) -> I2S1SEL_W<'_, CCIPRrs> {
        I2S1SEL_W::new(self, 14)
    }
    ///Bits 18:19 - LPTIM1 clock source selection
    #[inline(always)]
    pub fn lptim1sel(&mut self) -> LPTIM1SEL_W<'_, CCIPRrs> {
        LPTIM1SEL_W::new(self, 18)
    }
    ///Bits 20:21 - LPTIM2 clock source selection
    #[inline(always)]
    pub fn lptim2sel(&mut self) -> LPTIM2SEL_W<'_, CCIPRrs> {
        LPTIM2SEL_W::new(self, 20)
    }
    ///Bit 22 - TIM1 clock source selection
    #[inline(always)]
    pub fn tim1sel(&mut self) -> TIM1SEL_W<'_, CCIPRrs> {
        TIM1SEL_W::new(self, 22)
    }
    ///Bit 24 - TIM15 clock source selection
    #[inline(always)]
    pub fn tim15sel(&mut self) -> TIM15SEL_W<'_, CCIPRrs> {
        TIM15SEL_W::new(self, 24)
    }
    ///Bits 30:31 - ADCs clock source selection
    #[inline(always)]
    pub fn adcsel(&mut self) -> ADCSEL_W<'_, CCIPRrs> {
        ADCSEL_W::new(self, 30)
    }
}
/**Peripherals independent clock configuration register

You can [`read`](crate::Reg::read) this register and get [`ccipr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G071.html#RCC:CCIPR)*/
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
