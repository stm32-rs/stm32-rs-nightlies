///Register `HCCHAR0` reader
pub type R = crate::R<HCCHAR0rs>;
///Register `HCCHAR0` writer
pub type W = crate::W<HCCHAR0rs>;
///Field `MPSIZ` reader - Maximum packet size Indicates the maximum packet size of the associated endpoint.
pub type MPSIZ_R = crate::FieldReader<u16>;
///Field `MPSIZ` writer - Maximum packet size Indicates the maximum packet size of the associated endpoint.
pub type MPSIZ_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
///Field `EPNUM` reader - Endpoint number Indicates the endpoint number on the device serving as the data source or sink.
pub type EPNUM_R = crate::FieldReader;
///Field `EPNUM` writer - Endpoint number Indicates the endpoint number on the device serving as the data source or sink.
pub type EPNUM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `EPDIR` reader - Endpoint direction Indicates whether the transaction is IN or OUT.
pub type EPDIR_R = crate::BitReader;
///Field `EPDIR` writer - Endpoint direction Indicates whether the transaction is IN or OUT.
pub type EPDIR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSDEV` reader - Low-speed device This field is set by the application to indicate that this channel is communicating to a low-speed device.
pub type LSDEV_R = crate::BitReader;
///Field `LSDEV` writer - Low-speed device This field is set by the application to indicate that this channel is communicating to a low-speed device.
pub type LSDEV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EPTYP` reader - Endpoint type Indicates the transfer type selected.
pub type EPTYP_R = crate::FieldReader;
///Field `EPTYP` writer - Endpoint type Indicates the transfer type selected.
pub type EPTYP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `MCNT` reader - Multicount This field indicates to the host the number of transactions that must be executed per frame for this periodic endpoint. For non-periodic transfers, this field is not used Note: This field must be set to at least 01.
pub type MCNT_R = crate::FieldReader;
///Field `MCNT` writer - Multicount This field indicates to the host the number of transactions that must be executed per frame for this periodic endpoint. For non-periodic transfers, this field is not used Note: This field must be set to at least 01.
pub type MCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DAD` reader - Device address This field selects the specific device serving as the data source or sink.
pub type DAD_R = crate::FieldReader;
///Field `DAD` writer - Device address This field selects the specific device serving as the data source or sink.
pub type DAD_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `ODDFRM` reader - Odd frame This field is set (reset) by the application to indicate that the OTG host must perform a transfer in an odd frame. This field is applicable for only periodic (isochronous and interrupt) transactions.
pub type ODDFRM_R = crate::BitReader;
///Field `ODDFRM` writer - Odd frame This field is set (reset) by the application to indicate that the OTG host must perform a transfer in an odd frame. This field is applicable for only periodic (isochronous and interrupt) transactions.
pub type ODDFRM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHDIS` reader - Channel disable The application sets this bit to stop transmitting/receiving data on a channel, even before the transfer for that channel is complete. The application must wait for the Channel disabled interrupt before treating the channel as disabled.
pub type CHDIS_R = crate::BitReader;
///Field `CHDIS` writer - Channel disable The application sets this bit to stop transmitting/receiving data on a channel, even before the transfer for that channel is complete. The application must wait for the Channel disabled interrupt before treating the channel as disabled.
pub type CHDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHENA` reader - Channel enable This field is set by the application and cleared by the OTG host.
pub type CHENA_R = crate::BitReader;
///Field `CHENA` writer - Channel enable This field is set by the application and cleared by the OTG host.
pub type CHENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:10 - Maximum packet size Indicates the maximum packet size of the associated endpoint.
    #[inline(always)]
    pub fn mpsiz(&self) -> MPSIZ_R {
        MPSIZ_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 11:14 - Endpoint number Indicates the endpoint number on the device serving as the data source or sink.
    #[inline(always)]
    pub fn epnum(&self) -> EPNUM_R {
        EPNUM_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    ///Bit 15 - Endpoint direction Indicates whether the transaction is IN or OUT.
    #[inline(always)]
    pub fn epdir(&self) -> EPDIR_R {
        EPDIR_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 17 - Low-speed device This field is set by the application to indicate that this channel is communicating to a low-speed device.
    #[inline(always)]
    pub fn lsdev(&self) -> LSDEV_R {
        LSDEV_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bits 18:19 - Endpoint type Indicates the transfer type selected.
    #[inline(always)]
    pub fn eptyp(&self) -> EPTYP_R {
        EPTYP_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bits 20:21 - Multicount This field indicates to the host the number of transactions that must be executed per frame for this periodic endpoint. For non-periodic transfers, this field is not used Note: This field must be set to at least 01.
    #[inline(always)]
    pub fn mcnt(&self) -> MCNT_R {
        MCNT_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bits 22:28 - Device address This field selects the specific device serving as the data source or sink.
    #[inline(always)]
    pub fn dad(&self) -> DAD_R {
        DAD_R::new(((self.bits >> 22) & 0x7f) as u8)
    }
    ///Bit 29 - Odd frame This field is set (reset) by the application to indicate that the OTG host must perform a transfer in an odd frame. This field is applicable for only periodic (isochronous and interrupt) transactions.
    #[inline(always)]
    pub fn oddfrm(&self) -> ODDFRM_R {
        ODDFRM_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Channel disable The application sets this bit to stop transmitting/receiving data on a channel, even before the transfer for that channel is complete. The application must wait for the Channel disabled interrupt before treating the channel as disabled.
    #[inline(always)]
    pub fn chdis(&self) -> CHDIS_R {
        CHDIS_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Channel enable This field is set by the application and cleared by the OTG host.
    #[inline(always)]
    pub fn chena(&self) -> CHENA_R {
        CHENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCCHAR0")
            .field("mpsiz", &self.mpsiz())
            .field("epnum", &self.epnum())
            .field("epdir", &self.epdir())
            .field("lsdev", &self.lsdev())
            .field("eptyp", &self.eptyp())
            .field("mcnt", &self.mcnt())
            .field("dad", &self.dad())
            .field("oddfrm", &self.oddfrm())
            .field("chdis", &self.chdis())
            .field("chena", &self.chena())
            .finish()
    }
}
impl W {
    ///Bits 0:10 - Maximum packet size Indicates the maximum packet size of the associated endpoint.
    #[inline(always)]
    pub fn mpsiz(&mut self) -> MPSIZ_W<'_, HCCHAR0rs> {
        MPSIZ_W::new(self, 0)
    }
    ///Bits 11:14 - Endpoint number Indicates the endpoint number on the device serving as the data source or sink.
    #[inline(always)]
    pub fn epnum(&mut self) -> EPNUM_W<'_, HCCHAR0rs> {
        EPNUM_W::new(self, 11)
    }
    ///Bit 15 - Endpoint direction Indicates whether the transaction is IN or OUT.
    #[inline(always)]
    pub fn epdir(&mut self) -> EPDIR_W<'_, HCCHAR0rs> {
        EPDIR_W::new(self, 15)
    }
    ///Bit 17 - Low-speed device This field is set by the application to indicate that this channel is communicating to a low-speed device.
    #[inline(always)]
    pub fn lsdev(&mut self) -> LSDEV_W<'_, HCCHAR0rs> {
        LSDEV_W::new(self, 17)
    }
    ///Bits 18:19 - Endpoint type Indicates the transfer type selected.
    #[inline(always)]
    pub fn eptyp(&mut self) -> EPTYP_W<'_, HCCHAR0rs> {
        EPTYP_W::new(self, 18)
    }
    ///Bits 20:21 - Multicount This field indicates to the host the number of transactions that must be executed per frame for this periodic endpoint. For non-periodic transfers, this field is not used Note: This field must be set to at least 01.
    #[inline(always)]
    pub fn mcnt(&mut self) -> MCNT_W<'_, HCCHAR0rs> {
        MCNT_W::new(self, 20)
    }
    ///Bits 22:28 - Device address This field selects the specific device serving as the data source or sink.
    #[inline(always)]
    pub fn dad(&mut self) -> DAD_W<'_, HCCHAR0rs> {
        DAD_W::new(self, 22)
    }
    ///Bit 29 - Odd frame This field is set (reset) by the application to indicate that the OTG host must perform a transfer in an odd frame. This field is applicable for only periodic (isochronous and interrupt) transactions.
    #[inline(always)]
    pub fn oddfrm(&mut self) -> ODDFRM_W<'_, HCCHAR0rs> {
        ODDFRM_W::new(self, 29)
    }
    ///Bit 30 - Channel disable The application sets this bit to stop transmitting/receiving data on a channel, even before the transfer for that channel is complete. The application must wait for the Channel disabled interrupt before treating the channel as disabled.
    #[inline(always)]
    pub fn chdis(&mut self) -> CHDIS_W<'_, HCCHAR0rs> {
        CHDIS_W::new(self, 30)
    }
    ///Bit 31 - Channel enable This field is set by the application and cleared by the OTG host.
    #[inline(always)]
    pub fn chena(&mut self) -> CHENA_W<'_, HCCHAR0rs> {
        CHENA_W::new(self, 31)
    }
}
/**OTG host channel 0 characteristics register

You can [`read`](crate::Reg::read) this register and get [`hcchar0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcchar0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#OTG_HS:HCCHAR0)*/
pub struct HCCHAR0rs;
impl crate::RegisterSpec for HCCHAR0rs {
    type Ux = u32;
}
///`read()` method returns [`hcchar0::R`](R) reader structure
impl crate::Readable for HCCHAR0rs {}
///`write(|w| ..)` method takes [`hcchar0::W`](W) writer structure
impl crate::Writable for HCCHAR0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HCCHAR0 to value 0
impl crate::Resettable for HCCHAR0rs {}
