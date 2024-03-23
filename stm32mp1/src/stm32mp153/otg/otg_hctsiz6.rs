#[doc = "Register `OTG_HCTSIZ6` reader"]
pub type R = crate::R<OTG_HCTSIZ6rs>;
#[doc = "Register `OTG_HCTSIZ6` writer"]
pub type W = crate::W<OTG_HCTSIZ6rs>;
#[doc = "Field `XFRSIZ` reader - XFRSIZ"]
pub type XFRSIZ_R = crate::FieldReader<u32>;
#[doc = "Field `XFRSIZ` writer - XFRSIZ"]
pub type XFRSIZ_W<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
#[doc = "Field `PKTCNT` reader - PKTCNT"]
pub type PKTCNT_R = crate::FieldReader<u16>;
#[doc = "Field `PKTCNT` writer - PKTCNT"]
pub type PKTCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `DPID` reader - DPID"]
pub type DPID_R = crate::FieldReader;
#[doc = "Field `DPID` writer - DPID"]
pub type DPID_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
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
    #[doc = "Bits 29:30 - DPID"]
    #[inline(always)]
    pub fn dpid(&self) -> DPID_R {
        DPID_R::new(((self.bits >> 29) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:18 - XFRSIZ"]
    #[inline(always)]
    #[must_use]
    pub fn xfrsiz(&mut self) -> XFRSIZ_W<OTG_HCTSIZ6rs> {
        XFRSIZ_W::new(self, 0)
    }
    #[doc = "Bits 19:28 - PKTCNT"]
    #[inline(always)]
    #[must_use]
    pub fn pktcnt(&mut self) -> PKTCNT_W<OTG_HCTSIZ6rs> {
        PKTCNT_W::new(self, 19)
    }
    #[doc = "Bits 29:30 - DPID"]
    #[inline(always)]
    #[must_use]
    pub fn dpid(&mut self) -> DPID_W<OTG_HCTSIZ6rs> {
        DPID_W::new(self, 29)
    }
}
#[doc = "OTG host channel 6 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hctsiz6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hctsiz6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTG_HCTSIZ6rs;
impl crate::RegisterSpec for OTG_HCTSIZ6rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_hctsiz6::R`](R) reader structure"]
impl crate::Readable for OTG_HCTSIZ6rs {}
#[doc = "`write(|w| ..)` method takes [`otg_hctsiz6::W`](W) writer structure"]
impl crate::Writable for OTG_HCTSIZ6rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OTG_HCTSIZ6 to value 0"]
impl crate::Resettable for OTG_HCTSIZ6rs {
    const RESET_VALUE: u32 = 0;
}
