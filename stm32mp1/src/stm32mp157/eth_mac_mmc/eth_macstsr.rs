#[doc = "Register `ETH_MACSTSR` reader"]
pub struct R(crate::R<ETH_MACSTSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACSTSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACSTSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACSTSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TSS` reader - TSS"]
pub struct TSS_R(crate::FieldReader<u32, u32>);
impl TSS_R {
    pub(crate) fn new(bits: u32) -> Self {
        TSS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSS_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - TSS"]
    #[inline(always)]
    pub fn tss(&self) -> TSS_R {
        TSS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "The System Time Seconds register, along with System Time Nanoseconds register, indicates the current value of the system time maintained by the MAC. Though it is updated on a continuous basis, there is some delay from the actual time because of clock domain transfer latencies (from HCLK to CSR clock). This register is present only when the IEEE 1588 Timestamp feature is selected without external timestamp input.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macstsr](index.html) module"]
pub struct ETH_MACSTSR_SPEC;
impl crate::RegisterSpec for ETH_MACSTSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_macstsr::R](R) reader structure"]
impl crate::Readable for ETH_MACSTSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ETH_MACSTSR to value 0"]
impl crate::Resettable for ETH_MACSTSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
