#[doc = "Register `HWCFGR6` reader"]
pub type R = crate::R<HWCFGR6rs>;
#[doc = "Field `CPUEVENT` reader - HW configuration CPU event generation"]
pub type CPUEVENT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - HW configuration CPU event generation"]
    #[inline(always)]
    pub fn cpuevent(&self) -> CPUEVENT_R {
        CPUEVENT_R::new(self.bits)
    }
}
#[doc = "Hardware configuration registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwcfgr6::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HWCFGR6rs;
impl crate::RegisterSpec for HWCFGR6rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwcfgr6::R`](R) reader structure"]
impl crate::Readable for HWCFGR6rs {}
#[doc = "`reset()` method sets HWCFGR6 to value 0x0300"]
impl crate::Resettable for HWCFGR6rs {
    const RESET_VALUE: u32 = 0x0300;
}
