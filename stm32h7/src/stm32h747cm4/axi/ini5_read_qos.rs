#[doc = "Register `INI5_READ_QOS` reader"]
pub struct R(crate::R<INI5_READ_QOS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INI5_READ_QOS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INI5_READ_QOS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INI5_READ_QOS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INI5_READ_QOS` writer"]
pub struct W(crate::W<INI5_READ_QOS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INI5_READ_QOS_SPEC>;
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
impl From<crate::W<INI5_READ_QOS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INI5_READ_QOS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AR_QOS` reader - Read channel QoS setting"]
pub struct AR_QOS_R(crate::FieldReader<u8, u8>);
impl AR_QOS_R {
    pub(crate) fn new(bits: u8) -> Self {
        AR_QOS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AR_QOS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AR_QOS` writer - Read channel QoS setting"]
pub struct AR_QOS_W<'a> {
    w: &'a mut W,
}
impl<'a> AR_QOS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Read channel QoS setting"]
    #[inline(always)]
    pub fn ar_qos(&self) -> AR_QOS_R {
        AR_QOS_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Read channel QoS setting"]
    #[inline(always)]
    pub fn ar_qos(&mut self) -> AR_QOS_W {
        AR_QOS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AXI interconnect - INI x read QoS register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ini5_read_qos](index.html) module"]
pub struct INI5_READ_QOS_SPEC;
impl crate::RegisterSpec for INI5_READ_QOS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ini5_read_qos::R](R) reader structure"]
impl crate::Readable for INI5_READ_QOS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ini5_read_qos::W](W) writer structure"]
impl crate::Writable for INI5_READ_QOS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INI5_READ_QOS to value 0x04"]
impl crate::Resettable for INI5_READ_QOS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x04
    }
}
