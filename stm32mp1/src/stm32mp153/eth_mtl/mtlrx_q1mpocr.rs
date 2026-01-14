///Register `MTLRxQ1MPOCR` reader
pub type R = crate::R<MTLRX_Q1MPOCRrs>;
///Field `OVFPKTCNT` reader - OVFPKTCNT
pub type OVFPKTCNT_R = crate::FieldReader<u16>;
///Field `OVFCNTOVF` reader - OVFCNTOVF
pub type OVFCNTOVF_R = crate::BitReader;
///Field `MISPKTCNT` reader - MISPKTCNT
pub type MISPKTCNT_R = crate::FieldReader<u16>;
///Field `MISCNTOVF` reader - MISCNTOVF
pub type MISCNTOVF_R = crate::BitReader;
impl R {
    ///Bits 0:10 - OVFPKTCNT
    #[inline(always)]
    pub fn ovfpktcnt(&self) -> OVFPKTCNT_R {
        OVFPKTCNT_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bit 11 - OVFCNTOVF
    #[inline(always)]
    pub fn ovfcntovf(&self) -> OVFCNTOVF_R {
        OVFCNTOVF_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 16:26 - MISPKTCNT
    #[inline(always)]
    pub fn mispktcnt(&self) -> MISPKTCNT_R {
        MISPKTCNT_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    ///Bit 27 - MISCNTOVF
    #[inline(always)]
    pub fn miscntovf(&self) -> MISCNTOVF_R {
        MISCNTOVF_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MTLRxQ1MPOCR")
            .field("ovfpktcnt", &self.ovfpktcnt())
            .field("ovfcntovf", &self.ovfcntovf())
            .field("mispktcnt", &self.mispktcnt())
            .field("miscntovf", &self.miscntovf())
            .finish()
    }
}
/**Rx queue 1 missed packet and overflow counter register

You can [`read`](crate::Reg::read) this register and get [`mtlrx_q1mpocr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MTL:MTLRxQ1MPOCR)*/
pub struct MTLRX_Q1MPOCRrs;
impl crate::RegisterSpec for MTLRX_Q1MPOCRrs {
    type Ux = u32;
}
///`read()` method returns [`mtlrx_q1mpocr::R`](R) reader structure
impl crate::Readable for MTLRX_Q1MPOCRrs {}
///`reset()` method sets MTLRxQ1MPOCR to value 0
impl crate::Resettable for MTLRX_Q1MPOCRrs {}
