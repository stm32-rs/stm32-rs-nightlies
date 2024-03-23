#[doc = "Register `MACSTSR` reader"]
pub type R = crate::R<MACSTSRrs>;
#[doc = "Field `TSS` reader - Timestamp Second"]
pub type TSS_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Timestamp Second"]
    #[inline(always)]
    pub fn tss(&self) -> TSS_R {
        TSS_R::new(self.bits)
    }
}
#[doc = "System time seconds register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macstsr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACSTSRrs;
impl crate::RegisterSpec for MACSTSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macstsr::R`](R) reader structure"]
impl crate::Readable for MACSTSRrs {}
#[doc = "`reset()` method sets MACSTSR to value 0"]
impl crate::Resettable for MACSTSRrs {
    const RESET_VALUE: u32 = 0;
}
