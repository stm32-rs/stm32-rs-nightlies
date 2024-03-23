#[doc = "Register `TZC_SPECULATION_CTRL` reader"]
pub type R = crate::R<TZC_SPECULATION_CTRLrs>;
#[doc = "Register `TZC_SPECULATION_CTRL` writer"]
pub type W = crate::W<TZC_SPECULATION_CTRLrs>;
#[doc = "Field `READSPEC_DISABLE` reader - READSPEC_DISABLE"]
pub type READSPEC_DISABLE_R = crate::BitReader;
#[doc = "Field `READSPEC_DISABLE` writer - READSPEC_DISABLE"]
pub type READSPEC_DISABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITESPEC_DISABLE` reader - WRITESPEC_DISABLE"]
pub type WRITESPEC_DISABLE_R = crate::BitReader;
#[doc = "Field `WRITESPEC_DISABLE` writer - WRITESPEC_DISABLE"]
pub type WRITESPEC_DISABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - READSPEC_DISABLE"]
    #[inline(always)]
    pub fn readspec_disable(&self) -> READSPEC_DISABLE_R {
        READSPEC_DISABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - WRITESPEC_DISABLE"]
    #[inline(always)]
    pub fn writespec_disable(&self) -> WRITESPEC_DISABLE_R {
        WRITESPEC_DISABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - READSPEC_DISABLE"]
    #[inline(always)]
    #[must_use]
    pub fn readspec_disable(&mut self) -> READSPEC_DISABLE_W<TZC_SPECULATION_CTRLrs> {
        READSPEC_DISABLE_W::new(self, 0)
    }
    #[doc = "Bit 1 - WRITESPEC_DISABLE"]
    #[inline(always)]
    #[must_use]
    pub fn writespec_disable(&mut self) -> WRITESPEC_DISABLE_W<TZC_SPECULATION_CTRLrs> {
        WRITESPEC_DISABLE_W::new(self, 1)
    }
}
#[doc = "Controls read and write access speculation.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_speculation_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_speculation_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TZC_SPECULATION_CTRLrs;
impl crate::RegisterSpec for TZC_SPECULATION_CTRLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_speculation_ctrl::R`](R) reader structure"]
impl crate::Readable for TZC_SPECULATION_CTRLrs {}
#[doc = "`write(|w| ..)` method takes [`tzc_speculation_ctrl::W`](W) writer structure"]
impl crate::Writable for TZC_SPECULATION_CTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TZC_SPECULATION_CTRL to value 0"]
impl crate::Resettable for TZC_SPECULATION_CTRLrs {
    const RESET_VALUE: u32 = 0;
}
