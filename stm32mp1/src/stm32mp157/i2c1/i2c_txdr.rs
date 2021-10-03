#[doc = "Register `I2C_TXDR` reader"]
pub struct R(crate::R<I2C_TXDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_TXDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_TXDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_TXDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C_TXDR` writer"]
pub struct W(crate::W<I2C_TXDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_TXDR_SPEC>;
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
impl From<crate::W<I2C_TXDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C_TXDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXDATA` reader - TXDATA"]
pub struct TXDATA_R(crate::FieldReader<u8, u8>);
impl TXDATA_R {
    pub(crate) fn new(bits: u8) -> Self {
        TXDATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXDATA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXDATA` writer - TXDATA"]
pub struct TXDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - TXDATA"]
    #[inline(always)]
    pub fn txdata(&self) -> TXDATA_R {
        TXDATA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - TXDATA"]
    #[inline(always)]
    pub fn txdata(&mut self) -> TXDATA_W {
        TXDATA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Access: No wait states\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_txdr](index.html) module"]
pub struct I2C_TXDR_SPEC;
impl crate::RegisterSpec for I2C_TXDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c_txdr::R](R) reader structure"]
impl crate::Readable for I2C_TXDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c_txdr::W](W) writer structure"]
impl crate::Writable for I2C_TXDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2C_TXDR to value 0"]
impl crate::Resettable for I2C_TXDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
