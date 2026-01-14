///Register `ISR` reader
pub type R = crate::R<ISRrs>;
///Field `EOPF` reader - End-of-program flag This bit is set when a programming operation completes. An interrupt is generated if the EOPIE is set. It is not necessary to reset EOPF before starting a new operation. Setting EOPF bit in FLASH_ICR register clears this bit.
pub type EOPF_R = crate::BitReader;
///Field `WRPERRF` reader - Write protection error flag This bit is set when a protection error occurs during a program operation. An interrupt is also generated if the WRPERRIE is set. Setting WRPERRF bit in FLASH_ICR register clears this bit.
pub type WRPERRF_R = crate::BitReader;
///Field `PGSERRF` reader - Programming sequence error flag This bit is set when a sequence error occurs. An interrupt is generated if the PGSERRIE bit is set. Setting PGSERRF bit in FLASH_ICR register clears this bit.
pub type PGSERRF_R = crate::BitReader;
///Field `STRBERRF` reader - Strobe error flag This bit is set when a strobe error occurs (when the master attempts to write several times the same byte in the write buffer). An interrupt is generated if the STRBERRIE bit is set. Setting STRBERRF bit in FLASH_ICR register clears this bit.
pub type STRBERRF_R = crate::BitReader;
///Field `OBLERRF` reader - Option byte loading error flag This bit is set when an error is found during the option byte loading sequence. An interrupt is generated if OBLERRIE is set. Setting OBLERRF bit in the FLASH_ICR register clears this bit.
pub type OBLERRF_R = crate::BitReader;
///Field `INCERRF` reader - Inconsistency error flag This bit is set when a inconsistency error occurs. An interrupt is generated if INCERRIE is set. Setting INCERRF bit in the FLASH_ICR register clears this bit.
pub type INCERRF_R = crate::BitReader;
///Field `RDSERRF` reader - Read security error flag This bit is set when a read security error occurs (read access to hide protected area with incorrect hide protection level). An interrupt is generated if RDSERRIE is set. Setting RDSERRF bit in FLASH_ICR register clears this bit.
pub type RDSERRF_R = crate::BitReader;
///Field `SNECCERRF` reader - ECC single error flag This bit is set when an ECC single correction error occurs during a read operation. An interrupt is generated if SNECCERRIE is set. Setting SNECCERRF bit in FLASH_ICR register clears this bit.
pub type SNECCERRF_R = crate::BitReader;
///Field `DBECCERRF` reader - ECC double error flag This bit is set when an ECC double detection error occurs during a read operation. An interrupt is generated if DBECCERRIE is set. Setting DBECCERRF bit in FLASH_ICR register clears this bit.
pub type DBECCERRF_R = crate::BitReader;
///Field `CRCENDF` reader - CRC end flag This bit is set when the CRC computation has completed. An interrupt is generated if CRCENDIE is set. It is not necessary to reset CRCEND before restarting CRC computation. Setting CRCENDF bit in FLASH_ICR register clears this bit.
pub type CRCENDF_R = crate::BitReader;
///Field `CRCRDERRF` reader - CRC read error flag This bit is set when a word is found read protected during a CRC operation. An interrupt is generated if CRCRDIE is set. Setting CRCRDERRF bit in FLASH_ICR register clears this bit. This flag is valid only when CRCEND bit is set.
pub type CRCRDERRF_R = crate::BitReader;
impl R {
    ///Bit 16 - End-of-program flag This bit is set when a programming operation completes. An interrupt is generated if the EOPIE is set. It is not necessary to reset EOPF before starting a new operation. Setting EOPF bit in FLASH_ICR register clears this bit.
    #[inline(always)]
    pub fn eopf(&self) -> EOPF_R {
        EOPF_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Write protection error flag This bit is set when a protection error occurs during a program operation. An interrupt is also generated if the WRPERRIE is set. Setting WRPERRF bit in FLASH_ICR register clears this bit.
    #[inline(always)]
    pub fn wrperrf(&self) -> WRPERRF_R {
        WRPERRF_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Programming sequence error flag This bit is set when a sequence error occurs. An interrupt is generated if the PGSERRIE bit is set. Setting PGSERRF bit in FLASH_ICR register clears this bit.
    #[inline(always)]
    pub fn pgserrf(&self) -> PGSERRF_R {
        PGSERRF_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Strobe error flag This bit is set when a strobe error occurs (when the master attempts to write several times the same byte in the write buffer). An interrupt is generated if the STRBERRIE bit is set. Setting STRBERRF bit in FLASH_ICR register clears this bit.
    #[inline(always)]
    pub fn strberrf(&self) -> STRBERRF_R {
        STRBERRF_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Option byte loading error flag This bit is set when an error is found during the option byte loading sequence. An interrupt is generated if OBLERRIE is set. Setting OBLERRF bit in the FLASH_ICR register clears this bit.
    #[inline(always)]
    pub fn oblerrf(&self) -> OBLERRF_R {
        OBLERRF_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Inconsistency error flag This bit is set when a inconsistency error occurs. An interrupt is generated if INCERRIE is set. Setting INCERRF bit in the FLASH_ICR register clears this bit.
    #[inline(always)]
    pub fn incerrf(&self) -> INCERRF_R {
        INCERRF_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 24 - Read security error flag This bit is set when a read security error occurs (read access to hide protected area with incorrect hide protection level). An interrupt is generated if RDSERRIE is set. Setting RDSERRF bit in FLASH_ICR register clears this bit.
    #[inline(always)]
    pub fn rdserrf(&self) -> RDSERRF_R {
        RDSERRF_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - ECC single error flag This bit is set when an ECC single correction error occurs during a read operation. An interrupt is generated if SNECCERRIE is set. Setting SNECCERRF bit in FLASH_ICR register clears this bit.
    #[inline(always)]
    pub fn sneccerrf(&self) -> SNECCERRF_R {
        SNECCERRF_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - ECC double error flag This bit is set when an ECC double detection error occurs during a read operation. An interrupt is generated if DBECCERRIE is set. Setting DBECCERRF bit in FLASH_ICR register clears this bit.
    #[inline(always)]
    pub fn dbeccerrf(&self) -> DBECCERRF_R {
        DBECCERRF_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - CRC end flag This bit is set when the CRC computation has completed. An interrupt is generated if CRCENDIE is set. It is not necessary to reset CRCEND before restarting CRC computation. Setting CRCENDF bit in FLASH_ICR register clears this bit.
    #[inline(always)]
    pub fn crcendf(&self) -> CRCENDF_R {
        CRCENDF_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - CRC read error flag This bit is set when a word is found read protected during a CRC operation. An interrupt is generated if CRCRDIE is set. Setting CRCRDERRF bit in FLASH_ICR register clears this bit. This flag is valid only when CRCEND bit is set.
    #[inline(always)]
    pub fn crcrderrf(&self) -> CRCRDERRF_R {
        CRCRDERRF_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR")
            .field("eopf", &self.eopf())
            .field("wrperrf", &self.wrperrf())
            .field("pgserrf", &self.pgserrf())
            .field("strberrf", &self.strberrf())
            .field("oblerrf", &self.oblerrf())
            .field("incerrf", &self.incerrf())
            .field("rdserrf", &self.rdserrf())
            .field("sneccerrf", &self.sneccerrf())
            .field("dbeccerrf", &self.dbeccerrf())
            .field("crcendf", &self.crcendf())
            .field("crcrderrf", &self.crcrderrf())
            .finish()
    }
}
/**FLASH interrupt status register

You can [`read`](crate::Reg::read) this register and get [`isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#FLASH:ISR)*/
pub struct ISRrs;
impl crate::RegisterSpec for ISRrs {
    type Ux = u32;
}
///`read()` method returns [`isr::R`](R) reader structure
impl crate::Readable for ISRrs {}
///`reset()` method sets ISR to value 0
impl crate::Resettable for ISRrs {}
