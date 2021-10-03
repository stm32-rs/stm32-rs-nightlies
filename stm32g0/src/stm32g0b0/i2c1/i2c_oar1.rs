#[doc = "Register `I2C_OAR1` reader"]
pub struct R(crate::R<I2C_OAR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_OAR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_OAR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_OAR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C_OAR1` writer"]
pub struct W(crate::W<I2C_OAR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_OAR1_SPEC>;
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
impl From<crate::W<I2C_OAR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C_OAR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OA1` reader - Interface own slave address 7-bit addressing mode: OA1\\[7:1\\]
contains the 7-bit own slave address. The bits OA1\\[9\\], OA1\\[8\\]
and OA1\\[0\\]
are don't care. 10-bit addressing mode: OA1\\[9:0\\]
contains the 10-bit own slave address. Note: These bits can be written only when OA1EN=0."]
pub struct OA1_R(crate::FieldReader<u16, u16>);
impl OA1_R {
    pub(crate) fn new(bits: u16) -> Self {
        OA1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OA1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OA1` writer - Interface own slave address 7-bit addressing mode: OA1\\[7:1\\]
contains the 7-bit own slave address. The bits OA1\\[9\\], OA1\\[8\\]
and OA1\\[0\\]
are don't care. 10-bit addressing mode: OA1\\[9:0\\]
contains the 10-bit own slave address. Note: These bits can be written only when OA1EN=0."]
pub struct OA1_W<'a> {
    w: &'a mut W,
}
impl<'a> OA1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
#[doc = "Own Address 1 10-bit mode Note: This bit can be written only when OA1EN=0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OA1MODE_A {
    #[doc = "0: Own address 1 is a 7-bit address."]
    B_0X0 = 0,
    #[doc = "1: Own address 1 is a 10-bit address."]
    B_0X1 = 1,
}
impl From<OA1MODE_A> for bool {
    #[inline(always)]
    fn from(variant: OA1MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OA1MODE` reader - Own Address 1 10-bit mode Note: This bit can be written only when OA1EN=0."]
pub struct OA1MODE_R(crate::FieldReader<bool, OA1MODE_A>);
impl OA1MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        OA1MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OA1MODE_A {
        match self.bits {
            false => OA1MODE_A::B_0X0,
            true => OA1MODE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == OA1MODE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == OA1MODE_A::B_0X1
    }
}
impl core::ops::Deref for OA1MODE_R {
    type Target = crate::FieldReader<bool, OA1MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OA1MODE` writer - Own Address 1 10-bit mode Note: This bit can be written only when OA1EN=0."]
pub struct OA1MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> OA1MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OA1MODE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Own address 1 is a 7-bit address."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(OA1MODE_A::B_0X0)
    }
    #[doc = "Own address 1 is a 10-bit address."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(OA1MODE_A::B_0X1)
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
#[doc = "Own Address 1 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OA1EN_A {
    #[doc = "0: Own address 1 disabled. The received slave address OA1 is NACKed."]
    B_0X0 = 0,
    #[doc = "1: Own address 1 enabled. The received slave address OA1 is ACKed."]
    B_0X1 = 1,
}
impl From<OA1EN_A> for bool {
    #[inline(always)]
    fn from(variant: OA1EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OA1EN` reader - Own Address 1 enable"]
pub struct OA1EN_R(crate::FieldReader<bool, OA1EN_A>);
impl OA1EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        OA1EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OA1EN_A {
        match self.bits {
            false => OA1EN_A::B_0X0,
            true => OA1EN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == OA1EN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == OA1EN_A::B_0X1
    }
}
impl core::ops::Deref for OA1EN_R {
    type Target = crate::FieldReader<bool, OA1EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OA1EN` writer - Own Address 1 enable"]
pub struct OA1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> OA1EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OA1EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Own address 1 disabled. The received slave address OA1 is NACKed."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(OA1EN_A::B_0X0)
    }
    #[doc = "Own address 1 enabled. The received slave address OA1 is ACKed."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(OA1EN_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Interface own slave address 7-bit addressing mode: OA1\\[7:1\\]
contains the 7-bit own slave address. The bits OA1\\[9\\], OA1\\[8\\]
and OA1\\[0\\]
are don't care. 10-bit addressing mode: OA1\\[9:0\\]
contains the 10-bit own slave address. Note: These bits can be written only when OA1EN=0."]
    #[inline(always)]
    pub fn oa1(&self) -> OA1_R {
        OA1_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - Own Address 1 10-bit mode Note: This bit can be written only when OA1EN=0."]
    #[inline(always)]
    pub fn oa1mode(&self) -> OA1MODE_R {
        OA1MODE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Own Address 1 enable"]
    #[inline(always)]
    pub fn oa1en(&self) -> OA1EN_R {
        OA1EN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Interface own slave address 7-bit addressing mode: OA1\\[7:1\\]
contains the 7-bit own slave address. The bits OA1\\[9\\], OA1\\[8\\]
and OA1\\[0\\]
are don't care. 10-bit addressing mode: OA1\\[9:0\\]
contains the 10-bit own slave address. Note: These bits can be written only when OA1EN=0."]
    #[inline(always)]
    pub fn oa1(&mut self) -> OA1_W {
        OA1_W { w: self }
    }
    #[doc = "Bit 10 - Own Address 1 10-bit mode Note: This bit can be written only when OA1EN=0."]
    #[inline(always)]
    pub fn oa1mode(&mut self) -> OA1MODE_W {
        OA1MODE_W { w: self }
    }
    #[doc = "Bit 15 - Own Address 1 enable"]
    #[inline(always)]
    pub fn oa1en(&mut self) -> OA1EN_W {
        OA1EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Own address register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_oar1](index.html) module"]
pub struct I2C_OAR1_SPEC;
impl crate::RegisterSpec for I2C_OAR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c_oar1::R](R) reader structure"]
impl crate::Readable for I2C_OAR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c_oar1::W](W) writer structure"]
impl crate::Writable for I2C_OAR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2C_OAR1 to value 0"]
impl crate::Resettable for I2C_OAR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
