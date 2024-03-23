#[doc = "Register `CWD` reader"]
pub type R = crate::R<CWDrs>;
#[doc = "Register `CWD` writer"]
pub type W = crate::W<CWDrs>;
#[doc = "Field `WDC` reader - WDC"]
pub type WDC_R = crate::FieldReader<u16>;
#[doc = "Field `WDC` writer - WDC"]
pub type WDC_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `WDV` reader - WDV"]
pub type WDV_R = crate::FieldReader<u16>;
#[doc = "Field `WDV` writer - WDV"]
pub type WDV_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - WDC"]
    #[inline(always)]
    pub fn wdc(&self) -> WDC_R {
        WDC_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - WDV"]
    #[inline(always)]
    pub fn wdv(&self) -> WDV_R {
        WDV_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - WDC"]
    #[inline(always)]
    #[must_use]
    pub fn wdc(&mut self) -> WDC_W<CWDrs> {
        WDC_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - WDV"]
    #[inline(always)]
    #[must_use]
    pub fn wdv(&mut self) -> WDV_W<CWDrs> {
        WDV_W::new(self, 16)
    }
}
#[doc = "Calibration Watchdog Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cwd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cwd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CWDrs;
impl crate::RegisterSpec for CWDrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cwd::R`](R) reader structure"]
impl crate::Readable for CWDrs {}
#[doc = "`write(|w| ..)` method takes [`cwd::W`](W) writer structure"]
impl crate::Writable for CWDrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CWD to value 0"]
impl crate::Resettable for CWDrs {
    const RESET_VALUE: u32 = 0;
}
