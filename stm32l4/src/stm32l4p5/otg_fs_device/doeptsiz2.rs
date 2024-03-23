#[doc = "Register `DOEPTSIZ2` reader"]
pub type R = crate::R<DOEPTSIZ2rs>;
#[doc = "Register `DOEPTSIZ2` writer"]
pub type W = crate::W<DOEPTSIZ2rs>;
#[doc = "Field `XFRSIZ` reader - Transfer size"]
pub type XFRSIZ_R = crate::FieldReader<u32>;
#[doc = "Field `XFRSIZ` writer - Transfer size"]
pub type XFRSIZ_W<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
#[doc = "Field `PKTCNT` reader - Packet count"]
pub type PKTCNT_R = crate::FieldReader<u16>;
#[doc = "Field `PKTCNT` writer - Packet count"]
pub type PKTCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `RXDPID_STUPCNT` reader - Received data PID/SETUP packet count"]
pub type RXDPID_STUPCNT_R = crate::FieldReader;
#[doc = "Field `RXDPID_STUPCNT` writer - Received data PID/SETUP packet count"]
pub type RXDPID_STUPCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:18 - Transfer size"]
    #[inline(always)]
    pub fn xfrsiz(&self) -> XFRSIZ_R {
        XFRSIZ_R::new(self.bits & 0x0007_ffff)
    }
    #[doc = "Bits 19:28 - Packet count"]
    #[inline(always)]
    pub fn pktcnt(&self) -> PKTCNT_R {
        PKTCNT_R::new(((self.bits >> 19) & 0x03ff) as u16)
    }
    #[doc = "Bits 29:30 - Received data PID/SETUP packet count"]
    #[inline(always)]
    pub fn rxdpid_stupcnt(&self) -> RXDPID_STUPCNT_R {
        RXDPID_STUPCNT_R::new(((self.bits >> 29) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:18 - Transfer size"]
    #[inline(always)]
    #[must_use]
    pub fn xfrsiz(&mut self) -> XFRSIZ_W<DOEPTSIZ2rs> {
        XFRSIZ_W::new(self, 0)
    }
    #[doc = "Bits 19:28 - Packet count"]
    #[inline(always)]
    #[must_use]
    pub fn pktcnt(&mut self) -> PKTCNT_W<DOEPTSIZ2rs> {
        PKTCNT_W::new(self, 19)
    }
    #[doc = "Bits 29:30 - Received data PID/SETUP packet count"]
    #[inline(always)]
    #[must_use]
    pub fn rxdpid_stupcnt(&mut self) -> RXDPID_STUPCNT_W<DOEPTSIZ2rs> {
        RXDPID_STUPCNT_W::new(self, 29)
    }
}
#[doc = "device OUT endpoint-2 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doeptsiz2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doeptsiz2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOEPTSIZ2rs;
impl crate::RegisterSpec for DOEPTSIZ2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doeptsiz2::R`](R) reader structure"]
impl crate::Readable for DOEPTSIZ2rs {}
#[doc = "`write(|w| ..)` method takes [`doeptsiz2::W`](W) writer structure"]
impl crate::Writable for DOEPTSIZ2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOEPTSIZ2 to value 0"]
impl crate::Resettable for DOEPTSIZ2rs {
    const RESET_VALUE: u32 = 0;
}
