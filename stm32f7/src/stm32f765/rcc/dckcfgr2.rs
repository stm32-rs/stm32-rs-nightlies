///Register `DCKCFGR2` reader
pub type R = crate::R<DCKCFGR2rs>;
///Register `DCKCFGR2` writer
pub type W = crate::W<DCKCFGR2rs>;
/**USART 1 clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USART1SEL {
    ///0: APB2 clock (PCLK2) is selected as USART clock
    Apb2 = 0,
    ///1: System clock is selected as USART clock
    Sysclk = 1,
    ///2: HSI clock is selected as USART clock
    Hsi = 2,
    ///3: LSE clock is selected as USART clock
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
///Field `USART1SEL` reader - USART 1 clock source selection
pub type USART1SEL_R = crate::FieldReader<USART1SEL>;
impl USART1SEL_R {
    ///Get enumerated values variant
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
    ///APB2 clock (PCLK2) is selected as USART clock
    #[inline(always)]
    pub fn is_apb2(&self) -> bool {
        *self == USART1SEL::Apb2
    }
    ///System clock is selected as USART clock
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == USART1SEL::Sysclk
    }
    ///HSI clock is selected as USART clock
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == USART1SEL::Hsi
    }
    ///LSE clock is selected as USART clock
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == USART1SEL::Lse
    }
}
///Field `USART1SEL` writer - USART 1 clock source selection
pub type USART1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, USART1SEL, crate::Safe>;
impl<'a, REG> USART1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///APB2 clock (PCLK2) is selected as USART clock
    #[inline(always)]
    pub fn apb2(self) -> &'a mut crate::W<REG> {
        self.variant(USART1SEL::Apb2)
    }
    ///System clock is selected as USART clock
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut crate::W<REG> {
        self.variant(USART1SEL::Sysclk)
    }
    ///HSI clock is selected as USART clock
    #[inline(always)]
    pub fn hsi(self) -> &'a mut crate::W<REG> {
        self.variant(USART1SEL::Hsi)
    }
    ///LSE clock is selected as USART clock
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(USART1SEL::Lse)
    }
}
/**USART 2 clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USART2SEL {
    ///0: APB1 clock (PCLK1) is selected as USART clock
    Apb1 = 0,
    ///1: System clock is selected as USART clock
    Sysclk = 1,
    ///2: HSI clock is selected as USART clock
    Hsi = 2,
    ///3: LSE clock is selected as USART clock
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
impl crate::IsEnum for USART2SEL {}
///Field `USART2SEL` reader - USART 2 clock source selection
pub type USART2SEL_R = crate::FieldReader<USART2SEL>;
impl USART2SEL_R {
    ///Get enumerated values variant
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
    ///APB1 clock (PCLK1) is selected as USART clock
    #[inline(always)]
    pub fn is_apb1(&self) -> bool {
        *self == USART2SEL::Apb1
    }
    ///System clock is selected as USART clock
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == USART2SEL::Sysclk
    }
    ///HSI clock is selected as USART clock
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == USART2SEL::Hsi
    }
    ///LSE clock is selected as USART clock
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == USART2SEL::Lse
    }
}
///Field `USART2SEL` writer - USART 2 clock source selection
pub type USART2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, USART2SEL, crate::Safe>;
impl<'a, REG> USART2SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///APB1 clock (PCLK1) is selected as USART clock
    #[inline(always)]
    pub fn apb1(self) -> &'a mut crate::W<REG> {
        self.variant(USART2SEL::Apb1)
    }
    ///System clock is selected as USART clock
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut crate::W<REG> {
        self.variant(USART2SEL::Sysclk)
    }
    ///HSI clock is selected as USART clock
    #[inline(always)]
    pub fn hsi(self) -> &'a mut crate::W<REG> {
        self.variant(USART2SEL::Hsi)
    }
    ///LSE clock is selected as USART clock
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(USART2SEL::Lse)
    }
}
///Field `USART6SEL` reader - USART 6 clock source selection
pub use USART1SEL_R as USART6SEL_R;
///Field `USART6SEL` writer - USART 6 clock source selection
pub use USART1SEL_W as USART6SEL_W;
///Field `USART3SEL` reader - USART 3 clock source selection
pub use USART2SEL_R as USART3SEL_R;
///Field `UART4SEL` reader - UART 4 clock source selection
pub use USART2SEL_R as UART4SEL_R;
///Field `UART5SEL` reader - UART 5 clock source selection
pub use USART2SEL_R as UART5SEL_R;
///Field `UART7SEL` reader - UART 7 clock source selection
pub use USART2SEL_R as UART7SEL_R;
///Field `UART8SEL` reader - UART 8 clock source selection
pub use USART2SEL_R as UART8SEL_R;
///Field `USART3SEL` writer - USART 3 clock source selection
pub use USART2SEL_W as USART3SEL_W;
///Field `UART4SEL` writer - UART 4 clock source selection
pub use USART2SEL_W as UART4SEL_W;
///Field `UART5SEL` writer - UART 5 clock source selection
pub use USART2SEL_W as UART5SEL_W;
///Field `UART7SEL` writer - UART 7 clock source selection
pub use USART2SEL_W as UART7SEL_W;
///Field `UART8SEL` writer - UART 8 clock source selection
pub use USART2SEL_W as UART8SEL_W;
/**I2C1 clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2C1SEL {
    ///0: APB clock selected as I2C clock
    Apb = 0,
    ///1: System clock selected as I2C clock
    Sysclk = 1,
    ///2: HSI clock selected as I2C clock
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
impl crate::IsEnum for I2C1SEL {}
///Field `I2C1SEL` reader - I2C1 clock source selection
pub type I2C1SEL_R = crate::FieldReader<I2C1SEL>;
impl I2C1SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<I2C1SEL> {
        match self.bits {
            0 => Some(I2C1SEL::Apb),
            1 => Some(I2C1SEL::Sysclk),
            2 => Some(I2C1SEL::Hsi),
            _ => None,
        }
    }
    ///APB clock selected as I2C clock
    #[inline(always)]
    pub fn is_apb(&self) -> bool {
        *self == I2C1SEL::Apb
    }
    ///System clock selected as I2C clock
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == I2C1SEL::Sysclk
    }
    ///HSI clock selected as I2C clock
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == I2C1SEL::Hsi
    }
}
///Field `I2C1SEL` writer - I2C1 clock source selection
pub type I2C1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, I2C1SEL>;
impl<'a, REG> I2C1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///APB clock selected as I2C clock
    #[inline(always)]
    pub fn apb(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1SEL::Apb)
    }
    ///System clock selected as I2C clock
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1SEL::Sysclk)
    }
    ///HSI clock selected as I2C clock
    #[inline(always)]
    pub fn hsi(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1SEL::Hsi)
    }
}
///Field `I2C2SEL` reader - I2C2 clock source selection
pub use I2C1SEL_R as I2C2SEL_R;
///Field `I2C3SEL` reader - I2C3 clock source selection
pub use I2C1SEL_R as I2C3SEL_R;
///Field `I2C4SEL` reader - I2C4 clock source selection
pub use I2C1SEL_R as I2C4SEL_R;
///Field `I2C2SEL` writer - I2C2 clock source selection
pub use I2C1SEL_W as I2C2SEL_W;
///Field `I2C3SEL` writer - I2C3 clock source selection
pub use I2C1SEL_W as I2C3SEL_W;
///Field `I2C4SEL` writer - I2C4 clock source selection
pub use I2C1SEL_W as I2C4SEL_W;
/**Low power timer 1 clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LPTIM1SEL {
    ///0: APB1 clock (PCLK1) selected as LPTILM1 clock
    Apb1 = 0,
    ///1: LSI clock is selected as LPTILM1 clock
    Lsi = 1,
    ///2: HSI clock is selected as LPTILM1 clock
    Hsi = 2,
    ///3: LSE clock is selected as LPTILM1 clock
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
            0 => LPTIM1SEL::Apb1,
            1 => LPTIM1SEL::Lsi,
            2 => LPTIM1SEL::Hsi,
            3 => LPTIM1SEL::Lse,
            _ => unreachable!(),
        }
    }
    ///APB1 clock (PCLK1) selected as LPTILM1 clock
    #[inline(always)]
    pub fn is_apb1(&self) -> bool {
        *self == LPTIM1SEL::Apb1
    }
    ///LSI clock is selected as LPTILM1 clock
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == LPTIM1SEL::Lsi
    }
    ///HSI clock is selected as LPTILM1 clock
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == LPTIM1SEL::Hsi
    }
    ///LSE clock is selected as LPTILM1 clock
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
    ///APB1 clock (PCLK1) selected as LPTILM1 clock
    #[inline(always)]
    pub fn apb1(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM1SEL::Apb1)
    }
    ///LSI clock is selected as LPTILM1 clock
    #[inline(always)]
    pub fn lsi(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM1SEL::Lsi)
    }
    ///HSI clock is selected as LPTILM1 clock
    #[inline(always)]
    pub fn hsi(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM1SEL::Hsi)
    }
    ///LSE clock is selected as LPTILM1 clock
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM1SEL::Lse)
    }
}
/**HDMI-CEC clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CECSEL {
    ///0: LSE clock is selected as HDMI-CEC clock
    Lse = 0,
    ///1: HSI divided by 488 clock is selected as HDMI-CEC clock
    HsiDiv488 = 1,
}
impl From<CECSEL> for bool {
    #[inline(always)]
    fn from(variant: CECSEL) -> Self {
        variant as u8 != 0
    }
}
///Field `CECSEL` reader - HDMI-CEC clock source selection
pub type CECSEL_R = crate::BitReader<CECSEL>;
impl CECSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CECSEL {
        match self.bits {
            false => CECSEL::Lse,
            true => CECSEL::HsiDiv488,
        }
    }
    ///LSE clock is selected as HDMI-CEC clock
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == CECSEL::Lse
    }
    ///HSI divided by 488 clock is selected as HDMI-CEC clock
    #[inline(always)]
    pub fn is_hsi_div488(&self) -> bool {
        *self == CECSEL::HsiDiv488
    }
}
///Field `CECSEL` writer - HDMI-CEC clock source selection
pub type CECSEL_W<'a, REG> = crate::BitWriter<'a, REG, CECSEL>;
impl<'a, REG> CECSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///LSE clock is selected as HDMI-CEC clock
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(CECSEL::Lse)
    }
    ///HSI divided by 488 clock is selected as HDMI-CEC clock
    #[inline(always)]
    pub fn hsi_div488(self) -> &'a mut crate::W<REG> {
        self.variant(CECSEL::HsiDiv488)
    }
}
/**48MHz clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CK48MSEL {
    ///0: 48MHz clock from PLL is selected
    Pll = 0,
    ///1: 48MHz clock from PLLSAI is selected
    Pllsai = 1,
}
impl From<CK48MSEL> for bool {
    #[inline(always)]
    fn from(variant: CK48MSEL) -> Self {
        variant as u8 != 0
    }
}
///Field `CK48MSEL` reader - 48MHz clock source selection
pub type CK48MSEL_R = crate::BitReader<CK48MSEL>;
impl CK48MSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CK48MSEL {
        match self.bits {
            false => CK48MSEL::Pll,
            true => CK48MSEL::Pllsai,
        }
    }
    ///48MHz clock from PLL is selected
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == CK48MSEL::Pll
    }
    ///48MHz clock from PLLSAI is selected
    #[inline(always)]
    pub fn is_pllsai(&self) -> bool {
        *self == CK48MSEL::Pllsai
    }
}
///Field `CK48MSEL` writer - 48MHz clock source selection
pub type CK48MSEL_W<'a, REG> = crate::BitWriter<'a, REG, CK48MSEL>;
impl<'a, REG> CK48MSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///48MHz clock from PLL is selected
    #[inline(always)]
    pub fn pll(self) -> &'a mut crate::W<REG> {
        self.variant(CK48MSEL::Pll)
    }
    ///48MHz clock from PLLSAI is selected
    #[inline(always)]
    pub fn pllsai(self) -> &'a mut crate::W<REG> {
        self.variant(CK48MSEL::Pllsai)
    }
}
/**SDMMC1 clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDMMC1SEL {
    ///0: 48 MHz clock is selected as SD clock
    Ck48m = 0,
    ///1: System clock is selected as SD clock
    Sysclk = 1,
}
impl From<SDMMC1SEL> for bool {
    #[inline(always)]
    fn from(variant: SDMMC1SEL) -> Self {
        variant as u8 != 0
    }
}
///Field `SDMMC1SEL` reader - SDMMC1 clock source selection
pub type SDMMC1SEL_R = crate::BitReader<SDMMC1SEL>;
impl SDMMC1SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SDMMC1SEL {
        match self.bits {
            false => SDMMC1SEL::Ck48m,
            true => SDMMC1SEL::Sysclk,
        }
    }
    ///48 MHz clock is selected as SD clock
    #[inline(always)]
    pub fn is_ck48m(&self) -> bool {
        *self == SDMMC1SEL::Ck48m
    }
    ///System clock is selected as SD clock
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == SDMMC1SEL::Sysclk
    }
}
///Field `SDMMC1SEL` writer - SDMMC1 clock source selection
pub type SDMMC1SEL_W<'a, REG> = crate::BitWriter<'a, REG, SDMMC1SEL>;
impl<'a, REG> SDMMC1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///48 MHz clock is selected as SD clock
    #[inline(always)]
    pub fn ck48m(self) -> &'a mut crate::W<REG> {
        self.variant(SDMMC1SEL::Ck48m)
    }
    ///System clock is selected as SD clock
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut crate::W<REG> {
        self.variant(SDMMC1SEL::Sysclk)
    }
}
///Field `SDMMC2SEL` reader - SDMMC2 clock source selection
pub use SDMMC1SEL_R as SDMMC2SEL_R;
///Field `SDMMC2SEL` writer - SDMMC2 clock source selection
pub use SDMMC1SEL_W as SDMMC2SEL_W;
/**DSI clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSISEL {
    ///0: DSI-PHY used as DSI byte lane clock source (usual case)
    DsiPhy = 0,
    ///1: PLLR used as DSI byte lane clock source, used in case DSI PLL and DSI-PHY are off (low power mode)
    Pllr = 1,
}
impl From<DSISEL> for bool {
    #[inline(always)]
    fn from(variant: DSISEL) -> Self {
        variant as u8 != 0
    }
}
///Field `DSISEL` reader - DSI clock source selection
pub type DSISEL_R = crate::BitReader<DSISEL>;
impl DSISEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DSISEL {
        match self.bits {
            false => DSISEL::DsiPhy,
            true => DSISEL::Pllr,
        }
    }
    ///DSI-PHY used as DSI byte lane clock source (usual case)
    #[inline(always)]
    pub fn is_dsi_phy(&self) -> bool {
        *self == DSISEL::DsiPhy
    }
    ///PLLR used as DSI byte lane clock source, used in case DSI PLL and DSI-PHY are off (low power mode)
    #[inline(always)]
    pub fn is_pllr(&self) -> bool {
        *self == DSISEL::Pllr
    }
}
///Field `DSISEL` writer - DSI clock source selection
pub type DSISEL_W<'a, REG> = crate::BitWriter<'a, REG, DSISEL>;
impl<'a, REG> DSISEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DSI-PHY used as DSI byte lane clock source (usual case)
    #[inline(always)]
    pub fn dsi_phy(self) -> &'a mut crate::W<REG> {
        self.variant(DSISEL::DsiPhy)
    }
    ///PLLR used as DSI byte lane clock source, used in case DSI PLL and DSI-PHY are off (low power mode)
    #[inline(always)]
    pub fn pllr(self) -> &'a mut crate::W<REG> {
        self.variant(DSISEL::Pllr)
    }
}
impl R {
    ///Bits 0:1 - USART 1 clock source selection
    #[inline(always)]
    pub fn usart1sel(&self) -> USART1SEL_R {
        USART1SEL_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - USART 2 clock source selection
    #[inline(always)]
    pub fn usart2sel(&self) -> USART2SEL_R {
        USART2SEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - USART 3 clock source selection
    #[inline(always)]
    pub fn usart3sel(&self) -> USART3SEL_R {
        USART3SEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 6:7 - UART 4 clock source selection
    #[inline(always)]
    pub fn uart4sel(&self) -> UART4SEL_R {
        UART4SEL_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:9 - UART 5 clock source selection
    #[inline(always)]
    pub fn uart5sel(&self) -> UART5SEL_R {
        UART5SEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - USART 6 clock source selection
    #[inline(always)]
    pub fn usart6sel(&self) -> USART6SEL_R {
        USART6SEL_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:13 - UART 7 clock source selection
    #[inline(always)]
    pub fn uart7sel(&self) -> UART7SEL_R {
        UART7SEL_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 14:15 - UART 8 clock source selection
    #[inline(always)]
    pub fn uart8sel(&self) -> UART8SEL_R {
        UART8SEL_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bits 16:17 - I2C1 clock source selection
    #[inline(always)]
    pub fn i2c1sel(&self) -> I2C1SEL_R {
        I2C1SEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:19 - I2C2 clock source selection
    #[inline(always)]
    pub fn i2c2sel(&self) -> I2C2SEL_R {
        I2C2SEL_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bits 20:21 - I2C3 clock source selection
    #[inline(always)]
    pub fn i2c3sel(&self) -> I2C3SEL_R {
        I2C3SEL_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bits 22:23 - I2C4 clock source selection
    #[inline(always)]
    pub fn i2c4sel(&self) -> I2C4SEL_R {
        I2C4SEL_R::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bits 24:25 - Low power timer 1 clock source selection
    #[inline(always)]
    pub fn lptim1sel(&self) -> LPTIM1SEL_R {
        LPTIM1SEL_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bit 26 - HDMI-CEC clock source selection
    #[inline(always)]
    pub fn cecsel(&self) -> CECSEL_R {
        CECSEL_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - 48MHz clock source selection
    #[inline(always)]
    pub fn ck48msel(&self) -> CK48MSEL_R {
        CK48MSEL_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - SDMMC1 clock source selection
    #[inline(always)]
    pub fn sdmmc1sel(&self) -> SDMMC1SEL_R {
        SDMMC1SEL_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - SDMMC2 clock source selection
    #[inline(always)]
    pub fn sdmmc2sel(&self) -> SDMMC2SEL_R {
        SDMMC2SEL_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - DSI clock source selection
    #[inline(always)]
    pub fn dsisel(&self) -> DSISEL_R {
        DSISEL_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCKCFGR2")
            .field("usart1sel", &self.usart1sel())
            .field("usart2sel", &self.usart2sel())
            .field("usart3sel", &self.usart3sel())
            .field("uart4sel", &self.uart4sel())
            .field("uart5sel", &self.uart5sel())
            .field("usart6sel", &self.usart6sel())
            .field("uart7sel", &self.uart7sel())
            .field("uart8sel", &self.uart8sel())
            .field("i2c1sel", &self.i2c1sel())
            .field("i2c2sel", &self.i2c2sel())
            .field("i2c3sel", &self.i2c3sel())
            .field("i2c4sel", &self.i2c4sel())
            .field("lptim1sel", &self.lptim1sel())
            .field("cecsel", &self.cecsel())
            .field("ck48msel", &self.ck48msel())
            .field("sdmmc1sel", &self.sdmmc1sel())
            .field("sdmmc2sel", &self.sdmmc2sel())
            .field("dsisel", &self.dsisel())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - USART 1 clock source selection
    #[inline(always)]
    pub fn usart1sel(&mut self) -> USART1SEL_W<'_, DCKCFGR2rs> {
        USART1SEL_W::new(self, 0)
    }
    ///Bits 2:3 - USART 2 clock source selection
    #[inline(always)]
    pub fn usart2sel(&mut self) -> USART2SEL_W<'_, DCKCFGR2rs> {
        USART2SEL_W::new(self, 2)
    }
    ///Bits 4:5 - USART 3 clock source selection
    #[inline(always)]
    pub fn usart3sel(&mut self) -> USART3SEL_W<'_, DCKCFGR2rs> {
        USART3SEL_W::new(self, 4)
    }
    ///Bits 6:7 - UART 4 clock source selection
    #[inline(always)]
    pub fn uart4sel(&mut self) -> UART4SEL_W<'_, DCKCFGR2rs> {
        UART4SEL_W::new(self, 6)
    }
    ///Bits 8:9 - UART 5 clock source selection
    #[inline(always)]
    pub fn uart5sel(&mut self) -> UART5SEL_W<'_, DCKCFGR2rs> {
        UART5SEL_W::new(self, 8)
    }
    ///Bits 10:11 - USART 6 clock source selection
    #[inline(always)]
    pub fn usart6sel(&mut self) -> USART6SEL_W<'_, DCKCFGR2rs> {
        USART6SEL_W::new(self, 10)
    }
    ///Bits 12:13 - UART 7 clock source selection
    #[inline(always)]
    pub fn uart7sel(&mut self) -> UART7SEL_W<'_, DCKCFGR2rs> {
        UART7SEL_W::new(self, 12)
    }
    ///Bits 14:15 - UART 8 clock source selection
    #[inline(always)]
    pub fn uart8sel(&mut self) -> UART8SEL_W<'_, DCKCFGR2rs> {
        UART8SEL_W::new(self, 14)
    }
    ///Bits 16:17 - I2C1 clock source selection
    #[inline(always)]
    pub fn i2c1sel(&mut self) -> I2C1SEL_W<'_, DCKCFGR2rs> {
        I2C1SEL_W::new(self, 16)
    }
    ///Bits 18:19 - I2C2 clock source selection
    #[inline(always)]
    pub fn i2c2sel(&mut self) -> I2C2SEL_W<'_, DCKCFGR2rs> {
        I2C2SEL_W::new(self, 18)
    }
    ///Bits 20:21 - I2C3 clock source selection
    #[inline(always)]
    pub fn i2c3sel(&mut self) -> I2C3SEL_W<'_, DCKCFGR2rs> {
        I2C3SEL_W::new(self, 20)
    }
    ///Bits 22:23 - I2C4 clock source selection
    #[inline(always)]
    pub fn i2c4sel(&mut self) -> I2C4SEL_W<'_, DCKCFGR2rs> {
        I2C4SEL_W::new(self, 22)
    }
    ///Bits 24:25 - Low power timer 1 clock source selection
    #[inline(always)]
    pub fn lptim1sel(&mut self) -> LPTIM1SEL_W<'_, DCKCFGR2rs> {
        LPTIM1SEL_W::new(self, 24)
    }
    ///Bit 26 - HDMI-CEC clock source selection
    #[inline(always)]
    pub fn cecsel(&mut self) -> CECSEL_W<'_, DCKCFGR2rs> {
        CECSEL_W::new(self, 26)
    }
    ///Bit 27 - 48MHz clock source selection
    #[inline(always)]
    pub fn ck48msel(&mut self) -> CK48MSEL_W<'_, DCKCFGR2rs> {
        CK48MSEL_W::new(self, 27)
    }
    ///Bit 28 - SDMMC1 clock source selection
    #[inline(always)]
    pub fn sdmmc1sel(&mut self) -> SDMMC1SEL_W<'_, DCKCFGR2rs> {
        SDMMC1SEL_W::new(self, 28)
    }
    ///Bit 29 - SDMMC2 clock source selection
    #[inline(always)]
    pub fn sdmmc2sel(&mut self) -> SDMMC2SEL_W<'_, DCKCFGR2rs> {
        SDMMC2SEL_W::new(self, 29)
    }
    ///Bit 30 - DSI clock source selection
    #[inline(always)]
    pub fn dsisel(&mut self) -> DSISEL_W<'_, DCKCFGR2rs> {
        DSISEL_W::new(self, 30)
    }
}
/**dedicated clocks configuration register

You can [`read`](crate::Reg::read) this register and get [`dckcfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dckcfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F765.html#RCC:DCKCFGR2)*/
pub struct DCKCFGR2rs;
impl crate::RegisterSpec for DCKCFGR2rs {
    type Ux = u32;
}
///`read()` method returns [`dckcfgr2::R`](R) reader structure
impl crate::Readable for DCKCFGR2rs {}
///`write(|w| ..)` method takes [`dckcfgr2::W`](W) writer structure
impl crate::Writable for DCKCFGR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DCKCFGR2 to value 0
impl crate::Resettable for DCKCFGR2rs {}
