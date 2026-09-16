///Register `BLUE_SLEEP_REQUEST_MODE` reader
pub type R = crate::R<BLUE_SLEEP_REQUEST_MODErs>;
///Register `BLUE_SLEEP_REQUEST_MODE` writer
pub type W = crate::W<BLUE_SLEEP_REQUEST_MODErs>;
///Field `SLEEP_EN` reader - IP_BLE sleeping mode enable:
pub type SLEEP_EN_R = crate::BitReader;
///Field `SLEEP_EN` writer - IP_BLE sleeping mode enable:
pub type SLEEP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BLE_WAKEUP_EN` reader - IP_BLE wakeup enable:
pub type BLE_WAKEUP_EN_R = crate::BitReader;
///Field `BLE_WAKEUP_EN` writer - IP_BLE wakeup enable:
pub type BLE_WAKEUP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FORCE_SLEEPING` reader - IP_BLE sleeping control:
pub type FORCE_SLEEPING_R = crate::BitReader;
///Field `FORCE_SLEEPING` writer - IP_BLE sleeping control:
pub type FORCE_SLEEPING_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 29 - IP_BLE sleeping mode enable:
    #[inline(always)]
    pub fn sleep_en(&self) -> SLEEP_EN_R {
        SLEEP_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - IP_BLE wakeup enable:
    #[inline(always)]
    pub fn ble_wakeup_en(&self) -> BLE_WAKEUP_EN_R {
        BLE_WAKEUP_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - IP_BLE sleeping control:
    #[inline(always)]
    pub fn force_sleeping(&self) -> FORCE_SLEEPING_R {
        FORCE_SLEEPING_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLUE_SLEEP_REQUEST_MODE")
            .field("sleep_en", &self.sleep_en())
            .field("ble_wakeup_en", &self.ble_wakeup_en())
            .field("force_sleeping", &self.force_sleeping())
            .finish()
    }
}
impl W {
    ///Bit 29 - IP_BLE sleeping mode enable:
    #[inline(always)]
    pub fn sleep_en(&mut self) -> SLEEP_EN_W<'_, BLUE_SLEEP_REQUEST_MODErs> {
        SLEEP_EN_W::new(self, 29)
    }
    ///Bit 30 - IP_BLE wakeup enable:
    #[inline(always)]
    pub fn ble_wakeup_en(&mut self) -> BLE_WAKEUP_EN_W<'_, BLUE_SLEEP_REQUEST_MODErs> {
        BLE_WAKEUP_EN_W::new(self, 30)
    }
    ///Bit 31 - IP_BLE sleeping control:
    #[inline(always)]
    pub fn force_sleeping(&mut self) -> FORCE_SLEEPING_W<'_, BLUE_SLEEP_REQUEST_MODErs> {
        FORCE_SLEEPING_W::new(self, 31)
    }
}
/**BLUE_SLEEP_REQUEST_MODE register

You can [`read`](crate::Reg::read) this register and get [`blue_sleep_request_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blue_sleep_request_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#WAKEUP:BLUE_SLEEP_REQUEST_MODE)*/
pub struct BLUE_SLEEP_REQUEST_MODErs;
impl crate::RegisterSpec for BLUE_SLEEP_REQUEST_MODErs {
    type Ux = u32;
}
///`read()` method returns [`blue_sleep_request_mode::R`](R) reader structure
impl crate::Readable for BLUE_SLEEP_REQUEST_MODErs {}
///`write(|w| ..)` method takes [`blue_sleep_request_mode::W`](W) writer structure
impl crate::Writable for BLUE_SLEEP_REQUEST_MODErs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BLUE_SLEEP_REQUEST_MODE to value 0x07
impl crate::Resettable for BLUE_SLEEP_REQUEST_MODErs {
    const RESET_VALUE: u32 = 0x07;
}
