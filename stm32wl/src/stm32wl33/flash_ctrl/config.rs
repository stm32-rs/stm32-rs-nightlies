///Register `CONFIG` reader
pub type R = crate::R<CONFIGrs>;
///Register `CONFIG` writer
pub type W = crate::W<CONFIGrs>;
///Field `REMAP` reader - CPU access routing (it supersedes PREMAP configuration): - 0 : FLASH memory addressed - 1 : SRAM0 memory addressed
pub type REMAP_R = crate::BitReader;
///Field `REMAP` writer - CPU access routing (it supersedes PREMAP configuration): - 0 : FLASH memory addressed - 1 : SRAM0 memory addressed
pub type REMAP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DIS_GROUP_WRITE` reader - Burst write Control: - 0 : burst write allowed - 1 : burst write forbidden
pub type DIS_GROUP_WRITE_R = crate::BitReader;
///Field `DIS_GROUP_WRITE` writer - Burst write Control: - 0 : burst write allowed - 1 : burst write forbidden
pub type DIS_GROUP_WRITE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WAIT_STATE` reader - Add latency to flash read opeations: - 00 : no latency - 01 : 1 clock cycle latency - 10 : 2 clock cycles latency - 11 : 3 clock cycles latency
pub type WAIT_STATE_R = crate::FieldReader;
///Field `WAIT_STATE` writer - Add latency to flash read opeations: - 00 : no latency - 01 : 1 clock cycle latency - 10 : 2 clock cycles latency - 11 : 3 clock cycles latency
pub type WAIT_STATE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SLEEP_SM` reader - Flash memory power-down mode enable in SLEEP mode This bit allows to have the Flash memory in power-down mode or in idle mode when the device is in Sleep mode. - 0: When the device is in Sleep mode, the NVM is in Idle mode. - 1: When the device is in Sleep mode, the NVM is in power-down mode.
pub type SLEEP_SM_R = crate::BitReader;
///Field `SLEEP_SM` writer - Flash memory power-down mode enable in SLEEP mode This bit allows to have the Flash memory in power-down mode or in idle mode when the device is in Sleep mode. - 0: When the device is in Sleep mode, the NVM is in Idle mode. - 1: When the device is in Sleep mode, the NVM is in power-down mode.
pub type SLEEP_SM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 1 - CPU access routing (it supersedes PREMAP configuration): - 0 : FLASH memory addressed - 1 : SRAM0 memory addressed
    #[inline(always)]
    pub fn remap(&self) -> REMAP_R {
        REMAP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Burst write Control: - 0 : burst write allowed - 1 : burst write forbidden
    #[inline(always)]
    pub fn dis_group_write(&self) -> DIS_GROUP_WRITE_R {
        DIS_GROUP_WRITE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 4:5 - Add latency to flash read opeations: - 00 : no latency - 01 : 1 clock cycle latency - 10 : 2 clock cycles latency - 11 : 3 clock cycles latency
    #[inline(always)]
    pub fn wait_state(&self) -> WAIT_STATE_R {
        WAIT_STATE_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 6 - Flash memory power-down mode enable in SLEEP mode This bit allows to have the Flash memory in power-down mode or in idle mode when the device is in Sleep mode. - 0: When the device is in Sleep mode, the NVM is in Idle mode. - 1: When the device is in Sleep mode, the NVM is in power-down mode.
    #[inline(always)]
    pub fn sleep_sm(&self) -> SLEEP_SM_R {
        SLEEP_SM_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONFIG")
            .field("remap", &self.remap())
            .field("dis_group_write", &self.dis_group_write())
            .field("wait_state", &self.wait_state())
            .field("sleep_sm", &self.sleep_sm())
            .finish()
    }
}
impl W {
    ///Bit 1 - CPU access routing (it supersedes PREMAP configuration): - 0 : FLASH memory addressed - 1 : SRAM0 memory addressed
    #[inline(always)]
    pub fn remap(&mut self) -> REMAP_W<'_, CONFIGrs> {
        REMAP_W::new(self, 1)
    }
    ///Bit 2 - Burst write Control: - 0 : burst write allowed - 1 : burst write forbidden
    #[inline(always)]
    pub fn dis_group_write(&mut self) -> DIS_GROUP_WRITE_W<'_, CONFIGrs> {
        DIS_GROUP_WRITE_W::new(self, 2)
    }
    ///Bits 4:5 - Add latency to flash read opeations: - 00 : no latency - 01 : 1 clock cycle latency - 10 : 2 clock cycles latency - 11 : 3 clock cycles latency
    #[inline(always)]
    pub fn wait_state(&mut self) -> WAIT_STATE_W<'_, CONFIGrs> {
        WAIT_STATE_W::new(self, 4)
    }
    ///Bit 6 - Flash memory power-down mode enable in SLEEP mode This bit allows to have the Flash memory in power-down mode or in idle mode when the device is in Sleep mode. - 0: When the device is in Sleep mode, the NVM is in Idle mode. - 1: When the device is in Sleep mode, the NVM is in power-down mode.
    #[inline(always)]
    pub fn sleep_sm(&mut self) -> SLEEP_SM_W<'_, CONFIGrs> {
        SLEEP_SM_W::new(self, 6)
    }
}
/**CONFIG register

You can [`read`](crate::Reg::read) this register and get [`config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#FLASH_CTRL:CONFIG)*/
pub struct CONFIGrs;
impl crate::RegisterSpec for CONFIGrs {
    type Ux = u32;
}
///`read()` method returns [`config::R`](R) reader structure
impl crate::Readable for CONFIGrs {}
///`write(|w| ..)` method takes [`config::W`](W) writer structure
impl crate::Writable for CONFIGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CONFIG to value 0x10
impl crate::Resettable for CONFIGrs {
    const RESET_VALUE: u32 = 0x10;
}
