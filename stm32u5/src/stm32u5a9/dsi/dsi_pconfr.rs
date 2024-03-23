#[doc = "Register `DSI_PCONFR` reader"]
pub type R = crate::R<DSI_PCONFRrs>;
#[doc = "Register `DSI_PCONFR` writer"]
pub type W = crate::W<DSI_PCONFRrs>;
#[doc = "Field `NL` reader - Number of lanes This field configures the number of active data lanes: Others: Reserved"]
pub type NL_R = crate::FieldReader;
#[doc = "Field `NL` writer - Number of lanes This field configures the number of active data lanes: Others: Reserved"]
pub type NL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SW_TIME` reader - Stop wait time This field configures the minimum wait period to request a high-speed transmission after the Stop state."]
pub type SW_TIME_R = crate::FieldReader;
#[doc = "Field `SW_TIME` writer - Stop wait time This field configures the minimum wait period to request a high-speed transmission after the Stop state."]
pub type SW_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:1 - Number of lanes This field configures the number of active data lanes: Others: Reserved"]
    #[inline(always)]
    pub fn nl(&self) -> NL_R {
        NL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:15 - Stop wait time This field configures the minimum wait period to request a high-speed transmission after the Stop state."]
    #[inline(always)]
    pub fn sw_time(&self) -> SW_TIME_R {
        SW_TIME_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Number of lanes This field configures the number of active data lanes: Others: Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nl(&mut self) -> NL_W<DSI_PCONFRrs> {
        NL_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Stop wait time This field configures the minimum wait period to request a high-speed transmission after the Stop state."]
    #[inline(always)]
    #[must_use]
    pub fn sw_time(&mut self) -> SW_TIME_W<DSI_PCONFRrs> {
        SW_TIME_W::new(self, 8)
    }
}
#[doc = "DSI Host PHY configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_pconfr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_pconfr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_PCONFRrs;
impl crate::RegisterSpec for DSI_PCONFRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_pconfr::R`](R) reader structure"]
impl crate::Readable for DSI_PCONFRrs {}
#[doc = "`write(|w| ..)` method takes [`dsi_pconfr::W`](W) writer structure"]
impl crate::Writable for DSI_PCONFRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSI_PCONFR to value 0x01"]
impl crate::Resettable for DSI_PCONFRrs {
    const RESET_VALUE: u32 = 0x01;
}
