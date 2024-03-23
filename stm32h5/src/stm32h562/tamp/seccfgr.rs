#[doc = "Register `SECCFGR` reader"]
pub type R = crate::R<SECCFGRrs>;
#[doc = "Register `SECCFGR` writer"]
pub type W = crate::W<SECCFGRrs>;
#[doc = "Field `BKPRWSEC` reader - Backup registers read/write protection offset Protection zone 1 is defined for backup registers from TAMP_BKP0R to TAMP_BKPxR (x = BKPRWSEC-1, from 0 to 128). if TZEN=1, these backup registers can be read and written only with secure access. If TZEN=0: the protection zone 1 can be read and written with non-secure access. If BKPRWSEC = 0: there is no protection zone 1. If BKPRWPRIV is set, BKPRWSEC\\[7:0\\]
can be written only in privileged mode."]
pub type BKPRWSEC_R = crate::FieldReader;
#[doc = "Field `BKPRWSEC` writer - Backup registers read/write protection offset Protection zone 1 is defined for backup registers from TAMP_BKP0R to TAMP_BKPxR (x = BKPRWSEC-1, from 0 to 128). if TZEN=1, these backup registers can be read and written only with secure access. If TZEN=0: the protection zone 1 can be read and written with non-secure access. If BKPRWSEC = 0: there is no protection zone 1. If BKPRWPRIV is set, BKPRWSEC\\[7:0\\]
can be written only in privileged mode."]
pub type BKPRWSEC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CNT1SEC` reader - Monotonic counter 1 secure protection"]
pub type CNT1SEC_R = crate::BitReader;
#[doc = "Field `CNT1SEC` writer - Monotonic counter 1 secure protection"]
pub type CNT1SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKPWSEC` reader - Backup registers write protection offset Protection zone 2 is defined for backup registers from TAMP_BKPyR (y = BKPRWSEC, from 0 to 128) to TAMP_BKPzR (z = BKPWSEC-1, from 0 to 128, BKPWSEC ≥ BKPRWSEC): if TZEN=1, these backup registers can be written only with secure access. They can be read with secure or non-secure access. Protection zone 3 defined for backup registers from TAMP_BKPtR (t = BKPWSEC, from 0 to 127). They can be read or written with secure or non-secure access. If TZEN=0: the protection zone 2 can be read and written with non-secure access. If BKPWSEC = 0 or if BKPWSEC ≤ BKPRWSEC: there is no protection zone 2. If BKPWPRIV is set, BKPRWSEC\\[7:0\\]
can be written only in privileged mode."]
pub type BKPWSEC_R = crate::FieldReader;
#[doc = "Field `BKPWSEC` writer - Backup registers write protection offset Protection zone 2 is defined for backup registers from TAMP_BKPyR (y = BKPRWSEC, from 0 to 128) to TAMP_BKPzR (z = BKPWSEC-1, from 0 to 128, BKPWSEC ≥ BKPRWSEC): if TZEN=1, these backup registers can be written only with secure access. They can be read with secure or non-secure access. Protection zone 3 defined for backup registers from TAMP_BKPtR (t = BKPWSEC, from 0 to 127). They can be read or written with secure or non-secure access. If TZEN=0: the protection zone 2 can be read and written with non-secure access. If BKPWSEC = 0 or if BKPWSEC ≤ BKPRWSEC: there is no protection zone 2. If BKPWPRIV is set, BKPRWSEC\\[7:0\\]
can be written only in privileged mode."]
pub type BKPWSEC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BHKLOCK` reader - Boot hardware key lock This bit can be read and can only be written to 1 by software. It is cleared by hardware together with the backup registers following a tamper detection event or when the readout protection (RDP) is disabled."]
pub type BHKLOCK_R = crate::BitReader;
#[doc = "Field `BHKLOCK` writer - Boot hardware key lock This bit can be read and can only be written to 1 by software. It is cleared by hardware together with the backup registers following a tamper detection event or when the readout protection (RDP) is disabled."]
pub type BHKLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMPSEC` reader - Tamper protection (excluding monotonic counters and backup registers) Note: Refer to for details on the read protection."]
pub type TAMPSEC_R = crate::BitReader;
#[doc = "Field `TAMPSEC` writer - Tamper protection (excluding monotonic counters and backup registers) Note: Refer to for details on the read protection."]
pub type TAMPSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Backup registers read/write protection offset Protection zone 1 is defined for backup registers from TAMP_BKP0R to TAMP_BKPxR (x = BKPRWSEC-1, from 0 to 128). if TZEN=1, these backup registers can be read and written only with secure access. If TZEN=0: the protection zone 1 can be read and written with non-secure access. If BKPRWSEC = 0: there is no protection zone 1. If BKPRWPRIV is set, BKPRWSEC\\[7:0\\]
can be written only in privileged mode."]
    #[inline(always)]
    pub fn bkprwsec(&self) -> BKPRWSEC_R {
        BKPRWSEC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 15 - Monotonic counter 1 secure protection"]
    #[inline(always)]
    pub fn cnt1sec(&self) -> CNT1SEC_R {
        CNT1SEC_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Backup registers write protection offset Protection zone 2 is defined for backup registers from TAMP_BKPyR (y = BKPRWSEC, from 0 to 128) to TAMP_BKPzR (z = BKPWSEC-1, from 0 to 128, BKPWSEC ≥ BKPRWSEC): if TZEN=1, these backup registers can be written only with secure access. They can be read with secure or non-secure access. Protection zone 3 defined for backup registers from TAMP_BKPtR (t = BKPWSEC, from 0 to 127). They can be read or written with secure or non-secure access. If TZEN=0: the protection zone 2 can be read and written with non-secure access. If BKPWSEC = 0 or if BKPWSEC ≤ BKPRWSEC: there is no protection zone 2. If BKPWPRIV is set, BKPRWSEC\\[7:0\\]
can be written only in privileged mode."]
    #[inline(always)]
    pub fn bkpwsec(&self) -> BKPWSEC_R {
        BKPWSEC_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 30 - Boot hardware key lock This bit can be read and can only be written to 1 by software. It is cleared by hardware together with the backup registers following a tamper detection event or when the readout protection (RDP) is disabled."]
    #[inline(always)]
    pub fn bhklock(&self) -> BHKLOCK_R {
        BHKLOCK_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Tamper protection (excluding monotonic counters and backup registers) Note: Refer to for details on the read protection."]
    #[inline(always)]
    pub fn tampsec(&self) -> TAMPSEC_R {
        TAMPSEC_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Backup registers read/write protection offset Protection zone 1 is defined for backup registers from TAMP_BKP0R to TAMP_BKPxR (x = BKPRWSEC-1, from 0 to 128). if TZEN=1, these backup registers can be read and written only with secure access. If TZEN=0: the protection zone 1 can be read and written with non-secure access. If BKPRWSEC = 0: there is no protection zone 1. If BKPRWPRIV is set, BKPRWSEC\\[7:0\\]
can be written only in privileged mode."]
    #[inline(always)]
    #[must_use]
    pub fn bkprwsec(&mut self) -> BKPRWSEC_W<SECCFGRrs> {
        BKPRWSEC_W::new(self, 0)
    }
    #[doc = "Bit 15 - Monotonic counter 1 secure protection"]
    #[inline(always)]
    #[must_use]
    pub fn cnt1sec(&mut self) -> CNT1SEC_W<SECCFGRrs> {
        CNT1SEC_W::new(self, 15)
    }
    #[doc = "Bits 16:23 - Backup registers write protection offset Protection zone 2 is defined for backup registers from TAMP_BKPyR (y = BKPRWSEC, from 0 to 128) to TAMP_BKPzR (z = BKPWSEC-1, from 0 to 128, BKPWSEC ≥ BKPRWSEC): if TZEN=1, these backup registers can be written only with secure access. They can be read with secure or non-secure access. Protection zone 3 defined for backup registers from TAMP_BKPtR (t = BKPWSEC, from 0 to 127). They can be read or written with secure or non-secure access. If TZEN=0: the protection zone 2 can be read and written with non-secure access. If BKPWSEC = 0 or if BKPWSEC ≤ BKPRWSEC: there is no protection zone 2. If BKPWPRIV is set, BKPRWSEC\\[7:0\\]
can be written only in privileged mode."]
    #[inline(always)]
    #[must_use]
    pub fn bkpwsec(&mut self) -> BKPWSEC_W<SECCFGRrs> {
        BKPWSEC_W::new(self, 16)
    }
    #[doc = "Bit 30 - Boot hardware key lock This bit can be read and can only be written to 1 by software. It is cleared by hardware together with the backup registers following a tamper detection event or when the readout protection (RDP) is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn bhklock(&mut self) -> BHKLOCK_W<SECCFGRrs> {
        BHKLOCK_W::new(self, 30)
    }
    #[doc = "Bit 31 - Tamper protection (excluding monotonic counters and backup registers) Note: Refer to for details on the read protection."]
    #[inline(always)]
    #[must_use]
    pub fn tampsec(&mut self) -> TAMPSEC_W<SECCFGRrs> {
        TAMPSEC_W::new(self, 31)
    }
}
#[doc = "TAMP secure mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seccfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seccfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SECCFGRrs;
impl crate::RegisterSpec for SECCFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seccfgr::R`](R) reader structure"]
impl crate::Readable for SECCFGRrs {}
#[doc = "`write(|w| ..)` method takes [`seccfgr::W`](W) writer structure"]
impl crate::Writable for SECCFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SECCFGR to value 0"]
impl crate::Resettable for SECCFGRrs {
    const RESET_VALUE: u32 = 0;
}
