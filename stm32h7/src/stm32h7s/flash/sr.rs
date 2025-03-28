///Register `SR` reader
pub type R = crate::R<SRrs>;
///Field `BUSY` reader - Busy flag This bit is set when an effective write, erase or option byte change operation is ongoing. It is possible to know what type of operation is being executed reading the flags IS_PROGRAM, IS_ERASE and IS_OPTCHANGE. BUSY cannot be cleared by application. It is automatically reset by hardware every time a step in a write, erase or option byte change operation completes. It is not recommended to do software polling on BUSY to know when one operation completed because, depending of operation, more pulses are possible for one only operation. For software polling it is therefore better to use QW flag or to check the EOPF flag.
pub type BUSY_R = crate::BitReader;
///Field `WBNE` reader - Write buffer not empty flag This bit is set when the embedded Flash memory is waiting for new data to complete the write buffer. In this state, the write buffer is not empty. WBNE is reset by hardware each time the write buffer is complete or the write buffer is emptied following one of the event below: the application software forces the write operation using FW bit in FLASH_CR the embedded Flash memory detects an error that involves data loss the application software has disabled write operations This bit cannot be forced to 0. To reset it, clear the write buffer by performing any of the above listed actions, or send the missing data.
pub type WBNE_R = crate::BitReader;
///Field `QW` reader - Wait queue flag This bit is set when a write, erase or option byte change operation is pending in the command queue buffer. It is not possible to know what type of programming operation is present in the queue. This flag is reset by hardware when all write, erase or option byte change operations have been executed and thus removed from the waiting queue(s). This bit cannot be forced to 0. It is reset after a deterministic time if no other operations are requested.
pub type QW_R = crate::BitReader;
///Field `CRC_BUSY` reader - CRC busy flag This bit is set when a CRC calculation is ongoing. This bit cannot be forced to 0. The user must wait until the CRC calculation has completed or disable CRC computation using CRC_EN bit in FLASH_CR register.
pub type CRC_BUSY_R = crate::BitReader;
///Field `IS_PROGRAM` reader - Is a program This bit is set together with BUSY when a program operation is ongoing. It is cleared when BUSY is cleared. This flag can also raise with IS_OPTCHANGE, because an program operation can happen during an option change.
pub type IS_PROGRAM_R = crate::BitReader;
///Field `IS_ERASE` reader - Is an erase This bit is set together with BUSY when an erase operation is ongoing. It is cleared when BUSY is cleared. This flag can also raise with IS_OPTCHANGE, because an erase operation can happen during an option change.
pub type IS_ERASE_R = crate::BitReader;
///Field `IS_OPTCHANGE` reader - Is an option change This bit is set together with BUSY when an option change operation is ongoing. It is cleared when BUSY is cleared. This flag can also raise with IS_PROGRAM or IS_ERASE, because a program or erase step is ongoing during option change.
pub type IS_OPTCHANGE_R = crate::BitReader;
///Field `RCHECKF` reader - Root code check flag This bit returns the status of the root code check performed following the first access to the Flash. This bit is cleared with RCHECKF bit in FLASH_FCR (optional).
pub type RCHECKF_R = crate::BitReader;
impl R {
    ///Bit 0 - Busy flag This bit is set when an effective write, erase or option byte change operation is ongoing. It is possible to know what type of operation is being executed reading the flags IS_PROGRAM, IS_ERASE and IS_OPTCHANGE. BUSY cannot be cleared by application. It is automatically reset by hardware every time a step in a write, erase or option byte change operation completes. It is not recommended to do software polling on BUSY to know when one operation completed because, depending of operation, more pulses are possible for one only operation. For software polling it is therefore better to use QW flag or to check the EOPF flag.
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Write buffer not empty flag This bit is set when the embedded Flash memory is waiting for new data to complete the write buffer. In this state, the write buffer is not empty. WBNE is reset by hardware each time the write buffer is complete or the write buffer is emptied following one of the event below: the application software forces the write operation using FW bit in FLASH_CR the embedded Flash memory detects an error that involves data loss the application software has disabled write operations This bit cannot be forced to 0. To reset it, clear the write buffer by performing any of the above listed actions, or send the missing data.
    #[inline(always)]
    pub fn wbne(&self) -> WBNE_R {
        WBNE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Wait queue flag This bit is set when a write, erase or option byte change operation is pending in the command queue buffer. It is not possible to know what type of programming operation is present in the queue. This flag is reset by hardware when all write, erase or option byte change operations have been executed and thus removed from the waiting queue(s). This bit cannot be forced to 0. It is reset after a deterministic time if no other operations are requested.
    #[inline(always)]
    pub fn qw(&self) -> QW_R {
        QW_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - CRC busy flag This bit is set when a CRC calculation is ongoing. This bit cannot be forced to 0. The user must wait until the CRC calculation has completed or disable CRC computation using CRC_EN bit in FLASH_CR register.
    #[inline(always)]
    pub fn crc_busy(&self) -> CRC_BUSY_R {
        CRC_BUSY_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Is a program This bit is set together with BUSY when a program operation is ongoing. It is cleared when BUSY is cleared. This flag can also raise with IS_OPTCHANGE, because an program operation can happen during an option change.
    #[inline(always)]
    pub fn is_program(&self) -> IS_PROGRAM_R {
        IS_PROGRAM_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Is an erase This bit is set together with BUSY when an erase operation is ongoing. It is cleared when BUSY is cleared. This flag can also raise with IS_OPTCHANGE, because an erase operation can happen during an option change.
    #[inline(always)]
    pub fn is_erase(&self) -> IS_ERASE_R {
        IS_ERASE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Is an option change This bit is set together with BUSY when an option change operation is ongoing. It is cleared when BUSY is cleared. This flag can also raise with IS_PROGRAM or IS_ERASE, because a program or erase step is ongoing during option change.
    #[inline(always)]
    pub fn is_optchange(&self) -> IS_OPTCHANGE_R {
        IS_OPTCHANGE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 25 - Root code check flag This bit returns the status of the root code check performed following the first access to the Flash. This bit is cleared with RCHECKF bit in FLASH_FCR (optional).
    #[inline(always)]
    pub fn rcheckf(&self) -> RCHECKF_R {
        RCHECKF_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("busy", &self.busy())
            .field("wbne", &self.wbne())
            .field("qw", &self.qw())
            .field("crc_busy", &self.crc_busy())
            .field("is_program", &self.is_program())
            .field("is_erase", &self.is_erase())
            .field("is_optchange", &self.is_optchange())
            .field("rcheckf", &self.rcheckf())
            .finish()
    }
}
/**FLASH status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#FLASH:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`reset()` method sets SR to value 0
impl crate::Resettable for SRrs {}
