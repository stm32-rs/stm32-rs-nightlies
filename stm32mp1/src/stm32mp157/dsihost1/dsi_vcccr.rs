#[doc = "Reader of register DSI_VCCCR"]
pub type R = crate::R<u32, super::DSI_VCCCR>;
#[doc = "Reader of field `NUMC`"]
pub type NUMC_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:12 - NUMC"]
    #[inline(always)]
    pub fn numc(&self) -> NUMC_R {
        NUMC_R::new((self.bits & 0x1fff) as u16)
    }
}
