///Register `MTLRxQMPOCR` reader
pub type R = crate::R<MTLRX_QMPOCRrs>;
///Field `OVFPKTCNT` reader - Overflow Packet Counter
pub type OVFPKTCNT_R = crate::FieldReader<u16>;
///Field `OVFCNTOVF` reader - Overflow Counter Overflow Bit
pub type OVFCNTOVF_R = crate::BitReader;
///Field `MISPKTCNT` reader - Missed Packet Counter
pub type MISPKTCNT_R = crate::FieldReader<u16>;
///Field `MISCNTOVF` reader - Missed Packet Counter Overflow Bit
pub type MISCNTOVF_R = crate::BitReader;
impl R {
    ///Bits 0:10 - Overflow Packet Counter
    #[inline(always)]
    pub fn ovfpktcnt(&self) -> OVFPKTCNT_R {
        OVFPKTCNT_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bit 11 - Overflow Counter Overflow Bit
    #[inline(always)]
    pub fn ovfcntovf(&self) -> OVFCNTOVF_R {
        OVFCNTOVF_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 16:26 - Missed Packet Counter
    #[inline(always)]
    pub fn mispktcnt(&self) -> MISPKTCNT_R {
        MISPKTCNT_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    ///Bit 27 - Missed Packet Counter Overflow Bit
    #[inline(always)]
    pub fn miscntovf(&self) -> MISCNTOVF_R {
        MISCNTOVF_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MTLRxQMPOCR")
            .field("ovfpktcnt", &self.ovfpktcnt())
            .field("ovfcntovf", &self.ovfcntovf())
            .field("mispktcnt", &self.mispktcnt())
            .field("miscntovf", &self.miscntovf())
            .finish()
    }
}
/**Rx queue missed packet and overflow counter register

You can [`read`](crate::Reg::read) this register and get [`mtlrx_qmpocr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H753V.html#Ethernet_MTL:MTLRxQMPOCR)*/
pub struct MTLRX_QMPOCRrs;
impl crate::RegisterSpec for MTLRX_QMPOCRrs {
    type Ux = u32;
}
///`read()` method returns [`mtlrx_qmpocr::R`](R) reader structure
impl crate::Readable for MTLRX_QMPOCRrs {}
///`reset()` method sets MTLRxQMPOCR to value 0
impl crate::Resettable for MTLRX_QMPOCRrs {}
