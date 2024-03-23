#[doc = "Register `RQR` writer"]
pub type W = crate::W<RQRrs>;
#[doc = "Field `ABRRQ` writer - Auto baud rate request"]
pub type ABRRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SBKRQ` writer - Send break request"]
pub type SBKRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MMRQ` writer - Mute mode request"]
pub type MMRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFRQ` writer - Receive data flush request"]
pub type RXFRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFRQ` writer - Transmit data flush request"]
pub type TXFRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Auto baud rate request"]
    #[inline(always)]
    #[must_use]
    pub fn abrrq(&mut self) -> ABRRQ_W<RQRrs> {
        ABRRQ_W::new(self, 0)
    }
    #[doc = "Bit 1 - Send break request"]
    #[inline(always)]
    #[must_use]
    pub fn sbkrq(&mut self) -> SBKRQ_W<RQRrs> {
        SBKRQ_W::new(self, 1)
    }
    #[doc = "Bit 2 - Mute mode request"]
    #[inline(always)]
    #[must_use]
    pub fn mmrq(&mut self) -> MMRQ_W<RQRrs> {
        MMRQ_W::new(self, 2)
    }
    #[doc = "Bit 3 - Receive data flush request"]
    #[inline(always)]
    #[must_use]
    pub fn rxfrq(&mut self) -> RXFRQ_W<RQRrs> {
        RXFRQ_W::new(self, 3)
    }
    #[doc = "Bit 4 - Transmit data flush request"]
    #[inline(always)]
    #[must_use]
    pub fn txfrq(&mut self) -> TXFRQ_W<RQRrs> {
        TXFRQ_W::new(self, 4)
    }
}
#[doc = "Request register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rqr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RQRrs;
impl crate::RegisterSpec for RQRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`rqr::W`](W) writer structure"]
impl crate::Writable for RQRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RQR to value 0"]
impl crate::Resettable for RQRrs {
    const RESET_VALUE: u32 = 0;
}
