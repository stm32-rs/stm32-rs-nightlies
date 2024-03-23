#[doc = "Register `DMACHTDR` reader"]
pub type R = crate::R<DMACHTDRrs>;
#[doc = "Field `HTDAP` reader - Host transmit descriptor address pointer"]
pub type HTDAP_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Host transmit descriptor address pointer"]
    #[inline(always)]
    pub fn htdap(&self) -> HTDAP_R {
        HTDAP_R::new(self.bits)
    }
}
#[doc = "Ethernet DMA current host transmit descriptor register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmachtdr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMACHTDRrs;
impl crate::RegisterSpec for DMACHTDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmachtdr::R`](R) reader structure"]
impl crate::Readable for DMACHTDRrs {}
#[doc = "`reset()` method sets DMACHTDR to value 0"]
impl crate::Resettable for DMACHTDRrs {
    const RESET_VALUE: u32 = 0;
}
