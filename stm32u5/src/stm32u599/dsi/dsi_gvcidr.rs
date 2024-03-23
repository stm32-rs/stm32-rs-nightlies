#[doc = "Register `DSI_GVCIDR` reader"]
pub type R = crate::R<DSI_GVCIDRrs>;
#[doc = "Register `DSI_GVCIDR` writer"]
pub type W = crate::W<DSI_GVCIDRrs>;
#[doc = "Field `VCIDRX` reader - Virtual channel ID for reception This field indicates the generic interface read-back virtual channel identification."]
pub type VCIDRX_R = crate::FieldReader;
#[doc = "Field `VCIDRX` writer - Virtual channel ID for reception This field indicates the generic interface read-back virtual channel identification."]
pub type VCIDRX_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VCIDTX` reader - Virtual channel ID for transmission This field indicates the generic interface virtual channel identification where the generic packet is automatically generated and transmitted."]
pub type VCIDTX_R = crate::FieldReader;
#[doc = "Field `VCIDTX` writer - Virtual channel ID for transmission This field indicates the generic interface virtual channel identification where the generic packet is automatically generated and transmitted."]
pub type VCIDTX_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Virtual channel ID for reception This field indicates the generic interface read-back virtual channel identification."]
    #[inline(always)]
    pub fn vcidrx(&self) -> VCIDRX_R {
        VCIDRX_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:17 - Virtual channel ID for transmission This field indicates the generic interface virtual channel identification where the generic packet is automatically generated and transmitted."]
    #[inline(always)]
    pub fn vcidtx(&self) -> VCIDTX_R {
        VCIDTX_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Virtual channel ID for reception This field indicates the generic interface read-back virtual channel identification."]
    #[inline(always)]
    #[must_use]
    pub fn vcidrx(&mut self) -> VCIDRX_W<DSI_GVCIDRrs> {
        VCIDRX_W::new(self, 0)
    }
    #[doc = "Bits 16:17 - Virtual channel ID for transmission This field indicates the generic interface virtual channel identification where the generic packet is automatically generated and transmitted."]
    #[inline(always)]
    #[must_use]
    pub fn vcidtx(&mut self) -> VCIDTX_W<DSI_GVCIDRrs> {
        VCIDTX_W::new(self, 16)
    }
}
#[doc = "DSI Host generic VCID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_gvcidr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_gvcidr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_GVCIDRrs;
impl crate::RegisterSpec for DSI_GVCIDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_gvcidr::R`](R) reader structure"]
impl crate::Readable for DSI_GVCIDRrs {}
#[doc = "`write(|w| ..)` method takes [`dsi_gvcidr::W`](W) writer structure"]
impl crate::Writable for DSI_GVCIDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSI_GVCIDR to value 0"]
impl crate::Resettable for DSI_GVCIDRrs {
    const RESET_VALUE: u32 = 0;
}
