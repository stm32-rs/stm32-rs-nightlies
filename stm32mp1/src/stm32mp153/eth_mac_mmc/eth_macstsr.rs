#[doc = "Register `ETH_MACSTSR` reader"]
pub type R = crate::R<ETH_MACSTSRrs>;
#[doc = "Field `TSS` reader - TSS"]
pub type TSS_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - TSS"]
    #[inline(always)]
    pub fn tss(&self) -> TSS_R {
        TSS_R::new(self.bits)
    }
}
#[doc = "The System Time Seconds register, along with System Time Nanoseconds register, indicates the current value of the system time maintained by the MAC. Though it is updated on a continuous basis, there is some delay from the actual time because of clock domain transfer latencies (from HCLK to CSR clock). This register is present only when the IEEE 1588 Timestamp feature is selected without external timestamp input.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macstsr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_MACSTSRrs;
impl crate::RegisterSpec for ETH_MACSTSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_macstsr::R`](R) reader structure"]
impl crate::Readable for ETH_MACSTSRrs {}
#[doc = "`reset()` method sets ETH_MACSTSR to value 0"]
impl crate::Resettable for ETH_MACSTSRrs {
    const RESET_VALUE: u32 = 0;
}
