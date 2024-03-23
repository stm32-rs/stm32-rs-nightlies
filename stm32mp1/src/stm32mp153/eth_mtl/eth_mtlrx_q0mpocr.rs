#[doc = "Register `ETH_MTLRxQ0MPOCR` reader"]
pub type R = crate::R<ETH_MTLRX_Q0MPOCRrs>;
#[doc = "Field `OVFPKTCNT` reader - OVFPKTCNT"]
pub type OVFPKTCNT_R = crate::FieldReader<u16>;
#[doc = "Field `OVFCNTOVF` reader - OVFCNTOVF"]
pub type OVFCNTOVF_R = crate::BitReader;
#[doc = "Field `MISPKTCNT` reader - MISPKTCNT"]
pub type MISPKTCNT_R = crate::FieldReader<u16>;
#[doc = "Field `MISCNTOVF` reader - MISCNTOVF"]
pub type MISCNTOVF_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:10 - OVFPKTCNT"]
    #[inline(always)]
    pub fn ovfpktcnt(&self) -> OVFPKTCNT_R {
        OVFPKTCNT_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 11 - OVFCNTOVF"]
    #[inline(always)]
    pub fn ovfcntovf(&self) -> OVFCNTOVF_R {
        OVFCNTOVF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 16:26 - MISPKTCNT"]
    #[inline(always)]
    pub fn mispktcnt(&self) -> MISPKTCNT_R {
        MISPKTCNT_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    #[doc = "Bit 27 - MISCNTOVF"]
    #[inline(always)]
    pub fn miscntovf(&self) -> MISCNTOVF_R {
        MISCNTOVF_R::new(((self.bits >> 27) & 1) != 0)
    }
}
#[doc = "Rx queue 0 missed packet and overflow counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_mtlrx_q0mpocr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_MTLRX_Q0MPOCRrs;
impl crate::RegisterSpec for ETH_MTLRX_Q0MPOCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_mtlrx_q0mpocr::R`](R) reader structure"]
impl crate::Readable for ETH_MTLRX_Q0MPOCRrs {}
#[doc = "`reset()` method sets ETH_MTLRxQ0MPOCR to value 0"]
impl crate::Resettable for ETH_MTLRX_Q0MPOCRrs {
    const RESET_VALUE: u32 = 0;
}
