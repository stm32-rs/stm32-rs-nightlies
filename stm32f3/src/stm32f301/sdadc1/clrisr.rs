#[doc = "Register `CLRISR` reader"]
pub type R = crate::R<CLRISRrs>;
#[doc = "Register `CLRISR` writer"]
pub type W = crate::W<CLRISRrs>;
#[doc = "Field `CLREOCALF` reader - Clear the end of calibration flag"]
pub type CLREOCALF_R = crate::BitReader;
#[doc = "Field `CLREOCALF` writer - Clear the end of calibration flag"]
pub type CLREOCALF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLRJOVRF` reader - Clear the injected conversion overrun flag"]
pub type CLRJOVRF_R = crate::BitReader;
#[doc = "Field `CLRJOVRF` writer - Clear the injected conversion overrun flag"]
pub type CLRJOVRF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLRROVRF` reader - Clear the regular conversion overrun flag"]
pub type CLRROVRF_R = crate::BitReader;
#[doc = "Field `CLRROVRF` writer - Clear the regular conversion overrun flag"]
pub type CLRROVRF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Clear the end of calibration flag"]
    #[inline(always)]
    pub fn clreocalf(&self) -> CLREOCALF_R {
        CLREOCALF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Clear the injected conversion overrun flag"]
    #[inline(always)]
    pub fn clrjovrf(&self) -> CLRJOVRF_R {
        CLRJOVRF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Clear the regular conversion overrun flag"]
    #[inline(always)]
    pub fn clrrovrf(&self) -> CLRROVRF_R {
        CLRROVRF_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clear the end of calibration flag"]
    #[inline(always)]
    #[must_use]
    pub fn clreocalf(&mut self) -> CLREOCALF_W<CLRISRrs> {
        CLREOCALF_W::new(self, 0)
    }
    #[doc = "Bit 2 - Clear the injected conversion overrun flag"]
    #[inline(always)]
    #[must_use]
    pub fn clrjovrf(&mut self) -> CLRJOVRF_W<CLRISRrs> {
        CLRJOVRF_W::new(self, 2)
    }
    #[doc = "Bit 4 - Clear the regular conversion overrun flag"]
    #[inline(always)]
    #[must_use]
    pub fn clrrovrf(&mut self) -> CLRROVRF_W<CLRISRrs> {
        CLRROVRF_W::new(self, 4)
    }
}
#[doc = "interrupt and status clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clrisr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clrisr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLRISRrs;
impl crate::RegisterSpec for CLRISRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clrisr::R`](R) reader structure"]
impl crate::Readable for CLRISRrs {}
#[doc = "`write(|w| ..)` method takes [`clrisr::W`](W) writer structure"]
impl crate::Writable for CLRISRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLRISR to value 0"]
impl crate::Resettable for CLRISRrs {
    const RESET_VALUE: u32 = 0;
}
