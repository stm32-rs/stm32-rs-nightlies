#[doc = "Register `WKUPCR` reader"]
pub type R = crate::R<WKUPCRrs>;
#[doc = "Register `WKUPCR` writer"]
pub type W = crate::W<WKUPCRrs>;
#[doc = "Field `WKUPC` reader - Clear Wakeup pin flag for WKUP. These bits are always read as 0."]
pub type WKUPC_R = crate::FieldReader;
#[doc = "Field `WKUPC` writer - Clear Wakeup pin flag for WKUP. These bits are always read as 0."]
pub type WKUPC_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Clear Wakeup pin flag for WKUP. These bits are always read as 0."]
    #[inline(always)]
    pub fn wkupc(&self) -> WKUPC_R {
        WKUPC_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Clear Wakeup pin flag for WKUP. These bits are always read as 0."]
    #[inline(always)]
    #[must_use]
    pub fn wkupc(&mut self) -> WKUPC_W<WKUPCRrs> {
        WKUPC_W::new(self, 0)
    }
}
#[doc = "reset only by system reset, not reset by wakeup from Standby mode5 wait states are required when writing this register (when clearing a WKUPF bit in PWR_WKUPFR, the AHB write access will complete after the WKUPF has been cleared).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wkupcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wkupcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WKUPCRrs;
impl crate::RegisterSpec for WKUPCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wkupcr::R`](R) reader structure"]
impl crate::Readable for WKUPCRrs {}
#[doc = "`write(|w| ..)` method takes [`wkupcr::W`](W) writer structure"]
impl crate::Writable for WKUPCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WKUPCR to value 0"]
impl crate::Resettable for WKUPCRrs {
    const RESET_VALUE: u32 = 0;
}
