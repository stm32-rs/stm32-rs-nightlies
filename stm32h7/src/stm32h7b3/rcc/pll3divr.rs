#[doc = "Register `PLL3DIVR` reader"]
pub struct R(crate::R<PLL3DIVR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL3DIVR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLL3DIVR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLL3DIVR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLL3DIVR` writer"]
pub struct W(crate::W<PLL3DIVR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLL3DIVR_SPEC>;
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
impl From<crate::W<PLL3DIVR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLL3DIVR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Multiplication factor for PLL3 VCO Set and reset by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled (PLL3ON = PLL3RDY = 0). ...........: not used ... ... Others: wrong configurations The software must set correctly these bits to insure that the VCO output frequency is between its valid frequency range, that is: 128 to 560Â MHz if PLL3VCOSEL = 0 150 to 420Â MHz if PLL3VCOSEL = 1 VCO output frequency = Fref3_ck x DIVN3, when fractional value 0 has been loaded into FRACN3, with: DIVN3 between 8 and 420 The input frequency Fref3_ck must be between 1 and 16MHz\n\nValue on reset: 128"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum DIVN3_A {
    #[doc = "6: wrong configuration"]
    B_0X6 = 6,
    #[doc = "7: DIVN3 = 8"]
    B_0X7 = 7,
    #[doc = "128: DIVN3 = 129 (default after reset)"]
    B_0X80 = 128,
    #[doc = "419: DIVN3 = 420"]
    B_0X1A3 = 419,
}
impl From<DIVN3_A> for u16 {
    #[inline(always)]
    fn from(variant: DIVN3_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DIVN3` reader - Multiplication factor for PLL3 VCO Set and reset by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled (PLL3ON = PLL3RDY = 0). ...........: not used ... ... Others: wrong configurations The software must set correctly these bits to insure that the VCO output frequency is between its valid frequency range, that is: 128 to 560Â MHz if PLL3VCOSEL = 0 150 to 420Â MHz if PLL3VCOSEL = 1 VCO output frequency = Fref3_ck x DIVN3, when fractional value 0 has been loaded into FRACN3, with: DIVN3 between 8 and 420 The input frequency Fref3_ck must be between 1 and 16MHz"]
pub struct DIVN3_R(crate::FieldReader<u16, DIVN3_A>);
impl DIVN3_R {
    pub(crate) fn new(bits: u16) -> Self {
        DIVN3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DIVN3_A> {
        match self.bits {
            6 => Some(DIVN3_A::B_0X6),
            7 => Some(DIVN3_A::B_0X7),
            128 => Some(DIVN3_A::B_0X80),
            419 => Some(DIVN3_A::B_0X1A3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X6`"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        **self == DIVN3_A::B_0X6
    }
    #[doc = "Checks if the value of the field is `B_0X7`"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        **self == DIVN3_A::B_0X7
    }
    #[doc = "Checks if the value of the field is `B_0X80`"]
    #[inline(always)]
    pub fn is_b_0x80(&self) -> bool {
        **self == DIVN3_A::B_0X80
    }
    #[doc = "Checks if the value of the field is `B_0X1A3`"]
    #[inline(always)]
    pub fn is_b_0x1a3(&self) -> bool {
        **self == DIVN3_A::B_0X1A3
    }
}
impl core::ops::Deref for DIVN3_R {
    type Target = crate::FieldReader<u16, DIVN3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIVN3` writer - Multiplication factor for PLL3 VCO Set and reset by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled (PLL3ON = PLL3RDY = 0). ...........: not used ... ... Others: wrong configurations The software must set correctly these bits to insure that the VCO output frequency is between its valid frequency range, that is: 128 to 560Â MHz if PLL3VCOSEL = 0 150 to 420Â MHz if PLL3VCOSEL = 1 VCO output frequency = Fref3_ck x DIVN3, when fractional value 0 has been loaded into FRACN3, with: DIVN3 between 8 and 420 The input frequency Fref3_ck must be between 1 and 16MHz"]
pub struct DIVN3_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVN3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIVN3_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "wrong configuration"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut W {
        self.variant(DIVN3_A::B_0X6)
    }
    #[doc = "DIVN3 = 8"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut W {
        self.variant(DIVN3_A::B_0X7)
    }
    #[doc = "DIVN3 = 129 (default after reset)"]
    #[inline(always)]
    pub fn b_0x80(self) -> &'a mut W {
        self.variant(DIVN3_A::B_0X80)
    }
    #[doc = "DIVN3 = 420"]
    #[inline(always)]
    pub fn b_0x1a3(self) -> &'a mut W {
        self.variant(DIVN3_A::B_0X1A3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | (value as u32 & 0x01ff);
        self.w
    }
}
#[doc = "PLL3 DIVP division factor Set and reset by software to control the frequency of the pll3_p_ck clock. These bits can be written only when the PLL3 is disabled (PLL3ON = PLL3RDY = 0). ...\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DIVP3_A {
    #[doc = "0: pll3_p_ck = vco3_ck"]
    B_0X0 = 0,
    #[doc = "1: pll3_p_ck = vco3_ck / 2 (default after reset)"]
    B_0X1 = 1,
    #[doc = "2: pll3_p_ck = vco3_ck / 3"]
    B_0X2 = 2,
    #[doc = "3: pll3_p_ck = vco3_ck / 4"]
    B_0X3 = 3,
    #[doc = "127: pll3_p_ck = vco3_ck / 128"]
    B_0X7F = 127,
}
impl From<DIVP3_A> for u8 {
    #[inline(always)]
    fn from(variant: DIVP3_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DIVP3` reader - PLL3 DIVP division factor Set and reset by software to control the frequency of the pll3_p_ck clock. These bits can be written only when the PLL3 is disabled (PLL3ON = PLL3RDY = 0). ..."]
pub struct DIVP3_R(crate::FieldReader<u8, DIVP3_A>);
impl DIVP3_R {
    pub(crate) fn new(bits: u8) -> Self {
        DIVP3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DIVP3_A> {
        match self.bits {
            0 => Some(DIVP3_A::B_0X0),
            1 => Some(DIVP3_A::B_0X1),
            2 => Some(DIVP3_A::B_0X2),
            3 => Some(DIVP3_A::B_0X3),
            127 => Some(DIVP3_A::B_0X7F),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == DIVP3_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == DIVP3_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        **self == DIVP3_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        **self == DIVP3_A::B_0X3
    }
    #[doc = "Checks if the value of the field is `B_0X7F`"]
    #[inline(always)]
    pub fn is_b_0x7f(&self) -> bool {
        **self == DIVP3_A::B_0X7F
    }
}
impl core::ops::Deref for DIVP3_R {
    type Target = crate::FieldReader<u8, DIVP3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIVP3` writer - PLL3 DIVP division factor Set and reset by software to control the frequency of the pll3_p_ck clock. These bits can be written only when the PLL3 is disabled (PLL3ON = PLL3RDY = 0). ..."]
pub struct DIVP3_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVP3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIVP3_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "pll3_p_ck = vco3_ck"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DIVP3_A::B_0X0)
    }
    #[doc = "pll3_p_ck = vco3_ck / 2 (default after reset)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DIVP3_A::B_0X1)
    }
    #[doc = "pll3_p_ck = vco3_ck / 3"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(DIVP3_A::B_0X2)
    }
    #[doc = "pll3_p_ck = vco3_ck / 4"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(DIVP3_A::B_0X3)
    }
    #[doc = "pll3_p_ck = vco3_ck / 128"]
    #[inline(always)]
    pub fn b_0x7f(self) -> &'a mut W {
        self.variant(DIVP3_A::B_0X7F)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 9)) | ((value as u32 & 0x7f) << 9);
        self.w
    }
}
#[doc = "PLL3 DIVQ division factor Set and reset by software to control the frequency of the pll3_q_ck clock. These bits can be written only when the PLL3 is disabled (PLL3ON = PLL3RDY = 0). ...\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DIVQ3_A {
    #[doc = "0: pll3_q_ck = vco3_ck "]
    B_0X0 = 0,
    #[doc = "1: pll3_q_ck = vco3_ck / 2 (default after reset)"]
    B_0X1 = 1,
    #[doc = "2: pll3_q_ck = vco3_ck / 3"]
    B_0X2 = 2,
    #[doc = "3: pll3_q_ck = vco3_ck / 4"]
    B_0X3 = 3,
    #[doc = "127: pll3_q_ck = vco3_ck / 128"]
    B_0X7F = 127,
}
impl From<DIVQ3_A> for u8 {
    #[inline(always)]
    fn from(variant: DIVQ3_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DIVQ3` reader - PLL3 DIVQ division factor Set and reset by software to control the frequency of the pll3_q_ck clock. These bits can be written only when the PLL3 is disabled (PLL3ON = PLL3RDY = 0). ..."]
pub struct DIVQ3_R(crate::FieldReader<u8, DIVQ3_A>);
impl DIVQ3_R {
    pub(crate) fn new(bits: u8) -> Self {
        DIVQ3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DIVQ3_A> {
        match self.bits {
            0 => Some(DIVQ3_A::B_0X0),
            1 => Some(DIVQ3_A::B_0X1),
            2 => Some(DIVQ3_A::B_0X2),
            3 => Some(DIVQ3_A::B_0X3),
            127 => Some(DIVQ3_A::B_0X7F),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == DIVQ3_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == DIVQ3_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        **self == DIVQ3_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        **self == DIVQ3_A::B_0X3
    }
    #[doc = "Checks if the value of the field is `B_0X7F`"]
    #[inline(always)]
    pub fn is_b_0x7f(&self) -> bool {
        **self == DIVQ3_A::B_0X7F
    }
}
impl core::ops::Deref for DIVQ3_R {
    type Target = crate::FieldReader<u8, DIVQ3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIVQ3` writer - PLL3 DIVQ division factor Set and reset by software to control the frequency of the pll3_q_ck clock. These bits can be written only when the PLL3 is disabled (PLL3ON = PLL3RDY = 0). ..."]
pub struct DIVQ3_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVQ3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIVQ3_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "pll3_q_ck = vco3_ck"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DIVQ3_A::B_0X0)
    }
    #[doc = "pll3_q_ck = vco3_ck / 2 (default after reset)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DIVQ3_A::B_0X1)
    }
    #[doc = "pll3_q_ck = vco3_ck / 3"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(DIVQ3_A::B_0X2)
    }
    #[doc = "pll3_q_ck = vco3_ck / 4"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(DIVQ3_A::B_0X3)
    }
    #[doc = "pll3_q_ck = vco3_ck / 128"]
    #[inline(always)]
    pub fn b_0x7f(self) -> &'a mut W {
        self.variant(DIVQ3_A::B_0X7F)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | ((value as u32 & 0x7f) << 16);
        self.w
    }
}
#[doc = "PLL3 DIVR division factor Set and reset by software to control the frequency of the pll3_r_ck clock. These bits can be written only when the PLL3 is disabled (PLL3ON = PLL3RDY = 0). ...\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DIVR3_A {
    #[doc = "0: pll3_r_ck = vco3_ck "]
    B_0X0 = 0,
    #[doc = "1: pll3_r_ck = vco3_ck / 2 (default after reset)"]
    B_0X1 = 1,
    #[doc = "2: pll3_r_ck = vco3_ck / 3"]
    B_0X2 = 2,
    #[doc = "3: pll3_r_ck = vco3_ck / 4"]
    B_0X3 = 3,
    #[doc = "127: pll3_r_ck = vco3_ck / 128"]
    B_0X7F = 127,
}
impl From<DIVR3_A> for u8 {
    #[inline(always)]
    fn from(variant: DIVR3_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DIVR3` reader - PLL3 DIVR division factor Set and reset by software to control the frequency of the pll3_r_ck clock. These bits can be written only when the PLL3 is disabled (PLL3ON = PLL3RDY = 0). ..."]
pub struct DIVR3_R(crate::FieldReader<u8, DIVR3_A>);
impl DIVR3_R {
    pub(crate) fn new(bits: u8) -> Self {
        DIVR3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DIVR3_A> {
        match self.bits {
            0 => Some(DIVR3_A::B_0X0),
            1 => Some(DIVR3_A::B_0X1),
            2 => Some(DIVR3_A::B_0X2),
            3 => Some(DIVR3_A::B_0X3),
            127 => Some(DIVR3_A::B_0X7F),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == DIVR3_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == DIVR3_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        **self == DIVR3_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        **self == DIVR3_A::B_0X3
    }
    #[doc = "Checks if the value of the field is `B_0X7F`"]
    #[inline(always)]
    pub fn is_b_0x7f(&self) -> bool {
        **self == DIVR3_A::B_0X7F
    }
}
impl core::ops::Deref for DIVR3_R {
    type Target = crate::FieldReader<u8, DIVR3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIVR3` writer - PLL3 DIVR division factor Set and reset by software to control the frequency of the pll3_r_ck clock. These bits can be written only when the PLL3 is disabled (PLL3ON = PLL3RDY = 0). ..."]
pub struct DIVR3_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVR3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIVR3_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "pll3_r_ck = vco3_ck"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DIVR3_A::B_0X0)
    }
    #[doc = "pll3_r_ck = vco3_ck / 2 (default after reset)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DIVR3_A::B_0X1)
    }
    #[doc = "pll3_r_ck = vco3_ck / 3"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(DIVR3_A::B_0X2)
    }
    #[doc = "pll3_r_ck = vco3_ck / 4"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(DIVR3_A::B_0X3)
    }
    #[doc = "pll3_r_ck = vco3_ck / 128"]
    #[inline(always)]
    pub fn b_0x7f(self) -> &'a mut W {
        self.variant(DIVR3_A::B_0X7F)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 24)) | ((value as u32 & 0x7f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - Multiplication factor for PLL3 VCO Set and reset by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled (PLL3ON = PLL3RDY = 0). ...........: not used ... ... Others: wrong configurations The software must set correctly these bits to insure that the VCO output frequency is between its valid frequency range, that is: 128 to 560Â MHz if PLL3VCOSEL = 0 150 to 420Â MHz if PLL3VCOSEL = 1 VCO output frequency = Fref3_ck x DIVN3, when fractional value 0 has been loaded into FRACN3, with: DIVN3 between 8 and 420 The input frequency Fref3_ck must be between 1 and 16MHz"]
    #[inline(always)]
    pub fn divn3(&self) -> DIVN3_R {
        DIVN3_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:15 - PLL3 DIVP division factor Set and reset by software to control the frequency of the pll3_p_ck clock. These bits can be written only when the PLL3 is disabled (PLL3ON = PLL3RDY = 0). ..."]
    #[inline(always)]
    pub fn divp3(&self) -> DIVP3_R {
        DIVP3_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - PLL3 DIVQ division factor Set and reset by software to control the frequency of the pll3_q_ck clock. These bits can be written only when the PLL3 is disabled (PLL3ON = PLL3RDY = 0). ..."]
    #[inline(always)]
    pub fn divq3(&self) -> DIVQ3_R {
        DIVQ3_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - PLL3 DIVR division factor Set and reset by software to control the frequency of the pll3_r_ck clock. These bits can be written only when the PLL3 is disabled (PLL3ON = PLL3RDY = 0). ..."]
    #[inline(always)]
    pub fn divr3(&self) -> DIVR3_R {
        DIVR3_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8 - Multiplication factor for PLL3 VCO Set and reset by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled (PLL3ON = PLL3RDY = 0). ...........: not used ... ... Others: wrong configurations The software must set correctly these bits to insure that the VCO output frequency is between its valid frequency range, that is: 128 to 560Â MHz if PLL3VCOSEL = 0 150 to 420Â MHz if PLL3VCOSEL = 1 VCO output frequency = Fref3_ck x DIVN3, when fractional value 0 has been loaded into FRACN3, with: DIVN3 between 8 and 420 The input frequency Fref3_ck must be between 1 and 16MHz"]
    #[inline(always)]
    pub fn divn3(&mut self) -> DIVN3_W {
        DIVN3_W { w: self }
    }
    #[doc = "Bits 9:15 - PLL3 DIVP division factor Set and reset by software to control the frequency of the pll3_p_ck clock. These bits can be written only when the PLL3 is disabled (PLL3ON = PLL3RDY = 0). ..."]
    #[inline(always)]
    pub fn divp3(&mut self) -> DIVP3_W {
        DIVP3_W { w: self }
    }
    #[doc = "Bits 16:22 - PLL3 DIVQ division factor Set and reset by software to control the frequency of the pll3_q_ck clock. These bits can be written only when the PLL3 is disabled (PLL3ON = PLL3RDY = 0). ..."]
    #[inline(always)]
    pub fn divq3(&mut self) -> DIVQ3_W {
        DIVQ3_W { w: self }
    }
    #[doc = "Bits 24:30 - PLL3 DIVR division factor Set and reset by software to control the frequency of the pll3_r_ck clock. These bits can be written only when the PLL3 is disabled (PLL3ON = PLL3RDY = 0). ..."]
    #[inline(always)]
    pub fn divr3(&mut self) -> DIVR3_W {
        DIVR3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll3divr](index.html) module"]
pub struct PLL3DIVR_SPEC;
impl crate::RegisterSpec for PLL3DIVR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pll3divr::R](R) reader structure"]
impl crate::Readable for PLL3DIVR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pll3divr::W](W) writer structure"]
impl crate::Writable for PLL3DIVR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PLL3DIVR to value 0x0101_0280"]
impl crate::Resettable for PLL3DIVR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0101_0280
    }
}
