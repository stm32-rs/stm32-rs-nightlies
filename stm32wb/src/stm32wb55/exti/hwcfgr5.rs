#[doc = "Reader of register HWCFGR5"]
pub type R = crate::R<u32, super::HWCFGR5>;
#[doc = "Reader of field `CPUEVENT`"]
pub type CPUEVENT_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - HW configuration CPU event generation"]
    #[inline(always)]
    pub fn cpuevent(&self) -> CPUEVENT_R {
        CPUEVENT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
