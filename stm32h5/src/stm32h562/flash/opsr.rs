///Register `OPSR` reader
pub type R = crate::R<OPSRrs>;
///Field `ADDR_OP` reader - Interrupted operation address
pub type ADDR_OP_R = crate::FieldReader<u32>;
///Field `DATA_OP` reader - Flash high-cycle data area operation interrupted It indicates if flash high-cycle data area is concerned by operation.
pub type DATA_OP_R = crate::BitReader;
///Field `BK_OP` reader - Interrupted operation bank It indicates which bank was concerned by operation.
pub type BK_OP_R = crate::BitReader;
///Field `SYSF_OP` reader - Operation in system flash memory interrupted Indicates that reset interrupted an ongoing operation in system flash.
pub type SYSF_OP_R = crate::BitReader;
///Field `OTP_OP` reader - OTP operation interrupted Indicates that reset interrupted an ongoing operation in OTP area (or OBKeys area).
pub type OTP_OP_R = crate::BitReader;
///Field `CODE_OP` reader - Flash memory operation code
pub type CODE_OP_R = crate::FieldReader;
impl R {
    ///Bits 0:19 - Interrupted operation address
    #[inline(always)]
    pub fn addr_op(&self) -> ADDR_OP_R {
        ADDR_OP_R::new(self.bits & 0x000f_ffff)
    }
    ///Bit 21 - Flash high-cycle data area operation interrupted It indicates if flash high-cycle data area is concerned by operation.
    #[inline(always)]
    pub fn data_op(&self) -> DATA_OP_R {
        DATA_OP_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Interrupted operation bank It indicates which bank was concerned by operation.
    #[inline(always)]
    pub fn bk_op(&self) -> BK_OP_R {
        BK_OP_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Operation in system flash memory interrupted Indicates that reset interrupted an ongoing operation in system flash.
    #[inline(always)]
    pub fn sysf_op(&self) -> SYSF_OP_R {
        SYSF_OP_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - OTP operation interrupted Indicates that reset interrupted an ongoing operation in OTP area (or OBKeys area).
    #[inline(always)]
    pub fn otp_op(&self) -> OTP_OP_R {
        OTP_OP_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bits 29:31 - Flash memory operation code
    #[inline(always)]
    pub fn code_op(&self) -> CODE_OP_R {
        CODE_OP_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OPSR")
            .field("addr_op", &self.addr_op())
            .field("data_op", &self.data_op())
            .field("bk_op", &self.bk_op())
            .field("sysf_op", &self.sysf_op())
            .field("otp_op", &self.otp_op())
            .field("code_op", &self.code_op())
            .finish()
    }
}
/**FLASH operation status register

You can [`read`](crate::Reg::read) this register and get [`opsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#FLASH:OPSR)*/
pub struct OPSRrs;
impl crate::RegisterSpec for OPSRrs {
    type Ux = u32;
}
///`read()` method returns [`opsr::R`](R) reader structure
impl crate::Readable for OPSRrs {}
///`reset()` method sets OPSR to value 0
impl crate::Resettable for OPSRrs {}
