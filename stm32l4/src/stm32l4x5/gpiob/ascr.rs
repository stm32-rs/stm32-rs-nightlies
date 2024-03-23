#[doc = "Register `ASCR` reader"]
pub type R = crate::R<ASCRrs>;
#[doc = "Register `ASCR` writer"]
pub type W = crate::W<ASCRrs>;
#[doc = "These bits are written by software to configure the analog connection of the IOs.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASC0W {
    #[doc = "0: Disconnect analog switch to the ADC input"]
    NoAction = 0,
    #[doc = "1: Connect analog switch to the ADC input"]
    Reset = 1,
}
impl From<ASC0W> for bool {
    #[inline(always)]
    fn from(variant: ASC0W) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASC0` reader - These bits are written by software to configure the analog connection of the IOs."]
pub type ASC0_R = crate::BitReader<ASC0W>;
impl ASC0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ASC0W {
        match self.bits {
            false => ASC0W::NoAction,
            true => ASC0W::Reset,
        }
    }
    #[doc = "Disconnect analog switch to the ADC input"]
    #[inline(always)]
    pub fn is_no_action(&self) -> bool {
        *self == ASC0W::NoAction
    }
    #[doc = "Connect analog switch to the ADC input"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == ASC0W::Reset
    }
}
#[doc = "Field `ASC0` writer - These bits are written by software to configure the analog connection of the IOs."]
pub type ASC0_W<'a, REG> = crate::BitWriter<'a, REG, ASC0W>;
impl<'a, REG> ASC0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disconnect analog switch to the ADC input"]
    #[inline(always)]
    pub fn no_action(self) -> &'a mut crate::W<REG> {
        self.variant(ASC0W::NoAction)
    }
    #[doc = "Connect analog switch to the ADC input"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(ASC0W::Reset)
    }
}
#[doc = "Field `ASC1` reader - These bits are written by software to configure the analog connection of the IOs."]
pub use ASC0_R as ASC1_R;
#[doc = "Field `ASC2` reader - These bits are written by software to configure the analog connection of the IOs."]
pub use ASC0_R as ASC2_R;
#[doc = "Field `ASC3` reader - These bits are written by software to configure the analog connection of the IOs."]
pub use ASC0_R as ASC3_R;
#[doc = "Field `ASC4` reader - These bits are written by software to configure the analog connection of the IOs."]
pub use ASC0_R as ASC4_R;
#[doc = "Field `ASC5` reader - These bits are written by software to configure the analog connection of the IOs."]
pub use ASC0_R as ASC5_R;
#[doc = "Field `ASC6` reader - These bits are written by software to configure the analog connection of the IOs."]
pub use ASC0_R as ASC6_R;
#[doc = "Field `ASC7` reader - These bits are written by software to configure the analog connection of the IOs."]
pub use ASC0_R as ASC7_R;
#[doc = "Field `ASC8` reader - These bits are written by software to configure the analog connection of the IOs."]
pub use ASC0_R as ASC8_R;
#[doc = "Field `ASC9` reader - These bits are written by software to configure the analog connection of the IOs."]
pub use ASC0_R as ASC9_R;
#[doc = "Field `ASC10` reader - These bits are written by software to configure the analog connection of the IOs."]
pub use ASC0_R as ASC10_R;
#[doc = "Field `ASC11` reader - These bits are written by software to configure the analog connection of the IOs."]
pub use ASC0_R as ASC11_R;
#[doc = "Field `ASC12` reader - These bits are written by software to configure the analog connection of the IOs."]
pub use ASC0_R as ASC12_R;
#[doc = "Field `ASC13` reader - These bits are written by software to configure the analog connection of the IOs."]
pub use ASC0_R as ASC13_R;
#[doc = "Field `ASC14` reader - These bits are written by software to configure the analog connection of the IOs."]
pub use ASC0_R as ASC14_R;
#[doc = "Field `ASC15` reader - These bits are written by software to configure the analog connection of the IOs."]
pub use ASC0_R as ASC15_R;
#[doc = "Field `ASC1` writer - These bits are written by software to configure the analog connection of the IOs."]
pub use ASC0_W as ASC1_W;
#[doc = "Field `ASC2` writer - These bits are written by software to configure the analog connection of the IOs."]
pub use ASC0_W as ASC2_W;
#[doc = "Field `ASC3` writer - These bits are written by software to configure the analog connection of the IOs."]
pub use ASC0_W as ASC3_W;
#[doc = "Field `ASC4` writer - These bits are written by software to configure the analog connection of the IOs."]
pub use ASC0_W as ASC4_W;
#[doc = "Field `ASC5` writer - These bits are written by software to configure the analog connection of the IOs."]
pub use ASC0_W as ASC5_W;
#[doc = "Field `ASC6` writer - These bits are written by software to configure the analog connection of the IOs."]
pub use ASC0_W as ASC6_W;
#[doc = "Field `ASC7` writer - These bits are written by software to configure the analog connection of the IOs."]
pub use ASC0_W as ASC7_W;
#[doc = "Field `ASC8` writer - These bits are written by software to configure the analog connection of the IOs."]
pub use ASC0_W as ASC8_W;
#[doc = "Field `ASC9` writer - These bits are written by software to configure the analog connection of the IOs."]
pub use ASC0_W as ASC9_W;
#[doc = "Field `ASC10` writer - These bits are written by software to configure the analog connection of the IOs."]
pub use ASC0_W as ASC10_W;
#[doc = "Field `ASC11` writer - These bits are written by software to configure the analog connection of the IOs."]
pub use ASC0_W as ASC11_W;
#[doc = "Field `ASC12` writer - These bits are written by software to configure the analog connection of the IOs."]
pub use ASC0_W as ASC12_W;
#[doc = "Field `ASC13` writer - These bits are written by software to configure the analog connection of the IOs."]
pub use ASC0_W as ASC13_W;
#[doc = "Field `ASC14` writer - These bits are written by software to configure the analog connection of the IOs."]
pub use ASC0_W as ASC14_W;
#[doc = "Field `ASC15` writer - These bits are written by software to configure the analog connection of the IOs."]
pub use ASC0_W as ASC15_W;
impl R {
    #[doc = "Bit 0 - These bits are written by software to configure the analog connection of the IOs."]
    #[inline(always)]
    pub fn asc0(&self) -> ASC0_R {
        ASC0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - These bits are written by software to configure the analog connection of the IOs."]
    #[inline(always)]
    pub fn asc1(&self) -> ASC1_R {
        ASC1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - These bits are written by software to configure the analog connection of the IOs."]
    #[inline(always)]
    pub fn asc2(&self) -> ASC2_R {
        ASC2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - These bits are written by software to configure the analog connection of the IOs."]
    #[inline(always)]
    pub fn asc3(&self) -> ASC3_R {
        ASC3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - These bits are written by software to configure the analog connection of the IOs."]
    #[inline(always)]
    pub fn asc4(&self) -> ASC4_R {
        ASC4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - These bits are written by software to configure the analog connection of the IOs."]
    #[inline(always)]
    pub fn asc5(&self) -> ASC5_R {
        ASC5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - These bits are written by software to configure the analog connection of the IOs."]
    #[inline(always)]
    pub fn asc6(&self) -> ASC6_R {
        ASC6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - These bits are written by software to configure the analog connection of the IOs."]
    #[inline(always)]
    pub fn asc7(&self) -> ASC7_R {
        ASC7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - These bits are written by software to configure the analog connection of the IOs."]
    #[inline(always)]
    pub fn asc8(&self) -> ASC8_R {
        ASC8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - These bits are written by software to configure the analog connection of the IOs."]
    #[inline(always)]
    pub fn asc9(&self) -> ASC9_R {
        ASC9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - These bits are written by software to configure the analog connection of the IOs."]
    #[inline(always)]
    pub fn asc10(&self) -> ASC10_R {
        ASC10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - These bits are written by software to configure the analog connection of the IOs."]
    #[inline(always)]
    pub fn asc11(&self) -> ASC11_R {
        ASC11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - These bits are written by software to configure the analog connection of the IOs."]
    #[inline(always)]
    pub fn asc12(&self) -> ASC12_R {
        ASC12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - These bits are written by software to configure the analog connection of the IOs."]
    #[inline(always)]
    pub fn asc13(&self) -> ASC13_R {
        ASC13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - These bits are written by software to configure the analog connection of the IOs."]
    #[inline(always)]
    pub fn asc14(&self) -> ASC14_R {
        ASC14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - These bits are written by software to configure the analog connection of the IOs."]
    #[inline(always)]
    pub fn asc15(&self) -> ASC15_R {
        ASC15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - These bits are written by software to configure the analog connection of the IOs."]
    #[inline(always)]
    #[must_use]
    pub fn asc0(&mut self) -> ASC0_W<ASCRrs> {
        ASC0_W::new(self, 0)
    }
    #[doc = "Bit 1 - These bits are written by software to configure the analog connection of the IOs."]
    #[inline(always)]
    #[must_use]
    pub fn asc1(&mut self) -> ASC1_W<ASCRrs> {
        ASC1_W::new(self, 1)
    }
    #[doc = "Bit 2 - These bits are written by software to configure the analog connection of the IOs."]
    #[inline(always)]
    #[must_use]
    pub fn asc2(&mut self) -> ASC2_W<ASCRrs> {
        ASC2_W::new(self, 2)
    }
    #[doc = "Bit 3 - These bits are written by software to configure the analog connection of the IOs."]
    #[inline(always)]
    #[must_use]
    pub fn asc3(&mut self) -> ASC3_W<ASCRrs> {
        ASC3_W::new(self, 3)
    }
    #[doc = "Bit 4 - These bits are written by software to configure the analog connection of the IOs."]
    #[inline(always)]
    #[must_use]
    pub fn asc4(&mut self) -> ASC4_W<ASCRrs> {
        ASC4_W::new(self, 4)
    }
    #[doc = "Bit 5 - These bits are written by software to configure the analog connection of the IOs."]
    #[inline(always)]
    #[must_use]
    pub fn asc5(&mut self) -> ASC5_W<ASCRrs> {
        ASC5_W::new(self, 5)
    }
    #[doc = "Bit 6 - These bits are written by software to configure the analog connection of the IOs."]
    #[inline(always)]
    #[must_use]
    pub fn asc6(&mut self) -> ASC6_W<ASCRrs> {
        ASC6_W::new(self, 6)
    }
    #[doc = "Bit 7 - These bits are written by software to configure the analog connection of the IOs."]
    #[inline(always)]
    #[must_use]
    pub fn asc7(&mut self) -> ASC7_W<ASCRrs> {
        ASC7_W::new(self, 7)
    }
    #[doc = "Bit 8 - These bits are written by software to configure the analog connection of the IOs."]
    #[inline(always)]
    #[must_use]
    pub fn asc8(&mut self) -> ASC8_W<ASCRrs> {
        ASC8_W::new(self, 8)
    }
    #[doc = "Bit 9 - These bits are written by software to configure the analog connection of the IOs."]
    #[inline(always)]
    #[must_use]
    pub fn asc9(&mut self) -> ASC9_W<ASCRrs> {
        ASC9_W::new(self, 9)
    }
    #[doc = "Bit 10 - These bits are written by software to configure the analog connection of the IOs."]
    #[inline(always)]
    #[must_use]
    pub fn asc10(&mut self) -> ASC10_W<ASCRrs> {
        ASC10_W::new(self, 10)
    }
    #[doc = "Bit 11 - These bits are written by software to configure the analog connection of the IOs."]
    #[inline(always)]
    #[must_use]
    pub fn asc11(&mut self) -> ASC11_W<ASCRrs> {
        ASC11_W::new(self, 11)
    }
    #[doc = "Bit 12 - These bits are written by software to configure the analog connection of the IOs."]
    #[inline(always)]
    #[must_use]
    pub fn asc12(&mut self) -> ASC12_W<ASCRrs> {
        ASC12_W::new(self, 12)
    }
    #[doc = "Bit 13 - These bits are written by software to configure the analog connection of the IOs."]
    #[inline(always)]
    #[must_use]
    pub fn asc13(&mut self) -> ASC13_W<ASCRrs> {
        ASC13_W::new(self, 13)
    }
    #[doc = "Bit 14 - These bits are written by software to configure the analog connection of the IOs."]
    #[inline(always)]
    #[must_use]
    pub fn asc14(&mut self) -> ASC14_W<ASCRrs> {
        ASC14_W::new(self, 14)
    }
    #[doc = "Bit 15 - These bits are written by software to configure the analog connection of the IOs."]
    #[inline(always)]
    #[must_use]
    pub fn asc15(&mut self) -> ASC15_W<ASCRrs> {
        ASC15_W::new(self, 15)
    }
}
#[doc = "GPIO port analog switch control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ascr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ascr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ASCRrs;
impl crate::RegisterSpec for ASCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ascr::R`](R) reader structure"]
impl crate::Readable for ASCRrs {}
#[doc = "`write(|w| ..)` method takes [`ascr::W`](W) writer structure"]
impl crate::Writable for ASCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ASCR to value 0"]
impl crate::Resettable for ASCRrs {
    const RESET_VALUE: u32 = 0;
}
