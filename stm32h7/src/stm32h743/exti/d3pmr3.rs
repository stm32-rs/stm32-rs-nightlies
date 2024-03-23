#[doc = "Register `D3PMR3` reader"]
pub type R = crate::R<D3PMR3rs>;
#[doc = "Register `D3PMR3` writer"]
pub type W = crate::W<D3PMR3rs>;
#[doc = "D3 Pending Mask on Event input x+64\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MR88 {
    #[doc = "0: Interrupt request line is masked"]
    Masked = 0,
    #[doc = "1: Interrupt request line is unmasked"]
    Unmasked = 1,
}
impl From<MR88> for bool {
    #[inline(always)]
    fn from(variant: MR88) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR88` reader - D3 Pending Mask on Event input x+64"]
pub type MR88_R = crate::BitReader<MR88>;
impl MR88_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MR88 {
        match self.bits {
            false => MR88::Masked,
            true => MR88::Unmasked,
        }
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == MR88::Masked
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == MR88::Unmasked
    }
}
#[doc = "Field `MR88` writer - D3 Pending Mask on Event input x+64"]
pub type MR88_W<'a, REG> = crate::BitWriter<'a, REG, MR88>;
impl<'a, REG> MR88_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(MR88::Masked)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(MR88::Unmasked)
    }
}
impl R {
    #[doc = "Bit 24 - D3 Pending Mask on Event input x+64"]
    #[inline(always)]
    pub fn mr88(&self) -> MR88_R {
        MR88_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - D3 Pending Mask on Event input x+64"]
    #[inline(always)]
    #[must_use]
    pub fn mr88(&mut self) -> MR88_W<D3PMR3rs> {
        MR88_W::new(self, 24)
    }
}
#[doc = "EXTI D3 pending mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d3pmr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d3pmr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D3PMR3rs;
impl crate::RegisterSpec for D3PMR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d3pmr3::R`](R) reader structure"]
impl crate::Readable for D3PMR3rs {}
#[doc = "`write(|w| ..)` method takes [`d3pmr3::W`](W) writer structure"]
impl crate::Writable for D3PMR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets D3PMR3 to value 0"]
impl crate::Resettable for D3PMR3rs {
    const RESET_VALUE: u32 = 0;
}
