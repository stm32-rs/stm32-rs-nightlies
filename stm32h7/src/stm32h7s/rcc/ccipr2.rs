///Register `CCIPR2` reader
pub type R = crate::R<CCIPR2rs>;
///Register `CCIPR2` writer
pub type W = crate::W<CCIPR2rs>;
/**USART2,3, UART4,5,7,8 (APB1) kernel clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UART234578SEL {
    ///0: pclk1 selected as clock
    Pclk1 = 0,
    ///1: pll2_q selected as clock
    Pll2Q = 1,
    ///2: pll3_q selected as clock
    Pll3Q = 2,
    ///3: hsi_ker selected as clock
    HsiKer = 3,
    ///4: csi_ker selected as clock
    CsiKer = 4,
    ///5: lse selected as clock
    Lse = 5,
}
impl From<UART234578SEL> for u8 {
    #[inline(always)]
    fn from(variant: UART234578SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for UART234578SEL {
    type Ux = u8;
}
impl crate::IsEnum for UART234578SEL {}
///Field `UART234578SEL` reader - USART2,3, UART4,5,7,8 (APB1) kernel clock source selection
pub type UART234578SEL_R = crate::FieldReader<UART234578SEL>;
impl UART234578SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<UART234578SEL> {
        match self.bits {
            0 => Some(UART234578SEL::Pclk1),
            1 => Some(UART234578SEL::Pll2Q),
            2 => Some(UART234578SEL::Pll3Q),
            3 => Some(UART234578SEL::HsiKer),
            4 => Some(UART234578SEL::CsiKer),
            5 => Some(UART234578SEL::Lse),
            _ => None,
        }
    }
    ///pclk1 selected as clock
    #[inline(always)]
    pub fn is_pclk1(&self) -> bool {
        *self == UART234578SEL::Pclk1
    }
    ///pll2_q selected as clock
    #[inline(always)]
    pub fn is_pll2_q(&self) -> bool {
        *self == UART234578SEL::Pll2Q
    }
    ///pll3_q selected as clock
    #[inline(always)]
    pub fn is_pll3_q(&self) -> bool {
        *self == UART234578SEL::Pll3Q
    }
    ///hsi_ker selected as clock
    #[inline(always)]
    pub fn is_hsi_ker(&self) -> bool {
        *self == UART234578SEL::HsiKer
    }
    ///csi_ker selected as clock
    #[inline(always)]
    pub fn is_csi_ker(&self) -> bool {
        *self == UART234578SEL::CsiKer
    }
    ///lse selected as clock
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == UART234578SEL::Lse
    }
}
///Field `UART234578SEL` writer - USART2,3, UART4,5,7,8 (APB1) kernel clock source selection
pub type UART234578SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, UART234578SEL>;
impl<'a, REG> UART234578SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///pclk1 selected as clock
    #[inline(always)]
    pub fn pclk1(self) -> &'a mut crate::W<REG> {
        self.variant(UART234578SEL::Pclk1)
    }
    ///pll2_q selected as clock
    #[inline(always)]
    pub fn pll2_q(self) -> &'a mut crate::W<REG> {
        self.variant(UART234578SEL::Pll2Q)
    }
    ///pll3_q selected as clock
    #[inline(always)]
    pub fn pll3_q(self) -> &'a mut crate::W<REG> {
        self.variant(UART234578SEL::Pll3Q)
    }
    ///hsi_ker selected as clock
    #[inline(always)]
    pub fn hsi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(UART234578SEL::HsiKer)
    }
    ///csi_ker selected as clock
    #[inline(always)]
    pub fn csi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(UART234578SEL::CsiKer)
    }
    ///lse selected as clock
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(UART234578SEL::Lse)
    }
}
/**SPI/I2S2 and SPI/I2S3 kernel clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPI23SEL {
    ///0: pll1_q selected as clock
    Pll1Q = 0,
    ///1: pll2_p selected as clock
    Pll2P = 1,
    ///2: pll3_p selected as clock
    Pll3P = 2,
    ///3: I2S_CKIN selected as clock
    I2sCkin = 3,
    ///4: per selected as clock
    Per = 4,
}
impl From<SPI23SEL> for u8 {
    #[inline(always)]
    fn from(variant: SPI23SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SPI23SEL {
    type Ux = u8;
}
impl crate::IsEnum for SPI23SEL {}
///Field `SPI23SEL` reader - SPI/I2S2 and SPI/I2S3 kernel clock source selection
pub type SPI23SEL_R = crate::FieldReader<SPI23SEL>;
impl SPI23SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<SPI23SEL> {
        match self.bits {
            0 => Some(SPI23SEL::Pll1Q),
            1 => Some(SPI23SEL::Pll2P),
            2 => Some(SPI23SEL::Pll3P),
            3 => Some(SPI23SEL::I2sCkin),
            4 => Some(SPI23SEL::Per),
            _ => None,
        }
    }
    ///pll1_q selected as clock
    #[inline(always)]
    pub fn is_pll1_q(&self) -> bool {
        *self == SPI23SEL::Pll1Q
    }
    ///pll2_p selected as clock
    #[inline(always)]
    pub fn is_pll2_p(&self) -> bool {
        *self == SPI23SEL::Pll2P
    }
    ///pll3_p selected as clock
    #[inline(always)]
    pub fn is_pll3_p(&self) -> bool {
        *self == SPI23SEL::Pll3P
    }
    ///I2S_CKIN selected as clock
    #[inline(always)]
    pub fn is_i2s_ckin(&self) -> bool {
        *self == SPI23SEL::I2sCkin
    }
    ///per selected as clock
    #[inline(always)]
    pub fn is_per(&self) -> bool {
        *self == SPI23SEL::Per
    }
}
///Field `SPI23SEL` writer - SPI/I2S2 and SPI/I2S3 kernel clock source selection
pub type SPI23SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SPI23SEL>;
impl<'a, REG> SPI23SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///pll1_q selected as clock
    #[inline(always)]
    pub fn pll1_q(self) -> &'a mut crate::W<REG> {
        self.variant(SPI23SEL::Pll1Q)
    }
    ///pll2_p selected as clock
    #[inline(always)]
    pub fn pll2_p(self) -> &'a mut crate::W<REG> {
        self.variant(SPI23SEL::Pll2P)
    }
    ///pll3_p selected as clock
    #[inline(always)]
    pub fn pll3_p(self) -> &'a mut crate::W<REG> {
        self.variant(SPI23SEL::Pll3P)
    }
    ///I2S_CKIN selected as clock
    #[inline(always)]
    pub fn i2s_ckin(self) -> &'a mut crate::W<REG> {
        self.variant(SPI23SEL::I2sCkin)
    }
    ///per selected as clock
    #[inline(always)]
    pub fn per(self) -> &'a mut crate::W<REG> {
        self.variant(SPI23SEL::Per)
    }
}
/**I2C2, I2C3 kernel clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2C23SEL {
    ///0: pclk1 selected as clock
    Pclk1 = 0,
    ///1: pll3_r selected as clock
    Pll3R = 1,
    ///2: hsi_ker selected as clock
    HsiKer = 2,
    ///3: csi_ker selected as clock
    CsiKer = 3,
}
impl From<I2C23SEL> for u8 {
    #[inline(always)]
    fn from(variant: I2C23SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for I2C23SEL {
    type Ux = u8;
}
impl crate::IsEnum for I2C23SEL {}
///Field `I2C23SEL` reader - I2C2, I2C3 kernel clock source selection
pub type I2C23SEL_R = crate::FieldReader<I2C23SEL>;
impl I2C23SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> I2C23SEL {
        match self.bits {
            0 => I2C23SEL::Pclk1,
            1 => I2C23SEL::Pll3R,
            2 => I2C23SEL::HsiKer,
            3 => I2C23SEL::CsiKer,
            _ => unreachable!(),
        }
    }
    ///pclk1 selected as clock
    #[inline(always)]
    pub fn is_pclk1(&self) -> bool {
        *self == I2C23SEL::Pclk1
    }
    ///pll3_r selected as clock
    #[inline(always)]
    pub fn is_pll3_r(&self) -> bool {
        *self == I2C23SEL::Pll3R
    }
    ///hsi_ker selected as clock
    #[inline(always)]
    pub fn is_hsi_ker(&self) -> bool {
        *self == I2C23SEL::HsiKer
    }
    ///csi_ker selected as clock
    #[inline(always)]
    pub fn is_csi_ker(&self) -> bool {
        *self == I2C23SEL::CsiKer
    }
}
///Field `I2C23SEL` writer - I2C2, I2C3 kernel clock source selection
pub type I2C23SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, I2C23SEL, crate::Safe>;
impl<'a, REG> I2C23SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///pclk1 selected as clock
    #[inline(always)]
    pub fn pclk1(self) -> &'a mut crate::W<REG> {
        self.variant(I2C23SEL::Pclk1)
    }
    ///pll3_r selected as clock
    #[inline(always)]
    pub fn pll3_r(self) -> &'a mut crate::W<REG> {
        self.variant(I2C23SEL::Pll3R)
    }
    ///hsi_ker selected as clock
    #[inline(always)]
    pub fn hsi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(I2C23SEL::HsiKer)
    }
    ///csi_ker selected as clock
    #[inline(always)]
    pub fn csi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(I2C23SEL::CsiKer)
    }
}
/**I2C1 or I3C1 kernel clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2C1I3C1SEL {
    ///0: pclk1 selected as peripheral clock
    Pclk1 = 0,
    ///1: pll3_r selected as peripheral clock
    Pll3R = 1,
    ///2: hsi_ker selected as peripheral clock
    HsiKer = 2,
    ///3: csi_ker selected as peripheral clock
    CsiKer = 3,
}
impl From<I2C1I3C1SEL> for u8 {
    #[inline(always)]
    fn from(variant: I2C1I3C1SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for I2C1I3C1SEL {
    type Ux = u8;
}
impl crate::IsEnum for I2C1I3C1SEL {}
///Field `I2C1I3C1SEL` reader - I2C1 or I3C1 kernel clock source selection
pub type I2C1I3C1SEL_R = crate::FieldReader<I2C1I3C1SEL>;
impl I2C1I3C1SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> I2C1I3C1SEL {
        match self.bits {
            0 => I2C1I3C1SEL::Pclk1,
            1 => I2C1I3C1SEL::Pll3R,
            2 => I2C1I3C1SEL::HsiKer,
            3 => I2C1I3C1SEL::CsiKer,
            _ => unreachable!(),
        }
    }
    ///pclk1 selected as peripheral clock
    #[inline(always)]
    pub fn is_pclk1(&self) -> bool {
        *self == I2C1I3C1SEL::Pclk1
    }
    ///pll3_r selected as peripheral clock
    #[inline(always)]
    pub fn is_pll3_r(&self) -> bool {
        *self == I2C1I3C1SEL::Pll3R
    }
    ///hsi_ker selected as peripheral clock
    #[inline(always)]
    pub fn is_hsi_ker(&self) -> bool {
        *self == I2C1I3C1SEL::HsiKer
    }
    ///csi_ker selected as peripheral clock
    #[inline(always)]
    pub fn is_csi_ker(&self) -> bool {
        *self == I2C1I3C1SEL::CsiKer
    }
}
///Field `I2C1I3C1SEL` writer - I2C1 or I3C1 kernel clock source selection
pub type I2C1I3C1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, I2C1I3C1SEL, crate::Safe>;
impl<'a, REG> I2C1I3C1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///pclk1 selected as peripheral clock
    #[inline(always)]
    pub fn pclk1(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1I3C1SEL::Pclk1)
    }
    ///pll3_r selected as peripheral clock
    #[inline(always)]
    pub fn pll3_r(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1I3C1SEL::Pll3R)
    }
    ///hsi_ker selected as peripheral clock
    #[inline(always)]
    pub fn hsi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1I3C1SEL::HsiKer)
    }
    ///csi_ker selected as peripheral clock
    #[inline(always)]
    pub fn csi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1I3C1SEL::CsiKer)
    }
}
/**LPTIM1 kernel clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LPTIM1SEL {
    ///0: pclk1 selected as peripheral clock
    Pclk1 = 0,
    ///1: pll2_p selected as peripheral clock
    Pll2P = 1,
    ///2: pll3_r selected as peripheral clock
    Pll3R = 2,
    ///3: lse selected as peripheral clock
    Lse = 3,
    ///4: lsi selected as peripheral clock
    Lsi = 4,
    ///5: per selected as peripheral clock
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
            0 => Some(LPTIM1SEL::Pclk1),
            1 => Some(LPTIM1SEL::Pll2P),
            2 => Some(LPTIM1SEL::Pll3R),
            3 => Some(LPTIM1SEL::Lse),
            4 => Some(LPTIM1SEL::Lsi),
            5 => Some(LPTIM1SEL::Per),
            _ => None,
        }
    }
    ///pclk1 selected as peripheral clock
    #[inline(always)]
    pub fn is_pclk1(&self) -> bool {
        *self == LPTIM1SEL::Pclk1
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
    ///lse selected as peripheral clock
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == LPTIM1SEL::Lse
    }
    ///lsi selected as peripheral clock
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == LPTIM1SEL::Lsi
    }
    ///per selected as peripheral clock
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
    ///pclk1 selected as peripheral clock
    #[inline(always)]
    pub fn pclk1(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM1SEL::Pclk1)
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
    ///lse selected as peripheral clock
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM1SEL::Lse)
    }
    ///lsi selected as peripheral clock
    #[inline(always)]
    pub fn lsi(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM1SEL::Lsi)
    }
    ///per selected as peripheral clock
    #[inline(always)]
    pub fn per(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM1SEL::Per)
    }
}
/**FDCAN kernel clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FDCANSEL {
    ///0: hse_ker selected as clock
    HseKer = 0,
    ///1: pll1_q selected as clock
    Pll1Q = 1,
    ///2: pll2_p selected as clock
    Pll2P = 2,
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
///Field `FDCANSEL` reader - FDCAN kernel clock source selection
pub type FDCANSEL_R = crate::FieldReader<FDCANSEL>;
impl FDCANSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<FDCANSEL> {
        match self.bits {
            0 => Some(FDCANSEL::HseKer),
            1 => Some(FDCANSEL::Pll1Q),
            2 => Some(FDCANSEL::Pll2P),
            _ => None,
        }
    }
    ///hse_ker selected as clock
    #[inline(always)]
    pub fn is_hse_ker(&self) -> bool {
        *self == FDCANSEL::HseKer
    }
    ///pll1_q selected as clock
    #[inline(always)]
    pub fn is_pll1_q(&self) -> bool {
        *self == FDCANSEL::Pll1Q
    }
    ///pll2_p selected as clock
    #[inline(always)]
    pub fn is_pll2_p(&self) -> bool {
        *self == FDCANSEL::Pll2P
    }
}
///Field `FDCANSEL` writer - FDCAN kernel clock source selection
pub type FDCANSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, FDCANSEL>;
impl<'a, REG> FDCANSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///hse_ker selected as clock
    #[inline(always)]
    pub fn hse_ker(self) -> &'a mut crate::W<REG> {
        self.variant(FDCANSEL::HseKer)
    }
    ///pll1_q selected as clock
    #[inline(always)]
    pub fn pll1_q(self) -> &'a mut crate::W<REG> {
        self.variant(FDCANSEL::Pll1Q)
    }
    ///pll2_p selected as clock
    #[inline(always)]
    pub fn pll2_p(self) -> &'a mut crate::W<REG> {
        self.variant(FDCANSEL::Pll2P)
    }
}
/**SPDIFRX kernel clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPDIFRXSEL {
    ///0: pll1_q selected as clock
    Pll1Q = 0,
    ///1: pll2_r selected as clock
    Pll2R = 1,
    ///2: pll3_r selected as clock
    Pll3R = 2,
    ///3: hsi_ker selected as clock
    HsiKer = 3,
}
impl From<SPDIFRXSEL> for u8 {
    #[inline(always)]
    fn from(variant: SPDIFRXSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SPDIFRXSEL {
    type Ux = u8;
}
impl crate::IsEnum for SPDIFRXSEL {}
///Field `SPDIFRXSEL` reader - SPDIFRX kernel clock source selection
pub type SPDIFRXSEL_R = crate::FieldReader<SPDIFRXSEL>;
impl SPDIFRXSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SPDIFRXSEL {
        match self.bits {
            0 => SPDIFRXSEL::Pll1Q,
            1 => SPDIFRXSEL::Pll2R,
            2 => SPDIFRXSEL::Pll3R,
            3 => SPDIFRXSEL::HsiKer,
            _ => unreachable!(),
        }
    }
    ///pll1_q selected as clock
    #[inline(always)]
    pub fn is_pll1_q(&self) -> bool {
        *self == SPDIFRXSEL::Pll1Q
    }
    ///pll2_r selected as clock
    #[inline(always)]
    pub fn is_pll2_r(&self) -> bool {
        *self == SPDIFRXSEL::Pll2R
    }
    ///pll3_r selected as clock
    #[inline(always)]
    pub fn is_pll3_r(&self) -> bool {
        *self == SPDIFRXSEL::Pll3R
    }
    ///hsi_ker selected as clock
    #[inline(always)]
    pub fn is_hsi_ker(&self) -> bool {
        *self == SPDIFRXSEL::HsiKer
    }
}
///Field `SPDIFRXSEL` writer - SPDIFRX kernel clock source selection
pub type SPDIFRXSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SPDIFRXSEL, crate::Safe>;
impl<'a, REG> SPDIFRXSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///pll1_q selected as clock
    #[inline(always)]
    pub fn pll1_q(self) -> &'a mut crate::W<REG> {
        self.variant(SPDIFRXSEL::Pll1Q)
    }
    ///pll2_r selected as clock
    #[inline(always)]
    pub fn pll2_r(self) -> &'a mut crate::W<REG> {
        self.variant(SPDIFRXSEL::Pll2R)
    }
    ///pll3_r selected as clock
    #[inline(always)]
    pub fn pll3_r(self) -> &'a mut crate::W<REG> {
        self.variant(SPDIFRXSEL::Pll3R)
    }
    ///hsi_ker selected as clock
    #[inline(always)]
    pub fn hsi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(SPDIFRXSEL::HsiKer)
    }
}
/**HDMI-CEC kernel clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CECSEL {
    ///0: lse selected as clock
    Lse = 0,
    ///1: lsi selected as clock
    Lsi = 1,
    ///2: csi_ker divided by 122 selected as clock
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
    ///lse selected as clock
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == CECSEL::Lse
    }
    ///lsi selected as clock
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == CECSEL::Lsi
    }
    ///csi_ker divided by 122 selected as clock
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
    ///lse selected as clock
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(CECSEL::Lse)
    }
    ///lsi selected as clock
    #[inline(always)]
    pub fn lsi(self) -> &'a mut crate::W<REG> {
        self.variant(CECSEL::Lsi)
    }
    ///csi_ker divided by 122 selected as clock
    #[inline(always)]
    pub fn csi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(CECSEL::CsiKer)
    }
}
impl R {
    ///Bits 0:2 - USART2,3, UART4,5,7,8 (APB1) kernel clock source selection
    #[inline(always)]
    pub fn uart234578sel(&self) -> UART234578SEL_R {
        UART234578SEL_R::new((self.bits & 7) as u8)
    }
    ///Bits 4:6 - SPI/I2S2 and SPI/I2S3 kernel clock source selection
    #[inline(always)]
    pub fn spi23sel(&self) -> SPI23SEL_R {
        SPI23SEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 8:9 - I2C2, I2C3 kernel clock source selection
    #[inline(always)]
    pub fn i2c23sel(&self) -> I2C23SEL_R {
        I2C23SEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 12:13 - I2C1 or I3C1 kernel clock source selection
    #[inline(always)]
    pub fn i2c1i3c1sel(&self) -> I2C1I3C1SEL_R {
        I2C1I3C1SEL_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 16:18 - LPTIM1 kernel clock source selection
    #[inline(always)]
    pub fn lptim1sel(&self) -> LPTIM1SEL_R {
        LPTIM1SEL_R::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bits 22:23 - FDCAN kernel clock source selection
    #[inline(always)]
    pub fn fdcansel(&self) -> FDCANSEL_R {
        FDCANSEL_R::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bits 24:25 - SPDIFRX kernel clock source selection
    #[inline(always)]
    pub fn spdifrxsel(&self) -> SPDIFRXSEL_R {
        SPDIFRXSEL_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bits 28:29 - HDMI-CEC kernel clock source selection
    #[inline(always)]
    pub fn cecsel(&self) -> CECSEL_R {
        CECSEL_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCIPR2")
            .field("uart234578sel", &self.uart234578sel())
            .field("spi23sel", &self.spi23sel())
            .field("i2c23sel", &self.i2c23sel())
            .field("i2c1i3c1sel", &self.i2c1i3c1sel())
            .field("lptim1sel", &self.lptim1sel())
            .field("fdcansel", &self.fdcansel())
            .field("spdifrxsel", &self.spdifrxsel())
            .field("cecsel", &self.cecsel())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - USART2,3, UART4,5,7,8 (APB1) kernel clock source selection
    #[inline(always)]
    pub fn uart234578sel(&mut self) -> UART234578SEL_W<'_, CCIPR2rs> {
        UART234578SEL_W::new(self, 0)
    }
    ///Bits 4:6 - SPI/I2S2 and SPI/I2S3 kernel clock source selection
    #[inline(always)]
    pub fn spi23sel(&mut self) -> SPI23SEL_W<'_, CCIPR2rs> {
        SPI23SEL_W::new(self, 4)
    }
    ///Bits 8:9 - I2C2, I2C3 kernel clock source selection
    #[inline(always)]
    pub fn i2c23sel(&mut self) -> I2C23SEL_W<'_, CCIPR2rs> {
        I2C23SEL_W::new(self, 8)
    }
    ///Bits 12:13 - I2C1 or I3C1 kernel clock source selection
    #[inline(always)]
    pub fn i2c1i3c1sel(&mut self) -> I2C1I3C1SEL_W<'_, CCIPR2rs> {
        I2C1I3C1SEL_W::new(self, 12)
    }
    ///Bits 16:18 - LPTIM1 kernel clock source selection
    #[inline(always)]
    pub fn lptim1sel(&mut self) -> LPTIM1SEL_W<'_, CCIPR2rs> {
        LPTIM1SEL_W::new(self, 16)
    }
    ///Bits 22:23 - FDCAN kernel clock source selection
    #[inline(always)]
    pub fn fdcansel(&mut self) -> FDCANSEL_W<'_, CCIPR2rs> {
        FDCANSEL_W::new(self, 22)
    }
    ///Bits 24:25 - SPDIFRX kernel clock source selection
    #[inline(always)]
    pub fn spdifrxsel(&mut self) -> SPDIFRXSEL_W<'_, CCIPR2rs> {
        SPDIFRXSEL_W::new(self, 24)
    }
    ///Bits 28:29 - HDMI-CEC kernel clock source selection
    #[inline(always)]
    pub fn cecsel(&mut self) -> CECSEL_W<'_, CCIPR2rs> {
        CECSEL_W::new(self, 28)
    }
}
/**RCC APB1 peripherals kernel clock selection register

You can [`read`](crate::Reg::read) this register and get [`ccipr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#RCC:CCIPR2)*/
pub struct CCIPR2rs;
impl crate::RegisterSpec for CCIPR2rs {
    type Ux = u32;
}
///`read()` method returns [`ccipr2::R`](R) reader structure
impl crate::Readable for CCIPR2rs {}
///`write(|w| ..)` method takes [`ccipr2::W`](W) writer structure
impl crate::Writable for CCIPR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCIPR2 to value 0
impl crate::Resettable for CCIPR2rs {}
