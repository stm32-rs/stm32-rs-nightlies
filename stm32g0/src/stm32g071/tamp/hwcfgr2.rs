///Register `HWCFGR2` reader
pub type R = crate::R<HWCFGR2rs>;
///Field `PTIONREG_OUT` reader - PTIONREG_OUT
pub type PTIONREG_OUT_R = crate::FieldReader;
///Field `TRUST_ZONE` reader - TRUST_ZONE
pub type TRUST_ZONE_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - PTIONREG_OUT
    #[inline(always)]
    pub fn ptionreg_out(&self) -> PTIONREG_OUT_R {
        PTIONREG_OUT_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - TRUST_ZONE
    #[inline(always)]
    pub fn trust_zone(&self) -> TRUST_ZONE_R {
        TRUST_ZONE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HWCFGR2")
            .field("ptionreg_out", &self.ptionreg_out())
            .field("trust_zone", &self.trust_zone())
            .finish()
    }
}
/**TAMP hardware configuration register 2

You can [`read`](crate::Reg::read) this register and get [`hwcfgr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G071.html#TAMP:HWCFGR2)*/
pub struct HWCFGR2rs;
impl crate::RegisterSpec for HWCFGR2rs {
    type Ux = u32;
}
///`read()` method returns [`hwcfgr2::R`](R) reader structure
impl crate::Readable for HWCFGR2rs {}
///`reset()` method sets HWCFGR2 to value 0
impl crate::Resettable for HWCFGR2rs {}
