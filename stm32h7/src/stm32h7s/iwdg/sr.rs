///Register `SR` reader
pub type R = crate::R<SRrs>;
///Field `PVU` reader - Watchdog prescaler value update This bit is set by hardware to indicate that an update of the prescaler value is ongoing. It is reset by hardware when the prescaler update operation is completed in the V<sub>DD</sub> voltage domain (takes up to six periods of the IWDG kernel clock iwdg_ker_ck). The prescaler value can be updated only when PVU bit is reset.
pub type PVU_R = crate::BitReader;
///Field `RVU` reader - Watchdog counter reload value update This bit is set by hardware to indicate that an update of the reload value is ongoing. It is reset by hardware when the reload value update operation is completed in the V<sub>DD</sub> voltage domain (takes up to six periods of the IWDG kernel clock iwdg_ker_ck). The reload value can be updated only when RVU bit is reset.
pub type RVU_R = crate::BitReader;
///Field `WVU` reader - Watchdog counter window value update This bit is set by hardware to indicate that an update of the window value is ongoing. It is reset by hardware when the reload value update operation is completed in the V<sub>DD</sub> voltage domain (takes up to one period of presc_ck and two periods of the IWDG kernel clock iwdg_ker_ck). The window value can be updated only when WVU bit is reset. This bit is generated only if generic window = 1.
pub type WVU_R = crate::BitReader;
///Field `EWU` reader - Watchdog interrupt comparator value update This bit is set by hardware to indicate that an update of the interrupt comparator value (EWIT\[11:0\]) or an update of the EWIE is ongoing. It is reset by hardware when the update operation is completed in the V<sub>DD</sub> voltage domain (takes up to one period of presc_ck and two periods of the IWDG kernel clock iwdg_ker_ck). The EWIT\[11:0\] and EWIE fields can be updated only when EWU bit is reset.
pub type EWU_R = crate::BitReader;
///Field `ONF` reader - Watchdog enable status bit Set to 1 by hardware as soon as the IWDG is started. In software mode, it remains to '1' until the IWDG is reset. In hardware mode, this bit is always set to '1'.
pub type ONF_R = crate::BitReader;
///Field `EWIF` reader - Watchdog early interrupt flag This bit is set to 1 by hardware in order to indicate that an early interrupt is pending. This bit must be cleared by the software by writing the bit EWIC of IWDG_EWCR register to 1.
pub type EWIF_R = crate::BitReader;
impl R {
    ///Bit 0 - Watchdog prescaler value update This bit is set by hardware to indicate that an update of the prescaler value is ongoing. It is reset by hardware when the prescaler update operation is completed in the V<sub>DD</sub> voltage domain (takes up to six periods of the IWDG kernel clock iwdg_ker_ck). The prescaler value can be updated only when PVU bit is reset.
    #[inline(always)]
    pub fn pvu(&self) -> PVU_R {
        PVU_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Watchdog counter reload value update This bit is set by hardware to indicate that an update of the reload value is ongoing. It is reset by hardware when the reload value update operation is completed in the V<sub>DD</sub> voltage domain (takes up to six periods of the IWDG kernel clock iwdg_ker_ck). The reload value can be updated only when RVU bit is reset.
    #[inline(always)]
    pub fn rvu(&self) -> RVU_R {
        RVU_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Watchdog counter window value update This bit is set by hardware to indicate that an update of the window value is ongoing. It is reset by hardware when the reload value update operation is completed in the V<sub>DD</sub> voltage domain (takes up to one period of presc_ck and two periods of the IWDG kernel clock iwdg_ker_ck). The window value can be updated only when WVU bit is reset. This bit is generated only if generic window = 1.
    #[inline(always)]
    pub fn wvu(&self) -> WVU_R {
        WVU_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Watchdog interrupt comparator value update This bit is set by hardware to indicate that an update of the interrupt comparator value (EWIT\[11:0\]) or an update of the EWIE is ongoing. It is reset by hardware when the update operation is completed in the V<sub>DD</sub> voltage domain (takes up to one period of presc_ck and two periods of the IWDG kernel clock iwdg_ker_ck). The EWIT\[11:0\] and EWIE fields can be updated only when EWU bit is reset.
    #[inline(always)]
    pub fn ewu(&self) -> EWU_R {
        EWU_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 8 - Watchdog enable status bit Set to 1 by hardware as soon as the IWDG is started. In software mode, it remains to '1' until the IWDG is reset. In hardware mode, this bit is always set to '1'.
    #[inline(always)]
    pub fn onf(&self) -> ONF_R {
        ONF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 14 - Watchdog early interrupt flag This bit is set to 1 by hardware in order to indicate that an early interrupt is pending. This bit must be cleared by the software by writing the bit EWIC of IWDG_EWCR register to 1.
    #[inline(always)]
    pub fn ewif(&self) -> EWIF_R {
        EWIF_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("pvu", &self.pvu())
            .field("rvu", &self.rvu())
            .field("wvu", &self.wvu())
            .field("ewu", &self.ewu())
            .field("onf", &self.onf())
            .field("ewif", &self.ewif())
            .finish()
    }
}
/**IWDG status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#IWDG:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u16;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`reset()` method sets SR to value 0
impl crate::Resettable for SRrs {}
