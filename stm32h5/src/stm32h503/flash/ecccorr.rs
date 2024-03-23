#[doc = "Register `ECCCORR` reader"]
pub type R = crate::R<ECCCORRrs>;
#[doc = "Register `ECCCORR` writer"]
pub type W = crate::W<ECCCORRrs>;
#[doc = "Field `ADDR_ECC` reader - ECC error address When an ECC error occurs (for single correction) during a read operation, the ADDR_ECC contains the address that generated the error. ADDR_ECC is reset when the flag error is reset. The embedded Flash memory programs the address in this register only when no ECC error flags are set. This means that only the first address that generated an ECC error is saved. The address in ADDR_ECC is relative to the Flash memory area where the error occurred (user Flash memory, system Flash memory, data area, read-only/OTP area)."]
pub type ADDR_ECC_R = crate::FieldReader<u16>;
#[doc = "Field `BK_ECC` reader - ECC bank flag for corrected ECC error It indicates which bank is concerned by ECC error"]
pub type BK_ECC_R = crate::BitReader;
#[doc = "Field `SYSF_ECC` reader - ECC flag for corrected ECC error in system FLASH It indicates if system Flash memory is concerned by ECC error."]
pub type SYSF_ECC_R = crate::BitReader;
#[doc = "Field `OTP_ECC` reader - OTP ECC error bit This bit is set to 1 when one single ECC correction occurred during the last successful read operation from the read-only/ OTP area. The address of the ECC error is available in ADDR_ECC bitfield."]
pub type OTP_ECC_R = crate::BitReader;
#[doc = "Field `ECCCIE` reader - ECC single correction error interrupt enable bit When ECCCIE bit is set to 1, an interrupt is generated when an ECC single correction error occurs during a read operation."]
pub type ECCCIE_R = crate::BitReader;
#[doc = "Field `ECCCIE` writer - ECC single correction error interrupt enable bit When ECCCIE bit is set to 1, an interrupt is generated when an ECC single correction error occurs during a read operation."]
pub type ECCCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECCC` reader - ECC correction set by hardware when single ECC error has been detected and corrected. Cleared by writing 1."]
pub type ECCC_R = crate::BitReader;
#[doc = "Field `ECCC` writer - ECC correction set by hardware when single ECC error has been detected and corrected. Cleared by writing 1."]
pub type ECCC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - ECC error address When an ECC error occurs (for single correction) during a read operation, the ADDR_ECC contains the address that generated the error. ADDR_ECC is reset when the flag error is reset. The embedded Flash memory programs the address in this register only when no ECC error flags are set. This means that only the first address that generated an ECC error is saved. The address in ADDR_ECC is relative to the Flash memory area where the error occurred (user Flash memory, system Flash memory, data area, read-only/OTP area)."]
    #[inline(always)]
    pub fn addr_ecc(&self) -> ADDR_ECC_R {
        ADDR_ECC_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 22 - ECC bank flag for corrected ECC error It indicates which bank is concerned by ECC error"]
    #[inline(always)]
    pub fn bk_ecc(&self) -> BK_ECC_R {
        BK_ECC_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - ECC flag for corrected ECC error in system FLASH It indicates if system Flash memory is concerned by ECC error."]
    #[inline(always)]
    pub fn sysf_ecc(&self) -> SYSF_ECC_R {
        SYSF_ECC_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - OTP ECC error bit This bit is set to 1 when one single ECC correction occurred during the last successful read operation from the read-only/ OTP area. The address of the ECC error is available in ADDR_ECC bitfield."]
    #[inline(always)]
    pub fn otp_ecc(&self) -> OTP_ECC_R {
        OTP_ECC_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - ECC single correction error interrupt enable bit When ECCCIE bit is set to 1, an interrupt is generated when an ECC single correction error occurs during a read operation."]
    #[inline(always)]
    pub fn ecccie(&self) -> ECCCIE_R {
        ECCCIE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 30 - ECC correction set by hardware when single ECC error has been detected and corrected. Cleared by writing 1."]
    #[inline(always)]
    pub fn eccc(&self) -> ECCC_R {
        ECCC_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 25 - ECC single correction error interrupt enable bit When ECCCIE bit is set to 1, an interrupt is generated when an ECC single correction error occurs during a read operation."]
    #[inline(always)]
    #[must_use]
    pub fn ecccie(&mut self) -> ECCCIE_W<ECCCORRrs> {
        ECCCIE_W::new(self, 25)
    }
    #[doc = "Bit 30 - ECC correction set by hardware when single ECC error has been detected and corrected. Cleared by writing 1."]
    #[inline(always)]
    #[must_use]
    pub fn eccc(&mut self) -> ECCC_W<ECCCORRrs> {
        ECCC_W::new(self, 30)
    }
}
#[doc = "FLASH Flash ECC correction register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecccorr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecccorr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ECCCORRrs;
impl crate::RegisterSpec for ECCCORRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecccorr::R`](R) reader structure"]
impl crate::Readable for ECCCORRrs {}
#[doc = "`write(|w| ..)` method takes [`ecccorr::W`](W) writer structure"]
impl crate::Writable for ECCCORRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ECCCORR to value 0"]
impl crate::Resettable for ECCCORRrs {
    const RESET_VALUE: u32 = 0;
}
