#[doc = "Register `OTG_DOEPTSIZ0` reader"]
pub type R = crate::R<OTG_DOEPTSIZ0rs>;
#[doc = "Register `OTG_DOEPTSIZ0` writer"]
pub type W = crate::W<OTG_DOEPTSIZ0rs>;
#[doc = "Field `XFRSIZ` reader - XFRSIZ"]
pub type XFRSIZ_R = crate::FieldReader;
#[doc = "Field `XFRSIZ` writer - XFRSIZ"]
pub type XFRSIZ_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PKTCNT` reader - PKTCNT"]
pub type PKTCNT_R = crate::BitReader;
#[doc = "Field `PKTCNT` writer - PKTCNT"]
pub type PKTCNT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STUPCNT` reader - STUPCNT"]
pub type STUPCNT_R = crate::FieldReader;
#[doc = "Field `STUPCNT` writer - STUPCNT"]
pub type STUPCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:6 - XFRSIZ"]
    #[inline(always)]
    pub fn xfrsiz(&self) -> XFRSIZ_R {
        XFRSIZ_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 19 - PKTCNT"]
    #[inline(always)]
    pub fn pktcnt(&self) -> PKTCNT_R {
        PKTCNT_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 29:30 - STUPCNT"]
    #[inline(always)]
    pub fn stupcnt(&self) -> STUPCNT_R {
        STUPCNT_R::new(((self.bits >> 29) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - XFRSIZ"]
    #[inline(always)]
    #[must_use]
    pub fn xfrsiz(&mut self) -> XFRSIZ_W<OTG_DOEPTSIZ0rs> {
        XFRSIZ_W::new(self, 0)
    }
    #[doc = "Bit 19 - PKTCNT"]
    #[inline(always)]
    #[must_use]
    pub fn pktcnt(&mut self) -> PKTCNT_W<OTG_DOEPTSIZ0rs> {
        PKTCNT_W::new(self, 19)
    }
    #[doc = "Bits 29:30 - STUPCNT"]
    #[inline(always)]
    #[must_use]
    pub fn stupcnt(&mut self) -> STUPCNT_W<OTG_DOEPTSIZ0rs> {
        STUPCNT_W::new(self, 29)
    }
}
#[doc = "The application must modify this register before enabling endpoint 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_doeptsiz0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_doeptsiz0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTG_DOEPTSIZ0rs;
impl crate::RegisterSpec for OTG_DOEPTSIZ0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_doeptsiz0::R`](R) reader structure"]
impl crate::Readable for OTG_DOEPTSIZ0rs {}
#[doc = "`write(|w| ..)` method takes [`otg_doeptsiz0::W`](W) writer structure"]
impl crate::Writable for OTG_DOEPTSIZ0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OTG_DOEPTSIZ0 to value 0"]
impl crate::Resettable for OTG_DOEPTSIZ0rs {
    const RESET_VALUE: u32 = 0;
}
