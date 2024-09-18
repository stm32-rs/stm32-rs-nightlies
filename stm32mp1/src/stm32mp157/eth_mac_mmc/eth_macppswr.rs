///Register `ETH_MACPPSWR` reader
pub type R = crate::R<ETH_MACPPSWRrs>;
///Register `ETH_MACPPSWR` writer
pub type W = crate::W<ETH_MACPPSWRrs>;
///Field `PPSWIDTH0` reader - PPSWIDTH0
pub type PPSWIDTH0_R = crate::FieldReader<u32>;
///Field `PPSWIDTH0` writer - PPSWIDTH0
pub type PPSWIDTH0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - PPSWIDTH0
    #[inline(always)]
    pub fn ppswidth0(&self) -> PPSWIDTH0_R {
        PPSWIDTH0_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ETH_MACPPSWR")
            .field("ppswidth0", &self.ppswidth0())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - PPSWIDTH0
    #[inline(always)]
    #[must_use]
    pub fn ppswidth0(&mut self) -> PPSWIDTH0_W<ETH_MACPPSWRrs> {
        PPSWIDTH0_W::new(self, 0)
    }
}
/**The PPS Width register contains the number of units of sub-second increment value between the rising and corresponding falling edges of PPS signal output (ptp_pps_o).

You can [`read`](crate::Reg::read) this register and get [`eth_macppswr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eth_macppswr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETH_MAC_MMC:ETH_MACPPSWR)*/
pub struct ETH_MACPPSWRrs;
impl crate::RegisterSpec for ETH_MACPPSWRrs {
    type Ux = u32;
}
///`read()` method returns [`eth_macppswr::R`](R) reader structure
impl crate::Readable for ETH_MACPPSWRrs {}
///`write(|w| ..)` method takes [`eth_macppswr::W`](W) writer structure
impl crate::Writable for ETH_MACPPSWRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ETH_MACPPSWR to value 0
impl crate::Resettable for ETH_MACPPSWRrs {
    const RESET_VALUE: u32 = 0;
}
