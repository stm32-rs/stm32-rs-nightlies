///Register `CDR` reader
pub type R = crate::R<CDRrs>;
///Field `DATA1` reader - 1st data item of a pair of regular conversions
pub type DATA1_R = crate::FieldReader<u16>;
///Field `DATA2` reader - 2nd data item of a pair of regular conversions
pub type DATA2_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - 1st data item of a pair of regular conversions
    #[inline(always)]
    pub fn data1(&self) -> DATA1_R {
        DATA1_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - 2nd data item of a pair of regular conversions
    #[inline(always)]
    pub fn data2(&self) -> DATA2_R {
        DATA2_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CDR")
            .field("data2", &self.data2())
            .field("data1", &self.data1())
            .finish()
    }
}
/**ADC common regular data register for dual and triple modes

You can [`read`](crate::Reg::read) this register and get [`cdr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F733.html#ADC_Common:CDR)*/
pub struct CDRrs;
impl crate::RegisterSpec for CDRrs {
    type Ux = u32;
}
///`read()` method returns [`cdr::R`](R) reader structure
impl crate::Readable for CDRrs {}
///`reset()` method sets CDR to value 0
impl crate::Resettable for CDRrs {}
