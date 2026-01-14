///Register `GUSBCFG` reader
pub type R = crate::R<GUSBCFGrs>;
///Register `GUSBCFG` writer
pub type W = crate::W<GUSBCFGrs>;
///Field `TOCAL` reader - FS timeout calibration The number of PHY clocks that the application programs in this field is added to the full-speed interpacket timeout duration in the core to account for any additional delays introduced by the PHY. This can be required, because the delay introduced by the PHY in generating the line state condition can vary from one PHY to another. The USB standard timeout value for full-speed operation is 16 to 18 (inclusive) bit times. The application must program this field based on the speed of enumeration. The number of bit times added per PHY clock is 0.25 bit times.
pub type TOCAL_R = crate::FieldReader;
///Field `TOCAL` writer - FS timeout calibration The number of PHY clocks that the application programs in this field is added to the full-speed interpacket timeout duration in the core to account for any additional delays introduced by the PHY. This can be required, because the delay introduced by the PHY in generating the line state condition can vary from one PHY to another. The USB standard timeout value for full-speed operation is 16 to 18 (inclusive) bit times. The application must program this field based on the speed of enumeration. The number of bit times added per PHY clock is 0.25 bit times.
pub type TOCAL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `TRDT` reader - USB turnaround time These bits allows to set the turnaround time in PHY clocks. They must be configured according to Table 683: TRDT values, depending on the application AHB frequency. Higher TRDT values allow stretching the USB response time to IN tokens in order to compensate for longer AHB read access latency to the data FIFO. Note: Only accessible in device mode.
pub type TRDT_R = crate::FieldReader;
///Field `TRDT` writer - USB turnaround time These bits allows to set the turnaround time in PHY clocks. They must be configured according to Table 683: TRDT values, depending on the application AHB frequency. Higher TRDT values allow stretching the USB response time to IN tokens in order to compensate for longer AHB read access latency to the data FIFO. Note: Only accessible in device mode.
pub type TRDT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `PHYLPC` reader - PHY Low-power clock select This bit selects either 480 MHz or 48 MHz (low-power) PHY mode. In FS and LS modes, the PHY can usually operate on a 48 MHz clock to save power. In 480 MHz mode, the UTMI interface operates at either 60 or 30 MHz, depending on whether the 8- or 16-bit data width is selected. In 48 MHz mode, the UTMI interface operates at 48 MHz in FS and LS modes.
pub type PHYLPC_R = crate::BitReader;
///Field `PHYLPC` writer - PHY Low-power clock select This bit selects either 480 MHz or 48 MHz (low-power) PHY mode. In FS and LS modes, the PHY can usually operate on a 48 MHz clock to save power. In 480 MHz mode, the UTMI interface operates at either 60 or 30 MHz, depending on whether the 8- or 16-bit data width is selected. In 48 MHz mode, the UTMI interface operates at 48 MHz in FS and LS modes.
pub type PHYLPC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSDPS` reader - TermSel DLine pulsing selection This bit selects utmi_termselect to drive the data line pulse during SRP (session request protocol).
pub type TSDPS_R = crate::BitReader;
///Field `TSDPS` writer - TermSel DLine pulsing selection This bit selects utmi_termselect to drive the data line pulse during SRP (session request protocol).
pub type TSDPS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FHMOD` reader - Force host mode Writing a 1 to this bit, forces the core to host mode irrespective of the OTG_ID input pin. After setting the force bit, the application must wait at least 25 ms before the change takes effect. Note: Accessible in both device and host modes.
pub type FHMOD_R = crate::BitReader;
///Field `FHMOD` writer - Force host mode Writing a 1 to this bit, forces the core to host mode irrespective of the OTG_ID input pin. After setting the force bit, the application must wait at least 25 ms before the change takes effect. Note: Accessible in both device and host modes.
pub type FHMOD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FDMOD` reader - Force device mode Writing a 1 to this bit, forces the core to device mode irrespective of the OTG_ID input pin. After setting the force bit, the application must wait at least 25 ms before the change takes effect. Note: Accessible in both device and host modes.
pub type FDMOD_R = crate::BitReader;
///Field `FDMOD` writer - Force device mode Writing a 1 to this bit, forces the core to device mode irrespective of the OTG_ID input pin. After setting the force bit, the application must wait at least 25 ms before the change takes effect. Note: Accessible in both device and host modes.
pub type FDMOD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:2 - FS timeout calibration The number of PHY clocks that the application programs in this field is added to the full-speed interpacket timeout duration in the core to account for any additional delays introduced by the PHY. This can be required, because the delay introduced by the PHY in generating the line state condition can vary from one PHY to another. The USB standard timeout value for full-speed operation is 16 to 18 (inclusive) bit times. The application must program this field based on the speed of enumeration. The number of bit times added per PHY clock is 0.25 bit times.
    #[inline(always)]
    pub fn tocal(&self) -> TOCAL_R {
        TOCAL_R::new((self.bits & 7) as u8)
    }
    ///Bits 10:13 - USB turnaround time These bits allows to set the turnaround time in PHY clocks. They must be configured according to Table 683: TRDT values, depending on the application AHB frequency. Higher TRDT values allow stretching the USB response time to IN tokens in order to compensate for longer AHB read access latency to the data FIFO. Note: Only accessible in device mode.
    #[inline(always)]
    pub fn trdt(&self) -> TRDT_R {
        TRDT_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    ///Bit 15 - PHY Low-power clock select This bit selects either 480 MHz or 48 MHz (low-power) PHY mode. In FS and LS modes, the PHY can usually operate on a 48 MHz clock to save power. In 480 MHz mode, the UTMI interface operates at either 60 or 30 MHz, depending on whether the 8- or 16-bit data width is selected. In 48 MHz mode, the UTMI interface operates at 48 MHz in FS and LS modes.
    #[inline(always)]
    pub fn phylpc(&self) -> PHYLPC_R {
        PHYLPC_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 22 - TermSel DLine pulsing selection This bit selects utmi_termselect to drive the data line pulse during SRP (session request protocol).
    #[inline(always)]
    pub fn tsdps(&self) -> TSDPS_R {
        TSDPS_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 29 - Force host mode Writing a 1 to this bit, forces the core to host mode irrespective of the OTG_ID input pin. After setting the force bit, the application must wait at least 25 ms before the change takes effect. Note: Accessible in both device and host modes.
    #[inline(always)]
    pub fn fhmod(&self) -> FHMOD_R {
        FHMOD_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Force device mode Writing a 1 to this bit, forces the core to device mode irrespective of the OTG_ID input pin. After setting the force bit, the application must wait at least 25 ms before the change takes effect. Note: Accessible in both device and host modes.
    #[inline(always)]
    pub fn fdmod(&self) -> FDMOD_R {
        FDMOD_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GUSBCFG")
            .field("tocal", &self.tocal())
            .field("trdt", &self.trdt())
            .field("phylpc", &self.phylpc())
            .field("tsdps", &self.tsdps())
            .field("fhmod", &self.fhmod())
            .field("fdmod", &self.fdmod())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - FS timeout calibration The number of PHY clocks that the application programs in this field is added to the full-speed interpacket timeout duration in the core to account for any additional delays introduced by the PHY. This can be required, because the delay introduced by the PHY in generating the line state condition can vary from one PHY to another. The USB standard timeout value for full-speed operation is 16 to 18 (inclusive) bit times. The application must program this field based on the speed of enumeration. The number of bit times added per PHY clock is 0.25 bit times.
    #[inline(always)]
    pub fn tocal(&mut self) -> TOCAL_W<'_, GUSBCFGrs> {
        TOCAL_W::new(self, 0)
    }
    ///Bits 10:13 - USB turnaround time These bits allows to set the turnaround time in PHY clocks. They must be configured according to Table 683: TRDT values, depending on the application AHB frequency. Higher TRDT values allow stretching the USB response time to IN tokens in order to compensate for longer AHB read access latency to the data FIFO. Note: Only accessible in device mode.
    #[inline(always)]
    pub fn trdt(&mut self) -> TRDT_W<'_, GUSBCFGrs> {
        TRDT_W::new(self, 10)
    }
    ///Bit 15 - PHY Low-power clock select This bit selects either 480 MHz or 48 MHz (low-power) PHY mode. In FS and LS modes, the PHY can usually operate on a 48 MHz clock to save power. In 480 MHz mode, the UTMI interface operates at either 60 or 30 MHz, depending on whether the 8- or 16-bit data width is selected. In 48 MHz mode, the UTMI interface operates at 48 MHz in FS and LS modes.
    #[inline(always)]
    pub fn phylpc(&mut self) -> PHYLPC_W<'_, GUSBCFGrs> {
        PHYLPC_W::new(self, 15)
    }
    ///Bit 22 - TermSel DLine pulsing selection This bit selects utmi_termselect to drive the data line pulse during SRP (session request protocol).
    #[inline(always)]
    pub fn tsdps(&mut self) -> TSDPS_W<'_, GUSBCFGrs> {
        TSDPS_W::new(self, 22)
    }
    ///Bit 29 - Force host mode Writing a 1 to this bit, forces the core to host mode irrespective of the OTG_ID input pin. After setting the force bit, the application must wait at least 25 ms before the change takes effect. Note: Accessible in both device and host modes.
    #[inline(always)]
    pub fn fhmod(&mut self) -> FHMOD_W<'_, GUSBCFGrs> {
        FHMOD_W::new(self, 29)
    }
    ///Bit 30 - Force device mode Writing a 1 to this bit, forces the core to device mode irrespective of the OTG_ID input pin. After setting the force bit, the application must wait at least 25 ms before the change takes effect. Note: Accessible in both device and host modes.
    #[inline(always)]
    pub fn fdmod(&mut self) -> FDMOD_W<'_, GUSBCFGrs> {
        FDMOD_W::new(self, 30)
    }
}
/**OTG USB configuration register

You can [`read`](crate::Reg::read) this register and get [`gusbcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gusbcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#OTG_HS:GUSBCFG)*/
pub struct GUSBCFGrs;
impl crate::RegisterSpec for GUSBCFGrs {
    type Ux = u32;
}
///`read()` method returns [`gusbcfg::R`](R) reader structure
impl crate::Readable for GUSBCFGrs {}
///`write(|w| ..)` method takes [`gusbcfg::W`](W) writer structure
impl crate::Writable for GUSBCFGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GUSBCFG to value 0x1400
impl crate::Resettable for GUSBCFGrs {
    const RESET_VALUE: u32 = 0x1400;
}
