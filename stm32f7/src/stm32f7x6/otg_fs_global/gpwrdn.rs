#[doc = "Register `GPWRDN` reader"]
pub type R = crate::R<GPWRDNrs>;
#[doc = "Register `GPWRDN` writer"]
pub type W = crate::W<GPWRDNrs>;
#[doc = "Field `ADPMEN` reader - ADP module enable"]
pub type ADPMEN_R = crate::BitReader;
#[doc = "Field `ADPMEN` writer - ADP module enable"]
pub type ADPMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADPIF` reader - ADP interrupt flag"]
pub type ADPIF_R = crate::BitReader;
#[doc = "Field `ADPIF` writer - ADP interrupt flag"]
pub type ADPIF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ADP module enable"]
    #[inline(always)]
    pub fn adpmen(&self) -> ADPMEN_R {
        ADPMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 23 - ADP interrupt flag"]
    #[inline(always)]
    pub fn adpif(&self) -> ADPIF_R {
        ADPIF_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADP module enable"]
    #[inline(always)]
    #[must_use]
    pub fn adpmen(&mut self) -> ADPMEN_W<GPWRDNrs> {
        ADPMEN_W::new(self, 0)
    }
    #[doc = "Bit 23 - ADP interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn adpif(&mut self) -> ADPIF_W<GPWRDNrs> {
        ADPIF_W::new(self, 23)
    }
}
#[doc = "OTG power down register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpwrdn::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpwrdn::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPWRDNrs;
impl crate::RegisterSpec for GPWRDNrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpwrdn::R`](R) reader structure"]
impl crate::Readable for GPWRDNrs {}
#[doc = "`write(|w| ..)` method takes [`gpwrdn::W`](W) writer structure"]
impl crate::Writable for GPWRDNrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPWRDN to value 0x0200_0400"]
impl crate::Resettable for GPWRDNrs {
    const RESET_VALUE: u32 = 0x0200_0400;
}
