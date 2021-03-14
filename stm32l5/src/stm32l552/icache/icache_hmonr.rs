#[doc = "Reader of register ICACHE_HMONR"]
pub type R = crate::R<u32, super::ICACHE_HMONR>;
#[doc = "Reader of field `HITMON`"]
pub type HITMON_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - HITMON"]
    #[inline(always)]
    pub fn hitmon(&self) -> HITMON_R {
        HITMON_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
