#[doc = "Register `DSI_VSCR` reader"]
pub type R = crate::R<DSI_VSCRrs>;
#[doc = "Register `DSI_VSCR` writer"]
pub type W = crate::W<DSI_VSCRrs>;
#[doc = "Field `EN` reader - Enable When set to 1, DSI Host LTDC interface receives the active configuration from the auxiliary registers. When this bit is set along with the UR bit, the auxiliary registers are automatically updated."]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - Enable When set to 1, DSI Host LTDC interface receives the active configuration from the auxiliary registers. When this bit is set along with the UR bit, the auxiliary registers are automatically updated."]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UR` reader - Update register When set to 1, the LTDC registers are copied to the auxiliary registers. After copying, this bit is auto cleared."]
pub type UR_R = crate::BitReader;
#[doc = "Field `UR` writer - Update register When set to 1, the LTDC registers are copied to the auxiliary registers. After copying, this bit is auto cleared."]
pub type UR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable When set to 1, DSI Host LTDC interface receives the active configuration from the auxiliary registers. When this bit is set along with the UR bit, the auxiliary registers are automatically updated."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Update register When set to 1, the LTDC registers are copied to the auxiliary registers. After copying, this bit is auto cleared."]
    #[inline(always)]
    pub fn ur(&self) -> UR_R {
        UR_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable When set to 1, DSI Host LTDC interface receives the active configuration from the auxiliary registers. When this bit is set along with the UR bit, the auxiliary registers are automatically updated."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<DSI_VSCRrs> {
        EN_W::new(self, 0)
    }
    #[doc = "Bit 8 - Update register When set to 1, the LTDC registers are copied to the auxiliary registers. After copying, this bit is auto cleared."]
    #[inline(always)]
    #[must_use]
    pub fn ur(&mut self) -> UR_W<DSI_VSCRrs> {
        UR_W::new(self, 8)
    }
}
#[doc = "DSI Host video shadow control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vscr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_vscr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_VSCRrs;
impl crate::RegisterSpec for DSI_VSCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_vscr::R`](R) reader structure"]
impl crate::Readable for DSI_VSCRrs {}
#[doc = "`write(|w| ..)` method takes [`dsi_vscr::W`](W) writer structure"]
impl crate::Writable for DSI_VSCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSI_VSCR to value 0"]
impl crate::Resettable for DSI_VSCRrs {
    const RESET_VALUE: u32 = 0;
}
