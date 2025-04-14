///Register `DMAC1MFCR` reader
pub type R = crate::R<DMAC1MFCRrs>;
///Field `MFC` reader - Dropped Packet Counters
pub type MFC_R = crate::FieldReader<u16>;
///Field `MFCO` reader - Overflow status of the MFC Counter
pub type MFCO_R = crate::BitReader;
impl R {
    ///Bits 0:10 - Dropped Packet Counters
    #[inline(always)]
    pub fn mfc(&self) -> MFC_R {
        MFC_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bit 15 - Overflow status of the MFC Counter
    #[inline(always)]
    pub fn mfco(&self) -> MFCO_R {
        MFCO_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAC1MFCR")
            .field("mfc", &self.mfc())
            .field("mfco", &self.mfco())
            .finish()
    }
}
/**Channel missed frame count register

You can [`read`](crate::Reg::read) this register and get [`dmac1mfcr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:DMAC1MFCR)*/
pub struct DMAC1MFCRrs;
impl crate::RegisterSpec for DMAC1MFCRrs {
    type Ux = u32;
}
///`read()` method returns [`dmac1mfcr::R`](R) reader structure
impl crate::Readable for DMAC1MFCRrs {}
///`reset()` method sets DMAC1MFCR to value 0
impl crate::Resettable for DMAC1MFCRrs {}
