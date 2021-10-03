#[doc = "Register `CIFR` reader"]
pub struct R(crate::R<CIFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CIFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CIFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CIFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "LSI ready interrupt flag Reset by software by writing LSIRDYC bit. Set by hardware when the LSI clock becomes stable and LSIRDYIE is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSIRDYF_A {
    #[doc = "0: no clock ready interrupt caused by the LSI (default after reset)"]
    B_0X0 = 0,
    #[doc = "1: clock ready interrupt caused by the LSI "]
    B_0X1 = 1,
}
impl From<LSIRDYF_A> for bool {
    #[inline(always)]
    fn from(variant: LSIRDYF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSIRDYF` reader - LSI ready interrupt flag Reset by software by writing LSIRDYC bit. Set by hardware when the LSI clock becomes stable and LSIRDYIE is set."]
pub struct LSIRDYF_R(crate::FieldReader<bool, LSIRDYF_A>);
impl LSIRDYF_R {
    pub(crate) fn new(bits: bool) -> Self {
        LSIRDYF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSIRDYF_A {
        match self.bits {
            false => LSIRDYF_A::B_0X0,
            true => LSIRDYF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == LSIRDYF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == LSIRDYF_A::B_0X1
    }
}
impl core::ops::Deref for LSIRDYF_R {
    type Target = crate::FieldReader<bool, LSIRDYF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "LSE ready interrupt flag Reset by software by writing LSERDYC bit. Set by hardware when the LSE clock becomes stable and LSERDYIE is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSERDYF_A {
    #[doc = "0: no clock ready interrupt caused by the LSE (default after reset)"]
    B_0X0 = 0,
    #[doc = "1: clock ready interrupt caused by the LSE "]
    B_0X1 = 1,
}
impl From<LSERDYF_A> for bool {
    #[inline(always)]
    fn from(variant: LSERDYF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSERDYF` reader - LSE ready interrupt flag Reset by software by writing LSERDYC bit. Set by hardware when the LSE clock becomes stable and LSERDYIE is set."]
pub struct LSERDYF_R(crate::FieldReader<bool, LSERDYF_A>);
impl LSERDYF_R {
    pub(crate) fn new(bits: bool) -> Self {
        LSERDYF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSERDYF_A {
        match self.bits {
            false => LSERDYF_A::B_0X0,
            true => LSERDYF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == LSERDYF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == LSERDYF_A::B_0X1
    }
}
impl core::ops::Deref for LSERDYF_R {
    type Target = crate::FieldReader<bool, LSERDYF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "HSI ready interrupt flag Reset by software by writing HSIRDYC bit. Set by hardware when the HSI clock becomes stable and HSIRDYIE is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSIRDYF_A {
    #[doc = "0: no clock ready interrupt caused by the HSI (default after reset)"]
    B_0X0 = 0,
    #[doc = "1: clock ready interrupt caused by the HSI "]
    B_0X1 = 1,
}
impl From<HSIRDYF_A> for bool {
    #[inline(always)]
    fn from(variant: HSIRDYF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSIRDYF` reader - HSI ready interrupt flag Reset by software by writing HSIRDYC bit. Set by hardware when the HSI clock becomes stable and HSIRDYIE is set."]
pub struct HSIRDYF_R(crate::FieldReader<bool, HSIRDYF_A>);
impl HSIRDYF_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSIRDYF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSIRDYF_A {
        match self.bits {
            false => HSIRDYF_A::B_0X0,
            true => HSIRDYF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == HSIRDYF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == HSIRDYF_A::B_0X1
    }
}
impl core::ops::Deref for HSIRDYF_R {
    type Target = crate::FieldReader<bool, HSIRDYF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "HSE ready interrupt flag Reset by software by writing HSERDYC bit. Set by hardware when the HSE clock becomes stable and HSERDYIE is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSERDYF_A {
    #[doc = "0: no clock ready interrupt caused by the HSE (default after reset)"]
    B_0X0 = 0,
    #[doc = "1: clock ready interrupt caused by the HSE "]
    B_0X1 = 1,
}
impl From<HSERDYF_A> for bool {
    #[inline(always)]
    fn from(variant: HSERDYF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSERDYF` reader - HSE ready interrupt flag Reset by software by writing HSERDYC bit. Set by hardware when the HSE clock becomes stable and HSERDYIE is set."]
pub struct HSERDYF_R(crate::FieldReader<bool, HSERDYF_A>);
impl HSERDYF_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSERDYF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSERDYF_A {
        match self.bits {
            false => HSERDYF_A::B_0X0,
            true => HSERDYF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == HSERDYF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == HSERDYF_A::B_0X1
    }
}
impl core::ops::Deref for HSERDYF_R {
    type Target = crate::FieldReader<bool, HSERDYF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "CSI ready interrupt flag Reset by software by writing CSIRDYC bit. Set by hardware when the CSI clock becomes stable and CSIRDYIE is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSIRDYF_A {
    #[doc = "0: no clock ready interrupt caused by the CSI (default after reset)"]
    B_0X0 = 0,
    #[doc = "1: clock ready interrupt caused by the CSI "]
    B_0X1 = 1,
}
impl From<CSIRDYF_A> for bool {
    #[inline(always)]
    fn from(variant: CSIRDYF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSIRDYF` reader - CSI ready interrupt flag Reset by software by writing CSIRDYC bit. Set by hardware when the CSI clock becomes stable and CSIRDYIE is set."]
pub struct CSIRDYF_R(crate::FieldReader<bool, CSIRDYF_A>);
impl CSIRDYF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSIRDYF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSIRDYF_A {
        match self.bits {
            false => CSIRDYF_A::B_0X0,
            true => CSIRDYF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == CSIRDYF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == CSIRDYF_A::B_0X1
    }
}
impl core::ops::Deref for CSIRDYF_R {
    type Target = crate::FieldReader<bool, CSIRDYF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "HSI48 ready interrupt flag Reset by software by writing HSI48RDYC bit. Set by hardware when the HSI48 clock becomes stable and HSI48RDYIE is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSI48RDYF_A {
    #[doc = "0: no clock ready interrupt caused by the HSI48 oscillator (default after reset)"]
    B_0X0 = 0,
    #[doc = "1: clock ready interrupt caused by the HSI48 oscillator"]
    B_0X1 = 1,
}
impl From<HSI48RDYF_A> for bool {
    #[inline(always)]
    fn from(variant: HSI48RDYF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSI48RDYF` reader - HSI48 ready interrupt flag Reset by software by writing HSI48RDYC bit. Set by hardware when the HSI48 clock becomes stable and HSI48RDYIE is set."]
pub struct HSI48RDYF_R(crate::FieldReader<bool, HSI48RDYF_A>);
impl HSI48RDYF_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSI48RDYF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSI48RDYF_A {
        match self.bits {
            false => HSI48RDYF_A::B_0X0,
            true => HSI48RDYF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == HSI48RDYF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == HSI48RDYF_A::B_0X1
    }
}
impl core::ops::Deref for HSI48RDYF_R {
    type Target = crate::FieldReader<bool, HSI48RDYF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "PLL1 ready interrupt flag Reset by software by writing PLL1RDYC bit. Set by hardware when the PLL1 locks and PLL1RDYIE is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLL1RDYF_A {
    #[doc = "0: no clock ready interrupt caused by PLL1 lock (default after reset)"]
    B_0X0 = 0,
    #[doc = "1: clock ready interrupt caused by PLL1 lock"]
    B_0X1 = 1,
}
impl From<PLL1RDYF_A> for bool {
    #[inline(always)]
    fn from(variant: PLL1RDYF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLL1RDYF` reader - PLL1 ready interrupt flag Reset by software by writing PLL1RDYC bit. Set by hardware when the PLL1 locks and PLL1RDYIE is set."]
pub struct PLL1RDYF_R(crate::FieldReader<bool, PLL1RDYF_A>);
impl PLL1RDYF_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLL1RDYF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLL1RDYF_A {
        match self.bits {
            false => PLL1RDYF_A::B_0X0,
            true => PLL1RDYF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == PLL1RDYF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == PLL1RDYF_A::B_0X1
    }
}
impl core::ops::Deref for PLL1RDYF_R {
    type Target = crate::FieldReader<bool, PLL1RDYF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "PLL2 ready interrupt flag Reset by software by writing PLL2RDYC bit. Set by hardware when the PLL2 locks and PLL2RDYIE is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLL2RDYF_A {
    #[doc = "0: no clock ready interrupt caused by PLL2 lock (default after reset)"]
    B_0X0 = 0,
    #[doc = "1: clock ready interrupt caused by PLL2 lock"]
    B_0X1 = 1,
}
impl From<PLL2RDYF_A> for bool {
    #[inline(always)]
    fn from(variant: PLL2RDYF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLL2RDYF` reader - PLL2 ready interrupt flag Reset by software by writing PLL2RDYC bit. Set by hardware when the PLL2 locks and PLL2RDYIE is set."]
pub struct PLL2RDYF_R(crate::FieldReader<bool, PLL2RDYF_A>);
impl PLL2RDYF_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLL2RDYF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLL2RDYF_A {
        match self.bits {
            false => PLL2RDYF_A::B_0X0,
            true => PLL2RDYF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == PLL2RDYF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == PLL2RDYF_A::B_0X1
    }
}
impl core::ops::Deref for PLL2RDYF_R {
    type Target = crate::FieldReader<bool, PLL2RDYF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "PLL3 ready interrupt flag Reset by software by writing PLL3RDYC bit. Set by hardware when the PLL3 locks and PLL3RDYIE is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLL3RDYF_A {
    #[doc = "0: no clock ready interrupt caused by PLL3 lock (default after reset)"]
    B_0X0 = 0,
    #[doc = "1: clock ready interrupt caused by PLL3 lock"]
    B_0X1 = 1,
}
impl From<PLL3RDYF_A> for bool {
    #[inline(always)]
    fn from(variant: PLL3RDYF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLL3RDYF` reader - PLL3 ready interrupt flag Reset by software by writing PLL3RDYC bit. Set by hardware when the PLL3 locks and PLL3RDYIE is set."]
pub struct PLL3RDYF_R(crate::FieldReader<bool, PLL3RDYF_A>);
impl PLL3RDYF_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLL3RDYF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLL3RDYF_A {
        match self.bits {
            false => PLL3RDYF_A::B_0X0,
            true => PLL3RDYF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == PLL3RDYF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == PLL3RDYF_A::B_0X1
    }
}
impl core::ops::Deref for PLL3RDYF_R {
    type Target = crate::FieldReader<bool, PLL3RDYF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "LSE clock security system interrupt flag Reset by software by writing LSECSSC bit. Set by hardware when a failure is detected on the external 32 kHz oscillator and LSECSSIE is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSECSSF_A {
    #[doc = "0: no failure detected on the external 32 kHz oscillator (default after reset)"]
    B_0X0 = 0,
    #[doc = "1: failure detected on the external 32 kHz oscillator"]
    B_0X1 = 1,
}
impl From<LSECSSF_A> for bool {
    #[inline(always)]
    fn from(variant: LSECSSF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSECSSF` reader - LSE clock security system interrupt flag Reset by software by writing LSECSSC bit. Set by hardware when a failure is detected on the external 32 kHz oscillator and LSECSSIE is set."]
pub struct LSECSSF_R(crate::FieldReader<bool, LSECSSF_A>);
impl LSECSSF_R {
    pub(crate) fn new(bits: bool) -> Self {
        LSECSSF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSECSSF_A {
        match self.bits {
            false => LSECSSF_A::B_0X0,
            true => LSECSSF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == LSECSSF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == LSECSSF_A::B_0X1
    }
}
impl core::ops::Deref for LSECSSF_R {
    type Target = crate::FieldReader<bool, LSECSSF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "HSE clock security system interrupt flag Reset by software by writing HSECSSC bit. Set by hardware in case of HSE clock failure.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSECSSF_A {
    #[doc = "0: no clock security interrupt caused by HSE clock failure (default after reset)"]
    B_0X0 = 0,
    #[doc = "1: clock security interrupt caused by HSE clock failure"]
    B_0X1 = 1,
}
impl From<HSECSSF_A> for bool {
    #[inline(always)]
    fn from(variant: HSECSSF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSECSSF` reader - HSE clock security system interrupt flag Reset by software by writing HSECSSC bit. Set by hardware in case of HSE clock failure."]
pub struct HSECSSF_R(crate::FieldReader<bool, HSECSSF_A>);
impl HSECSSF_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSECSSF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSECSSF_A {
        match self.bits {
            false => HSECSSF_A::B_0X0,
            true => HSECSSF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == HSECSSF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == HSECSSF_A::B_0X1
    }
}
impl core::ops::Deref for HSECSSF_R {
    type Target = crate::FieldReader<bool, HSECSSF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - LSI ready interrupt flag Reset by software by writing LSIRDYC bit. Set by hardware when the LSI clock becomes stable and LSIRDYIE is set."]
    #[inline(always)]
    pub fn lsirdyf(&self) -> LSIRDYF_R {
        LSIRDYF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - LSE ready interrupt flag Reset by software by writing LSERDYC bit. Set by hardware when the LSE clock becomes stable and LSERDYIE is set."]
    #[inline(always)]
    pub fn lserdyf(&self) -> LSERDYF_R {
        LSERDYF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - HSI ready interrupt flag Reset by software by writing HSIRDYC bit. Set by hardware when the HSI clock becomes stable and HSIRDYIE is set."]
    #[inline(always)]
    pub fn hsirdyf(&self) -> HSIRDYF_R {
        HSIRDYF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - HSE ready interrupt flag Reset by software by writing HSERDYC bit. Set by hardware when the HSE clock becomes stable and HSERDYIE is set."]
    #[inline(always)]
    pub fn hserdyf(&self) -> HSERDYF_R {
        HSERDYF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CSI ready interrupt flag Reset by software by writing CSIRDYC bit. Set by hardware when the CSI clock becomes stable and CSIRDYIE is set."]
    #[inline(always)]
    pub fn csirdyf(&self) -> CSIRDYF_R {
        CSIRDYF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - HSI48 ready interrupt flag Reset by software by writing HSI48RDYC bit. Set by hardware when the HSI48 clock becomes stable and HSI48RDYIE is set."]
    #[inline(always)]
    pub fn hsi48rdyf(&self) -> HSI48RDYF_R {
        HSI48RDYF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PLL1 ready interrupt flag Reset by software by writing PLL1RDYC bit. Set by hardware when the PLL1 locks and PLL1RDYIE is set."]
    #[inline(always)]
    pub fn pll1rdyf(&self) -> PLL1RDYF_R {
        PLL1RDYF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - PLL2 ready interrupt flag Reset by software by writing PLL2RDYC bit. Set by hardware when the PLL2 locks and PLL2RDYIE is set."]
    #[inline(always)]
    pub fn pll2rdyf(&self) -> PLL2RDYF_R {
        PLL2RDYF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - PLL3 ready interrupt flag Reset by software by writing PLL3RDYC bit. Set by hardware when the PLL3 locks and PLL3RDYIE is set."]
    #[inline(always)]
    pub fn pll3rdyf(&self) -> PLL3RDYF_R {
        PLL3RDYF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - LSE clock security system interrupt flag Reset by software by writing LSECSSC bit. Set by hardware when a failure is detected on the external 32 kHz oscillator and LSECSSIE is set."]
    #[inline(always)]
    pub fn lsecssf(&self) -> LSECSSF_R {
        LSECSSF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - HSE clock security system interrupt flag Reset by software by writing HSECSSC bit. Set by hardware in case of HSE clock failure."]
    #[inline(always)]
    pub fn hsecssf(&self) -> HSECSSF_R {
        HSECSSF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cifr](index.html) module"]
pub struct CIFR_SPEC;
impl crate::RegisterSpec for CIFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cifr::R](R) reader structure"]
impl crate::Readable for CIFR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CIFR to value 0"]
impl crate::Resettable for CIFR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
