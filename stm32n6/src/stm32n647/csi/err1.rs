///Register `ERR1` reader
pub type R = crate::R<ERR1rs>;
///Field `CRCDTERR` reader - Data type having a CRC error
pub type CRCDTERR_R = crate::FieldReader;
///Field `CRCVCERR` reader - Virtual channel having a CRC error
pub type CRCVCERR_R = crate::FieldReader;
///Field `CECCDTERR` reader - Data type having a corrected ECC error
pub type CECCDTERR_R = crate::FieldReader;
///Field `CECCVCERR` reader - Virtual channel having a corrected ECC error
pub type CECCVCERR_R = crate::FieldReader;
///Field `IDDTERR` reader - Data type in error
pub type IDDTERR_R = crate::FieldReader;
///Field `IDVCERR` reader - Virtual channel having ID error
pub type IDVCERR_R = crate::FieldReader;
impl R {
    ///Bits 0:5 - Data type having a CRC error
    #[inline(always)]
    pub fn crcdterr(&self) -> CRCDTERR_R {
        CRCDTERR_R::new((self.bits & 0x3f) as u8)
    }
    ///Bits 6:7 - Virtual channel having a CRC error
    #[inline(always)]
    pub fn crcvcerr(&self) -> CRCVCERR_R {
        CRCVCERR_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:13 - Data type having a corrected ECC error
    #[inline(always)]
    pub fn ceccdterr(&self) -> CECCDTERR_R {
        CECCDTERR_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    ///Bits 14:15 - Virtual channel having a corrected ECC error
    #[inline(always)]
    pub fn ceccvcerr(&self) -> CECCVCERR_R {
        CECCVCERR_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bits 16:21 - Data type in error
    #[inline(always)]
    pub fn iddterr(&self) -> IDDTERR_R {
        IDDTERR_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    ///Bits 22:23 - Virtual channel having ID error
    #[inline(always)]
    pub fn idvcerr(&self) -> IDVCERR_R {
        IDVCERR_R::new(((self.bits >> 22) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ERR1")
            .field("crcdterr", &self.crcdterr())
            .field("crcvcerr", &self.crcvcerr())
            .field("ceccdterr", &self.ceccdterr())
            .field("ceccvcerr", &self.ceccvcerr())
            .field("iddterr", &self.iddterr())
            .field("idvcerr", &self.idvcerr())
            .finish()
    }
}
/**CSI-2 Host error register 1

You can [`read`](crate::Reg::read) this register and get [`err1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#CSI:ERR1)*/
pub struct ERR1rs;
impl crate::RegisterSpec for ERR1rs {
    type Ux = u32;
}
///`read()` method returns [`err1::R`](R) reader structure
impl crate::Readable for ERR1rs {}
///`reset()` method sets ERR1 to value 0
impl crate::Resettable for ERR1rs {}
