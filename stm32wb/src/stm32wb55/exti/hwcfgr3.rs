#[doc = "Register `HWCFGR3` reader"]
pub type R = crate::R<HWCFGR3rs>;
#[doc = "Field `EVENT_TRG` reader - HW configuration event trigger type"]
pub type EVENT_TRG_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - HW configuration event trigger type"]
    #[inline(always)]
    pub fn event_trg(&self) -> EVENT_TRG_R {
        EVENT_TRG_R::new(self.bits)
    }
}
#[doc = "Hardware configuration registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwcfgr3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HWCFGR3rs;
impl crate::RegisterSpec for HWCFGR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwcfgr3::R`](R) reader structure"]
impl crate::Readable for HWCFGR3rs {}
#[doc = "`reset()` method sets HWCFGR3 to value 0x0302"]
impl crate::Resettable for HWCFGR3rs {
    const RESET_VALUE: u32 = 0x0302;
}
