#[doc = "Register `DSI_BCFGR` reader"]
pub type R = crate::R<DSI_BCFGRrs>;
#[doc = "Register `DSI_BCFGR` writer"]
pub type W = crate::W<DSI_BCFGRrs>;
#[doc = "Field `PWRUP` reader - Power-up This bit powers-up the reference bias for the MIPI D-PHY"]
pub type PWRUP_R = crate::BitReader;
#[doc = "Field `PWRUP` writer - Power-up This bit powers-up the reference bias for the MIPI D-PHY"]
pub type PWRUP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 6 - Power-up This bit powers-up the reference bias for the MIPI D-PHY"]
    #[inline(always)]
    pub fn pwrup(&self) -> PWRUP_R {
        PWRUP_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - Power-up This bit powers-up the reference bias for the MIPI D-PHY"]
    #[inline(always)]
    #[must_use]
    pub fn pwrup(&mut self) -> PWRUP_W<DSI_BCFGRrs> {
        PWRUP_W::new(self, 6)
    }
}
#[doc = "DSI bias configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_bcfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_bcfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_BCFGRrs;
impl crate::RegisterSpec for DSI_BCFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_bcfgr::R`](R) reader structure"]
impl crate::Readable for DSI_BCFGRrs {}
#[doc = "`write(|w| ..)` method takes [`dsi_bcfgr::W`](W) writer structure"]
impl crate::Writable for DSI_BCFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSI_BCFGR to value 0"]
impl crate::Resettable for DSI_BCFGRrs {
    const RESET_VALUE: u32 = 0;
}
