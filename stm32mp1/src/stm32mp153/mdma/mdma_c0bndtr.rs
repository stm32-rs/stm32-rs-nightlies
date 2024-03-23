#[doc = "Register `MDMA_C0BNDTR` reader"]
pub type R = crate::R<MDMA_C0BNDTRrs>;
#[doc = "Register `MDMA_C0BNDTR` writer"]
pub type W = crate::W<MDMA_C0BNDTRrs>;
#[doc = "Field `BNDT` reader - BNDT"]
pub type BNDT_R = crate::FieldReader<u32>;
#[doc = "Field `BNDT` writer - BNDT"]
pub type BNDT_W<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
#[doc = "Field `BRSUM` reader - BRSUM"]
pub type BRSUM_R = crate::BitReader;
#[doc = "Field `BRSUM` writer - BRSUM"]
pub type BRSUM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRDUM` reader - BRDUM"]
pub type BRDUM_R = crate::BitReader;
#[doc = "Field `BRDUM` writer - BRDUM"]
pub type BRDUM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRC` reader - BRC"]
pub type BRC_R = crate::FieldReader<u16>;
#[doc = "Field `BRC` writer - BRC"]
pub type BRC_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:16 - BNDT"]
    #[inline(always)]
    pub fn bndt(&self) -> BNDT_R {
        BNDT_R::new(self.bits & 0x0001_ffff)
    }
    #[doc = "Bit 18 - BRSUM"]
    #[inline(always)]
    pub fn brsum(&self) -> BRSUM_R {
        BRSUM_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - BRDUM"]
    #[inline(always)]
    pub fn brdum(&self) -> BRDUM_R {
        BRDUM_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:31 - BRC"]
    #[inline(always)]
    pub fn brc(&self) -> BRC_R {
        BRC_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:16 - BNDT"]
    #[inline(always)]
    #[must_use]
    pub fn bndt(&mut self) -> BNDT_W<MDMA_C0BNDTRrs> {
        BNDT_W::new(self, 0)
    }
    #[doc = "Bit 18 - BRSUM"]
    #[inline(always)]
    #[must_use]
    pub fn brsum(&mut self) -> BRSUM_W<MDMA_C0BNDTRrs> {
        BRSUM_W::new(self, 18)
    }
    #[doc = "Bit 19 - BRDUM"]
    #[inline(always)]
    #[must_use]
    pub fn brdum(&mut self) -> BRDUM_W<MDMA_C0BNDTRrs> {
        BRDUM_W::new(self, 19)
    }
    #[doc = "Bits 20:31 - BRC"]
    #[inline(always)]
    #[must_use]
    pub fn brc(&mut self) -> BRC_W<MDMA_C0BNDTRrs> {
        BRC_W::new(self, 20)
    }
}
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x04).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c0bndtr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c0bndtr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDMA_C0BNDTRrs;
impl crate::RegisterSpec for MDMA_C0BNDTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdma_c0bndtr::R`](R) reader structure"]
impl crate::Readable for MDMA_C0BNDTRrs {}
#[doc = "`write(|w| ..)` method takes [`mdma_c0bndtr::W`](W) writer structure"]
impl crate::Writable for MDMA_C0BNDTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MDMA_C0BNDTR to value 0"]
impl crate::Resettable for MDMA_C0BNDTRrs {
    const RESET_VALUE: u32 = 0;
}
