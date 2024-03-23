#[doc = "Register `ETH_MAC1USTCR` reader"]
pub type R = crate::R<ETH_MAC1USTCRrs>;
#[doc = "Register `ETH_MAC1USTCR` writer"]
pub type W = crate::W<ETH_MAC1USTCRrs>;
#[doc = "Field `TIC_1US_CNTR` reader - TIC_1US_CNTR"]
pub type TIC_1US_CNTR_R = crate::FieldReader<u16>;
#[doc = "Field `TIC_1US_CNTR` writer - TIC_1US_CNTR"]
pub type TIC_1US_CNTR_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - TIC_1US_CNTR"]
    #[inline(always)]
    pub fn tic_1us_cntr(&self) -> TIC_1US_CNTR_R {
        TIC_1US_CNTR_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - TIC_1US_CNTR"]
    #[inline(always)]
    #[must_use]
    pub fn tic_1us_cntr(&mut self) -> TIC_1US_CNTR_W<ETH_MAC1USTCRrs> {
        TIC_1US_CNTR_W::new(self, 0)
    }
}
#[doc = "This register controls the generation of the Reference time (1-microsecond tick) for all the LPI timers. This timer has to be programmed by the software initially.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_mac1ustcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_mac1ustcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_MAC1USTCRrs;
impl crate::RegisterSpec for ETH_MAC1USTCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_mac1ustcr::R`](R) reader structure"]
impl crate::Readable for ETH_MAC1USTCRrs {}
#[doc = "`write(|w| ..)` method takes [`eth_mac1ustcr::W`](W) writer structure"]
impl crate::Writable for ETH_MAC1USTCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETH_MAC1USTCR to value 0"]
impl crate::Resettable for ETH_MAC1USTCRrs {
    const RESET_VALUE: u32 = 0;
}
