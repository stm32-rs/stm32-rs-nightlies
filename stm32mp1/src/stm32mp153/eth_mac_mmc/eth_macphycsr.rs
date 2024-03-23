#[doc = "Register `ETH_MACPHYCSR` reader"]
pub type R = crate::R<ETH_MACPHYCSRrs>;
#[doc = "Register `ETH_MACPHYCSR` writer"]
pub type W = crate::W<ETH_MACPHYCSRrs>;
#[doc = "Field `TC` reader - TC"]
pub type TC_R = crate::BitReader;
#[doc = "Field `TC` writer - TC"]
pub type TC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LUD` reader - LUD"]
pub type LUD_R = crate::BitReader;
#[doc = "Field `LUD` writer - LUD"]
pub type LUD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LNKMOD` reader - LNKMOD"]
pub type LNKMOD_R = crate::BitReader;
#[doc = "Field `LNKSPEED` reader - LNKSPEED"]
pub type LNKSPEED_R = crate::FieldReader;
#[doc = "Field `LNKSTS` reader - LNKSTS"]
pub type LNKSTS_R = crate::BitReader;
#[doc = "Field `JABTO` reader - JABTO"]
pub type JABTO_R = crate::BitReader;
#[doc = "Field `FALSCARDET` reader - FALSCARDET"]
pub type FALSCARDET_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - TC"]
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LUD"]
    #[inline(always)]
    pub fn lud(&self) -> LUD_R {
        LUD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16 - LNKMOD"]
    #[inline(always)]
    pub fn lnkmod(&self) -> LNKMOD_R {
        LNKMOD_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - LNKSPEED"]
    #[inline(always)]
    pub fn lnkspeed(&self) -> LNKSPEED_R {
        LNKSPEED_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 19 - LNKSTS"]
    #[inline(always)]
    pub fn lnksts(&self) -> LNKSTS_R {
        LNKSTS_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - JABTO"]
    #[inline(always)]
    pub fn jabto(&self) -> JABTO_R {
        JABTO_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - FALSCARDET"]
    #[inline(always)]
    pub fn falscardet(&self) -> FALSCARDET_R {
        FALSCARDET_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TC"]
    #[inline(always)]
    #[must_use]
    pub fn tc(&mut self) -> TC_W<ETH_MACPHYCSRrs> {
        TC_W::new(self, 0)
    }
    #[doc = "Bit 1 - LUD"]
    #[inline(always)]
    #[must_use]
    pub fn lud(&mut self) -> LUD_W<ETH_MACPHYCSRrs> {
        LUD_W::new(self, 1)
    }
}
#[doc = "The PHY Interface Control and Status register indicates the status signals received by the, RGMII interface from the PHY.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macphycsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_macphycsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_MACPHYCSRrs;
impl crate::RegisterSpec for ETH_MACPHYCSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_macphycsr::R`](R) reader structure"]
impl crate::Readable for ETH_MACPHYCSRrs {}
#[doc = "`write(|w| ..)` method takes [`eth_macphycsr::W`](W) writer structure"]
impl crate::Writable for ETH_MACPHYCSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETH_MACPHYCSR to value 0"]
impl crate::Resettable for ETH_MACPHYCSRrs {
    const RESET_VALUE: u32 = 0;
}
