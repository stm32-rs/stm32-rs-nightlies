#[doc = "Register `MACRxFCR` reader"]
pub type R = crate::R<MACRX_FCRrs>;
#[doc = "Register `MACRxFCR` writer"]
pub type W = crate::W<MACRX_FCRrs>;
#[doc = "Field `RFE` reader - Receive Flow Control Enable"]
pub type RFE_R = crate::BitReader;
#[doc = "Field `RFE` writer - Receive Flow Control Enable"]
pub type RFE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UP` reader - Unicast Pause Packet Detect"]
pub type UP_R = crate::BitReader;
#[doc = "Field `UP` writer - Unicast Pause Packet Detect"]
pub type UP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Receive Flow Control Enable"]
    #[inline(always)]
    pub fn rfe(&self) -> RFE_R {
        RFE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Unicast Pause Packet Detect"]
    #[inline(always)]
    pub fn up(&self) -> UP_R {
        UP_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive Flow Control Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rfe(&mut self) -> RFE_W<MACRX_FCRrs> {
        RFE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Unicast Pause Packet Detect"]
    #[inline(always)]
    #[must_use]
    pub fn up(&mut self) -> UP_W<MACRX_FCRrs> {
        UP_W::new(self, 1)
    }
}
#[doc = "Rx flow control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macrx_fcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macrx_fcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACRX_FCRrs;
impl crate::RegisterSpec for MACRX_FCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macrx_fcr::R`](R) reader structure"]
impl crate::Readable for MACRX_FCRrs {}
#[doc = "`write(|w| ..)` method takes [`macrx_fcr::W`](W) writer structure"]
impl crate::Writable for MACRX_FCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACRxFCR to value 0"]
impl crate::Resettable for MACRX_FCRrs {
    const RESET_VALUE: u32 = 0;
}
