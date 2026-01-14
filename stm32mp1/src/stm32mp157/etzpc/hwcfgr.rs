///Register `HWCFGR` reader
pub type R = crate::R<HWCFGRrs>;
///Field `NUM_TZMA` reader - NUM_TZMA
pub type NUM_TZMA_R = crate::FieldReader;
///Field `NUM_PER_SEC` reader - NUM_PER_SEC
pub type NUM_PER_SEC_R = crate::FieldReader;
///Field `NUM_AHB_SEC` reader - NUM_AHB_SEC
pub type NUM_AHB_SEC_R = crate::FieldReader;
///Field `CHUNKS1N4` reader - CHUNKS1N4
pub type CHUNKS1N4_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - NUM_TZMA
    #[inline(always)]
    pub fn num_tzma(&self) -> NUM_TZMA_R {
        NUM_TZMA_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - NUM_PER_SEC
    #[inline(always)]
    pub fn num_per_sec(&self) -> NUM_PER_SEC_R {
        NUM_PER_SEC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - NUM_AHB_SEC
    #[inline(always)]
    pub fn num_ahb_sec(&self) -> NUM_AHB_SEC_R {
        NUM_AHB_SEC_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - CHUNKS1N4
    #[inline(always)]
    pub fn chunks1n4(&self) -> CHUNKS1N4_R {
        CHUNKS1N4_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HWCFGR")
            .field("num_tzma", &self.num_tzma())
            .field("num_per_sec", &self.num_per_sec())
            .field("num_ahb_sec", &self.num_ahb_sec())
            .field("chunks1n4", &self.chunks1n4())
            .finish()
    }
}
/**ETZPC IP HW configuration register

You can [`read`](crate::Reg::read) this register and get [`hwcfgr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETZPC:HWCFGR)*/
pub struct HWCFGRrs;
impl crate::RegisterSpec for HWCFGRrs {
    type Ux = u32;
}
///`read()` method returns [`hwcfgr::R`](R) reader structure
impl crate::Readable for HWCFGRrs {}
///`reset()` method sets HWCFGR to value 0x6002
impl crate::Resettable for HWCFGRrs {
    const RESET_VALUE: u32 = 0x6002;
}
