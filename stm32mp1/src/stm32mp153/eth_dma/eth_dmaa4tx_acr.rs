#[doc = "Register `ETH_DMAA4TxACR` reader"]
pub type R = crate::R<ETH_DMAA4TX_ACRrs>;
#[doc = "Register `ETH_DMAA4TxACR` writer"]
pub type W = crate::W<ETH_DMAA4TX_ACRrs>;
#[doc = "Field `TDRC` reader - TDRC"]
pub type TDRC_R = crate::FieldReader;
#[doc = "Field `TDRC` writer - TDRC"]
pub type TDRC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TEC` reader - TEC"]
pub type TEC_R = crate::FieldReader;
#[doc = "Field `TEC` writer - TEC"]
pub type TEC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `THC` reader - THC"]
pub type THC_R = crate::FieldReader;
#[doc = "Field `THC` writer - THC"]
pub type THC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - TDRC"]
    #[inline(always)]
    pub fn tdrc(&self) -> TDRC_R {
        TDRC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - TEC"]
    #[inline(always)]
    pub fn tec(&self) -> TEC_R {
        TEC_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - THC"]
    #[inline(always)]
    pub fn thc(&self) -> THC_R {
        THC_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - TDRC"]
    #[inline(always)]
    #[must_use]
    pub fn tdrc(&mut self) -> TDRC_W<ETH_DMAA4TX_ACRrs> {
        TDRC_W::new(self, 0)
    }
    #[doc = "Bits 8:11 - TEC"]
    #[inline(always)]
    #[must_use]
    pub fn tec(&mut self) -> TEC_W<ETH_DMAA4TX_ACRrs> {
        TEC_W::new(self, 8)
    }
    #[doc = "Bits 16:19 - THC"]
    #[inline(always)]
    #[must_use]
    pub fn thc(&mut self) -> THC_W<ETH_DMAA4TX_ACRrs> {
        THC_W::new(self, 16)
    }
}
#[doc = "AXI4 transmit channel ACE control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_dmaa4tx_acr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_dmaa4tx_acr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_DMAA4TX_ACRrs;
impl crate::RegisterSpec for ETH_DMAA4TX_ACRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_dmaa4tx_acr::R`](R) reader structure"]
impl crate::Readable for ETH_DMAA4TX_ACRrs {}
#[doc = "`write(|w| ..)` method takes [`eth_dmaa4tx_acr::W`](W) writer structure"]
impl crate::Writable for ETH_DMAA4TX_ACRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETH_DMAA4TxACR to value 0"]
impl crate::Resettable for ETH_DMAA4TX_ACRrs {
    const RESET_VALUE: u32 = 0;
}
