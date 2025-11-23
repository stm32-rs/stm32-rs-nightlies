///Register `DCFG` reader
pub type R = crate::R<DCFGrs>;
///Register `DCFG` writer
pub type W = crate::W<DCFGrs>;
///Field `DSPD` reader - Device speed Indicates the speed at which the application requires the core to enumerate, or the maximum speed the application can support. However, the actual bus speed is determined only after the chirp sequence is completed, and is based on the speed of the USB host to which the core is connected.
pub type DSPD_R = crate::FieldReader;
///Field `DSPD` writer - Device speed Indicates the speed at which the application requires the core to enumerate, or the maximum speed the application can support. However, the actual bus speed is determined only after the chirp sequence is completed, and is based on the speed of the USB host to which the core is connected.
pub type DSPD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `NZLSOHSK` reader - Non-zero-length status OUT handshake The application can use this field to select the handshake the core sends on receiving a nonzero-length data packet during the OUT transaction of a control transfers status stage.
pub type NZLSOHSK_R = crate::BitReader;
///Field `NZLSOHSK` writer - Non-zero-length status OUT handshake The application can use this field to select the handshake the core sends on receiving a nonzero-length data packet during the OUT transaction of a control transfers status stage.
pub type NZLSOHSK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DAD` reader - Device address The application must program this field after every SetAddress control command.
pub type DAD_R = crate::FieldReader;
///Field `DAD` writer - Device address The application must program this field after every SetAddress control command.
pub type DAD_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `PFIVL` reader - Periodic frame interval Indicates the time within a frame at which the application must be notified using the end of periodic frame interrupt. This can be used to determine if all the isochronous traffic for that frame is complete.
pub type PFIVL_R = crate::FieldReader;
///Field `PFIVL` writer - Periodic frame interval Indicates the time within a frame at which the application must be notified using the end of periodic frame interrupt. This can be used to determine if all the isochronous traffic for that frame is complete.
pub type PFIVL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `ERRATIM` reader - Erratic error interrupt mask
pub type ERRATIM_R = crate::BitReader;
///Field `ERRATIM` writer - Erratic error interrupt mask
pub type ERRATIM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PERSCHIVL` reader - Periodic schedule interval This field specifies the amount of time the Internal DMA engine must allocate for fetching periodic IN endpoint data. Based on the number of periodic endpoints, this value must be specified as 25, 50 or 75% of the (micro) frame. When any periodic endpoints are active, the internal DMA engine allocates the specified amount of time in fetching periodic IN endpoint data When no periodic endpoint is active, then the internal DMA engine services nonperiodic endpoints, ignoring this field After the specified time within a (micro) frame, the DMA switches to fetching nonperiodic endpoints
pub type PERSCHIVL_R = crate::FieldReader;
///Field `PERSCHIVL` writer - Periodic schedule interval This field specifies the amount of time the Internal DMA engine must allocate for fetching periodic IN endpoint data. Based on the number of periodic endpoints, this value must be specified as 25, 50 or 75% of the (micro) frame. When any periodic endpoints are active, the internal DMA engine allocates the specified amount of time in fetching periodic IN endpoint data When no periodic endpoint is active, then the internal DMA engine services nonperiodic endpoints, ignoring this field After the specified time within a (micro) frame, the DMA switches to fetching nonperiodic endpoints
pub type PERSCHIVL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - Device speed Indicates the speed at which the application requires the core to enumerate, or the maximum speed the application can support. However, the actual bus speed is determined only after the chirp sequence is completed, and is based on the speed of the USB host to which the core is connected.
    #[inline(always)]
    pub fn dspd(&self) -> DSPD_R {
        DSPD_R::new((self.bits & 3) as u8)
    }
    ///Bit 2 - Non-zero-length status OUT handshake The application can use this field to select the handshake the core sends on receiving a nonzero-length data packet during the OUT transaction of a control transfers status stage.
    #[inline(always)]
    pub fn nzlsohsk(&self) -> NZLSOHSK_R {
        NZLSOHSK_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 4:10 - Device address The application must program this field after every SetAddress control command.
    #[inline(always)]
    pub fn dad(&self) -> DAD_R {
        DAD_R::new(((self.bits >> 4) & 0x7f) as u8)
    }
    ///Bits 11:12 - Periodic frame interval Indicates the time within a frame at which the application must be notified using the end of periodic frame interrupt. This can be used to determine if all the isochronous traffic for that frame is complete.
    #[inline(always)]
    pub fn pfivl(&self) -> PFIVL_R {
        PFIVL_R::new(((self.bits >> 11) & 3) as u8)
    }
    ///Bit 15 - Erratic error interrupt mask
    #[inline(always)]
    pub fn erratim(&self) -> ERRATIM_R {
        ERRATIM_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 24:25 - Periodic schedule interval This field specifies the amount of time the Internal DMA engine must allocate for fetching periodic IN endpoint data. Based on the number of periodic endpoints, this value must be specified as 25, 50 or 75% of the (micro) frame. When any periodic endpoints are active, the internal DMA engine allocates the specified amount of time in fetching periodic IN endpoint data When no periodic endpoint is active, then the internal DMA engine services nonperiodic endpoints, ignoring this field After the specified time within a (micro) frame, the DMA switches to fetching nonperiodic endpoints
    #[inline(always)]
    pub fn perschivl(&self) -> PERSCHIVL_R {
        PERSCHIVL_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCFG")
            .field("dspd", &self.dspd())
            .field("nzlsohsk", &self.nzlsohsk())
            .field("dad", &self.dad())
            .field("pfivl", &self.pfivl())
            .field("erratim", &self.erratim())
            .field("perschivl", &self.perschivl())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Device speed Indicates the speed at which the application requires the core to enumerate, or the maximum speed the application can support. However, the actual bus speed is determined only after the chirp sequence is completed, and is based on the speed of the USB host to which the core is connected.
    #[inline(always)]
    pub fn dspd(&mut self) -> DSPD_W<'_, DCFGrs> {
        DSPD_W::new(self, 0)
    }
    ///Bit 2 - Non-zero-length status OUT handshake The application can use this field to select the handshake the core sends on receiving a nonzero-length data packet during the OUT transaction of a control transfers status stage.
    #[inline(always)]
    pub fn nzlsohsk(&mut self) -> NZLSOHSK_W<'_, DCFGrs> {
        NZLSOHSK_W::new(self, 2)
    }
    ///Bits 4:10 - Device address The application must program this field after every SetAddress control command.
    #[inline(always)]
    pub fn dad(&mut self) -> DAD_W<'_, DCFGrs> {
        DAD_W::new(self, 4)
    }
    ///Bits 11:12 - Periodic frame interval Indicates the time within a frame at which the application must be notified using the end of periodic frame interrupt. This can be used to determine if all the isochronous traffic for that frame is complete.
    #[inline(always)]
    pub fn pfivl(&mut self) -> PFIVL_W<'_, DCFGrs> {
        PFIVL_W::new(self, 11)
    }
    ///Bit 15 - Erratic error interrupt mask
    #[inline(always)]
    pub fn erratim(&mut self) -> ERRATIM_W<'_, DCFGrs> {
        ERRATIM_W::new(self, 15)
    }
    ///Bits 24:25 - Periodic schedule interval This field specifies the amount of time the Internal DMA engine must allocate for fetching periodic IN endpoint data. Based on the number of periodic endpoints, this value must be specified as 25, 50 or 75% of the (micro) frame. When any periodic endpoints are active, the internal DMA engine allocates the specified amount of time in fetching periodic IN endpoint data When no periodic endpoint is active, then the internal DMA engine services nonperiodic endpoints, ignoring this field After the specified time within a (micro) frame, the DMA switches to fetching nonperiodic endpoints
    #[inline(always)]
    pub fn perschivl(&mut self) -> PERSCHIVL_W<'_, DCFGrs> {
        PERSCHIVL_W::new(self, 24)
    }
}
/**OTG device configuration register

You can [`read`](crate::Reg::read) this register and get [`dcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#OTG_HS:DCFG)*/
pub struct DCFGrs;
impl crate::RegisterSpec for DCFGrs {
    type Ux = u32;
}
///`read()` method returns [`dcfg::R`](R) reader structure
impl crate::Readable for DCFGrs {}
///`write(|w| ..)` method takes [`dcfg::W`](W) writer structure
impl crate::Writable for DCFGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DCFG to value 0x0220_0000
impl crate::Resettable for DCFGrs {
    const RESET_VALUE: u32 = 0x0220_0000;
}
