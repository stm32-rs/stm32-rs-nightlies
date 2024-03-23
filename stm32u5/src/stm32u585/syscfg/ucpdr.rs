#[doc = "Register `UCPDR` reader"]
pub type R = crate::R<UCPDRrs>;
#[doc = "Register `UCPDR` writer"]
pub type W = crate::W<UCPDRrs>;
#[doc = "Field `CC1ENRXFILTER` reader - CC1ENRXFILTER"]
pub type CC1ENRXFILTER_R = crate::BitReader;
#[doc = "Field `CC1ENRXFILTER` writer - CC1ENRXFILTER"]
pub type CC1ENRXFILTER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2ENRXFILTER` reader - CC2ENRXFILTER"]
pub type CC2ENRXFILTER_R = crate::BitReader;
#[doc = "Field `CC2ENRXFILTER` writer - CC2ENRXFILTER"]
pub type CC2ENRXFILTER_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CC1ENRXFILTER"]
    #[inline(always)]
    pub fn cc1enrxfilter(&self) -> CC1ENRXFILTER_R {
        CC1ENRXFILTER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CC2ENRXFILTER"]
    #[inline(always)]
    pub fn cc2enrxfilter(&self) -> CC2ENRXFILTER_R {
        CC2ENRXFILTER_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CC1ENRXFILTER"]
    #[inline(always)]
    #[must_use]
    pub fn cc1enrxfilter(&mut self) -> CC1ENRXFILTER_W<UCPDRrs> {
        CC1ENRXFILTER_W::new(self, 0)
    }
    #[doc = "Bit 1 - CC2ENRXFILTER"]
    #[inline(always)]
    #[must_use]
    pub fn cc2enrxfilter(&mut self) -> CC2ENRXFILTER_W<UCPDRrs> {
        CC2ENRXFILTER_W::new(self, 1)
    }
}
#[doc = "USB Type C and Power Delivery register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucpdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucpdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UCPDRrs;
impl crate::RegisterSpec for UCPDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ucpdr::R`](R) reader structure"]
impl crate::Readable for UCPDRrs {}
#[doc = "`write(|w| ..)` method takes [`ucpdr::W`](W) writer structure"]
impl crate::Writable for UCPDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UCPDR to value 0"]
impl crate::Resettable for UCPDRrs {
    const RESET_VALUE: u32 = 0;
}
