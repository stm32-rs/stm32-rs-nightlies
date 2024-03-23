#[doc = "Register `ETH_MACPPSWR` reader"]
pub type R = crate::R<ETH_MACPPSWRrs>;
#[doc = "Register `ETH_MACPPSWR` writer"]
pub type W = crate::W<ETH_MACPPSWRrs>;
#[doc = "Field `PPSWIDTH0` reader - PPSWIDTH0"]
pub type PPSWIDTH0_R = crate::FieldReader<u32>;
#[doc = "Field `PPSWIDTH0` writer - PPSWIDTH0"]
pub type PPSWIDTH0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - PPSWIDTH0"]
    #[inline(always)]
    pub fn ppswidth0(&self) -> PPSWIDTH0_R {
        PPSWIDTH0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - PPSWIDTH0"]
    #[inline(always)]
    #[must_use]
    pub fn ppswidth0(&mut self) -> PPSWIDTH0_W<ETH_MACPPSWRrs> {
        PPSWIDTH0_W::new(self, 0)
    }
}
#[doc = "The PPS Width register contains the number of units of sub-second increment value between the rising and corresponding falling edges of PPS signal output (ptp_pps_o).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macppswr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_macppswr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_MACPPSWRrs;
impl crate::RegisterSpec for ETH_MACPPSWRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_macppswr::R`](R) reader structure"]
impl crate::Readable for ETH_MACPPSWRrs {}
#[doc = "`write(|w| ..)` method takes [`eth_macppswr::W`](W) writer structure"]
impl crate::Writable for ETH_MACPPSWRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETH_MACPPSWR to value 0"]
impl crate::Resettable for ETH_MACPPSWRrs {
    const RESET_VALUE: u32 = 0;
}
