///Register `MACTSIACR` reader
pub type R = crate::R<MACTSIACRrs>;
///Register `MACTSIACR` writer
pub type W = crate::W<MACTSIACRrs>;
///Field `OSTIAC` reader - OSTIAC
pub type OSTIAC_R = crate::FieldReader<u32>;
///Field `OSTIAC` writer - OSTIAC
pub type OSTIAC_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - OSTIAC
    #[inline(always)]
    pub fn ostiac(&self) -> OSTIAC_R {
        OSTIAC_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACTSIACR")
            .field("ostiac", &self.ostiac())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - OSTIAC
    #[inline(always)]
    pub fn ostiac(&mut self) -> OSTIAC_W<'_, MACTSIACRrs> {
        OSTIAC_W::new(self, 0)
    }
}
/**The MAC Timestamp Ingress Asymmetry Correction register contains the Ingress Asymmetry Correction value to be used while updating correction field in PDelay_Resp PTP messages.

You can [`read`](crate::Reg::read) this register and get [`mactsiacr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mactsiacr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACTSIACR)*/
pub struct MACTSIACRrs;
impl crate::RegisterSpec for MACTSIACRrs {
    type Ux = u32;
}
///`read()` method returns [`mactsiacr::R`](R) reader structure
impl crate::Readable for MACTSIACRrs {}
///`write(|w| ..)` method takes [`mactsiacr::W`](W) writer structure
impl crate::Writable for MACTSIACRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACTSIACR to value 0
impl crate::Resettable for MACTSIACRrs {}
