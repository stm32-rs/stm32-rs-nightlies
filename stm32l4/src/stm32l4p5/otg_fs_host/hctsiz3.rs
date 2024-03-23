#[doc = "Register `HCTSIZ3` reader"]
pub type R = crate::R<HCTSIZ3rs>;
#[doc = "Register `HCTSIZ3` writer"]
pub type W = crate::W<HCTSIZ3rs>;
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
#[doc = "Field `DOPNG` reader - Do Ping"]
pub type DOPNG_R = crate::BitReader;
#[doc = "Field `DOPNG` writer - Do Ping"]
pub type DOPNG_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 31 - Do Ping"]
    #[inline(always)]
    pub fn dopng(&self) -> DOPNG_R {
        DOPNG_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:18 - Transfer size"]
    #[inline(always)]
    #[must_use]
    pub fn xfrsiz(&mut self) -> XFRSIZ_W<HCTSIZ3rs> {
        XFRSIZ_W::new(self, 0)
    }
    #[doc = "Bits 19:28 - Packet count"]
    #[inline(always)]
    #[must_use]
    pub fn pktcnt(&mut self) -> PKTCNT_W<HCTSIZ3rs> {
        PKTCNT_W::new(self, 19)
    }
    #[doc = "Bits 29:30 - Data PID"]
    #[inline(always)]
    #[must_use]
    pub fn dpid(&mut self) -> DPID_W<HCTSIZ3rs> {
        DPID_W::new(self, 29)
    }
    #[doc = "Bit 31 - Do Ping"]
    #[inline(always)]
    #[must_use]
    pub fn dopng(&mut self) -> DOPNG_W<HCTSIZ3rs> {
        DOPNG_W::new(self, 31)
    }
}
#[doc = "OTG_FS host channel-3 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hctsiz3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hctsiz3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCTSIZ3rs;
impl crate::RegisterSpec for HCTSIZ3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hctsiz3::R`](R) reader structure"]
impl crate::Readable for HCTSIZ3rs {}
#[doc = "`write(|w| ..)` method takes [`hctsiz3::W`](W) writer structure"]
impl crate::Writable for HCTSIZ3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HCTSIZ3 to value 0"]
impl crate::Resettable for HCTSIZ3rs {
    const RESET_VALUE: u32 = 0;
}
