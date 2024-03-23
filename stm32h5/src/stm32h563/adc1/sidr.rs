#[doc = "Register `SIDR` reader"]
pub type R = crate::R<SIDRrs>;
#[doc = "Field `SID` reader - Size Identification SID\\[31:8\\]: fixed code that characterizes the ADC_SIDR register. This field is always read at 0xA3C5DD. SID\\[7:0\\]: read-only numeric field that returns the address offset (in Kbytes) of the identification registers from the IP base address:"]
pub type SID_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Size Identification SID\\[31:8\\]: fixed code that characterizes the ADC_SIDR register. This field is always read at 0xA3C5DD. SID\\[7:0\\]: read-only numeric field that returns the address offset (in Kbytes) of the identification registers from the IP base address:"]
    #[inline(always)]
    pub fn sid(&self) -> SID_R {
        SID_R::new(self.bits)
    }
}
#[doc = "ADC size identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sidr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SIDRrs;
impl crate::RegisterSpec for SIDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sidr::R`](R) reader structure"]
impl crate::Readable for SIDRrs {}
#[doc = "`reset()` method sets SIDR to value 0xa3c5_dd01"]
impl crate::Resettable for SIDRrs {
    const RESET_VALUE: u32 = 0xa3c5_dd01;
}
