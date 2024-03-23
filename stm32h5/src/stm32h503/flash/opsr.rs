#[doc = "Register `OPSR` reader"]
pub type R = crate::R<OPSRrs>;
#[doc = "Field `ADDR_OP` reader - Interrupted operation address."]
pub type ADDR_OP_R = crate::FieldReader<u32>;
#[doc = "Field `BK_OP` reader - Interrupted operation bank It indicates which bank was concerned by operation."]
pub type BK_OP_R = crate::BitReader;
#[doc = "Field `SYSF_OP` reader - Operation in system Flash memory interrupted Indicates that reset interrupted an ongoing operation in System Flash."]
pub type SYSF_OP_R = crate::BitReader;
#[doc = "Field `OTP_OP` reader - OTP operation interrupted Indicates that reset interrupted an ongoing operation in OTP area."]
pub type OTP_OP_R = crate::BitReader;
#[doc = "Field `CODE_OP` reader - Flash memory operation code"]
pub type CODE_OP_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:19 - Interrupted operation address."]
    #[inline(always)]
    pub fn addr_op(&self) -> ADDR_OP_R {
        ADDR_OP_R::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bit 22 - Interrupted operation bank It indicates which bank was concerned by operation."]
    #[inline(always)]
    pub fn bk_op(&self) -> BK_OP_R {
        BK_OP_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Operation in system Flash memory interrupted Indicates that reset interrupted an ongoing operation in System Flash."]
    #[inline(always)]
    pub fn sysf_op(&self) -> SYSF_OP_R {
        SYSF_OP_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - OTP operation interrupted Indicates that reset interrupted an ongoing operation in OTP area."]
    #[inline(always)]
    pub fn otp_op(&self) -> OTP_OP_R {
        OTP_OP_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 29:31 - Flash memory operation code"]
    #[inline(always)]
    pub fn code_op(&self) -> CODE_OP_R {
        CODE_OP_R::new(((self.bits >> 29) & 7) as u8)
    }
}
#[doc = "FLASH operation status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opsr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OPSRrs;
impl crate::RegisterSpec for OPSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`opsr::R`](R) reader structure"]
impl crate::Readable for OPSRrs {}
#[doc = "`reset()` method sets OPSR to value 0"]
impl crate::Resettable for OPSRrs {
    const RESET_VALUE: u32 = 0;
}
