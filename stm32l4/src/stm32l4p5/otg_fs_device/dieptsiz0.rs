#[doc = "Register `DIEPTSIZ0` reader"]
pub type R = crate::R<DIEPTSIZ0rs>;
#[doc = "Register `DIEPTSIZ0` writer"]
pub type W = crate::W<DIEPTSIZ0rs>;
#[doc = "Field `XFRSIZ` reader - Transfer size"]
pub type XFRSIZ_R = crate::FieldReader;
#[doc = "Field `XFRSIZ` writer - Transfer size"]
pub type XFRSIZ_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PKTCNT` reader - Packet count"]
pub type PKTCNT_R = crate::FieldReader;
#[doc = "Field `PKTCNT` writer - Packet count"]
pub type PKTCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:6 - Transfer size"]
    #[inline(always)]
    pub fn xfrsiz(&self) -> XFRSIZ_R {
        XFRSIZ_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 19:20 - Packet count"]
    #[inline(always)]
    pub fn pktcnt(&self) -> PKTCNT_R {
        PKTCNT_R::new(((self.bits >> 19) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Transfer size"]
    #[inline(always)]
    #[must_use]
    pub fn xfrsiz(&mut self) -> XFRSIZ_W<DIEPTSIZ0rs> {
        XFRSIZ_W::new(self, 0)
    }
    #[doc = "Bits 19:20 - Packet count"]
    #[inline(always)]
    #[must_use]
    pub fn pktcnt(&mut self) -> PKTCNT_W<DIEPTSIZ0rs> {
        PKTCNT_W::new(self, 19)
    }
}
#[doc = "device endpoint-0 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptsiz0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptsiz0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEPTSIZ0rs;
impl crate::RegisterSpec for DIEPTSIZ0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dieptsiz0::R`](R) reader structure"]
impl crate::Readable for DIEPTSIZ0rs {}
#[doc = "`write(|w| ..)` method takes [`dieptsiz0::W`](W) writer structure"]
impl crate::Writable for DIEPTSIZ0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIEPTSIZ0 to value 0"]
impl crate::Resettable for DIEPTSIZ0rs {
    const RESET_VALUE: u32 = 0;
}
