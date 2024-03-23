#[doc = "Register `OTG_HAINTMSK` reader"]
pub type R = crate::R<OTG_HAINTMSKrs>;
#[doc = "Register `OTG_HAINTMSK` writer"]
pub type W = crate::W<OTG_HAINTMSKrs>;
#[doc = "Field `HAINTM` reader - HAINTM"]
pub type HAINTM_R = crate::FieldReader<u16>;
#[doc = "Field `HAINTM` writer - HAINTM"]
pub type HAINTM_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - HAINTM"]
    #[inline(always)]
    pub fn haintm(&self) -> HAINTM_R {
        HAINTM_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - HAINTM"]
    #[inline(always)]
    #[must_use]
    pub fn haintm(&mut self) -> HAINTM_W<OTG_HAINTMSKrs> {
        HAINTM_W::new(self, 0)
    }
}
#[doc = "The host all channel interrupt mask register works with the host all channel interrupt register to interrupt the application when an event occurs on a channel. There is one interrupt mask bit per channel, up to a maximum of 16 bits.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_haintmsk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_haintmsk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTG_HAINTMSKrs;
impl crate::RegisterSpec for OTG_HAINTMSKrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_haintmsk::R`](R) reader structure"]
impl crate::Readable for OTG_HAINTMSKrs {}
#[doc = "`write(|w| ..)` method takes [`otg_haintmsk::W`](W) writer structure"]
impl crate::Writable for OTG_HAINTMSKrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OTG_HAINTMSK to value 0"]
impl crate::Resettable for OTG_HAINTMSKrs {
    const RESET_VALUE: u32 = 0;
}
