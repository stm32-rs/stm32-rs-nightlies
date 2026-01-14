///Register `TTLGT` reader
pub type R = crate::R<TTLGTrs>;
///Field `LT` reader - Local time
pub type LT_R = crate::FieldReader<u16>;
///Field `GT` reader - Global time
pub type GT_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - Local time
    #[inline(always)]
    pub fn lt(&self) -> LT_R {
        LT_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - Global time
    #[inline(always)]
    pub fn gt(&self) -> GT_R {
        GT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TTLGT")
            .field("lt", &self.lt())
            .field("gt", &self.gt())
            .finish()
    }
}
/**FDCAN TT local and global time register

You can [`read`](crate::Reg::read) this register and get [`ttlgt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#FDCAN1:TTLGT)*/
pub struct TTLGTrs;
impl crate::RegisterSpec for TTLGTrs {
    type Ux = u32;
}
///`read()` method returns [`ttlgt::R`](R) reader structure
impl crate::Readable for TTLGTrs {}
///`reset()` method sets TTLGT to value 0
impl crate::Resettable for TTLGTrs {}
