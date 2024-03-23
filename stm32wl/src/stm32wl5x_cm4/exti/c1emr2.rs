#[doc = "Register `C1EMR2` reader"]
pub type R = crate::R<C1EMR2rs>;
#[doc = "Register `C1EMR2` writer"]
pub type W = crate::W<C1EMR2rs>;
#[doc = "Wakeup with event generation Mask on Event input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EM40 {
    #[doc = "0: Interrupt request line is masked"]
    Masked = 0,
    #[doc = "1: Interrupt request line is unmasked"]
    Unmasked = 1,
}
impl From<EM40> for bool {
    #[inline(always)]
    fn from(variant: EM40) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM40` reader - Wakeup with event generation Mask on Event input"]
pub type EM40_R = crate::BitReader<EM40>;
impl EM40_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EM40 {
        match self.bits {
            false => EM40::Masked,
            true => EM40::Unmasked,
        }
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == EM40::Masked
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == EM40::Unmasked
    }
}
#[doc = "Field `EM40` writer - Wakeup with event generation Mask on Event input"]
pub type EM40_W<'a, REG> = crate::BitWriter<'a, REG, EM40>;
impl<'a, REG> EM40_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(EM40::Masked)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(EM40::Unmasked)
    }
}
#[doc = "Field `EM41` reader - Wakeup with event generation Mask on Event input"]
pub use EM40_R as EM41_R;
#[doc = "Field `EM41` writer - Wakeup with event generation Mask on Event input"]
pub use EM40_W as EM41_W;
impl R {
    #[doc = "Bit 8 - Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    pub fn em40(&self) -> EM40_R {
        EM40_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    pub fn em41(&self) -> EM41_R {
        EM41_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    #[must_use]
    pub fn em40(&mut self) -> EM40_W<C1EMR2rs> {
        EM40_W::new(self, 8)
    }
    #[doc = "Bit 9 - Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    #[must_use]
    pub fn em41(&mut self) -> EM41_W<C1EMR2rs> {
        EM41_W::new(self, 9)
    }
}
#[doc = "wakeup with event mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1emr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1emr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C1EMR2rs;
impl crate::RegisterSpec for C1EMR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c1emr2::R`](R) reader structure"]
impl crate::Readable for C1EMR2rs {}
#[doc = "`write(|w| ..)` method takes [`c1emr2::W`](W) writer structure"]
impl crate::Writable for C1EMR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C1EMR2 to value 0"]
impl crate::Resettable for C1EMR2rs {
    const RESET_VALUE: u32 = 0;
}
