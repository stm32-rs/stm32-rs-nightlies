#[doc = "Register `FDCAN_RXF0A` reader"]
pub struct R(crate::R<FDCAN_RXF0A_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_RXF0A_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_RXF0A_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_RXF0A_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FDCAN_RXF0A` writer"]
pub struct W(crate::W<FDCAN_RXF0A_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCAN_RXF0A_SPEC>;
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
impl From<crate::W<FDCAN_RXF0A_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCAN_RXF0A_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FA01` reader - Rx FIFO 0 Acknowledge Index"]
pub struct FA01_R(crate::FieldReader<u8, u8>);
impl FA01_R {
    pub(crate) fn new(bits: u8) -> Self {
        FA01_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FA01_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FA01` writer - Rx FIFO 0 Acknowledge Index"]
pub struct FA01_W<'a> {
    w: &'a mut W,
}
impl<'a> FA01_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Rx FIFO 0 Acknowledge Index"]
    #[inline(always)]
    pub fn fa01(&self) -> FA01_R {
        FA01_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Rx FIFO 0 Acknowledge Index"]
    #[inline(always)]
    pub fn fa01(&mut self) -> FA01_W {
        FA01_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAN Rx FIFO 0 Acknowledge Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_rxf0a](index.html) module"]
pub struct FDCAN_RXF0A_SPEC;
impl crate::RegisterSpec for FDCAN_RXF0A_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_rxf0a::R](R) reader structure"]
impl crate::Readable for FDCAN_RXF0A_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fdcan_rxf0a::W](W) writer structure"]
impl crate::Writable for FDCAN_RXF0A_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FDCAN_RXF0A to value 0"]
impl crate::Resettable for FDCAN_RXF0A_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}