///Register `UCPDR` reader
pub type R = crate::R<UCPDRrs>;
///Register `UCPDR` writer
pub type W = crate::W<UCPDRrs>;
///Field `UCPD_DBDIS` reader - USB Type-C and power delivery dead battery disable After exiting reset, the USB Type-C 'dead battery' behavior is enabled, which may have a pull-down effect on CC1 and CC2 pins. It is recommended to disable it in all case, either to stop this pull-down or to hand over control to the UCPD (which should therefore be initialized before doing the disable).
pub type UCPD_DBDIS_R = crate::BitReader;
///Field `UCPD_DBDIS` writer - USB Type-C and power delivery dead battery disable After exiting reset, the USB Type-C 'dead battery' behavior is enabled, which may have a pull-down effect on CC1 and CC2 pins. It is recommended to disable it in all case, either to stop this pull-down or to hand over control to the UCPD (which should therefore be initialized before doing the disable).
pub type UCPD_DBDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UCPD_STBY` reader - USB Type-c and Power delivery Standby mode When set, this bit is used to memorize the UCPD configuration in Standby mode. This bit must be written to 1 just before entering Standby mode when using UCPD, and it must be written to 0 after exiting the standby mode and before writing any UCPD register.
pub type UCPD_STBY_R = crate::BitReader;
///Field `UCPD_STBY` writer - USB Type-c and Power delivery Standby mode When set, this bit is used to memorize the UCPD configuration in Standby mode. This bit must be written to 1 just before entering Standby mode when using UCPD, and it must be written to 0 after exiting the standby mode and before writing any UCPD register.
pub type UCPD_STBY_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - USB Type-C and power delivery dead battery disable After exiting reset, the USB Type-C 'dead battery' behavior is enabled, which may have a pull-down effect on CC1 and CC2 pins. It is recommended to disable it in all case, either to stop this pull-down or to hand over control to the UCPD (which should therefore be initialized before doing the disable).
    #[inline(always)]
    pub fn ucpd_dbdis(&self) -> UCPD_DBDIS_R {
        UCPD_DBDIS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - USB Type-c and Power delivery Standby mode When set, this bit is used to memorize the UCPD configuration in Standby mode. This bit must be written to 1 just before entering Standby mode when using UCPD, and it must be written to 0 after exiting the standby mode and before writing any UCPD register.
    #[inline(always)]
    pub fn ucpd_stby(&self) -> UCPD_STBY_R {
        UCPD_STBY_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UCPDR")
            .field("ucpd_dbdis", &self.ucpd_dbdis())
            .field("ucpd_stby", &self.ucpd_stby())
            .finish()
    }
}
impl W {
    ///Bit 0 - USB Type-C and power delivery dead battery disable After exiting reset, the USB Type-C 'dead battery' behavior is enabled, which may have a pull-down effect on CC1 and CC2 pins. It is recommended to disable it in all case, either to stop this pull-down or to hand over control to the UCPD (which should therefore be initialized before doing the disable).
    #[inline(always)]
    pub fn ucpd_dbdis(&mut self) -> UCPD_DBDIS_W<'_, UCPDRrs> {
        UCPD_DBDIS_W::new(self, 0)
    }
    ///Bit 1 - USB Type-c and Power delivery Standby mode When set, this bit is used to memorize the UCPD configuration in Standby mode. This bit must be written to 1 just before entering Standby mode when using UCPD, and it must be written to 0 after exiting the standby mode and before writing any UCPD register.
    #[inline(always)]
    pub fn ucpd_stby(&mut self) -> UCPD_STBY_W<'_, UCPDRrs> {
        UCPD_STBY_W::new(self, 1)
    }
}
/**PWR USB Type-C power delivery register

You can [`read`](crate::Reg::read) this register and get [`ucpdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucpdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#PWR:UCPDR)*/
pub struct UCPDRrs;
impl crate::RegisterSpec for UCPDRrs {
    type Ux = u32;
}
///`read()` method returns [`ucpdr::R`](R) reader structure
impl crate::Readable for UCPDRrs {}
///`write(|w| ..)` method takes [`ucpdr::W`](W) writer structure
impl crate::Writable for UCPDRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets UCPDR to value 0
impl crate::Resettable for UCPDRrs {}
