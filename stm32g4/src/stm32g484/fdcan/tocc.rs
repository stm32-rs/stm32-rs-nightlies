#[doc = "Register `TOCC` reader"]
pub type R = crate::R<TOCCrs>;
#[doc = "Register `TOCC` writer"]
pub type W = crate::W<TOCCrs>;
#[doc = "Field `ETOC` reader - ETOC"]
pub type ETOC_R = crate::BitReader;
#[doc = "Field `ETOC` writer - ETOC"]
pub type ETOC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOS` writer - TOS"]
pub type TOS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TOP` reader - TOP"]
pub type TOP_R = crate::FieldReader<u16>;
#[doc = "Field `TOP` writer - TOP"]
pub type TOP_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - ETOC"]
    #[inline(always)]
    pub fn etoc(&self) -> ETOC_R {
        ETOC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:31 - TOP"]
    #[inline(always)]
    pub fn top(&self) -> TOP_R {
        TOP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - ETOC"]
    #[inline(always)]
    #[must_use]
    pub fn etoc(&mut self) -> ETOC_W<TOCCrs> {
        ETOC_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - TOS"]
    #[inline(always)]
    #[must_use]
    pub fn tos(&mut self) -> TOS_W<TOCCrs> {
        TOS_W::new(self, 1)
    }
    #[doc = "Bits 16:31 - TOP"]
    #[inline(always)]
    #[must_use]
    pub fn top(&mut self) -> TOP_W<TOCCrs> {
        TOP_W::new(self, 16)
    }
}
#[doc = "FDCAN Timeout Counter Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tocc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tocc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TOCCrs;
impl crate::RegisterSpec for TOCCrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tocc::R`](R) reader structure"]
impl crate::Readable for TOCCrs {}
#[doc = "`write(|w| ..)` method takes [`tocc::W`](W) writer structure"]
impl crate::Writable for TOCCrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TOCC to value 0xffff_0000"]
impl crate::Resettable for TOCCrs {
    const RESET_VALUE: u32 = 0xffff_0000;
}
