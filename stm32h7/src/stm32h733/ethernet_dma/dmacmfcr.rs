///Register `DMACMFCR` reader
pub type R = crate::R<DMACMFCRrs>;
///Register `DMACMFCR` writer
pub type W = crate::W<DMACMFCRrs>;
///Field `MFC` reader - Dropped Packet Counters
pub type MFC_R = crate::FieldReader<u16>;
///Field `MFC` writer - Dropped Packet Counters
pub type MFC_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
///Field `MFCO` reader - Overflow status of the MFC Counter
pub type MFCO_R = crate::BitReader;
///Field `MFCO` writer - Overflow status of the MFC Counter
pub type MFCO_W<'a, REG> = crate::BitWriter<'a, REG>;
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
        f.debug_struct("DMACMFCR")
            .field("mfco", &self.mfco())
            .field("mfc", &self.mfc())
            .finish()
    }
}
impl W {
    ///Bits 0:10 - Dropped Packet Counters
    #[inline(always)]
    pub fn mfc(&mut self) -> MFC_W<'_, DMACMFCRrs> {
        MFC_W::new(self, 0)
    }
    ///Bit 15 - Overflow status of the MFC Counter
    #[inline(always)]
    pub fn mfco(&mut self) -> MFCO_W<'_, DMACMFCRrs> {
        MFCO_W::new(self, 15)
    }
}
/**Channel missed frame count register

You can [`read`](crate::Reg::read) this register and get [`dmacmfcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacmfcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#Ethernet_DMA:DMACMFCR)*/
pub struct DMACMFCRrs;
impl crate::RegisterSpec for DMACMFCRrs {
    type Ux = u32;
}
///`read()` method returns [`dmacmfcr::R`](R) reader structure
impl crate::Readable for DMACMFCRrs {}
///`write(|w| ..)` method takes [`dmacmfcr::W`](W) writer structure
impl crate::Writable for DMACMFCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMACMFCR to value 0
impl crate::Resettable for DMACMFCRrs {}
