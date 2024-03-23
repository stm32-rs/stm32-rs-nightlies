#[doc = "Register `OTG_DOEPTSIZ8` reader"]
pub type R = crate::R<OTG_DOEPTSIZ8rs>;
#[doc = "Register `OTG_DOEPTSIZ8` writer"]
pub type W = crate::W<OTG_DOEPTSIZ8rs>;
#[doc = "Field `XFRSIZ` reader - XFRSIZ"]
pub type XFRSIZ_R = crate::FieldReader<u32>;
#[doc = "Field `XFRSIZ` writer - XFRSIZ"]
pub type XFRSIZ_W<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
#[doc = "Field `PKTCNT` reader - PKTCNT"]
pub type PKTCNT_R = crate::FieldReader<u16>;
#[doc = "Field `PKTCNT` writer - PKTCNT"]
pub type PKTCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `RXDPID_STUPCNT` reader - RXDPID_STUPCNT"]
pub type RXDPID_STUPCNT_R = crate::FieldReader;
#[doc = "Field `RXDPID_STUPCNT` writer - RXDPID_STUPCNT"]
pub type RXDPID_STUPCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:18 - XFRSIZ"]
    #[inline(always)]
    pub fn xfrsiz(&self) -> XFRSIZ_R {
        XFRSIZ_R::new(self.bits & 0x0007_ffff)
    }
    #[doc = "Bits 19:28 - PKTCNT"]
    #[inline(always)]
    pub fn pktcnt(&self) -> PKTCNT_R {
        PKTCNT_R::new(((self.bits >> 19) & 0x03ff) as u16)
    }
    #[doc = "Bits 29:30 - RXDPID_STUPCNT"]
    #[inline(always)]
    pub fn rxdpid_stupcnt(&self) -> RXDPID_STUPCNT_R {
        RXDPID_STUPCNT_R::new(((self.bits >> 29) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:18 - XFRSIZ"]
    #[inline(always)]
    #[must_use]
    pub fn xfrsiz(&mut self) -> XFRSIZ_W<OTG_DOEPTSIZ8rs> {
        XFRSIZ_W::new(self, 0)
    }
    #[doc = "Bits 19:28 - PKTCNT"]
    #[inline(always)]
    #[must_use]
    pub fn pktcnt(&mut self) -> PKTCNT_W<OTG_DOEPTSIZ8rs> {
        PKTCNT_W::new(self, 19)
    }
    #[doc = "Bits 29:30 - RXDPID_STUPCNT"]
    #[inline(always)]
    #[must_use]
    pub fn rxdpid_stupcnt(&mut self) -> RXDPID_STUPCNT_W<OTG_DOEPTSIZ8rs> {
        RXDPID_STUPCNT_W::new(self, 29)
    }
}
#[doc = "The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in OTG_DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_doeptsiz8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_doeptsiz8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTG_DOEPTSIZ8rs;
impl crate::RegisterSpec for OTG_DOEPTSIZ8rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_doeptsiz8::R`](R) reader structure"]
impl crate::Readable for OTG_DOEPTSIZ8rs {}
#[doc = "`write(|w| ..)` method takes [`otg_doeptsiz8::W`](W) writer structure"]
impl crate::Writable for OTG_DOEPTSIZ8rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OTG_DOEPTSIZ8 to value 0"]
impl crate::Resettable for OTG_DOEPTSIZ8rs {
    const RESET_VALUE: u32 = 0;
}
