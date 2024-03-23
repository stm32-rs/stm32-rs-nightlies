#[doc = "Register `ETH_MACWTR` reader"]
pub type R = crate::R<ETH_MACWTRrs>;
#[doc = "Register `ETH_MACWTR` writer"]
pub type W = crate::W<ETH_MACWTRrs>;
#[doc = "Field `WTO` reader - WTO"]
pub type WTO_R = crate::FieldReader;
#[doc = "Field `WTO` writer - WTO"]
pub type WTO_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PWE` reader - PWE"]
pub type PWE_R = crate::BitReader;
#[doc = "Field `PWE` writer - PWE"]
pub type PWE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - WTO"]
    #[inline(always)]
    pub fn wto(&self) -> WTO_R {
        WTO_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - PWE"]
    #[inline(always)]
    pub fn pwe(&self) -> PWE_R {
        PWE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - WTO"]
    #[inline(always)]
    #[must_use]
    pub fn wto(&mut self) -> WTO_W<ETH_MACWTRrs> {
        WTO_W::new(self, 0)
    }
    #[doc = "Bit 8 - PWE"]
    #[inline(always)]
    #[must_use]
    pub fn pwe(&mut self) -> PWE_W<ETH_MACWTRrs> {
        PWE_W::new(self, 8)
    }
}
#[doc = "The Watchdog Timeout register controls the watchdog timeout for received packets.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macwtr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_macwtr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_MACWTRrs;
impl crate::RegisterSpec for ETH_MACWTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_macwtr::R`](R) reader structure"]
impl crate::Readable for ETH_MACWTRrs {}
#[doc = "`write(|w| ..)` method takes [`eth_macwtr::W`](W) writer structure"]
impl crate::Writable for ETH_MACWTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETH_MACWTR to value 0"]
impl crate::Resettable for ETH_MACWTRrs {
    const RESET_VALUE: u32 = 0;
}
