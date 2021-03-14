#[doc = "Reader of register PTPPPSCR"]
pub type R = crate::R<u32, super::PTPPPSCR>;
#[doc = "Reader of field `PPSFREQ`"]
pub type PPSFREQ_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - PPS frequency selection"]
    #[inline(always)]
    pub fn ppsfreq(&self) -> PPSFREQ_R {
        PPSFREQ_R::new((self.bits & 0x0f) as u8)
    }
}
