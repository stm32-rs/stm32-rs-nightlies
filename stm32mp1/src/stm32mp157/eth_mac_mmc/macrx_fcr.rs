///Register `MACRxFCR` reader
pub type R = crate::R<MACRX_FCRrs>;
///Register `MACRxFCR` writer
pub type W = crate::W<MACRX_FCRrs>;
///Field `RFE` reader - RFE
pub type RFE_R = crate::BitReader;
///Field `RFE` writer - RFE
pub type RFE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UP` reader - UP
pub type UP_R = crate::BitReader;
///Field `UP` writer - UP
pub type UP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - RFE
    #[inline(always)]
    pub fn rfe(&self) -> RFE_R {
        RFE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - UP
    #[inline(always)]
    pub fn up(&self) -> UP_R {
        UP_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACRxFCR")
            .field("rfe", &self.rfe())
            .field("up", &self.up())
            .finish()
    }
}
impl W {
    ///Bit 0 - RFE
    #[inline(always)]
    pub fn rfe(&mut self) -> RFE_W<'_, MACRX_FCRrs> {
        RFE_W::new(self, 0)
    }
    ///Bit 1 - UP
    #[inline(always)]
    pub fn up(&mut self) -> UP_W<'_, MACRX_FCRrs> {
        UP_W::new(self, 1)
    }
}
/**The Receive Flow Control register controls the pausing of MAC Transmit based on the received Pause packet.

You can [`read`](crate::Reg::read) this register and get [`macrx_fcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macrx_fcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETH_MAC_MMC:MACRxFCR)*/
pub struct MACRX_FCRrs;
impl crate::RegisterSpec for MACRX_FCRrs {
    type Ux = u32;
}
///`read()` method returns [`macrx_fcr::R`](R) reader structure
impl crate::Readable for MACRX_FCRrs {}
///`write(|w| ..)` method takes [`macrx_fcr::W`](W) writer structure
impl crate::Writable for MACRX_FCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACRxFCR to value 0
impl crate::Resettable for MACRX_FCRrs {}
