#[doc = "Register `DIVL` reader"]
pub type R = crate::R<DIVLrs>;
#[doc = "Field `DIVL` reader - RTC prescaler divider register Low"]
pub type DIVL_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - RTC prescaler divider register Low"]
    #[inline(always)]
    pub fn divl(&self) -> DIVL_R {
        DIVL_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "RTC Prescaler Divider Register Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`divl::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIVLrs;
impl crate::RegisterSpec for DIVLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`divl::R`](R) reader structure"]
impl crate::Readable for DIVLrs {}
#[doc = "`reset()` method sets DIVL to value 0x8000"]
impl crate::Resettable for DIVLrs {
    const RESET_VALUE: u32 = 0x8000;
}
