#[doc = "Register `LPGPIO_MODER` reader"]
pub type R = crate::R<LPGPIO_MODERrs>;
#[doc = "Register `LPGPIO_MODER` writer"]
pub type W = crate::W<LPGPIO_MODERrs>;
#[doc = "Field `MODE0` reader - MODE0"]
pub type MODE0_R = crate::BitReader;
#[doc = "Field `MODE0` writer - MODE0"]
pub type MODE0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODE1` reader - MODE1"]
pub type MODE1_R = crate::BitReader;
#[doc = "Field `MODE1` writer - MODE1"]
pub type MODE1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODE2` reader - MODE2"]
pub type MODE2_R = crate::BitReader;
#[doc = "Field `MODE2` writer - MODE2"]
pub type MODE2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODE3` reader - MODE3"]
pub type MODE3_R = crate::BitReader;
#[doc = "Field `MODE3` writer - MODE3"]
pub type MODE3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODE4` reader - MODE4"]
pub type MODE4_R = crate::BitReader;
#[doc = "Field `MODE4` writer - MODE4"]
pub type MODE4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODE5` reader - MODE5"]
pub type MODE5_R = crate::BitReader;
#[doc = "Field `MODE5` writer - MODE5"]
pub type MODE5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODE6` reader - MODE6"]
pub type MODE6_R = crate::BitReader;
#[doc = "Field `MODE6` writer - MODE6"]
pub type MODE6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODE7` reader - MODE7"]
pub type MODE7_R = crate::BitReader;
#[doc = "Field `MODE7` writer - MODE7"]
pub type MODE7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODE8` reader - MODE8"]
pub type MODE8_R = crate::BitReader;
#[doc = "Field `MODE8` writer - MODE8"]
pub type MODE8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODE9` reader - MODE9"]
pub type MODE9_R = crate::BitReader;
#[doc = "Field `MODE9` writer - MODE9"]
pub type MODE9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODE10` reader - MODE10"]
pub type MODE10_R = crate::BitReader;
#[doc = "Field `MODE10` writer - MODE10"]
pub type MODE10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODE11` reader - MODE11"]
pub type MODE11_R = crate::BitReader;
#[doc = "Field `MODE11` writer - MODE11"]
pub type MODE11_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODE12` reader - MODE12"]
pub type MODE12_R = crate::BitReader;
#[doc = "Field `MODE12` writer - MODE12"]
pub type MODE12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODE13` reader - MODE13"]
pub type MODE13_R = crate::BitReader;
#[doc = "Field `MODE13` writer - MODE13"]
pub type MODE13_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODE14` reader - MODE14"]
pub type MODE14_R = crate::BitReader;
#[doc = "Field `MODE14` writer - MODE14"]
pub type MODE14_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODE15` reader - MODE15"]
pub type MODE15_R = crate::BitReader;
#[doc = "Field `MODE15` writer - MODE15"]
pub type MODE15_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - MODE0"]
    #[inline(always)]
    pub fn mode0(&self) -> MODE0_R {
        MODE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MODE1"]
    #[inline(always)]
    pub fn mode1(&self) -> MODE1_R {
        MODE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MODE2"]
    #[inline(always)]
    pub fn mode2(&self) -> MODE2_R {
        MODE2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MODE3"]
    #[inline(always)]
    pub fn mode3(&self) -> MODE3_R {
        MODE3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MODE4"]
    #[inline(always)]
    pub fn mode4(&self) -> MODE4_R {
        MODE4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MODE5"]
    #[inline(always)]
    pub fn mode5(&self) -> MODE5_R {
        MODE5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MODE6"]
    #[inline(always)]
    pub fn mode6(&self) -> MODE6_R {
        MODE6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - MODE7"]
    #[inline(always)]
    pub fn mode7(&self) -> MODE7_R {
        MODE7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - MODE8"]
    #[inline(always)]
    pub fn mode8(&self) -> MODE8_R {
        MODE8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - MODE9"]
    #[inline(always)]
    pub fn mode9(&self) -> MODE9_R {
        MODE9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - MODE10"]
    #[inline(always)]
    pub fn mode10(&self) -> MODE10_R {
        MODE10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - MODE11"]
    #[inline(always)]
    pub fn mode11(&self) -> MODE11_R {
        MODE11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - MODE12"]
    #[inline(always)]
    pub fn mode12(&self) -> MODE12_R {
        MODE12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - MODE13"]
    #[inline(always)]
    pub fn mode13(&self) -> MODE13_R {
        MODE13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - MODE14"]
    #[inline(always)]
    pub fn mode14(&self) -> MODE14_R {
        MODE14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - MODE15"]
    #[inline(always)]
    pub fn mode15(&self) -> MODE15_R {
        MODE15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MODE0"]
    #[inline(always)]
    #[must_use]
    pub fn mode0(&mut self) -> MODE0_W<LPGPIO_MODERrs> {
        MODE0_W::new(self, 0)
    }
    #[doc = "Bit 1 - MODE1"]
    #[inline(always)]
    #[must_use]
    pub fn mode1(&mut self) -> MODE1_W<LPGPIO_MODERrs> {
        MODE1_W::new(self, 1)
    }
    #[doc = "Bit 2 - MODE2"]
    #[inline(always)]
    #[must_use]
    pub fn mode2(&mut self) -> MODE2_W<LPGPIO_MODERrs> {
        MODE2_W::new(self, 2)
    }
    #[doc = "Bit 3 - MODE3"]
    #[inline(always)]
    #[must_use]
    pub fn mode3(&mut self) -> MODE3_W<LPGPIO_MODERrs> {
        MODE3_W::new(self, 3)
    }
    #[doc = "Bit 4 - MODE4"]
    #[inline(always)]
    #[must_use]
    pub fn mode4(&mut self) -> MODE4_W<LPGPIO_MODERrs> {
        MODE4_W::new(self, 4)
    }
    #[doc = "Bit 5 - MODE5"]
    #[inline(always)]
    #[must_use]
    pub fn mode5(&mut self) -> MODE5_W<LPGPIO_MODERrs> {
        MODE5_W::new(self, 5)
    }
    #[doc = "Bit 6 - MODE6"]
    #[inline(always)]
    #[must_use]
    pub fn mode6(&mut self) -> MODE6_W<LPGPIO_MODERrs> {
        MODE6_W::new(self, 6)
    }
    #[doc = "Bit 7 - MODE7"]
    #[inline(always)]
    #[must_use]
    pub fn mode7(&mut self) -> MODE7_W<LPGPIO_MODERrs> {
        MODE7_W::new(self, 7)
    }
    #[doc = "Bit 8 - MODE8"]
    #[inline(always)]
    #[must_use]
    pub fn mode8(&mut self) -> MODE8_W<LPGPIO_MODERrs> {
        MODE8_W::new(self, 8)
    }
    #[doc = "Bit 9 - MODE9"]
    #[inline(always)]
    #[must_use]
    pub fn mode9(&mut self) -> MODE9_W<LPGPIO_MODERrs> {
        MODE9_W::new(self, 9)
    }
    #[doc = "Bit 10 - MODE10"]
    #[inline(always)]
    #[must_use]
    pub fn mode10(&mut self) -> MODE10_W<LPGPIO_MODERrs> {
        MODE10_W::new(self, 10)
    }
    #[doc = "Bit 11 - MODE11"]
    #[inline(always)]
    #[must_use]
    pub fn mode11(&mut self) -> MODE11_W<LPGPIO_MODERrs> {
        MODE11_W::new(self, 11)
    }
    #[doc = "Bit 12 - MODE12"]
    #[inline(always)]
    #[must_use]
    pub fn mode12(&mut self) -> MODE12_W<LPGPIO_MODERrs> {
        MODE12_W::new(self, 12)
    }
    #[doc = "Bit 13 - MODE13"]
    #[inline(always)]
    #[must_use]
    pub fn mode13(&mut self) -> MODE13_W<LPGPIO_MODERrs> {
        MODE13_W::new(self, 13)
    }
    #[doc = "Bit 14 - MODE14"]
    #[inline(always)]
    #[must_use]
    pub fn mode14(&mut self) -> MODE14_W<LPGPIO_MODERrs> {
        MODE14_W::new(self, 14)
    }
    #[doc = "Bit 15 - MODE15"]
    #[inline(always)]
    #[must_use]
    pub fn mode15(&mut self) -> MODE15_W<LPGPIO_MODERrs> {
        MODE15_W::new(self, 15)
    }
}
#[doc = "LPGPIO port mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpgpio_moder::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpgpio_moder::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LPGPIO_MODERrs;
impl crate::RegisterSpec for LPGPIO_MODERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpgpio_moder::R`](R) reader structure"]
impl crate::Readable for LPGPIO_MODERrs {}
#[doc = "`write(|w| ..)` method takes [`lpgpio_moder::W`](W) writer structure"]
impl crate::Writable for LPGPIO_MODERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LPGPIO_MODER to value 0"]
impl crate::Resettable for LPGPIO_MODERrs {
    const RESET_VALUE: u32 = 0;
}
