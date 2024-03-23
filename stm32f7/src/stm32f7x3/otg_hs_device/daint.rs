#[doc = "Register `DAINT` reader"]
pub type R = crate::R<DAINTrs>;
#[doc = "Field `IEPINT` reader - IN endpoint interrupt bits"]
pub type IEPINT_R = crate::FieldReader<u16>;
#[doc = "Field `OEPINT` reader - OUT endpoint interrupt bits"]
pub type OEPINT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - IN endpoint interrupt bits"]
    #[inline(always)]
    pub fn iepint(&self) -> IEPINT_R {
        IEPINT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - OUT endpoint interrupt bits"]
    #[inline(always)]
    pub fn oepint(&self) -> OEPINT_R {
        OEPINT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "OTG_HS device all endpoints interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`daint::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
