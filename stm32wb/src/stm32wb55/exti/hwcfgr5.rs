#[doc = "Register `HWCFGR5` reader"]
pub type R = crate::R<HWCFGR5rs>;
#[doc = "Field `CPUEVENT` reader - HW configuration CPU event generation"]
pub type CPUEVENT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - HW configuration CPU event generation"]
    #[inline(always)]
    pub fn cpuevent(&self) -> CPUEVENT_R {
        CPUEVENT_R::new(self.bits)
    }
}
#[doc = "Hardware configuration registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwcfgr5::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HWCFGR5rs;
impl crate::RegisterSpec for HWCFGR5rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwcfgr5::R`](R) reader structure"]
impl crate::Readable for HWCFGR5rs {}
#[doc = "`reset()` method sets HWCFGR5 to value 0x003e_ffff"]
impl crate::Resettable for HWCFGR5rs {
    const RESET_VALUE: u32 = 0x003e_ffff;
}
