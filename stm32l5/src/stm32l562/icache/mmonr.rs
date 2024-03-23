#[doc = "Register `MMONR` reader"]
pub type R = crate::R<MMONRrs>;
#[doc = "Field `MISSMON` reader - MISSMON"]
pub type MISSMON_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - MISSMON"]
    #[inline(always)]
    pub fn missmon(&self) -> MISSMON_R {
        MISSMON_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "ICACHE miss monitor register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmonr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MMONRrs;
impl crate::RegisterSpec for MMONRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmonr::R`](R) reader structure"]
impl crate::Readable for MMONRrs {}
#[doc = "`reset()` method sets MMONR to value 0"]
impl crate::Resettable for MMONRrs {
    const RESET_VALUE: u32 = 0;
}
