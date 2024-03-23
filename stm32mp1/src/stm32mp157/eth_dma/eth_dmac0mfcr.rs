#[doc = "Register `ETH_DMAC0MFCR` reader"]
pub type R = crate::R<ETH_DMAC0MFCRrs>;
#[doc = "Field `MFC` reader - Dropped Packet Counters"]
pub type MFC_R = crate::FieldReader<u16>;
#[doc = "Field `MFCO` reader - Overflow status of the MFC Counter"]
pub type MFCO_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:10 - Dropped Packet Counters"]
    #[inline(always)]
    pub fn mfc(&self) -> MFC_R {
        MFC_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 15 - Overflow status of the MFC Counter"]
    #[inline(always)]
    pub fn mfco(&self) -> MFCO_R {
        MFCO_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Channel missed frame count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_dmac0mfcr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_DMAC0MFCRrs;
impl crate::RegisterSpec for ETH_DMAC0MFCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_dmac0mfcr::R`](R) reader structure"]
impl crate::Readable for ETH_DMAC0MFCRrs {}
#[doc = "`reset()` method sets ETH_DMAC0MFCR to value 0"]
impl crate::Resettable for ETH_DMAC0MFCRrs {
    const RESET_VALUE: u32 = 0;
}
