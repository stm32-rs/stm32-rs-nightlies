#[doc = "Register `DSI_GHCR` reader"]
pub type R = crate::R<DSI_GHCRrs>;
#[doc = "Register `DSI_GHCR` writer"]
pub type W = crate::W<DSI_GHCRrs>;
#[doc = "Field `DT` reader - Type This field configures the packet data type of the header packet."]
pub type DT_R = crate::FieldReader;
#[doc = "Field `DT` writer - Type This field configures the packet data type of the header packet."]
pub type DT_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `VCID` reader - Channel This field configures the virtual channel ID of the header packet."]
pub type VCID_R = crate::FieldReader;
#[doc = "Field `VCID` writer - Channel This field configures the virtual channel ID of the header packet."]
pub type VCID_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WCLSB` reader - WordCount LSB This field configures the less significant byte of the header packet word count for long packets, or data 0 for short packets."]
pub type WCLSB_R = crate::FieldReader;
#[doc = "Field `WCLSB` writer - WordCount LSB This field configures the less significant byte of the header packet word count for long packets, or data 0 for short packets."]
pub type WCLSB_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WCMSB` reader - WordCount MSB This field configures the most significant byte of the header packet's word count for long packets, or data 1 for short packets."]
pub type WCMSB_R = crate::FieldReader;
#[doc = "Field `WCMSB` writer - WordCount MSB This field configures the most significant byte of the header packet's word count for long packets, or data 1 for short packets."]
pub type WCMSB_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:5 - Type This field configures the packet data type of the header packet."]
    #[inline(always)]
    pub fn dt(&self) -> DT_R {
        DT_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - Channel This field configures the virtual channel ID of the header packet."]
    #[inline(always)]
    pub fn vcid(&self) -> VCID_R {
        VCID_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:15 - WordCount LSB This field configures the less significant byte of the header packet word count for long packets, or data 0 for short packets."]
    #[inline(always)]
    pub fn wclsb(&self) -> WCLSB_R {
        WCLSB_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - WordCount MSB This field configures the most significant byte of the header packet's word count for long packets, or data 1 for short packets."]
    #[inline(always)]
    pub fn wcmsb(&self) -> WCMSB_R {
        WCMSB_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Type This field configures the packet data type of the header packet."]
    #[inline(always)]
    #[must_use]
    pub fn dt(&mut self) -> DT_W<DSI_GHCRrs> {
        DT_W::new(self, 0)
    }
    #[doc = "Bits 6:7 - Channel This field configures the virtual channel ID of the header packet."]
    #[inline(always)]
    #[must_use]
    pub fn vcid(&mut self) -> VCID_W<DSI_GHCRrs> {
        VCID_W::new(self, 6)
    }
    #[doc = "Bits 8:15 - WordCount LSB This field configures the less significant byte of the header packet word count for long packets, or data 0 for short packets."]
    #[inline(always)]
    #[must_use]
    pub fn wclsb(&mut self) -> WCLSB_W<DSI_GHCRrs> {
        WCLSB_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - WordCount MSB This field configures the most significant byte of the header packet's word count for long packets, or data 1 for short packets."]
    #[inline(always)]
    #[must_use]
    pub fn wcmsb(&mut self) -> WCMSB_W<DSI_GHCRrs> {
        WCMSB_W::new(self, 16)
    }
}
#[doc = "DSI Host generic header configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_ghcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_ghcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_GHCRrs;
impl crate::RegisterSpec for DSI_GHCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_ghcr::R`](R) reader structure"]
impl crate::Readable for DSI_GHCRrs {}
#[doc = "`write(|w| ..)` method takes [`dsi_ghcr::W`](W) writer structure"]
impl crate::Writable for DSI_GHCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSI_GHCR to value 0"]
impl crate::Resettable for DSI_GHCRrs {
    const RESET_VALUE: u32 = 0;
}
