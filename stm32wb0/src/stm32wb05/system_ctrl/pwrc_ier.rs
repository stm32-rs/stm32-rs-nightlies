///Register `PWRC_IER` reader
pub type R = crate::R<PWRC_IERrs>;
///Register `PWRC_IER` writer
pub type W = crate::W<PWRC_IERrs>;
///Field `BORH_IE` reader - BORH_IE: BORH interrupt enable. 0: BORH interrupt is disabled. 1: BORH interrupt is enabled.
pub type BORH_IE_R = crate::BitReader;
///Field `BORH_IE` writer - BORH_IE: BORH interrupt enable. 0: BORH interrupt is disabled. 1: BORH interrupt is enabled.
pub type BORH_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PVD_IE` reader - PVD_IE: Programmable Voltage Detector interrupt enable. 0: PVD interrupt is disabled. 1: PVD interrupt is enabled.
pub type PVD_IE_R = crate::BitReader;
///Field `PVD_IE` writer - PVD_IE: Programmable Voltage Detector interrupt enable. 0: PVD interrupt is disabled. 1: PVD interrupt is enabled.
pub type PVD_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WKUP_IE` reader - WKUP_IE: Power Controller Wakeup event interrupt enable. 0: Interrupt on wakeup event seen by the PWRC is disabled. 1: Interrupt on wakeup event seen by the PWRC is enabled.
pub type WKUP_IE_R = crate::BitReader;
///Field `WKUP_IE` writer - WKUP_IE: Power Controller Wakeup event interrupt enable. 0: Interrupt on wakeup event seen by the PWRC is disabled. 1: Interrupt on wakeup event seen by the PWRC is enabled.
pub type WKUP_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - BORH_IE: BORH interrupt enable. 0: BORH interrupt is disabled. 1: BORH interrupt is enabled.
    #[inline(always)]
    pub fn borh_ie(&self) -> BORH_IE_R {
        BORH_IE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - PVD_IE: Programmable Voltage Detector interrupt enable. 0: PVD interrupt is disabled. 1: PVD interrupt is enabled.
    #[inline(always)]
    pub fn pvd_ie(&self) -> PVD_IE_R {
        PVD_IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - WKUP_IE: Power Controller Wakeup event interrupt enable. 0: Interrupt on wakeup event seen by the PWRC is disabled. 1: Interrupt on wakeup event seen by the PWRC is enabled.
    #[inline(always)]
    pub fn wkup_ie(&self) -> WKUP_IE_R {
        WKUP_IE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWRC_IER")
            .field("borh_ie", &self.borh_ie())
            .field("pvd_ie", &self.pvd_ie())
            .field("wkup_ie", &self.wkup_ie())
            .finish()
    }
}
impl W {
    ///Bit 0 - BORH_IE: BORH interrupt enable. 0: BORH interrupt is disabled. 1: BORH interrupt is enabled.
    #[inline(always)]
    pub fn borh_ie(&mut self) -> BORH_IE_W<'_, PWRC_IERrs> {
        BORH_IE_W::new(self, 0)
    }
    ///Bit 1 - PVD_IE: Programmable Voltage Detector interrupt enable. 0: PVD interrupt is disabled. 1: PVD interrupt is enabled.
    #[inline(always)]
    pub fn pvd_ie(&mut self) -> PVD_IE_W<'_, PWRC_IERrs> {
        PVD_IE_W::new(self, 1)
    }
    ///Bit 2 - WKUP_IE: Power Controller Wakeup event interrupt enable. 0: Interrupt on wakeup event seen by the PWRC is disabled. 1: Interrupt on wakeup event seen by the PWRC is enabled.
    #[inline(always)]
    pub fn wkup_ie(&mut self) -> WKUP_IE_W<'_, PWRC_IERrs> {
        WKUP_IE_W::new(self, 2)
    }
}
/**PWRC_IER register

You can [`read`](crate::Reg::read) this register and get [`pwrc_ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrc_ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#SYSTEM_CTRL:PWRC_IER)*/
pub struct PWRC_IERrs;
impl crate::RegisterSpec for PWRC_IERrs {
    type Ux = u32;
}
///`read()` method returns [`pwrc_ier::R`](R) reader structure
impl crate::Readable for PWRC_IERrs {}
///`write(|w| ..)` method takes [`pwrc_ier::W`](W) writer structure
impl crate::Writable for PWRC_IERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PWRC_IER to value 0
impl crate::Resettable for PWRC_IERrs {}
