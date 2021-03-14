#[doc = "Reader of register FMC_SR"]
pub type R = crate::R<u32, super::FMC_SR>;
#[doc = "Reader of field `ISOST`"]
pub type ISOST_R = crate::R<u8, u8>;
#[doc = "Reader of field `PEF`"]
pub type PEF_R = crate::R<bool, bool>;
#[doc = "Reader of field `NWRF`"]
pub type NWRF_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:1 - ISOST"]
    #[inline(always)]
    pub fn isost(&self) -> ISOST_R {
        ISOST_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 4 - PEF"]
    #[inline(always)]
    pub fn pef(&self) -> PEF_R {
        PEF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - NWRF"]
    #[inline(always)]
    pub fn nwrf(&self) -> NWRF_R {
        NWRF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
