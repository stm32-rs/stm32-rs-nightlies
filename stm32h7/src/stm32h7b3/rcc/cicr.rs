#[doc = "Register `CICR` reader"]
pub struct R(crate::R<CICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CICR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CICR` writer"]
pub struct W(crate::W<CICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CICR_SPEC>;
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
impl From<crate::W<CICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "LSI ready interrupt clear Set by software to clear LSIRDYF. Reset by hardware when clear done.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSIRDYC_A {
    #[doc = "1: Clear interrupt flag"]
    CLEAR = 1,
}
impl From<LSIRDYC_A> for bool {
    #[inline(always)]
    fn from(variant: LSIRDYC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSIRDYC` reader - LSI ready interrupt clear Set by software to clear LSIRDYF. Reset by hardware when clear done."]
pub struct LSIRDYC_R(crate::FieldReader<bool, LSIRDYC_A>);
impl LSIRDYC_R {
    pub(crate) fn new(bits: bool) -> Self {
        LSIRDYC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LSIRDYC_A> {
        match self.bits {
            true => Some(LSIRDYC_A::CLEAR),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == LSIRDYC_A::CLEAR
    }
}
impl core::ops::Deref for LSIRDYC_R {
    type Target = crate::FieldReader<bool, LSIRDYC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSIRDYC` writer - LSI ready interrupt clear Set by software to clear LSIRDYF. Reset by hardware when clear done."]
pub struct LSIRDYC_W<'a> {
    w: &'a mut W,
}
impl<'a> LSIRDYC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LSIRDYC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear interrupt flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(LSIRDYC_A::CLEAR)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "LSE ready interrupt clear Set by software to clear LSERDYF. Reset by hardware when clear done."]
pub type LSERDYC_A = LSIRDYC_A;
#[doc = "Field `LSERDYC` reader - LSE ready interrupt clear Set by software to clear LSERDYF. Reset by hardware when clear done."]
pub type LSERDYC_R = LSIRDYC_R;
#[doc = "Field `LSERDYC` writer - LSE ready interrupt clear Set by software to clear LSERDYF. Reset by hardware when clear done."]
pub struct LSERDYC_W<'a> {
    w: &'a mut W,
}
impl<'a> LSERDYC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LSERDYC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear interrupt flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(LSERDYC_A::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "HSI ready interrupt clear Set by software to clear HSIRDYF. Reset by hardware when clear done."]
pub type HSIRDYC_A = LSIRDYC_A;
#[doc = "Field `HSIRDYC` reader - HSI ready interrupt clear Set by software to clear HSIRDYF. Reset by hardware when clear done."]
pub type HSIRDYC_R = LSIRDYC_R;
#[doc = "Field `HSIRDYC` writer - HSI ready interrupt clear Set by software to clear HSIRDYF. Reset by hardware when clear done."]
pub struct HSIRDYC_W<'a> {
    w: &'a mut W,
}
impl<'a> HSIRDYC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSIRDYC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear interrupt flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(HSIRDYC_A::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "HSE ready interrupt clear Set by software to clear HSERDYF. Reset by hardware when clear done."]
pub type HSERDYC_A = LSIRDYC_A;
#[doc = "Field `HSERDYC` reader - HSE ready interrupt clear Set by software to clear HSERDYF. Reset by hardware when clear done."]
pub type HSERDYC_R = LSIRDYC_R;
#[doc = "Field `HSERDYC` writer - HSE ready interrupt clear Set by software to clear HSERDYF. Reset by hardware when clear done."]
pub struct HSERDYC_W<'a> {
    w: &'a mut W,
}
impl<'a> HSERDYC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSERDYC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear interrupt flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(HSERDYC_A::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "CSI ready interrupt clear Set by software to clear CSIRDYF. Reset by hardware when clear done."]
pub type CSIRDYC_A = LSIRDYC_A;
#[doc = "Field `CSIRDYC` reader - CSI ready interrupt clear Set by software to clear CSIRDYF. Reset by hardware when clear done."]
pub type CSIRDYC_R = LSIRDYC_R;
#[doc = "Field `CSIRDYC` writer - CSI ready interrupt clear Set by software to clear CSIRDYF. Reset by hardware when clear done."]
pub struct CSIRDYC_W<'a> {
    w: &'a mut W,
}
impl<'a> CSIRDYC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSIRDYC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear interrupt flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CSIRDYC_A::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "HSI48 ready interrupt clear Set by software to clear HSI48RDYF. Reset by hardware when clear done."]
pub type HSI48RDYC_A = LSIRDYC_A;
#[doc = "Field `HSI48RDYC` reader - HSI48 ready interrupt clear Set by software to clear HSI48RDYF. Reset by hardware when clear done."]
pub type HSI48RDYC_R = LSIRDYC_R;
#[doc = "Field `HSI48RDYC` writer - HSI48 ready interrupt clear Set by software to clear HSI48RDYF. Reset by hardware when clear done."]
pub struct HSI48RDYC_W<'a> {
    w: &'a mut W,
}
impl<'a> HSI48RDYC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSI48RDYC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear interrupt flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(HSI48RDYC_A::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "PLL1 ready interrupt clear Set by software to clear PLL1RDYF. Reset by hardware when clear done."]
pub type PLL1RDYC_A = LSIRDYC_A;
#[doc = "Field `PLL1RDYC` reader - PLL1 ready interrupt clear Set by software to clear PLL1RDYF. Reset by hardware when clear done."]
pub type PLL1RDYC_R = LSIRDYC_R;
#[doc = "Field `PLL1RDYC` writer - PLL1 ready interrupt clear Set by software to clear PLL1RDYF. Reset by hardware when clear done."]
pub struct PLL1RDYC_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL1RDYC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLL1RDYC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear interrupt flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PLL1RDYC_A::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "PLL2 ready interrupt clear Set by software to clear PLL2RDYF. Reset by hardware when clear done."]
pub type PLL2RDYC_A = LSIRDYC_A;
#[doc = "Field `PLL2RDYC` reader - PLL2 ready interrupt clear Set by software to clear PLL2RDYF. Reset by hardware when clear done."]
pub type PLL2RDYC_R = LSIRDYC_R;
#[doc = "Field `PLL2RDYC` writer - PLL2 ready interrupt clear Set by software to clear PLL2RDYF. Reset by hardware when clear done."]
pub struct PLL2RDYC_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL2RDYC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLL2RDYC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear interrupt flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PLL2RDYC_A::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "PLL3 ready interrupt clear Set by software to clear PLL3RDYF. Reset by hardware when clear done."]
pub type PLL3RDYC_A = LSIRDYC_A;
#[doc = "Field `PLL3RDYC` reader - PLL3 ready interrupt clear Set by software to clear PLL3RDYF. Reset by hardware when clear done."]
pub type PLL3RDYC_R = LSIRDYC_R;
#[doc = "Field `PLL3RDYC` writer - PLL3 ready interrupt clear Set by software to clear PLL3RDYF. Reset by hardware when clear done."]
pub struct PLL3RDYC_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL3RDYC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLL3RDYC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear interrupt flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PLL3RDYC_A::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "LSE clock security system interrupt clear Set by software to clear LSECSSF. Reset by hardware when clear done."]
pub type LSECSSC_A = LSIRDYC_A;
#[doc = "Field `LSECSSC` reader - LSE clock security system interrupt clear Set by software to clear LSECSSF. Reset by hardware when clear done."]
pub type LSECSSC_R = LSIRDYC_R;
#[doc = "Field `LSECSSC` writer - LSE clock security system interrupt clear Set by software to clear LSECSSF. Reset by hardware when clear done."]
pub struct LSECSSC_W<'a> {
    w: &'a mut W,
}
impl<'a> LSECSSC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LSECSSC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear interrupt flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(LSECSSC_A::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "HSE clock security system interrupt clear Set by software to clear HSECSSF. Reset by hardware when clear done."]
pub type HSECSSC_A = LSIRDYC_A;
#[doc = "Field `HSECSSC` reader - HSE clock security system interrupt clear Set by software to clear HSECSSF. Reset by hardware when clear done."]
pub type HSECSSC_R = LSIRDYC_R;
#[doc = "Field `HSECSSC` writer - HSE clock security system interrupt clear Set by software to clear HSECSSF. Reset by hardware when clear done."]
pub struct HSECSSC_W<'a> {
    w: &'a mut W,
}
impl<'a> HSECSSC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSECSSC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear interrupt flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(HSECSSC_A::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - LSI ready interrupt clear Set by software to clear LSIRDYF. Reset by hardware when clear done."]
    #[inline(always)]
    pub fn lsirdyc(&self) -> LSIRDYC_R {
        LSIRDYC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - LSE ready interrupt clear Set by software to clear LSERDYF. Reset by hardware when clear done."]
    #[inline(always)]
    pub fn lserdyc(&self) -> LSERDYC_R {
        LSERDYC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - HSI ready interrupt clear Set by software to clear HSIRDYF. Reset by hardware when clear done."]
    #[inline(always)]
    pub fn hsirdyc(&self) -> HSIRDYC_R {
        HSIRDYC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - HSE ready interrupt clear Set by software to clear HSERDYF. Reset by hardware when clear done."]
    #[inline(always)]
    pub fn hserdyc(&self) -> HSERDYC_R {
        HSERDYC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CSI ready interrupt clear Set by software to clear CSIRDYF. Reset by hardware when clear done."]
    #[inline(always)]
    pub fn csirdyc(&self) -> CSIRDYC_R {
        CSIRDYC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - HSI48 ready interrupt clear Set by software to clear HSI48RDYF. Reset by hardware when clear done."]
    #[inline(always)]
    pub fn hsi48rdyc(&self) -> HSI48RDYC_R {
        HSI48RDYC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PLL1 ready interrupt clear Set by software to clear PLL1RDYF. Reset by hardware when clear done."]
    #[inline(always)]
    pub fn pll1rdyc(&self) -> PLL1RDYC_R {
        PLL1RDYC_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - PLL2 ready interrupt clear Set by software to clear PLL2RDYF. Reset by hardware when clear done."]
    #[inline(always)]
    pub fn pll2rdyc(&self) -> PLL2RDYC_R {
        PLL2RDYC_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - PLL3 ready interrupt clear Set by software to clear PLL3RDYF. Reset by hardware when clear done."]
    #[inline(always)]
    pub fn pll3rdyc(&self) -> PLL3RDYC_R {
        PLL3RDYC_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - LSE clock security system interrupt clear Set by software to clear LSECSSF. Reset by hardware when clear done."]
    #[inline(always)]
    pub fn lsecssc(&self) -> LSECSSC_R {
        LSECSSC_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - HSE clock security system interrupt clear Set by software to clear HSECSSF. Reset by hardware when clear done."]
    #[inline(always)]
    pub fn hsecssc(&self) -> HSECSSC_R {
        HSECSSC_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LSI ready interrupt clear Set by software to clear LSIRDYF. Reset by hardware when clear done."]
    #[inline(always)]
    pub fn lsirdyc(&mut self) -> LSIRDYC_W {
        LSIRDYC_W { w: self }
    }
    #[doc = "Bit 1 - LSE ready interrupt clear Set by software to clear LSERDYF. Reset by hardware when clear done."]
    #[inline(always)]
    pub fn lserdyc(&mut self) -> LSERDYC_W {
        LSERDYC_W { w: self }
    }
    #[doc = "Bit 2 - HSI ready interrupt clear Set by software to clear HSIRDYF. Reset by hardware when clear done."]
    #[inline(always)]
    pub fn hsirdyc(&mut self) -> HSIRDYC_W {
        HSIRDYC_W { w: self }
    }
    #[doc = "Bit 3 - HSE ready interrupt clear Set by software to clear HSERDYF. Reset by hardware when clear done."]
    #[inline(always)]
    pub fn hserdyc(&mut self) -> HSERDYC_W {
        HSERDYC_W { w: self }
    }
    #[doc = "Bit 4 - CSI ready interrupt clear Set by software to clear CSIRDYF. Reset by hardware when clear done."]
    #[inline(always)]
    pub fn csirdyc(&mut self) -> CSIRDYC_W {
        CSIRDYC_W { w: self }
    }
    #[doc = "Bit 5 - HSI48 ready interrupt clear Set by software to clear HSI48RDYF. Reset by hardware when clear done."]
    #[inline(always)]
    pub fn hsi48rdyc(&mut self) -> HSI48RDYC_W {
        HSI48RDYC_W { w: self }
    }
    #[doc = "Bit 6 - PLL1 ready interrupt clear Set by software to clear PLL1RDYF. Reset by hardware when clear done."]
    #[inline(always)]
    pub fn pll1rdyc(&mut self) -> PLL1RDYC_W {
        PLL1RDYC_W { w: self }
    }
    #[doc = "Bit 7 - PLL2 ready interrupt clear Set by software to clear PLL2RDYF. Reset by hardware when clear done."]
    #[inline(always)]
    pub fn pll2rdyc(&mut self) -> PLL2RDYC_W {
        PLL2RDYC_W { w: self }
    }
    #[doc = "Bit 8 - PLL3 ready interrupt clear Set by software to clear PLL3RDYF. Reset by hardware when clear done."]
    #[inline(always)]
    pub fn pll3rdyc(&mut self) -> PLL3RDYC_W {
        PLL3RDYC_W { w: self }
    }
    #[doc = "Bit 9 - LSE clock security system interrupt clear Set by software to clear LSECSSF. Reset by hardware when clear done."]
    #[inline(always)]
    pub fn lsecssc(&mut self) -> LSECSSC_W {
        LSECSSC_W { w: self }
    }
    #[doc = "Bit 10 - HSE clock security system interrupt clear Set by software to clear HSECSSF. Reset by hardware when clear done."]
    #[inline(always)]
    pub fn hsecssc(&mut self) -> HSECSSC_W {
        HSECSSC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cicr](index.html) module"]
pub struct CICR_SPEC;
impl crate::RegisterSpec for CICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cicr::R](R) reader structure"]
impl crate::Readable for CICR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cicr::W](W) writer structure"]
impl crate::Writable for CICR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CICR to value 0"]
impl crate::Resettable for CICR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
