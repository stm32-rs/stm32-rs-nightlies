///Register `HWCFR` reader
pub type R = crate::R<HWCFRrs>;
///Field `CFG1` reader - HW Generic 1
pub type CFG1_R = crate::FieldReader;
///Field `CFG2` reader - HW Generic 2
pub type CFG2_R = crate::FieldReader;
///Field `CFG3` reader - HW Generic 3
pub type CFG3_R = crate::FieldReader;
///Field `CFG4` reader - HW Generic 4
pub type CFG4_R = crate::FieldReader;
impl R {
    ///Bits 0:3 - HW Generic 1
    #[inline(always)]
    pub fn cfg1(&self) -> CFG1_R {
        CFG1_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - HW Generic 2
    #[inline(always)]
    pub fn cfg2(&self) -> CFG2_R {
        CFG2_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - HW Generic 3
    #[inline(always)]
    pub fn cfg3(&self) -> CFG3_R {
        CFG3_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - HW Generic 4
    #[inline(always)]
    pub fn cfg4(&self) -> CFG4_R {
        CFG4_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HWCFR")
            .field("cfg4", &self.cfg4())
            .field("cfg3", &self.cfg3())
            .field("cfg2", &self.cfg2())
            .field("cfg1", &self.cfg1())
            .finish()
    }
}
/**AES hardware configuration register

You can [`read`](crate::Reg::read) this register and get [`hwcfr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G081.html#AES:HWCFR)*/
pub struct HWCFRrs;
impl crate::RegisterSpec for HWCFRrs {
    type Ux = u32;
}
///`read()` method returns [`hwcfr::R`](R) reader structure
impl crate::Readable for HWCFRrs {}
///`reset()` method sets HWCFR to value 0x02
impl crate::Resettable for HWCFRrs {
    const RESET_VALUE: u32 = 0x02;
}
