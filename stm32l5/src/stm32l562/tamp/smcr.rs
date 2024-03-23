#[doc = "Register `SMCR` reader"]
pub type R = crate::R<SMCRrs>;
#[doc = "Register `SMCR` writer"]
pub type W = crate::W<SMCRrs>;
#[doc = "Field `BKPRWDPROT` reader - Backup registers read/write protection offset"]
pub type BKPRWDPROT_R = crate::FieldReader;
#[doc = "Field `BKPRWDPROT` writer - Backup registers read/write protection offset"]
pub type BKPRWDPROT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BKPWDPROT` reader - Backup registers write protection offset"]
pub type BKPWDPROT_R = crate::FieldReader;
#[doc = "Field `BKPWDPROT` writer - Backup registers write protection offset"]
pub type BKPWDPROT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TAMPDPROT` reader - Tamper protection"]
pub type TAMPDPROT_R = crate::BitReader;
#[doc = "Field `TAMPDPROT` writer - Tamper protection"]
pub type TAMPDPROT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Backup registers read/write protection offset"]
    #[inline(always)]
    pub fn bkprwdprot(&self) -> BKPRWDPROT_R {
        BKPRWDPROT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Backup registers write protection offset"]
    #[inline(always)]
    pub fn bkpwdprot(&self) -> BKPWDPROT_R {
        BKPWDPROT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 31 - Tamper protection"]
    #[inline(always)]
    pub fn tampdprot(&self) -> TAMPDPROT_R {
        TAMPDPROT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Backup registers read/write protection offset"]
    #[inline(always)]
    #[must_use]
    pub fn bkprwdprot(&mut self) -> BKPRWDPROT_W<SMCRrs> {
        BKPRWDPROT_W::new(self, 0)
    }
    #[doc = "Bits 16:23 - Backup registers write protection offset"]
    #[inline(always)]
    #[must_use]
    pub fn bkpwdprot(&mut self) -> BKPWDPROT_W<SMCRrs> {
        BKPWDPROT_W::new(self, 16)
    }
    #[doc = "Bit 31 - Tamper protection"]
    #[inline(always)]
    #[must_use]
    pub fn tampdprot(&mut self) -> TAMPDPROT_W<SMCRrs> {
        TAMPDPROT_W::new(self, 31)
    }
}
#[doc = "TAMP secure mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SMCRrs;
impl crate::RegisterSpec for SMCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smcr::R`](R) reader structure"]
impl crate::Readable for SMCRrs {}
#[doc = "`write(|w| ..)` method takes [`smcr::W`](W) writer structure"]
impl crate::Writable for SMCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SMCR to value 0"]
impl crate::Resettable for SMCRrs {
    const RESET_VALUE: u32 = 0;
}
