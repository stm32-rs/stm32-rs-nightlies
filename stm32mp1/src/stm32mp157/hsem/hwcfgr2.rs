///Register `HWCFGR2` reader
pub type R = crate::R<HWCFGR2rs>;
///Field `MASTERID1` reader - MASTERID1
pub type MASTERID1_R = crate::FieldReader;
///Field `MASTERID2` reader - MASTERID2
pub type MASTERID2_R = crate::FieldReader;
///Field `MASTERID3` reader - MASTERID3
pub type MASTERID3_R = crate::FieldReader;
///Field `MASTERID4` reader - MASTERID4
pub type MASTERID4_R = crate::FieldReader;
impl R {
    ///Bits 0:3 - MASTERID1
    #[inline(always)]
    pub fn masterid1(&self) -> MASTERID1_R {
        MASTERID1_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - MASTERID2
    #[inline(always)]
    pub fn masterid2(&self) -> MASTERID2_R {
        MASTERID2_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - MASTERID3
    #[inline(always)]
    pub fn masterid3(&self) -> MASTERID3_R {
        MASTERID3_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - MASTERID4
    #[inline(always)]
    pub fn masterid4(&self) -> MASTERID4_R {
        MASTERID4_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HWCFGR2")
            .field("masterid1", &self.masterid1())
            .field("masterid2", &self.masterid2())
            .field("masterid3", &self.masterid3())
            .field("masterid4", &self.masterid4())
            .finish()
    }
}
/**HSEM hardware configuration register 2

You can [`read`](crate::Reg::read) this register and get [`hwcfgr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HWCFGR2)*/
pub struct HWCFGR2rs;
impl crate::RegisterSpec for HWCFGR2rs {
    type Ux = u32;
}
///`read()` method returns [`hwcfgr2::R`](R) reader structure
impl crate::Readable for HWCFGR2rs {}
///`reset()` method sets HWCFGR2 to value 0x21
impl crate::Resettable for HWCFGR2rs {
    const RESET_VALUE: u32 = 0x21;
}
