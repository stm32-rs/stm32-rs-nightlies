///Register `OPSR` reader
pub type R = crate::R<OPSRrs>;
///Field `ADDR_OP` reader - Interrupted operation address This field indicates which address in the Flash memory was accessed when reset occurred. The address is given relative to the Flash base address, from offset 0x00000 to 0xFFFF0. Note that bit 19 is reserved on STM32WBAxEx devices.
pub type ADDR_OP_R = crate::FieldReader<u32>;
///Field `SYSF_OP` reader - Operation in system Flash memory interrupted This bit indicates that the reset occurred during an operation in the system Flash memory.
pub type SYSF_OP_R = crate::BitReader;
///Field `CODE_OP` reader - Flash memory operation code This field indicates which Flash memory operation has been interrupted by a system reset:
pub type CODE_OP_R = crate::FieldReader;
impl R {
    ///Bits 0:19 - Interrupted operation address This field indicates which address in the Flash memory was accessed when reset occurred. The address is given relative to the Flash base address, from offset 0x00000 to 0xFFFF0. Note that bit 19 is reserved on STM32WBAxEx devices.
    #[inline(always)]
    pub fn addr_op(&self) -> ADDR_OP_R {
        ADDR_OP_R::new(self.bits & 0x000f_ffff)
    }
    ///Bit 22 - Operation in system Flash memory interrupted This bit indicates that the reset occurred during an operation in the system Flash memory.
    #[inline(always)]
    pub fn sysf_op(&self) -> SYSF_OP_R {
        SYSF_OP_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bits 29:31 - Flash memory operation code This field indicates which Flash memory operation has been interrupted by a system reset:
    #[inline(always)]
    pub fn code_op(&self) -> CODE_OP_R {
        CODE_OP_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OPSR")
            .field("addr_op", &self.addr_op())
            .field("sysf_op", &self.sysf_op())
            .field("code_op", &self.code_op())
            .finish()
    }
}
/**FLASH operation status register

You can [`read`](crate::Reg::read) this register and get [`opsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#FLASH:OPSR)*/
pub struct OPSRrs;
impl crate::RegisterSpec for OPSRrs {
    type Ux = u32;
}
///`read()` method returns [`opsr::R`](R) reader structure
impl crate::Readable for OPSRrs {}
///`reset()` method sets OPSR to value 0
impl crate::Resettable for OPSRrs {}
