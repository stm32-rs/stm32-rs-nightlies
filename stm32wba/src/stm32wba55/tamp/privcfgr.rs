///Register `PRIVCFGR` reader
pub type R = crate::R<PRIVCFGRrs>;
///Register `PRIVCFGR` writer
pub type W = crate::W<PRIVCFGRrs>;
///Field `CNT1PRIV` reader - Monotonic counter 1 privilege protection
pub type CNT1PRIV_R = crate::BitReader;
///Field `CNT1PRIV` writer - Monotonic counter 1 privilege protection
pub type CNT1PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BKPRWPRIV` reader - Backup registers zone 1 privilege protection
pub type BKPRWPRIV_R = crate::BitReader;
///Field `BKPRWPRIV` writer - Backup registers zone 1 privilege protection
pub type BKPRWPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BKPWPRIV` reader - Backup registers zone 2 privilege protection
pub type BKPWPRIV_R = crate::BitReader;
///Field `BKPWPRIV` writer - Backup registers zone 2 privilege protection
pub type BKPWPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMPPRIV` reader - Tamper privilege protection (excluding backup registers) Note: Refer to Section75.3.7: TAMP privilege protection modes for details on the read protection.
pub type TAMPPRIV_R = crate::BitReader;
///Field `TAMPPRIV` writer - Tamper privilege protection (excluding backup registers) Note: Refer to Section75.3.7: TAMP privilege protection modes for details on the read protection.
pub type TAMPPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 15 - Monotonic counter 1 privilege protection
    #[inline(always)]
    pub fn cnt1priv(&self) -> CNT1PRIV_R {
        CNT1PRIV_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 29 - Backup registers zone 1 privilege protection
    #[inline(always)]
    pub fn bkprwpriv(&self) -> BKPRWPRIV_R {
        BKPRWPRIV_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Backup registers zone 2 privilege protection
    #[inline(always)]
    pub fn bkpwpriv(&self) -> BKPWPRIV_R {
        BKPWPRIV_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Tamper privilege protection (excluding backup registers) Note: Refer to Section75.3.7: TAMP privilege protection modes for details on the read protection.
    #[inline(always)]
    pub fn tamppriv(&self) -> TAMPPRIV_R {
        TAMPPRIV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRIVCFGR")
            .field("cnt1priv", &self.cnt1priv())
            .field("bkprwpriv", &self.bkprwpriv())
            .field("bkpwpriv", &self.bkpwpriv())
            .field("tamppriv", &self.tamppriv())
            .finish()
    }
}
impl W {
    ///Bit 15 - Monotonic counter 1 privilege protection
    #[inline(always)]
    pub fn cnt1priv(&mut self) -> CNT1PRIV_W<'_, PRIVCFGRrs> {
        CNT1PRIV_W::new(self, 15)
    }
    ///Bit 29 - Backup registers zone 1 privilege protection
    #[inline(always)]
    pub fn bkprwpriv(&mut self) -> BKPRWPRIV_W<'_, PRIVCFGRrs> {
        BKPRWPRIV_W::new(self, 29)
    }
    ///Bit 30 - Backup registers zone 2 privilege protection
    #[inline(always)]
    pub fn bkpwpriv(&mut self) -> BKPWPRIV_W<'_, PRIVCFGRrs> {
        BKPWPRIV_W::new(self, 30)
    }
    ///Bit 31 - Tamper privilege protection (excluding backup registers) Note: Refer to Section75.3.7: TAMP privilege protection modes for details on the read protection.
    #[inline(always)]
    pub fn tamppriv(&mut self) -> TAMPPRIV_W<'_, PRIVCFGRrs> {
        TAMPPRIV_W::new(self, 31)
    }
}
/**TAMP privilege configuration register

You can [`read`](crate::Reg::read) this register and get [`privcfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#TAMP:PRIVCFGR)*/
pub struct PRIVCFGRrs;
impl crate::RegisterSpec for PRIVCFGRrs {
    type Ux = u32;
}
///`read()` method returns [`privcfgr::R`](R) reader structure
impl crate::Readable for PRIVCFGRrs {}
///`write(|w| ..)` method takes [`privcfgr::W`](W) writer structure
impl crate::Writable for PRIVCFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PRIVCFGR to value 0
impl crate::Resettable for PRIVCFGRrs {}
