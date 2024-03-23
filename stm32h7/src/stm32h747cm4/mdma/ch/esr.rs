#[doc = "Register `ESR` reader"]
pub type R = crate::R<ESRrs>;
#[doc = "Field `TEA` reader - Transfer Error Address These bits are set and cleared by HW, in case of an MDMA data transfer error. It is used in conjunction with TED. This field indicates the 7 LSBits of the address which generated a transfer/access error. It may be used by SW to retrieve the failing address, by adding this value (truncated to the buffer transfer length size) to the current SAR/DAR value. Note: The SAR/DAR current value doesnt reflect this last address due to the FIFO management system. The SAR/DAR are only updated at the end of a (buffer) transfer (of TLEN+1 bytes). Note: It is not set in case of a link data error."]
pub type TEA_R = crate::FieldReader;
#[doc = "Field `TED` reader - Transfer Error Direction These bit is set and cleared by HW, in case of an MDMA data transfer error."]
pub type TED_R = crate::BitReader;
#[doc = "Field `TELD` reader - Transfer Error Link Data These bit is set by HW, in case of a transfer error while reading the block link data structure. It is cleared by software writing 1 to the CTEIFx bit in the DMA_IFCRy register."]
pub type TELD_R = crate::BitReader;
#[doc = "Field `TEMD` reader - Transfer Error Mask Data These bit is set by HW, in case of a transfer error while writing the Mask Data. It is cleared by software writing 1 to the CTEIFx bit in the DMA_IFCRy register."]
pub type TEMD_R = crate::BitReader;
#[doc = "Field `ASE` reader - Address/Size Error These bit is set by HW, when the programmed address is not aligned with the data size. TED will indicate whether the problem is on the source or destination. It is cleared by software writing 1 to the CTEIFx bit in the DMA_IFCRy register."]
pub type ASE_R = crate::BitReader;
#[doc = "Field `BSE` reader - Block Size Error These bit is set by HW, when the block size is not an integer multiple of the data size either for source or destination. TED will indicate whether the problem is on the source or destination. It is cleared by software writing 1 to the CTEIFx bit in the DMA_IFCRy register."]
pub type BSE_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:6 - Transfer Error Address These bits are set and cleared by HW, in case of an MDMA data transfer error. It is used in conjunction with TED. This field indicates the 7 LSBits of the address which generated a transfer/access error. It may be used by SW to retrieve the failing address, by adding this value (truncated to the buffer transfer length size) to the current SAR/DAR value. Note: The SAR/DAR current value doesnt reflect this last address due to the FIFO management system. The SAR/DAR are only updated at the end of a (buffer) transfer (of TLEN+1 bytes). Note: It is not set in case of a link data error."]
    #[inline(always)]
    pub fn tea(&self) -> TEA_R {
        TEA_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Transfer Error Direction These bit is set and cleared by HW, in case of an MDMA data transfer error."]
    #[inline(always)]
    pub fn ted(&self) -> TED_R {
        TED_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Transfer Error Link Data These bit is set by HW, in case of a transfer error while reading the block link data structure. It is cleared by software writing 1 to the CTEIFx bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn teld(&self) -> TELD_R {
        TELD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transfer Error Mask Data These bit is set by HW, in case of a transfer error while writing the Mask Data. It is cleared by software writing 1 to the CTEIFx bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn temd(&self) -> TEMD_R {
        TEMD_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Address/Size Error These bit is set by HW, when the programmed address is not aligned with the data size. TED will indicate whether the problem is on the source or destination. It is cleared by software writing 1 to the CTEIFx bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn ase(&self) -> ASE_R {
        ASE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Block Size Error These bit is set by HW, when the block size is not an integer multiple of the data size either for source or destination. TED will indicate whether the problem is on the source or destination. It is cleared by software writing 1 to the CTEIFx bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn bse(&self) -> BSE_R {
        BSE_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "MDMA Channel x error status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`esr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ESRrs;
impl crate::RegisterSpec for ESRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`esr::R`](R) reader structure"]
impl crate::Readable for ESRrs {}
#[doc = "`reset()` method sets ESR to value 0"]
impl crate::Resettable for ESRrs {
    const RESET_VALUE: u32 = 0;
}
