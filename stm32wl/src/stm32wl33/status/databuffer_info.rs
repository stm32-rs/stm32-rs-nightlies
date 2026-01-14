///Register `DATABUFFER_INFO` reader
pub type R = crate::R<DATABUFFER_INFOrs>;
///Field `CURRENT_DATABUFFER_COUNT` reader - Indicates the number of bytes used in the last used DATA BUFFER.
pub type CURRENT_DATABUFFER_COUNT_R = crate::FieldReader<u16>;
///Field `NB_DATABUFFER_USED` reader - Provides the number of data buffers which have been fully used
pub type NB_DATABUFFER_USED_R = crate::FieldReader<u16>;
///Field `CURRENT_DATABUFFER` reader - Indicates which Data Buffer is currently used by the HW
pub type CURRENT_DATABUFFER_R = crate::BitReader;
impl R {
    ///Bits 0:15 - Indicates the number of bytes used in the last used DATA BUFFER.
    #[inline(always)]
    pub fn current_databuffer_count(&self) -> CURRENT_DATABUFFER_COUNT_R {
        CURRENT_DATABUFFER_COUNT_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:30 - Provides the number of data buffers which have been fully used
    #[inline(always)]
    pub fn nb_databuffer_used(&self) -> NB_DATABUFFER_USED_R {
        NB_DATABUFFER_USED_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
    ///Bit 31 - Indicates which Data Buffer is currently used by the HW
    #[inline(always)]
    pub fn current_databuffer(&self) -> CURRENT_DATABUFFER_R {
        CURRENT_DATABUFFER_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATABUFFER_INFO")
            .field("current_databuffer_count", &self.current_databuffer_count())
            .field("nb_databuffer_used", &self.nb_databuffer_used())
            .field("current_databuffer", &self.current_databuffer())
            .finish()
    }
}
/**DATABUFFER_INFO register

You can [`read`](crate::Reg::read) this register and get [`databuffer_info::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#STATUS:DATABUFFER_INFO)*/
pub struct DATABUFFER_INFOrs;
impl crate::RegisterSpec for DATABUFFER_INFOrs {
    type Ux = u32;
}
///`read()` method returns [`databuffer_info::R`](R) reader structure
impl crate::Readable for DATABUFFER_INFOrs {}
///`reset()` method sets DATABUFFER_INFO to value 0
impl crate::Resettable for DATABUFFER_INFOrs {}
