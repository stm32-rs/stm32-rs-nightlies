#[doc = "Register `FLTDR` reader"]
pub type R = crate::R<FLTDRrs>;
#[doc = "Register `FLTDR` writer"]
pub type W = crate::W<FLTDRrs>;
#[doc = "Field `FLT1EN` reader - Fault 1 enable"]
pub type FLT1EN_R = crate::BitReader;
#[doc = "Field `FLT1EN` writer - Fault 1 enable"]
pub type FLT1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT2EN` reader - Fault 2 enable"]
pub type FLT2EN_R = crate::BitReader;
#[doc = "Field `FLT2EN` writer - Fault 2 enable"]
pub type FLT2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT3EN` reader - Fault 3 enable"]
pub type FLT3EN_R = crate::BitReader;
#[doc = "Field `FLT3EN` writer - Fault 3 enable"]
pub type FLT3EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT4EN` reader - Fault 4 enable"]
pub type FLT4EN_R = crate::BitReader;
#[doc = "Field `FLT4EN` writer - Fault 4 enable"]
pub type FLT4EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT5EN` reader - Fault 5 enable"]
pub type FLT5EN_R = crate::BitReader;
#[doc = "Field `FLT5EN` writer - Fault 5 enable"]
pub type FLT5EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT6EN` reader - Fault 6 enable"]
pub type FLT6EN_R = crate::BitReader;
#[doc = "Field `FLT6EN` writer - Fault 6 enable"]
pub type FLT6EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLTLCK` reader - Fault sources Lock"]
pub type FLTLCK_R = crate::BitReader;
#[doc = "Field `FLTLCK` writer - Fault sources Lock"]
pub type FLTLCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Fault 1 enable"]
    #[inline(always)]
    pub fn flt1en(&self) -> FLT1EN_R {
        FLT1EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Fault 2 enable"]
    #[inline(always)]
    pub fn flt2en(&self) -> FLT2EN_R {
        FLT2EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Fault 3 enable"]
    #[inline(always)]
    pub fn flt3en(&self) -> FLT3EN_R {
        FLT3EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Fault 4 enable"]
    #[inline(always)]
    pub fn flt4en(&self) -> FLT4EN_R {
        FLT4EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Fault 5 enable"]
    #[inline(always)]
    pub fn flt5en(&self) -> FLT5EN_R {
        FLT5EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Fault 6 enable"]
    #[inline(always)]
    pub fn flt6en(&self) -> FLT6EN_R {
        FLT6EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 31 - Fault sources Lock"]
    #[inline(always)]
    pub fn fltlck(&self) -> FLTLCK_R {
        FLTLCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fault 1 enable"]
    #[inline(always)]
    #[must_use]
    pub fn flt1en(&mut self) -> FLT1EN_W<FLTDRrs> {
        FLT1EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Fault 2 enable"]
    #[inline(always)]
    #[must_use]
    pub fn flt2en(&mut self) -> FLT2EN_W<FLTDRrs> {
        FLT2EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Fault 3 enable"]
    #[inline(always)]
    #[must_use]
    pub fn flt3en(&mut self) -> FLT3EN_W<FLTDRrs> {
        FLT3EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Fault 4 enable"]
    #[inline(always)]
    #[must_use]
    pub fn flt4en(&mut self) -> FLT4EN_W<FLTDRrs> {
        FLT4EN_W::new(self, 3)
    }
    #[doc = "Bit 4 - Fault 5 enable"]
    #[inline(always)]
    #[must_use]
    pub fn flt5en(&mut self) -> FLT5EN_W<FLTDRrs> {
        FLT5EN_W::new(self, 4)
    }
    #[doc = "Bit 5 - Fault 6 enable"]
    #[inline(always)]
    #[must_use]
    pub fn flt6en(&mut self) -> FLT6EN_W<FLTDRrs> {
        FLT6EN_W::new(self, 5)
    }
    #[doc = "Bit 31 - Fault sources Lock"]
    #[inline(always)]
    #[must_use]
    pub fn fltlck(&mut self) -> FLTLCK_W<FLTDRrs> {
        FLTLCK_W::new(self, 31)
    }
}
#[doc = "Timerx Fault Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fltdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fltdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLTDRrs;
impl crate::RegisterSpec for FLTDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fltdr::R`](R) reader structure"]
impl crate::Readable for FLTDRrs {}
#[doc = "`write(|w| ..)` method takes [`fltdr::W`](W) writer structure"]
impl crate::Writable for FLTDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLTDR to value 0"]
impl crate::Resettable for FLTDRrs {
    const RESET_VALUE: u32 = 0;
}
