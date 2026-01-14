///Register `DMAC1MFCR` reader
pub type R = crate::R<DMAC1MFCRrs>;
///Register `DMAC1MFCR` writer
pub type W = crate::W<DMAC1MFCRrs>;
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
        f.debug_struct("DMAC1MFCR").finish()
    }
}
impl W {
    ///Bits 0:10 - Dropped Packet Counters
    #[inline(always)]
    pub fn mfc(&mut self) -> MFC_W<'_, DMAC1MFCRrs> {
        MFC_W::new(self, 0)
    }
    ///Bit 15 - Overflow status of the MFC Counter
    #[inline(always)]
    pub fn mfco(&mut self) -> MFCO_W<'_, DMAC1MFCRrs> {
        MFCO_W::new(self, 15)
    }
}
/**Channel 1 missed frame count register

You can [`read`](crate::Reg::read) this register and get [`dmac1mfcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac1mfcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#ETH:DMAC1MFCR)*/
pub struct DMAC1MFCRrs;
impl crate::RegisterSpec for DMAC1MFCRrs {
    type Ux = u32;
}
///`read()` method returns [`dmac1mfcr::R`](R) reader structure
impl crate::Readable for DMAC1MFCRrs {}
///`write(|w| ..)` method takes [`dmac1mfcr::W`](W) writer structure
impl crate::Writable for DMAC1MFCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMAC1MFCR to value 0
impl crate::Resettable for DMAC1MFCRrs {}
