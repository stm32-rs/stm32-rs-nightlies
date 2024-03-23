#[doc = "Register `GICD_SGIR` writer"]
pub type W = crate::W<GICD_SGIRrs>;
#[doc = "Field `SGIINTID` writer - SGIINTID"]
pub type SGIINTID_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NSATT` writer - NSATT"]
pub type NSATT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPUTARGETLIST` writer - CPUTARGETLIST"]
pub type CPUTARGETLIST_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TARGETLISTFILTER` writer - TARGETLISTFILTER"]
pub type TARGETLISTFILTER_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl W {
    #[doc = "Bits 0:3 - SGIINTID"]
    #[inline(always)]
    #[must_use]
    pub fn sgiintid(&mut self) -> SGIINTID_W<GICD_SGIRrs> {
        SGIINTID_W::new(self, 0)
    }
    #[doc = "Bit 15 - NSATT"]
    #[inline(always)]
    #[must_use]
    pub fn nsatt(&mut self) -> NSATT_W<GICD_SGIRrs> {
        NSATT_W::new(self, 15)
    }
    #[doc = "Bits 16:17 - CPUTARGETLIST"]
    #[inline(always)]
    #[must_use]
    pub fn cputargetlist(&mut self) -> CPUTARGETLIST_W<GICD_SGIRrs> {
        CPUTARGETLIST_W::new(self, 16)
    }
    #[doc = "Bits 24:25 - TARGETLISTFILTER"]
    #[inline(always)]
    #[must_use]
    pub fn targetlistfilter(&mut self) -> TARGETLISTFILTER_W<GICD_SGIRrs> {
        TARGETLISTFILTER_W::new(self, 24)
    }
}
#[doc = "GICD software generated interrupt register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_sgir::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_SGIRrs;
impl crate::RegisterSpec for GICD_SGIRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gicd_sgir::W`](W) writer structure"]
impl crate::Writable for GICD_SGIRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_SGIR to value 0"]
impl crate::Resettable for GICD_SGIRrs {
    const RESET_VALUE: u32 = 0;
}
