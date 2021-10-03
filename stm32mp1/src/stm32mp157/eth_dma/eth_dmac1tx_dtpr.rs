#[doc = "Register `ETH_DMAC1TxDTPR` reader"]
pub struct R(crate::R<ETH_DMAC1TXDTPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_DMAC1TXDTPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_DMAC1TXDTPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_DMAC1TXDTPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_DMAC1TxDTPR` writer"]
pub struct W(crate::W<ETH_DMAC1TXDTPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_DMAC1TXDTPR_SPEC>;
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
impl From<crate::W<ETH_DMAC1TXDTPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_DMAC1TXDTPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TDT` reader - Transmit Descriptor Tail Pointer"]
pub struct TDT_R(crate::FieldReader<u32, u32>);
impl TDT_R {
    pub(crate) fn new(bits: u32) -> Self {
        TDT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TDT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TDT` writer - Transmit Descriptor Tail Pointer"]
pub struct TDT_W<'a> {
    w: &'a mut W,
}
impl<'a> TDT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff_ffff << 3)) | ((value as u32 & 0x1fff_ffff) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:31 - Transmit Descriptor Tail Pointer"]
    #[inline(always)]
    pub fn tdt(&self) -> TDT_R {
        TDT_R::new(((self.bits >> 3) & 0x1fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 3:31 - Transmit Descriptor Tail Pointer"]
    #[inline(always)]
    pub fn tdt(&mut self) -> TDT_W {
        TDT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Tx descriptor tail pointer register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_dmac1tx_dtpr](index.html) module"]
pub struct ETH_DMAC1TXDTPR_SPEC;
impl crate::RegisterSpec for ETH_DMAC1TXDTPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_dmac1tx_dtpr::R](R) reader structure"]
impl crate::Readable for ETH_DMAC1TXDTPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_dmac1tx_dtpr::W](W) writer structure"]
impl crate::Writable for ETH_DMAC1TXDTPR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ETH_DMAC1TxDTPR to value 0"]
impl crate::Resettable for ETH_DMAC1TXDTPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
