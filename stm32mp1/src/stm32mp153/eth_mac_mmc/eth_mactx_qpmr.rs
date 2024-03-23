#[doc = "Register `ETH_MACTxQPMR` reader"]
pub type R = crate::R<ETH_MACTX_QPMRrs>;
#[doc = "Field `PSTQ0` reader - PSTQ0"]
pub type PSTQ0_R = crate::FieldReader;
#[doc = "Field `PSTQ1` reader - PSTQ1"]
pub type PSTQ1_R = crate::FieldReader;
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
#[doc = "The transmit queue priority mapping 0 register contains the priority values assigned to Tx queue 0 and tx queue 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_mactx_qpmr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_MACTX_QPMRrs;
impl crate::RegisterSpec for ETH_MACTX_QPMRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_mactx_qpmr::R`](R) reader structure"]
impl crate::Readable for ETH_MACTX_QPMRrs {}
#[doc = "`reset()` method sets ETH_MACTxQPMR to value 0"]
impl crate::Resettable for ETH_MACTX_QPMRrs {
    const RESET_VALUE: u32 = 0;
}
