///Register `CFGR` reader
pub type R = crate::R<CFGRrs>;
///Register `CFGR` writer
pub type W = crate::W<CFGRrs>;
///Field `BKPRW` reader - Backup registers read/write protection offset Protection zone 1 is defined for backup registers from TAMP_BKP0R to TAMP_BKPxR (x = BKPRW-1, from 0 to 128). Note: If BKPRW = 0: there is no protection zone 1. Note: If BKPRWPRIV is set, BKPRW\[7:0\] can be written only in privileged mode.
pub type BKPRW_R = crate::FieldReader;
///Field `BKPRW` writer - Backup registers read/write protection offset Protection zone 1 is defined for backup registers from TAMP_BKP0R to TAMP_BKPxR (x = BKPRW-1, from 0 to 128). Note: If BKPRW = 0: there is no protection zone 1. Note: If BKPRWPRIV is set, BKPRW\[7:0\] can be written only in privileged mode.
pub type BKPRW_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `BKPW` reader - Backup registers write protection offset Protection zone 2 is defined for backup registers from TAMP_BKPyR (y = BKPRW, from 0 to 128) to TAMP_BKPzR (z = BKPW-1, from 0 to 128, BKPW greater than or equal BKPRW): Protection zone 3 defined for backup registers from TAMP_BKPtR (t = BKPW, from 0 to 127)..
pub type BKPW_R = crate::FieldReader;
///Field `BKPW` writer - Backup registers write protection offset Protection zone 2 is defined for backup registers from TAMP_BKPyR (y = BKPRW, from 0 to 128) to TAMP_BKPzR (z = BKPW-1, from 0 to 128, BKPW greater than or equal BKPRW): Protection zone 3 defined for backup registers from TAMP_BKPtR (t = BKPW, from 0 to 127)..
pub type BKPW_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Backup registers read/write protection offset Protection zone 1 is defined for backup registers from TAMP_BKP0R to TAMP_BKPxR (x = BKPRW-1, from 0 to 128). Note: If BKPRW = 0: there is no protection zone 1. Note: If BKPRWPRIV is set, BKPRW\[7:0\] can be written only in privileged mode.
    #[inline(always)]
    pub fn bkprw(&self) -> BKPRW_R {
        BKPRW_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 16:23 - Backup registers write protection offset Protection zone 2 is defined for backup registers from TAMP_BKPyR (y = BKPRW, from 0 to 128) to TAMP_BKPzR (z = BKPW-1, from 0 to 128, BKPW greater than or equal BKPRW): Protection zone 3 defined for backup registers from TAMP_BKPtR (t = BKPW, from 0 to 127)..
    #[inline(always)]
    pub fn bkpw(&self) -> BKPW_R {
        BKPW_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR")
            .field("bkprw", &self.bkprw())
            .field("bkpw", &self.bkpw())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Backup registers read/write protection offset Protection zone 1 is defined for backup registers from TAMP_BKP0R to TAMP_BKPxR (x = BKPRW-1, from 0 to 128). Note: If BKPRW = 0: there is no protection zone 1. Note: If BKPRWPRIV is set, BKPRW\[7:0\] can be written only in privileged mode.
    #[inline(always)]
    pub fn bkprw(&mut self) -> BKPRW_W<'_, CFGRrs> {
        BKPRW_W::new(self, 0)
    }
    ///Bits 16:23 - Backup registers write protection offset Protection zone 2 is defined for backup registers from TAMP_BKPyR (y = BKPRW, from 0 to 128) to TAMP_BKPzR (z = BKPW-1, from 0 to 128, BKPW greater than or equal BKPRW): Protection zone 3 defined for backup registers from TAMP_BKPtR (t = BKPW, from 0 to 127)..
    #[inline(always)]
    pub fn bkpw(&mut self) -> BKPW_W<'_, CFGRrs> {
        BKPW_W::new(self, 16)
    }
}
/**TAMP configuration register

You can [`read`](crate::Reg::read) this register and get [`cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#TAMP:CFGR)*/
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
