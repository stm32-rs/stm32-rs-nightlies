#[doc = "Register `CDCCIP2R` reader"]
pub type R = crate::R<CDCCIP2Rrs>;
#[doc = "Register `CDCCIP2R` writer"]
pub type W = crate::W<CDCCIP2Rrs>;
#[doc = "USART2/3, UART4,5, 7 and 8 (APB1) kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USART234578SEL {
    #[doc = "0: rcc_pclk1 selected as peripheral clock"]
    RccPclk1 = 0,
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
impl From<USART234578SEL> for u8 {
    #[inline(always)]
    fn from(variant: USART234578SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for USART234578SEL {
    type Ux = u8;
}
#[doc = "Field `USART234578SEL` reader - USART2/3, UART4,5, 7 and 8 (APB1) kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
pub type USART234578SEL_R = crate::FieldReader<USART234578SEL>;
impl USART234578SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<USART234578SEL> {
        match self.bits {
            0 => Some(USART234578SEL::RccPclk1),
            1 => Some(USART234578SEL::Pll2Q),
            2 => Some(USART234578SEL::Pll3Q),
            3 => Some(USART234578SEL::HsiKer),
            4 => Some(USART234578SEL::CsiKer),
            5 => Some(USART234578SEL::Lse),
            _ => None,
        }
    }
    #[doc = "rcc_pclk1 selected as peripheral clock"]
    #[inline(always)]
    pub fn is_rcc_pclk1(&self) -> bool {
        *self == USART234578SEL::RccPclk1
    }
    #[doc = "pll2_q selected as peripheral clock"]
    #[inline(always)]
    pub fn is_pll2_q(&self) -> bool {
        *self == USART234578SEL::Pll2Q
    }
    #[doc = "pll3_q selected as peripheral clock"]
    #[inline(always)]
    pub fn is_pll3_q(&self) -> bool {
        *self == USART234578SEL::Pll3Q
    }
    #[doc = "hsi_ker selected as peripheral clock"]
    #[inline(always)]
    pub fn is_hsi_ker(&self) -> bool {
        *self == USART234578SEL::HsiKer
    }
    #[doc = "csi_ker selected as peripheral clock"]
    #[inline(always)]
    pub fn is_csi_ker(&self) -> bool {
        *self == USART234578SEL::CsiKer
    }
    #[doc = "LSE selected as peripheral clock"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == USART234578SEL::Lse
    }
}
#[doc = "Field `USART234578SEL` writer - USART2/3, UART4,5, 7 and 8 (APB1) kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
pub type USART234578SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, USART234578SEL>;
impl<'a, REG> USART234578SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "rcc_pclk1 selected as peripheral clock"]
    #[inline(always)]
    pub fn rcc_pclk1(self) -> &'a mut crate::W<REG> {
        self.variant(USART234578SEL::RccPclk1)
    }
    #[doc = "pll2_q selected as peripheral clock"]
    #[inline(always)]
    pub fn pll2_q(self) -> &'a mut crate::W<REG> {
        self.variant(USART234578SEL::Pll2Q)
    }
    #[doc = "pll3_q selected as peripheral clock"]
    #[inline(always)]
    pub fn pll3_q(self) -> &'a mut crate::W<REG> {
        self.variant(USART234578SEL::Pll3Q)
    }
    #[doc = "hsi_ker selected as peripheral clock"]
    #[inline(always)]
    pub fn hsi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(USART234578SEL::HsiKer)
    }
    #[doc = "csi_ker selected as peripheral clock"]
    #[inline(always)]
    pub fn csi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(USART234578SEL::CsiKer)
    }
    #[doc = "LSE selected as peripheral clock"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(USART234578SEL::Lse)
    }
}
#[doc = "USART1, 6, 9 and 10 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USART16910SEL {
    #[doc = "0: rcc_pclk2 selected as peripheral clock"]
    RccPclk2 = 0,
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
impl From<USART16910SEL> for u8 {
    #[inline(always)]
    fn from(variant: USART16910SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for USART16910SEL {
    type Ux = u8;
}
#[doc = "Field `USART16910SEL` reader - USART1, 6, 9 and 10 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
pub type USART16910SEL_R = crate::FieldReader<USART16910SEL>;
impl USART16910SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<USART16910SEL> {
        match self.bits {
            0 => Some(USART16910SEL::RccPclk2),
            1 => Some(USART16910SEL::Pll2Q),
            2 => Some(USART16910SEL::Pll3Q),
            3 => Some(USART16910SEL::HsiKer),
            4 => Some(USART16910SEL::CsiKer),
            5 => Some(USART16910SEL::Lse),
            _ => None,
        }
    }
    #[doc = "rcc_pclk2 selected as peripheral clock"]
    #[inline(always)]
    pub fn is_rcc_pclk2(&self) -> bool {
        *self == USART16910SEL::RccPclk2
    }
    #[doc = "pll2_q selected as peripheral clock"]
    #[inline(always)]
    pub fn is_pll2_q(&self) -> bool {
        *self == USART16910SEL::Pll2Q
    }
    #[doc = "pll3_q selected as peripheral clock"]
    #[inline(always)]
    pub fn is_pll3_q(&self) -> bool {
        *self == USART16910SEL::Pll3Q
    }
    #[doc = "hsi_ker selected as peripheral clock"]
    #[inline(always)]
    pub fn is_hsi_ker(&self) -> bool {
        *self == USART16910SEL::HsiKer
    }
    #[doc = "csi_ker selected as peripheral clock"]
    #[inline(always)]
    pub fn is_csi_ker(&self) -> bool {
        *self == USART16910SEL::CsiKer
    }
    #[doc = "LSE selected as peripheral clock"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == USART16910SEL::Lse
    }
}
#[doc = "Field `USART16910SEL` writer - USART1, 6, 9 and 10 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
pub type USART16910SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, USART16910SEL>;
impl<'a, REG> USART16910SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "rcc_pclk2 selected as peripheral clock"]
    #[inline(always)]
    pub fn rcc_pclk2(self) -> &'a mut crate::W<REG> {
        self.variant(USART16910SEL::RccPclk2)
    }
    #[doc = "pll2_q selected as peripheral clock"]
    #[inline(always)]
    pub fn pll2_q(self) -> &'a mut crate::W<REG> {
        self.variant(USART16910SEL::Pll2Q)
    }
    #[doc = "pll3_q selected as peripheral clock"]
    #[inline(always)]
    pub fn pll3_q(self) -> &'a mut crate::W<REG> {
        self.variant(USART16910SEL::Pll3Q)
    }
    #[doc = "hsi_ker selected as peripheral clock"]
    #[inline(always)]
    pub fn hsi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(USART16910SEL::HsiKer)
    }
    #[doc = "csi_ker selected as peripheral clock"]
    #[inline(always)]
    pub fn csi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(USART16910SEL::CsiKer)
    }
    #[doc = "LSE selected as peripheral clock"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(USART16910SEL::Lse)
    }
}
#[doc = "RNG kernel clock source selection Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RNGSEL {
    #[doc = "0: HSI48 selected as peripheral clock"]
    Hsi48 = 0,
    #[doc = "1: pll1_q selected as peripheral clock"]
    Pll1Q = 1,
    #[doc = "2: LSE selected as peripheral clock"]
    Lse = 2,
    #[doc = "3: LSI selected as peripheral clock"]
    Lsi = 3,
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
#[doc = "Field `RNGSEL` reader - RNG kernel clock source selection Set and reset by software."]
pub type RNGSEL_R = crate::FieldReader<RNGSEL>;
impl RNGSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RNGSEL {
        match self.bits {
            0 => RNGSEL::Hsi48,
            1 => RNGSEL::Pll1Q,
            2 => RNGSEL::Lse,
            3 => RNGSEL::Lsi,
            _ => unreachable!(),
        }
    }
    #[doc = "HSI48 selected as peripheral clock"]
    #[inline(always)]
    pub fn is_hsi48(&self) -> bool {
        *self == RNGSEL::Hsi48
    }
    #[doc = "pll1_q selected as peripheral clock"]
    #[inline(always)]
    pub fn is_pll1_q(&self) -> bool {
        *self == RNGSEL::Pll1Q
    }
    #[doc = "LSE selected as peripheral clock"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == RNGSEL::Lse
    }
    #[doc = "LSI selected as peripheral clock"]
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == RNGSEL::Lsi
    }
}
#[doc = "Field `RNGSEL` writer - RNG kernel clock source selection Set and reset by software."]
pub type RNGSEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, RNGSEL>;
impl<'a, REG> RNGSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "HSI48 selected as peripheral clock"]
    #[inline(always)]
    pub fn hsi48(self) -> &'a mut crate::W<REG> {
        self.variant(RNGSEL::Hsi48)
    }
    #[doc = "pll1_q selected as peripheral clock"]
    #[inline(always)]
    pub fn pll1_q(self) -> &'a mut crate::W<REG> {
        self.variant(RNGSEL::Pll1Q)
    }
    #[doc = "LSE selected as peripheral clock"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(RNGSEL::Lse)
    }
    #[doc = "LSI selected as peripheral clock"]
    #[inline(always)]
    pub fn lsi(self) -> &'a mut crate::W<REG> {
        self.variant(RNGSEL::Lsi)
    }
}
#[doc = "I2C1,2,3 kernel clock source selection Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2C123SEL {
    #[doc = "0: rcc_pclk1 selected as peripheral clock"]
    RccPclk1 = 0,
    #[doc = "1: pll3_r selected as peripheral clock"]
    Pll3R = 1,
    #[doc = "2: hsi_ker selected as peripheral clock"]
    HsiKer = 2,
    #[doc = "3: csi_ker selected as peripheral clock"]
    CsiKer = 3,
}
impl From<I2C123SEL> for u8 {
    #[inline(always)]
    fn from(variant: I2C123SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for I2C123SEL {
    type Ux = u8;
}
#[doc = "Field `I2C123SEL` reader - I2C1,2,3 kernel clock source selection Set and reset by software."]
pub type I2C123SEL_R = crate::FieldReader<I2C123SEL>;
impl I2C123SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2C123SEL {
        match self.bits {
            0 => I2C123SEL::RccPclk1,
            1 => I2C123SEL::Pll3R,
            2 => I2C123SEL::HsiKer,
            3 => I2C123SEL::CsiKer,
            _ => unreachable!(),
        }
    }
    #[doc = "rcc_pclk1 selected as peripheral clock"]
    #[inline(always)]
    pub fn is_rcc_pclk1(&self) -> bool {
        *self == I2C123SEL::RccPclk1
    }
    #[doc = "pll3_r selected as peripheral clock"]
    #[inline(always)]
    pub fn is_pll3_r(&self) -> bool {
        *self == I2C123SEL::Pll3R
    }
    #[doc = "hsi_ker selected as peripheral clock"]
    #[inline(always)]
    pub fn is_hsi_ker(&self) -> bool {
        *self == I2C123SEL::HsiKer
    }
    #[doc = "csi_ker selected as peripheral clock"]
    #[inline(always)]
    pub fn is_csi_ker(&self) -> bool {
        *self == I2C123SEL::CsiKer
    }
}
#[doc = "Field `I2C123SEL` writer - I2C1,2,3 kernel clock source selection Set and reset by software."]
pub type I2C123SEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, I2C123SEL>;
impl<'a, REG> I2C123SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "rcc_pclk1 selected as peripheral clock"]
    #[inline(always)]
    pub fn rcc_pclk1(self) -> &'a mut crate::W<REG> {
        self.variant(I2C123SEL::RccPclk1)
    }
    #[doc = "pll3_r selected as peripheral clock"]
    #[inline(always)]
    pub fn pll3_r(self) -> &'a mut crate::W<REG> {
        self.variant(I2C123SEL::Pll3R)
    }
    #[doc = "hsi_ker selected as peripheral clock"]
    #[inline(always)]
    pub fn hsi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(I2C123SEL::HsiKer)
    }
    #[doc = "csi_ker selected as peripheral clock"]
    #[inline(always)]
    pub fn csi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(I2C123SEL::CsiKer)
    }
}
#[doc = "USBOTG 1 and 2 kernel clock source selection Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USBSEL {
    #[doc = "0: Disable the kernel clock"]
    Disable = 0,
    #[doc = "1: pll1_q selected as peripheral clock"]
    Pll1Q = 1,
    #[doc = "2: pll3_q selected as peripheral clock"]
    Pll3Q = 2,
    #[doc = "3: HSI48 selected as peripheral clock"]
    Hsi48 = 3,
}
impl From<USBSEL> for u8 {
    #[inline(always)]
    fn from(variant: USBSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for USBSEL {
    type Ux = u8;
}
#[doc = "Field `USBSEL` reader - USBOTG 1 and 2 kernel clock source selection Set and reset by software."]
pub type USBSEL_R = crate::FieldReader<USBSEL>;
impl USBSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USBSEL {
        match self.bits {
            0 => USBSEL::Disable,
            1 => USBSEL::Pll1Q,
            2 => USBSEL::Pll3Q,
            3 => USBSEL::Hsi48,
            _ => unreachable!(),
        }
    }
    #[doc = "Disable the kernel clock"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == USBSEL::Disable
    }
    #[doc = "pll1_q selected as peripheral clock"]
    #[inline(always)]
    pub fn is_pll1_q(&self) -> bool {
        *self == USBSEL::Pll1Q
    }
    #[doc = "pll3_q selected as peripheral clock"]
    #[inline(always)]
    pub fn is_pll3_q(&self) -> bool {
        *self == USBSEL::Pll3Q
    }
    #[doc = "HSI48 selected as peripheral clock"]
    #[inline(always)]
    pub fn is_hsi48(&self) -> bool {
        *self == USBSEL::Hsi48
    }
}
#[doc = "Field `USBSEL` writer - USBOTG 1 and 2 kernel clock source selection Set and reset by software."]
pub type USBSEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, USBSEL>;
impl<'a, REG> USBSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable the kernel clock"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(USBSEL::Disable)
    }
    #[doc = "pll1_q selected as peripheral clock"]
    #[inline(always)]
    pub fn pll1_q(self) -> &'a mut crate::W<REG> {
        self.variant(USBSEL::Pll1Q)
    }
    #[doc = "pll3_q selected as peripheral clock"]
    #[inline(always)]
    pub fn pll3_q(self) -> &'a mut crate::W<REG> {
        self.variant(USBSEL::Pll3Q)
    }
    #[doc = "HSI48 selected as peripheral clock"]
    #[inline(always)]
    pub fn hsi48(self) -> &'a mut crate::W<REG> {
        self.variant(USBSEL::Hsi48)
    }
}
#[doc = "HDMI-CEC kernel clock source selection Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CECSEL {
    #[doc = "0: LSE selected as peripheral clock"]
    Lse = 0,
    #[doc = "1: LSI selected as peripheral clock"]
    Lsi = 1,
    #[doc = "2: csi_ker selected as peripheral clock"]
    CsiKer = 2,
}
impl From<CECSEL> for u8 {
    #[inline(always)]
    fn from(variant: CECSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CECSEL {
    type Ux = u8;
}
#[doc = "Field `CECSEL` reader - HDMI-CEC kernel clock source selection Set and reset by software."]
pub type CECSEL_R = crate::FieldReader<CECSEL>;
impl CECSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CECSEL> {
        match self.bits {
            0 => Some(CECSEL::Lse),
            1 => Some(CECSEL::Lsi),
            2 => Some(CECSEL::CsiKer),
            _ => None,
        }
    }
    #[doc = "LSE selected as peripheral clock"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == CECSEL::Lse
    }
    #[doc = "LSI selected as peripheral clock"]
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == CECSEL::Lsi
    }
    #[doc = "csi_ker selected as peripheral clock"]
    #[inline(always)]
    pub fn is_csi_ker(&self) -> bool {
        *self == CECSEL::CsiKer
    }
}
#[doc = "Field `CECSEL` writer - HDMI-CEC kernel clock source selection Set and reset by software."]
pub type CECSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CECSEL>;
impl<'a, REG> CECSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LSE selected as peripheral clock"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(CECSEL::Lse)
    }
    #[doc = "LSI selected as peripheral clock"]
    #[inline(always)]
    pub fn lsi(self) -> &'a mut crate::W<REG> {
        self.variant(CECSEL::Lsi)
    }
    #[doc = "csi_ker selected as peripheral clock"]
    #[inline(always)]
    pub fn csi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(CECSEL::CsiKer)
    }
}
#[doc = "LPTIM1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LPTIM1SEL {
    #[doc = "0: rcc_pclk1 selected as peripheral clock"]
    RccPclk1 = 0,
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
impl From<LPTIM1SEL> for u8 {
    #[inline(always)]
    fn from(variant: LPTIM1SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LPTIM1SEL {
    type Ux = u8;
}
#[doc = "Field `LPTIM1SEL` reader - LPTIM1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
pub type LPTIM1SEL_R = crate::FieldReader<LPTIM1SEL>;
impl LPTIM1SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LPTIM1SEL> {
        match self.bits {
            0 => Some(LPTIM1SEL::RccPclk1),
            1 => Some(LPTIM1SEL::Pll2P),
            2 => Some(LPTIM1SEL::Pll3R),
            3 => Some(LPTIM1SEL::Lse),
            4 => Some(LPTIM1SEL::Lsi),
            5 => Some(LPTIM1SEL::Per),
            _ => None,
        }
    }
    #[doc = "rcc_pclk1 selected as peripheral clock"]
    #[inline(always)]
    pub fn is_rcc_pclk1(&self) -> bool {
        *self == LPTIM1SEL::RccPclk1
    }
    #[doc = "pll2_p selected as peripheral clock"]
    #[inline(always)]
    pub fn is_pll2_p(&self) -> bool {
        *self == LPTIM1SEL::Pll2P
    }
    #[doc = "pll3_r selected as peripheral clock"]
    #[inline(always)]
    pub fn is_pll3_r(&self) -> bool {
        *self == LPTIM1SEL::Pll3R
    }
    #[doc = "LSE selected as peripheral clock"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == LPTIM1SEL::Lse
    }
    #[doc = "LSI selected as peripheral clock"]
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == LPTIM1SEL::Lsi
    }
    #[doc = "PER selected as peripheral clock"]
    #[inline(always)]
    pub fn is_per(&self) -> bool {
        *self == LPTIM1SEL::Per
    }
}
#[doc = "Field `LPTIM1SEL` writer - LPTIM1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
pub type LPTIM1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, LPTIM1SEL>;
impl<'a, REG> LPTIM1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "rcc_pclk1 selected as peripheral clock"]
    #[inline(always)]
    pub fn rcc_pclk1(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM1SEL::RccPclk1)
    }
    #[doc = "pll2_p selected as peripheral clock"]
    #[inline(always)]
    pub fn pll2_p(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM1SEL::Pll2P)
    }
    #[doc = "pll3_r selected as peripheral clock"]
    #[inline(always)]
    pub fn pll3_r(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM1SEL::Pll3R)
    }
    #[doc = "LSE selected as peripheral clock"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM1SEL::Lse)
    }
    #[doc = "LSI selected as peripheral clock"]
    #[inline(always)]
    pub fn lsi(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM1SEL::Lsi)
    }
    #[doc = "PER selected as peripheral clock"]
    #[inline(always)]
    pub fn per(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM1SEL::Per)
    }
}
impl R {
    #[doc = "Bits 0:2 - USART2/3, UART4,5, 7 and 8 (APB1) kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn usart234578sel(&self) -> USART234578SEL_R {
        USART234578SEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - USART1, 6, 9 and 10 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn usart16910sel(&self) -> USART16910SEL_R {
        USART16910SEL_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 8:9 - RNG kernel clock source selection Set and reset by software."]
    #[inline(always)]
    pub fn rngsel(&self) -> RNGSEL_R {
        RNGSEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - I2C1,2,3 kernel clock source selection Set and reset by software."]
    #[inline(always)]
    pub fn i2c123sel(&self) -> I2C123SEL_R {
        I2C123SEL_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 20:21 - USBOTG 1 and 2 kernel clock source selection Set and reset by software."]
    #[inline(always)]
    pub fn usbsel(&self) -> USBSEL_R {
        USBSEL_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - HDMI-CEC kernel clock source selection Set and reset by software."]
    #[inline(always)]
    pub fn cecsel(&self) -> CECSEL_R {
        CECSEL_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 28:30 - LPTIM1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn lptim1sel(&self) -> LPTIM1SEL_R {
        LPTIM1SEL_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - USART2/3, UART4,5, 7 and 8 (APB1) kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn usart234578sel(&mut self) -> USART234578SEL_W<CDCCIP2Rrs> {
        USART234578SEL_W::new(self, 0)
    }
    #[doc = "Bits 3:5 - USART1, 6, 9 and 10 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn usart16910sel(&mut self) -> USART16910SEL_W<CDCCIP2Rrs> {
        USART16910SEL_W::new(self, 3)
    }
    #[doc = "Bits 8:9 - RNG kernel clock source selection Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn rngsel(&mut self) -> RNGSEL_W<CDCCIP2Rrs> {
        RNGSEL_W::new(self, 8)
    }
    #[doc = "Bits 12:13 - I2C1,2,3 kernel clock source selection Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn i2c123sel(&mut self) -> I2C123SEL_W<CDCCIP2Rrs> {
        I2C123SEL_W::new(self, 12)
    }
    #[doc = "Bits 20:21 - USBOTG 1 and 2 kernel clock source selection Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn usbsel(&mut self) -> USBSEL_W<CDCCIP2Rrs> {
        USBSEL_W::new(self, 20)
    }
    #[doc = "Bits 22:23 - HDMI-CEC kernel clock source selection Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn cecsel(&mut self) -> CECSEL_W<CDCCIP2Rrs> {
        CECSEL_W::new(self, 22)
    }
    #[doc = "Bits 28:30 - LPTIM1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn lptim1sel(&mut self) -> LPTIM1SEL_W<CDCCIP2Rrs> {
        LPTIM1SEL_W::new(self, 28)
    }
}
#[doc = "RCC CPU domain kernel clock configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdccip2r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cdccip2r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CDCCIP2Rrs;
impl crate::RegisterSpec for CDCCIP2Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cdccip2r::R`](R) reader structure"]
impl crate::Readable for CDCCIP2Rrs {}
#[doc = "`write(|w| ..)` method takes [`cdccip2r::W`](W) writer structure"]
impl crate::Writable for CDCCIP2Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CDCCIP2R to value 0"]
impl crate::Resettable for CDCCIP2Rrs {
    const RESET_VALUE: u32 = 0;
}
