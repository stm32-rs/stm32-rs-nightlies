#[doc = "Register `FLASH_ECCR` reader"]
pub type R = crate::R<FLASH_ECCRrs>;
#[doc = "Register `FLASH_ECCR` writer"]
pub type W = crate::W<FLASH_ECCRrs>;
#[doc = "Field `ADDR_ECC` reader - ECC fail address"]
pub type ADDR_ECC_R = crate::FieldReader<u32>;
#[doc = "Field `BK_ECC` reader - ECC fail bank"]
pub type BK_ECC_R = crate::BitReader;
#[doc = "Field `SYSF_ECC` reader - System Flash memory ECC fail This bit indicates that the ECC error correction or double ECC error detection is located in the system Flash memory."]
pub type SYSF_ECC_R = crate::BitReader;
#[doc = "Field `ECCIE` reader - ECC correction interrupt enable This bit enables the interrupt generation when the ECCC bit in the FLASH_ECCR register is set."]
pub type ECCIE_R = crate::BitReader;
#[doc = "Field `ECCIE` writer - ECC correction interrupt enable This bit enables the interrupt generation when the ECCC bit in the FLASH_ECCR register is set."]
pub type ECCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECCC` reader - ECC correction This bit is set by hardware when one ECC error has been detected and corrected (only if ECCC and ECCD were previously cleared). An interrupt is generated if ECCIE is set. This bit is cleared by writing 1."]
pub type ECCC_R = crate::BitReader;
#[doc = "Field `ECCC` writer - ECC correction This bit is set by hardware when one ECC error has been detected and corrected (only if ECCC and ECCD were previously cleared). An interrupt is generated if ECCIE is set. This bit is cleared by writing 1."]
pub type ECCC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECCD` reader - ECC detection This bit is set by hardware when two ECC errors have been detected (only if ECCC and ECCD were previously cleared). When this bit is set, a NMI is generated. This bit is cleared by writing 1."]
pub type ECCD_R = crate::BitReader;
#[doc = "Field `ECCD` writer - ECC detection This bit is set by hardware when two ECC errors have been detected (only if ECCC and ECCD were previously cleared). When this bit is set, a NMI is generated. This bit is cleared by writing 1."]
pub type ECCD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:19 - ECC fail address"]
    #[inline(always)]
    pub fn addr_ecc(&self) -> ADDR_ECC_R {
        ADDR_ECC_R::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bit 21 - ECC fail bank"]
    #[inline(always)]
    pub fn bk_ecc(&self) -> BK_ECC_R {
        BK_ECC_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - System Flash memory ECC fail This bit indicates that the ECC error correction or double ECC error detection is located in the system Flash memory."]
    #[inline(always)]
    pub fn sysf_ecc(&self) -> SYSF_ECC_R {
        SYSF_ECC_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - ECC correction interrupt enable This bit enables the interrupt generation when the ECCC bit in the FLASH_ECCR register is set."]
    #[inline(always)]
    pub fn eccie(&self) -> ECCIE_R {
        ECCIE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 30 - ECC correction This bit is set by hardware when one ECC error has been detected and corrected (only if ECCC and ECCD were previously cleared). An interrupt is generated if ECCIE is set. This bit is cleared by writing 1."]
    #[inline(always)]
    pub fn eccc(&self) -> ECCC_R {
        ECCC_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - ECC detection This bit is set by hardware when two ECC errors have been detected (only if ECCC and ECCD were previously cleared). When this bit is set, a NMI is generated. This bit is cleared by writing 1."]
    #[inline(always)]
    pub fn eccd(&self) -> ECCD_R {
        ECCD_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - ECC correction interrupt enable This bit enables the interrupt generation when the ECCC bit in the FLASH_ECCR register is set."]
    #[inline(always)]
    #[must_use]
    pub fn eccie(&mut self) -> ECCIE_W<FLASH_ECCRrs> {
        ECCIE_W::new(self, 24)
    }
    #[doc = "Bit 30 - ECC correction This bit is set by hardware when one ECC error has been detected and corrected (only if ECCC and ECCD were previously cleared). An interrupt is generated if ECCIE is set. This bit is cleared by writing 1."]
    #[inline(always)]
    #[must_use]
    pub fn eccc(&mut self) -> ECCC_W<FLASH_ECCRrs> {
        ECCC_W::new(self, 30)
    }
    #[doc = "Bit 31 - ECC detection This bit is set by hardware when two ECC errors have been detected (only if ECCC and ECCD were previously cleared). When this bit is set, a NMI is generated. This bit is cleared by writing 1."]
    #[inline(always)]
    #[must_use]
    pub fn eccd(&mut self) -> ECCD_W<FLASH_ECCRrs> {
        ECCD_W::new(self, 31)
    }
}
#[doc = "FLASH ECC register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_eccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_eccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLASH_ECCRrs;
impl crate::RegisterSpec for FLASH_ECCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_eccr::R`](R) reader structure"]
impl crate::Readable for FLASH_ECCRrs {}
#[doc = "`write(|w| ..)` method takes [`flash_eccr::W`](W) writer structure"]
impl crate::Writable for FLASH_ECCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASH_ECCR to value 0"]
impl crate::Resettable for FLASH_ECCRrs {
    const RESET_VALUE: u32 = 0;
}
