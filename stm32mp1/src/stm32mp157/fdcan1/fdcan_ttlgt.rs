///Register `FDCAN_TTLGT` reader
pub type R = crate::R<FDCAN_TTLGTrs>;
///Field `LT` reader - LT
pub type LT_R = crate::FieldReader<u16>;
///Field `GT` reader - GT
pub type GT_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - LT
    #[inline(always)]
    pub fn lt(&self) -> LT_R {
        LT_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - GT
    #[inline(always)]
    pub fn gt(&self) -> GT_R {
        GT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_TTLGT")
            .field("lt", &self.lt())
            .field("gt", &self.gt())
            .finish()
    }
}
/**FDCAN TT local and global time register

You can [`read`](crate::Reg::read) this register and get [`fdcan_ttlgt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FDCAN1:FDCAN_TTLGT)*/
pub struct FDCAN_TTLGTrs;
impl crate::RegisterSpec for FDCAN_TTLGTrs {
    type Ux = u32;
}
///`read()` method returns [`fdcan_ttlgt::R`](R) reader structure
impl crate::Readable for FDCAN_TTLGTrs {}
///`reset()` method sets FDCAN_TTLGT to value 0
impl crate::Resettable for FDCAN_TTLGTrs {
    const RESET_VALUE: u32 = 0;
}
