#[doc = "Register `PRIVCFGR` reader"]
pub type R = crate::R<PRIVCFGRrs>;
#[doc = "Register `PRIVCFGR` writer"]
pub type W = crate::W<PRIVCFGRrs>;
#[doc = "Field `ALRAPRIV` reader - Alarm A and SSR underflow privilege protection"]
pub type ALRAPRIV_R = crate::BitReader;
#[doc = "Field `ALRAPRIV` writer - Alarm A and SSR underflow privilege protection"]
pub type ALRAPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALRBPRIV` reader - Alarm B privilege protection"]
pub type ALRBPRIV_R = crate::BitReader;
#[doc = "Field `ALRBPRIV` writer - Alarm B privilege protection"]
pub type ALRBPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUTPRIV` reader - Wakeup timer privilege protection"]
pub type WUTPRIV_R = crate::BitReader;
#[doc = "Field `WUTPRIV` writer - Wakeup timer privilege protection"]
pub type WUTPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSPRIV` reader - Timestamp privilege protection"]
pub type TSPRIV_R = crate::BitReader;
#[doc = "Field `TSPRIV` writer - Timestamp privilege protection"]
pub type TSPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALPRIV` reader - Shift register, Delight saving, calibration and reference clock privilege protection"]
pub type CALPRIV_R = crate::BitReader;
#[doc = "Field `CALPRIV` writer - Shift register, Delight saving, calibration and reference clock privilege protection"]
pub type CALPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INITPRIV` reader - Initialization privilege protection"]
pub type INITPRIV_R = crate::BitReader;
#[doc = "Field `INITPRIV` writer - Initialization privilege protection"]
pub type INITPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV` reader - RTC privilege protection"]
pub type PRIV_R = crate::BitReader;
#[doc = "Field `PRIV` writer - RTC privilege protection"]
pub type PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Alarm A and SSR underflow privilege protection"]
    #[inline(always)]
    pub fn alrapriv(&self) -> ALRAPRIV_R {
        ALRAPRIV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Alarm B privilege protection"]
    #[inline(always)]
    pub fn alrbpriv(&self) -> ALRBPRIV_R {
        ALRBPRIV_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wakeup timer privilege protection"]
    #[inline(always)]
    pub fn wutpriv(&self) -> WUTPRIV_R {
        WUTPRIV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timestamp privilege protection"]
    #[inline(always)]
    pub fn tspriv(&self) -> TSPRIV_R {
        TSPRIV_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 13 - Shift register, Delight saving, calibration and reference clock privilege protection"]
    #[inline(always)]
    pub fn calpriv(&self) -> CALPRIV_R {
        CALPRIV_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Initialization privilege protection"]
    #[inline(always)]
    pub fn initpriv(&self) -> INITPRIV_R {
        INITPRIV_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - RTC privilege protection"]
    #[inline(always)]
    pub fn priv_(&self) -> PRIV_R {
        PRIV_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Alarm A and SSR underflow privilege protection"]
    #[inline(always)]
    #[must_use]
    pub fn alrapriv(&mut self) -> ALRAPRIV_W<PRIVCFGRrs> {
        ALRAPRIV_W::new(self, 0)
    }
    #[doc = "Bit 1 - Alarm B privilege protection"]
    #[inline(always)]
    #[must_use]
    pub fn alrbpriv(&mut self) -> ALRBPRIV_W<PRIVCFGRrs> {
        ALRBPRIV_W::new(self, 1)
    }
    #[doc = "Bit 2 - Wakeup timer privilege protection"]
    #[inline(always)]
    #[must_use]
    pub fn wutpriv(&mut self) -> WUTPRIV_W<PRIVCFGRrs> {
        WUTPRIV_W::new(self, 2)
    }
    #[doc = "Bit 3 - Timestamp privilege protection"]
    #[inline(always)]
    #[must_use]
    pub fn tspriv(&mut self) -> TSPRIV_W<PRIVCFGRrs> {
        TSPRIV_W::new(self, 3)
    }
    #[doc = "Bit 13 - Shift register, Delight saving, calibration and reference clock privilege protection"]
    #[inline(always)]
    #[must_use]
    pub fn calpriv(&mut self) -> CALPRIV_W<PRIVCFGRrs> {
        CALPRIV_W::new(self, 13)
    }
    #[doc = "Bit 14 - Initialization privilege protection"]
    #[inline(always)]
    #[must_use]
    pub fn initpriv(&mut self) -> INITPRIV_W<PRIVCFGRrs> {
        INITPRIV_W::new(self, 14)
    }
    #[doc = "Bit 15 - RTC privilege protection"]
    #[inline(always)]
    #[must_use]
    pub fn priv_(&mut self) -> PRIV_W<PRIVCFGRrs> {
        PRIV_W::new(self, 15)
    }
}
#[doc = "RTC privilege mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`privcfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`privcfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRIVCFGRrs;
impl crate::RegisterSpec for PRIVCFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`privcfgr::R`](R) reader structure"]
impl crate::Readable for PRIVCFGRrs {}
#[doc = "`write(|w| ..)` method takes [`privcfgr::W`](W) writer structure"]
impl crate::Writable for PRIVCFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRIVCFGR to value 0"]
impl crate::Resettable for PRIVCFGRrs {
    const RESET_VALUE: u32 = 0;
}
