#[doc = "Register `CFGR` reader"]
pub type R = crate::R<CFGRrs>;
#[doc = "Register `CFGR` writer"]
pub type W = crate::W<CFGRrs>;
#[doc = "Field `SW` reader - System clock switch"]
pub type SW_R = crate::FieldReader;
#[doc = "Field `SW` writer - System clock switch"]
pub type SW_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SWS` reader - System clock switch status"]
pub type SWS_R = crate::FieldReader;
#[doc = "Field `HPRE` reader - AHB prescaler"]
pub type HPRE_R = crate::FieldReader;
#[doc = "Field `HPRE` writer - AHB prescaler"]
pub type HPRE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PPRE` reader - APB prescaler"]
pub type PPRE_R = crate::FieldReader;
#[doc = "Field `PPRE` writer - APB prescaler"]
pub type PPRE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MCOSEL` reader - Microcontroller clock output"]
pub type MCOSEL_R = crate::FieldReader;
#[doc = "Field `MCOSEL` writer - Microcontroller clock output"]
pub type MCOSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MCOPRE` reader - Microcontroller clock output prescaler"]
pub type MCOPRE_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - System clock switch"]
    #[inline(always)]
    pub fn sw(&self) -> SW_R {
        SW_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - System clock switch status"]
    #[inline(always)]
    pub fn sws(&self) -> SWS_R {
        SWS_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 8:11 - AHB prescaler"]
    #[inline(always)]
    pub fn hpre(&self) -> HPRE_R {
        HPRE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - APB prescaler"]
    #[inline(always)]
    pub fn ppre(&self) -> PPRE_R {
        PPRE_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Microcontroller clock output"]
    #[inline(always)]
    pub fn mcosel(&self) -> MCOSEL_R {
        MCOSEL_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - Microcontroller clock output prescaler"]
    #[inline(always)]
    pub fn mcopre(&self) -> MCOPRE_R {
        MCOPRE_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - System clock switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw(&mut self) -> SW_W<CFGRrs> {
        SW_W::new(self, 0)
    }
    #[doc = "Bits 8:11 - AHB prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn hpre(&mut self) -> HPRE_W<CFGRrs> {
        HPRE_W::new(self, 8)
    }
    #[doc = "Bits 12:14 - APB prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn ppre(&mut self) -> PPRE_W<CFGRrs> {
        PPRE_W::new(self, 12)
    }
    #[doc = "Bits 24:26 - Microcontroller clock output"]
    #[inline(always)]
    #[must_use]
    pub fn mcosel(&mut self) -> MCOSEL_W<CFGRrs> {
        MCOSEL_W::new(self, 24)
    }
}
#[doc = "Clock configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGRrs;
impl crate::RegisterSpec for CFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr::R`](R) reader structure"]
impl crate::Readable for CFGRrs {}
#[doc = "`write(|w| ..)` method takes [`cfgr::W`](W) writer structure"]
impl crate::Writable for CFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFGR to value 0"]
impl crate::Resettable for CFGRrs {
    const RESET_VALUE: u32 = 0;
}
