#[doc = "Register `DMACHRDR` reader"]
pub type R = crate::R<DMACHRDRrs>;
#[doc = "Field `HRDAP` reader - Host receive descriptor address pointer"]
pub type HRDAP_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Host receive descriptor address pointer"]
    #[inline(always)]
    pub fn hrdap(&self) -> HRDAP_R {
        HRDAP_R::new(self.bits)
    }
}
#[doc = "Ethernet DMA current host receive descriptor register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmachrdr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMACHRDRrs;
impl crate::RegisterSpec for DMACHRDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmachrdr::R`](R) reader structure"]
impl crate::Readable for DMACHRDRrs {}
#[doc = "`reset()` method sets DMACHRDR to value 0"]
impl crate::Resettable for DMACHRDRrs {
    const RESET_VALUE: u32 = 0;
}
