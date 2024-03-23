#[doc = "Register `ETH_DMAA4DACR` reader"]
pub type R = crate::R<ETH_DMAA4DACRrs>;
#[doc = "Register `ETH_DMAA4DACR` writer"]
pub type W = crate::W<ETH_DMAA4DACRrs>;
#[doc = "Field `TDWC` reader - TDWC"]
pub type TDWC_R = crate::FieldReader;
#[doc = "Field `TDWC` writer - TDWC"]
pub type TDWC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TDWD` reader - TDWD"]
pub type TDWD_R = crate::FieldReader;
#[doc = "Field `TDWD` writer - TDWD"]
pub type TDWD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RDRC` reader - RDRC"]
pub type RDRC_R = crate::FieldReader;
#[doc = "Field `RDRC` writer - RDRC"]
pub type RDRC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RDP` reader - RDP"]
pub type RDP_R = crate::FieldReader;
#[doc = "Field `RDP` writer - RDP"]
pub type RDP_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WRP` reader - WRP"]
pub type WRP_R = crate::FieldReader;
#[doc = "Field `WRP` writer - WRP"]
pub type WRP_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:3 - TDWC"]
    #[inline(always)]
    pub fn tdwc(&self) -> TDWC_R {
        TDWC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - TDWD"]
    #[inline(always)]
    pub fn tdwd(&self) -> TDWD_R {
        TDWD_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:11 - RDRC"]
    #[inline(always)]
    pub fn rdrc(&self) -> RDRC_R {
        RDRC_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:18 - RDP"]
    #[inline(always)]
    pub fn rdp(&self) -> RDP_R {
        RDP_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - WRP"]
    #[inline(always)]
    pub fn wrp(&self) -> WRP_R {
        WRP_R::new(((self.bits >> 20) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - TDWC"]
    #[inline(always)]
    #[must_use]
    pub fn tdwc(&mut self) -> TDWC_W<ETH_DMAA4DACRrs> {
        TDWC_W::new(self, 0)
    }
    #[doc = "Bits 4:5 - TDWD"]
    #[inline(always)]
    #[must_use]
    pub fn tdwd(&mut self) -> TDWD_W<ETH_DMAA4DACRrs> {
        TDWD_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - RDRC"]
    #[inline(always)]
    #[must_use]
    pub fn rdrc(&mut self) -> RDRC_W<ETH_DMAA4DACRrs> {
        RDRC_W::new(self, 8)
    }
    #[doc = "Bits 16:18 - RDP"]
    #[inline(always)]
    #[must_use]
    pub fn rdp(&mut self) -> RDP_W<ETH_DMAA4DACRrs> {
        RDP_W::new(self, 16)
    }
    #[doc = "Bits 20:22 - WRP"]
    #[inline(always)]
    #[must_use]
    pub fn wrp(&mut self) -> WRP_W<ETH_DMAA4DACRrs> {
        WRP_W::new(self, 20)
    }
}
#[doc = "AXI4 descriptor ACE control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_dmaa4dacr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_dmaa4dacr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_DMAA4DACRrs;
impl crate::RegisterSpec for ETH_DMAA4DACRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_dmaa4dacr::R`](R) reader structure"]
impl crate::Readable for ETH_DMAA4DACRrs {}
#[doc = "`write(|w| ..)` method takes [`eth_dmaa4dacr::W`](W) writer structure"]
impl crate::Writable for ETH_DMAA4DACRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETH_DMAA4DACR to value 0"]
impl crate::Resettable for ETH_DMAA4DACRrs {
    const RESET_VALUE: u32 = 0;
}
