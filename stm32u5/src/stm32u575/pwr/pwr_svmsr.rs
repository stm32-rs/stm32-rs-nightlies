///Register `PWR_SVMSR` reader
pub type R = crate::R<PWR_SVMSRrs>;
///Field `REGS` reader - Regulator selection
pub type REGS_R = crate::BitReader;
///Field `PVDO` reader - VDD voltage detector output
pub type PVDO_R = crate::BitReader;
///Field `ACTVOSRDY` reader - Voltage level ready for currently used VOS
pub type ACTVOSRDY_R = crate::BitReader;
///Field `ACTVOS` reader - VOS currently applied to VCORE This field provides the last VOS value.
pub type ACTVOS_R = crate::FieldReader;
///Field `VDDUSBRDY` reader - VDDUSB ready
pub type VDDUSBRDY_R = crate::BitReader;
///Field `VDDIO2RDY` reader - VDDIO2 ready
pub type VDDIO2RDY_R = crate::BitReader;
///Field `VDDA1RDY` reader - VDDA ready versus 1.6V voltage monitor
pub type VDDA1RDY_R = crate::BitReader;
///Field `VDDA2RDY` reader - VDDA ready versus 1.8 V voltage monitor
pub type VDDA2RDY_R = crate::BitReader;
impl R {
    ///Bit 1 - Regulator selection
    #[inline(always)]
    pub fn regs(&self) -> REGS_R {
        REGS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 4 - VDD voltage detector output
    #[inline(always)]
    pub fn pvdo(&self) -> PVDO_R {
        PVDO_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 15 - Voltage level ready for currently used VOS
    #[inline(always)]
    pub fn actvosrdy(&self) -> ACTVOSRDY_R {
        ACTVOSRDY_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:17 - VOS currently applied to VCORE This field provides the last VOS value.
    #[inline(always)]
    pub fn actvos(&self) -> ACTVOS_R {
        ACTVOS_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bit 24 - VDDUSB ready
    #[inline(always)]
    pub fn vddusbrdy(&self) -> VDDUSBRDY_R {
        VDDUSBRDY_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - VDDIO2 ready
    #[inline(always)]
    pub fn vddio2rdy(&self) -> VDDIO2RDY_R {
        VDDIO2RDY_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - VDDA ready versus 1.6V voltage monitor
    #[inline(always)]
    pub fn vdda1rdy(&self) -> VDDA1RDY_R {
        VDDA1RDY_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - VDDA ready versus 1.8 V voltage monitor
    #[inline(always)]
    pub fn vdda2rdy(&self) -> VDDA2RDY_R {
        VDDA2RDY_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWR_SVMSR")
            .field("regs", &self.regs())
            .field("pvdo", &self.pvdo())
            .field("actvosrdy", &self.actvosrdy())
            .field("actvos", &self.actvos())
            .field("vddusbrdy", &self.vddusbrdy())
            .field("vddio2rdy", &self.vddio2rdy())
            .field("vdda1rdy", &self.vdda1rdy())
            .field("vdda2rdy", &self.vdda2rdy())
            .finish()
    }
}
/**

You can [`read`](crate::Reg::read) this register and get [`pwr_svmsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#PWR:PWR_SVMSR)*/
pub struct PWR_SVMSRrs;
impl crate::RegisterSpec for PWR_SVMSRrs {
    type Ux = u32;
}
///`read()` method returns [`pwr_svmsr::R`](R) reader structure
impl crate::Readable for PWR_SVMSRrs {}
///`reset()` method sets PWR_SVMSR to value 0x8000
impl crate::Resettable for PWR_SVMSRrs {
    const RESET_VALUE: u32 = 0x8000;
}
