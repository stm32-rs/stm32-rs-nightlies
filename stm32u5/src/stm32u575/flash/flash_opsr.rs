#[doc = "Register `FLASH_OPSR` reader"]
pub type R = crate::R<FLASH_OPSRrs>;
#[doc = "Field `ADDR_OP` reader - Interrupted operation address This field indicates which address in the Flash memory was accessed when reset occurred. The address is given by bank from address 0x0 0000 to 0xF FFF0."]
pub type ADDR_OP_R = crate::FieldReader<u32>;
#[doc = "Field `BK_OP` reader - Interrupted operation bank This bit indicates which Flash memory bank was accessed when reset occurred"]
pub type BK_OP_R = crate::BitReader;
#[doc = "Field `SYSF_OP` reader - Operation in system Flash memory interrupted This bit indicates that the reset occurred during an operation in the system Flash memory."]
pub type SYSF_OP_R = crate::BitReader;
#[doc = "Field `CODE_OP` reader - Flash memory operation code This field indicates which Flash memory operation has been interrupted by a system reset:"]
pub type CODE_OP_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:19 - Interrupted operation address This field indicates which address in the Flash memory was accessed when reset occurred. The address is given by bank from address 0x0 0000 to 0xF FFF0."]
    #[inline(always)]
    pub fn addr_op(&self) -> ADDR_OP_R {
        ADDR_OP_R::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bit 21 - Interrupted operation bank This bit indicates which Flash memory bank was accessed when reset occurred"]
    #[inline(always)]
    pub fn bk_op(&self) -> BK_OP_R {
        BK_OP_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Operation in system Flash memory interrupted This bit indicates that the reset occurred during an operation in the system Flash memory."]
    #[inline(always)]
    pub fn sysf_op(&self) -> SYSF_OP_R {
        SYSF_OP_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 29:31 - Flash memory operation code This field indicates which Flash memory operation has been interrupted by a system reset:"]
    #[inline(always)]
    pub fn code_op(&self) -> CODE_OP_R {
        CODE_OP_R::new(((self.bits >> 29) & 7) as u8)
    }
}
#[doc = "FLASH operation status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_opsr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLASH_OPSRrs;
impl crate::RegisterSpec for FLASH_OPSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_opsr::R`](R) reader structure"]
impl crate::Readable for FLASH_OPSRrs {}
#[doc = "`reset()` method sets FLASH_OPSR to value 0"]
impl crate::Resettable for FLASH_OPSRrs {
    const RESET_VALUE: u32 = 0;
}
