///Register `NONSEC_AIDCR` reader
pub type R = crate::R<NONSEC_AIDCRrs>;
///Register `NONSEC_AIDCR` writer
pub type W = crate::W<NONSEC_AIDCRrs>;
///Field `DMACID_NONSEC` reader - Non-secure OS allocates specific CID to DMA channel through these bits
pub type DMACID_NONSEC_R = crate::FieldReader;
///Field `DMACID_NONSEC` writer - Non-secure OS allocates specific CID to DMA channel through these bits
pub type DMACID_NONSEC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - Non-secure OS allocates specific CID to DMA channel through these bits
    #[inline(always)]
    pub fn dmacid_nonsec(&self) -> DMACID_NONSEC_R {
        DMACID_NONSEC_R::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NONSEC_AIDCR")
            .field("dmacid_nonsec", &self.dmacid_nonsec())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Non-secure OS allocates specific CID to DMA channel through these bits
    #[inline(always)]
    pub fn dmacid_nonsec(&mut self) -> DMACID_NONSEC_W<'_, NONSEC_AIDCRrs> {
        DMACID_NONSEC_W::new(self, 0)
    }
}
/**SYSCFG DMA CID non-secure control register

You can [`read`](crate::Reg::read) this register and get [`nonsec_aidcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nonsec_aidcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#SYSCFG:NONSEC_AIDCR)*/
pub struct NONSEC_AIDCRrs;
impl crate::RegisterSpec for NONSEC_AIDCRrs {
    type Ux = u32;
}
///`read()` method returns [`nonsec_aidcr::R`](R) reader structure
impl crate::Readable for NONSEC_AIDCRrs {}
///`write(|w| ..)` method takes [`nonsec_aidcr::W`](W) writer structure
impl crate::Writable for NONSEC_AIDCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets NONSEC_AIDCR to value 0x01
impl crate::Resettable for NONSEC_AIDCRrs {
    const RESET_VALUE: u32 = 0x01;
}
