///Register `DMAC0MFCR` reader
pub type R = crate::R<DMAC0MFCRrs>;
///Register `DMAC0MFCR` writer
pub type W = crate::W<DMAC0MFCRrs>;
/**Field `MFC` reader - Dropped Packet Counters

<div class="warning">The field is <b>cleared</b> (set to zero) following a read operation.</div>*/
pub type MFC_R = crate::FieldReader<u16>;
///Field `MFC` writer - Dropped Packet Counters
pub type MFC_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
/**Field `MFCO` reader - Overflow status of the MFC Counter

<div class="warning">The field is <b>cleared</b> (set to zero) following a read operation.</div>*/
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
        f.debug_struct("DMAC0MFCR").finish()
    }
}
impl W {
    ///Bits 0:10 - Dropped Packet Counters
    #[inline(always)]
    pub fn mfc(&mut self) -> MFC_W<'_, DMAC0MFCRrs> {
        MFC_W::new(self, 0)
    }
    ///Bit 15 - Overflow status of the MFC Counter
    #[inline(always)]
    pub fn mfco(&mut self) -> MFCO_W<'_, DMAC0MFCRrs> {
        MFCO_W::new(self, 15)
    }
}
/**Channel 0 missed frame count register

You can [`read`](crate::Reg::read) this register and get [`dmac0mfcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac0mfcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#ETH:DMAC0MFCR)*/
pub struct DMAC0MFCRrs;
impl crate::RegisterSpec for DMAC0MFCRrs {
    type Ux = u32;
}
///`read()` method returns [`dmac0mfcr::R`](R) reader structure
impl crate::Readable for DMAC0MFCRrs {}
///`write(|w| ..)` method takes [`dmac0mfcr::W`](W) writer structure
impl crate::Writable for DMAC0MFCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMAC0MFCR to value 0
impl crate::Resettable for DMAC0MFCRrs {}
