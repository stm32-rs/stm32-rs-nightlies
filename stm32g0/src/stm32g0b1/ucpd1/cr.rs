///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `TXMODE` reader - Type of Tx packet Writing the bitfield triggers the action as follows, depending on the value: Others: invalid From V1.1 of the USB PD specification, there is a counter defined for the duration of the BIST Carrier Mode 2. To quit this mode correctly (after the "tBISTContMode" delay), disable the peripheral (UCPDEN = 0).
pub type TXMODE_R = crate::FieldReader;
///Field `TXMODE` writer - Type of Tx packet Writing the bitfield triggers the action as follows, depending on the value: Others: invalid From V1.1 of the USB PD specification, there is a counter defined for the duration of the BIST Carrier Mode 2. To quit this mode correctly (after the "tBISTContMode" delay), disable the peripheral (UCPDEN = 0).
pub type TXMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `TXSEND` reader - Command to send a Tx packet The bit is cleared by hardware as soon as the packet transmission begins or is discarded.
pub type TXSEND_R = crate::BitReader;
///Field `TXSEND` writer - Command to send a Tx packet The bit is cleared by hardware as soon as the packet transmission begins or is discarded.
pub type TXSEND_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXHRST` reader - Command to send a Tx Hard Reset The bit is cleared by hardware as soon as the message transmission begins or is discarded.
pub type TXHRST_R = crate::BitReader;
///Field `TXHRST` writer - Command to send a Tx Hard Reset The bit is cleared by hardware as soon as the message transmission begins or is discarded.
pub type TXHRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXMODE` reader - Receiver mode Determines the mode of the receiver. When the bit is set, RXORDSET behaves normally, RXDR no longer receives bytes yet the CRC checking still proceeds as for a normal message.
pub type RXMODE_R = crate::BitReader;
///Field `RXMODE` writer - Receiver mode Determines the mode of the receiver. When the bit is set, RXORDSET behaves normally, RXDR no longer receives bytes yet the CRC checking still proceeds as for a normal message.
pub type RXMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PHYRXEN` reader - USB Power Delivery receiver enable Both CC1 and CC2 receivers are disabled when the bit is cleared. Only the CC receiver selected via the PHYCCSEL bit is enabled when the bit is set.
pub type PHYRXEN_R = crate::BitReader;
///Field `PHYRXEN` writer - USB Power Delivery receiver enable Both CC1 and CC2 receivers are disabled when the bit is cleared. Only the CC receiver selected via the PHYCCSEL bit is enabled when the bit is set.
pub type PHYRXEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PHYCCSEL` reader - CC1/CC2 line selector for USB Power Delivery signaling The selection depends on the cable orientation as discovered at attach.
pub type PHYCCSEL_R = crate::BitReader;
///Field `PHYCCSEL` writer - CC1/CC2 line selector for USB Power Delivery signaling The selection depends on the cable orientation as discovered at attach.
pub type PHYCCSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ANASUBMODE` reader - Analog PHY sub-mode Refer to TYPEC_VSTATE_CCx for the effect of this bitfield.
pub type ANASUBMODE_R = crate::FieldReader;
///Field `ANASUBMODE` writer - Analog PHY sub-mode Refer to TYPEC_VSTATE_CCx for the effect of this bitfield.
pub type ANASUBMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `ANAMODE` reader - Analog PHY operating mode The bit takes effect upon setting the UCPDx_STROBE bit of the SYS_CONFIG register. The use of CC1 and CC2 depends on CCENABLE. Refer to ANAMODE, ANASUBMODE and link with TYPEC_VSTATE_CCx for the effect of this bitfield in conjunction with ANASUBMODE\[1:0\].
pub type ANAMODE_R = crate::BitReader;
///Field `ANAMODE` writer - Analog PHY operating mode The bit takes effect upon setting the UCPDx_STROBE bit of the SYS_CONFIG register. The use of CC1 and CC2 depends on CCENABLE. Refer to ANAMODE, ANASUBMODE and link with TYPEC_VSTATE_CCx for the effect of this bitfield in conjunction with ANASUBMODE\[1:0\].
pub type ANAMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Field `CCENABLE` reader - CC line enable This bitfield enables CC1 and CC2 line analog PHYs (pull-ups and pull-downs) according to ANAMODE and ANASUBMODE\[1:0\]
setting. A single line PHY can be enabled when, for example, the other line is driven by VCONN via an external VCONN switch. Enabling both PHYs is the normal usage for sink/source.*/
pub type CCENABLE_R = crate::FieldReader;
/**Field `CCENABLE` writer - CC line enable This bitfield enables CC1 and CC2 line analog PHYs (pull-ups and pull-downs) according to ANAMODE and ANASUBMODE\[1:0\]
setting. A single line PHY can be enabled when, for example, the other line is driven by VCONN via an external VCONN switch. Enabling both PHYs is the normal usage for sink/source.*/
pub type CCENABLE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `CC1VCONNEN` reader - VCONN switch enable for CC1
pub type CC1VCONNEN_R = crate::BitReader;
///Field `CC1VCONNEN` writer - VCONN switch enable for CC1
pub type CC1VCONNEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC2VCONNEN` reader - VCONN switch enable for CC2
pub type CC2VCONNEN_R = crate::BitReader;
///Field `CC2VCONNEN` writer - VCONN switch enable for CC2
pub type CC2VCONNEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBATTEN` reader - Dead battery function enable The bit takes effect upon setting the USBPDstrobe bit of the SYS_CONFIG register. Dead battery function only operates if the external circuit is appropriately configured.
pub type DBATTEN_R = crate::BitReader;
///Field `DBATTEN` writer - Dead battery function enable The bit takes effect upon setting the USBPDstrobe bit of the SYS_CONFIG register. Dead battery function only operates if the external circuit is appropriately configured.
pub type DBATTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FRSRXEN` reader - FRS event detection enable Setting the bit enables FRS Rx event (FRSEVT) detection on the CC line selected through the PHYCCSEL bit. 0: Disable Clear the bit when the device is attached to an FRS-incapable source/sink.
pub type FRSRXEN_R = crate::BitReader;
///Field `FRSRXEN` writer - FRS event detection enable Setting the bit enables FRS Rx event (FRSEVT) detection on the CC line selected through the PHYCCSEL bit. 0: Disable Clear the bit when the device is attached to an FRS-incapable source/sink.
pub type FRSRXEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FRSTX` reader - FRS Tx signaling enable. Setting the bit enables FRS Tx signaling. The bit is cleared by hardware after a delay respecting the USB Power Delivery specification Revision 3.0.
pub type FRSTX_R = crate::BitReader;
///Field `FRSTX` writer - FRS Tx signaling enable. Setting the bit enables FRS Tx signaling. The bit is cleared by hardware after a delay respecting the USB Power Delivery specification Revision 3.0.
pub type FRSTX_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Field `RDCH` reader - Rdch condition drive The bit drives Rdch condition on the CC line selected through the PHYCCSEL bit (thus associated with VCONN), by remaining set during the source-only UnattachedWait.SRC state, to respect the Type-C state. Refer to "USB Type-C ECN for Source VCONN Discharge". The CCENABLE\[1:0\]
bitfield must be set accordingly, too. Changing the bit value only takes effect upon setting the UCPDx_STROBE bit of the SYSCFG_CFGR1 register.*/
pub type RDCH_R = crate::BitReader;
/**Field `RDCH` writer - Rdch condition drive The bit drives Rdch condition on the CC line selected through the PHYCCSEL bit (thus associated with VCONN), by remaining set during the source-only UnattachedWait.SRC state, to respect the Type-C state. Refer to "USB Type-C ECN for Source VCONN Discharge". The CCENABLE\[1:0\]
bitfield must be set accordingly, too. Changing the bit value only takes effect upon setting the UCPDx_STROBE bit of the SYSCFG_CFGR1 register.*/
pub type RDCH_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC1TCDIS` reader - CC1 Type-C detector disable The bit disables the Type-C detector on the CC1 line. When enabled, the Type-C detector for CC1 is configured through ANAMODE and ANASUBMODE\[1:0\].
pub type CC1TCDIS_R = crate::BitReader;
///Field `CC1TCDIS` writer - CC1 Type-C detector disable The bit disables the Type-C detector on the CC1 line. When enabled, the Type-C detector for CC1 is configured through ANAMODE and ANASUBMODE\[1:0\].
pub type CC1TCDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC2TCDIS` reader - CC2 Type-C detector disable The bit disables the Type-C detector on the CC2 line. When enabled, the Type-C detector for CC2 is configured through ANAMODE and ANASUBMODE\[1:0\].
pub type CC2TCDIS_R = crate::BitReader;
///Field `CC2TCDIS` writer - CC2 Type-C detector disable The bit disables the Type-C detector on the CC2 line. When enabled, the Type-C detector for CC2 is configured through ANAMODE and ANASUBMODE\[1:0\].
pub type CC2TCDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - Type of Tx packet Writing the bitfield triggers the action as follows, depending on the value: Others: invalid From V1.1 of the USB PD specification, there is a counter defined for the duration of the BIST Carrier Mode 2. To quit this mode correctly (after the "tBISTContMode" delay), disable the peripheral (UCPDEN = 0).
    #[inline(always)]
    pub fn txmode(&self) -> TXMODE_R {
        TXMODE_R::new((self.bits & 3) as u8)
    }
    ///Bit 2 - Command to send a Tx packet The bit is cleared by hardware as soon as the packet transmission begins or is discarded.
    #[inline(always)]
    pub fn txsend(&self) -> TXSEND_R {
        TXSEND_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Command to send a Tx Hard Reset The bit is cleared by hardware as soon as the message transmission begins or is discarded.
    #[inline(always)]
    pub fn txhrst(&self) -> TXHRST_R {
        TXHRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Receiver mode Determines the mode of the receiver. When the bit is set, RXORDSET behaves normally, RXDR no longer receives bytes yet the CRC checking still proceeds as for a normal message.
    #[inline(always)]
    pub fn rxmode(&self) -> RXMODE_R {
        RXMODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - USB Power Delivery receiver enable Both CC1 and CC2 receivers are disabled when the bit is cleared. Only the CC receiver selected via the PHYCCSEL bit is enabled when the bit is set.
    #[inline(always)]
    pub fn phyrxen(&self) -> PHYRXEN_R {
        PHYRXEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - CC1/CC2 line selector for USB Power Delivery signaling The selection depends on the cable orientation as discovered at attach.
    #[inline(always)]
    pub fn phyccsel(&self) -> PHYCCSEL_R {
        PHYCCSEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bits 7:8 - Analog PHY sub-mode Refer to TYPEC_VSTATE_CCx for the effect of this bitfield.
    #[inline(always)]
    pub fn anasubmode(&self) -> ANASUBMODE_R {
        ANASUBMODE_R::new(((self.bits >> 7) & 3) as u8)
    }
    ///Bit 9 - Analog PHY operating mode The bit takes effect upon setting the UCPDx_STROBE bit of the SYS_CONFIG register. The use of CC1 and CC2 depends on CCENABLE. Refer to ANAMODE, ANASUBMODE and link with TYPEC_VSTATE_CCx for the effect of this bitfield in conjunction with ANASUBMODE\[1:0\].
    #[inline(always)]
    pub fn anamode(&self) -> ANAMODE_R {
        ANAMODE_R::new(((self.bits >> 9) & 1) != 0)
    }
    /**Bits 10:11 - CC line enable This bitfield enables CC1 and CC2 line analog PHYs (pull-ups and pull-downs) according to ANAMODE and ANASUBMODE\[1:0\]
    setting. A single line PHY can be enabled when, for example, the other line is driven by VCONN via an external VCONN switch. Enabling both PHYs is the normal usage for sink/source.*/
    #[inline(always)]
    pub fn ccenable(&self) -> CCENABLE_R {
        CCENABLE_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bit 13 - VCONN switch enable for CC1
    #[inline(always)]
    pub fn cc1vconnen(&self) -> CC1VCONNEN_R {
        CC1VCONNEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - VCONN switch enable for CC2
    #[inline(always)]
    pub fn cc2vconnen(&self) -> CC2VCONNEN_R {
        CC2VCONNEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Dead battery function enable The bit takes effect upon setting the USBPDstrobe bit of the SYS_CONFIG register. Dead battery function only operates if the external circuit is appropriately configured.
    #[inline(always)]
    pub fn dbatten(&self) -> DBATTEN_R {
        DBATTEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - FRS event detection enable Setting the bit enables FRS Rx event (FRSEVT) detection on the CC line selected through the PHYCCSEL bit. 0: Disable Clear the bit when the device is attached to an FRS-incapable source/sink.
    #[inline(always)]
    pub fn frsrxen(&self) -> FRSRXEN_R {
        FRSRXEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - FRS Tx signaling enable. Setting the bit enables FRS Tx signaling. The bit is cleared by hardware after a delay respecting the USB Power Delivery specification Revision 3.0.
    #[inline(always)]
    pub fn frstx(&self) -> FRSTX_R {
        FRSTX_R::new(((self.bits >> 17) & 1) != 0)
    }
    /**Bit 18 - Rdch condition drive The bit drives Rdch condition on the CC line selected through the PHYCCSEL bit (thus associated with VCONN), by remaining set during the source-only UnattachedWait.SRC state, to respect the Type-C state. Refer to "USB Type-C ECN for Source VCONN Discharge". The CCENABLE\[1:0\]
    bitfield must be set accordingly, too. Changing the bit value only takes effect upon setting the UCPDx_STROBE bit of the SYSCFG_CFGR1 register.*/
    #[inline(always)]
    pub fn rdch(&self) -> RDCH_R {
        RDCH_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 20 - CC1 Type-C detector disable The bit disables the Type-C detector on the CC1 line. When enabled, the Type-C detector for CC1 is configured through ANAMODE and ANASUBMODE\[1:0\].
    #[inline(always)]
    pub fn cc1tcdis(&self) -> CC1TCDIS_R {
        CC1TCDIS_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - CC2 Type-C detector disable The bit disables the Type-C detector on the CC2 line. When enabled, the Type-C detector for CC2 is configured through ANAMODE and ANASUBMODE\[1:0\].
    #[inline(always)]
    pub fn cc2tcdis(&self) -> CC2TCDIS_R {
        CC2TCDIS_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("txmode", &self.txmode())
            .field("txsend", &self.txsend())
            .field("txhrst", &self.txhrst())
            .field("rxmode", &self.rxmode())
            .field("phyrxen", &self.phyrxen())
            .field("phyccsel", &self.phyccsel())
            .field("anasubmode", &self.anasubmode())
            .field("anamode", &self.anamode())
            .field("ccenable", &self.ccenable())
            .field("cc1vconnen", &self.cc1vconnen())
            .field("cc2vconnen", &self.cc2vconnen())
            .field("dbatten", &self.dbatten())
            .field("frsrxen", &self.frsrxen())
            .field("frstx", &self.frstx())
            .field("rdch", &self.rdch())
            .field("cc1tcdis", &self.cc1tcdis())
            .field("cc2tcdis", &self.cc2tcdis())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Type of Tx packet Writing the bitfield triggers the action as follows, depending on the value: Others: invalid From V1.1 of the USB PD specification, there is a counter defined for the duration of the BIST Carrier Mode 2. To quit this mode correctly (after the "tBISTContMode" delay), disable the peripheral (UCPDEN = 0).
    #[inline(always)]
    #[must_use]
    pub fn txmode(&mut self) -> TXMODE_W<CRrs> {
        TXMODE_W::new(self, 0)
    }
    ///Bit 2 - Command to send a Tx packet The bit is cleared by hardware as soon as the packet transmission begins or is discarded.
    #[inline(always)]
    #[must_use]
    pub fn txsend(&mut self) -> TXSEND_W<CRrs> {
        TXSEND_W::new(self, 2)
    }
    ///Bit 3 - Command to send a Tx Hard Reset The bit is cleared by hardware as soon as the message transmission begins or is discarded.
    #[inline(always)]
    #[must_use]
    pub fn txhrst(&mut self) -> TXHRST_W<CRrs> {
        TXHRST_W::new(self, 3)
    }
    ///Bit 4 - Receiver mode Determines the mode of the receiver. When the bit is set, RXORDSET behaves normally, RXDR no longer receives bytes yet the CRC checking still proceeds as for a normal message.
    #[inline(always)]
    #[must_use]
    pub fn rxmode(&mut self) -> RXMODE_W<CRrs> {
        RXMODE_W::new(self, 4)
    }
    ///Bit 5 - USB Power Delivery receiver enable Both CC1 and CC2 receivers are disabled when the bit is cleared. Only the CC receiver selected via the PHYCCSEL bit is enabled when the bit is set.
    #[inline(always)]
    #[must_use]
    pub fn phyrxen(&mut self) -> PHYRXEN_W<CRrs> {
        PHYRXEN_W::new(self, 5)
    }
    ///Bit 6 - CC1/CC2 line selector for USB Power Delivery signaling The selection depends on the cable orientation as discovered at attach.
    #[inline(always)]
    #[must_use]
    pub fn phyccsel(&mut self) -> PHYCCSEL_W<CRrs> {
        PHYCCSEL_W::new(self, 6)
    }
    ///Bits 7:8 - Analog PHY sub-mode Refer to TYPEC_VSTATE_CCx for the effect of this bitfield.
    #[inline(always)]
    #[must_use]
    pub fn anasubmode(&mut self) -> ANASUBMODE_W<CRrs> {
        ANASUBMODE_W::new(self, 7)
    }
    ///Bit 9 - Analog PHY operating mode The bit takes effect upon setting the UCPDx_STROBE bit of the SYS_CONFIG register. The use of CC1 and CC2 depends on CCENABLE. Refer to ANAMODE, ANASUBMODE and link with TYPEC_VSTATE_CCx for the effect of this bitfield in conjunction with ANASUBMODE\[1:0\].
    #[inline(always)]
    #[must_use]
    pub fn anamode(&mut self) -> ANAMODE_W<CRrs> {
        ANAMODE_W::new(self, 9)
    }
    /**Bits 10:11 - CC line enable This bitfield enables CC1 and CC2 line analog PHYs (pull-ups and pull-downs) according to ANAMODE and ANASUBMODE\[1:0\]
    setting. A single line PHY can be enabled when, for example, the other line is driven by VCONN via an external VCONN switch. Enabling both PHYs is the normal usage for sink/source.*/
    #[inline(always)]
    #[must_use]
    pub fn ccenable(&mut self) -> CCENABLE_W<CRrs> {
        CCENABLE_W::new(self, 10)
    }
    ///Bit 13 - VCONN switch enable for CC1
    #[inline(always)]
    #[must_use]
    pub fn cc1vconnen(&mut self) -> CC1VCONNEN_W<CRrs> {
        CC1VCONNEN_W::new(self, 13)
    }
    ///Bit 14 - VCONN switch enable for CC2
    #[inline(always)]
    #[must_use]
    pub fn cc2vconnen(&mut self) -> CC2VCONNEN_W<CRrs> {
        CC2VCONNEN_W::new(self, 14)
    }
    ///Bit 15 - Dead battery function enable The bit takes effect upon setting the USBPDstrobe bit of the SYS_CONFIG register. Dead battery function only operates if the external circuit is appropriately configured.
    #[inline(always)]
    #[must_use]
    pub fn dbatten(&mut self) -> DBATTEN_W<CRrs> {
        DBATTEN_W::new(self, 15)
    }
    ///Bit 16 - FRS event detection enable Setting the bit enables FRS Rx event (FRSEVT) detection on the CC line selected through the PHYCCSEL bit. 0: Disable Clear the bit when the device is attached to an FRS-incapable source/sink.
    #[inline(always)]
    #[must_use]
    pub fn frsrxen(&mut self) -> FRSRXEN_W<CRrs> {
        FRSRXEN_W::new(self, 16)
    }
    ///Bit 17 - FRS Tx signaling enable. Setting the bit enables FRS Tx signaling. The bit is cleared by hardware after a delay respecting the USB Power Delivery specification Revision 3.0.
    #[inline(always)]
    #[must_use]
    pub fn frstx(&mut self) -> FRSTX_W<CRrs> {
        FRSTX_W::new(self, 17)
    }
    /**Bit 18 - Rdch condition drive The bit drives Rdch condition on the CC line selected through the PHYCCSEL bit (thus associated with VCONN), by remaining set during the source-only UnattachedWait.SRC state, to respect the Type-C state. Refer to "USB Type-C ECN for Source VCONN Discharge". The CCENABLE\[1:0\]
    bitfield must be set accordingly, too. Changing the bit value only takes effect upon setting the UCPDx_STROBE bit of the SYSCFG_CFGR1 register.*/
    #[inline(always)]
    #[must_use]
    pub fn rdch(&mut self) -> RDCH_W<CRrs> {
        RDCH_W::new(self, 18)
    }
    ///Bit 20 - CC1 Type-C detector disable The bit disables the Type-C detector on the CC1 line. When enabled, the Type-C detector for CC1 is configured through ANAMODE and ANASUBMODE\[1:0\].
    #[inline(always)]
    #[must_use]
    pub fn cc1tcdis(&mut self) -> CC1TCDIS_W<CRrs> {
        CC1TCDIS_W::new(self, 20)
    }
    ///Bit 21 - CC2 Type-C detector disable The bit disables the Type-C detector on the CC2 line. When enabled, the Type-C detector for CC2 is configured through ANAMODE and ANASUBMODE\[1:0\].
    #[inline(always)]
    #[must_use]
    pub fn cc2tcdis(&mut self) -> CC2TCDIS_W<CRrs> {
        CC2TCDIS_W::new(self, 21)
    }
}
/**UCPD control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G0B1.html#UCPD1:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0;
}
