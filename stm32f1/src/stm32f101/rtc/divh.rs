#[doc = "Register `DIVH` reader"]
pub type R = crate::R<DIVHrs>;
#[doc = "Field `DIVH` reader - RTC prescaler divider register high"]
pub type DIVH_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - RTC prescaler divider register high"]
    #[inline(always)]
    pub fn divh(&self) -> DIVH_R {
        DIVH_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "RTC Prescaler Divider Register High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`divh::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIVHrs;
impl crate::RegisterSpec for DIVHrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`divh::R`](R) reader structure"]
impl crate::Readable for DIVHrs {}
#[doc = "`reset()` method sets DIVH to value 0"]
impl crate::Resettable for DIVHrs {
    const RESET_VALUE: u32 = 0;
}
