#[doc = "Register `CCIPR4` reader"]
pub type R = crate::R<CCIPR4rs>;
#[doc = "Register `CCIPR4` writer"]
pub type W = crate::W<CCIPR4rs>;
#[doc = "SYSTICK clock source selection Note: rcc_hclk frequency must be four times higher than lsi_ker_ck/lse_ck (period (LSI/LSE) ≥ 4 * period (HCLK).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYSTICKSEL {
    #[doc = "0: RCC HLCK divided by 8 selected as clock source (rcc_hclk / 8)"]
    HclkDiv8 = 0,
    #[doc = "1: LSI kernel selected as clock source (lsi_ker_ck)"]
    LsiKer = 1,
    #[doc = "2: LSE selected as clock source (lse_ck)"]
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
#[doc = "Field `SYSTICKSEL` reader - SYSTICK clock source selection Note: rcc_hclk frequency must be four times higher than lsi_ker_ck/lse_ck (period (LSI/LSE) ≥ 4 * period (HCLK)."]
pub type SYSTICKSEL_R = crate::FieldReader<SYSTICKSEL>;
impl SYSTICKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SYSTICKSEL> {
        match self.bits {
            0 => Some(SYSTICKSEL::HclkDiv8),
            1 => Some(SYSTICKSEL::LsiKer),
            2 => Some(SYSTICKSEL::Lse),
            _ => None,
        }
    }
    #[doc = "RCC HLCK divided by 8 selected as clock source (rcc_hclk / 8)"]
    #[inline(always)]
    pub fn is_hclk_div8(&self) -> bool {
        *self == SYSTICKSEL::HclkDiv8
    }
    #[doc = "LSI kernel selected as clock source (lsi_ker_ck)"]
    #[inline(always)]
    pub fn is_lsi_ker(&self) -> bool {
        *self == SYSTICKSEL::LsiKer
    }
    #[doc = "LSE selected as clock source (lse_ck)"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == SYSTICKSEL::Lse
    }
}
#[doc = "Field `SYSTICKSEL` writer - SYSTICK clock source selection Note: rcc_hclk frequency must be four times higher than lsi_ker_ck/lse_ck (period (LSI/LSE) ≥ 4 * period (HCLK)."]
pub type SYSTICKSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SYSTICKSEL>;
impl<'a, REG> SYSTICKSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "RCC HLCK divided by 8 selected as clock source (rcc_hclk / 8)"]
    #[inline(always)]
    pub fn hclk_div8(self) -> &'a mut crate::W<REG> {
        self.variant(SYSTICKSEL::HclkDiv8)
    }
    #[doc = "LSI kernel selected as clock source (lsi_ker_ck)"]
    #[inline(always)]
    pub fn lsi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(SYSTICKSEL::LsiKer)
    }
    #[doc = "LSE selected as clock source (lse_ck)"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(SYSTICKSEL::Lse)
    }
}
#[doc = "USB kernel clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USBSEL {
    #[doc = "0: Disable the clock"]
    Disable = 0,
    #[doc = "1: PLL1 Q clock selected as clock source (pll1_q_ck)"]
    Pll1Q = 1,
    #[doc = "2: PLL2 Q clock selected as clock source (pll2_q_ck)"]
    Pll2Q = 2,
    #[doc = "3: HSI48 clock selected as clock source (hsi48_ker_ck)"]
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
#[doc = "Field `USBSEL` reader - USB kernel clock source selection"]
pub type USBSEL_R = crate::FieldReader<USBSEL>;
impl USBSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USBSEL {
        match self.bits {
            0 => USBSEL::Disable,
            1 => USBSEL::Pll1Q,
            2 => USBSEL::Pll2Q,
            3 => USBSEL::Hsi48,
            _ => unreachable!(),
        }
    }
    #[doc = "Disable the clock"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == USBSEL::Disable
    }
    #[doc = "PLL1 Q clock selected as clock source (pll1_q_ck)"]
    #[inline(always)]
    pub fn is_pll1_q(&self) -> bool {
        *self == USBSEL::Pll1Q
    }
    #[doc = "PLL2 Q clock selected as clock source (pll2_q_ck)"]
    #[inline(always)]
    pub fn is_pll2_q(&self) -> bool {
        *self == USBSEL::Pll2Q
    }
    #[doc = "HSI48 clock selected as clock source (hsi48_ker_ck)"]
    #[inline(always)]
    pub fn is_hsi48(&self) -> bool {
        *self == USBSEL::Hsi48
    }
}
#[doc = "Field `USBSEL` writer - USB kernel clock source selection"]
pub type USBSEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, USBSEL>;
impl<'a, REG> USBSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable the clock"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(USBSEL::Disable)
    }
    #[doc = "PLL1 Q clock selected as clock source (pll1_q_ck)"]
    #[inline(always)]
    pub fn pll1_q(self) -> &'a mut crate::W<REG> {
        self.variant(USBSEL::Pll1Q)
    }
    #[doc = "PLL2 Q clock selected as clock source (pll2_q_ck)"]
    #[inline(always)]
    pub fn pll2_q(self) -> &'a mut crate::W<REG> {
        self.variant(USBSEL::Pll2Q)
    }
    #[doc = "HSI48 clock selected as clock source (hsi48_ker_ck)"]
    #[inline(always)]
    pub fn hsi48(self) -> &'a mut crate::W<REG> {
        self.variant(USBSEL::Hsi48)
    }
}
#[doc = "I2C1 kernel clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2C1SEL {
    #[doc = "0: PCLK1 selected as clock source (rcc_pclk1)"]
    RccPclk1 = 0,
    #[doc = "1: PLL2 R Clock selected as clock source (pll2_r_ck)"]
    Pll2R = 1,
    #[doc = "2: HSI kernel clock selected as clock source (hsi_ker_ck)"]
    HsiKer = 2,
    #[doc = "3: CSI kernel clock selected as clock source (csi_ker_ck)"]
    CsiKer = 3,
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
#[doc = "Field `I2C1SEL` reader - I2C1 kernel clock source selection"]
pub type I2C1SEL_R = crate::FieldReader<I2C1SEL>;
impl I2C1SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2C1SEL {
        match self.bits {
            0 => I2C1SEL::RccPclk1,
            1 => I2C1SEL::Pll2R,
            2 => I2C1SEL::HsiKer,
            3 => I2C1SEL::CsiKer,
            _ => unreachable!(),
        }
    }
    #[doc = "PCLK1 selected as clock source (rcc_pclk1)"]
    #[inline(always)]
    pub fn is_rcc_pclk1(&self) -> bool {
        *self == I2C1SEL::RccPclk1
    }
    #[doc = "PLL2 R Clock selected as clock source (pll2_r_ck)"]
    #[inline(always)]
    pub fn is_pll2_r(&self) -> bool {
        *self == I2C1SEL::Pll2R
    }
    #[doc = "HSI kernel clock selected as clock source (hsi_ker_ck)"]
    #[inline(always)]
    pub fn is_hsi_ker(&self) -> bool {
        *self == I2C1SEL::HsiKer
    }
    #[doc = "CSI kernel clock selected as clock source (csi_ker_ck)"]
    #[inline(always)]
    pub fn is_csi_ker(&self) -> bool {
        *self == I2C1SEL::CsiKer
    }
}
#[doc = "Field `I2C1SEL` writer - I2C1 kernel clock source selection"]
pub type I2C1SEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, I2C1SEL>;
impl<'a, REG> I2C1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PCLK1 selected as clock source (rcc_pclk1)"]
    #[inline(always)]
    pub fn rcc_pclk1(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1SEL::RccPclk1)
    }
    #[doc = "PLL2 R Clock selected as clock source (pll2_r_ck)"]
    #[inline(always)]
    pub fn pll2_r(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1SEL::Pll2R)
    }
    #[doc = "HSI kernel clock selected as clock source (hsi_ker_ck)"]
    #[inline(always)]
    pub fn hsi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1SEL::HsiKer)
    }
    #[doc = "CSI kernel clock selected as clock source (csi_ker_ck)"]
    #[inline(always)]
    pub fn csi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1SEL::CsiKer)
    }
}
#[doc = "I2C2 kernel clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2C2SEL {
    #[doc = "0: PCLK1 selected as clock source (rcc_pclk1)"]
    RccPclk1 = 0,
    #[doc = "1: PLL2 R Clock selected as clock source (pll2_r_ck)"]
    Pll2R = 1,
    #[doc = "2: HSI kernel clock selected as clock source (hsi_ker_ck)"]
    HsiKer = 2,
    #[doc = "3: CSI kernel clock selected as clock source (csi_ker_ck)"]
    CsiKer = 3,
}
impl From<I2C2SEL> for u8 {
    #[inline(always)]
    fn from(variant: I2C2SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for I2C2SEL {
    type Ux = u8;
}
#[doc = "Field `I2C2SEL` reader - I2C2 kernel clock source selection"]
pub type I2C2SEL_R = crate::FieldReader<I2C2SEL>;
impl I2C2SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2C2SEL {
        match self.bits {
            0 => I2C2SEL::RccPclk1,
            1 => I2C2SEL::Pll2R,
            2 => I2C2SEL::HsiKer,
            3 => I2C2SEL::CsiKer,
            _ => unreachable!(),
        }
    }
    #[doc = "PCLK1 selected as clock source (rcc_pclk1)"]
    #[inline(always)]
    pub fn is_rcc_pclk1(&self) -> bool {
        *self == I2C2SEL::RccPclk1
    }
    #[doc = "PLL2 R Clock selected as clock source (pll2_r_ck)"]
    #[inline(always)]
    pub fn is_pll2_r(&self) -> bool {
        *self == I2C2SEL::Pll2R
    }
    #[doc = "HSI kernel clock selected as clock source (hsi_ker_ck)"]
    #[inline(always)]
    pub fn is_hsi_ker(&self) -> bool {
        *self == I2C2SEL::HsiKer
    }
    #[doc = "CSI kernel clock selected as clock source (csi_ker_ck)"]
    #[inline(always)]
    pub fn is_csi_ker(&self) -> bool {
        *self == I2C2SEL::CsiKer
    }
}
#[doc = "Field `I2C2SEL` writer - I2C2 kernel clock source selection"]
pub type I2C2SEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, I2C2SEL>;
impl<'a, REG> I2C2SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PCLK1 selected as clock source (rcc_pclk1)"]
    #[inline(always)]
    pub fn rcc_pclk1(self) -> &'a mut crate::W<REG> {
        self.variant(I2C2SEL::RccPclk1)
    }
    #[doc = "PLL2 R Clock selected as clock source (pll2_r_ck)"]
    #[inline(always)]
    pub fn pll2_r(self) -> &'a mut crate::W<REG> {
        self.variant(I2C2SEL::Pll2R)
    }
    #[doc = "HSI kernel clock selected as clock source (hsi_ker_ck)"]
    #[inline(always)]
    pub fn hsi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(I2C2SEL::HsiKer)
    }
    #[doc = "CSI kernel clock selected as clock source (csi_ker_ck)"]
    #[inline(always)]
    pub fn csi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(I2C2SEL::CsiKer)
    }
}
#[doc = "I3C1 kernel clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I3C1SEL {
    #[doc = "0: PCLK1 selected as clock source (rcc_pclk1)"]
    RccPclk1 = 0,
    #[doc = "1: PLL2 R Clock selected as clock source (pll2_r_ck)"]
    Pll2R = 1,
    #[doc = "2: HSI kernel clock selected as clock source (hsi_ker_ck)"]
    HsiKer = 2,
}
impl From<I3C1SEL> for u8 {
    #[inline(always)]
    fn from(variant: I3C1SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for I3C1SEL {
    type Ux = u8;
}
#[doc = "Field `I3C1SEL` reader - I3C1 kernel clock source selection"]
pub type I3C1SEL_R = crate::FieldReader<I3C1SEL>;
impl I3C1SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<I3C1SEL> {
        match self.bits {
            0 => Some(I3C1SEL::RccPclk1),
            1 => Some(I3C1SEL::Pll2R),
            2 => Some(I3C1SEL::HsiKer),
            _ => None,
        }
    }
    #[doc = "PCLK1 selected as clock source (rcc_pclk1)"]
    #[inline(always)]
    pub fn is_rcc_pclk1(&self) -> bool {
        *self == I3C1SEL::RccPclk1
    }
    #[doc = "PLL2 R Clock selected as clock source (pll2_r_ck)"]
    #[inline(always)]
    pub fn is_pll2_r(&self) -> bool {
        *self == I3C1SEL::Pll2R
    }
    #[doc = "HSI kernel clock selected as clock source (hsi_ker_ck)"]
    #[inline(always)]
    pub fn is_hsi_ker(&self) -> bool {
        *self == I3C1SEL::HsiKer
    }
}
#[doc = "Field `I3C1SEL` writer - I3C1 kernel clock source selection"]
pub type I3C1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, I3C1SEL>;
impl<'a, REG> I3C1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PCLK1 selected as clock source (rcc_pclk1)"]
    #[inline(always)]
    pub fn rcc_pclk1(self) -> &'a mut crate::W<REG> {
        self.variant(I3C1SEL::RccPclk1)
    }
    #[doc = "PLL2 R Clock selected as clock source (pll2_r_ck)"]
    #[inline(always)]
    pub fn pll2_r(self) -> &'a mut crate::W<REG> {
        self.variant(I3C1SEL::Pll2R)
    }
    #[doc = "HSI kernel clock selected as clock source (hsi_ker_ck)"]
    #[inline(always)]
    pub fn hsi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(I3C1SEL::HsiKer)
    }
}
#[doc = "I3C2 kernel clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I3C2SEL {
    #[doc = "0: PCLK3 selected as clock source (rcc_pclk3)"]
    RccPclk3 = 0,
    #[doc = "1: PLL2 R clock selected as clock source (pll2_r_ck)"]
    Pll2R = 1,
    #[doc = "2: HSI kernel clock selected as clock source (hsi_ker_ck)"]
    HsiKer = 2,
}
impl From<I3C2SEL> for u8 {
    #[inline(always)]
    fn from(variant: I3C2SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for I3C2SEL {
    type Ux = u8;
}
#[doc = "Field `I3C2SEL` reader - I3C2 kernel clock source selection"]
pub type I3C2SEL_R = crate::FieldReader<I3C2SEL>;
impl I3C2SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<I3C2SEL> {
        match self.bits {
            0 => Some(I3C2SEL::RccPclk3),
            1 => Some(I3C2SEL::Pll2R),
            2 => Some(I3C2SEL::HsiKer),
            _ => None,
        }
    }
    #[doc = "PCLK3 selected as clock source (rcc_pclk3)"]
    #[inline(always)]
    pub fn is_rcc_pclk3(&self) -> bool {
        *self == I3C2SEL::RccPclk3
    }
    #[doc = "PLL2 R clock selected as clock source (pll2_r_ck)"]
    #[inline(always)]
    pub fn is_pll2_r(&self) -> bool {
        *self == I3C2SEL::Pll2R
    }
    #[doc = "HSI kernel clock selected as clock source (hsi_ker_ck)"]
    #[inline(always)]
    pub fn is_hsi_ker(&self) -> bool {
        *self == I3C2SEL::HsiKer
    }
}
#[doc = "Field `I3C2SEL` writer - I3C2 kernel clock source selection"]
pub type I3C2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, I3C2SEL>;
impl<'a, REG> I3C2SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PCLK3 selected as clock source (rcc_pclk3)"]
    #[inline(always)]
    pub fn rcc_pclk3(self) -> &'a mut crate::W<REG> {
        self.variant(I3C2SEL::RccPclk3)
    }
    #[doc = "PLL2 R clock selected as clock source (pll2_r_ck)"]
    #[inline(always)]
    pub fn pll2_r(self) -> &'a mut crate::W<REG> {
        self.variant(I3C2SEL::Pll2R)
    }
    #[doc = "HSI kernel clock selected as clock source (hsi_ker_ck)"]
    #[inline(always)]
    pub fn hsi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(I3C2SEL::HsiKer)
    }
}
impl R {
    #[doc = "Bits 2:3 - SYSTICK clock source selection Note: rcc_hclk frequency must be four times higher than lsi_ker_ck/lse_ck (period (LSI/LSE) ≥ 4 * period (HCLK)."]
    #[inline(always)]
    pub fn systicksel(&self) -> SYSTICKSEL_R {
        SYSTICKSEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - USB kernel clock source selection"]
    #[inline(always)]
    pub fn usbsel(&self) -> USBSEL_R {
        USBSEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 16:17 - I2C1 kernel clock source selection"]
    #[inline(always)]
    pub fn i2c1sel(&self) -> I2C1SEL_R {
        I2C1SEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - I2C2 kernel clock source selection"]
    #[inline(always)]
    pub fn i2c2sel(&self) -> I2C2SEL_R {
        I2C2SEL_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 24:25 - I3C1 kernel clock source selection"]
    #[inline(always)]
    pub fn i3c1sel(&self) -> I3C1SEL_R {
        I3C1SEL_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - I3C2 kernel clock source selection"]
    #[inline(always)]
    pub fn i3c2sel(&self) -> I3C2SEL_R {
        I3C2SEL_R::new(((self.bits >> 26) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 2:3 - SYSTICK clock source selection Note: rcc_hclk frequency must be four times higher than lsi_ker_ck/lse_ck (period (LSI/LSE) ≥ 4 * period (HCLK)."]
    #[inline(always)]
    #[must_use]
    pub fn systicksel(&mut self) -> SYSTICKSEL_W<CCIPR4rs> {
        SYSTICKSEL_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - USB kernel clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn usbsel(&mut self) -> USBSEL_W<CCIPR4rs> {
        USBSEL_W::new(self, 4)
    }
    #[doc = "Bits 16:17 - I2C1 kernel clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1sel(&mut self) -> I2C1SEL_W<CCIPR4rs> {
        I2C1SEL_W::new(self, 16)
    }
    #[doc = "Bits 18:19 - I2C2 kernel clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2sel(&mut self) -> I2C2SEL_W<CCIPR4rs> {
        I2C2SEL_W::new(self, 18)
    }
    #[doc = "Bits 24:25 - I3C1 kernel clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn i3c1sel(&mut self) -> I3C1SEL_W<CCIPR4rs> {
        I3C1SEL_W::new(self, 24)
    }
    #[doc = "Bits 26:27 - I3C2 kernel clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn i3c2sel(&mut self) -> I3C2SEL_W<CCIPR4rs> {
        I3C2SEL_W::new(self, 26)
    }
}
#[doc = "RCC kernel clock configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccipr4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccipr4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCIPR4rs;
impl crate::RegisterSpec for CCIPR4rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccipr4::R`](R) reader structure"]
impl crate::Readable for CCIPR4rs {}
#[doc = "`write(|w| ..)` method takes [`ccipr4::W`](W) writer structure"]
impl crate::Writable for CCIPR4rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCIPR4 to value 0"]
impl crate::Resettable for CCIPR4rs {
    const RESET_VALUE: u32 = 0;
}
