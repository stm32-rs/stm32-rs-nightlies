///Register `SVMSR` reader
pub type R = crate::R<SVMSRrs>;
///Field `REGS` reader - Regulator selection
pub type REGS_R = crate::BitReader;
///Field `PVDO` reader - Programmable voltage detector output
pub type PVDO_R = crate::BitReader;
///Field `ACTVOSRDY` reader - Voltage level ready for currently used VOS
pub type ACTVOSRDY_R = crate::BitReader;
///Field `ACTVOS` reader - VOS currently applied to VsubCORE/sub This field provides the last VOS value.
pub type ACTVOS_R = crate::BitReader;
impl R {
    ///Bit 1 - Regulator selection
    #[inline(always)]
    pub fn regs(&self) -> REGS_R {
        REGS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 4 - Programmable voltage detector output
    #[inline(always)]
    pub fn pvdo(&self) -> PVDO_R {
        PVDO_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 15 - Voltage level ready for currently used VOS
    #[inline(always)]
    pub fn actvosrdy(&self) -> ACTVOSRDY_R {
        ACTVOSRDY_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - VOS currently applied to VsubCORE/sub This field provides the last VOS value.
    #[inline(always)]
    pub fn actvos(&self) -> ACTVOS_R {
        ACTVOS_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SVMSR")
            .field("regs", &self.regs())
            .field("pvdo", &self.pvdo())
            .field("actvosrdy", &self.actvosrdy())
            .field("actvos", &self.actvos())
            .finish()
    }
}
/**PWR supply voltage monitoring status register

You can [`read`](crate::Reg::read) this register and get [`svmsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#PWR:SVMSR)*/
pub struct SVMSRrs;
impl crate::RegisterSpec for SVMSRrs {
    type Ux = u32;
}
///`read()` method returns [`svmsr::R`](R) reader structure
impl crate::Readable for SVMSRrs {}
///`reset()` method sets SVMSR to value 0x8000
impl crate::Resettable for SVMSRrs {
    const RESET_VALUE: u32 = 0x8000;
}
