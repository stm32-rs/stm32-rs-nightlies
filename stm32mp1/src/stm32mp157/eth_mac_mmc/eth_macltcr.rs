#[doc = "Register `ETH_MACLTCR` reader"]
pub type R = crate::R<ETH_MACLTCRrs>;
#[doc = "Register `ETH_MACLTCR` writer"]
pub type W = crate::W<ETH_MACLTCRrs>;
#[doc = "Field `TWT` reader - TWT"]
pub type TWT_R = crate::FieldReader<u16>;
#[doc = "Field `TWT` writer - TWT"]
pub type TWT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `LST` reader - LST"]
pub type LST_R = crate::FieldReader<u16>;
#[doc = "Field `LST` writer - LST"]
pub type LST_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:15 - TWT"]
    #[inline(always)]
    pub fn twt(&self) -> TWT_R {
        TWT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:25 - LST"]
    #[inline(always)]
    pub fn lst(&self) -> LST_R {
        LST_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - TWT"]
    #[inline(always)]
    #[must_use]
    pub fn twt(&mut self) -> TWT_W<ETH_MACLTCRrs> {
        TWT_W::new(self, 0)
    }
    #[doc = "Bits 16:25 - LST"]
    #[inline(always)]
    #[must_use]
    pub fn lst(&mut self) -> LST_W<ETH_MACLTCRrs> {
        LST_W::new(self, 16)
    }
}
#[doc = "The LPI Timers Control register controls the timeout values in the LPI states. It specifies the time for which the MAC transmits the LPI pattern and also the time for which the MAC waits before resuming the normal transmission.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macltcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_macltcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_MACLTCRrs;
impl crate::RegisterSpec for ETH_MACLTCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_macltcr::R`](R) reader structure"]
impl crate::Readable for ETH_MACLTCRrs {}
#[doc = "`write(|w| ..)` method takes [`eth_macltcr::W`](W) writer structure"]
impl crate::Writable for ETH_MACLTCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETH_MACLTCR to value 0x03e8_0000"]
impl crate::Resettable for ETH_MACLTCRrs {
    const RESET_VALUE: u32 = 0x03e8_0000;
}
