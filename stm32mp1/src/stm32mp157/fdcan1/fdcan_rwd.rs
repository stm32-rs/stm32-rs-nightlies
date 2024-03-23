#[doc = "Register `FDCAN_RWD` reader"]
pub type R = crate::R<FDCAN_RWDrs>;
#[doc = "Register `FDCAN_RWD` writer"]
pub type W = crate::W<FDCAN_RWDrs>;
#[doc = "Field `WDC` reader - WDC"]
pub type WDC_R = crate::FieldReader;
#[doc = "Field `WDC` writer - WDC"]
pub type WDC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WDV` reader - WDV"]
pub type WDV_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - WDC"]
    #[inline(always)]
    pub fn wdc(&self) -> WDC_R {
        WDC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - WDV"]
    #[inline(always)]
    pub fn wdv(&self) -> WDV_R {
        WDV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - WDC"]
    #[inline(always)]
    #[must_use]
    pub fn wdc(&mut self) -> WDC_W<FDCAN_RWDrs> {
        WDC_W::new(self, 0)
    }
}
#[doc = "The RAM watchdog monitors the READY output of the message RAM. A message RAM access starts the message RAM watchdog counter with the value configured by the FDCAN_RWD.WDC bits. The counter is reloaded with FDCAN_RWD.WDC bits when the message RAM signals successful completion by activating its READY output. In case there is no response from the message RAM until the counter has counted down to 0, the counter stops and interrupt flag FDCAN_IR.WDI bit is set. The RAM watchdog counter is clocked by the fdcan_pclk clock.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_rwd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_rwd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_RWDrs;
impl crate::RegisterSpec for FDCAN_RWDrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_rwd::R`](R) reader structure"]
impl crate::Readable for FDCAN_RWDrs {}
#[doc = "`write(|w| ..)` method takes [`fdcan_rwd::W`](W) writer structure"]
impl crate::Writable for FDCAN_RWDrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FDCAN_RWD to value 0"]
impl crate::Resettable for FDCAN_RWDrs {
    const RESET_VALUE: u32 = 0;
}
