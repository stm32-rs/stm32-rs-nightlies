#[doc = "Register `HSEM_R5` reader"]
pub type R = crate::R<HSEM_R5rs>;
#[doc = "Register `HSEM_R5` writer"]
pub type W = crate::W<HSEM_R5rs>;
#[doc = "Field `PROCID` reader - PROCID"]
pub type PROCID_R = crate::FieldReader;
#[doc = "Field `PROCID` writer - PROCID"]
pub type PROCID_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `COREID` reader - COREID"]
pub type COREID_R = crate::FieldReader;
#[doc = "Field `COREID` writer - COREID"]
pub type COREID_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LOCK` reader - LOCK"]
pub type LOCK_R = crate::BitReader;
#[doc = "Field `LOCK` writer - LOCK"]
pub type LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PROCID"]
    #[inline(always)]
    pub fn procid(&self) -> PROCID_R {
        PROCID_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - COREID"]
    #[inline(always)]
    pub fn coreid(&self) -> COREID_R {
        COREID_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - LOCK"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PROCID"]
    #[inline(always)]
    #[must_use]
    pub fn procid(&mut self) -> PROCID_W<HSEM_R5rs> {
        PROCID_W::new(self, 0)
    }
    #[doc = "Bits 8:11 - COREID"]
    #[inline(always)]
    #[must_use]
    pub fn coreid(&mut self) -> COREID_W<HSEM_R5rs> {
        COREID_W::new(self, 8)
    }
    #[doc = "Bit 31 - LOCK"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<HSEM_R5rs> {
        LOCK_W::new(self, 31)
    }
}
#[doc = "The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_r5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hsem_r5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HSEM_R5rs;
impl crate::RegisterSpec for HSEM_R5rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hsem_r5::R`](R) reader structure"]
impl crate::Readable for HSEM_R5rs {}
#[doc = "`write(|w| ..)` method takes [`hsem_r5::W`](W) writer structure"]
impl crate::Writable for HSEM_R5rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HSEM_R5 to value 0"]
impl crate::Resettable for HSEM_R5rs {
    const RESET_VALUE: u32 = 0;
}
