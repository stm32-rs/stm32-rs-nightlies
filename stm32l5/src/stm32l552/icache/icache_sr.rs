#[doc = "Reader of register ICACHE_SR"]
pub type R = crate::R<u32, super::ICACHE_SR>;
#[doc = "Reader of field `BUSYF`"]
pub type BUSYF_R = crate::R<bool, bool>;
#[doc = "Reader of field `BSYENDF`"]
pub type BSYENDF_R = crate::R<bool, bool>;
#[doc = "Reader of field `ERRF`"]
pub type ERRF_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - BUSYF"]
    #[inline(always)]
    pub fn busyf(&self) -> BUSYF_R {
        BUSYF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - BSYENDF"]
    #[inline(always)]
    pub fn bsyendf(&self) -> BSYENDF_R {
        BSYENDF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ERRF"]
    #[inline(always)]
    pub fn errf(&self) -> ERRF_R {
        ERRF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
