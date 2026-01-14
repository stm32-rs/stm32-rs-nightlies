///Register `DEBUGSTATUSREG` reader
pub type R = crate::R<DEBUGSTATUSREGrs>;
///Field `DEBUGSTATUSREG` reader - DEBUGSTATUSREG
pub type DEBUGSTATUSREG_R = crate::FieldReader;
///Field `AESDBG_0` reader - AESDBG_0
pub type AESDBG_0_R = crate::BitReader;
///Field `AESDBG_1` reader - AESDBG_1
pub type AESDBG_1_R = crate::BitReader;
///Field `AESDBG_2` reader - AESDBG_2
pub type AESDBG_2_R = crate::BitReader;
///Field `AESDBG_3` reader - AESDBG_3
pub type AESDBG_3_R = crate::BitReader;
impl R {
    ///Bits 0:6 - DEBUGSTATUSREG
    #[inline(always)]
    pub fn debugstatusreg(&self) -> DEBUGSTATUSREG_R {
        DEBUGSTATUSREG_R::new((self.bits & 0x7f) as u8)
    }
    ///Bit 16 - AESDBG_0
    #[inline(always)]
    pub fn aesdbg_0(&self) -> AESDBG_0_R {
        AESDBG_0_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - AESDBG_1
    #[inline(always)]
    pub fn aesdbg_1(&self) -> AESDBG_1_R {
        AESDBG_1_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - AESDBG_2
    #[inline(always)]
    pub fn aesdbg_2(&self) -> AESDBG_2_R {
        AESDBG_2_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - AESDBG_3
    #[inline(always)]
    pub fn aesdbg_3(&self) -> AESDBG_3_R {
        AESDBG_3_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEBUGSTATUSREG")
            .field("debugstatusreg", &self.debugstatusreg())
            .field("aesdbg_0", &self.aesdbg_0())
            .field("aesdbg_1", &self.aesdbg_1())
            .field("aesdbg_2", &self.aesdbg_2())
            .field("aesdbg_3", &self.aesdbg_3())
            .finish()
    }
}
/**DebugStatus register

You can [`read`](crate::Reg::read) this register and get [`debugstatusreg::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#BLUE:DEBUGSTATUSREG)*/
pub struct DEBUGSTATUSREGrs;
impl crate::RegisterSpec for DEBUGSTATUSREGrs {
    type Ux = u32;
}
///`read()` method returns [`debugstatusreg::R`](R) reader structure
impl crate::Readable for DEBUGSTATUSREGrs {}
///`reset()` method sets DEBUGSTATUSREG to value 0
impl crate::Resettable for DEBUGSTATUSREGrs {}
