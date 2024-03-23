#[doc = "Register `SHIFTR` writer"]
pub type W = crate::W<SHIFTRrs>;
#[doc = "Field `SUBFS` writer - Subtract a fraction of a second"]
pub type SUBFS_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `ADD1S` writer - ADD1S"]
pub type ADD1S_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bits 0:14 - Subtract a fraction of a second"]
    #[inline(always)]
    #[must_use]
    pub fn subfs(&mut self) -> SUBFS_W<SHIFTRrs> {
        SUBFS_W::new(self, 0)
    }
    #[doc = "Bit 31 - ADD1S"]
    #[inline(always)]
    #[must_use]
    pub fn add1s(&mut self) -> ADD1S_W<SHIFTRrs> {
        ADD1S_W::new(self, 31)
    }
}
#[doc = "shift control register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shiftr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SHIFTRrs;
impl crate::RegisterSpec for SHIFTRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`shiftr::W`](W) writer structure"]
impl crate::Writable for SHIFTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SHIFTR to value 0"]
impl crate::Resettable for SHIFTRrs {
    const RESET_VALUE: u32 = 0;
}
