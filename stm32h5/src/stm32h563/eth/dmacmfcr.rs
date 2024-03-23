#[doc = "Register `DMACMFCR` reader"]
pub type R = crate::R<DMACMFCRrs>;
#[doc = "Register `DMACMFCR` writer"]
pub type W = crate::W<DMACMFCRrs>;
#[doc = "Field `MFC` reader - Dropped Packet Counters This counter indicates the number of packet counters that are dropped by the DMA either because of bus error or because of programing RPF field in Channel receive control register (ETH_DMACRXCR). The counter gets cleared when this register is read.\n\nThe field is **cleared** (set to zero) following a read operation."]
pub type MFC_R = crate::FieldReader<u16>;
#[doc = "Field `MFC` writer - Dropped Packet Counters This counter indicates the number of packet counters that are dropped by the DMA either because of bus error or because of programing RPF field in Channel receive control register (ETH_DMACRXCR). The counter gets cleared when this register is read."]
pub type MFC_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `MFCO` reader - Overflow status of the MFC Counter When this bit is set then the MFC counter does not get incremented further. The bit gets cleared when this register is read.\n\nThe field is **cleared** (set to zero) following a read operation."]
pub type MFCO_R = crate::BitReader;
#[doc = "Field `MFCO` writer - Overflow status of the MFC Counter When this bit is set then the MFC counter does not get incremented further. The bit gets cleared when this register is read."]
pub type MFCO_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10 - Dropped Packet Counters This counter indicates the number of packet counters that are dropped by the DMA either because of bus error or because of programing RPF field in Channel receive control register (ETH_DMACRXCR). The counter gets cleared when this register is read."]
    #[inline(always)]
    pub fn mfc(&self) -> MFC_R {
        MFC_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 15 - Overflow status of the MFC Counter When this bit is set then the MFC counter does not get incremented further. The bit gets cleared when this register is read."]
    #[inline(always)]
    pub fn mfco(&self) -> MFCO_R {
        MFCO_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - Dropped Packet Counters This counter indicates the number of packet counters that are dropped by the DMA either because of bus error or because of programing RPF field in Channel receive control register (ETH_DMACRXCR). The counter gets cleared when this register is read."]
    #[inline(always)]
    #[must_use]
    pub fn mfc(&mut self) -> MFC_W<DMACMFCRrs> {
        MFC_W::new(self, 0)
    }
    #[doc = "Bit 15 - Overflow status of the MFC Counter When this bit is set then the MFC counter does not get incremented further. The bit gets cleared when this register is read."]
    #[inline(always)]
    #[must_use]
    pub fn mfco(&mut self) -> MFCO_W<DMACMFCRrs> {
        MFCO_W::new(self, 15)
    }
}
#[doc = "Channel missed frame count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacmfcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmacmfcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMACMFCRrs;
impl crate::RegisterSpec for DMACMFCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmacmfcr::R`](R) reader structure"]
impl crate::Readable for DMACMFCRrs {}
#[doc = "`write(|w| ..)` method takes [`dmacmfcr::W`](W) writer structure"]
impl crate::Writable for DMACMFCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMACMFCR to value 0"]
impl crate::Resettable for DMACMFCRrs {
    const RESET_VALUE: u32 = 0;
}
