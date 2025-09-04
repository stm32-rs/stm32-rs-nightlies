///Register `MACPPSIR` reader
pub type R = crate::R<MACPPSIRrs>;
///Register `MACPPSIR` writer
pub type W = crate::W<MACPPSIRrs>;
///Field `PPSINT0` reader - PPSINT0
pub type PPSINT0_R = crate::FieldReader<u32>;
///Field `PPSINT0` writer - PPSINT0
pub type PPSINT0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - PPSINT0
    #[inline(always)]
    pub fn ppsint0(&self) -> PPSINT0_R {
        PPSINT0_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACPPSIR")
            .field("ppsint0", &self.ppsint0())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - PPSINT0
    #[inline(always)]
    pub fn ppsint0(&mut self) -> PPSINT0_W<MACPPSIRrs> {
        PPSINT0_W::new(self, 0)
    }
}
/**The PPS Interval register contains the number of units of sub-second increment value between the rising edges of PPS signal output (ptp_pps_o\[0\]).

You can [`read`](crate::Reg::read) this register and get [`macppsir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macppsir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACPPSIR)*/
pub struct MACPPSIRrs;
impl crate::RegisterSpec for MACPPSIRrs {
    type Ux = u32;
}
///`read()` method returns [`macppsir::R`](R) reader structure
impl crate::Readable for MACPPSIRrs {}
///`write(|w| ..)` method takes [`macppsir::W`](W) writer structure
impl crate::Writable for MACPPSIRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACPPSIR to value 0
impl crate::Resettable for MACPPSIRrs {}
