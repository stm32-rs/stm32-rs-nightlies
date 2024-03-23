#[doc = "Register `DCKCFGR2` reader"]
pub type R = crate::R<DCKCFGR2rs>;
#[doc = "Register `DCKCFGR2` writer"]
pub type W = crate::W<DCKCFGR2rs>;
#[doc = "USART 1 clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USART1SEL {
    #[doc = "0: APB2 clock (PCLK2) is selected as USART clock"]
    Apb2 = 0,
    #[doc = "1: System clock is selected as USART clock"]
    Sysclk = 1,
    #[doc = "2: HSI clock is selected as USART clock"]
    Hsi = 2,
    #[doc = "3: LSE clock is selected as USART clock"]
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
#[doc = "Field `USART1SEL` reader - USART 1 clock source selection"]
pub type USART1SEL_R = crate::FieldReader<USART1SEL>;
impl USART1SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USART1SEL {
        match self.bits {
            0 => USART1SEL::Apb2,
            1 => USART1SEL::Sysclk,
            2 => USART1SEL::Hsi,
            3 => USART1SEL::Lse,
            _ => unreachable!(),
        }
    }
    #[doc = "APB2 clock (PCLK2) is selected as USART clock"]
    #[inline(always)]
    pub fn is_apb2(&self) -> bool {
        *self == USART1SEL::Apb2
    }
    #[doc = "System clock is selected as USART clock"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == USART1SEL::Sysclk
    }
    #[doc = "HSI clock is selected as USART clock"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == USART1SEL::Hsi
    }
    #[doc = "LSE clock is selected as USART clock"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == USART1SEL::Lse
    }
}
#[doc = "Field `USART1SEL` writer - USART 1 clock source selection"]
pub type USART1SEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, USART1SEL>;
impl<'a, REG> USART1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "APB2 clock (PCLK2) is selected as USART clock"]
    #[inline(always)]
    pub fn apb2(self) -> &'a mut crate::W<REG> {
        self.variant(USART1SEL::Apb2)
    }
    #[doc = "System clock is selected as USART clock"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut crate::W<REG> {
        self.variant(USART1SEL::Sysclk)
    }
    #[doc = "HSI clock is selected as USART clock"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut crate::W<REG> {
        self.variant(USART1SEL::Hsi)
    }
    #[doc = "LSE clock is selected as USART clock"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(USART1SEL::Lse)
    }
}
#[doc = "USART 2 clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USART2SEL {
    #[doc = "0: APB1 clock (PCLK1) is selected as USART clock"]
    Apb1 = 0,
    #[doc = "1: System clock is selected as USART clock"]
    Sysclk = 1,
    #[doc = "2: HSI clock is selected as USART clock"]
    Hsi = 2,
    #[doc = "3: LSE clock is selected as USART clock"]
    Lse = 3,
}
impl From<USART2SEL> for u8 {
    #[inline(always)]
    fn from(variant: USART2SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for USART2SEL {
    type Ux = u8;
}
#[doc = "Field `USART2SEL` reader - USART 2 clock source selection"]
pub type USART2SEL_R = crate::FieldReader<USART2SEL>;
impl USART2SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USART2SEL {
        match self.bits {
            0 => USART2SEL::Apb1,
            1 => USART2SEL::Sysclk,
            2 => USART2SEL::Hsi,
            3 => USART2SEL::Lse,
            _ => unreachable!(),
        }
    }
    #[doc = "APB1 clock (PCLK1) is selected as USART clock"]
    #[inline(always)]
    pub fn is_apb1(&self) -> bool {
        *self == USART2SEL::Apb1
    }
    #[doc = "System clock is selected as USART clock"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == USART2SEL::Sysclk
    }
    #[doc = "HSI clock is selected as USART clock"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == USART2SEL::Hsi
    }
    #[doc = "LSE clock is selected as USART clock"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == USART2SEL::Lse
    }
}
#[doc = "Field `USART2SEL` writer - USART 2 clock source selection"]
pub type USART2SEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, USART2SEL>;
impl<'a, REG> USART2SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "APB1 clock (PCLK1) is selected as USART clock"]
    #[inline(always)]
    pub fn apb1(self) -> &'a mut crate::W<REG> {
        self.variant(USART2SEL::Apb1)
    }
    #[doc = "System clock is selected as USART clock"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut crate::W<REG> {
        self.variant(USART2SEL::Sysclk)
    }
    #[doc = "HSI clock is selected as USART clock"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut crate::W<REG> {
        self.variant(USART2SEL::Hsi)
    }
    #[doc = "LSE clock is selected as USART clock"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(USART2SEL::Lse)
    }
}
#[doc = "Field `USART6SEL` reader - USART 6 clock source selection"]
pub use USART1SEL_R as USART6SEL_R;
#[doc = "Field `USART6SEL` writer - USART 6 clock source selection"]
pub use USART1SEL_W as USART6SEL_W;
#[doc = "Field `USART3SEL` reader - USART 3 clock source selection"]
pub use USART2SEL_R as USART3SEL_R;
#[doc = "Field `UART4SEL` reader - UART 4 clock source selection"]
pub use USART2SEL_R as UART4SEL_R;
#[doc = "Field `UART5SEL` reader - UART 5 clock source selection"]
pub use USART2SEL_R as UART5SEL_R;
#[doc = "Field `UART7SEL` reader - UART 7 clock source selection"]
pub use USART2SEL_R as UART7SEL_R;
#[doc = "Field `UART8SEL` reader - UART 8 clock source selection"]
pub use USART2SEL_R as UART8SEL_R;
#[doc = "Field `USART3SEL` writer - USART 3 clock source selection"]
pub use USART2SEL_W as USART3SEL_W;
#[doc = "Field `UART4SEL` writer - UART 4 clock source selection"]
pub use USART2SEL_W as UART4SEL_W;
#[doc = "Field `UART5SEL` writer - UART 5 clock source selection"]
pub use USART2SEL_W as UART5SEL_W;
#[doc = "Field `UART7SEL` writer - UART 7 clock source selection"]
pub use USART2SEL_W as UART7SEL_W;
#[doc = "Field `UART8SEL` writer - UART 8 clock source selection"]
pub use USART2SEL_W as UART8SEL_W;
#[doc = "I2C1 clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2C1SEL {
    #[doc = "0: APB clock selected as I2C clock"]
    Apb = 0,
    #[doc = "1: System clock selected as I2C clock"]
    Sysclk = 1,
    #[doc = "2: HSI clock selected as I2C clock"]
    Hsi = 2,
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
            0 => Some(I2C1SEL::Apb),
            1 => Some(I2C1SEL::Sysclk),
            2 => Some(I2C1SEL::Hsi),
            _ => None,
        }
    }
    #[doc = "APB clock selected as I2C clock"]
    #[inline(always)]
    pub fn is_apb(&self) -> bool {
        *self == I2C1SEL::Apb
    }
    #[doc = "System clock selected as I2C clock"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == I2C1SEL::Sysclk
    }
    #[doc = "HSI clock selected as I2C clock"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == I2C1SEL::Hsi
    }
}
#[doc = "Field `I2C1SEL` writer - I2C1 clock source selection"]
pub type I2C1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, I2C1SEL>;
impl<'a, REG> I2C1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "APB clock selected as I2C clock"]
    #[inline(always)]
    pub fn apb(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1SEL::Apb)
    }
    #[doc = "System clock selected as I2C clock"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1SEL::Sysclk)
    }
    #[doc = "HSI clock selected as I2C clock"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1SEL::Hsi)
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
    #[doc = "0: APB1 clock (PCLK1) selected as LPTILM1 clock"]
    Apb1 = 0,
    #[doc = "1: LSI clock is selected as LPTILM1 clock"]
    Lsi = 1,
    #[doc = "2: HSI clock is selected as LPTILM1 clock"]
    Hsi = 2,
    #[doc = "3: LSE clock is selected as LPTILM1 clock"]
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
            0 => LPTIM1SEL::Apb1,
            1 => LPTIM1SEL::Lsi,
            2 => LPTIM1SEL::Hsi,
            3 => LPTIM1SEL::Lse,
            _ => unreachable!(),
        }
    }
    #[doc = "APB1 clock (PCLK1) selected as LPTILM1 clock"]
    #[inline(always)]
    pub fn is_apb1(&self) -> bool {
        *self == LPTIM1SEL::Apb1
    }
    #[doc = "LSI clock is selected as LPTILM1 clock"]
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == LPTIM1SEL::Lsi
    }
    #[doc = "HSI clock is selected as LPTILM1 clock"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == LPTIM1SEL::Hsi
    }
    #[doc = "LSE clock is selected as LPTILM1 clock"]
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
    #[doc = "APB1 clock (PCLK1) selected as LPTILM1 clock"]
    #[inline(always)]
    pub fn apb1(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM1SEL::Apb1)
    }
    #[doc = "LSI clock is selected as LPTILM1 clock"]
    #[inline(always)]
    pub fn lsi(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM1SEL::Lsi)
    }
    #[doc = "HSI clock is selected as LPTILM1 clock"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM1SEL::Hsi)
    }
    #[doc = "LSE clock is selected as LPTILM1 clock"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM1SEL::Lse)
    }
}
#[doc = "48MHz clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CK48MSEL {
    #[doc = "0: 48MHz clock from PLL is selected"]
    Pll = 0,
    #[doc = "1: 48MHz clock from PLLSAI is selected"]
    Pllsai = 1,
}
impl From<CK48MSEL> for bool {
    #[inline(always)]
    fn from(variant: CK48MSEL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CK48MSEL` reader - 48MHz clock source selection"]
pub type CK48MSEL_R = crate::BitReader<CK48MSEL>;
impl CK48MSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CK48MSEL {
        match self.bits {
            false => CK48MSEL::Pll,
            true => CK48MSEL::Pllsai,
        }
    }
    #[doc = "48MHz clock from PLL is selected"]
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == CK48MSEL::Pll
    }
    #[doc = "48MHz clock from PLLSAI is selected"]
    #[inline(always)]
    pub fn is_pllsai(&self) -> bool {
        *self == CK48MSEL::Pllsai
    }
}
#[doc = "Field `CK48MSEL` writer - 48MHz clock source selection"]
pub type CK48MSEL_W<'a, REG> = crate::BitWriter<'a, REG, CK48MSEL>;
impl<'a, REG> CK48MSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "48MHz clock from PLL is selected"]
    #[inline(always)]
    pub fn pll(self) -> &'a mut crate::W<REG> {
        self.variant(CK48MSEL::Pll)
    }
    #[doc = "48MHz clock from PLLSAI is selected"]
    #[inline(always)]
    pub fn pllsai(self) -> &'a mut crate::W<REG> {
        self.variant(CK48MSEL::Pllsai)
    }
}
#[doc = "SDMMC1 clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDMMC1SEL {
    #[doc = "0: 48 MHz clock is selected as SD clock"]
    Ck48m = 0,
    #[doc = "1: System clock is selected as SD clock"]
    Sysclk = 1,
}
impl From<SDMMC1SEL> for bool {
    #[inline(always)]
    fn from(variant: SDMMC1SEL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDMMC1SEL` reader - SDMMC1 clock source selection"]
pub type SDMMC1SEL_R = crate::BitReader<SDMMC1SEL>;
impl SDMMC1SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SDMMC1SEL {
        match self.bits {
            false => SDMMC1SEL::Ck48m,
            true => SDMMC1SEL::Sysclk,
        }
    }
    #[doc = "48 MHz clock is selected as SD clock"]
    #[inline(always)]
    pub fn is_ck48m(&self) -> bool {
        *self == SDMMC1SEL::Ck48m
    }
    #[doc = "System clock is selected as SD clock"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == SDMMC1SEL::Sysclk
    }
}
#[doc = "Field `SDMMC1SEL` writer - SDMMC1 clock source selection"]
pub type SDMMC1SEL_W<'a, REG> = crate::BitWriter<'a, REG, SDMMC1SEL>;
impl<'a, REG> SDMMC1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "48 MHz clock is selected as SD clock"]
    #[inline(always)]
    pub fn ck48m(self) -> &'a mut crate::W<REG> {
        self.variant(SDMMC1SEL::Ck48m)
    }
    #[doc = "System clock is selected as SD clock"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut crate::W<REG> {
        self.variant(SDMMC1SEL::Sysclk)
    }
}
#[doc = "Field `SDMMC2SEL` reader - SDMMC2 clock source selection"]
pub use SDMMC1SEL_R as SDMMC2SEL_R;
#[doc = "Field `SDMMC2SEL` writer - SDMMC2 clock source selection"]
pub use SDMMC1SEL_W as SDMMC2SEL_W;
impl R {
    #[doc = "Bits 0:1 - USART 1 clock source selection"]
    #[inline(always)]
    pub fn usart1sel(&self) -> USART1SEL_R {
        USART1SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - USART 2 clock source selection"]
    #[inline(always)]
    pub fn usart2sel(&self) -> USART2SEL_R {
        USART2SEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - USART 3 clock source selection"]
    #[inline(always)]
    pub fn usart3sel(&self) -> USART3SEL_R {
        USART3SEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - UART 4 clock source selection"]
    #[inline(always)]
    pub fn uart4sel(&self) -> UART4SEL_R {
        UART4SEL_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - UART 5 clock source selection"]
    #[inline(always)]
    pub fn uart5sel(&self) -> UART5SEL_R {
        UART5SEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - USART 6 clock source selection"]
    #[inline(always)]
    pub fn usart6sel(&self) -> USART6SEL_R {
        USART6SEL_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - UART 7 clock source selection"]
    #[inline(always)]
    pub fn uart7sel(&self) -> UART7SEL_R {
        UART7SEL_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - UART 8 clock source selection"]
    #[inline(always)]
    pub fn uart8sel(&self) -> UART8SEL_R {
        UART8SEL_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - I2C1 clock source selection"]
    #[inline(always)]
    pub fn i2c1sel(&self) -> I2C1SEL_R {
        I2C1SEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - I2C2 clock source selection"]
    #[inline(always)]
    pub fn i2c2sel(&self) -> I2C2SEL_R {
        I2C2SEL_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - I2C3 clock source selection"]
    #[inline(always)]
    pub fn i2c3sel(&self) -> I2C3SEL_R {
        I2C3SEL_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Low power timer 1 clock source selection"]
    #[inline(always)]
    pub fn lptim1sel(&self) -> LPTIM1SEL_R {
        LPTIM1SEL_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 27 - 48MHz clock source selection"]
    #[inline(always)]
    pub fn ck48msel(&self) -> CK48MSEL_R {
        CK48MSEL_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - SDMMC1 clock source selection"]
    #[inline(always)]
    pub fn sdmmc1sel(&self) -> SDMMC1SEL_R {
        SDMMC1SEL_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - SDMMC2 clock source selection"]
    #[inline(always)]
    pub fn sdmmc2sel(&self) -> SDMMC2SEL_R {
        SDMMC2SEL_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - USART 1 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn usart1sel(&mut self) -> USART1SEL_W<DCKCFGR2rs> {
        USART1SEL_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - USART 2 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn usart2sel(&mut self) -> USART2SEL_W<DCKCFGR2rs> {
        USART2SEL_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - USART 3 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn usart3sel(&mut self) -> USART3SEL_W<DCKCFGR2rs> {
        USART3SEL_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - UART 4 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn uart4sel(&mut self) -> UART4SEL_W<DCKCFGR2rs> {
        UART4SEL_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - UART 5 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn uart5sel(&mut self) -> UART5SEL_W<DCKCFGR2rs> {
        UART5SEL_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - USART 6 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn usart6sel(&mut self) -> USART6SEL_W<DCKCFGR2rs> {
        USART6SEL_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - UART 7 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn uart7sel(&mut self) -> UART7SEL_W<DCKCFGR2rs> {
        UART7SEL_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - UART 8 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn uart8sel(&mut self) -> UART8SEL_W<DCKCFGR2rs> {
        UART8SEL_W::new(self, 14)
    }
    #[doc = "Bits 16:17 - I2C1 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1sel(&mut self) -> I2C1SEL_W<DCKCFGR2rs> {
        I2C1SEL_W::new(self, 16)
    }
    #[doc = "Bits 18:19 - I2C2 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2sel(&mut self) -> I2C2SEL_W<DCKCFGR2rs> {
        I2C2SEL_W::new(self, 18)
    }
    #[doc = "Bits 20:21 - I2C3 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn i2c3sel(&mut self) -> I2C3SEL_W<DCKCFGR2rs> {
        I2C3SEL_W::new(self, 20)
    }
    #[doc = "Bits 24:25 - Low power timer 1 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn lptim1sel(&mut self) -> LPTIM1SEL_W<DCKCFGR2rs> {
        LPTIM1SEL_W::new(self, 24)
    }
    #[doc = "Bit 27 - 48MHz clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn ck48msel(&mut self) -> CK48MSEL_W<DCKCFGR2rs> {
        CK48MSEL_W::new(self, 27)
    }
    #[doc = "Bit 28 - SDMMC1 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc1sel(&mut self) -> SDMMC1SEL_W<DCKCFGR2rs> {
        SDMMC1SEL_W::new(self, 28)
    }
    #[doc = "Bit 29 - SDMMC2 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc2sel(&mut self) -> SDMMC2SEL_W<DCKCFGR2rs> {
        SDMMC2SEL_W::new(self, 29)
    }
}
#[doc = "dedicated clocks configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dckcfgr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dckcfgr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCKCFGR2rs;
impl crate::RegisterSpec for DCKCFGR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dckcfgr2::R`](R) reader structure"]
impl crate::Readable for DCKCFGR2rs {}
#[doc = "`write(|w| ..)` method takes [`dckcfgr2::W`](W) writer structure"]
impl crate::Writable for DCKCFGR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCKCFGR2 to value 0"]
impl crate::Resettable for DCKCFGR2rs {
    const RESET_VALUE: u32 = 0;
}
