#[doc = "Register `ETH_DMAA4RxACR` reader"]
pub type R = crate::R<ETH_DMAA4RX_ACRrs>;
#[doc = "Register `ETH_DMAA4RxACR` writer"]
pub type W = crate::W<ETH_DMAA4RX_ACRrs>;
#[doc = "Field `RDWC` reader - RDWC"]
pub type RDWC_R = crate::FieldReader;
#[doc = "Field `RDWC` writer - RDWC"]
pub type RDWC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RPC` reader - RPC"]
pub type RPC_R = crate::FieldReader;
#[doc = "Field `RPC` writer - RPC"]
pub type RPC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RHC` reader - RHC"]
pub type RHC_R = crate::FieldReader;
#[doc = "Field `RHC` writer - RHC"]
pub type RHC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RDC` reader - RDC"]
pub type RDC_R = crate::FieldReader;
#[doc = "Field `RDC` writer - RDC"]
pub type RDC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3 - RDWC"]
    #[inline(always)]
    pub fn rdwc(&self) -> RDWC_R {
        RDWC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - RPC"]
    #[inline(always)]
    pub fn rpc(&self) -> RPC_R {
        RPC_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - RHC"]
    #[inline(always)]
    pub fn rhc(&self) -> RHC_R {
        RHC_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:25 - RDC"]
    #[inline(always)]
    pub fn rdc(&self) -> RDC_R {
        RDC_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - RDWC"]
    #[inline(always)]
    #[must_use]
    pub fn rdwc(&mut self) -> RDWC_W<ETH_DMAA4RX_ACRrs> {
        RDWC_W::new(self, 0)
    }
    #[doc = "Bits 8:11 - RPC"]
    #[inline(always)]
    #[must_use]
    pub fn rpc(&mut self) -> RPC_W<ETH_DMAA4RX_ACRrs> {
        RPC_W::new(self, 8)
    }
    #[doc = "Bits 16:19 - RHC"]
    #[inline(always)]
    #[must_use]
    pub fn rhc(&mut self) -> RHC_W<ETH_DMAA4RX_ACRrs> {
        RHC_W::new(self, 16)
    }
    #[doc = "Bits 24:25 - RDC"]
    #[inline(always)]
    #[must_use]
    pub fn rdc(&mut self) -> RDC_W<ETH_DMAA4RX_ACRrs> {
        RDC_W::new(self, 24)
    }
}
#[doc = "AXI4 receive channel ACE control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_dmaa4rx_acr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_dmaa4rx_acr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_DMAA4RX_ACRrs;
impl crate::RegisterSpec for ETH_DMAA4RX_ACRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_dmaa4rx_acr::R`](R) reader structure"]
impl crate::Readable for ETH_DMAA4RX_ACRrs {}
#[doc = "`write(|w| ..)` method takes [`eth_dmaa4rx_acr::W`](W) writer structure"]
impl crate::Writable for ETH_DMAA4RX_ACRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETH_DMAA4RxACR to value 0"]
impl crate::Resettable for ETH_DMAA4RX_ACRrs {
    const RESET_VALUE: u32 = 0;
}
