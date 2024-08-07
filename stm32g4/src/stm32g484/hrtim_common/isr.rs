///Register `ISR` reader
pub type R = crate::R<ISRrs>;
///Field `FLT1` reader - Fault 1 Interrupt Flag
pub type FLT1_R = crate::BitReader;
///Field `FLT2` reader - Fault 2 Interrupt Flag
pub type FLT2_R = crate::BitReader;
///Field `FLT3` reader - Fault 3 Interrupt Flag
pub type FLT3_R = crate::BitReader;
///Field `FLT4` reader - Fault 4 Interrupt Flag
pub type FLT4_R = crate::BitReader;
///Field `FLT5` reader - Fault 5 Interrupt Flag
pub type FLT5_R = crate::BitReader;
///Field `SYSFLT` reader - System Fault Interrupt Flag
pub type SYSFLT_R = crate::BitReader;
///Field `FLT6` reader - Fault 6 Interrupt Flag
pub type FLT6_R = crate::BitReader;
///Field `DLLRDY` reader - DLL Ready Interrupt Flag
pub type DLLRDY_R = crate::BitReader;
///Field `BMPER` reader - Burst mode Period Interrupt Flag
pub type BMPER_R = crate::BitReader;
impl R {
    ///Bit 0 - Fault 1 Interrupt Flag
    #[inline(always)]
    pub fn flt1(&self) -> FLT1_R {
        FLT1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Fault 2 Interrupt Flag
    #[inline(always)]
    pub fn flt2(&self) -> FLT2_R {
        FLT2_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Fault 3 Interrupt Flag
    #[inline(always)]
    pub fn flt3(&self) -> FLT3_R {
        FLT3_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Fault 4 Interrupt Flag
    #[inline(always)]
    pub fn flt4(&self) -> FLT4_R {
        FLT4_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Fault 5 Interrupt Flag
    #[inline(always)]
    pub fn flt5(&self) -> FLT5_R {
        FLT5_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - System Fault Interrupt Flag
    #[inline(always)]
    pub fn sysflt(&self) -> SYSFLT_R {
        SYSFLT_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Fault 6 Interrupt Flag
    #[inline(always)]
    pub fn flt6(&self) -> FLT6_R {
        FLT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 16 - DLL Ready Interrupt Flag
    #[inline(always)]
    pub fn dllrdy(&self) -> DLLRDY_R {
        DLLRDY_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Burst mode Period Interrupt Flag
    #[inline(always)]
    pub fn bmper(&self) -> BMPER_R {
        BMPER_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR")
            .field("bmper", &self.bmper())
            .field("dllrdy", &self.dllrdy())
            .field("flt6", &self.flt6())
            .field("sysflt", &self.sysflt())
            .field("flt5", &self.flt5())
            .field("flt4", &self.flt4())
            .field("flt3", &self.flt3())
            .field("flt2", &self.flt2())
            .field("flt1", &self.flt1())
            .finish()
    }
}
/**Interrupt Status Register

You can [`read`](crate::Reg::read) this register and get [`isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G484xx.html#HRTIM_Common:ISR)*/
pub struct ISRrs;
impl crate::RegisterSpec for ISRrs {
    type Ux = u32;
}
///`read()` method returns [`isr::R`](R) reader structure
impl crate::Readable for ISRrs {}
///`reset()` method sets ISR to value 0
impl crate::Resettable for ISRrs {
    const RESET_VALUE: u32 = 0;
}
