///Register `HPRT` reader
pub type R = crate::R<HPRTrs>;
///Register `HPRT` writer
pub type W = crate::W<HPRTrs>;
///Field `PCSTS` reader - Port connect status
pub type PCSTS_R = crate::BitReader;
///Field `PCDET` reader - Port connect detected The core sets this bit when a device connection is detected to trigger an interrupt to the application using the host port interrupt bit in the core interrupt register (HPRTINT bit in OTG_GINTSTS). The application must write a 1 to this bit to clear the interrupt.
pub type PCDET_R = crate::BitReader;
///Field `PCDET` writer - Port connect detected The core sets this bit when a device connection is detected to trigger an interrupt to the application using the host port interrupt bit in the core interrupt register (HPRTINT bit in OTG_GINTSTS). The application must write a 1 to this bit to clear the interrupt.
pub type PCDET_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PENA` reader - Port enable A port is enabled only by the core after a reset sequence, and is disabled by an overcurrent condition, a disconnect condition, or by the application clearing this bit. The application cannot set this bit by a register write. It can only clear it to disable the port. This bit does not trigger any interrupt to the application.
pub type PENA_R = crate::BitReader;
///Field `PENA` writer - Port enable A port is enabled only by the core after a reset sequence, and is disabled by an overcurrent condition, a disconnect condition, or by the application clearing this bit. The application cannot set this bit by a register write. It can only clear it to disable the port. This bit does not trigger any interrupt to the application.
pub type PENA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PENCHNG` reader - Port enable/disable change The core sets this bit when the status of the port enable bit 2 in this register changes.
pub type PENCHNG_R = crate::BitReader;
///Field `PENCHNG` writer - Port enable/disable change The core sets this bit when the status of the port enable bit 2 in this register changes.
pub type PENCHNG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `POCA` reader - Port overcurrent active Indicates the overcurrent condition of the port.
pub type POCA_R = crate::BitReader;
///Field `POCCHNG` reader - Port overcurrent change The core sets this bit when the status of the port overcurrent active bit (bit 4) in this register changes.
pub type POCCHNG_R = crate::BitReader;
///Field `POCCHNG` writer - Port overcurrent change The core sets this bit when the status of the port overcurrent active bit (bit 4) in this register changes.
pub type POCCHNG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRES` reader - Port resume The application sets this bit to drive resume signaling on the port. The core continues to drive the resume signal until the application clears this bit. If the core detects a USB remote wakeup sequence, as indicated by the port resume/remote wakeup detected interrupt bit of the core interrupt register (WKUPINT bit in OTG_GINTSTS), the core starts driving resume signaling without application intervention and clears this bit when it detects a disconnect condition. The read value of this bit indicates whether the core is currently driving resume signaling. When LPM is enabled and the core is in L1 state, the behavior of this bit is as follow: 1. The application sets this bit to drive resume signaling on the port. 2. The core continues to drive the resume signal until a predetermined time specified in BESLTHRS\[3:0\] field of OTG_GLPMCFG register. 3. If the core detects a USB remote wakeup sequence, as indicated by the port L1Resume/Remote L1Wakeup detected interrupt bit of the core interrupt register (WKUPINT in OTG_GINTSTS), the core starts driving resume signaling without application intervention and clears this bit at the end of resume.This bit can be set or cleared by both the core and the application. This bit is cleared by the core even if there is no device connected to the host.
pub type PRES_R = crate::BitReader;
///Field `PRES` writer - Port resume The application sets this bit to drive resume signaling on the port. The core continues to drive the resume signal until the application clears this bit. If the core detects a USB remote wakeup sequence, as indicated by the port resume/remote wakeup detected interrupt bit of the core interrupt register (WKUPINT bit in OTG_GINTSTS), the core starts driving resume signaling without application intervention and clears this bit when it detects a disconnect condition. The read value of this bit indicates whether the core is currently driving resume signaling. When LPM is enabled and the core is in L1 state, the behavior of this bit is as follow: 1. The application sets this bit to drive resume signaling on the port. 2. The core continues to drive the resume signal until a predetermined time specified in BESLTHRS\[3:0\] field of OTG_GLPMCFG register. 3. If the core detects a USB remote wakeup sequence, as indicated by the port L1Resume/Remote L1Wakeup detected interrupt bit of the core interrupt register (WKUPINT in OTG_GINTSTS), the core starts driving resume signaling without application intervention and clears this bit at the end of resume.This bit can be set or cleared by both the core and the application. This bit is cleared by the core even if there is no device connected to the host.
pub type PRES_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PSUSP` reader - Port suspend The application sets this bit to put this port in suspend mode. The core only stops sending SOFs when this is set. To stop the PHY clock, the application must set the port clock stop bit, which asserts the suspend input pin of the PHY. The read value of this bit reflects the current suspend status of the port. This bit is cleared by the core after a remote wakeup signal is detected or the application sets the port reset bit or port resume bit in this register or the resume/remote wakeup detected interrupt bit or disconnect detected interrupt bit in the core interrupt register (WKUPINT or DISCINT in OTG_GINTSTS, respectively).
pub type PSUSP_R = crate::BitReader;
///Field `PSUSP` writer - Port suspend The application sets this bit to put this port in suspend mode. The core only stops sending SOFs when this is set. To stop the PHY clock, the application must set the port clock stop bit, which asserts the suspend input pin of the PHY. The read value of this bit reflects the current suspend status of the port. This bit is cleared by the core after a remote wakeup signal is detected or the application sets the port reset bit or port resume bit in this register or the resume/remote wakeup detected interrupt bit or disconnect detected interrupt bit in the core interrupt register (WKUPINT or DISCINT in OTG_GINTSTS, respectively).
pub type PSUSP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRST` reader - Port reset When the application sets this bit, a reset sequence is started on this port. The application must time the reset period and clear this bit after the reset sequence is complete. The application must leave this bit set for a minimum duration of at least 10 ms to start a reset on the port. The application can leave it set for another 10 ms in addition to the required minimum duration, before clearing the bit, even though there is no maximum limit set by the USB standard. High speed: 50 ms Full speed/Low speed: 10 ms
pub type PRST_R = crate::BitReader;
///Field `PRST` writer - Port reset When the application sets this bit, a reset sequence is started on this port. The application must time the reset period and clear this bit after the reset sequence is complete. The application must leave this bit set for a minimum duration of at least 10 ms to start a reset on the port. The application can leave it set for another 10 ms in addition to the required minimum duration, before clearing the bit, even though there is no maximum limit set by the USB standard. High speed: 50 ms Full speed/Low speed: 10 ms
pub type PRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLSTS` reader - Port line status Indicates the current logic level USB data lines Bit 10: Logic level of OTG_DP Bit 11: Logic level of OTG_DM
pub type PLSTS_R = crate::FieldReader;
///Field `PPWR` reader - Port power The application uses this field to control power to this port, and the core clears this bit on an overcurrent condition.
pub type PPWR_R = crate::BitReader;
///Field `PPWR` writer - Port power The application uses this field to control power to this port, and the core clears this bit on an overcurrent condition.
pub type PPWR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PTCTL` reader - Port test control The application writes a nonzero value to this field to put the port into a Test mode, and the corresponding pattern is signaled on the port. Others: Reserved
pub type PTCTL_R = crate::FieldReader;
///Field `PTCTL` writer - Port test control The application writes a nonzero value to this field to put the port into a Test mode, and the corresponding pattern is signaled on the port. Others: Reserved
pub type PTCTL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `PSPD` reader - Port speed Indicates the speed of the device attached to this port.
pub type PSPD_R = crate::FieldReader;
impl R {
    ///Bit 0 - Port connect status
    #[inline(always)]
    pub fn pcsts(&self) -> PCSTS_R {
        PCSTS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Port connect detected The core sets this bit when a device connection is detected to trigger an interrupt to the application using the host port interrupt bit in the core interrupt register (HPRTINT bit in OTG_GINTSTS). The application must write a 1 to this bit to clear the interrupt.
    #[inline(always)]
    pub fn pcdet(&self) -> PCDET_R {
        PCDET_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Port enable A port is enabled only by the core after a reset sequence, and is disabled by an overcurrent condition, a disconnect condition, or by the application clearing this bit. The application cannot set this bit by a register write. It can only clear it to disable the port. This bit does not trigger any interrupt to the application.
    #[inline(always)]
    pub fn pena(&self) -> PENA_R {
        PENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Port enable/disable change The core sets this bit when the status of the port enable bit 2 in this register changes.
    #[inline(always)]
    pub fn penchng(&self) -> PENCHNG_R {
        PENCHNG_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Port overcurrent active Indicates the overcurrent condition of the port.
    #[inline(always)]
    pub fn poca(&self) -> POCA_R {
        POCA_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Port overcurrent change The core sets this bit when the status of the port overcurrent active bit (bit 4) in this register changes.
    #[inline(always)]
    pub fn pocchng(&self) -> POCCHNG_R {
        POCCHNG_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Port resume The application sets this bit to drive resume signaling on the port. The core continues to drive the resume signal until the application clears this bit. If the core detects a USB remote wakeup sequence, as indicated by the port resume/remote wakeup detected interrupt bit of the core interrupt register (WKUPINT bit in OTG_GINTSTS), the core starts driving resume signaling without application intervention and clears this bit when it detects a disconnect condition. The read value of this bit indicates whether the core is currently driving resume signaling. When LPM is enabled and the core is in L1 state, the behavior of this bit is as follow: 1. The application sets this bit to drive resume signaling on the port. 2. The core continues to drive the resume signal until a predetermined time specified in BESLTHRS\[3:0\] field of OTG_GLPMCFG register. 3. If the core detects a USB remote wakeup sequence, as indicated by the port L1Resume/Remote L1Wakeup detected interrupt bit of the core interrupt register (WKUPINT in OTG_GINTSTS), the core starts driving resume signaling without application intervention and clears this bit at the end of resume.This bit can be set or cleared by both the core and the application. This bit is cleared by the core even if there is no device connected to the host.
    #[inline(always)]
    pub fn pres(&self) -> PRES_R {
        PRES_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Port suspend The application sets this bit to put this port in suspend mode. The core only stops sending SOFs when this is set. To stop the PHY clock, the application must set the port clock stop bit, which asserts the suspend input pin of the PHY. The read value of this bit reflects the current suspend status of the port. This bit is cleared by the core after a remote wakeup signal is detected or the application sets the port reset bit or port resume bit in this register or the resume/remote wakeup detected interrupt bit or disconnect detected interrupt bit in the core interrupt register (WKUPINT or DISCINT in OTG_GINTSTS, respectively).
    #[inline(always)]
    pub fn psusp(&self) -> PSUSP_R {
        PSUSP_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Port reset When the application sets this bit, a reset sequence is started on this port. The application must time the reset period and clear this bit after the reset sequence is complete. The application must leave this bit set for a minimum duration of at least 10 ms to start a reset on the port. The application can leave it set for another 10 ms in addition to the required minimum duration, before clearing the bit, even though there is no maximum limit set by the USB standard. High speed: 50 ms Full speed/Low speed: 10 ms
    #[inline(always)]
    pub fn prst(&self) -> PRST_R {
        PRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 10:11 - Port line status Indicates the current logic level USB data lines Bit 10: Logic level of OTG_DP Bit 11: Logic level of OTG_DM
    #[inline(always)]
    pub fn plsts(&self) -> PLSTS_R {
        PLSTS_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bit 12 - Port power The application uses this field to control power to this port, and the core clears this bit on an overcurrent condition.
    #[inline(always)]
    pub fn ppwr(&self) -> PPWR_R {
        PPWR_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 13:16 - Port test control The application writes a nonzero value to this field to put the port into a Test mode, and the corresponding pattern is signaled on the port. Others: Reserved
    #[inline(always)]
    pub fn ptctl(&self) -> PTCTL_R {
        PTCTL_R::new(((self.bits >> 13) & 0x0f) as u8)
    }
    ///Bits 17:18 - Port speed Indicates the speed of the device attached to this port.
    #[inline(always)]
    pub fn pspd(&self) -> PSPD_R {
        PSPD_R::new(((self.bits >> 17) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HPRT")
            .field("pcsts", &self.pcsts())
            .field("pcdet", &self.pcdet())
            .field("pena", &self.pena())
            .field("penchng", &self.penchng())
            .field("poca", &self.poca())
            .field("pocchng", &self.pocchng())
            .field("pres", &self.pres())
            .field("psusp", &self.psusp())
            .field("prst", &self.prst())
            .field("plsts", &self.plsts())
            .field("ppwr", &self.ppwr())
            .field("ptctl", &self.ptctl())
            .field("pspd", &self.pspd())
            .finish()
    }
}
impl W {
    ///Bit 1 - Port connect detected The core sets this bit when a device connection is detected to trigger an interrupt to the application using the host port interrupt bit in the core interrupt register (HPRTINT bit in OTG_GINTSTS). The application must write a 1 to this bit to clear the interrupt.
    #[inline(always)]
    pub fn pcdet(&mut self) -> PCDET_W<'_, HPRTrs> {
        PCDET_W::new(self, 1)
    }
    ///Bit 2 - Port enable A port is enabled only by the core after a reset sequence, and is disabled by an overcurrent condition, a disconnect condition, or by the application clearing this bit. The application cannot set this bit by a register write. It can only clear it to disable the port. This bit does not trigger any interrupt to the application.
    #[inline(always)]
    pub fn pena(&mut self) -> PENA_W<'_, HPRTrs> {
        PENA_W::new(self, 2)
    }
    ///Bit 3 - Port enable/disable change The core sets this bit when the status of the port enable bit 2 in this register changes.
    #[inline(always)]
    pub fn penchng(&mut self) -> PENCHNG_W<'_, HPRTrs> {
        PENCHNG_W::new(self, 3)
    }
    ///Bit 5 - Port overcurrent change The core sets this bit when the status of the port overcurrent active bit (bit 4) in this register changes.
    #[inline(always)]
    pub fn pocchng(&mut self) -> POCCHNG_W<'_, HPRTrs> {
        POCCHNG_W::new(self, 5)
    }
    ///Bit 6 - Port resume The application sets this bit to drive resume signaling on the port. The core continues to drive the resume signal until the application clears this bit. If the core detects a USB remote wakeup sequence, as indicated by the port resume/remote wakeup detected interrupt bit of the core interrupt register (WKUPINT bit in OTG_GINTSTS), the core starts driving resume signaling without application intervention and clears this bit when it detects a disconnect condition. The read value of this bit indicates whether the core is currently driving resume signaling. When LPM is enabled and the core is in L1 state, the behavior of this bit is as follow: 1. The application sets this bit to drive resume signaling on the port. 2. The core continues to drive the resume signal until a predetermined time specified in BESLTHRS\[3:0\] field of OTG_GLPMCFG register. 3. If the core detects a USB remote wakeup sequence, as indicated by the port L1Resume/Remote L1Wakeup detected interrupt bit of the core interrupt register (WKUPINT in OTG_GINTSTS), the core starts driving resume signaling without application intervention and clears this bit at the end of resume.This bit can be set or cleared by both the core and the application. This bit is cleared by the core even if there is no device connected to the host.
    #[inline(always)]
    pub fn pres(&mut self) -> PRES_W<'_, HPRTrs> {
        PRES_W::new(self, 6)
    }
    ///Bit 7 - Port suspend The application sets this bit to put this port in suspend mode. The core only stops sending SOFs when this is set. To stop the PHY clock, the application must set the port clock stop bit, which asserts the suspend input pin of the PHY. The read value of this bit reflects the current suspend status of the port. This bit is cleared by the core after a remote wakeup signal is detected or the application sets the port reset bit or port resume bit in this register or the resume/remote wakeup detected interrupt bit or disconnect detected interrupt bit in the core interrupt register (WKUPINT or DISCINT in OTG_GINTSTS, respectively).
    #[inline(always)]
    pub fn psusp(&mut self) -> PSUSP_W<'_, HPRTrs> {
        PSUSP_W::new(self, 7)
    }
    ///Bit 8 - Port reset When the application sets this bit, a reset sequence is started on this port. The application must time the reset period and clear this bit after the reset sequence is complete. The application must leave this bit set for a minimum duration of at least 10 ms to start a reset on the port. The application can leave it set for another 10 ms in addition to the required minimum duration, before clearing the bit, even though there is no maximum limit set by the USB standard. High speed: 50 ms Full speed/Low speed: 10 ms
    #[inline(always)]
    pub fn prst(&mut self) -> PRST_W<'_, HPRTrs> {
        PRST_W::new(self, 8)
    }
    ///Bit 12 - Port power The application uses this field to control power to this port, and the core clears this bit on an overcurrent condition.
    #[inline(always)]
    pub fn ppwr(&mut self) -> PPWR_W<'_, HPRTrs> {
        PPWR_W::new(self, 12)
    }
    ///Bits 13:16 - Port test control The application writes a nonzero value to this field to put the port into a Test mode, and the corresponding pattern is signaled on the port. Others: Reserved
    #[inline(always)]
    pub fn ptctl(&mut self) -> PTCTL_W<'_, HPRTrs> {
        PTCTL_W::new(self, 13)
    }
}
/**OTG host port control and status register

You can [`read`](crate::Reg::read) this register and get [`hprt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hprt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#OTG_HS:HPRT)*/
pub struct HPRTrs;
impl crate::RegisterSpec for HPRTrs {
    type Ux = u32;
}
///`read()` method returns [`hprt::R`](R) reader structure
impl crate::Readable for HPRTrs {}
///`write(|w| ..)` method takes [`hprt::W`](W) writer structure
impl crate::Writable for HPRTrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HPRT to value 0
impl crate::Resettable for HPRTrs {}
