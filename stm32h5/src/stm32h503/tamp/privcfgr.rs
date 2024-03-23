#[doc = "Register `PRIVCFGR` reader"]
pub type R = crate::R<PRIVCFGRrs>;
#[doc = "Register `PRIVCFGR` writer"]
pub type W = crate::W<PRIVCFGRrs>;
#[doc = "Field `CNT1PRIV` reader - Monotonic counter 1 privilege protection"]
pub type CNT1PRIV_R = crate::BitReader;
#[doc = "Field `CNT1PRIV` writer - Monotonic counter 1 privilege protection"]
pub type CNT1PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKPRWPRIV` reader - Backup registers zone 1 privilege protection"]
pub type BKPRWPRIV_R = crate::BitReader;
#[doc = "Field `BKPRWPRIV` writer - Backup registers zone 1 privilege protection"]
pub type BKPRWPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKPWPRIV` reader - Backup registers zone 2 privilege protection"]
pub type BKPWPRIV_R = crate::BitReader;
#[doc = "Field `BKPWPRIV` writer - Backup registers zone 2 privilege protection"]
pub type BKPWPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMPPRIV` reader - Tamper privilege protection (excluding backup registers) Note: Refer to Section 32.3.6: TAMP privilege protection modes for details on the read protection."]
pub type TAMPPRIV_R = crate::BitReader;
#[doc = "Field `TAMPPRIV` writer - Tamper privilege protection (excluding backup registers) Note: Refer to Section 32.3.6: TAMP privilege protection modes for details on the read protection."]
pub type TAMPPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 15 - Monotonic counter 1 privilege protection"]
    #[inline(always)]
    pub fn cnt1priv(&self) -> CNT1PRIV_R {
        CNT1PRIV_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 29 - Backup registers zone 1 privilege protection"]
    #[inline(always)]
    pub fn bkprwpriv(&self) -> BKPRWPRIV_R {
        BKPRWPRIV_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Backup registers zone 2 privilege protection"]
    #[inline(always)]
    pub fn bkpwpriv(&self) -> BKPWPRIV_R {
        BKPWPRIV_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Tamper privilege protection (excluding backup registers) Note: Refer to Section 32.3.6: TAMP privilege protection modes for details on the read protection."]
    #[inline(always)]
    pub fn tamppriv(&self) -> TAMPPRIV_R {
        TAMPPRIV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - Monotonic counter 1 privilege protection"]
    #[inline(always)]
    #[must_use]
    pub fn cnt1priv(&mut self) -> CNT1PRIV_W<PRIVCFGRrs> {
        CNT1PRIV_W::new(self, 15)
    }
    #[doc = "Bit 29 - Backup registers zone 1 privilege protection"]
    #[inline(always)]
    #[must_use]
    pub fn bkprwpriv(&mut self) -> BKPRWPRIV_W<PRIVCFGRrs> {
        BKPRWPRIV_W::new(self, 29)
    }
    #[doc = "Bit 30 - Backup registers zone 2 privilege protection"]
    #[inline(always)]
    #[must_use]
    pub fn bkpwpriv(&mut self) -> BKPWPRIV_W<PRIVCFGRrs> {
        BKPWPRIV_W::new(self, 30)
    }
    #[doc = "Bit 31 - Tamper privilege protection (excluding backup registers) Note: Refer to Section 32.3.6: TAMP privilege protection modes for details on the read protection."]
    #[inline(always)]
    #[must_use]
    pub fn tamppriv(&mut self) -> TAMPPRIV_W<PRIVCFGRrs> {
        TAMPPRIV_W::new(self, 31)
    }
}
#[doc = "TAMP privilege configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`privcfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`privcfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
