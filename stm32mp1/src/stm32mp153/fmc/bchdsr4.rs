///Register `BCHDSR4` reader
pub type R = crate::R<BCHDSR4rs>;
///Field `EBP7` reader - EBP7
pub type EBP7_R = crate::FieldReader<u16>;
///Field `EBP8` reader - EBP8
pub type EBP8_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:12 - EBP7
    #[inline(always)]
    pub fn ebp7(&self) -> EBP7_R {
        EBP7_R::new((self.bits & 0x1fff) as u16)
    }
    ///Bits 16:28 - EBP8
    #[inline(always)]
    pub fn ebp8(&self) -> EBP8_R {
        EBP8_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BCHDSR4")
            .field("ebp7", &self.ebp7())
            .field("ebp8", &self.ebp8())
            .finish()
    }
}
/**The maximum error correction capability of the BCH block embedded in the FMC is 8 errors. This register contains the positions of the 7th and 8th error bits in EBP7 and EPB8 fields, respectively. .

You can [`read`](crate::Reg::read) this register and get [`bchdsr4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FMC:BCHDSR4)*/
pub struct BCHDSR4rs;
impl crate::RegisterSpec for BCHDSR4rs {
    type Ux = u32;
}
///`read()` method returns [`bchdsr4::R`](R) reader structure
impl crate::Readable for BCHDSR4rs {}
///`reset()` method sets BCHDSR4 to value 0
impl crate::Resettable for BCHDSR4rs {}
