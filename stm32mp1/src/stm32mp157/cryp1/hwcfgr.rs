///Register `HWCFGR` reader
pub type R = crate::R<HWCFGRrs>;
///Field `CFG1` reader - CFG1
pub type CFG1_R = crate::FieldReader;
///Field `CFG2` reader - CFG2
pub type CFG2_R = crate::FieldReader;
///Field `CFG3` reader - CFG3
pub type CFG3_R = crate::FieldReader;
///Field `CFG4` reader - CFG4
pub type CFG4_R = crate::FieldReader;
impl R {
    ///Bits 0:3 - CFG1
    #[inline(always)]
    pub fn cfg1(&self) -> CFG1_R {
        CFG1_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - CFG2
    #[inline(always)]
    pub fn cfg2(&self) -> CFG2_R {
        CFG2_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - CFG3
    #[inline(always)]
    pub fn cfg3(&self) -> CFG3_R {
        CFG3_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - CFG4
    #[inline(always)]
    pub fn cfg4(&self) -> CFG4_R {
        CFG4_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HWCFGR")
            .field("cfg1", &self.cfg1())
            .field("cfg2", &self.cfg2())
            .field("cfg3", &self.cfg3())
            .field("cfg4", &self.cfg4())
            .finish()
    }
}
/**CRYP hardware configuration register

You can [`read`](crate::Reg::read) this register and get [`hwcfgr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#CRYP1:HWCFGR)*/
pub struct HWCFGRrs;
impl crate::RegisterSpec for HWCFGRrs {
    type Ux = u32;
}
///`read()` method returns [`hwcfgr::R`](R) reader structure
impl crate::Readable for HWCFGRrs {}
///`reset()` method sets HWCFGR to value 0x0131
impl crate::Resettable for HWCFGRrs {
    const RESET_VALUE: u32 = 0x0131;
}
