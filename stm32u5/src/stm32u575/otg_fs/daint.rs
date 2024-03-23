#[doc = "Register `DAINT` reader"]
pub type R = crate::R<DAINTrs>;
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
#[doc = "When a significant event occurs on an endpoint, a DAINT register interrupts the application using the device OUT endpoints interrupt bit or device IN endpoints interrupt bit of the GINTSTS register (OEPINT or IEPINT in GINTSTS, respectively). There is one interrupt bit per endpoint, up to a maximum of 16 bits for OUT endpoints and 16 bits for IN endpoints. For a bidirectional endpoint, the corresponding IN and OUT interrupt bits are used. Bits in this register are set and cleared when the application sets and clears bits in the corresponding device endpoint-x interrupt register (DIEPINTx/DOEPINTx).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`daint::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DAINTrs;
impl crate::RegisterSpec for DAINTrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`daint::R`](R) reader structure"]
impl crate::Readable for DAINTrs {}
#[doc = "`reset()` method sets DAINT to value 0"]
impl crate::Resettable for DAINTrs {
    const RESET_VALUE: u32 = 0;
}
