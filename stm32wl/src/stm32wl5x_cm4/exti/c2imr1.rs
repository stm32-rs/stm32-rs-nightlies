#[doc = "Register `C2IMR1` reader"]
pub type R = crate::R<C2IMR1rs>;
#[doc = "Register `C2IMR1` writer"]
pub type W = crate::W<C2IMR1rs>;
#[doc = "wakeup with interrupt Mask on Event input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum IM {
    #[doc = "0: Interrupt request line is masked"]
    Masked = 0,
    #[doc = "1: Interrupt request line is unmasked"]
    Unmasked = 1,
}
impl From<IM> for u32 {
    #[inline(always)]
    fn from(variant: IM) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IM {
    type Ux = u32;
}
#[doc = "Field `IM` reader - wakeup with interrupt Mask on Event input"]
pub type IM_R = crate::FieldReader<IM>;
impl IM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<IM> {
        match self.bits {
            0 => Some(IM::Masked),
            1 => Some(IM::Unmasked),
            _ => None,
        }
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == IM::Masked
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == IM::Unmasked
    }
}
#[doc = "Field `IM` writer - wakeup with interrupt Mask on Event input"]
pub type IM_W<'a, REG> = crate::FieldWriter<'a, REG, 32, IM>;
impl<'a, REG> IM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(IM::Masked)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(IM::Unmasked)
    }
}
impl R {
    #[doc = "Bits 0:31 - wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    pub fn im(&self) -> IM_R {
        IM_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    #[must_use]
    pub fn im(&mut self) -> IM_W<C2IMR1rs> {
        IM_W::new(self, 0)
    }
}
#[doc = "interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2imr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2imr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C2IMR1rs;
impl crate::RegisterSpec for C2IMR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c2imr1::R`](R) reader structure"]
impl crate::Readable for C2IMR1rs {}
#[doc = "`write(|w| ..)` method takes [`c2imr1::W`](W) writer structure"]
impl crate::Writable for C2IMR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C2IMR1 to value 0"]
impl crate::Resettable for C2IMR1rs {
    const RESET_VALUE: u32 = 0;
}
