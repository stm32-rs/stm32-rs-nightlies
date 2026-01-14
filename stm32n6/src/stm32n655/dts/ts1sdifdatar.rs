///Register `TS1SDIFDATAR` reader
pub type R = crate::R<TS1SDIFDATARrs>;
///Field `SAMPLE_DATA` reader - Sample data.
pub type SAMPLE_DATA_R = crate::FieldReader<u16>;
///Field `SAMPLE_TYPE` reader - TS sample type
pub type SAMPLE_TYPE_R = crate::BitReader;
///Field `SAMPLE_FAULT` reader - Sample fault
pub type SAMPLE_FAULT_R = crate::BitReader;
impl R {
    ///Bits 0:15 - Sample data.
    #[inline(always)]
    pub fn sample_data(&self) -> SAMPLE_DATA_R {
        SAMPLE_DATA_R::new((self.bits & 0xffff) as u16)
    }
    ///Bit 16 - TS sample type
    #[inline(always)]
    pub fn sample_type(&self) -> SAMPLE_TYPE_R {
        SAMPLE_TYPE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Sample fault
    #[inline(always)]
    pub fn sample_fault(&self) -> SAMPLE_FAULT_R {
        SAMPLE_FAULT_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TS1SDIFDATAR")
            .field("sample_data", &self.sample_data())
            .field("sample_type", &self.sample_type())
            .field("sample_fault", &self.sample_fault())
            .finish()
    }
}
/**DTS TS1 SDIF data register

You can [`read`](crate::Reg::read) this register and get [`ts1sdifdatar::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#DTS:TS1SDIFDATAR)*/
pub struct TS1SDIFDATARrs;
impl crate::RegisterSpec for TS1SDIFDATARrs {
    type Ux = u32;
}
///`read()` method returns [`ts1sdifdatar::R`](R) reader structure
impl crate::Readable for TS1SDIFDATARrs {}
///`reset()` method sets TS1SDIFDATAR to value 0
impl crate::Resettable for TS1SDIFDATARrs {}
