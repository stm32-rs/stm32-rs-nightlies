#[doc = "Register `MACLTCR` reader"]
pub type R = crate::R<MACLTCRrs>;
#[doc = "Register `MACLTCR` writer"]
pub type W = crate::W<MACLTCRrs>;
#[doc = "Field `TWT` reader - LPI TW Timer This field specifies the minimum time (in microseconds) for which the MAC waits after it stops transmitting the LPI pattern to the PHY and before it resumes the normal transmission. The TLPIEX status bit is set after the expiry of this timer."]
pub type TWT_R = crate::FieldReader<u16>;
#[doc = "Field `TWT` writer - LPI TW Timer This field specifies the minimum time (in microseconds) for which the MAC waits after it stops transmitting the LPI pattern to the PHY and before it resumes the normal transmission. The TLPIEX status bit is set after the expiry of this timer."]
pub type TWT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `LST` reader - LPI LS Timer This field specifies the minimum time (in milliseconds) for which the link status from the PHY should be up (OKAY) before the LPI pattern can be transmitted to the PHY. The MAC does not transmit the LPI pattern even when the LPIEN bit is set unless the LPI LS Timer reaches the programmed terminal count. The default value of the LPI LS Timer is 1000 (1 sec) as defined in the IEEE standard."]
pub type LST_R = crate::FieldReader<u16>;
#[doc = "Field `LST` writer - LPI LS Timer This field specifies the minimum time (in milliseconds) for which the link status from the PHY should be up (OKAY) before the LPI pattern can be transmitted to the PHY. The MAC does not transmit the LPI pattern even when the LPIEN bit is set unless the LPI LS Timer reaches the programmed terminal count. The default value of the LPI LS Timer is 1000 (1 sec) as defined in the IEEE standard."]
pub type LST_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:15 - LPI TW Timer This field specifies the minimum time (in microseconds) for which the MAC waits after it stops transmitting the LPI pattern to the PHY and before it resumes the normal transmission. The TLPIEX status bit is set after the expiry of this timer."]
    #[inline(always)]
    pub fn twt(&self) -> TWT_R {
        TWT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:25 - LPI LS Timer This field specifies the minimum time (in milliseconds) for which the link status from the PHY should be up (OKAY) before the LPI pattern can be transmitted to the PHY. The MAC does not transmit the LPI pattern even when the LPIEN bit is set unless the LPI LS Timer reaches the programmed terminal count. The default value of the LPI LS Timer is 1000 (1 sec) as defined in the IEEE standard."]
    #[inline(always)]
    pub fn lst(&self) -> LST_R {
        LST_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - LPI TW Timer This field specifies the minimum time (in microseconds) for which the MAC waits after it stops transmitting the LPI pattern to the PHY and before it resumes the normal transmission. The TLPIEX status bit is set after the expiry of this timer."]
    #[inline(always)]
    #[must_use]
    pub fn twt(&mut self) -> TWT_W<MACLTCRrs> {
        TWT_W::new(self, 0)
    }
    #[doc = "Bits 16:25 - LPI LS Timer This field specifies the minimum time (in milliseconds) for which the link status from the PHY should be up (OKAY) before the LPI pattern can be transmitted to the PHY. The MAC does not transmit the LPI pattern even when the LPIEN bit is set unless the LPI LS Timer reaches the programmed terminal count. The default value of the LPI LS Timer is 1000 (1 sec) as defined in the IEEE standard."]
    #[inline(always)]
    #[must_use]
    pub fn lst(&mut self) -> LST_W<MACLTCRrs> {
        LST_W::new(self, 16)
    }
}
#[doc = "LPI timers control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macltcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macltcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACLTCRrs;
impl crate::RegisterSpec for MACLTCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macltcr::R`](R) reader structure"]
impl crate::Readable for MACLTCRrs {}
#[doc = "`write(|w| ..)` method takes [`macltcr::W`](W) writer structure"]
impl crate::Writable for MACLTCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACLTCR to value 0x03e8_0000"]
impl crate::Resettable for MACLTCRrs {
    const RESET_VALUE: u32 = 0x03e8_0000;
}
