#[doc = "Reader of register HWCFGR3"]
pub type R = crate::R<u32, super::HWCFGR3>;
#[doc = "Reader of field `EVENT_TRG`"]
pub type EVENT_TRG_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - HW configuration event trigger type"]
    #[inline(always)]
    pub fn event_trg(&self) -> EVENT_TRG_R {
        EVENT_TRG_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
