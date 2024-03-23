#[doc = "Register `C2PR3` reader"]
pub type R = crate::R<C2PR3rs>;
#[doc = "Register `C2PR3` writer"]
pub type W = crate::W<C2PR3rs>;
#[doc = "CPU2 configurable event inputs x+64 Pending bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PR82R {
    #[doc = "0: No trigger request occurred"]
    NotPending = 0,
    #[doc = "1: Selected trigger request occurred"]
    Pending = 1,
}
impl From<PR82R> for bool {
    #[inline(always)]
    fn from(variant: PR82R) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PR82` reader - CPU2 configurable event inputs x+64 Pending bit"]
pub type PR82_R = crate::BitReader<PR82R>;
impl PR82_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PR82R {
        match self.bits {
            false => PR82R::NotPending,
            true => PR82R::Pending,
        }
    }
    #[doc = "No trigger request occurred"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == PR82R::NotPending
    }
    #[doc = "Selected trigger request occurred"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == PR82R::Pending
    }
}
#[doc = "CPU2 configurable event inputs x+64 Pending bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PR82W {
    #[doc = "1: Clears pending bit"]
    Clear = 1,
}
impl From<PR82W> for bool {
    #[inline(always)]
    fn from(variant: PR82W) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PR82` writer - CPU2 configurable event inputs x+64 Pending bit"]
pub type PR82_W<'a, REG> = crate::BitWriter1C<'a, REG, PR82W>;
impl<'a, REG> PR82_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(PR82W::Clear)
    }
}
#[doc = "Field `PR84` reader - CPU2 configurable event inputs x+64 Pending bit"]
pub use PR82_R as PR84_R;
#[doc = "Field `PR85` reader - CPU2 configurable event inputs x+64 Pending bit"]
pub use PR82_R as PR85_R;
#[doc = "Field `PR86` reader - CPU2 configurable event inputs x+64 Pending bit"]
pub use PR82_R as PR86_R;
#[doc = "Field `PR84` writer - CPU2 configurable event inputs x+64 Pending bit"]
pub use PR82_W as PR84_W;
#[doc = "Field `PR85` writer - CPU2 configurable event inputs x+64 Pending bit"]
pub use PR82_W as PR85_W;
#[doc = "Field `PR86` writer - CPU2 configurable event inputs x+64 Pending bit"]
pub use PR82_W as PR86_W;
impl R {
    #[doc = "Bit 18 - CPU2 configurable event inputs x+64 Pending bit"]
    #[inline(always)]
    pub fn pr82(&self) -> PR82_R {
        PR82_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - CPU2 configurable event inputs x+64 Pending bit"]
    #[inline(always)]
    pub fn pr84(&self) -> PR84_R {
        PR84_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - CPU2 configurable event inputs x+64 Pending bit"]
    #[inline(always)]
    pub fn pr85(&self) -> PR85_R {
        PR85_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - CPU2 configurable event inputs x+64 Pending bit"]
    #[inline(always)]
    pub fn pr86(&self) -> PR86_R {
        PR86_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 18 - CPU2 configurable event inputs x+64 Pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn pr82(&mut self) -> PR82_W<C2PR3rs> {
        PR82_W::new(self, 18)
    }
    #[doc = "Bit 20 - CPU2 configurable event inputs x+64 Pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn pr84(&mut self) -> PR84_W<C2PR3rs> {
        PR84_W::new(self, 20)
    }
    #[doc = "Bit 21 - CPU2 configurable event inputs x+64 Pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn pr85(&mut self) -> PR85_W<C2PR3rs> {
        PR85_W::new(self, 21)
    }
    #[doc = "Bit 22 - CPU2 configurable event inputs x+64 Pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn pr86(&mut self) -> PR86_W<C2PR3rs> {
        PR86_W::new(self, 22)
    }
}
#[doc = "CPU2 EXTI pending register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2pr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2pr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C2PR3rs;
impl crate::RegisterSpec for C2PR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c2pr3::R`](R) reader structure"]
impl crate::Readable for C2PR3rs {}
#[doc = "`write(|w| ..)` method takes [`c2pr3::W`](W) writer structure"]
impl crate::Writable for C2PR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0074_0000;
}
#[doc = "`reset()` method sets C2PR3 to value 0"]
impl crate::Resettable for C2PR3rs {
    const RESET_VALUE: u32 = 0;
}
