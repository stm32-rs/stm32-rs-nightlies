///Register `SR2` reader
pub type R = crate::R<SR2rs>;
///Field `SDBF` reader - Step Down converter Bypass mode flag
pub type SDBF_R = crate::BitReader;
///Field `SDSMPSF` reader - Step Down converter SMPS mode flag
pub type SDSMPSF_R = crate::BitReader;
///Field `REGLPS` reader - Low-power regulator started
pub type REGLPS_R = crate::BitReader;
///Field `REGLPF` reader - Low-power regulator flag
pub type REGLPF_R = crate::BitReader;
///Field `VOSF` reader - Voltage scaling flag
pub type VOSF_R = crate::BitReader;
///Field `PVDO` reader - Power voltage detector output
pub type PVDO_R = crate::BitReader;
///Field `PVMO1` reader - Peripheral voltage monitoring output: VDDUSB vs. 1.2 V
pub type PVMO1_R = crate::BitReader;
///Field `PVMO3` reader - Peripheral voltage monitoring output: VDDA vs. 1.62 V
pub type PVMO3_R = crate::BitReader;
impl R {
    ///Bit 0 - Step Down converter Bypass mode flag
    #[inline(always)]
    pub fn sdbf(&self) -> SDBF_R {
        SDBF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Step Down converter SMPS mode flag
    #[inline(always)]
    pub fn sdsmpsf(&self) -> SDSMPSF_R {
        SDSMPSF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 8 - Low-power regulator started
    #[inline(always)]
    pub fn reglps(&self) -> REGLPS_R {
        REGLPS_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Low-power regulator flag
    #[inline(always)]
    pub fn reglpf(&self) -> REGLPF_R {
        REGLPF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Voltage scaling flag
    #[inline(always)]
    pub fn vosf(&self) -> VOSF_R {
        VOSF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Power voltage detector output
    #[inline(always)]
    pub fn pvdo(&self) -> PVDO_R {
        PVDO_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Peripheral voltage monitoring output: VDDUSB vs. 1.2 V
    #[inline(always)]
    pub fn pvmo1(&self) -> PVMO1_R {
        PVMO1_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - Peripheral voltage monitoring output: VDDA vs. 1.62 V
    #[inline(always)]
    pub fn pvmo3(&self) -> PVMO3_R {
        PVMO3_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR2")
            .field("pvmo3", &self.pvmo3())
            .field("pvmo1", &self.pvmo1())
            .field("pvdo", &self.pvdo())
            .field("vosf", &self.vosf())
            .field("reglpf", &self.reglpf())
            .field("reglps", &self.reglps())
            .field("sdsmpsf", &self.sdsmpsf())
            .field("sdbf", &self.sdbf())
            .finish()
    }
}
/**Power status register 2

You can [`read`](crate::Reg::read) this register and get [`sr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#PWR:SR2)*/
pub struct SR2rs;
impl crate::RegisterSpec for SR2rs {
    type Ux = u32;
}
///`read()` method returns [`sr2::R`](R) reader structure
impl crate::Readable for SR2rs {}
///`reset()` method sets SR2 to value 0x02
impl crate::Resettable for SR2rs {
    const RESET_VALUE: u32 = 0x02;
}
