#[doc = "Register `TSIZ` reader"]
pub type R = crate::R<TSIZrs>;
#[doc = "Register `TSIZ` writer"]
pub type W = crate::W<TSIZrs>;
#[doc = "Field `XFRSIZ` reader - Transfer size"]
pub type XFRSIZ_R = crate::FieldReader<u32>;
#[doc = "Field `XFRSIZ` writer - Transfer size"]
pub type XFRSIZ_W<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
#[doc = "Field `PKTCNT` reader - Packet count"]
pub type PKTCNT_R = crate::FieldReader<u16>;
#[doc = "Field `PKTCNT` writer - Packet count"]
pub type PKTCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `DPID` reader - Data PID"]
pub type DPID_R = crate::FieldReader;
#[doc = "Field `DPID` writer - Data PID"]
pub type DPID_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
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
    #[doc = "Bits 29:30 - Data PID"]
    #[inline(always)]
    pub fn dpid(&self) -> DPID_R {
        DPID_R::new(((self.bits >> 29) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:18 - Transfer size"]
    #[inline(always)]
    #[must_use]
    pub fn xfrsiz(&mut self) -> XFRSIZ_W<TSIZrs> {
        XFRSIZ_W::new(self, 0)
    }
    #[doc = "Bits 19:28 - Packet count"]
    #[inline(always)]
    #[must_use]
    pub fn pktcnt(&mut self) -> PKTCNT_W<TSIZrs> {
        PKTCNT_W::new(self, 19)
    }
    #[doc = "Bits 29:30 - Data PID"]
    #[inline(always)]
    #[must_use]
    pub fn dpid(&mut self) -> DPID_W<TSIZrs> {
        DPID_W::new(self, 29)
    }
}
#[doc = "OTG_FS host channel-0 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsiz::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsiz::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TSIZrs;
impl crate::RegisterSpec for TSIZrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsiz::R`](R) reader structure"]
impl crate::Readable for TSIZrs {}
#[doc = "`write(|w| ..)` method takes [`tsiz::W`](W) writer structure"]
impl crate::Writable for TSIZrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TSIZ to value 0"]
impl crate::Resettable for TSIZrs {
    const RESET_VALUE: u32 = 0;
}
