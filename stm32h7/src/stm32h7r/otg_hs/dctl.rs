///Register `DCTL` reader
pub type R = crate::R<DCTLrs>;
///Register `DCTL` writer
pub type W = crate::W<DCTLrs>;
///Field `RWUSIG` reader - Remote wakeup signaling When the application sets this bit, the core initiates remote signaling to wake up the USB host. The application must set this bit to instruct the core to exit the suspend state. As specified in the USB 2.0 specification, the application must clear this bit 1 ms to 15 ms after setting it. If LPM is enabled and the core is in the L1 (sleep) state, when the application sets this bit, the core initiates L1 remote signaling to wake up the USB host. The application must set this bit to instruct the core to exit the sleep state. As specified in the LPM specification, the hardware automatically clears this bit 50 s (T<sub>L1DevDrvResume</sub>) after being set by the application. The application must not set this bit when bRemoteWake from the previous LPM transaction is zero (refer to REMWAKE bit in GLPMCFG register).
pub type RWUSIG_R = crate::BitReader;
///Field `RWUSIG` writer - Remote wakeup signaling When the application sets this bit, the core initiates remote signaling to wake up the USB host. The application must set this bit to instruct the core to exit the suspend state. As specified in the USB 2.0 specification, the application must clear this bit 1 ms to 15 ms after setting it. If LPM is enabled and the core is in the L1 (sleep) state, when the application sets this bit, the core initiates L1 remote signaling to wake up the USB host. The application must set this bit to instruct the core to exit the sleep state. As specified in the LPM specification, the hardware automatically clears this bit 50 s (T<sub>L1DevDrvResume</sub>) after being set by the application. The application must not set this bit when bRemoteWake from the previous LPM transaction is zero (refer to REMWAKE bit in GLPMCFG register).
pub type RWUSIG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDIS` reader - Soft disconnect The application uses this bit to signal the USB OTG core to perform a soft disconnect. As long as this bit is set, the host does not see that the device is connected, and the device does not receive signals on the USB. The core stays in the disconnected state until the application clears this bit.
pub type SDIS_R = crate::BitReader;
///Field `SDIS` writer - Soft disconnect The application uses this bit to signal the USB OTG core to perform a soft disconnect. As long as this bit is set, the host does not see that the device is connected, and the device does not receive signals on the USB. The core stays in the disconnected state until the application clears this bit.
pub type SDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GINSTS` reader - Global IN NAK status
pub type GINSTS_R = crate::BitReader;
///Field `GONSTS` reader - Global OUT NAK status
pub type GONSTS_R = crate::BitReader;
///Field `TCTL` reader - Test control Others: Reserved
pub type TCTL_R = crate::FieldReader;
///Field `TCTL` writer - Test control Others: Reserved
pub type TCTL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SGINAK` writer - Set global IN NAK Writing 1 to this field sets the Global non-periodic IN NAK.The application uses this bit to send a NAK handshake on all non-periodic IN endpoints. The application must set this bit only after making sure that the Global IN NAK effective bit in the core interrupt register (GINAKEFF bit in OTG_GINTSTS) is cleared.
pub type SGINAK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CGINAK` writer - Clear global IN NAK Writing 1 to this field clears the Global IN NAK.
pub type CGINAK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SGONAK` writer - Set global OUT NAK Writing 1 to this field sets the Global OUT NAK. The application uses this bit to send a NAK handshake on all OUT endpoints. The application must set the this bit only after making sure that the Global OUT NAK effective bit in the core interrupt register (GONAKEFF bit in OTG_GINTSTS) is cleared.
pub type SGONAK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CGONAK` writer - Clear global OUT NAK Writing 1 to this field clears the Global OUT NAK.
pub type CGONAK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `POPRGDNE` reader - Power-on programming done The application uses this bit to indicate that register programming is completed after a wakeup from power down mode.
pub type POPRGDNE_R = crate::BitReader;
///Field `POPRGDNE` writer - Power-on programming done The application uses this bit to indicate that register programming is completed after a wakeup from power down mode.
pub type POPRGDNE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DSBESLRJCT` reader - Deep sleep BESL reject Core rejects LPM request with BESL value greater than BESL threshold programmed. NYET response is sent for LPM tokens with BESL value greater than BESL threshold. By default, the deep sleep BESL reject feature is disabled.
pub type DSBESLRJCT_R = crate::BitReader;
///Field `DSBESLRJCT` writer - Deep sleep BESL reject Core rejects LPM request with BESL value greater than BESL threshold programmed. NYET response is sent for LPM tokens with BESL value greater than BESL threshold. By default, the deep sleep BESL reject feature is disabled.
pub type DSBESLRJCT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Remote wakeup signaling When the application sets this bit, the core initiates remote signaling to wake up the USB host. The application must set this bit to instruct the core to exit the suspend state. As specified in the USB 2.0 specification, the application must clear this bit 1 ms to 15 ms after setting it. If LPM is enabled and the core is in the L1 (sleep) state, when the application sets this bit, the core initiates L1 remote signaling to wake up the USB host. The application must set this bit to instruct the core to exit the sleep state. As specified in the LPM specification, the hardware automatically clears this bit 50 s (T<sub>L1DevDrvResume</sub>) after being set by the application. The application must not set this bit when bRemoteWake from the previous LPM transaction is zero (refer to REMWAKE bit in GLPMCFG register).
    #[inline(always)]
    pub fn rwusig(&self) -> RWUSIG_R {
        RWUSIG_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Soft disconnect The application uses this bit to signal the USB OTG core to perform a soft disconnect. As long as this bit is set, the host does not see that the device is connected, and the device does not receive signals on the USB. The core stays in the disconnected state until the application clears this bit.
    #[inline(always)]
    pub fn sdis(&self) -> SDIS_R {
        SDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Global IN NAK status
    #[inline(always)]
    pub fn ginsts(&self) -> GINSTS_R {
        GINSTS_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Global OUT NAK status
    #[inline(always)]
    pub fn gonsts(&self) -> GONSTS_R {
        GONSTS_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:6 - Test control Others: Reserved
    #[inline(always)]
    pub fn tctl(&self) -> TCTL_R {
        TCTL_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 11 - Power-on programming done The application uses this bit to indicate that register programming is completed after a wakeup from power down mode.
    #[inline(always)]
    pub fn poprgdne(&self) -> POPRGDNE_R {
        POPRGDNE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 18 - Deep sleep BESL reject Core rejects LPM request with BESL value greater than BESL threshold programmed. NYET response is sent for LPM tokens with BESL value greater than BESL threshold. By default, the deep sleep BESL reject feature is disabled.
    #[inline(always)]
    pub fn dsbeslrjct(&self) -> DSBESLRJCT_R {
        DSBESLRJCT_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCTL")
            .field("rwusig", &self.rwusig())
            .field("sdis", &self.sdis())
            .field("ginsts", &self.ginsts())
            .field("gonsts", &self.gonsts())
            .field("tctl", &self.tctl())
            .field("poprgdne", &self.poprgdne())
            .field("dsbeslrjct", &self.dsbeslrjct())
            .finish()
    }
}
impl W {
    ///Bit 0 - Remote wakeup signaling When the application sets this bit, the core initiates remote signaling to wake up the USB host. The application must set this bit to instruct the core to exit the suspend state. As specified in the USB 2.0 specification, the application must clear this bit 1 ms to 15 ms after setting it. If LPM is enabled and the core is in the L1 (sleep) state, when the application sets this bit, the core initiates L1 remote signaling to wake up the USB host. The application must set this bit to instruct the core to exit the sleep state. As specified in the LPM specification, the hardware automatically clears this bit 50 s (T<sub>L1DevDrvResume</sub>) after being set by the application. The application must not set this bit when bRemoteWake from the previous LPM transaction is zero (refer to REMWAKE bit in GLPMCFG register).
    #[inline(always)]
    pub fn rwusig(&mut self) -> RWUSIG_W<'_, DCTLrs> {
        RWUSIG_W::new(self, 0)
    }
    ///Bit 1 - Soft disconnect The application uses this bit to signal the USB OTG core to perform a soft disconnect. As long as this bit is set, the host does not see that the device is connected, and the device does not receive signals on the USB. The core stays in the disconnected state until the application clears this bit.
    #[inline(always)]
    pub fn sdis(&mut self) -> SDIS_W<'_, DCTLrs> {
        SDIS_W::new(self, 1)
    }
    ///Bits 4:6 - Test control Others: Reserved
    #[inline(always)]
    pub fn tctl(&mut self) -> TCTL_W<'_, DCTLrs> {
        TCTL_W::new(self, 4)
    }
    ///Bit 7 - Set global IN NAK Writing 1 to this field sets the Global non-periodic IN NAK.The application uses this bit to send a NAK handshake on all non-periodic IN endpoints. The application must set this bit only after making sure that the Global IN NAK effective bit in the core interrupt register (GINAKEFF bit in OTG_GINTSTS) is cleared.
    #[inline(always)]
    pub fn sginak(&mut self) -> SGINAK_W<'_, DCTLrs> {
        SGINAK_W::new(self, 7)
    }
    ///Bit 8 - Clear global IN NAK Writing 1 to this field clears the Global IN NAK.
    #[inline(always)]
    pub fn cginak(&mut self) -> CGINAK_W<'_, DCTLrs> {
        CGINAK_W::new(self, 8)
    }
    ///Bit 9 - Set global OUT NAK Writing 1 to this field sets the Global OUT NAK. The application uses this bit to send a NAK handshake on all OUT endpoints. The application must set the this bit only after making sure that the Global OUT NAK effective bit in the core interrupt register (GONAKEFF bit in OTG_GINTSTS) is cleared.
    #[inline(always)]
    pub fn sgonak(&mut self) -> SGONAK_W<'_, DCTLrs> {
        SGONAK_W::new(self, 9)
    }
    ///Bit 10 - Clear global OUT NAK Writing 1 to this field clears the Global OUT NAK.
    #[inline(always)]
    pub fn cgonak(&mut self) -> CGONAK_W<'_, DCTLrs> {
        CGONAK_W::new(self, 10)
    }
    ///Bit 11 - Power-on programming done The application uses this bit to indicate that register programming is completed after a wakeup from power down mode.
    #[inline(always)]
    pub fn poprgdne(&mut self) -> POPRGDNE_W<'_, DCTLrs> {
        POPRGDNE_W::new(self, 11)
    }
    ///Bit 18 - Deep sleep BESL reject Core rejects LPM request with BESL value greater than BESL threshold programmed. NYET response is sent for LPM tokens with BESL value greater than BESL threshold. By default, the deep sleep BESL reject feature is disabled.
    #[inline(always)]
    pub fn dsbeslrjct(&mut self) -> DSBESLRJCT_W<'_, DCTLrs> {
        DSBESLRJCT_W::new(self, 18)
    }
}
/**OTG device control register

You can [`read`](crate::Reg::read) this register and get [`dctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#OTG_HS:DCTL)*/
pub struct DCTLrs;
impl crate::RegisterSpec for DCTLrs {
    type Ux = u32;
}
///`read()` method returns [`dctl::R`](R) reader structure
impl crate::Readable for DCTLrs {}
///`write(|w| ..)` method takes [`dctl::W`](W) writer structure
impl crate::Writable for DCTLrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DCTL to value 0x02
impl crate::Resettable for DCTLrs {
    const RESET_VALUE: u32 = 0x02;
}
