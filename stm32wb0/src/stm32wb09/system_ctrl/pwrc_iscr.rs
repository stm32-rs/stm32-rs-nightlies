///Register `PWRC_ISCR` reader
pub type R = crate::R<PWRC_ISCRrs>;
///Register `PWRC_ISCR` writer
pub type W = crate::W<PWRC_ISCRrs>;
///Field `BORH_ISC` reader - BORH_ISC: BORH interrupt status. 0: no pending interrupt. 1: voltage went under BORH threshold / interrupt occurred (if enabled). Cleared by writing 1 in the bit.
pub type BORH_ISC_R = crate::BitReader;
///Field `BORH_ISC` writer - BORH_ISC: BORH interrupt status. 0: no pending interrupt. 1: voltage went under BORH threshold / interrupt occurred (if enabled). Cleared by writing 1 in the bit.
pub type BORH_ISC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PVD_ISC` reader - PVD_ISC: Programmable Voltage Detector status. 0: no pending interrupt. 1: voltage went under programmed threshold / interrupt occurred (if enabled). Cleared by writing 1 in the bit.
pub type PVD_ISC_R = crate::BitReader;
///Field `PVD_ISC` writer - PVD_ISC: Programmable Voltage Detector status. 0: no pending interrupt. 1: voltage went under programmed threshold / interrupt occurred (if enabled). Cleared by writing 1 in the bit.
pub type PVD_ISC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WKUP_ISC` reader - WKUP_ISC: Indicates the Power Controller receives a Wakeup event. 0: no pending interrupt. 1: Wakeup event on PWRC occurred / interrupt occurred (if enabled). Cleared by writing 1 in the bit. This flag will be read at 1 if a wakeup event arrives so close to the low power mode entry requests that the PWRC aborts before shutting down the system.
pub type WKUP_ISC_R = crate::BitReader;
///Field `WKUP_ISC` writer - WKUP_ISC: Indicates the Power Controller receives a Wakeup event. 0: no pending interrupt. 1: Wakeup event on PWRC occurred / interrupt occurred (if enabled). Cleared by writing 1 in the bit. This flag will be read at 1 if a wakeup event arrives so close to the low power mode entry requests that the PWRC aborts before shutting down the system.
pub type WKUP_ISC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - BORH_ISC: BORH interrupt status. 0: no pending interrupt. 1: voltage went under BORH threshold / interrupt occurred (if enabled). Cleared by writing 1 in the bit.
    #[inline(always)]
    pub fn borh_isc(&self) -> BORH_ISC_R {
        BORH_ISC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - PVD_ISC: Programmable Voltage Detector status. 0: no pending interrupt. 1: voltage went under programmed threshold / interrupt occurred (if enabled). Cleared by writing 1 in the bit.
    #[inline(always)]
    pub fn pvd_isc(&self) -> PVD_ISC_R {
        PVD_ISC_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - WKUP_ISC: Indicates the Power Controller receives a Wakeup event. 0: no pending interrupt. 1: Wakeup event on PWRC occurred / interrupt occurred (if enabled). Cleared by writing 1 in the bit. This flag will be read at 1 if a wakeup event arrives so close to the low power mode entry requests that the PWRC aborts before shutting down the system.
    #[inline(always)]
    pub fn wkup_isc(&self) -> WKUP_ISC_R {
        WKUP_ISC_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWRC_ISCR")
            .field("borh_isc", &self.borh_isc())
            .field("pvd_isc", &self.pvd_isc())
            .field("wkup_isc", &self.wkup_isc())
            .finish()
    }
}
impl W {
    ///Bit 0 - BORH_ISC: BORH interrupt status. 0: no pending interrupt. 1: voltage went under BORH threshold / interrupt occurred (if enabled). Cleared by writing 1 in the bit.
    #[inline(always)]
    pub fn borh_isc(&mut self) -> BORH_ISC_W<'_, PWRC_ISCRrs> {
        BORH_ISC_W::new(self, 0)
    }
    ///Bit 1 - PVD_ISC: Programmable Voltage Detector status. 0: no pending interrupt. 1: voltage went under programmed threshold / interrupt occurred (if enabled). Cleared by writing 1 in the bit.
    #[inline(always)]
    pub fn pvd_isc(&mut self) -> PVD_ISC_W<'_, PWRC_ISCRrs> {
        PVD_ISC_W::new(self, 1)
    }
    ///Bit 2 - WKUP_ISC: Indicates the Power Controller receives a Wakeup event. 0: no pending interrupt. 1: Wakeup event on PWRC occurred / interrupt occurred (if enabled). Cleared by writing 1 in the bit. This flag will be read at 1 if a wakeup event arrives so close to the low power mode entry requests that the PWRC aborts before shutting down the system.
    #[inline(always)]
    pub fn wkup_isc(&mut self) -> WKUP_ISC_W<'_, PWRC_ISCRrs> {
        WKUP_ISC_W::new(self, 2)
    }
}
/**PWRC_ISCR register

You can [`read`](crate::Reg::read) this register and get [`pwrc_iscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrc_iscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#SYSTEM_CTRL:PWRC_ISCR)*/
pub struct PWRC_ISCRrs;
impl crate::RegisterSpec for PWRC_ISCRrs {
    type Ux = u32;
}
///`read()` method returns [`pwrc_iscr::R`](R) reader structure
impl crate::Readable for PWRC_ISCRrs {}
///`write(|w| ..)` method takes [`pwrc_iscr::W`](W) writer structure
impl crate::Writable for PWRC_ISCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PWRC_ISCR to value 0
impl crate::Resettable for PWRC_ISCRrs {}
