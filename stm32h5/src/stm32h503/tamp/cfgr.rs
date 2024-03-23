#[doc = "Register `CFGR` reader"]
pub type R = crate::R<CFGRrs>;
#[doc = "Register `CFGR` writer"]
pub type W = crate::W<CFGRrs>;
#[doc = "Field `BKPRW` reader - Backup registers read/write protection offset Protection zone 1 is defined for backup registers from TAMP_BKP0R to TAMP_BKPxR (x = BKPRW-1, from 0 to 128). Note: If BKPRW = 0: there is no protection zone 1. Note: If BKPRWPRIV is set, BKPRW\\[7:0\\]
can be written only in privileged mode."]
pub type BKPRW_R = crate::FieldReader;
#[doc = "Field `BKPRW` writer - Backup registers read/write protection offset Protection zone 1 is defined for backup registers from TAMP_BKP0R to TAMP_BKPxR (x = BKPRW-1, from 0 to 128). Note: If BKPRW = 0: there is no protection zone 1. Note: If BKPRWPRIV is set, BKPRW\\[7:0\\]
can be written only in privileged mode."]
pub type BKPRW_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BKPW` reader - Backup registers write protection offset Protection zone 2 is defined for backup registers from TAMP_BKPyR (y = BKPRW, from 0 to 128) to TAMP_BKPzR (z = BKPW-1, from 0 to 128, BKPW ≥ BKPRW): Protection zone 3 defined for backup registers from TAMP_BKPtR (t = BKPW, from 0 to 127). Note: If BKPW = 0 or if BKPW ≤ BKPRW: there is no protection zone 2. Note: If BKPWPRIV is set, BKPRW\\[7:0\\]
can be written only in privileged mode."]
pub type BKPW_R = crate::FieldReader;
#[doc = "Field `BKPW` writer - Backup registers write protection offset Protection zone 2 is defined for backup registers from TAMP_BKPyR (y = BKPRW, from 0 to 128) to TAMP_BKPzR (z = BKPW-1, from 0 to 128, BKPW ≥ BKPRW): Protection zone 3 defined for backup registers from TAMP_BKPtR (t = BKPW, from 0 to 127). Note: If BKPW = 0 or if BKPW ≤ BKPRW: there is no protection zone 2. Note: If BKPWPRIV is set, BKPRW\\[7:0\\]
can be written only in privileged mode."]
pub type BKPW_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Backup registers read/write protection offset Protection zone 1 is defined for backup registers from TAMP_BKP0R to TAMP_BKPxR (x = BKPRW-1, from 0 to 128). Note: If BKPRW = 0: there is no protection zone 1. Note: If BKPRWPRIV is set, BKPRW\\[7:0\\]
can be written only in privileged mode."]
    #[inline(always)]
    pub fn bkprw(&self) -> BKPRW_R {
        BKPRW_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Backup registers write protection offset Protection zone 2 is defined for backup registers from TAMP_BKPyR (y = BKPRW, from 0 to 128) to TAMP_BKPzR (z = BKPW-1, from 0 to 128, BKPW ≥ BKPRW): Protection zone 3 defined for backup registers from TAMP_BKPtR (t = BKPW, from 0 to 127). Note: If BKPW = 0 or if BKPW ≤ BKPRW: there is no protection zone 2. Note: If BKPWPRIV is set, BKPRW\\[7:0\\]
can be written only in privileged mode."]
    #[inline(always)]
    pub fn bkpw(&self) -> BKPW_R {
        BKPW_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Backup registers read/write protection offset Protection zone 1 is defined for backup registers from TAMP_BKP0R to TAMP_BKPxR (x = BKPRW-1, from 0 to 128). Note: If BKPRW = 0: there is no protection zone 1. Note: If BKPRWPRIV is set, BKPRW\\[7:0\\]
can be written only in privileged mode."]
    #[inline(always)]
    #[must_use]
    pub fn bkprw(&mut self) -> BKPRW_W<CFGRrs> {
        BKPRW_W::new(self, 0)
    }
    #[doc = "Bits 16:23 - Backup registers write protection offset Protection zone 2 is defined for backup registers from TAMP_BKPyR (y = BKPRW, from 0 to 128) to TAMP_BKPzR (z = BKPW-1, from 0 to 128, BKPW ≥ BKPRW): Protection zone 3 defined for backup registers from TAMP_BKPtR (t = BKPW, from 0 to 127). Note: If BKPW = 0 or if BKPW ≤ BKPRW: there is no protection zone 2. Note: If BKPWPRIV is set, BKPRW\\[7:0\\]
can be written only in privileged mode."]
    #[inline(always)]
    #[must_use]
    pub fn bkpw(&mut self) -> BKPW_W<CFGRrs> {
        BKPW_W::new(self, 16)
    }
}
#[doc = "TAMP configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
