///Register `CFGR` reader
pub type R = crate::R<CFGRrs>;
///Register `CFGR` writer
pub type W = crate::W<CFGRrs>;
///Field `BKPRW` reader - Backup registers read/write protection offset BKPRW value must be from 0 to 32. Protection zone 1 is defined for backup registers from TAMP_BKP0R to TAMP_BKPxR (x = BKPRW-1, with BKPRW more or equal to 1). If BKPRW = 0: there is no protection zone 1. Refer to Figure 499: Backup registers protection zones. Note: If BKPRWPRIV is set, BKPRW\[7:0\] can be written only in privileged mode.
pub type BKPRW_R = crate::FieldReader;
///Field `BKPRW` writer - Backup registers read/write protection offset BKPRW value must be from 0 to 32. Protection zone 1 is defined for backup registers from TAMP_BKP0R to TAMP_BKPxR (x = BKPRW-1, with BKPRW more or equal to 1). If BKPRW = 0: there is no protection zone 1. Refer to Figure 499: Backup registers protection zones. Note: If BKPRWPRIV is set, BKPRW\[7:0\] can be written only in privileged mode.
pub type BKPRW_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `BKPW` reader - Backup registers write protection offset BKPW value must be from 0 to 32. Protection zone 2 is defined for backup registers from TAMP_BKPyR (y = BKPRW) to TAMP_BKPzR (z = BKPW-1, with BKPW > BKPRW): If BKPWSEC = 0 or if BKPWSEC UNDER OR EQUAL BKPRWSEC: there is no protection zone 2. Protection zone 3 is defined for backup registers from TAMP_BKPtR (t = BKPW if BKPWSEC more or equal to BKPRWSEC, else t = BKPRWSEC). If BKPWSEC = 32: there is no protection zone 3. Refer to Figure 499: Backup registers protection zones. Note: If BKPWPRIV is set, BKPRW\[7:0\] can be written only in privileged mode.
pub type BKPW_R = crate::FieldReader;
///Field `BKPW` writer - Backup registers write protection offset BKPW value must be from 0 to 32. Protection zone 2 is defined for backup registers from TAMP_BKPyR (y = BKPRW) to TAMP_BKPzR (z = BKPW-1, with BKPW > BKPRW): If BKPWSEC = 0 or if BKPWSEC UNDER OR EQUAL BKPRWSEC: there is no protection zone 2. Protection zone 3 is defined for backup registers from TAMP_BKPtR (t = BKPW if BKPWSEC more or equal to BKPRWSEC, else t = BKPRWSEC). If BKPWSEC = 32: there is no protection zone 3. Refer to Figure 499: Backup registers protection zones. Note: If BKPWPRIV is set, BKPRW\[7:0\] can be written only in privileged mode.
pub type BKPW_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `BHKLOCK` reader - Boot hardware key lock This bit can be read and can only be written to 1 by software. It is cleared by hardware together with the backup registers following a tamper detection event or when the readout protection (RDP) is disabled.
pub type BHKLOCK_R = crate::BitReader;
///Field `BHKLOCK` writer - Boot hardware key lock This bit can be read and can only be written to 1 by software. It is cleared by hardware together with the backup registers following a tamper detection event or when the readout protection (RDP) is disabled.
pub type BHKLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:7 - Backup registers read/write protection offset BKPRW value must be from 0 to 32. Protection zone 1 is defined for backup registers from TAMP_BKP0R to TAMP_BKPxR (x = BKPRW-1, with BKPRW more or equal to 1). If BKPRW = 0: there is no protection zone 1. Refer to Figure 499: Backup registers protection zones. Note: If BKPRWPRIV is set, BKPRW\[7:0\] can be written only in privileged mode.
    #[inline(always)]
    pub fn bkprw(&self) -> BKPRW_R {
        BKPRW_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 16:23 - Backup registers write protection offset BKPW value must be from 0 to 32. Protection zone 2 is defined for backup registers from TAMP_BKPyR (y = BKPRW) to TAMP_BKPzR (z = BKPW-1, with BKPW > BKPRW): If BKPWSEC = 0 or if BKPWSEC UNDER OR EQUAL BKPRWSEC: there is no protection zone 2. Protection zone 3 is defined for backup registers from TAMP_BKPtR (t = BKPW if BKPWSEC more or equal to BKPRWSEC, else t = BKPRWSEC). If BKPWSEC = 32: there is no protection zone 3. Refer to Figure 499: Backup registers protection zones. Note: If BKPWPRIV is set, BKPRW\[7:0\] can be written only in privileged mode.
    #[inline(always)]
    pub fn bkpw(&self) -> BKPW_R {
        BKPW_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bit 30 - Boot hardware key lock This bit can be read and can only be written to 1 by software. It is cleared by hardware together with the backup registers following a tamper detection event or when the readout protection (RDP) is disabled.
    #[inline(always)]
    pub fn bhklock(&self) -> BHKLOCK_R {
        BHKLOCK_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR")
            .field("bkprw", &self.bkprw())
            .field("bkpw", &self.bkpw())
            .field("bhklock", &self.bhklock())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Backup registers read/write protection offset BKPRW value must be from 0 to 32. Protection zone 1 is defined for backup registers from TAMP_BKP0R to TAMP_BKPxR (x = BKPRW-1, with BKPRW more or equal to 1). If BKPRW = 0: there is no protection zone 1. Refer to Figure 499: Backup registers protection zones. Note: If BKPRWPRIV is set, BKPRW\[7:0\] can be written only in privileged mode.
    #[inline(always)]
    pub fn bkprw(&mut self) -> BKPRW_W<'_, CFGRrs> {
        BKPRW_W::new(self, 0)
    }
    ///Bits 16:23 - Backup registers write protection offset BKPW value must be from 0 to 32. Protection zone 2 is defined for backup registers from TAMP_BKPyR (y = BKPRW) to TAMP_BKPzR (z = BKPW-1, with BKPW > BKPRW): If BKPWSEC = 0 or if BKPWSEC UNDER OR EQUAL BKPRWSEC: there is no protection zone 2. Protection zone 3 is defined for backup registers from TAMP_BKPtR (t = BKPW if BKPWSEC more or equal to BKPRWSEC, else t = BKPRWSEC). If BKPWSEC = 32: there is no protection zone 3. Refer to Figure 499: Backup registers protection zones. Note: If BKPWPRIV is set, BKPRW\[7:0\] can be written only in privileged mode.
    #[inline(always)]
    pub fn bkpw(&mut self) -> BKPW_W<'_, CFGRrs> {
        BKPW_W::new(self, 16)
    }
    ///Bit 30 - Boot hardware key lock This bit can be read and can only be written to 1 by software. It is cleared by hardware together with the backup registers following a tamper detection event or when the readout protection (RDP) is disabled.
    #[inline(always)]
    pub fn bhklock(&mut self) -> BHKLOCK_W<'_, CFGRrs> {
        BHKLOCK_W::new(self, 30)
    }
}
/**TAMP configuration register

You can [`read`](crate::Reg::read) this register and get [`cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#TAMP:CFGR)*/
pub struct CFGRrs;
impl crate::RegisterSpec for CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`cfgr::R`](R) reader structure
impl crate::Readable for CFGRrs {}
///`write(|w| ..)` method takes [`cfgr::W`](W) writer structure
impl crate::Writable for CFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFGR to value 0
impl crate::Resettable for CFGRrs {}
