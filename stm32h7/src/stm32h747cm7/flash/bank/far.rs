#[doc = "Register `FAR` reader"]
pub type R = crate::R<FARrs>;
#[doc = "Register `FAR` writer"]
pub type W = crate::W<FARrs>;
#[doc = "Field `FAIL_ECC_ADDR` reader - Bank 1 ECC error address"]
pub type FAIL_ECC_ADDR_R = crate::FieldReader<u16>;
#[doc = "Field `FAIL_ECC_ADDR` writer - Bank 1 ECC error address"]
pub type FAIL_ECC_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bits 0:14 - Bank 1 ECC error address"]
    #[inline(always)]
    pub fn fail_ecc_addr(&self) -> FAIL_ECC_ADDR_R {
        FAIL_ECC_ADDR_R::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - Bank 1 ECC error address"]
    #[inline(always)]
    #[must_use]
    pub fn fail_ecc_addr(&mut self) -> FAIL_ECC_ADDR_W<FARrs> {
        FAIL_ECC_ADDR_W::new(self, 0)
    }
}
#[doc = "FLASH ECC fail address for bank 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`far::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`far::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FARrs;
impl crate::RegisterSpec for FARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`far::R`](R) reader structure"]
impl crate::Readable for FARrs {}
#[doc = "`write(|w| ..)` method takes [`far::W`](W) writer structure"]
impl crate::Writable for FARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FAR to value 0"]
impl crate::Resettable for FARrs {
    const RESET_VALUE: u32 = 0;
}
