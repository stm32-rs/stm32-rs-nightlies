#[doc = "Reader of register ETH_MACTxQPMR"]
pub type R = crate::R<u32, super::ETH_MACTXQPMR>;
#[doc = "Reader of field `PSTQ0`"]
pub type PSTQ0_R = crate::R<u8, u8>;
#[doc = "Reader of field `PSTQ1`"]
pub type PSTQ1_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - PSTQ0"]
    #[inline(always)]
    pub fn pstq0(&self) -> PSTQ0_R {
        PSTQ0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - PSTQ1"]
    #[inline(always)]
    pub fn pstq1(&self) -> PSTQ1_R {
        PSTQ1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
