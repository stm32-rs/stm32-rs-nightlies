#[doc = "Register `ASCR` reader"]
pub type R = crate::R<ASCRrs>;
#[doc = "Register `ASCR` writer"]
pub type W = crate::W<ASCRrs>;
#[doc = "Field `ASC0` reader - Port analog switch control"]
pub type ASC0_R = crate::BitReader;
#[doc = "Field `ASC0` writer - Port analog switch control"]
pub type ASC0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASC1` reader - Port analog switch control"]
pub type ASC1_R = crate::BitReader;
#[doc = "Field `ASC1` writer - Port analog switch control"]
pub type ASC1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASC2` reader - Port analog switch control"]
pub type ASC2_R = crate::BitReader;
#[doc = "Field `ASC2` writer - Port analog switch control"]
pub type ASC2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASC3` reader - Port analog switch control"]
pub type ASC3_R = crate::BitReader;
#[doc = "Field `ASC3` writer - Port analog switch control"]
pub type ASC3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASC4` reader - Port analog switch control"]
pub type ASC4_R = crate::BitReader;
#[doc = "Field `ASC4` writer - Port analog switch control"]
pub type ASC4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASC5` reader - Port analog switch control"]
pub type ASC5_R = crate::BitReader;
#[doc = "Field `ASC5` writer - Port analog switch control"]
pub type ASC5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASC6` reader - Port analog switch control"]
pub type ASC6_R = crate::BitReader;
#[doc = "Field `ASC6` writer - Port analog switch control"]
pub type ASC6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASC7` reader - Port analog switch control"]
pub type ASC7_R = crate::BitReader;
#[doc = "Field `ASC7` writer - Port analog switch control"]
pub type ASC7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASC8` reader - Port analog switch control"]
pub type ASC8_R = crate::BitReader;
#[doc = "Field `ASC8` writer - Port analog switch control"]
pub type ASC8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASC9` reader - Port analog switch control"]
pub type ASC9_R = crate::BitReader;
#[doc = "Field `ASC9` writer - Port analog switch control"]
pub type ASC9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASC10` reader - Port analog switch control"]
pub type ASC10_R = crate::BitReader;
#[doc = "Field `ASC10` writer - Port analog switch control"]
pub type ASC10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASC11` reader - Port analog switch control"]
pub type ASC11_R = crate::BitReader;
#[doc = "Field `ASC11` writer - Port analog switch control"]
pub type ASC11_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASC12` reader - Port analog switch control"]
pub type ASC12_R = crate::BitReader;
#[doc = "Field `ASC12` writer - Port analog switch control"]
pub type ASC12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASC13` reader - Port analog switch control"]
pub type ASC13_R = crate::BitReader;
#[doc = "Field `ASC13` writer - Port analog switch control"]
pub type ASC13_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASC14` reader - Port analog switch control"]
pub type ASC14_R = crate::BitReader;
#[doc = "Field `ASC14` writer - Port analog switch control"]
pub type ASC14_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASC15` reader - Port analog switch control"]
pub type ASC15_R = crate::BitReader;
#[doc = "Field `ASC15` writer - Port analog switch control"]
pub type ASC15_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Port analog switch control"]
    #[inline(always)]
    pub fn asc0(&self) -> ASC0_R {
        ASC0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port analog switch control"]
    #[inline(always)]
    pub fn asc1(&self) -> ASC1_R {
        ASC1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port analog switch control"]
    #[inline(always)]
    pub fn asc2(&self) -> ASC2_R {
        ASC2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port analog switch control"]
    #[inline(always)]
    pub fn asc3(&self) -> ASC3_R {
        ASC3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port analog switch control"]
    #[inline(always)]
    pub fn asc4(&self) -> ASC4_R {
        ASC4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port analog switch control"]
    #[inline(always)]
    pub fn asc5(&self) -> ASC5_R {
        ASC5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port analog switch control"]
    #[inline(always)]
    pub fn asc6(&self) -> ASC6_R {
        ASC6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port analog switch control"]
    #[inline(always)]
    pub fn asc7(&self) -> ASC7_R {
        ASC7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port analog switch control"]
    #[inline(always)]
    pub fn asc8(&self) -> ASC8_R {
        ASC8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port analog switch control"]
    #[inline(always)]
    pub fn asc9(&self) -> ASC9_R {
        ASC9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port analog switch control"]
    #[inline(always)]
    pub fn asc10(&self) -> ASC10_R {
        ASC10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port analog switch control"]
    #[inline(always)]
    pub fn asc11(&self) -> ASC11_R {
        ASC11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port analog switch control"]
    #[inline(always)]
    pub fn asc12(&self) -> ASC12_R {
        ASC12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port analog switch control"]
    #[inline(always)]
    pub fn asc13(&self) -> ASC13_R {
        ASC13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port analog switch control"]
    #[inline(always)]
    pub fn asc14(&self) -> ASC14_R {
        ASC14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port analog switch control"]
    #[inline(always)]
    pub fn asc15(&self) -> ASC15_R {
        ASC15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port analog switch control"]
    #[inline(always)]
    #[must_use]
    pub fn asc0(&mut self) -> ASC0_W<ASCRrs> {
        ASC0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Port analog switch control"]
    #[inline(always)]
    #[must_use]
    pub fn asc1(&mut self) -> ASC1_W<ASCRrs> {
        ASC1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Port analog switch control"]
    #[inline(always)]
    #[must_use]
    pub fn asc2(&mut self) -> ASC2_W<ASCRrs> {
        ASC2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Port analog switch control"]
    #[inline(always)]
    #[must_use]
    pub fn asc3(&mut self) -> ASC3_W<ASCRrs> {
        ASC3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Port analog switch control"]
    #[inline(always)]
    #[must_use]
    pub fn asc4(&mut self) -> ASC4_W<ASCRrs> {
        ASC4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Port analog switch control"]
    #[inline(always)]
    #[must_use]
    pub fn asc5(&mut self) -> ASC5_W<ASCRrs> {
        ASC5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Port analog switch control"]
    #[inline(always)]
    #[must_use]
    pub fn asc6(&mut self) -> ASC6_W<ASCRrs> {
        ASC6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Port analog switch control"]
    #[inline(always)]
    #[must_use]
    pub fn asc7(&mut self) -> ASC7_W<ASCRrs> {
        ASC7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Port analog switch control"]
    #[inline(always)]
    #[must_use]
    pub fn asc8(&mut self) -> ASC8_W<ASCRrs> {
        ASC8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Port analog switch control"]
    #[inline(always)]
    #[must_use]
    pub fn asc9(&mut self) -> ASC9_W<ASCRrs> {
        ASC9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Port analog switch control"]
    #[inline(always)]
    #[must_use]
    pub fn asc10(&mut self) -> ASC10_W<ASCRrs> {
        ASC10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Port analog switch control"]
    #[inline(always)]
    #[must_use]
    pub fn asc11(&mut self) -> ASC11_W<ASCRrs> {
        ASC11_W::new(self, 11)
    }
    #[doc = "Bit 12 - Port analog switch control"]
    #[inline(always)]
    #[must_use]
    pub fn asc12(&mut self) -> ASC12_W<ASCRrs> {
        ASC12_W::new(self, 12)
    }
    #[doc = "Bit 13 - Port analog switch control"]
    #[inline(always)]
    #[must_use]
    pub fn asc13(&mut self) -> ASC13_W<ASCRrs> {
        ASC13_W::new(self, 13)
    }
    #[doc = "Bit 14 - Port analog switch control"]
    #[inline(always)]
    #[must_use]
    pub fn asc14(&mut self) -> ASC14_W<ASCRrs> {
        ASC14_W::new(self, 14)
    }
    #[doc = "Bit 15 - Port analog switch control"]
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
