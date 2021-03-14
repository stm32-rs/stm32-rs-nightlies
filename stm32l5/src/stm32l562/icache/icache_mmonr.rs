#[doc = "Reader of register ICACHE_MMONR"]
pub type R = crate::R<u32, super::ICACHE_MMONR>;
#[doc = "Reader of field `MISSMON`"]
pub type MISSMON_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - MISSMON"]
    #[inline(always)]
    pub fn missmon(&self) -> MISSMON_R {
        MISSMON_R::new((self.bits & 0xffff) as u16)
    }
}
