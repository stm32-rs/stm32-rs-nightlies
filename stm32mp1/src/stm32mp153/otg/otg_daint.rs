#[doc = "Register `OTG_DAINT` reader"]
pub type R = crate::R<OTG_DAINTrs>;
#[doc = "Field `IEPINT` reader - IEPINT"]
pub type IEPINT_R = crate::FieldReader<u16>;
#[doc = "Field `OEPINT` reader - OEPINT"]
pub type OEPINT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - IEPINT"]
    #[inline(always)]
    pub fn iepint(&self) -> IEPINT_R {
        IEPINT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - OEPINT"]
    #[inline(always)]
    pub fn oepint(&self) -> OEPINT_R {
        OEPINT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "When a significant event occurs on an endpoint, a OTG_DAINT register interrupts the application using the device OUT endpoints interrupt bit or device IN endpoints interrupt bit of the OTG_GINTSTS register (OEPINT or IEPINT in OTG_GINTSTS, respectively). There is one interrupt bit per endpoint, up to a maximum of 16 bits for OUT endpoints and 16 bits for IN endpoints. For a bidirectional endpoint, the corresponding IN and OUT interrupt bits are used. Bits in this register are set and cleared when the application sets and clears bits in the corresponding device endpoint-x interrupt register (OTG_DIEPINTx/OTG_DOEPINTx).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_daint::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTG_DAINTrs;
impl crate::RegisterSpec for OTG_DAINTrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_daint::R`](R) reader structure"]
impl crate::Readable for OTG_DAINTrs {}
#[doc = "`reset()` method sets OTG_DAINT to value 0"]
impl crate::Resettable for OTG_DAINTrs {
    const RESET_VALUE: u32 = 0;
}
