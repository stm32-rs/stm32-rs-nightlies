#[doc = "Register `SEC1R` reader"]
pub type R = crate::R<SEC1Rrs>;
#[doc = "Register `SEC1R` writer"]
pub type W = crate::W<SEC1Rrs>;
#[doc = "Field `SEC_SIZE1` reader - SEC_SIZE1"]
pub type SEC_SIZE1_R = crate::FieldReader;
#[doc = "Field `SEC_SIZE1` writer - SEC_SIZE1"]
pub type SEC_SIZE1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BOOT_LOCK` reader - BOOT_LOCK"]
pub type BOOT_LOCK_R = crate::BitReader;
#[doc = "Field `BOOT_LOCK` writer - BOOT_LOCK"]
pub type BOOT_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - SEC_SIZE1"]
    #[inline(always)]
    pub fn sec_size1(&self) -> SEC_SIZE1_R {
        SEC_SIZE1_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 16 - BOOT_LOCK"]
    #[inline(always)]
    pub fn boot_lock(&self) -> BOOT_LOCK_R {
        BOOT_LOCK_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - SEC_SIZE1"]
    #[inline(always)]
    #[must_use]
    pub fn sec_size1(&mut self) -> SEC_SIZE1_W<SEC1Rrs> {
        SEC_SIZE1_W::new(self, 0)
    }
    #[doc = "Bit 16 - BOOT_LOCK"]
    #[inline(always)]
    #[must_use]
    pub fn boot_lock(&mut self) -> BOOT_LOCK_W<SEC1Rrs> {
        BOOT_LOCK_W::new(self, 16)
    }
}
#[doc = "securable area bank1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sec1r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sec1r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SEC1Rrs;
impl crate::RegisterSpec for SEC1Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec1r::R`](R) reader structure"]
impl crate::Readable for SEC1Rrs {}
#[doc = "`write(|w| ..)` method takes [`sec1r::W`](W) writer structure"]
impl crate::Writable for SEC1Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SEC1R to value 0xff00_ff00"]
impl crate::Resettable for SEC1Rrs {
    const RESET_VALUE: u32 = 0xff00_ff00;
}
