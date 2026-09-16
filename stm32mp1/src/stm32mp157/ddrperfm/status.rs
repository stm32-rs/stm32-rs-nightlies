///Register `STATUS` reader
pub type R = crate::R<STATUSrs>;
///Field `COVF` reader - COVF
pub type COVF_R = crate::FieldReader;
///Field `BUSY` reader - BUSY
pub type BUSY_R = crate::BitReader;
///Field `TOVF` reader - TOVF
pub type TOVF_R = crate::BitReader;
impl R {
    ///Bits 0:3 - COVF
    #[inline(always)]
    pub fn covf(&self) -> COVF_R {
        COVF_R::new((self.bits & 0x0f) as u8)
    }
    ///Bit 16 - BUSY
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 31 - TOVF
    #[inline(always)]
    pub fn tovf(&self) -> TOVF_R {
        TOVF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS")
            .field("covf", &self.covf())
            .field("busy", &self.busy())
            .field("tovf", &self.tovf())
            .finish()
    }
}
/**DDRPERFM status register

You can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPERFM:STATUS)*/
pub struct STATUSrs;
impl crate::RegisterSpec for STATUSrs {
    type Ux = u32;
}
///`read()` method returns [`status::R`](R) reader structure
impl crate::Readable for STATUSrs {}
///`reset()` method sets STATUS to value 0
impl crate::Resettable for STATUSrs {}
