#[doc = "Register `DMACMFCR` reader"]
pub type R = crate::R<DMACMFCRrs>;
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
#[doc = "Channel missed frame count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacmfcr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMACMFCRrs;
impl crate::RegisterSpec for DMACMFCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmacmfcr::R`](R) reader structure"]
impl crate::Readable for DMACMFCRrs {}
#[doc = "`reset()` method sets DMACMFCR to value 0"]
impl crate::Resettable for DMACMFCRrs {
    const RESET_VALUE: u32 = 0;
}
