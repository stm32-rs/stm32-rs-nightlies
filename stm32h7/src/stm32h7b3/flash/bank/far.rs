///Register `FAR` reader
pub type R = crate::R<FARrs>;
///Field `FAIL_ECC_ADDR` reader - Bank 1 ECC error address When an ECC error occurs (both for single correction or double detection) during a read operation from bank 1, the FAIL_ECC_ADDR1 bitfield contains the address that generated the error. FAIL_ECC_ADDR1 is reset when the flag error in the FLASH_SR1 register (CLR_SNECCERR1 or CLR_DBECCERR1) is reset. The embedded Flash memory programs the address in this register only when no ECC error flags are set. This means that only the first address that generated an ECC error is saved. The address in FAIL_ECC_ADDR1 is relative to the Flash memory area where the error occurred (user Flash memory, system Flash memory, read-only/OTP area).
pub type FAIL_ECC_ADDR_R = crate::FieldReader<u16>;
///Field `OTP_FAIL_ECC` reader - OTP ECC error bit This bit is set to 1 when one single ECC correction or double ECC detection occurred during the last successful read operation from the read-only/ OTP area. The address of the ECC error is available in FAIL_ECC_ADDR1 bitfield.
pub type OTP_FAIL_ECC_R = crate::BitReader;
impl R {
    ///Bits 0:15 - Bank 1 ECC error address When an ECC error occurs (both for single correction or double detection) during a read operation from bank 1, the FAIL_ECC_ADDR1 bitfield contains the address that generated the error. FAIL_ECC_ADDR1 is reset when the flag error in the FLASH_SR1 register (CLR_SNECCERR1 or CLR_DBECCERR1) is reset. The embedded Flash memory programs the address in this register only when no ECC error flags are set. This means that only the first address that generated an ECC error is saved. The address in FAIL_ECC_ADDR1 is relative to the Flash memory area where the error occurred (user Flash memory, system Flash memory, read-only/OTP area).
    #[inline(always)]
    pub fn fail_ecc_addr(&self) -> FAIL_ECC_ADDR_R {
        FAIL_ECC_ADDR_R::new((self.bits & 0xffff) as u16)
    }
    ///Bit 31 - OTP ECC error bit This bit is set to 1 when one single ECC correction or double ECC detection occurred during the last successful read operation from the read-only/ OTP area. The address of the ECC error is available in FAIL_ECC_ADDR1 bitfield.
    #[inline(always)]
    pub fn otp_fail_ecc(&self) -> OTP_FAIL_ECC_R {
        OTP_FAIL_ECC_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FAR")
            .field("fail_ecc_addr", &self.fail_ecc_addr())
            .field("otp_fail_ecc", &self.otp_fail_ecc())
            .finish()
    }
}
/**

You can [`read`](crate::Reg::read) this register and get [`far::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FARrs;
impl crate::RegisterSpec for FARrs {
    type Ux = u32;
}
///`read()` method returns [`far::R`](R) reader structure
impl crate::Readable for FARrs {}
///`reset()` method sets FAR to value 0
impl crate::Resettable for FARrs {}
