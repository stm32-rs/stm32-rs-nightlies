#[doc = "Register `ETH_MACPPSIR` reader"]
pub type R = crate::R<ETH_MACPPSIRrs>;
#[doc = "Register `ETH_MACPPSIR` writer"]
pub type W = crate::W<ETH_MACPPSIRrs>;
#[doc = "Field `PPSINT0` reader - PPSINT0"]
pub type PPSINT0_R = crate::FieldReader<u32>;
#[doc = "Field `PPSINT0` writer - PPSINT0"]
pub type PPSINT0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - PPSINT0"]
    #[inline(always)]
    pub fn ppsint0(&self) -> PPSINT0_R {
        PPSINT0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - PPSINT0"]
    #[inline(always)]
    #[must_use]
    pub fn ppsint0(&mut self) -> PPSINT0_W<ETH_MACPPSIRrs> {
        PPSINT0_W::new(self, 0)
    }
}
#[doc = "The PPS Interval register contains the number of units of sub-second increment value between the rising edges of PPS signal output (ptp_pps_o\\[0\\]).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macppsir::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_macppsir::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_MACPPSIRrs;
impl crate::RegisterSpec for ETH_MACPPSIRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_macppsir::R`](R) reader structure"]
impl crate::Readable for ETH_MACPPSIRrs {}
#[doc = "`write(|w| ..)` method takes [`eth_macppsir::W`](W) writer structure"]
impl crate::Writable for ETH_MACPPSIRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETH_MACPPSIR to value 0"]
impl crate::Resettable for ETH_MACPPSIRrs {
    const RESET_VALUE: u32 = 0;
}
