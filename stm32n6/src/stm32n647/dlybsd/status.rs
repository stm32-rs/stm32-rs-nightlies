///Register `STATUS` reader
pub type R = crate::R<STATUSrs>;
///Register `STATUS` writer
pub type W = crate::W<STATUSrs>;
///Field `SDMMC_DLL_LOCK` reader - SDMMC DLL lock
pub type SDMMC_DLL_LOCK_R = crate::BitReader;
///Field `SDMMC_DLL_LOCK` writer - SDMMC DLL lock
pub type SDMMC_DLL_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDMMC_RX_TAP_SEL_ACK` reader - SDMMC RX delay selection acknowledge
pub type SDMMC_RX_TAP_SEL_ACK_R = crate::BitReader;
///Field `SDMMC_RX_TAP_SEL_ACK` writer - SDMMC RX delay selection acknowledge
pub type SDMMC_RX_TAP_SEL_ACK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - SDMMC DLL lock
    #[inline(always)]
    pub fn sdmmc_dll_lock(&self) -> SDMMC_DLL_LOCK_R {
        SDMMC_DLL_LOCK_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - SDMMC RX delay selection acknowledge
    #[inline(always)]
    pub fn sdmmc_rx_tap_sel_ack(&self) -> SDMMC_RX_TAP_SEL_ACK_R {
        SDMMC_RX_TAP_SEL_ACK_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS")
            .field("sdmmc_dll_lock", &self.sdmmc_dll_lock())
            .field("sdmmc_rx_tap_sel_ack", &self.sdmmc_rx_tap_sel_ack())
            .finish()
    }
}
impl W {
    ///Bit 0 - SDMMC DLL lock
    #[inline(always)]
    pub fn sdmmc_dll_lock(&mut self) -> SDMMC_DLL_LOCK_W<'_, STATUSrs> {
        SDMMC_DLL_LOCK_W::new(self, 0)
    }
    ///Bit 1 - SDMMC RX delay selection acknowledge
    #[inline(always)]
    pub fn sdmmc_rx_tap_sel_ack(&mut self) -> SDMMC_RX_TAP_SEL_ACK_W<'_, STATUSrs> {
        SDMMC_RX_TAP_SEL_ACK_W::new(self, 1)
    }
}
/**Delay block SDMMC DLL status

You can [`read`](crate::Reg::read) this register and get [`status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#DLYBSD:STATUS)*/
pub struct STATUSrs;
impl crate::RegisterSpec for STATUSrs {
    type Ux = u32;
}
///`read()` method returns [`status::R`](R) reader structure
impl crate::Readable for STATUSrs {}
///`write(|w| ..)` method takes [`status::W`](W) writer structure
impl crate::Writable for STATUSrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets STATUS to value 0x02
impl crate::Resettable for STATUSrs {
    const RESET_VALUE: u32 = 0x02;
}
