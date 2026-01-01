///Register `GOTGCTL` reader
pub type R = crate::R<GOTGCTLrs>;
///Register `GOTGCTL` writer
pub type W = crate::W<GOTGCTLrs>;
///Field `SRQSCS` reader - Session request success The core sets this bit when a session request initiation is successful. Note: Only accessible in device mode.
pub type SRQSCS_R = crate::BitReader;
///Field `SRQ` reader - Session request The application sets this bit to initiate a session request on the USB. The application can clear this bit by writing a 0 when the host negotiation success status change bit in the OTG_GOTGINT register (HNSSCHG bit in OTG_GOTGINT) is set. The core clears this bit when the HNSSCHG bit is cleared. If the user uses the USB 1.1 full-speed serial transceiver interface to initiate the session request, the application must wait until VBUS discharges to 0.2 V, after the B-session valid bit in this register (BSVLD bit in OTG_GOTGCTL) is cleared. Note: Only accessible in device mode.
pub type SRQ_R = crate::BitReader;
///Field `SRQ` writer - Session request The application sets this bit to initiate a session request on the USB. The application can clear this bit by writing a 0 when the host negotiation success status change bit in the OTG_GOTGINT register (HNSSCHG bit in OTG_GOTGINT) is set. The core clears this bit when the HNSSCHG bit is cleared. If the user uses the USB 1.1 full-speed serial transceiver interface to initiate the session request, the application must wait until VBUS discharges to 0.2 V, after the B-session valid bit in this register (BSVLD bit in OTG_GOTGCTL) is cleared. Note: Only accessible in device mode.
pub type SRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VBVALOEN` reader - VBUS valid override enable. This bit is used to enable/disable the software to override the vbusvalid signal using the VBVALOVAL bit. Note: Only accessible in host mode.
pub type VBVALOEN_R = crate::BitReader;
///Field `VBVALOEN` writer - VBUS valid override enable. This bit is used to enable/disable the software to override the vbusvalid signal using the VBVALOVAL bit. Note: Only accessible in host mode.
pub type VBVALOEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VBVALOVAL` reader - VBUS valid override value. This bit is used to set override value for vbusvalid signal when VBVALOEN bit is set. Note: Only accessible in host mode.
pub type VBVALOVAL_R = crate::BitReader;
///Field `VBVALOVAL` writer - VBUS valid override value. This bit is used to set override value for vbusvalid signal when VBVALOEN bit is set. Note: Only accessible in host mode.
pub type VBVALOVAL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AVALOEN` reader - A-peripheral session valid override enable. This bit is used to enable/disable the software to override the Avalid signal using the AVALOVAL bit. Note: Only accessible in host mode.
pub type AVALOEN_R = crate::BitReader;
///Field `AVALOEN` writer - A-peripheral session valid override enable. This bit is used to enable/disable the software to override the Avalid signal using the AVALOVAL bit. Note: Only accessible in host mode.
pub type AVALOEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AVALOVAL` reader - A-peripheral session valid override value. This bit is used to set override value for Avalid signal when AVALOEN bit is set. Note: Only accessible in host mode.
pub type AVALOVAL_R = crate::BitReader;
///Field `AVALOVAL` writer - A-peripheral session valid override value. This bit is used to set override value for Avalid signal when AVALOEN bit is set. Note: Only accessible in host mode.
pub type AVALOVAL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BVALOEN` reader - B-peripheral session valid override enable. This bit is used to enable/disable the software to override the Bvalid signal using the BVALOVAL bit. Note: Only accessible in device mode.
pub type BVALOEN_R = crate::BitReader;
///Field `BVALOEN` writer - B-peripheral session valid override enable. This bit is used to enable/disable the software to override the Bvalid signal using the BVALOVAL bit. Note: Only accessible in device mode.
pub type BVALOEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BVALOVAL` reader - B-peripheral session valid override value. This bit is used to set override value for Bvalid signal when BVALOEN bit is set. Note: Only accessible in device mode.
pub type BVALOVAL_R = crate::BitReader;
///Field `BVALOVAL` writer - B-peripheral session valid override value. This bit is used to set override value for Bvalid signal when BVALOEN bit is set. Note: Only accessible in device mode.
pub type BVALOVAL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HNGSCS` reader - Host negotiation success The core sets this bit when host negotiation is successful. The core clears this bit when the HNP request (HNPRQ) bit in this register is set. Note: Only accessible in device mode.
pub type HNGSCS_R = crate::BitReader;
///Field `HNPRQ` reader - HNP request The application sets this bit to initiate an HNP request to the connected USB host. The application can clear this bit by writing a 0 when the host negotiation success status change bit in the OTG_GOTGINT register (HNSSCHG bit in OTG_GOTGINT) is set. The core clears this bit when the HNSSCHG bit is cleared. Note: Only accessible in device mode.
pub type HNPRQ_R = crate::BitReader;
///Field `HNPRQ` writer - HNP request The application sets this bit to initiate an HNP request to the connected USB host. The application can clear this bit by writing a 0 when the host negotiation success status change bit in the OTG_GOTGINT register (HNSSCHG bit in OTG_GOTGINT) is set. The core clears this bit when the HNSSCHG bit is cleared. Note: Only accessible in device mode.
pub type HNPRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSHNPEN` reader - host set HNP enable The application sets this bit when it has successfully enabled HNP (using the SetFeature.SetHNPEnable command) on the connected device. Note: Only accessible in host mode.
pub type HSHNPEN_R = crate::BitReader;
///Field `HSHNPEN` writer - host set HNP enable The application sets this bit when it has successfully enabled HNP (using the SetFeature.SetHNPEnable command) on the connected device. Note: Only accessible in host mode.
pub type HSHNPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DHNPEN` reader - Device HNP enabled The application sets this bit when it successfully receives a SetFeature.SetHNPEnable command from the connected USB host. Note: Only accessible in device mode.
pub type DHNPEN_R = crate::BitReader;
///Field `DHNPEN` writer - Device HNP enabled The application sets this bit when it successfully receives a SetFeature.SetHNPEnable command from the connected USB host. Note: Only accessible in device mode.
pub type DHNPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EHEN` reader - Embedded host enable It is used to select between OTG A device state machine and embedded host state machine.
pub type EHEN_R = crate::BitReader;
///Field `EHEN` writer - Embedded host enable It is used to select between OTG A device state machine and embedded host state machine.
pub type EHEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CIDSTS` reader - Connector ID status Indicates the connector ID status on a connect event. Note: Accessible in both device and host modes.
pub type CIDSTS_R = crate::BitReader;
///Field `DBCT` reader - Long/short debounce time Indicates the debounce time of a detected connection. Note: Only accessible in host mode.
pub type DBCT_R = crate::BitReader;
///Field `ASVLD` reader - A-session valid Indicates the host mode transceiver status. Note: Only accessible in host mode.
pub type ASVLD_R = crate::BitReader;
///Field `BSVLD` reader - B-session valid Indicates the device mode transceiver status. In OTG mode, the user can use this bit to determine if the device is connected or disconnected. Note: Only accessible in device mode.
pub type BSVLD_R = crate::BitReader;
///Field `OTGVER` reader - OTG version Selects the OTG revision.
pub type OTGVER_R = crate::BitReader;
///Field `OTGVER` writer - OTG version Selects the OTG revision.
pub type OTGVER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CURMOD` reader - Current mode of operation Indicates the current mode (host or device).
pub type CURMOD_R = crate::BitReader;
impl R {
    ///Bit 0 - Session request success The core sets this bit when a session request initiation is successful. Note: Only accessible in device mode.
    #[inline(always)]
    pub fn srqscs(&self) -> SRQSCS_R {
        SRQSCS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Session request The application sets this bit to initiate a session request on the USB. The application can clear this bit by writing a 0 when the host negotiation success status change bit in the OTG_GOTGINT register (HNSSCHG bit in OTG_GOTGINT) is set. The core clears this bit when the HNSSCHG bit is cleared. If the user uses the USB 1.1 full-speed serial transceiver interface to initiate the session request, the application must wait until VBUS discharges to 0.2 V, after the B-session valid bit in this register (BSVLD bit in OTG_GOTGCTL) is cleared. Note: Only accessible in device mode.
    #[inline(always)]
    pub fn srq(&self) -> SRQ_R {
        SRQ_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - VBUS valid override enable. This bit is used to enable/disable the software to override the vbusvalid signal using the VBVALOVAL bit. Note: Only accessible in host mode.
    #[inline(always)]
    pub fn vbvaloen(&self) -> VBVALOEN_R {
        VBVALOEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - VBUS valid override value. This bit is used to set override value for vbusvalid signal when VBVALOEN bit is set. Note: Only accessible in host mode.
    #[inline(always)]
    pub fn vbvaloval(&self) -> VBVALOVAL_R {
        VBVALOVAL_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - A-peripheral session valid override enable. This bit is used to enable/disable the software to override the Avalid signal using the AVALOVAL bit. Note: Only accessible in host mode.
    #[inline(always)]
    pub fn avaloen(&self) -> AVALOEN_R {
        AVALOEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - A-peripheral session valid override value. This bit is used to set override value for Avalid signal when AVALOEN bit is set. Note: Only accessible in host mode.
    #[inline(always)]
    pub fn avaloval(&self) -> AVALOVAL_R {
        AVALOVAL_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - B-peripheral session valid override enable. This bit is used to enable/disable the software to override the Bvalid signal using the BVALOVAL bit. Note: Only accessible in device mode.
    #[inline(always)]
    pub fn bvaloen(&self) -> BVALOEN_R {
        BVALOEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - B-peripheral session valid override value. This bit is used to set override value for Bvalid signal when BVALOEN bit is set. Note: Only accessible in device mode.
    #[inline(always)]
    pub fn bvaloval(&self) -> BVALOVAL_R {
        BVALOVAL_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Host negotiation success The core sets this bit when host negotiation is successful. The core clears this bit when the HNP request (HNPRQ) bit in this register is set. Note: Only accessible in device mode.
    #[inline(always)]
    pub fn hngscs(&self) -> HNGSCS_R {
        HNGSCS_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - HNP request The application sets this bit to initiate an HNP request to the connected USB host. The application can clear this bit by writing a 0 when the host negotiation success status change bit in the OTG_GOTGINT register (HNSSCHG bit in OTG_GOTGINT) is set. The core clears this bit when the HNSSCHG bit is cleared. Note: Only accessible in device mode.
    #[inline(always)]
    pub fn hnprq(&self) -> HNPRQ_R {
        HNPRQ_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - host set HNP enable The application sets this bit when it has successfully enabled HNP (using the SetFeature.SetHNPEnable command) on the connected device. Note: Only accessible in host mode.
    #[inline(always)]
    pub fn hshnpen(&self) -> HSHNPEN_R {
        HSHNPEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Device HNP enabled The application sets this bit when it successfully receives a SetFeature.SetHNPEnable command from the connected USB host. Note: Only accessible in device mode.
    #[inline(always)]
    pub fn dhnpen(&self) -> DHNPEN_R {
        DHNPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Embedded host enable It is used to select between OTG A device state machine and embedded host state machine.
    #[inline(always)]
    pub fn ehen(&self) -> EHEN_R {
        EHEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 16 - Connector ID status Indicates the connector ID status on a connect event. Note: Accessible in both device and host modes.
    #[inline(always)]
    pub fn cidsts(&self) -> CIDSTS_R {
        CIDSTS_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Long/short debounce time Indicates the debounce time of a detected connection. Note: Only accessible in host mode.
    #[inline(always)]
    pub fn dbct(&self) -> DBCT_R {
        DBCT_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - A-session valid Indicates the host mode transceiver status. Note: Only accessible in host mode.
    #[inline(always)]
    pub fn asvld(&self) -> ASVLD_R {
        ASVLD_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - B-session valid Indicates the device mode transceiver status. In OTG mode, the user can use this bit to determine if the device is connected or disconnected. Note: Only accessible in device mode.
    #[inline(always)]
    pub fn bsvld(&self) -> BSVLD_R {
        BSVLD_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - OTG version Selects the OTG revision.
    #[inline(always)]
    pub fn otgver(&self) -> OTGVER_R {
        OTGVER_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Current mode of operation Indicates the current mode (host or device).
    #[inline(always)]
    pub fn curmod(&self) -> CURMOD_R {
        CURMOD_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GOTGCTL")
            .field("srqscs", &self.srqscs())
            .field("srq", &self.srq())
            .field("vbvaloen", &self.vbvaloen())
            .field("vbvaloval", &self.vbvaloval())
            .field("avaloen", &self.avaloen())
            .field("avaloval", &self.avaloval())
            .field("bvaloen", &self.bvaloen())
            .field("bvaloval", &self.bvaloval())
            .field("hngscs", &self.hngscs())
            .field("hnprq", &self.hnprq())
            .field("hshnpen", &self.hshnpen())
            .field("dhnpen", &self.dhnpen())
            .field("ehen", &self.ehen())
            .field("cidsts", &self.cidsts())
            .field("dbct", &self.dbct())
            .field("asvld", &self.asvld())
            .field("bsvld", &self.bsvld())
            .field("otgver", &self.otgver())
            .field("curmod", &self.curmod())
            .finish()
    }
}
impl W {
    ///Bit 1 - Session request The application sets this bit to initiate a session request on the USB. The application can clear this bit by writing a 0 when the host negotiation success status change bit in the OTG_GOTGINT register (HNSSCHG bit in OTG_GOTGINT) is set. The core clears this bit when the HNSSCHG bit is cleared. If the user uses the USB 1.1 full-speed serial transceiver interface to initiate the session request, the application must wait until VBUS discharges to 0.2 V, after the B-session valid bit in this register (BSVLD bit in OTG_GOTGCTL) is cleared. Note: Only accessible in device mode.
    #[inline(always)]
    pub fn srq(&mut self) -> SRQ_W<'_, GOTGCTLrs> {
        SRQ_W::new(self, 1)
    }
    ///Bit 2 - VBUS valid override enable. This bit is used to enable/disable the software to override the vbusvalid signal using the VBVALOVAL bit. Note: Only accessible in host mode.
    #[inline(always)]
    pub fn vbvaloen(&mut self) -> VBVALOEN_W<'_, GOTGCTLrs> {
        VBVALOEN_W::new(self, 2)
    }
    ///Bit 3 - VBUS valid override value. This bit is used to set override value for vbusvalid signal when VBVALOEN bit is set. Note: Only accessible in host mode.
    #[inline(always)]
    pub fn vbvaloval(&mut self) -> VBVALOVAL_W<'_, GOTGCTLrs> {
        VBVALOVAL_W::new(self, 3)
    }
    ///Bit 4 - A-peripheral session valid override enable. This bit is used to enable/disable the software to override the Avalid signal using the AVALOVAL bit. Note: Only accessible in host mode.
    #[inline(always)]
    pub fn avaloen(&mut self) -> AVALOEN_W<'_, GOTGCTLrs> {
        AVALOEN_W::new(self, 4)
    }
    ///Bit 5 - A-peripheral session valid override value. This bit is used to set override value for Avalid signal when AVALOEN bit is set. Note: Only accessible in host mode.
    #[inline(always)]
    pub fn avaloval(&mut self) -> AVALOVAL_W<'_, GOTGCTLrs> {
        AVALOVAL_W::new(self, 5)
    }
    ///Bit 6 - B-peripheral session valid override enable. This bit is used to enable/disable the software to override the Bvalid signal using the BVALOVAL bit. Note: Only accessible in device mode.
    #[inline(always)]
    pub fn bvaloen(&mut self) -> BVALOEN_W<'_, GOTGCTLrs> {
        BVALOEN_W::new(self, 6)
    }
    ///Bit 7 - B-peripheral session valid override value. This bit is used to set override value for Bvalid signal when BVALOEN bit is set. Note: Only accessible in device mode.
    #[inline(always)]
    pub fn bvaloval(&mut self) -> BVALOVAL_W<'_, GOTGCTLrs> {
        BVALOVAL_W::new(self, 7)
    }
    ///Bit 9 - HNP request The application sets this bit to initiate an HNP request to the connected USB host. The application can clear this bit by writing a 0 when the host negotiation success status change bit in the OTG_GOTGINT register (HNSSCHG bit in OTG_GOTGINT) is set. The core clears this bit when the HNSSCHG bit is cleared. Note: Only accessible in device mode.
    #[inline(always)]
    pub fn hnprq(&mut self) -> HNPRQ_W<'_, GOTGCTLrs> {
        HNPRQ_W::new(self, 9)
    }
    ///Bit 10 - host set HNP enable The application sets this bit when it has successfully enabled HNP (using the SetFeature.SetHNPEnable command) on the connected device. Note: Only accessible in host mode.
    #[inline(always)]
    pub fn hshnpen(&mut self) -> HSHNPEN_W<'_, GOTGCTLrs> {
        HSHNPEN_W::new(self, 10)
    }
    ///Bit 11 - Device HNP enabled The application sets this bit when it successfully receives a SetFeature.SetHNPEnable command from the connected USB host. Note: Only accessible in device mode.
    #[inline(always)]
    pub fn dhnpen(&mut self) -> DHNPEN_W<'_, GOTGCTLrs> {
        DHNPEN_W::new(self, 11)
    }
    ///Bit 12 - Embedded host enable It is used to select between OTG A device state machine and embedded host state machine.
    #[inline(always)]
    pub fn ehen(&mut self) -> EHEN_W<'_, GOTGCTLrs> {
        EHEN_W::new(self, 12)
    }
    ///Bit 20 - OTG version Selects the OTG revision.
    #[inline(always)]
    pub fn otgver(&mut self) -> OTGVER_W<'_, GOTGCTLrs> {
        OTGVER_W::new(self, 20)
    }
}
/**OTG_HS control and status register

You can [`read`](crate::Reg::read) this register and get [`gotgctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gotgctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#OTG1_HS_GLOBAL:GOTGCTL)*/
pub struct GOTGCTLrs;
impl crate::RegisterSpec for GOTGCTLrs {
    type Ux = u32;
}
///`read()` method returns [`gotgctl::R`](R) reader structure
impl crate::Readable for GOTGCTLrs {}
///`write(|w| ..)` method takes [`gotgctl::W`](W) writer structure
impl crate::Writable for GOTGCTLrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GOTGCTL to value 0x0800
impl crate::Resettable for GOTGCTLrs {
    const RESET_VALUE: u32 = 0x0800;
}
