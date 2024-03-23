#[doc = "Register `LPGPIO_ODR` reader"]
pub type R = crate::R<LPGPIO_ODRrs>;
#[doc = "Register `LPGPIO_ODR` writer"]
pub type W = crate::W<LPGPIO_ODRrs>;
#[doc = "Field `OD0` reader - OD0"]
pub type OD0_R = crate::BitReader;
#[doc = "Field `OD0` writer - OD0"]
pub type OD0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OD1` reader - OD1"]
pub type OD1_R = crate::BitReader;
#[doc = "Field `OD1` writer - OD1"]
pub type OD1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OD2` reader - OD2"]
pub type OD2_R = crate::BitReader;
#[doc = "Field `OD2` writer - OD2"]
pub type OD2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OD3` reader - OD3"]
pub type OD3_R = crate::BitReader;
#[doc = "Field `OD3` writer - OD3"]
pub type OD3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OD4` reader - OD4"]
pub type OD4_R = crate::BitReader;
#[doc = "Field `OD4` writer - OD4"]
pub type OD4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OD5` reader - OD5"]
pub type OD5_R = crate::BitReader;
#[doc = "Field `OD5` writer - OD5"]
pub type OD5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OD6` reader - OD6"]
pub type OD6_R = crate::BitReader;
#[doc = "Field `OD6` writer - OD6"]
pub type OD6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OD7` reader - OD7"]
pub type OD7_R = crate::BitReader;
#[doc = "Field `OD7` writer - OD7"]
pub type OD7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OD8` reader - OD8"]
pub type OD8_R = crate::BitReader;
#[doc = "Field `OD8` writer - OD8"]
pub type OD8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OD9` reader - OD9"]
pub type OD9_R = crate::BitReader;
#[doc = "Field `OD9` writer - OD9"]
pub type OD9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OD10` reader - OD10"]
pub type OD10_R = crate::BitReader;
#[doc = "Field `OD10` writer - OD10"]
pub type OD10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OD11` reader - OD11"]
pub type OD11_R = crate::BitReader;
#[doc = "Field `OD11` writer - OD11"]
pub type OD11_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OD12` reader - OD12"]
pub type OD12_R = crate::BitReader;
#[doc = "Field `OD12` writer - OD12"]
pub type OD12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OD13` reader - OD13"]
pub type OD13_R = crate::BitReader;
#[doc = "Field `OD13` writer - OD13"]
pub type OD13_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OD14` reader - OD14"]
pub type OD14_R = crate::BitReader;
#[doc = "Field `OD14` writer - OD14"]
pub type OD14_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OD15` reader - OD15"]
pub type OD15_R = crate::BitReader;
#[doc = "Field `OD15` writer - OD15"]
pub type OD15_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - OD0"]
    #[inline(always)]
    pub fn od0(&self) -> OD0_R {
        OD0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OD1"]
    #[inline(always)]
    pub fn od1(&self) -> OD1_R {
        OD1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - OD2"]
    #[inline(always)]
    pub fn od2(&self) -> OD2_R {
        OD2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - OD3"]
    #[inline(always)]
    pub fn od3(&self) -> OD3_R {
        OD3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - OD4"]
    #[inline(always)]
    pub fn od4(&self) -> OD4_R {
        OD4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - OD5"]
    #[inline(always)]
    pub fn od5(&self) -> OD5_R {
        OD5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - OD6"]
    #[inline(always)]
    pub fn od6(&self) -> OD6_R {
        OD6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - OD7"]
    #[inline(always)]
    pub fn od7(&self) -> OD7_R {
        OD7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - OD8"]
    #[inline(always)]
    pub fn od8(&self) -> OD8_R {
        OD8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - OD9"]
    #[inline(always)]
    pub fn od9(&self) -> OD9_R {
        OD9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - OD10"]
    #[inline(always)]
    pub fn od10(&self) -> OD10_R {
        OD10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - OD11"]
    #[inline(always)]
    pub fn od11(&self) -> OD11_R {
        OD11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - OD12"]
    #[inline(always)]
    pub fn od12(&self) -> OD12_R {
        OD12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - OD13"]
    #[inline(always)]
    pub fn od13(&self) -> OD13_R {
        OD13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - OD14"]
    #[inline(always)]
    pub fn od14(&self) -> OD14_R {
        OD14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - OD15"]
    #[inline(always)]
    pub fn od15(&self) -> OD15_R {
        OD15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OD0"]
    #[inline(always)]
    #[must_use]
    pub fn od0(&mut self) -> OD0_W<LPGPIO_ODRrs> {
        OD0_W::new(self, 0)
    }
    #[doc = "Bit 1 - OD1"]
    #[inline(always)]
    #[must_use]
    pub fn od1(&mut self) -> OD1_W<LPGPIO_ODRrs> {
        OD1_W::new(self, 1)
    }
    #[doc = "Bit 2 - OD2"]
    #[inline(always)]
    #[must_use]
    pub fn od2(&mut self) -> OD2_W<LPGPIO_ODRrs> {
        OD2_W::new(self, 2)
    }
    #[doc = "Bit 3 - OD3"]
    #[inline(always)]
    #[must_use]
    pub fn od3(&mut self) -> OD3_W<LPGPIO_ODRrs> {
        OD3_W::new(self, 3)
    }
    #[doc = "Bit 4 - OD4"]
    #[inline(always)]
    #[must_use]
    pub fn od4(&mut self) -> OD4_W<LPGPIO_ODRrs> {
        OD4_W::new(self, 4)
    }
    #[doc = "Bit 5 - OD5"]
    #[inline(always)]
    #[must_use]
    pub fn od5(&mut self) -> OD5_W<LPGPIO_ODRrs> {
        OD5_W::new(self, 5)
    }
    #[doc = "Bit 6 - OD6"]
    #[inline(always)]
    #[must_use]
    pub fn od6(&mut self) -> OD6_W<LPGPIO_ODRrs> {
        OD6_W::new(self, 6)
    }
    #[doc = "Bit 7 - OD7"]
    #[inline(always)]
    #[must_use]
    pub fn od7(&mut self) -> OD7_W<LPGPIO_ODRrs> {
        OD7_W::new(self, 7)
    }
    #[doc = "Bit 8 - OD8"]
    #[inline(always)]
    #[must_use]
    pub fn od8(&mut self) -> OD8_W<LPGPIO_ODRrs> {
        OD8_W::new(self, 8)
    }
    #[doc = "Bit 9 - OD9"]
    #[inline(always)]
    #[must_use]
    pub fn od9(&mut self) -> OD9_W<LPGPIO_ODRrs> {
        OD9_W::new(self, 9)
    }
    #[doc = "Bit 10 - OD10"]
    #[inline(always)]
    #[must_use]
    pub fn od10(&mut self) -> OD10_W<LPGPIO_ODRrs> {
        OD10_W::new(self, 10)
    }
    #[doc = "Bit 11 - OD11"]
    #[inline(always)]
    #[must_use]
    pub fn od11(&mut self) -> OD11_W<LPGPIO_ODRrs> {
        OD11_W::new(self, 11)
    }
    #[doc = "Bit 12 - OD12"]
    #[inline(always)]
    #[must_use]
    pub fn od12(&mut self) -> OD12_W<LPGPIO_ODRrs> {
        OD12_W::new(self, 12)
    }
    #[doc = "Bit 13 - OD13"]
    #[inline(always)]
    #[must_use]
    pub fn od13(&mut self) -> OD13_W<LPGPIO_ODRrs> {
        OD13_W::new(self, 13)
    }
    #[doc = "Bit 14 - OD14"]
    #[inline(always)]
    #[must_use]
    pub fn od14(&mut self) -> OD14_W<LPGPIO_ODRrs> {
        OD14_W::new(self, 14)
    }
    #[doc = "Bit 15 - OD15"]
    #[inline(always)]
    #[must_use]
    pub fn od15(&mut self) -> OD15_W<LPGPIO_ODRrs> {
        OD15_W::new(self, 15)
    }
}
#[doc = "LPGPIO port output data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpgpio_odr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpgpio_odr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LPGPIO_ODRrs;
impl crate::RegisterSpec for LPGPIO_ODRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpgpio_odr::R`](R) reader structure"]
impl crate::Readable for LPGPIO_ODRrs {}
#[doc = "`write(|w| ..)` method takes [`lpgpio_odr::W`](W) writer structure"]
impl crate::Writable for LPGPIO_ODRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LPGPIO_ODR to value 0"]
impl crate::Resettable for LPGPIO_ODRrs {
    const RESET_VALUE: u32 = 0;
}
