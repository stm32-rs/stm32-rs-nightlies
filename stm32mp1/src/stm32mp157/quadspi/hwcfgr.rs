///Register `HWCFGR` reader
pub type R = crate::R<HWCFGRrs>;
///Field `FIFOSIZE` reader - FIFOSIZE
pub type FIFOSIZE_R = crate::FieldReader;
///Field `FIFOPTR` reader - FIFOPTR
pub type FIFOPTR_R = crate::FieldReader;
///Field `PRESCVAL` reader - PRESCVAL
pub type PRESCVAL_R = crate::FieldReader;
///Field `IDLENGTH` reader - IDLENGTH
pub type IDLENGTH_R = crate::FieldReader;
impl R {
    ///Bits 0:3 - FIFOSIZE
    #[inline(always)]
    pub fn fifosize(&self) -> FIFOSIZE_R {
        FIFOSIZE_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - FIFOPTR
    #[inline(always)]
    pub fn fifoptr(&self) -> FIFOPTR_R {
        FIFOPTR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - PRESCVAL
    #[inline(always)]
    pub fn prescval(&self) -> PRESCVAL_R {
        PRESCVAL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - IDLENGTH
    #[inline(always)]
    pub fn idlength(&self) -> IDLENGTH_R {
        IDLENGTH_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HWCFGR")
            .field("fifosize", &self.fifosize())
            .field("fifoptr", &self.fifoptr())
            .field("prescval", &self.prescval())
            .field("idlength", &self.idlength())
            .finish()
    }
}
/**QUADSPI HW configuration register

You can [`read`](crate::Reg::read) this register and get [`hwcfgr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#QUADSPI:HWCFGR)*/
pub struct HWCFGRrs;
impl crate::RegisterSpec for HWCFGRrs {
    type Ux = u32;
}
///`read()` method returns [`hwcfgr::R`](R) reader structure
impl crate::Readable for HWCFGRrs {}
///`reset()` method sets HWCFGR to value 0xb058
impl crate::Resettable for HWCFGRrs {
    const RESET_VALUE: u32 = 0xb058;
}
