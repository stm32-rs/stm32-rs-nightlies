///Register `ETH_MACTSEACR` reader
pub type R = crate::R<ETH_MACTSEACRrs>;
///Register `ETH_MACTSEACR` writer
pub type W = crate::W<ETH_MACTSEACRrs>;
///Field `OSTEAC` reader - OSTEAC
pub type OSTEAC_R = crate::FieldReader<u32>;
///Field `OSTEAC` writer - OSTEAC
pub type OSTEAC_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - OSTEAC
    #[inline(always)]
    pub fn osteac(&self) -> OSTEAC_R {
        OSTEAC_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ETH_MACTSEACR")
            .field("osteac", &self.osteac())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - OSTEAC
    #[inline(always)]
    #[must_use]
    pub fn osteac(&mut self) -> OSTEAC_W<ETH_MACTSEACRrs> {
        OSTEAC_W::new(self, 0)
    }
}
/**The MAC Timestamp Egress Asymmetry Correction register contains the Egress Asymmetry Correction value to be used while updating the correction field in PDelay_Req PTP messages.

You can [`read`](crate::Reg::read) this register and get [`eth_mactseacr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eth_mactseacr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:ETH_MACTSEACR)*/
pub struct ETH_MACTSEACRrs;
impl crate::RegisterSpec for ETH_MACTSEACRrs {
    type Ux = u32;
}
///`read()` method returns [`eth_mactseacr::R`](R) reader structure
impl crate::Readable for ETH_MACTSEACRrs {}
///`write(|w| ..)` method takes [`eth_mactseacr::W`](W) writer structure
impl crate::Writable for ETH_MACTSEACRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ETH_MACTSEACR to value 0
impl crate::Resettable for ETH_MACTSEACRrs {
    const RESET_VALUE: u32 = 0;
}
