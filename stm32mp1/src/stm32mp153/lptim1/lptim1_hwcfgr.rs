///Register `LPTIM1_HWCFGR` reader
pub type R = crate::R<LPTIM1_HWCFGRrs>;
///Field `CFG1` reader - CFG1
pub type CFG1_R = crate::FieldReader;
///Field `CFG2` reader - CFG2
pub type CFG2_R = crate::FieldReader;
///Field `CFG3` reader - CFG3
pub type CFG3_R = crate::FieldReader;
///Field `CFG4` reader - CFG4
pub type CFG4_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - CFG1
    #[inline(always)]
    pub fn cfg1(&self) -> CFG1_R {
        CFG1_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - CFG2
    #[inline(always)]
    pub fn cfg2(&self) -> CFG2_R {
        CFG2_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:19 - CFG3
    #[inline(always)]
    pub fn cfg3(&self) -> CFG3_R {
        CFG3_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 24:31 - CFG4
    #[inline(always)]
    pub fn cfg4(&self) -> CFG4_R {
        CFG4_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPTIM1_HWCFGR")
            .field("cfg1", &self.cfg1())
            .field("cfg2", &self.cfg2())
            .field("cfg3", &self.cfg3())
            .field("cfg4", &self.cfg4())
            .finish()
    }
}
/**LPTIM 1 peripheral hardware configuration register

You can [`read`](crate::Reg::read) this register and get [`lptim1_hwcfgr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LPTIM1:LPTIM1_HWCFGR)*/
pub struct LPTIM1_HWCFGRrs;
impl crate::RegisterSpec for LPTIM1_HWCFGRrs {
    type Ux = u32;
}
///`read()` method returns [`lptim1_hwcfgr::R`](R) reader structure
impl crate::Readable for LPTIM1_HWCFGRrs {}
///`reset()` method sets LPTIM1_HWCFGR to value 0x0001_0804
impl crate::Resettable for LPTIM1_HWCFGRrs {
    const RESET_VALUE: u32 = 0x0001_0804;
}
