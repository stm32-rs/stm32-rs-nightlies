#[doc = "Register `ECCDETR` reader"]
pub type R = crate::R<ECCDETRrs>;
#[doc = "Register `ECCDETR` writer"]
pub type W = crate::W<ECCDETRrs>;
#[doc = "Field `ADDR_ECC` reader - ECC error address When an ECC error occurs (double detection) during a read operation, the ADDR_ECC contains the address that generated the error. ADDR_ECC is reset when the flag error is reset. The embedded Flash memory programs the address in this register only when no ECC error flags are set. This means that only the first address that generated an double ECC error is saved. The address in ADDR_ECC is relative to the Flash memory area where the error occurred (user Flash memory, system Flash memory, data area, read-only/OTP area)."]
pub type ADDR_ECC_R = crate::FieldReader<u16>;
#[doc = "Field `BK_ECC` reader - ECC fail bank for double ECC Error It indicates which bank is concerned by ECC error"]
pub type BK_ECC_R = crate::BitReader;
#[doc = "Field `SYSF_ECC` reader - ECC fail for double ECC error in system Flash memory It indicates if system Flash memory is concerned by ECC error."]
pub type SYSF_ECC_R = crate::BitReader;
#[doc = "Field `OTP_ECC` reader - OTP ECC error bit This bit is set to 1 when double ECC detection occurred during the last read operation from the read-only/ OTP area. The address of the ECC error is available in ADDR_ECC bit field."]
pub type OTP_ECC_R = crate::BitReader;
#[doc = "Field `ECCD` reader - ECC detection set by hardware when two ECC error has been detected. When this bit is set, a NMI is generated. Cleared by writing 1. Needs to be cleared in order to detect subsequent double ECC errors."]
pub type ECCD_R = crate::BitReader;
#[doc = "Field `ECCD` writer - ECC detection set by hardware when two ECC error has been detected. When this bit is set, a NMI is generated. Cleared by writing 1. Needs to be cleared in order to detect subsequent double ECC errors."]
pub type ECCD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - ECC error address When an ECC error occurs (double detection) during a read operation, the ADDR_ECC contains the address that generated the error. ADDR_ECC is reset when the flag error is reset. The embedded Flash memory programs the address in this register only when no ECC error flags are set. This means that only the first address that generated an double ECC error is saved. The address in ADDR_ECC is relative to the Flash memory area where the error occurred (user Flash memory, system Flash memory, data area, read-only/OTP area)."]
    #[inline(always)]
    pub fn addr_ecc(&self) -> ADDR_ECC_R {
        ADDR_ECC_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 22 - ECC fail bank for double ECC Error It indicates which bank is concerned by ECC error"]
    #[inline(always)]
    pub fn bk_ecc(&self) -> BK_ECC_R {
        BK_ECC_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - ECC fail for double ECC error in system Flash memory It indicates if system Flash memory is concerned by ECC error."]
    #[inline(always)]
    pub fn sysf_ecc(&self) -> SYSF_ECC_R {
        SYSF_ECC_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - OTP ECC error bit This bit is set to 1 when double ECC detection occurred during the last read operation from the read-only/ OTP area. The address of the ECC error is available in ADDR_ECC bit field."]
    #[inline(always)]
    pub fn otp_ecc(&self) -> OTP_ECC_R {
        OTP_ECC_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 31 - ECC detection set by hardware when two ECC error has been detected. When this bit is set, a NMI is generated. Cleared by writing 1. Needs to be cleared in order to detect subsequent double ECC errors."]
    #[inline(always)]
    pub fn eccd(&self) -> ECCD_R {
        ECCD_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - ECC detection set by hardware when two ECC error has been detected. When this bit is set, a NMI is generated. Cleared by writing 1. Needs to be cleared in order to detect subsequent double ECC errors."]
    #[inline(always)]
    #[must_use]
    pub fn eccd(&mut self) -> ECCD_W<ECCDETRrs> {
        ECCD_W::new(self, 31)
    }
}
#[doc = "FLASH ECC detection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eccdetr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eccdetr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ECCDETRrs;
impl crate::RegisterSpec for ECCDETRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eccdetr::R`](R) reader structure"]
impl crate::Readable for ECCDETRrs {}
#[doc = "`write(|w| ..)` method takes [`eccdetr::W`](W) writer structure"]
impl crate::Writable for ECCDETRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ECCDETR to value 0"]
impl crate::Resettable for ECCDETRrs {
    const RESET_VALUE: u32 = 0;
}
