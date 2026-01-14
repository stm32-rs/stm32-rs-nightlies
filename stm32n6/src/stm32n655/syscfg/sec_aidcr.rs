///Register `SEC_AIDCR` reader
pub type R = crate::R<SEC_AIDCRrs>;
///Register `SEC_AIDCR` writer
pub type W = crate::W<SEC_AIDCRrs>;
///Field `DMACID_SEC` reader - Secure OS allocates specific CID to DMA channel through these bits.
pub type DMACID_SEC_R = crate::FieldReader;
///Field `DMACID_SEC` writer - Secure OS allocates specific CID to DMA channel through these bits.
pub type DMACID_SEC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - Secure OS allocates specific CID to DMA channel through these bits.
    #[inline(always)]
    pub fn dmacid_sec(&self) -> DMACID_SEC_R {
        DMACID_SEC_R::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC_AIDCR")
            .field("dmacid_sec", &self.dmacid_sec())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Secure OS allocates specific CID to DMA channel through these bits.
    #[inline(always)]
    pub fn dmacid_sec(&mut self) -> DMACID_SEC_W<'_, SEC_AIDCRrs> {
        DMACID_SEC_W::new(self, 0)
    }
}
/**SYSCFG DMA CID secure control register

You can [`read`](crate::Reg::read) this register and get [`sec_aidcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_aidcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#SYSCFG:SEC_AIDCR)*/
pub struct SEC_AIDCRrs;
impl crate::RegisterSpec for SEC_AIDCRrs {
    type Ux = u32;
}
///`read()` method returns [`sec_aidcr::R`](R) reader structure
impl crate::Readable for SEC_AIDCRrs {}
///`write(|w| ..)` method takes [`sec_aidcr::W`](W) writer structure
impl crate::Writable for SEC_AIDCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SEC_AIDCR to value 0x01
impl crate::Resettable for SEC_AIDCRrs {
    const RESET_VALUE: u32 = 0x01;
}
