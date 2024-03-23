#[doc = "Register `RPR1` reader"]
pub type R = crate::R<RPR1rs>;
#[doc = "Register `RPR1` writer"]
pub type W = crate::W<RPR1rs>;
#[doc = "Field `RPIF0` reader - configurable event inputs x rising edge pending bit"]
pub type RPIF0_R = crate::BitReader;
#[doc = "Field `RPIF0` writer - configurable event inputs x rising edge pending bit"]
pub type RPIF0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPIF1` reader - configurable event inputs x rising edge pending bit"]
pub type RPIF1_R = crate::BitReader;
#[doc = "Field `RPIF1` writer - configurable event inputs x rising edge pending bit"]
pub type RPIF1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPIF2` reader - configurable event inputs x rising edge pending bit"]
pub type RPIF2_R = crate::BitReader;
#[doc = "Field `RPIF2` writer - configurable event inputs x rising edge pending bit"]
pub type RPIF2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPIF3` reader - configurable event inputs x rising edge pending bit"]
pub type RPIF3_R = crate::BitReader;
#[doc = "Field `RPIF3` writer - configurable event inputs x rising edge pending bit"]
pub type RPIF3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPIF4` reader - configurable event inputs x rising edge pending bit"]
pub type RPIF4_R = crate::BitReader;
#[doc = "Field `RPIF4` writer - configurable event inputs x rising edge pending bit"]
pub type RPIF4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPIF5` reader - configurable event inputs x rising edge pending bit"]
pub type RPIF5_R = crate::BitReader;
#[doc = "Field `RPIF5` writer - configurable event inputs x rising edge pending bit"]
pub type RPIF5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPIF6` reader - configurable event inputs x rising edge pending bit"]
pub type RPIF6_R = crate::BitReader;
#[doc = "Field `RPIF6` writer - configurable event inputs x rising edge pending bit"]
pub type RPIF6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPIF7` reader - configurable event inputs x rising edge pending bit"]
pub type RPIF7_R = crate::BitReader;
#[doc = "Field `RPIF7` writer - configurable event inputs x rising edge pending bit"]
pub type RPIF7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPIF8` reader - configurable event inputs x rising edge pending bit"]
pub type RPIF8_R = crate::BitReader;
#[doc = "Field `RPIF8` writer - configurable event inputs x rising edge pending bit"]
pub type RPIF8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPIF9` reader - configurable event inputs x rising edge pending bit"]
pub type RPIF9_R = crate::BitReader;
#[doc = "Field `RPIF9` writer - configurable event inputs x rising edge pending bit"]
pub type RPIF9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPIF10` reader - configurable event inputs x rising edge pending bit"]
pub type RPIF10_R = crate::BitReader;
#[doc = "Field `RPIF10` writer - configurable event inputs x rising edge pending bit"]
pub type RPIF10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPIF11` reader - configurable event inputs x rising edge pending bit"]
pub type RPIF11_R = crate::BitReader;
#[doc = "Field `RPIF11` writer - configurable event inputs x rising edge pending bit"]
pub type RPIF11_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPIF12` reader - configurable event inputs x rising edge pending bit"]
pub type RPIF12_R = crate::BitReader;
#[doc = "Field `RPIF12` writer - configurable event inputs x rising edge pending bit"]
pub type RPIF12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPIF13` reader - configurable event inputs x rising edge pending bit"]
pub type RPIF13_R = crate::BitReader;
#[doc = "Field `RPIF13` writer - configurable event inputs x rising edge pending bit"]
pub type RPIF13_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPIF14` reader - configurable event inputs x rising edge pending bit"]
pub type RPIF14_R = crate::BitReader;
#[doc = "Field `RPIF14` writer - configurable event inputs x rising edge pending bit"]
pub type RPIF14_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPIF15` reader - configurable event inputs x rising edge pending bit"]
pub type RPIF15_R = crate::BitReader;
#[doc = "Field `RPIF15` writer - configurable event inputs x rising edge pending bit"]
pub type RPIF15_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPIF16` reader - configurable event inputs x rising edge pending bit"]
pub type RPIF16_R = crate::BitReader;
#[doc = "Field `RPIF16` writer - configurable event inputs x rising edge pending bit"]
pub type RPIF16_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPIF21` reader - configurable event inputs x rising edge pending bit"]
pub type RPIF21_R = crate::BitReader;
#[doc = "Field `RPIF21` writer - configurable event inputs x rising edge pending bit"]
pub type RPIF21_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPIF22` reader - configurable event inputs x rising edge pending bit"]
pub type RPIF22_R = crate::BitReader;
#[doc = "Field `RPIF22` writer - configurable event inputs x rising edge pending bit"]
pub type RPIF22_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - configurable event inputs x rising edge pending bit"]
    #[inline(always)]
    pub fn rpif0(&self) -> RPIF0_R {
        RPIF0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - configurable event inputs x rising edge pending bit"]
    #[inline(always)]
    pub fn rpif1(&self) -> RPIF1_R {
        RPIF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - configurable event inputs x rising edge pending bit"]
    #[inline(always)]
    pub fn rpif2(&self) -> RPIF2_R {
        RPIF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - configurable event inputs x rising edge pending bit"]
    #[inline(always)]
    pub fn rpif3(&self) -> RPIF3_R {
        RPIF3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - configurable event inputs x rising edge pending bit"]
    #[inline(always)]
    pub fn rpif4(&self) -> RPIF4_R {
        RPIF4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - configurable event inputs x rising edge pending bit"]
    #[inline(always)]
    pub fn rpif5(&self) -> RPIF5_R {
        RPIF5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - configurable event inputs x rising edge pending bit"]
    #[inline(always)]
    pub fn rpif6(&self) -> RPIF6_R {
        RPIF6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - configurable event inputs x rising edge pending bit"]
    #[inline(always)]
    pub fn rpif7(&self) -> RPIF7_R {
        RPIF7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - configurable event inputs x rising edge pending bit"]
    #[inline(always)]
    pub fn rpif8(&self) -> RPIF8_R {
        RPIF8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - configurable event inputs x rising edge pending bit"]
    #[inline(always)]
    pub fn rpif9(&self) -> RPIF9_R {
        RPIF9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - configurable event inputs x rising edge pending bit"]
    #[inline(always)]
    pub fn rpif10(&self) -> RPIF10_R {
        RPIF10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - configurable event inputs x rising edge pending bit"]
    #[inline(always)]
    pub fn rpif11(&self) -> RPIF11_R {
        RPIF11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - configurable event inputs x rising edge pending bit"]
    #[inline(always)]
    pub fn rpif12(&self) -> RPIF12_R {
        RPIF12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - configurable event inputs x rising edge pending bit"]
    #[inline(always)]
    pub fn rpif13(&self) -> RPIF13_R {
        RPIF13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - configurable event inputs x rising edge pending bit"]
    #[inline(always)]
    pub fn rpif14(&self) -> RPIF14_R {
        RPIF14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - configurable event inputs x rising edge pending bit"]
    #[inline(always)]
    pub fn rpif15(&self) -> RPIF15_R {
        RPIF15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - configurable event inputs x rising edge pending bit"]
    #[inline(always)]
    pub fn rpif16(&self) -> RPIF16_R {
        RPIF16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 21 - configurable event inputs x rising edge pending bit"]
    #[inline(always)]
    pub fn rpif21(&self) -> RPIF21_R {
        RPIF21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - configurable event inputs x rising edge pending bit"]
    #[inline(always)]
    pub fn rpif22(&self) -> RPIF22_R {
        RPIF22_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - configurable event inputs x rising edge pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn rpif0(&mut self) -> RPIF0_W<RPR1rs> {
        RPIF0_W::new(self, 0)
    }
    #[doc = "Bit 1 - configurable event inputs x rising edge pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn rpif1(&mut self) -> RPIF1_W<RPR1rs> {
        RPIF1_W::new(self, 1)
    }
    #[doc = "Bit 2 - configurable event inputs x rising edge pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn rpif2(&mut self) -> RPIF2_W<RPR1rs> {
        RPIF2_W::new(self, 2)
    }
    #[doc = "Bit 3 - configurable event inputs x rising edge pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn rpif3(&mut self) -> RPIF3_W<RPR1rs> {
        RPIF3_W::new(self, 3)
    }
    #[doc = "Bit 4 - configurable event inputs x rising edge pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn rpif4(&mut self) -> RPIF4_W<RPR1rs> {
        RPIF4_W::new(self, 4)
    }
    #[doc = "Bit 5 - configurable event inputs x rising edge pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn rpif5(&mut self) -> RPIF5_W<RPR1rs> {
        RPIF5_W::new(self, 5)
    }
    #[doc = "Bit 6 - configurable event inputs x rising edge pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn rpif6(&mut self) -> RPIF6_W<RPR1rs> {
        RPIF6_W::new(self, 6)
    }
    #[doc = "Bit 7 - configurable event inputs x rising edge pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn rpif7(&mut self) -> RPIF7_W<RPR1rs> {
        RPIF7_W::new(self, 7)
    }
    #[doc = "Bit 8 - configurable event inputs x rising edge pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn rpif8(&mut self) -> RPIF8_W<RPR1rs> {
        RPIF8_W::new(self, 8)
    }
    #[doc = "Bit 9 - configurable event inputs x rising edge pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn rpif9(&mut self) -> RPIF9_W<RPR1rs> {
        RPIF9_W::new(self, 9)
    }
    #[doc = "Bit 10 - configurable event inputs x rising edge pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn rpif10(&mut self) -> RPIF10_W<RPR1rs> {
        RPIF10_W::new(self, 10)
    }
    #[doc = "Bit 11 - configurable event inputs x rising edge pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn rpif11(&mut self) -> RPIF11_W<RPR1rs> {
        RPIF11_W::new(self, 11)
    }
    #[doc = "Bit 12 - configurable event inputs x rising edge pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn rpif12(&mut self) -> RPIF12_W<RPR1rs> {
        RPIF12_W::new(self, 12)
    }
    #[doc = "Bit 13 - configurable event inputs x rising edge pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn rpif13(&mut self) -> RPIF13_W<RPR1rs> {
        RPIF13_W::new(self, 13)
    }
    #[doc = "Bit 14 - configurable event inputs x rising edge pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn rpif14(&mut self) -> RPIF14_W<RPR1rs> {
        RPIF14_W::new(self, 14)
    }
    #[doc = "Bit 15 - configurable event inputs x rising edge pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn rpif15(&mut self) -> RPIF15_W<RPR1rs> {
        RPIF15_W::new(self, 15)
    }
    #[doc = "Bit 16 - configurable event inputs x rising edge pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn rpif16(&mut self) -> RPIF16_W<RPR1rs> {
        RPIF16_W::new(self, 16)
    }
    #[doc = "Bit 21 - configurable event inputs x rising edge pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn rpif21(&mut self) -> RPIF21_W<RPR1rs> {
        RPIF21_W::new(self, 21)
    }
    #[doc = "Bit 22 - configurable event inputs x rising edge pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn rpif22(&mut self) -> RPIF22_W<RPR1rs> {
        RPIF22_W::new(self, 22)
    }
}
#[doc = "EXTI rising edge pending register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rpr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rpr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RPR1rs;
impl crate::RegisterSpec for RPR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rpr1::R`](R) reader structure"]
impl crate::Readable for RPR1rs {}
#[doc = "`write(|w| ..)` method takes [`rpr1::W`](W) writer structure"]
impl crate::Writable for RPR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RPR1 to value 0"]
impl crate::Resettable for RPR1rs {
    const RESET_VALUE: u32 = 0;
}
