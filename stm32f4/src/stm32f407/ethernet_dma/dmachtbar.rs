#[doc = "Register `DMACHTBAR` reader"]
pub type R = crate::R<DMACHTBARrs>;
#[doc = "Field `HTBAP` reader - Host transmit buffer address pointer"]
pub type HTBAP_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Host transmit buffer address pointer"]
    #[inline(always)]
    pub fn htbap(&self) -> HTBAP_R {
        HTBAP_R::new(self.bits)
    }
}
#[doc = "Ethernet DMA current host transmit buffer address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmachtbar::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMACHTBARrs;
impl crate::RegisterSpec for DMACHTBARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmachtbar::R`](R) reader structure"]
impl crate::Readable for DMACHTBARrs {}
#[doc = "`reset()` method sets DMACHTBAR to value 0"]
impl crate::Resettable for DMACHTBARrs {
    const RESET_VALUE: u32 = 0;
}
