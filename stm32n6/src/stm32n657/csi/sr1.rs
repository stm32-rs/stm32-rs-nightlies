///Register `SR1` reader
pub type R = crate::R<SR1rs>;
///Field `ESOTDL0F` reader - SOT error flag on lane 0
pub type ESOTDL0F_R = crate::BitReader;
///Field `ESOTSYNCDL0F` reader - SOT synchronization error flag on lane 0
pub type ESOTSYNCDL0F_R = crate::BitReader;
///Field `EESCDL0F` reader - D-PHY_RX lane 0 escape entry error flag
pub type EESCDL0F_R = crate::BitReader;
///Field `ESYNCESCDL0F` reader - D-PHY_RX lane 0 low-power data transmission synchronization error flag
pub type ESYNCESCDL0F_R = crate::BitReader;
///Field `ECTRLDL0F` reader - D-PHY_RX lane 0 control error flag
pub type ECTRLDL0F_R = crate::BitReader;
///Field `ESOTDL1F` reader - SOT error flag on lane 1
pub type ESOTDL1F_R = crate::BitReader;
///Field `ESOTSYNCDL1F` reader - SOT synchronization error flag on lane 1
pub type ESOTSYNCDL1F_R = crate::BitReader;
///Field `EESCDL1F` reader - D-PHY_RX lane 1 escape entry error flag
pub type EESCDL1F_R = crate::BitReader;
///Field `ESYNCESCDL1F` reader - D-PHY_RX lane 1 low-power data transmission synchronization error flag
pub type ESYNCESCDL1F_R = crate::BitReader;
///Field `ECTRLDL1F` reader - D-PHY_RX lane 1 control error flag
pub type ECTRLDL1F_R = crate::BitReader;
///Field `ACTDL0F` reader - D-PHY_RX lane 0 high-speed reception active
pub type ACTDL0F_R = crate::BitReader;
///Field `SYNCDL0F` reader - D-PHY_RX lane 0 receiver synchronization observed
pub type SYNCDL0F_R = crate::BitReader;
///Field `SKCALDL0F` reader - D-PHY_RX lane 0 high-speed skew calibration
pub type SKCALDL0F_R = crate::BitReader;
///Field `STOPDL0F` reader - D-PHY_RX receiver data lane 0 in stop state
pub type STOPDL0F_R = crate::BitReader;
///Field `ULPNDL0F` reader - D-PHY_RX receiver ultra-low-power state (not) active on data lane 0
pub type ULPNDL0F_R = crate::BitReader;
///Field `ACTDL1F` reader - D-PHY_RX lane 1 high-speed reception active
pub type ACTDL1F_R = crate::BitReader;
///Field `SYNCDL1F` reader - D-PHY_RX lane 1 receiver synchronization observed
pub type SYNCDL1F_R = crate::BitReader;
///Field `SKCALDL1F` reader - D-PHY_RX lane 1 high-speed skew calibration
pub type SKCALDL1F_R = crate::BitReader;
///Field `STOPDL1F` reader - D-PHY_RX receiver data lane 1 in stop state
pub type STOPDL1F_R = crate::BitReader;
///Field `ULPNDL1F` reader - D-PHY_RX receiver ultra-low-power state (not) active on data lane 1
pub type ULPNDL1F_R = crate::BitReader;
///Field `STOPCLF` reader - D-PHY_RX receiver in stop state for the clock lane
pub type STOPCLF_R = crate::BitReader;
///Field `ULPNACTF` reader - D-PHY_RX receiver ULP state (not) active
pub type ULPNACTF_R = crate::BitReader;
///Field `ULPNCLF` reader - D-PHY_RX receiver Ultra-Low power state (not) on clock lane.
pub type ULPNCLF_R = crate::BitReader;
///Field `ACTCLF` reader - D-PHY_RX receiver clock active flag
pub type ACTCLF_R = crate::BitReader;
impl R {
    ///Bit 0 - SOT error flag on lane 0
    #[inline(always)]
    pub fn esotdl0f(&self) -> ESOTDL0F_R {
        ESOTDL0F_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - SOT synchronization error flag on lane 0
    #[inline(always)]
    pub fn esotsyncdl0f(&self) -> ESOTSYNCDL0F_R {
        ESOTSYNCDL0F_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - D-PHY_RX lane 0 escape entry error flag
    #[inline(always)]
    pub fn eescdl0f(&self) -> EESCDL0F_R {
        EESCDL0F_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - D-PHY_RX lane 0 low-power data transmission synchronization error flag
    #[inline(always)]
    pub fn esyncescdl0f(&self) -> ESYNCESCDL0F_R {
        ESYNCESCDL0F_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - D-PHY_RX lane 0 control error flag
    #[inline(always)]
    pub fn ectrldl0f(&self) -> ECTRLDL0F_R {
        ECTRLDL0F_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - SOT error flag on lane 1
    #[inline(always)]
    pub fn esotdl1f(&self) -> ESOTDL1F_R {
        ESOTDL1F_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - SOT synchronization error flag on lane 1
    #[inline(always)]
    pub fn esotsyncdl1f(&self) -> ESOTSYNCDL1F_R {
        ESOTSYNCDL1F_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - D-PHY_RX lane 1 escape entry error flag
    #[inline(always)]
    pub fn eescdl1f(&self) -> EESCDL1F_R {
        EESCDL1F_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - D-PHY_RX lane 1 low-power data transmission synchronization error flag
    #[inline(always)]
    pub fn esyncescdl1f(&self) -> ESYNCESCDL1F_R {
        ESYNCESCDL1F_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - D-PHY_RX lane 1 control error flag
    #[inline(always)]
    pub fn ectrldl1f(&self) -> ECTRLDL1F_R {
        ECTRLDL1F_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 16 - D-PHY_RX lane 0 high-speed reception active
    #[inline(always)]
    pub fn actdl0f(&self) -> ACTDL0F_R {
        ACTDL0F_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - D-PHY_RX lane 0 receiver synchronization observed
    #[inline(always)]
    pub fn syncdl0f(&self) -> SYNCDL0F_R {
        SYNCDL0F_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - D-PHY_RX lane 0 high-speed skew calibration
    #[inline(always)]
    pub fn skcaldl0f(&self) -> SKCALDL0F_R {
        SKCALDL0F_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - D-PHY_RX receiver data lane 0 in stop state
    #[inline(always)]
    pub fn stopdl0f(&self) -> STOPDL0F_R {
        STOPDL0F_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - D-PHY_RX receiver ultra-low-power state (not) active on data lane 0
    #[inline(always)]
    pub fn ulpndl0f(&self) -> ULPNDL0F_R {
        ULPNDL0F_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 22 - D-PHY_RX lane 1 high-speed reception active
    #[inline(always)]
    pub fn actdl1f(&self) -> ACTDL1F_R {
        ACTDL1F_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - D-PHY_RX lane 1 receiver synchronization observed
    #[inline(always)]
    pub fn syncdl1f(&self) -> SYNCDL1F_R {
        SYNCDL1F_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - D-PHY_RX lane 1 high-speed skew calibration
    #[inline(always)]
    pub fn skcaldl1f(&self) -> SKCALDL1F_R {
        SKCALDL1F_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - D-PHY_RX receiver data lane 1 in stop state
    #[inline(always)]
    pub fn stopdl1f(&self) -> STOPDL1F_R {
        STOPDL1F_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - D-PHY_RX receiver ultra-low-power state (not) active on data lane 1
    #[inline(always)]
    pub fn ulpndl1f(&self) -> ULPNDL1F_R {
        ULPNDL1F_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 28 - D-PHY_RX receiver in stop state for the clock lane
    #[inline(always)]
    pub fn stopclf(&self) -> STOPCLF_R {
        STOPCLF_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - D-PHY_RX receiver ULP state (not) active
    #[inline(always)]
    pub fn ulpnactf(&self) -> ULPNACTF_R {
        ULPNACTF_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - D-PHY_RX receiver Ultra-Low power state (not) on clock lane.
    #[inline(always)]
    pub fn ulpnclf(&self) -> ULPNCLF_R {
        ULPNCLF_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - D-PHY_RX receiver clock active flag
    #[inline(always)]
    pub fn actclf(&self) -> ACTCLF_R {
        ACTCLF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR1")
            .field("esotdl0f", &self.esotdl0f())
            .field("esotsyncdl0f", &self.esotsyncdl0f())
            .field("eescdl0f", &self.eescdl0f())
            .field("esyncescdl0f", &self.esyncescdl0f())
            .field("ectrldl0f", &self.ectrldl0f())
            .field("esotdl1f", &self.esotdl1f())
            .field("esotsyncdl1f", &self.esotsyncdl1f())
            .field("eescdl1f", &self.eescdl1f())
            .field("esyncescdl1f", &self.esyncescdl1f())
            .field("ectrldl1f", &self.ectrldl1f())
            .field("actdl0f", &self.actdl0f())
            .field("syncdl0f", &self.syncdl0f())
            .field("skcaldl0f", &self.skcaldl0f())
            .field("stopdl0f", &self.stopdl0f())
            .field("ulpndl0f", &self.ulpndl0f())
            .field("actdl1f", &self.actdl1f())
            .field("syncdl1f", &self.syncdl1f())
            .field("skcaldl1f", &self.skcaldl1f())
            .field("stopdl1f", &self.stopdl1f())
            .field("ulpndl1f", &self.ulpndl1f())
            .field("stopclf", &self.stopclf())
            .field("ulpnactf", &self.ulpnactf())
            .field("ulpnclf", &self.ulpnclf())
            .field("actclf", &self.actclf())
            .finish()
    }
}
/**CSI-2 Host status register 1

You can [`read`](crate::Reg::read) this register and get [`sr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#CSI:SR1)*/
pub struct SR1rs;
impl crate::RegisterSpec for SR1rs {
    type Ux = u32;
}
///`read()` method returns [`sr1::R`](R) reader structure
impl crate::Readable for SR1rs {}
///`reset()` method sets SR1 to value 0
impl crate::Resettable for SR1rs {}
