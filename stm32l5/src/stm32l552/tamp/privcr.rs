#[doc = "Register `PRIVCR` reader"]
pub type R = crate::R<PRIVCRrs>;
#[doc = "Register `PRIVCR` writer"]
pub type W = crate::W<PRIVCRrs>;
#[doc = "Field `BKPRWPRIV` reader - Backup registers zone 1 privilege protection"]
pub type BKPRWPRIV_R = crate::BitReader;
#[doc = "Field `BKPRWPRIV` writer - Backup registers zone 1 privilege protection"]
pub type BKPRWPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKPWPRIV` reader - Backup registers zone 2 privilege protection"]
pub type BKPWPRIV_R = crate::BitReader;
#[doc = "Field `BKPWPRIV` writer - Backup registers zone 2 privilege protection"]
pub type BKPWPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMPPRIV` reader - Tamper privilege protection"]
pub type TAMPPRIV_R = crate::BitReader;
#[doc = "Field `TAMPPRIV` writer - Tamper privilege protection"]
pub type TAMPPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
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
    #[doc = "Bit 31 - Tamper privilege protection"]
    #[inline(always)]
    pub fn tamppriv(&self) -> TAMPPRIV_R {
        TAMPPRIV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 29 - Backup registers zone 1 privilege protection"]
    #[inline(always)]
    #[must_use]
    pub fn bkprwpriv(&mut self) -> BKPRWPRIV_W<PRIVCRrs> {
        BKPRWPRIV_W::new(self, 29)
    }
    #[doc = "Bit 30 - Backup registers zone 2 privilege protection"]
    #[inline(always)]
    #[must_use]
    pub fn bkpwpriv(&mut self) -> BKPWPRIV_W<PRIVCRrs> {
        BKPWPRIV_W::new(self, 30)
    }
    #[doc = "Bit 31 - Tamper privilege protection"]
    #[inline(always)]
    #[must_use]
    pub fn tamppriv(&mut self) -> TAMPPRIV_W<PRIVCRrs> {
        TAMPPRIV_W::new(self, 31)
    }
}
#[doc = "TAMP privilege mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`privcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`privcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRIVCRrs;
impl crate::RegisterSpec for PRIVCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`privcr::R`](R) reader structure"]
impl crate::Readable for PRIVCRrs {}
#[doc = "`write(|w| ..)` method takes [`privcr::W`](W) writer structure"]
impl crate::Writable for PRIVCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRIVCR to value 0"]
impl crate::Resettable for PRIVCRrs {
    const RESET_VALUE: u32 = 0;
}
