#[doc = "Register `HWCFGR7` reader"]
pub type R = crate::R<HWCFGR7rs>;
#[doc = "Field `CPUEVENT` reader - HW configuration CPU event generation"]
pub type CPUEVENT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - HW configuration CPU event generation"]
    #[inline(always)]
    pub fn cpuevent(&self) -> CPUEVENT_R {
        CPUEVENT_R::new(self.bits)
    }
}
#[doc = "EXTI Hardware configuration registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwcfgr7::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HWCFGR7rs;
impl crate::RegisterSpec for HWCFGR7rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwcfgr7::R`](R) reader structure"]
impl crate::Readable for HWCFGR7rs {}
#[doc = "`reset()` method sets HWCFGR7 to value 0"]
impl crate::Resettable for HWCFGR7rs {
    const RESET_VALUE: u32 = 0;
}
