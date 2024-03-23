#[doc = "Register `ETH_MACPPSTTSR` reader"]
pub type R = crate::R<ETH_MACPPSTTSRrs>;
#[doc = "Register `ETH_MACPPSTTSR` writer"]
pub type W = crate::W<ETH_MACPPSTTSRrs>;
#[doc = "Field `TSTRH0` reader - TSTRH0"]
pub type TSTRH0_R = crate::FieldReader<u32>;
#[doc = "Field `TSTRH0` writer - TSTRH0"]
pub type TSTRH0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - TSTRH0"]
    #[inline(always)]
    pub fn tstrh0(&self) -> TSTRH0_R {
        TSTRH0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - TSTRH0"]
    #[inline(always)]
    #[must_use]
    pub fn tstrh0(&mut self) -> TSTRH0_W<ETH_MACPPSTTSRrs> {
        TSTRH0_W::new(self, 0)
    }
}
#[doc = "The PPS Target Time Seconds register, along with PPS Target Time Nanoseconds register, is used to schedule an interrupt event \\[Bit 1 of ETH_MACTSSR\\]
when the system time exceeds the value programmed in these registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macppsttsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_macppsttsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_MACPPSTTSRrs;
impl crate::RegisterSpec for ETH_MACPPSTTSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_macppsttsr::R`](R) reader structure"]
impl crate::Readable for ETH_MACPPSTTSRrs {}
#[doc = "`write(|w| ..)` method takes [`eth_macppsttsr::W`](W) writer structure"]
impl crate::Writable for ETH_MACPPSTTSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETH_MACPPSTTSR to value 0"]
impl crate::Resettable for ETH_MACPPSTTSRrs {
    const RESET_VALUE: u32 = 0;
}
