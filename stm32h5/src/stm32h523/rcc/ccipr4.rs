///Register `CCIPR4` reader
pub type R = crate::R<CCIPR4rs>;
///Register `CCIPR4` writer
pub type W = crate::W<CCIPR4rs>;
/**OCTOSPI1 kernel clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OCTOSPI1SEL {
    ///0: HCLK4 selected as clock source (rcc_hclk4)
    RccHclk4 = 0,
    ///1: PLL1 Q clock selected as clock source (pll1_q_ck)
    Pll1Q = 1,
    ///2: PLL2 R clock selected as clock source (pll2_r_ck)
    Pll2R = 2,
    ///3: per_ck clock selected as clock source
    PerCk = 3,
}
impl From<OCTOSPI1SEL> for u8 {
    #[inline(always)]
    fn from(variant: OCTOSPI1SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OCTOSPI1SEL {
    type Ux = u8;
}
impl crate::IsEnum for OCTOSPI1SEL {}
///Field `OCTOSPI1SEL` reader - OCTOSPI1 kernel clock source selection
pub type OCTOSPI1SEL_R = crate::FieldReader<OCTOSPI1SEL>;
impl OCTOSPI1SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OCTOSPI1SEL {
        match self.bits {
            0 => OCTOSPI1SEL::RccHclk4,
            1 => OCTOSPI1SEL::Pll1Q,
            2 => OCTOSPI1SEL::Pll2R,
            3 => OCTOSPI1SEL::PerCk,
            _ => unreachable!(),
        }
    }
    ///HCLK4 selected as clock source (rcc_hclk4)
    #[inline(always)]
    pub fn is_rcc_hclk4(&self) -> bool {
        *self == OCTOSPI1SEL::RccHclk4
    }
    ///PLL1 Q clock selected as clock source (pll1_q_ck)
    #[inline(always)]
    pub fn is_pll1_q(&self) -> bool {
        *self == OCTOSPI1SEL::Pll1Q
    }
    ///PLL2 R clock selected as clock source (pll2_r_ck)
    #[inline(always)]
    pub fn is_pll2_r(&self) -> bool {
        *self == OCTOSPI1SEL::Pll2R
    }
    ///per_ck clock selected as clock source
    #[inline(always)]
    pub fn is_per_ck(&self) -> bool {
        *self == OCTOSPI1SEL::PerCk
    }
}
///Field `OCTOSPI1SEL` writer - OCTOSPI1 kernel clock source selection
pub type OCTOSPI1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, OCTOSPI1SEL, crate::Safe>;
impl<'a, REG> OCTOSPI1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///HCLK4 selected as clock source (rcc_hclk4)
    #[inline(always)]
    pub fn rcc_hclk4(self) -> &'a mut crate::W<REG> {
        self.variant(OCTOSPI1SEL::RccHclk4)
    }
    ///PLL1 Q clock selected as clock source (pll1_q_ck)
    #[inline(always)]
    pub fn pll1_q(self) -> &'a mut crate::W<REG> {
        self.variant(OCTOSPI1SEL::Pll1Q)
    }
    ///PLL2 R clock selected as clock source (pll2_r_ck)
    #[inline(always)]
    pub fn pll2_r(self) -> &'a mut crate::W<REG> {
        self.variant(OCTOSPI1SEL::Pll2R)
    }
    ///per_ck clock selected as clock source
    #[inline(always)]
    pub fn per_ck(self) -> &'a mut crate::W<REG> {
        self.variant(OCTOSPI1SEL::PerCk)
    }
}
/**SYSTICK clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYSTICKSEL {
    ///0: RCC HLCK divided by 8 selected as clock source (rcc_hclk / 8)
    HclkDiv8 = 0,
    ///1: LSI kernel selected as clock source (lsi_ker_ck)
    LsiKer = 1,
    ///2: LSE selected as clock source (lse_ck)
    Lse = 2,
}
impl From<SYSTICKSEL> for u8 {
    #[inline(always)]
    fn from(variant: SYSTICKSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SYSTICKSEL {
    type Ux = u8;
}
impl crate::IsEnum for SYSTICKSEL {}
///Field `SYSTICKSEL` reader - SYSTICK clock source selection
pub type SYSTICKSEL_R = crate::FieldReader<SYSTICKSEL>;
impl SYSTICKSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<SYSTICKSEL> {
        match self.bits {
            0 => Some(SYSTICKSEL::HclkDiv8),
            1 => Some(SYSTICKSEL::LsiKer),
            2 => Some(SYSTICKSEL::Lse),
            _ => None,
        }
    }
    ///RCC HLCK divided by 8 selected as clock source (rcc_hclk / 8)
    #[inline(always)]
    pub fn is_hclk_div8(&self) -> bool {
        *self == SYSTICKSEL::HclkDiv8
    }
    ///LSI kernel selected as clock source (lsi_ker_ck)
    #[inline(always)]
    pub fn is_lsi_ker(&self) -> bool {
        *self == SYSTICKSEL::LsiKer
    }
    ///LSE selected as clock source (lse_ck)
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == SYSTICKSEL::Lse
    }
}
///Field `SYSTICKSEL` writer - SYSTICK clock source selection
pub type SYSTICKSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SYSTICKSEL>;
impl<'a, REG> SYSTICKSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///RCC HLCK divided by 8 selected as clock source (rcc_hclk / 8)
    #[inline(always)]
    pub fn hclk_div8(self) -> &'a mut crate::W<REG> {
        self.variant(SYSTICKSEL::HclkDiv8)
    }
    ///LSI kernel selected as clock source (lsi_ker_ck)
    #[inline(always)]
    pub fn lsi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(SYSTICKSEL::LsiKer)
    }
    ///LSE selected as clock source (lse_ck)
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(SYSTICKSEL::Lse)
    }
}
/**USB kernel clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USBSEL {
    ///0: Disable the clock
    Disable = 0,
    ///1: PLL1 Q clock selected as clock source (pll1_q_ck)
    Pll1Q = 1,
    ///2: PLL3 Q clock selected as clock source (pll3_q_ck)
    Pll3Q = 2,
    ///3: HSI48 clock selected as clock source (hsi48_ker_ck)
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
///Field `USBSEL` reader - USB kernel clock source selection
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
    ///Disable the clock
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == USBSEL::Disable
    }
    ///PLL1 Q clock selected as clock source (pll1_q_ck)
    #[inline(always)]
    pub fn is_pll1_q(&self) -> bool {
        *self == USBSEL::Pll1Q
    }
    ///PLL3 Q clock selected as clock source (pll3_q_ck)
    #[inline(always)]
    pub fn is_pll3_q(&self) -> bool {
        *self == USBSEL::Pll3Q
    }
    ///HSI48 clock selected as clock source (hsi48_ker_ck)
    #[inline(always)]
    pub fn is_hsi48(&self) -> bool {
        *self == USBSEL::Hsi48
    }
}
///Field `USBSEL` writer - USB kernel clock source selection
pub type USBSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, USBSEL, crate::Safe>;
impl<'a, REG> USBSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Disable the clock
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(USBSEL::Disable)
    }
    ///PLL1 Q clock selected as clock source (pll1_q_ck)
    #[inline(always)]
    pub fn pll1_q(self) -> &'a mut crate::W<REG> {
        self.variant(USBSEL::Pll1Q)
    }
    ///PLL3 Q clock selected as clock source (pll3_q_ck)
    #[inline(always)]
    pub fn pll3_q(self) -> &'a mut crate::W<REG> {
        self.variant(USBSEL::Pll3Q)
    }
    ///HSI48 clock selected as clock source (hsi48_ker_ck)
    #[inline(always)]
    pub fn hsi48(self) -> &'a mut crate::W<REG> {
        self.variant(USBSEL::Hsi48)
    }
}
/**SDMMC1 kernel clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDMMCSEL {
    ///0: PLL1 Q clock selected as clock source (pll1_q_ck)
    Pll1Q = 0,
    ///1: PLL2 R clock selected as clock source (pll2_r_ck)
    Pll2R = 1,
}
impl From<SDMMCSEL> for bool {
    #[inline(always)]
    fn from(variant: SDMMCSEL) -> Self {
        variant as u8 != 0
    }
}
///Field `SDMMC1SEL` reader - SDMMC1 kernel clock source selection
pub type SDMMC1SEL_R = crate::BitReader<SDMMCSEL>;
impl SDMMC1SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SDMMCSEL {
        match self.bits {
            false => SDMMCSEL::Pll1Q,
            true => SDMMCSEL::Pll2R,
        }
    }
    ///PLL1 Q clock selected as clock source (pll1_q_ck)
    #[inline(always)]
    pub fn is_pll1_q(&self) -> bool {
        *self == SDMMCSEL::Pll1Q
    }
    ///PLL2 R clock selected as clock source (pll2_r_ck)
    #[inline(always)]
    pub fn is_pll2_r(&self) -> bool {
        *self == SDMMCSEL::Pll2R
    }
}
///Field `SDMMC1SEL` writer - SDMMC1 kernel clock source selection
pub type SDMMC1SEL_W<'a, REG> = crate::BitWriter<'a, REG, SDMMCSEL>;
impl<'a, REG> SDMMC1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PLL1 Q clock selected as clock source (pll1_q_ck)
    #[inline(always)]
    pub fn pll1_q(self) -> &'a mut crate::W<REG> {
        self.variant(SDMMCSEL::Pll1Q)
    }
    ///PLL2 R clock selected as clock source (pll2_r_ck)
    #[inline(always)]
    pub fn pll2_r(self) -> &'a mut crate::W<REG> {
        self.variant(SDMMCSEL::Pll2R)
    }
}
/**I2C1 kernel clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2CSEL {
    ///0: Peripheral bus clock used as selected as clock source (rcc_pclk_x)
    Pclk = 0,
    ///1: PLL3 R Clock selected as clock source (pll3_r_ck)
    Pll3R = 1,
    ///2: HSI kernel clock selected as clock source (hsi_ker_ck)
    HsiKer = 2,
    ///3: CSI kernel clock selected as clock source (csi_ker_ck)
    CsiKer = 3,
}
impl From<I2CSEL> for u8 {
    #[inline(always)]
    fn from(variant: I2CSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for I2CSEL {
    type Ux = u8;
}
impl crate::IsEnum for I2CSEL {}
///Field `I2C1SEL` reader - I2C1 kernel clock source selection
pub type I2C1SEL_R = crate::FieldReader<I2CSEL>;
impl I2C1SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> I2CSEL {
        match self.bits {
            0 => I2CSEL::Pclk,
            1 => I2CSEL::Pll3R,
            2 => I2CSEL::HsiKer,
            3 => I2CSEL::CsiKer,
            _ => unreachable!(),
        }
    }
    ///Peripheral bus clock used as selected as clock source (rcc_pclk_x)
    #[inline(always)]
    pub fn is_pclk(&self) -> bool {
        *self == I2CSEL::Pclk
    }
    ///PLL3 R Clock selected as clock source (pll3_r_ck)
    #[inline(always)]
    pub fn is_pll3_r(&self) -> bool {
        *self == I2CSEL::Pll3R
    }
    ///HSI kernel clock selected as clock source (hsi_ker_ck)
    #[inline(always)]
    pub fn is_hsi_ker(&self) -> bool {
        *self == I2CSEL::HsiKer
    }
    ///CSI kernel clock selected as clock source (csi_ker_ck)
    #[inline(always)]
    pub fn is_csi_ker(&self) -> bool {
        *self == I2CSEL::CsiKer
    }
}
///Field `I2C1SEL` writer - I2C1 kernel clock source selection
pub type I2C1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, I2CSEL, crate::Safe>;
impl<'a, REG> I2C1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Peripheral bus clock used as selected as clock source (rcc_pclk_x)
    #[inline(always)]
    pub fn pclk(self) -> &'a mut crate::W<REG> {
        self.variant(I2CSEL::Pclk)
    }
    ///PLL3 R Clock selected as clock source (pll3_r_ck)
    #[inline(always)]
    pub fn pll3_r(self) -> &'a mut crate::W<REG> {
        self.variant(I2CSEL::Pll3R)
    }
    ///HSI kernel clock selected as clock source (hsi_ker_ck)
    #[inline(always)]
    pub fn hsi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(I2CSEL::HsiKer)
    }
    ///CSI kernel clock selected as clock source (csi_ker_ck)
    #[inline(always)]
    pub fn csi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(I2CSEL::CsiKer)
    }
}
///Field `I2C2SEL` reader - I2C2 kernel clock source selection
pub use I2C1SEL_R as I2C2SEL_R;
///Field `I2C3SEL` reader - I2C3 kernel clock source selection
pub use I2C1SEL_R as I2C3SEL_R;
///Field `I2C2SEL` writer - I2C2 kernel clock source selection
pub use I2C1SEL_W as I2C2SEL_W;
///Field `I2C3SEL` writer - I2C3 kernel clock source selection
pub use I2C1SEL_W as I2C3SEL_W;
/**I3C1 kernel clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I3CSEL {
    ///0: Peripheral bus clock used as selected as clock source (rcc_pclk_x)
    Pclk = 0,
    ///1: PLL3 R clock selected as clock source (pll3_r_ck)
    Pll3R = 1,
    ///2: HSI kernel clock selected as clock source (hsi_ker_ck)
    HsiKer = 2,
}
impl From<I3CSEL> for u8 {
    #[inline(always)]
    fn from(variant: I3CSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for I3CSEL {
    type Ux = u8;
}
impl crate::IsEnum for I3CSEL {}
///Field `I3C1SEL` reader - I3C1 kernel clock source selection
pub type I3C1SEL_R = crate::FieldReader<I3CSEL>;
impl I3C1SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<I3CSEL> {
        match self.bits {
            0 => Some(I3CSEL::Pclk),
            1 => Some(I3CSEL::Pll3R),
            2 => Some(I3CSEL::HsiKer),
            _ => None,
        }
    }
    ///Peripheral bus clock used as selected as clock source (rcc_pclk_x)
    #[inline(always)]
    pub fn is_pclk(&self) -> bool {
        *self == I3CSEL::Pclk
    }
    ///PLL3 R clock selected as clock source (pll3_r_ck)
    #[inline(always)]
    pub fn is_pll3_r(&self) -> bool {
        *self == I3CSEL::Pll3R
    }
    ///HSI kernel clock selected as clock source (hsi_ker_ck)
    #[inline(always)]
    pub fn is_hsi_ker(&self) -> bool {
        *self == I3CSEL::HsiKer
    }
}
///Field `I3C1SEL` writer - I3C1 kernel clock source selection
pub type I3C1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, I3CSEL>;
impl<'a, REG> I3C1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Peripheral bus clock used as selected as clock source (rcc_pclk_x)
    #[inline(always)]
    pub fn pclk(self) -> &'a mut crate::W<REG> {
        self.variant(I3CSEL::Pclk)
    }
    ///PLL3 R clock selected as clock source (pll3_r_ck)
    #[inline(always)]
    pub fn pll3_r(self) -> &'a mut crate::W<REG> {
        self.variant(I3CSEL::Pll3R)
    }
    ///HSI kernel clock selected as clock source (hsi_ker_ck)
    #[inline(always)]
    pub fn hsi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(I3CSEL::HsiKer)
    }
}
///Field `I3C2SEL` reader - I3C2 kernel clock source selection
pub use I3C1SEL_R as I3C2SEL_R;
///Field `I3C2SEL` writer - I3C2 kernel clock source selection
pub use I3C1SEL_W as I3C2SEL_W;
impl R {
    ///Bits 0:1 - OCTOSPI1 kernel clock source selection
    #[inline(always)]
    pub fn octospi1sel(&self) -> OCTOSPI1SEL_R {
        OCTOSPI1SEL_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - SYSTICK clock source selection
    #[inline(always)]
    pub fn systicksel(&self) -> SYSTICKSEL_R {
        SYSTICKSEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - USB kernel clock source selection
    #[inline(always)]
    pub fn usbsel(&self) -> USBSEL_R {
        USBSEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 6 - SDMMC1 kernel clock source selection
    #[inline(always)]
    pub fn sdmmc1sel(&self) -> SDMMC1SEL_R {
        SDMMC1SEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bits 16:17 - I2C1 kernel clock source selection
    #[inline(always)]
    pub fn i2c1sel(&self) -> I2C1SEL_R {
        I2C1SEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:19 - I2C2 kernel clock source selection
    #[inline(always)]
    pub fn i2c2sel(&self) -> I2C2SEL_R {
        I2C2SEL_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bits 20:21 - I2C3 kernel clock source selection
    #[inline(always)]
    pub fn i2c3sel(&self) -> I2C3SEL_R {
        I2C3SEL_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bits 24:25 - I3C1 kernel clock source selection
    #[inline(always)]
    pub fn i3c1sel(&self) -> I3C1SEL_R {
        I3C1SEL_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bits 26:27 - I3C2 kernel clock source selection
    #[inline(always)]
    pub fn i3c2sel(&self) -> I3C2SEL_R {
        I3C2SEL_R::new(((self.bits >> 26) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCIPR4")
            .field("octospi1sel", &self.octospi1sel())
            .field("systicksel", &self.systicksel())
            .field("usbsel", &self.usbsel())
            .field("sdmmc1sel", &self.sdmmc1sel())
            .field("i2c1sel", &self.i2c1sel())
            .field("i2c2sel", &self.i2c2sel())
            .field("i2c3sel", &self.i2c3sel())
            .field("i3c1sel", &self.i3c1sel())
            .field("i3c2sel", &self.i3c2sel())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - OCTOSPI1 kernel clock source selection
    #[inline(always)]
    pub fn octospi1sel(&mut self) -> OCTOSPI1SEL_W<'_, CCIPR4rs> {
        OCTOSPI1SEL_W::new(self, 0)
    }
    ///Bits 2:3 - SYSTICK clock source selection
    #[inline(always)]
    pub fn systicksel(&mut self) -> SYSTICKSEL_W<'_, CCIPR4rs> {
        SYSTICKSEL_W::new(self, 2)
    }
    ///Bits 4:5 - USB kernel clock source selection
    #[inline(always)]
    pub fn usbsel(&mut self) -> USBSEL_W<'_, CCIPR4rs> {
        USBSEL_W::new(self, 4)
    }
    ///Bit 6 - SDMMC1 kernel clock source selection
    #[inline(always)]
    pub fn sdmmc1sel(&mut self) -> SDMMC1SEL_W<'_, CCIPR4rs> {
        SDMMC1SEL_W::new(self, 6)
    }
    ///Bits 16:17 - I2C1 kernel clock source selection
    #[inline(always)]
    pub fn i2c1sel(&mut self) -> I2C1SEL_W<'_, CCIPR4rs> {
        I2C1SEL_W::new(self, 16)
    }
    ///Bits 18:19 - I2C2 kernel clock source selection
    #[inline(always)]
    pub fn i2c2sel(&mut self) -> I2C2SEL_W<'_, CCIPR4rs> {
        I2C2SEL_W::new(self, 18)
    }
    ///Bits 20:21 - I2C3 kernel clock source selection
    #[inline(always)]
    pub fn i2c3sel(&mut self) -> I2C3SEL_W<'_, CCIPR4rs> {
        I2C3SEL_W::new(self, 20)
    }
    ///Bits 24:25 - I3C1 kernel clock source selection
    #[inline(always)]
    pub fn i3c1sel(&mut self) -> I3C1SEL_W<'_, CCIPR4rs> {
        I3C1SEL_W::new(self, 24)
    }
    ///Bits 26:27 - I3C2 kernel clock source selection
    #[inline(always)]
    pub fn i3c2sel(&mut self) -> I3C2SEL_W<'_, CCIPR4rs> {
        I3C2SEL_W::new(self, 26)
    }
}
/**RCC kernel clock configuration register

You can [`read`](crate::Reg::read) this register and get [`ccipr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#RCC:CCIPR4)*/
pub struct CCIPR4rs;
impl crate::RegisterSpec for CCIPR4rs {
    type Ux = u32;
}
///`read()` method returns [`ccipr4::R`](R) reader structure
impl crate::Readable for CCIPR4rs {}
///`write(|w| ..)` method takes [`ccipr4::W`](W) writer structure
impl crate::Writable for CCIPR4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCIPR4 to value 0
impl crate::Resettable for CCIPR4rs {}
