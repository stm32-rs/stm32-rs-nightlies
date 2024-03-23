#[doc = "Register `SECSR` reader"]
pub type R = crate::R<SECSRrs>;
#[doc = "Field `SECBSY` reader - busy flag BSY flag indicates that a FLASH memory is busy by an operation (write, erase, option byte change, OBK operations, PUF operation). It is set at the beginning of a Flash memory operation and cleared when the operation finishes or an error occurs."]
pub type SECBSY_R = crate::BitReader;
#[doc = "Field `SECWBNE` reader - write buffer not empty flag WBNE flag is set when the embedded Flash memory is waiting for new data to complete the write buffer. In this state, the write buffer is not empty. WBNE is reset by hardware each time the write buffer is complete or the write buffer is emptied following one of the event below: the application software forces the write operation using FW bit in FLASH_SECCR the embedded Flash memory detects an error that involves data loss This bit cannot be reset by writing 0 directly by software. To reset it, clear the write buffer by performing any of the above listed actions, or send the missing data."]
pub type SECWBNE_R = crate::BitReader;
#[doc = "Field `SECDBNE` reader - data buffer not empty flag DBNE flag is set when the embedded Flash memory interface is processing 6-bits ECC data in dedicated buffer. This bit cannot be set to 0 by software. The hardware resets it once the buffer is free."]
pub type SECDBNE_R = crate::BitReader;
#[doc = "Field `SECEOP` reader - end of operation flag EOP flag is set when a operation (program/erase) completes. An interrupt is generated if the EOPIE is set to. It is not necessary to reset EOP before starting a new operation. EOP bit is cleared by writing 1 to CLR_EOP bit in FLASH_SECCCR register."]
pub type SECEOP_R = crate::BitReader;
#[doc = "Field `SECWRPERR` reader - write protection error flag WRPERR flag is raised when a protection error occurs during a program operation. An interrupt is also generated if the WRPERRIE is set to 1. Writing 1 to CLR_WRPERR bit in FLASH_SECCCR register clears WRPERR."]
pub type SECWRPERR_R = crate::BitReader;
#[doc = "Field `SECPGSERR` reader - programming sequence error flag PGSERR flag is raised when a sequence error occurs. An interrupt is generated if the PGSERRIE bit is set to 1. Writing 1 to CLR_PGSERR bit in FLASH_SECCCR register clears PGSERR."]
pub type SECPGSERR_R = crate::BitReader;
#[doc = "Field `SECSTRBERR` reader - strobe error flag STRBERR flag is raised when a strobe error occurs (when the master attempts to write several times the same byte in the write buffer). An interrupt is generated if the STRBERRIE bit is set to 1. Writing 1 to CLR_STRBERR bit in FLASH_SECCCR register clears STRBERR."]
pub type SECSTRBERR_R = crate::BitReader;
#[doc = "Field `SECINCERR` reader - inconsistency error flag INCERR flag is raised when a inconsistency error occurs. An interrupt is generated if INCERRIE is set to 1. Writing 1 to CLR_INCERR bit in the FLASH_SECCCR register clears INCERR."]
pub type SECINCERR_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - busy flag BSY flag indicates that a FLASH memory is busy by an operation (write, erase, option byte change, OBK operations, PUF operation). It is set at the beginning of a Flash memory operation and cleared when the operation finishes or an error occurs."]
    #[inline(always)]
    pub fn secbsy(&self) -> SECBSY_R {
        SECBSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - write buffer not empty flag WBNE flag is set when the embedded Flash memory is waiting for new data to complete the write buffer. In this state, the write buffer is not empty. WBNE is reset by hardware each time the write buffer is complete or the write buffer is emptied following one of the event below: the application software forces the write operation using FW bit in FLASH_SECCR the embedded Flash memory detects an error that involves data loss This bit cannot be reset by writing 0 directly by software. To reset it, clear the write buffer by performing any of the above listed actions, or send the missing data."]
    #[inline(always)]
    pub fn secwbne(&self) -> SECWBNE_R {
        SECWBNE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - data buffer not empty flag DBNE flag is set when the embedded Flash memory interface is processing 6-bits ECC data in dedicated buffer. This bit cannot be set to 0 by software. The hardware resets it once the buffer is free."]
    #[inline(always)]
    pub fn secdbne(&self) -> SECDBNE_R {
        SECDBNE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - end of operation flag EOP flag is set when a operation (program/erase) completes. An interrupt is generated if the EOPIE is set to. It is not necessary to reset EOP before starting a new operation. EOP bit is cleared by writing 1 to CLR_EOP bit in FLASH_SECCCR register."]
    #[inline(always)]
    pub fn seceop(&self) -> SECEOP_R {
        SECEOP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - write protection error flag WRPERR flag is raised when a protection error occurs during a program operation. An interrupt is also generated if the WRPERRIE is set to 1. Writing 1 to CLR_WRPERR bit in FLASH_SECCCR register clears WRPERR."]
    #[inline(always)]
    pub fn secwrperr(&self) -> SECWRPERR_R {
        SECWRPERR_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - programming sequence error flag PGSERR flag is raised when a sequence error occurs. An interrupt is generated if the PGSERRIE bit is set to 1. Writing 1 to CLR_PGSERR bit in FLASH_SECCCR register clears PGSERR."]
    #[inline(always)]
    pub fn secpgserr(&self) -> SECPGSERR_R {
        SECPGSERR_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - strobe error flag STRBERR flag is raised when a strobe error occurs (when the master attempts to write several times the same byte in the write buffer). An interrupt is generated if the STRBERRIE bit is set to 1. Writing 1 to CLR_STRBERR bit in FLASH_SECCCR register clears STRBERR."]
    #[inline(always)]
    pub fn secstrberr(&self) -> SECSTRBERR_R {
        SECSTRBERR_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - inconsistency error flag INCERR flag is raised when a inconsistency error occurs. An interrupt is generated if INCERRIE is set to 1. Writing 1 to CLR_INCERR bit in the FLASH_SECCCR register clears INCERR."]
    #[inline(always)]
    pub fn secincerr(&self) -> SECINCERR_R {
        SECINCERR_R::new(((self.bits >> 20) & 1) != 0)
    }
}
#[doc = "FLASH secure status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`secsr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SECSRrs;
impl crate::RegisterSpec for SECSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secsr::R`](R) reader structure"]
impl crate::Readable for SECSRrs {}
#[doc = "`reset()` method sets SECSR to value 0"]
impl crate::Resettable for SECSRrs {
    const RESET_VALUE: u32 = 0;
}
