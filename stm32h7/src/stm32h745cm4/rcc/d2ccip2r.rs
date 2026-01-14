///Register `D2CCIP2R` reader
pub type R = crate::R<D2CCIP2Rrs>;
///Register `D2CCIP2R` writer
pub type W = crate::W<D2CCIP2Rrs>;
/**USART2/3, UART4,5, 7/8 (APB1) kernel clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USART234578SEL {
    ///0: rcc_pclk1 selected as peripheral clock
    RccPclk1 = 0,
    ///1: pll2_q selected as peripheral clock
    Pll2Q = 1,
    ///2: pll3_q selected as peripheral clock
    Pll3Q = 2,
    ///3: hsi_ker selected as peripheral clock
    HsiKer = 3,
    ///4: csi_ker selected as peripheral clock
    CsiKer = 4,
    ///5: LSE selected as peripheral clock
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
impl crate::IsEnum for USART234578SEL {}
///Field `USART234578SEL` reader - USART2/3, UART4,5, 7/8 (APB1) kernel clock source selection
pub type USART234578SEL_R = crate::FieldReader<USART234578SEL>;
impl USART234578SEL_R {
    ///Get enumerated values variant
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
    ///rcc_pclk1 selected as peripheral clock
    #[inline(always)]
    pub fn is_rcc_pclk1(&self) -> bool {
        *self == USART234578SEL::RccPclk1
    }
    ///pll2_q selected as peripheral clock
    #[inline(always)]
    pub fn is_pll2_q(&self) -> bool {
        *self == USART234578SEL::Pll2Q
    }
    ///pll3_q selected as peripheral clock
    #[inline(always)]
    pub fn is_pll3_q(&self) -> bool {
        *self == USART234578SEL::Pll3Q
    }
    ///hsi_ker selected as peripheral clock
    #[inline(always)]
    pub fn is_hsi_ker(&self) -> bool {
        *self == USART234578SEL::HsiKer
    }
    ///csi_ker selected as peripheral clock
    #[inline(always)]
    pub fn is_csi_ker(&self) -> bool {
        *self == USART234578SEL::CsiKer
    }
    ///LSE selected as peripheral clock
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == USART234578SEL::Lse
    }
}
///Field `USART234578SEL` writer - USART2/3, UART4,5, 7/8 (APB1) kernel clock source selection
pub type USART234578SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, USART234578SEL>;
impl<'a, REG> USART234578SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///rcc_pclk1 selected as peripheral clock
    #[inline(always)]
    pub fn rcc_pclk1(self) -> &'a mut crate::W<REG> {
        self.variant(USART234578SEL::RccPclk1)
    }
    ///pll2_q selected as peripheral clock
    #[inline(always)]
    pub fn pll2_q(self) -> &'a mut crate::W<REG> {
        self.variant(USART234578SEL::Pll2Q)
    }
    ///pll3_q selected as peripheral clock
    #[inline(always)]
    pub fn pll3_q(self) -> &'a mut crate::W<REG> {
        self.variant(USART234578SEL::Pll3Q)
    }
    ///hsi_ker selected as peripheral clock
    #[inline(always)]
    pub fn hsi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(USART234578SEL::HsiKer)
    }
    ///csi_ker selected as peripheral clock
    #[inline(always)]
    pub fn csi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(USART234578SEL::CsiKer)
    }
    ///LSE selected as peripheral clock
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(USART234578SEL::Lse)
    }
}
/**USART1 and 6 kernel clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USART16SEL {
    ///0: rcc_pclk2 selected as peripheral clock
    RccPclk2 = 0,
    ///1: pll2_q selected as peripheral clock
    Pll2Q = 1,
    ///2: pll3_q selected as peripheral clock
    Pll3Q = 2,
    ///3: hsi_ker selected as peripheral clock
    HsiKer = 3,
    ///4: csi_ker selected as peripheral clock
    CsiKer = 4,
    ///5: LSE selected as peripheral clock
    Lse = 5,
}
impl From<USART16SEL> for u8 {
    #[inline(always)]
    fn from(variant: USART16SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for USART16SEL {
    type Ux = u8;
}
impl crate::IsEnum for USART16SEL {}
///Field `USART16SEL` reader - USART1 and 6 kernel clock source selection
pub type USART16SEL_R = crate::FieldReader<USART16SEL>;
impl USART16SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<USART16SEL> {
        match self.bits {
            0 => Some(USART16SEL::RccPclk2),
            1 => Some(USART16SEL::Pll2Q),
            2 => Some(USART16SEL::Pll3Q),
            3 => Some(USART16SEL::HsiKer),
            4 => Some(USART16SEL::CsiKer),
            5 => Some(USART16SEL::Lse),
            _ => None,
        }
    }
    ///rcc_pclk2 selected as peripheral clock
    #[inline(always)]
    pub fn is_rcc_pclk2(&self) -> bool {
        *self == USART16SEL::RccPclk2
    }
    ///pll2_q selected as peripheral clock
    #[inline(always)]
    pub fn is_pll2_q(&self) -> bool {
        *self == USART16SEL::Pll2Q
    }
    ///pll3_q selected as peripheral clock
    #[inline(always)]
    pub fn is_pll3_q(&self) -> bool {
        *self == USART16SEL::Pll3Q
    }
    ///hsi_ker selected as peripheral clock
    #[inline(always)]
    pub fn is_hsi_ker(&self) -> bool {
        *self == USART16SEL::HsiKer
    }
    ///csi_ker selected as peripheral clock
    #[inline(always)]
    pub fn is_csi_ker(&self) -> bool {
        *self == USART16SEL::CsiKer
    }
    ///LSE selected as peripheral clock
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == USART16SEL::Lse
    }
}
///Field `USART16SEL` writer - USART1 and 6 kernel clock source selection
pub type USART16SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, USART16SEL>;
impl<'a, REG> USART16SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///rcc_pclk2 selected as peripheral clock
    #[inline(always)]
    pub fn rcc_pclk2(self) -> &'a mut crate::W<REG> {
        self.variant(USART16SEL::RccPclk2)
    }
    ///pll2_q selected as peripheral clock
    #[inline(always)]
    pub fn pll2_q(self) -> &'a mut crate::W<REG> {
        self.variant(USART16SEL::Pll2Q)
    }
    ///pll3_q selected as peripheral clock
    #[inline(always)]
    pub fn pll3_q(self) -> &'a mut crate::W<REG> {
        self.variant(USART16SEL::Pll3Q)
    }
    ///hsi_ker selected as peripheral clock
    #[inline(always)]
    pub fn hsi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(USART16SEL::HsiKer)
    }
    ///csi_ker selected as peripheral clock
    #[inline(always)]
    pub fn csi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(USART16SEL::CsiKer)
    }
    ///LSE selected as peripheral clock
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(USART16SEL::Lse)
    }
}
/**RNG kernel clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RNGSEL {
    ///0: HSI48 selected as peripheral clock
    Hsi48 = 0,
    ///1: pll1_q selected as peripheral clock
    Pll1Q = 1,
    ///2: LSE selected as peripheral clock
    Lse = 2,
    ///3: LSI selected as peripheral clock
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
impl crate::IsEnum for RNGSEL {}
///Field `RNGSEL` reader - RNG kernel clock source selection
pub type RNGSEL_R = crate::FieldReader<RNGSEL>;
impl RNGSEL_R {
    ///Get enumerated values variant
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
    ///HSI48 selected as peripheral clock
    #[inline(always)]
    pub fn is_hsi48(&self) -> bool {
        *self == RNGSEL::Hsi48
    }
    ///pll1_q selected as peripheral clock
    #[inline(always)]
    pub fn is_pll1_q(&self) -> bool {
        *self == RNGSEL::Pll1Q
    }
    ///LSE selected as peripheral clock
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == RNGSEL::Lse
    }
    ///LSI selected as peripheral clock
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == RNGSEL::Lsi
    }
}
///Field `RNGSEL` writer - RNG kernel clock source selection
pub type RNGSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, RNGSEL, crate::Safe>;
impl<'a, REG> RNGSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///HSI48 selected as peripheral clock
    #[inline(always)]
    pub fn hsi48(self) -> &'a mut crate::W<REG> {
        self.variant(RNGSEL::Hsi48)
    }
    ///pll1_q selected as peripheral clock
    #[inline(always)]
    pub fn pll1_q(self) -> &'a mut crate::W<REG> {
        self.variant(RNGSEL::Pll1Q)
    }
    ///LSE selected as peripheral clock
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(RNGSEL::Lse)
    }
    ///LSI selected as peripheral clock
    #[inline(always)]
    pub fn lsi(self) -> &'a mut crate::W<REG> {
        self.variant(RNGSEL::Lsi)
    }
}
/**I2C1,2,3 kernel clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2C123SEL {
    ///0: rcc_pclk1 selected as peripheral clock
    RccPclk1 = 0,
    ///1: pll3_r selected as peripheral clock
    Pll3R = 1,
    ///2: hsi_ker selected as peripheral clock
    HsiKer = 2,
    ///3: csi_ker selected as peripheral clock
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
impl crate::IsEnum for I2C123SEL {}
///Field `I2C123SEL` reader - I2C1,2,3 kernel clock source selection
pub type I2C123SEL_R = crate::FieldReader<I2C123SEL>;
impl I2C123SEL_R {
    ///Get enumerated values variant
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
    ///rcc_pclk1 selected as peripheral clock
    #[inline(always)]
    pub fn is_rcc_pclk1(&self) -> bool {
        *self == I2C123SEL::RccPclk1
    }
    ///pll3_r selected as peripheral clock
    #[inline(always)]
    pub fn is_pll3_r(&self) -> bool {
        *self == I2C123SEL::Pll3R
    }
    ///hsi_ker selected as peripheral clock
    #[inline(always)]
    pub fn is_hsi_ker(&self) -> bool {
        *self == I2C123SEL::HsiKer
    }
    ///csi_ker selected as peripheral clock
    #[inline(always)]
    pub fn is_csi_ker(&self) -> bool {
        *self == I2C123SEL::CsiKer
    }
}
///Field `I2C123SEL` writer - I2C1,2,3 kernel clock source selection
pub type I2C123SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, I2C123SEL, crate::Safe>;
impl<'a, REG> I2C123SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///rcc_pclk1 selected as peripheral clock
    #[inline(always)]
    pub fn rcc_pclk1(self) -> &'a mut crate::W<REG> {
        self.variant(I2C123SEL::RccPclk1)
    }
    ///pll3_r selected as peripheral clock
    #[inline(always)]
    pub fn pll3_r(self) -> &'a mut crate::W<REG> {
        self.variant(I2C123SEL::Pll3R)
    }
    ///hsi_ker selected as peripheral clock
    #[inline(always)]
    pub fn hsi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(I2C123SEL::HsiKer)
    }
    ///csi_ker selected as peripheral clock
    #[inline(always)]
    pub fn csi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(I2C123SEL::CsiKer)
    }
}
/**USBOTG 1 and 2 kernel clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USBSEL {
    ///0: Disable the kernel clock
    Disable = 0,
    ///1: pll1_q selected as peripheral clock
    Pll1Q = 1,
    ///2: pll3_q selected as peripheral clock
    Pll3Q = 2,
    ///3: HSI48 selected as peripheral clock
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
impl crate::IsEnum for USBSEL {}
///Field `USBSEL` reader - USBOTG 1 and 2 kernel clock source selection
pub type USBSEL_R = crate::FieldReader<USBSEL>;
impl USBSEL_R {
    ///Get enumerated values variant
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
    ///Disable the kernel clock
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == USBSEL::Disable
    }
    ///pll1_q selected as peripheral clock
    #[inline(always)]
    pub fn is_pll1_q(&self) -> bool {
        *self == USBSEL::Pll1Q
    }
    ///pll3_q selected as peripheral clock
    #[inline(always)]
    pub fn is_pll3_q(&self) -> bool {
        *self == USBSEL::Pll3Q
    }
    ///HSI48 selected as peripheral clock
    #[inline(always)]
    pub fn is_hsi48(&self) -> bool {
        *self == USBSEL::Hsi48
    }
}
///Field `USBSEL` writer - USBOTG 1 and 2 kernel clock source selection
pub type USBSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, USBSEL, crate::Safe>;
impl<'a, REG> USBSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Disable the kernel clock
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(USBSEL::Disable)
    }
    ///pll1_q selected as peripheral clock
    #[inline(always)]
    pub fn pll1_q(self) -> &'a mut crate::W<REG> {
        self.variant(USBSEL::Pll1Q)
    }
    ///pll3_q selected as peripheral clock
    #[inline(always)]
    pub fn pll3_q(self) -> &'a mut crate::W<REG> {
        self.variant(USBSEL::Pll3Q)
    }
    ///HSI48 selected as peripheral clock
    #[inline(always)]
    pub fn hsi48(self) -> &'a mut crate::W<REG> {
        self.variant(USBSEL::Hsi48)
    }
}
/**HDMI-CEC kernel clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CECSEL {
    ///0: LSE selected as peripheral clock
    Lse = 0,
    ///1: LSI selected as peripheral clock
    Lsi = 1,
    ///2: csi_ker selected as peripheral clock
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
impl crate::IsEnum for CECSEL {}
///Field `CECSEL` reader - HDMI-CEC kernel clock source selection
pub type CECSEL_R = crate::FieldReader<CECSEL>;
impl CECSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CECSEL> {
        match self.bits {
            0 => Some(CECSEL::Lse),
            1 => Some(CECSEL::Lsi),
            2 => Some(CECSEL::CsiKer),
            _ => None,
        }
    }
    ///LSE selected as peripheral clock
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == CECSEL::Lse
    }
    ///LSI selected as peripheral clock
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == CECSEL::Lsi
    }
    ///csi_ker selected as peripheral clock
    #[inline(always)]
    pub fn is_csi_ker(&self) -> bool {
        *self == CECSEL::CsiKer
    }
}
///Field `CECSEL` writer - HDMI-CEC kernel clock source selection
pub type CECSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CECSEL>;
impl<'a, REG> CECSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///LSE selected as peripheral clock
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(CECSEL::Lse)
    }
    ///LSI selected as peripheral clock
    #[inline(always)]
    pub fn lsi(self) -> &'a mut crate::W<REG> {
        self.variant(CECSEL::Lsi)
    }
    ///csi_ker selected as peripheral clock
    #[inline(always)]
    pub fn csi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(CECSEL::CsiKer)
    }
}
/**LPTIM1 kernel clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LPTIM1SEL {
    ///0: rcc_pclk1 selected as peripheral clock
    RccPclk1 = 0,
    ///1: pll2_p selected as peripheral clock
    Pll2P = 1,
    ///2: pll3_r selected as peripheral clock
    Pll3R = 2,
    ///3: LSE selected as peripheral clock
    Lse = 3,
    ///4: LSI selected as peripheral clock
    Lsi = 4,
    ///5: PER selected as peripheral clock
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
impl crate::IsEnum for LPTIM1SEL {}
///Field `LPTIM1SEL` reader - LPTIM1 kernel clock source selection
pub type LPTIM1SEL_R = crate::FieldReader<LPTIM1SEL>;
impl LPTIM1SEL_R {
    ///Get enumerated values variant
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
    ///rcc_pclk1 selected as peripheral clock
    #[inline(always)]
    pub fn is_rcc_pclk1(&self) -> bool {
        *self == LPTIM1SEL::RccPclk1
    }
    ///pll2_p selected as peripheral clock
    #[inline(always)]
    pub fn is_pll2_p(&self) -> bool {
        *self == LPTIM1SEL::Pll2P
    }
    ///pll3_r selected as peripheral clock
    #[inline(always)]
    pub fn is_pll3_r(&self) -> bool {
        *self == LPTIM1SEL::Pll3R
    }
    ///LSE selected as peripheral clock
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == LPTIM1SEL::Lse
    }
    ///LSI selected as peripheral clock
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == LPTIM1SEL::Lsi
    }
    ///PER selected as peripheral clock
    #[inline(always)]
    pub fn is_per(&self) -> bool {
        *self == LPTIM1SEL::Per
    }
}
///Field `LPTIM1SEL` writer - LPTIM1 kernel clock source selection
pub type LPTIM1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, LPTIM1SEL>;
impl<'a, REG> LPTIM1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///rcc_pclk1 selected as peripheral clock
    #[inline(always)]
    pub fn rcc_pclk1(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM1SEL::RccPclk1)
    }
    ///pll2_p selected as peripheral clock
    #[inline(always)]
    pub fn pll2_p(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM1SEL::Pll2P)
    }
    ///pll3_r selected as peripheral clock
    #[inline(always)]
    pub fn pll3_r(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM1SEL::Pll3R)
    }
    ///LSE selected as peripheral clock
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM1SEL::Lse)
    }
    ///LSI selected as peripheral clock
    #[inline(always)]
    pub fn lsi(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM1SEL::Lsi)
    }
    ///PER selected as peripheral clock
    #[inline(always)]
    pub fn per(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM1SEL::Per)
    }
}
impl R {
    ///Bits 0:2 - USART2/3, UART4,5, 7/8 (APB1) kernel clock source selection
    #[inline(always)]
    pub fn usart234578sel(&self) -> USART234578SEL_R {
        USART234578SEL_R::new((self.bits & 7) as u8)
    }
    ///Bits 3:5 - USART1 and 6 kernel clock source selection
    #[inline(always)]
    pub fn usart16sel(&self) -> USART16SEL_R {
        USART16SEL_R::new(((self.bits >> 3) & 7) as u8)
    }
    ///Bits 8:9 - RNG kernel clock source selection
    #[inline(always)]
    pub fn rngsel(&self) -> RNGSEL_R {
        RNGSEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 12:13 - I2C1,2,3 kernel clock source selection
    #[inline(always)]
    pub fn i2c123sel(&self) -> I2C123SEL_R {
        I2C123SEL_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 20:21 - USBOTG 1 and 2 kernel clock source selection
    #[inline(always)]
    pub fn usbsel(&self) -> USBSEL_R {
        USBSEL_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bits 22:23 - HDMI-CEC kernel clock source selection
    #[inline(always)]
    pub fn cecsel(&self) -> CECSEL_R {
        CECSEL_R::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bits 28:30 - LPTIM1 kernel clock source selection
    #[inline(always)]
    pub fn lptim1sel(&self) -> LPTIM1SEL_R {
        LPTIM1SEL_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("D2CCIP2R")
            .field("usart234578sel", &self.usart234578sel())
            .field("usart16sel", &self.usart16sel())
            .field("rngsel", &self.rngsel())
            .field("i2c123sel", &self.i2c123sel())
            .field("usbsel", &self.usbsel())
            .field("cecsel", &self.cecsel())
            .field("lptim1sel", &self.lptim1sel())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - USART2/3, UART4,5, 7/8 (APB1) kernel clock source selection
    #[inline(always)]
    pub fn usart234578sel(&mut self) -> USART234578SEL_W<'_, D2CCIP2Rrs> {
        USART234578SEL_W::new(self, 0)
    }
    ///Bits 3:5 - USART1 and 6 kernel clock source selection
    #[inline(always)]
    pub fn usart16sel(&mut self) -> USART16SEL_W<'_, D2CCIP2Rrs> {
        USART16SEL_W::new(self, 3)
    }
    ///Bits 8:9 - RNG kernel clock source selection
    #[inline(always)]
    pub fn rngsel(&mut self) -> RNGSEL_W<'_, D2CCIP2Rrs> {
        RNGSEL_W::new(self, 8)
    }
    ///Bits 12:13 - I2C1,2,3 kernel clock source selection
    #[inline(always)]
    pub fn i2c123sel(&mut self) -> I2C123SEL_W<'_, D2CCIP2Rrs> {
        I2C123SEL_W::new(self, 12)
    }
    ///Bits 20:21 - USBOTG 1 and 2 kernel clock source selection
    #[inline(always)]
    pub fn usbsel(&mut self) -> USBSEL_W<'_, D2CCIP2Rrs> {
        USBSEL_W::new(self, 20)
    }
    ///Bits 22:23 - HDMI-CEC kernel clock source selection
    #[inline(always)]
    pub fn cecsel(&mut self) -> CECSEL_W<'_, D2CCIP2Rrs> {
        CECSEL_W::new(self, 22)
    }
    ///Bits 28:30 - LPTIM1 kernel clock source selection
    #[inline(always)]
    pub fn lptim1sel(&mut self) -> LPTIM1SEL_W<'_, D2CCIP2Rrs> {
        LPTIM1SEL_W::new(self, 28)
    }
}
/**RCC Domain 2 Kernel Clock Configuration Register

You can [`read`](crate::Reg::read) this register and get [`d2ccip2r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d2ccip2r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H745_CM4.html#RCC:D2CCIP2R)*/
pub struct D2CCIP2Rrs;
impl crate::RegisterSpec for D2CCIP2Rrs {
    type Ux = u32;
}
///`read()` method returns [`d2ccip2r::R`](R) reader structure
impl crate::Readable for D2CCIP2Rrs {}
///`write(|w| ..)` method takes [`d2ccip2r::W`](W) writer structure
impl crate::Writable for D2CCIP2Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets D2CCIP2R to value 0
impl crate::Resettable for D2CCIP2Rrs {}
