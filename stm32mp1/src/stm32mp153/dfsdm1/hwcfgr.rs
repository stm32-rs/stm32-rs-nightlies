///Register `HWCFGR` reader
pub type R = crate::R<HWCFGRrs>;
///Field `NBT` reader - NBT
pub type NBT_R = crate::FieldReader;
///Field `NBF` reader - NBF
pub type NBF_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - NBT
    #[inline(always)]
    pub fn nbt(&self) -> NBT_R {
        NBT_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - NBF
    #[inline(always)]
    pub fn nbf(&self) -> NBF_R {
        NBF_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HWCFGR")
            .field("nbt", &self.nbt())
            .field("nbf", &self.nbf())
            .finish()
    }
}
/**This register specifies the hardware configuration of DFSDM peripheral.

You can [`read`](crate::Reg::read) this register and get [`hwcfgr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:HWCFGR)*/
pub struct HWCFGRrs;
impl crate::RegisterSpec for HWCFGRrs {
    type Ux = u32;
}
///`read()` method returns [`hwcfgr::R`](R) reader structure
impl crate::Readable for HWCFGRrs {}
///`reset()` method sets HWCFGR to value 0x0608
impl crate::Resettable for HWCFGRrs {
    const RESET_VALUE: u32 = 0x0608;
}
