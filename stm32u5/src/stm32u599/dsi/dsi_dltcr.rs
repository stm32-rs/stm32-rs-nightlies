#[doc = "Register `DSI_DLTCR` reader"]
pub type R = crate::R<DSI_DLTCRrs>;
#[doc = "Register `DSI_DLTCR` writer"]
pub type W = crate::W<DSI_DLTCRrs>;
#[doc = "Field `LP2HS_TIME` reader - Low-power to high-speed time This field configures the maximum time that the D-PHY data lanes take to go from low-power to high-speed transmission measured in lane byte clock cycles."]
pub type LP2HS_TIME_R = crate::FieldReader<u16>;
#[doc = "Field `LP2HS_TIME` writer - Low-power to high-speed time This field configures the maximum time that the D-PHY data lanes take to go from low-power to high-speed transmission measured in lane byte clock cycles."]
pub type LP2HS_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `HS2LP_TIME` reader - High-speed to low-power time This field configures the maximum time that the D-PHY data lanes take to go from high-speed to low-power transmission measured in lane byte clock cycles."]
pub type HS2LP_TIME_R = crate::FieldReader<u16>;
#[doc = "Field `HS2LP_TIME` writer - High-speed to low-power time This field configures the maximum time that the D-PHY data lanes take to go from high-speed to low-power transmission measured in lane byte clock cycles."]
pub type HS2LP_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Low-power to high-speed time This field configures the maximum time that the D-PHY data lanes take to go from low-power to high-speed transmission measured in lane byte clock cycles."]
    #[inline(always)]
    pub fn lp2hs_time(&self) -> LP2HS_TIME_R {
        LP2HS_TIME_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - High-speed to low-power time This field configures the maximum time that the D-PHY data lanes take to go from high-speed to low-power transmission measured in lane byte clock cycles."]
    #[inline(always)]
    pub fn hs2lp_time(&self) -> HS2LP_TIME_R {
        HS2LP_TIME_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Low-power to high-speed time This field configures the maximum time that the D-PHY data lanes take to go from low-power to high-speed transmission measured in lane byte clock cycles."]
    #[inline(always)]
    #[must_use]
    pub fn lp2hs_time(&mut self) -> LP2HS_TIME_W<DSI_DLTCRrs> {
        LP2HS_TIME_W::new(self, 0)
    }
    #[doc = "Bits 16:25 - High-speed to low-power time This field configures the maximum time that the D-PHY data lanes take to go from high-speed to low-power transmission measured in lane byte clock cycles."]
    #[inline(always)]
    #[must_use]
    pub fn hs2lp_time(&mut self) -> HS2LP_TIME_W<DSI_DLTCRrs> {
        HS2LP_TIME_W::new(self, 16)
    }
}
#[doc = "DSI Host data lane timer configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_dltcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_dltcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_DLTCRrs;
impl crate::RegisterSpec for DSI_DLTCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_dltcr::R`](R) reader structure"]
impl crate::Readable for DSI_DLTCRrs {}
#[doc = "`write(|w| ..)` method takes [`dsi_dltcr::W`](W) writer structure"]
impl crate::Writable for DSI_DLTCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSI_DLTCR to value 0"]
impl crate::Resettable for DSI_DLTCRrs {
    const RESET_VALUE: u32 = 0;
}
