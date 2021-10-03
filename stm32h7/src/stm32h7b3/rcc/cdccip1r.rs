#[doc = "Register `CDCCIP1R` reader"]
pub struct R(crate::R<CDCCIP1R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CDCCIP1R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CDCCIP1R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CDCCIP1R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CDCCIP1R` writer"]
pub struct W(crate::W<CDCCIP1R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CDCCIP1R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CDCCIP1R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CDCCIP1R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "SAI1 and DFSDM1 kernel Aclk clock source selection Set and reset by software. If the selected clock is the external clock and this clock is stopped, it isnot be possible to switch to another clock. Refer to for additional information. Note: DFSDM1 clock source selection is done by DFSDM1SEL. others: reserved, the kernel clock is disabled Note: I2S_CKIN is an external clock taken from a pin.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SAI1SEL_A {
    #[doc = "0: pll1_q selected as peripheral clock"]
    PLL1_Q = 0,
    #[doc = "1: pll2_p selected as peripheral clock"]
    PLL2_P = 1,
    #[doc = "2: pll3_p selected as peripheral clock"]
    PLL3_P = 2,
    #[doc = "3: I2S_CKIN selected as peripheral clock"]
    I2S_CKIN = 3,
    #[doc = "4: PER selected as peripheral clock"]
    PER = 4,
}
impl From<SAI1SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SAI1SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SAI1SEL` reader - SAI1 and DFSDM1 kernel Aclk clock source selection Set and reset by software. If the selected clock is the external clock and this clock is stopped, it isnot be possible to switch to another clock. Refer to for additional information. Note: DFSDM1 clock source selection is done by DFSDM1SEL. others: reserved, the kernel clock is disabled Note: I2S_CKIN is an external clock taken from a pin."]
pub struct SAI1SEL_R(crate::FieldReader<u8, SAI1SEL_A>);
impl SAI1SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        SAI1SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SAI1SEL_A> {
        match self.bits {
            0 => Some(SAI1SEL_A::PLL1_Q),
            1 => Some(SAI1SEL_A::PLL2_P),
            2 => Some(SAI1SEL_A::PLL3_P),
            3 => Some(SAI1SEL_A::I2S_CKIN),
            4 => Some(SAI1SEL_A::PER),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PLL1_Q`"]
    #[inline(always)]
    pub fn is_pll1_q(&self) -> bool {
        **self == SAI1SEL_A::PLL1_Q
    }
    #[doc = "Checks if the value of the field is `PLL2_P`"]
    #[inline(always)]
    pub fn is_pll2_p(&self) -> bool {
        **self == SAI1SEL_A::PLL2_P
    }
    #[doc = "Checks if the value of the field is `PLL3_P`"]
    #[inline(always)]
    pub fn is_pll3_p(&self) -> bool {
        **self == SAI1SEL_A::PLL3_P
    }
    #[doc = "Checks if the value of the field is `I2S_CKIN`"]
    #[inline(always)]
    pub fn is_i2s_ckin(&self) -> bool {
        **self == SAI1SEL_A::I2S_CKIN
    }
    #[doc = "Checks if the value of the field is `PER`"]
    #[inline(always)]
    pub fn is_per(&self) -> bool {
        **self == SAI1SEL_A::PER
    }
}
impl core::ops::Deref for SAI1SEL_R {
    type Target = crate::FieldReader<u8, SAI1SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAI1SEL` writer - SAI1 and DFSDM1 kernel Aclk clock source selection Set and reset by software. If the selected clock is the external clock and this clock is stopped, it isnot be possible to switch to another clock. Refer to for additional information. Note: DFSDM1 clock source selection is done by DFSDM1SEL. others: reserved, the kernel clock is disabled Note: I2S_CKIN is an external clock taken from a pin."]
pub struct SAI1SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI1SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAI1SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "pll1_q selected as peripheral clock"]
    #[inline(always)]
    pub fn pll1_q(self) -> &'a mut W {
        self.variant(SAI1SEL_A::PLL1_Q)
    }
    #[doc = "pll2_p selected as peripheral clock"]
    #[inline(always)]
    pub fn pll2_p(self) -> &'a mut W {
        self.variant(SAI1SEL_A::PLL2_P)
    }
    #[doc = "pll3_p selected as peripheral clock"]
    #[inline(always)]
    pub fn pll3_p(self) -> &'a mut W {
        self.variant(SAI1SEL_A::PLL3_P)
    }
    #[doc = "I2S_CKIN selected as peripheral clock"]
    #[inline(always)]
    pub fn i2s_ckin(self) -> &'a mut W {
        self.variant(SAI1SEL_A::I2S_CKIN)
    }
    #[doc = "PER selected as peripheral clock"]
    #[inline(always)]
    pub fn per(self) -> &'a mut W {
        self.variant(SAI1SEL_A::PER)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "SAI2 kernel clock source A selection Set and reset by software. If the selected clock is the external clock and this clock is stopped, it is not be possible to switch to another clock. Refer to for additional information. others: reserved, the kernel clock is disabled Note: I2S_CKIN is an external clock taken from a pin. spdifrx_symb_ck is the symbol clock generated by the SPDIFRX (see ).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SAI2ASEL_A {
    #[doc = "0: pll1_q selected as peripheral clock"]
    PLL1_Q = 0,
    #[doc = "1: pll2_p selected as peripheral clock"]
    PLL2_P = 1,
    #[doc = "2: pll3_p selected as peripheral clock"]
    PLL3_P = 2,
    #[doc = "3: i2s_ckin selected as peripheral clock"]
    I2S_CKIN = 3,
    #[doc = "4: PER selected as peripheral clock"]
    PER = 4,
}
impl From<SAI2ASEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SAI2ASEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SAI2ASEL` reader - SAI2 kernel clock source A selection Set and reset by software. If the selected clock is the external clock and this clock is stopped, it is not be possible to switch to another clock. Refer to for additional information. others: reserved, the kernel clock is disabled Note: I2S_CKIN is an external clock taken from a pin. spdifrx_symb_ck is the symbol clock generated by the SPDIFRX (see )."]
pub struct SAI2ASEL_R(crate::FieldReader<u8, SAI2ASEL_A>);
impl SAI2ASEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        SAI2ASEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SAI2ASEL_A> {
        match self.bits {
            0 => Some(SAI2ASEL_A::PLL1_Q),
            1 => Some(SAI2ASEL_A::PLL2_P),
            2 => Some(SAI2ASEL_A::PLL3_P),
            3 => Some(SAI2ASEL_A::I2S_CKIN),
            4 => Some(SAI2ASEL_A::PER),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PLL1_Q`"]
    #[inline(always)]
    pub fn is_pll1_q(&self) -> bool {
        **self == SAI2ASEL_A::PLL1_Q
    }
    #[doc = "Checks if the value of the field is `PLL2_P`"]
    #[inline(always)]
    pub fn is_pll2_p(&self) -> bool {
        **self == SAI2ASEL_A::PLL2_P
    }
    #[doc = "Checks if the value of the field is `PLL3_P`"]
    #[inline(always)]
    pub fn is_pll3_p(&self) -> bool {
        **self == SAI2ASEL_A::PLL3_P
    }
    #[doc = "Checks if the value of the field is `I2S_CKIN`"]
    #[inline(always)]
    pub fn is_i2s_ckin(&self) -> bool {
        **self == SAI2ASEL_A::I2S_CKIN
    }
    #[doc = "Checks if the value of the field is `PER`"]
    #[inline(always)]
    pub fn is_per(&self) -> bool {
        **self == SAI2ASEL_A::PER
    }
}
impl core::ops::Deref for SAI2ASEL_R {
    type Target = crate::FieldReader<u8, SAI2ASEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAI2ASEL` writer - SAI2 kernel clock source A selection Set and reset by software. If the selected clock is the external clock and this clock is stopped, it is not be possible to switch to another clock. Refer to for additional information. others: reserved, the kernel clock is disabled Note: I2S_CKIN is an external clock taken from a pin. spdifrx_symb_ck is the symbol clock generated by the SPDIFRX (see )."]
pub struct SAI2ASEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI2ASEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAI2ASEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "pll1_q selected as peripheral clock"]
    #[inline(always)]
    pub fn pll1_q(self) -> &'a mut W {
        self.variant(SAI2ASEL_A::PLL1_Q)
    }
    #[doc = "pll2_p selected as peripheral clock"]
    #[inline(always)]
    pub fn pll2_p(self) -> &'a mut W {
        self.variant(SAI2ASEL_A::PLL2_P)
    }
    #[doc = "pll3_p selected as peripheral clock"]
    #[inline(always)]
    pub fn pll3_p(self) -> &'a mut W {
        self.variant(SAI2ASEL_A::PLL3_P)
    }
    #[doc = "i2s_ckin selected as peripheral clock"]
    #[inline(always)]
    pub fn i2s_ckin(self) -> &'a mut W {
        self.variant(SAI2ASEL_A::I2S_CKIN)
    }
    #[doc = "PER selected as peripheral clock"]
    #[inline(always)]
    pub fn per(self) -> &'a mut W {
        self.variant(SAI2ASEL_A::PER)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 6)) | ((value as u32 & 0x07) << 6);
        self.w
    }
}
#[doc = "SAI2 kernel clock B source selection Set and reset by software. If the selected clock is the external clock and this clock is stopped, it is not be possible to switch to another clock. Refer to for additional information. others: reserved, the kernel clock is disabled Note: I2S_CKIN is an external clock taken from a pin. spdifrx_symb_ck is the symbol clock generated by the spdifrx (see )."]
pub type SAI2BSEL_A = SAI2ASEL_A;
#[doc = "Field `SAI2BSEL` reader - SAI2 kernel clock B source selection Set and reset by software. If the selected clock is the external clock and this clock is stopped, it is not be possible to switch to another clock. Refer to for additional information. others: reserved, the kernel clock is disabled Note: I2S_CKIN is an external clock taken from a pin. spdifrx_symb_ck is the symbol clock generated by the spdifrx (see )."]
pub type SAI2BSEL_R = SAI2ASEL_R;
#[doc = "Field `SAI2BSEL` writer - SAI2 kernel clock B source selection Set and reset by software. If the selected clock is the external clock and this clock is stopped, it is not be possible to switch to another clock. Refer to for additional information. others: reserved, the kernel clock is disabled Note: I2S_CKIN is an external clock taken from a pin. spdifrx_symb_ck is the symbol clock generated by the spdifrx (see )."]
pub struct SAI2BSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI2BSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAI2BSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "pll1_q selected as peripheral clock"]
    #[inline(always)]
    pub fn pll1_q(self) -> &'a mut W {
        self.variant(SAI2BSEL_A::PLL1_Q)
    }
    #[doc = "pll2_p selected as peripheral clock"]
    #[inline(always)]
    pub fn pll2_p(self) -> &'a mut W {
        self.variant(SAI2BSEL_A::PLL2_P)
    }
    #[doc = "pll3_p selected as peripheral clock"]
    #[inline(always)]
    pub fn pll3_p(self) -> &'a mut W {
        self.variant(SAI2BSEL_A::PLL3_P)
    }
    #[doc = "i2s_ckin selected as peripheral clock"]
    #[inline(always)]
    pub fn i2s_ckin(self) -> &'a mut W {
        self.variant(SAI2BSEL_A::I2S_CKIN)
    }
    #[doc = "PER selected as peripheral clock"]
    #[inline(always)]
    pub fn per(self) -> &'a mut W {
        self.variant(SAI2BSEL_A::PER)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 9)) | ((value as u32 & 0x07) << 9);
        self.w
    }
}
#[doc = "SPI/I2S1,2 and 3 kernel clock source selection Set and reset by software. If the selected clock is the external clock and this clock is stopped, it is not be possible to switch to another clock. Refer to for additional information. others: reserved, the kernel clock is disabled Note: I2S_CKIN is an external clock taken from a pin."]
pub type SPI123SEL_A = SAI1SEL_A;
#[doc = "Field `SPI123SEL` reader - SPI/I2S1,2 and 3 kernel clock source selection Set and reset by software. If the selected clock is the external clock and this clock is stopped, it is not be possible to switch to another clock. Refer to for additional information. others: reserved, the kernel clock is disabled Note: I2S_CKIN is an external clock taken from a pin."]
pub type SPI123SEL_R = SAI1SEL_R;
#[doc = "Field `SPI123SEL` writer - SPI/I2S1,2 and 3 kernel clock source selection Set and reset by software. If the selected clock is the external clock and this clock is stopped, it is not be possible to switch to another clock. Refer to for additional information. others: reserved, the kernel clock is disabled Note: I2S_CKIN is an external clock taken from a pin."]
pub struct SPI123SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI123SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI123SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "pll1_q selected as peripheral clock"]
    #[inline(always)]
    pub fn pll1_q(self) -> &'a mut W {
        self.variant(SPI123SEL_A::PLL1_Q)
    }
    #[doc = "pll2_p selected as peripheral clock"]
    #[inline(always)]
    pub fn pll2_p(self) -> &'a mut W {
        self.variant(SPI123SEL_A::PLL2_P)
    }
    #[doc = "pll3_p selected as peripheral clock"]
    #[inline(always)]
    pub fn pll3_p(self) -> &'a mut W {
        self.variant(SPI123SEL_A::PLL3_P)
    }
    #[doc = "I2S_CKIN selected as peripheral clock"]
    #[inline(always)]
    pub fn i2s_ckin(self) -> &'a mut W {
        self.variant(SPI123SEL_A::I2S_CKIN)
    }
    #[doc = "PER selected as peripheral clock"]
    #[inline(always)]
    pub fn per(self) -> &'a mut W {
        self.variant(SPI123SEL_A::PER)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u32 & 0x07) << 12);
        self.w
    }
}
#[doc = "SPI4 and 5 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SPI45SEL_A {
    #[doc = "0: APB clock selected as peripheral clock"]
    APB = 0,
    #[doc = "1: pll2_q selected as peripheral clock"]
    PLL2_Q = 1,
    #[doc = "2: pll3_q selected as peripheral clock"]
    PLL3_Q = 2,
    #[doc = "3: hsi_ker selected as peripheral clock"]
    HSI_KER = 3,
    #[doc = "4: csi_ker selected as peripheral clock"]
    CSI_KER = 4,
    #[doc = "5: HSE selected as peripheral clock"]
    HSE = 5,
}
impl From<SPI45SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SPI45SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SPI45SEL` reader - SPI4 and 5 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
pub struct SPI45SEL_R(crate::FieldReader<u8, SPI45SEL_A>);
impl SPI45SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        SPI45SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SPI45SEL_A> {
        match self.bits {
            0 => Some(SPI45SEL_A::APB),
            1 => Some(SPI45SEL_A::PLL2_Q),
            2 => Some(SPI45SEL_A::PLL3_Q),
            3 => Some(SPI45SEL_A::HSI_KER),
            4 => Some(SPI45SEL_A::CSI_KER),
            5 => Some(SPI45SEL_A::HSE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `APB`"]
    #[inline(always)]
    pub fn is_apb(&self) -> bool {
        **self == SPI45SEL_A::APB
    }
    #[doc = "Checks if the value of the field is `PLL2_Q`"]
    #[inline(always)]
    pub fn is_pll2_q(&self) -> bool {
        **self == SPI45SEL_A::PLL2_Q
    }
    #[doc = "Checks if the value of the field is `PLL3_Q`"]
    #[inline(always)]
    pub fn is_pll3_q(&self) -> bool {
        **self == SPI45SEL_A::PLL3_Q
    }
    #[doc = "Checks if the value of the field is `HSI_KER`"]
    #[inline(always)]
    pub fn is_hsi_ker(&self) -> bool {
        **self == SPI45SEL_A::HSI_KER
    }
    #[doc = "Checks if the value of the field is `CSI_KER`"]
    #[inline(always)]
    pub fn is_csi_ker(&self) -> bool {
        **self == SPI45SEL_A::CSI_KER
    }
    #[doc = "Checks if the value of the field is `HSE`"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        **self == SPI45SEL_A::HSE
    }
}
impl core::ops::Deref for SPI45SEL_R {
    type Target = crate::FieldReader<u8, SPI45SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI45SEL` writer - SPI4 and 5 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
pub struct SPI45SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI45SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI45SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "APB clock selected as peripheral clock"]
    #[inline(always)]
    pub fn apb(self) -> &'a mut W {
        self.variant(SPI45SEL_A::APB)
    }
    #[doc = "pll2_q selected as peripheral clock"]
    #[inline(always)]
    pub fn pll2_q(self) -> &'a mut W {
        self.variant(SPI45SEL_A::PLL2_Q)
    }
    #[doc = "pll3_q selected as peripheral clock"]
    #[inline(always)]
    pub fn pll3_q(self) -> &'a mut W {
        self.variant(SPI45SEL_A::PLL3_Q)
    }
    #[doc = "hsi_ker selected as peripheral clock"]
    #[inline(always)]
    pub fn hsi_ker(self) -> &'a mut W {
        self.variant(SPI45SEL_A::HSI_KER)
    }
    #[doc = "csi_ker selected as peripheral clock"]
    #[inline(always)]
    pub fn csi_ker(self) -> &'a mut W {
        self.variant(SPI45SEL_A::CSI_KER)
    }
    #[doc = "HSE selected as peripheral clock"]
    #[inline(always)]
    pub fn hse(self) -> &'a mut W {
        self.variant(SPI45SEL_A::HSE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
#[doc = "SPDIFRX kernel clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SPDIFRXSEL_A {
    #[doc = "0: pll1_q selected as peripheral clock"]
    PLL1_Q = 0,
    #[doc = "1: pll2_r selected as peripheral clock"]
    PLL2_R = 1,
    #[doc = "2: pll3_r selected as peripheral clock"]
    PLL3_R = 2,
    #[doc = "3: hsi_ker selected as peripheral clock"]
    HSI_KER = 3,
}
impl From<SPDIFRXSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SPDIFRXSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SPDIFRXSEL` reader - SPDIFRX kernel clock source selection"]
pub struct SPDIFRXSEL_R(crate::FieldReader<u8, SPDIFRXSEL_A>);
impl SPDIFRXSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        SPDIFRXSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPDIFRXSEL_A {
        match self.bits {
            0 => SPDIFRXSEL_A::PLL1_Q,
            1 => SPDIFRXSEL_A::PLL2_R,
            2 => SPDIFRXSEL_A::PLL3_R,
            3 => SPDIFRXSEL_A::HSI_KER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PLL1_Q`"]
    #[inline(always)]
    pub fn is_pll1_q(&self) -> bool {
        **self == SPDIFRXSEL_A::PLL1_Q
    }
    #[doc = "Checks if the value of the field is `PLL2_R`"]
    #[inline(always)]
    pub fn is_pll2_r(&self) -> bool {
        **self == SPDIFRXSEL_A::PLL2_R
    }
    #[doc = "Checks if the value of the field is `PLL3_R`"]
    #[inline(always)]
    pub fn is_pll3_r(&self) -> bool {
        **self == SPDIFRXSEL_A::PLL3_R
    }
    #[doc = "Checks if the value of the field is `HSI_KER`"]
    #[inline(always)]
    pub fn is_hsi_ker(&self) -> bool {
        **self == SPDIFRXSEL_A::HSI_KER
    }
}
impl core::ops::Deref for SPDIFRXSEL_R {
    type Target = crate::FieldReader<u8, SPDIFRXSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPDIFRXSEL` writer - SPDIFRX kernel clock source selection"]
pub struct SPDIFRXSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPDIFRXSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPDIFRXSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "pll1_q selected as peripheral clock"]
    #[inline(always)]
    pub fn pll1_q(self) -> &'a mut W {
        self.variant(SPDIFRXSEL_A::PLL1_Q)
    }
    #[doc = "pll2_r selected as peripheral clock"]
    #[inline(always)]
    pub fn pll2_r(self) -> &'a mut W {
        self.variant(SPDIFRXSEL_A::PLL2_R)
    }
    #[doc = "pll3_r selected as peripheral clock"]
    #[inline(always)]
    pub fn pll3_r(self) -> &'a mut W {
        self.variant(SPDIFRXSEL_A::PLL3_R)
    }
    #[doc = "hsi_ker selected as peripheral clock"]
    #[inline(always)]
    pub fn hsi_ker(self) -> &'a mut W {
        self.variant(SPDIFRXSEL_A::HSI_KER)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "DFSDM1 kernel clock Clk source selection Set and reset by software. Note: the DFSDM1 Aclk clock source selection is done by SAI1SEL (see ).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFSDM1SEL_A {
    #[doc = "0: rcc_pclk2 selected as peripheral clock"]
    RCC_PCLK2 = 0,
    #[doc = "1: System clock selected as peripheral clock"]
    SYS = 1,
}
impl From<DFSDM1SEL_A> for bool {
    #[inline(always)]
    fn from(variant: DFSDM1SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DFSDM1SEL` reader - DFSDM1 kernel clock Clk source selection Set and reset by software. Note: the DFSDM1 Aclk clock source selection is done by SAI1SEL (see )."]
pub struct DFSDM1SEL_R(crate::FieldReader<bool, DFSDM1SEL_A>);
impl DFSDM1SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        DFSDM1SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFSDM1SEL_A {
        match self.bits {
            false => DFSDM1SEL_A::RCC_PCLK2,
            true => DFSDM1SEL_A::SYS,
        }
    }
    #[doc = "Checks if the value of the field is `RCC_PCLK2`"]
    #[inline(always)]
    pub fn is_rcc_pclk2(&self) -> bool {
        **self == DFSDM1SEL_A::RCC_PCLK2
    }
    #[doc = "Checks if the value of the field is `SYS`"]
    #[inline(always)]
    pub fn is_sys(&self) -> bool {
        **self == DFSDM1SEL_A::SYS
    }
}
impl core::ops::Deref for DFSDM1SEL_R {
    type Target = crate::FieldReader<bool, DFSDM1SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFSDM1SEL` writer - DFSDM1 kernel clock Clk source selection Set and reset by software. Note: the DFSDM1 Aclk clock source selection is done by SAI1SEL (see )."]
pub struct DFSDM1SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DFSDM1SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DFSDM1SEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "rcc_pclk2 selected as peripheral clock"]
    #[inline(always)]
    pub fn rcc_pclk2(self) -> &'a mut W {
        self.variant(DFSDM1SEL_A::RCC_PCLK2)
    }
    #[doc = "System clock selected as peripheral clock"]
    #[inline(always)]
    pub fn sys(self) -> &'a mut W {
        self.variant(DFSDM1SEL_A::SYS)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "FDCAN kernel clock source selection Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FDCANSEL_A {
    #[doc = "0: HSE selected as peripheral clock"]
    HSE = 0,
    #[doc = "1: pll1_q selected as peripheral clock"]
    PLL1_Q = 1,
    #[doc = "2: pll2_q selected as peripheral clock"]
    PLL2_Q = 2,
}
impl From<FDCANSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FDCANSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FDCANSEL` reader - FDCAN kernel clock source selection Set and reset by software."]
pub struct FDCANSEL_R(crate::FieldReader<u8, FDCANSEL_A>);
impl FDCANSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        FDCANSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FDCANSEL_A> {
        match self.bits {
            0 => Some(FDCANSEL_A::HSE),
            1 => Some(FDCANSEL_A::PLL1_Q),
            2 => Some(FDCANSEL_A::PLL2_Q),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HSE`"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        **self == FDCANSEL_A::HSE
    }
    #[doc = "Checks if the value of the field is `PLL1_Q`"]
    #[inline(always)]
    pub fn is_pll1_q(&self) -> bool {
        **self == FDCANSEL_A::PLL1_Q
    }
    #[doc = "Checks if the value of the field is `PLL2_Q`"]
    #[inline(always)]
    pub fn is_pll2_q(&self) -> bool {
        **self == FDCANSEL_A::PLL2_Q
    }
}
impl core::ops::Deref for FDCANSEL_R {
    type Target = crate::FieldReader<u8, FDCANSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FDCANSEL` writer - FDCAN kernel clock source selection Set and reset by software."]
pub struct FDCANSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FDCANSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FDCANSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "HSE selected as peripheral clock"]
    #[inline(always)]
    pub fn hse(self) -> &'a mut W {
        self.variant(FDCANSEL_A::HSE)
    }
    #[doc = "pll1_q selected as peripheral clock"]
    #[inline(always)]
    pub fn pll1_q(self) -> &'a mut W {
        self.variant(FDCANSEL_A::PLL1_Q)
    }
    #[doc = "pll2_q selected as peripheral clock"]
    #[inline(always)]
    pub fn pll2_q(self) -> &'a mut W {
        self.variant(FDCANSEL_A::PLL2_Q)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | ((value as u32 & 0x03) << 28);
        self.w
    }
}
#[doc = "SWPMI kernel clock source selection Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWPMISEL_A {
    #[doc = "0: pclk selected as peripheral clock"]
    PCLK = 0,
    #[doc = "1: hsi_ker selected as peripheral clock"]
    HSI_KER = 1,
}
impl From<SWPMISEL_A> for bool {
    #[inline(always)]
    fn from(variant: SWPMISEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWPMISEL` reader - SWPMI kernel clock source selection Set and reset by software."]
pub struct SWPMISEL_R(crate::FieldReader<bool, SWPMISEL_A>);
impl SWPMISEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWPMISEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWPMISEL_A {
        match self.bits {
            false => SWPMISEL_A::PCLK,
            true => SWPMISEL_A::HSI_KER,
        }
    }
    #[doc = "Checks if the value of the field is `PCLK`"]
    #[inline(always)]
    pub fn is_pclk(&self) -> bool {
        **self == SWPMISEL_A::PCLK
    }
    #[doc = "Checks if the value of the field is `HSI_KER`"]
    #[inline(always)]
    pub fn is_hsi_ker(&self) -> bool {
        **self == SWPMISEL_A::HSI_KER
    }
}
impl core::ops::Deref for SWPMISEL_R {
    type Target = crate::FieldReader<bool, SWPMISEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWPMISEL` writer - SWPMI kernel clock source selection Set and reset by software."]
pub struct SWPMISEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SWPMISEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWPMISEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "pclk selected as peripheral clock"]
    #[inline(always)]
    pub fn pclk(self) -> &'a mut W {
        self.variant(SWPMISEL_A::PCLK)
    }
    #[doc = "hsi_ker selected as peripheral clock"]
    #[inline(always)]
    pub fn hsi_ker(self) -> &'a mut W {
        self.variant(SWPMISEL_A::HSI_KER)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - SAI1 and DFSDM1 kernel Aclk clock source selection Set and reset by software. If the selected clock is the external clock and this clock is stopped, it isnot be possible to switch to another clock. Refer to for additional information. Note: DFSDM1 clock source selection is done by DFSDM1SEL. others: reserved, the kernel clock is disabled Note: I2S_CKIN is an external clock taken from a pin."]
    #[inline(always)]
    pub fn sai1sel(&self) -> SAI1SEL_R {
        SAI1SEL_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 6:8 - SAI2 kernel clock source A selection Set and reset by software. If the selected clock is the external clock and this clock is stopped, it is not be possible to switch to another clock. Refer to for additional information. others: reserved, the kernel clock is disabled Note: I2S_CKIN is an external clock taken from a pin. spdifrx_symb_ck is the symbol clock generated by the SPDIFRX (see )."]
    #[inline(always)]
    pub fn sai2asel(&self) -> SAI2ASEL_R {
        SAI2ASEL_R::new(((self.bits >> 6) & 0x07) as u8)
    }
    #[doc = "Bits 9:11 - SAI2 kernel clock B source selection Set and reset by software. If the selected clock is the external clock and this clock is stopped, it is not be possible to switch to another clock. Refer to for additional information. others: reserved, the kernel clock is disabled Note: I2S_CKIN is an external clock taken from a pin. spdifrx_symb_ck is the symbol clock generated by the spdifrx (see )."]
    #[inline(always)]
    pub fn sai2bsel(&self) -> SAI2BSEL_R {
        SAI2BSEL_R::new(((self.bits >> 9) & 0x07) as u8)
    }
    #[doc = "Bits 12:14 - SPI/I2S1,2 and 3 kernel clock source selection Set and reset by software. If the selected clock is the external clock and this clock is stopped, it is not be possible to switch to another clock. Refer to for additional information. others: reserved, the kernel clock is disabled Note: I2S_CKIN is an external clock taken from a pin."]
    #[inline(always)]
    pub fn spi123sel(&self) -> SPI123SEL_R {
        SPI123SEL_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 16:18 - SPI4 and 5 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn spi45sel(&self) -> SPI45SEL_R {
        SPI45SEL_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 20:21 - SPDIFRX kernel clock source selection"]
    #[inline(always)]
    pub fn spdifrxsel(&self) -> SPDIFRXSEL_R {
        SPDIFRXSEL_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bit 24 - DFSDM1 kernel clock Clk source selection Set and reset by software. Note: the DFSDM1 Aclk clock source selection is done by SAI1SEL (see )."]
    #[inline(always)]
    pub fn dfsdm1sel(&self) -> DFSDM1SEL_R {
        DFSDM1SEL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 28:29 - FDCAN kernel clock source selection Set and reset by software."]
    #[inline(always)]
    pub fn fdcansel(&self) -> FDCANSEL_R {
        FDCANSEL_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bit 31 - SWPMI kernel clock source selection Set and reset by software."]
    #[inline(always)]
    pub fn swpmisel(&self) -> SWPMISEL_R {
        SWPMISEL_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - SAI1 and DFSDM1 kernel Aclk clock source selection Set and reset by software. If the selected clock is the external clock and this clock is stopped, it isnot be possible to switch to another clock. Refer to for additional information. Note: DFSDM1 clock source selection is done by DFSDM1SEL. others: reserved, the kernel clock is disabled Note: I2S_CKIN is an external clock taken from a pin."]
    #[inline(always)]
    pub fn sai1sel(&mut self) -> SAI1SEL_W {
        SAI1SEL_W { w: self }
    }
    #[doc = "Bits 6:8 - SAI2 kernel clock source A selection Set and reset by software. If the selected clock is the external clock and this clock is stopped, it is not be possible to switch to another clock. Refer to for additional information. others: reserved, the kernel clock is disabled Note: I2S_CKIN is an external clock taken from a pin. spdifrx_symb_ck is the symbol clock generated by the SPDIFRX (see )."]
    #[inline(always)]
    pub fn sai2asel(&mut self) -> SAI2ASEL_W {
        SAI2ASEL_W { w: self }
    }
    #[doc = "Bits 9:11 - SAI2 kernel clock B source selection Set and reset by software. If the selected clock is the external clock and this clock is stopped, it is not be possible to switch to another clock. Refer to for additional information. others: reserved, the kernel clock is disabled Note: I2S_CKIN is an external clock taken from a pin. spdifrx_symb_ck is the symbol clock generated by the spdifrx (see )."]
    #[inline(always)]
    pub fn sai2bsel(&mut self) -> SAI2BSEL_W {
        SAI2BSEL_W { w: self }
    }
    #[doc = "Bits 12:14 - SPI/I2S1,2 and 3 kernel clock source selection Set and reset by software. If the selected clock is the external clock and this clock is stopped, it is not be possible to switch to another clock. Refer to for additional information. others: reserved, the kernel clock is disabled Note: I2S_CKIN is an external clock taken from a pin."]
    #[inline(always)]
    pub fn spi123sel(&mut self) -> SPI123SEL_W {
        SPI123SEL_W { w: self }
    }
    #[doc = "Bits 16:18 - SPI4 and 5 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn spi45sel(&mut self) -> SPI45SEL_W {
        SPI45SEL_W { w: self }
    }
    #[doc = "Bits 20:21 - SPDIFRX kernel clock source selection"]
    #[inline(always)]
    pub fn spdifrxsel(&mut self) -> SPDIFRXSEL_W {
        SPDIFRXSEL_W { w: self }
    }
    #[doc = "Bit 24 - DFSDM1 kernel clock Clk source selection Set and reset by software. Note: the DFSDM1 Aclk clock source selection is done by SAI1SEL (see )."]
    #[inline(always)]
    pub fn dfsdm1sel(&mut self) -> DFSDM1SEL_W {
        DFSDM1SEL_W { w: self }
    }
    #[doc = "Bits 28:29 - FDCAN kernel clock source selection Set and reset by software."]
    #[inline(always)]
    pub fn fdcansel(&mut self) -> FDCANSEL_W {
        FDCANSEL_W { w: self }
    }
    #[doc = "Bit 31 - SWPMI kernel clock source selection Set and reset by software."]
    #[inline(always)]
    pub fn swpmisel(&mut self) -> SWPMISEL_W {
        SWPMISEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC CPU domain kernel clock configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdccip1r](index.html) module"]
pub struct CDCCIP1R_SPEC;
impl crate::RegisterSpec for CDCCIP1R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cdccip1r::R](R) reader structure"]
impl crate::Readable for CDCCIP1R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cdccip1r::W](W) writer structure"]
impl crate::Writable for CDCCIP1R_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CDCCIP1R to value 0"]
impl crate::Resettable for CDCCIP1R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
