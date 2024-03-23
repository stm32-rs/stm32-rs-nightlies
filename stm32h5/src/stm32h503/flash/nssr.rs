#[doc = "Register `NSSR` reader"]
pub type R = crate::R<NSSRrs>;
#[doc = "Field `BSY` reader - busy flag BSY flag indicates that a Flash memory is busy by an operation (write, erase, option byte change). It is set at the beginning of a Flash memory operation and cleared when the operation finishes or an error occurs."]
pub type BSY_R = crate::BitReader;
#[doc = "Field `WBNE` reader - write buffer not empty flag WBNE flag is set when the embedded Flash memory is waiting for new data to complete the write buffer. In this state, the write buffer is not empty. WBNE is reset by hardware each time the write buffer is complete or the write buffer is emptied following one of the event below: the application software forces the write operation using FW bit in FLASH_NSCR the embedded Flash memory detects an error that involves data loss This bit cannot be reset by software writing 0 directly. To reset it, clear the write buffer by performing any of the above listed actions, or send the missing data."]
pub type WBNE_R = crate::BitReader;
#[doc = "Field `DBNE` reader - data buffer not empty flag DBNE flag is set when the embedded Flash memory interface is processing 6-bits ECC data in dedicated buffer. This bit cannot be set to 0 by software. The hardware resets it once the buffer is free."]
pub type DBNE_R = crate::BitReader;
#[doc = "Field `EOP` reader - end of operation flag EOP flag is set when a operation (program/erase) completes. An interrupt is generated if the EOPIE is set to 1. It is not necessary to reset EOP before starting a new operation. EOP bit is cleared by writing 1 to CLR_EOP bit in FLASH_NSCCR register."]
pub type EOP_R = crate::BitReader;
#[doc = "Field `WRPERR` reader - write protection error flag WRPERR flag is raised when a protection error occurs during a program operation. An interrupt is also generated if the WRPERRIE is set to 1. Writing 1 to CLR_WRPERR bit in FLASH_NSCCR register clears WRPERR."]
pub type WRPERR_R = crate::BitReader;
#[doc = "Field `PGSERR` reader - programming sequence error flag PGSERR flag is raised when a sequence error occurs. An interrupt is generated if the PGSERRIE bit is set to 1. Writing 1 to CLR_PGSERR bit in FLASH_NSCCR register clears PGSERR."]
pub type PGSERR_R = crate::BitReader;
#[doc = "Field `STRBERR` reader - strobe error flag STRBERR flag is raised when a strobe error occurs (when the master attempts to write several times the same byte in the write buffer). An interrupt is generated if the STRBERRIE bit is set to 1. Writing 1 to CLR_STRBERR bit in FLASH_NSCCR register clears STRBERR."]
pub type STRBERR_R = crate::BitReader;
#[doc = "Field `INCERR` reader - inconsistency error flag INCERR flag is raised when a inconsistency error occurs. An interrupt is generated if INCERRIE is set to 1. Writing 1 to CLR_INCERR bit in the FLASH_NSCCR register clears INCERR."]
pub type INCERR_R = crate::BitReader;
#[doc = "Field `OPTCHANGEERR` reader - Option byte change error flag OPTCHANGEERR flag indicates that an error occurred during an option byte change operation. When OPTCHANGEERR is set to 1, the option byte change operation did not successfully complete. An interrupt is generated when this flag is raised if the OPTCHANGEERRIE bit of FLASH_NSCR register is set to 1. Writing 1 to CLR_OPTCHANGEERR of register FLASH_CCR clears OPTCHANGEERR. Note: The OPTSTRT bit in FLASH_OPTCR cannot be set while OPTCHANGEERR is set."]
pub type OPTCHANGEERR_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - busy flag BSY flag indicates that a Flash memory is busy by an operation (write, erase, option byte change). It is set at the beginning of a Flash memory operation and cleared when the operation finishes or an error occurs."]
    #[inline(always)]
    pub fn bsy(&self) -> BSY_R {
        BSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - write buffer not empty flag WBNE flag is set when the embedded Flash memory is waiting for new data to complete the write buffer. In this state, the write buffer is not empty. WBNE is reset by hardware each time the write buffer is complete or the write buffer is emptied following one of the event below: the application software forces the write operation using FW bit in FLASH_NSCR the embedded Flash memory detects an error that involves data loss This bit cannot be reset by software writing 0 directly. To reset it, clear the write buffer by performing any of the above listed actions, or send the missing data."]
    #[inline(always)]
    pub fn wbne(&self) -> WBNE_R {
        WBNE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - data buffer not empty flag DBNE flag is set when the embedded Flash memory interface is processing 6-bits ECC data in dedicated buffer. This bit cannot be set to 0 by software. The hardware resets it once the buffer is free."]
    #[inline(always)]
    pub fn dbne(&self) -> DBNE_R {
        DBNE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - end of operation flag EOP flag is set when a operation (program/erase) completes. An interrupt is generated if the EOPIE is set to 1. It is not necessary to reset EOP before starting a new operation. EOP bit is cleared by writing 1 to CLR_EOP bit in FLASH_NSCCR register."]
    #[inline(always)]
    pub fn eop(&self) -> EOP_R {
        EOP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - write protection error flag WRPERR flag is raised when a protection error occurs during a program operation. An interrupt is also generated if the WRPERRIE is set to 1. Writing 1 to CLR_WRPERR bit in FLASH_NSCCR register clears WRPERR."]
    #[inline(always)]
    pub fn wrperr(&self) -> WRPERR_R {
        WRPERR_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - programming sequence error flag PGSERR flag is raised when a sequence error occurs. An interrupt is generated if the PGSERRIE bit is set to 1. Writing 1 to CLR_PGSERR bit in FLASH_NSCCR register clears PGSERR."]
    #[inline(always)]
    pub fn pgserr(&self) -> PGSERR_R {
        PGSERR_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - strobe error flag STRBERR flag is raised when a strobe error occurs (when the master attempts to write several times the same byte in the write buffer). An interrupt is generated if the STRBERRIE bit is set to 1. Writing 1 to CLR_STRBERR bit in FLASH_NSCCR register clears STRBERR."]
    #[inline(always)]
    pub fn strberr(&self) -> STRBERR_R {
        STRBERR_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - inconsistency error flag INCERR flag is raised when a inconsistency error occurs. An interrupt is generated if INCERRIE is set to 1. Writing 1 to CLR_INCERR bit in the FLASH_NSCCR register clears INCERR."]
    #[inline(always)]
    pub fn incerr(&self) -> INCERR_R {
        INCERR_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 23 - Option byte change error flag OPTCHANGEERR flag indicates that an error occurred during an option byte change operation. When OPTCHANGEERR is set to 1, the option byte change operation did not successfully complete. An interrupt is generated when this flag is raised if the OPTCHANGEERRIE bit of FLASH_NSCR register is set to 1. Writing 1 to CLR_OPTCHANGEERR of register FLASH_CCR clears OPTCHANGEERR. Note: The OPTSTRT bit in FLASH_OPTCR cannot be set while OPTCHANGEERR is set."]
    #[inline(always)]
    pub fn optchangeerr(&self) -> OPTCHANGEERR_R {
        OPTCHANGEERR_R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[doc = "FLASH non-secure status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nssr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NSSRrs;
impl crate::RegisterSpec for NSSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nssr::R`](R) reader structure"]
impl crate::Readable for NSSRrs {}
#[doc = "`reset()` method sets NSSR to value 0"]
impl crate::Resettable for NSSRrs {
    const RESET_VALUE: u32 = 0;
}
