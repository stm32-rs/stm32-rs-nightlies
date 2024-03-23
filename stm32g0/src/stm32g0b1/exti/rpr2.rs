#[doc = "Register `RPR2` reader"]
pub type R = crate::R<RPR2rs>;
#[doc = "Register `RPR2` writer"]
pub type W = crate::W<RPR2rs>;
#[doc = "Rising edge event pending for configurable line 34\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPIF2R {
    #[doc = "0: No trigger request occurred"]
    NotPending = 0,
    #[doc = "1: Selected trigger request occurred"]
    Pending = 1,
}
impl From<RPIF2R> for bool {
    #[inline(always)]
    fn from(variant: RPIF2R) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RPIF2` reader - Rising edge event pending for configurable line 34"]
pub type RPIF2_R = crate::BitReader<RPIF2R>;
impl RPIF2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RPIF2R {
        match self.bits {
            false => RPIF2R::NotPending,
            true => RPIF2R::Pending,
        }
    }
    #[doc = "No trigger request occurred"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == RPIF2R::NotPending
    }
    #[doc = "Selected trigger request occurred"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == RPIF2R::Pending
    }
}
#[doc = "Rising edge event pending for configurable line 34\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPIF2W {
    #[doc = "1: Clears pending bit"]
    Clear = 1,
}
impl From<RPIF2W> for bool {
    #[inline(always)]
    fn from(variant: RPIF2W) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RPIF2` writer - Rising edge event pending for configurable line 34"]
pub type RPIF2_W<'a, REG> = crate::BitWriter1C<'a, REG, RPIF2W>;
impl<'a, REG> RPIF2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(RPIF2W::Clear)
    }
}
impl R {
    #[doc = "Bit 2 - Rising edge event pending for configurable line 34"]
    #[inline(always)]
    pub fn rpif2(&self) -> RPIF2_R {
        RPIF2_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Rising edge event pending for configurable line 34"]
    #[inline(always)]
    #[must_use]
    pub fn rpif2(&mut self) -> RPIF2_W<RPR2rs> {
        RPIF2_W::new(self, 2)
    }
}
#[doc = "EXTI rising edge pending register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rpr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rpr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RPR2rs;
impl crate::RegisterSpec for RPR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rpr2::R`](R) reader structure"]
impl crate::Readable for RPR2rs {}
#[doc = "`write(|w| ..)` method takes [`rpr2::W`](W) writer structure"]
impl crate::Writable for RPR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x04;
}
#[doc = "`reset()` method sets RPR2 to value 0"]
impl crate::Resettable for RPR2rs {
    const RESET_VALUE: u32 = 0;
}
