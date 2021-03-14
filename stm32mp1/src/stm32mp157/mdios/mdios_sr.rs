#[doc = "Reader of register MDIOS_SR"]
pub type R = crate::R<u32, super::MDIOS_SR>;
#[doc = "Reader of field `PERF`"]
pub type PERF_R = crate::R<bool, bool>;
#[doc = "Reader of field `SERF`"]
pub type SERF_R = crate::R<bool, bool>;
#[doc = "Reader of field `TERF`"]
pub type TERF_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - PERF"]
    #[inline(always)]
    pub fn perf(&self) -> PERF_R {
        PERF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SERF"]
    #[inline(always)]
    pub fn serf(&self) -> SERF_R {
        SERF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TERF"]
    #[inline(always)]
    pub fn terf(&self) -> TERF_R {
        TERF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
