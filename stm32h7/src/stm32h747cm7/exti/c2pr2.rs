#[doc = "Register `C2PR2` reader"]
pub type R = crate::R<C2PR2rs>;
#[doc = "Register `C2PR2` writer"]
pub type W = crate::W<C2PR2rs>;
#[doc = "CPU2 configurable event inputs x+32 Pending bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PR49R {
    #[doc = "0: No trigger request occurred"]
    NotPending = 0,
    #[doc = "1: Selected trigger request occurred"]
    Pending = 1,
}
impl From<PR49R> for bool {
    #[inline(always)]
    fn from(variant: PR49R) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PR49` reader - CPU2 configurable event inputs x+32 Pending bit"]
pub type PR49_R = crate::BitReader<PR49R>;
impl PR49_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PR49R {
        match self.bits {
            false => PR49R::NotPending,
            true => PR49R::Pending,
        }
    }
    #[doc = "No trigger request occurred"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == PR49R::NotPending
    }
    #[doc = "Selected trigger request occurred"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == PR49R::Pending
    }
}
#[doc = "CPU2 configurable event inputs x+32 Pending bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PR49W {
    #[doc = "1: Clears pending bit"]
    Clear = 1,
}
impl From<PR49W> for bool {
    #[inline(always)]
    fn from(variant: PR49W) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PR49` writer - CPU2 configurable event inputs x+32 Pending bit"]
pub type PR49_W<'a, REG> = crate::BitWriter1C<'a, REG, PR49W>;
impl<'a, REG> PR49_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(PR49W::Clear)
    }
}
#[doc = "Field `PR51` reader - CPU2 configurable event inputs x+32 Pending bit"]
pub use PR49_R as PR51_R;
#[doc = "Field `PR51` writer - CPU2 configurable event inputs x+32 Pending bit"]
pub use PR49_W as PR51_W;
impl R {
    #[doc = "Bit 17 - CPU2 configurable event inputs x+32 Pending bit"]
    #[inline(always)]
    pub fn pr49(&self) -> PR49_R {
        PR49_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - CPU2 configurable event inputs x+32 Pending bit"]
    #[inline(always)]
    pub fn pr51(&self) -> PR51_R {
        PR51_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 17 - CPU2 configurable event inputs x+32 Pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn pr49(&mut self) -> PR49_W<C2PR2rs> {
        PR49_W::new(self, 17)
    }
    #[doc = "Bit 19 - CPU2 configurable event inputs x+32 Pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn pr51(&mut self) -> PR51_W<C2PR2rs> {
        PR51_W::new(self, 19)
    }
}
#[doc = "CPU2 EXTI pending register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2pr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2pr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C2PR2rs;
impl crate::RegisterSpec for C2PR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c2pr2::R`](R) reader structure"]
impl crate::Readable for C2PR2rs {}
#[doc = "`write(|w| ..)` method takes [`c2pr2::W`](W) writer structure"]
impl crate::Writable for C2PR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x000a_0000;
}
#[doc = "`reset()` method sets C2PR2 to value 0"]
impl crate::Resettable for C2PR2rs {
    const RESET_VALUE: u32 = 0;
}
