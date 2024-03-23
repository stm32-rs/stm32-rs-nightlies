#[doc = "Register `PWR_PDCRA` reader"]
pub type R = crate::R<PWR_PDCRArs>;
#[doc = "Register `PWR_PDCRA` writer"]
pub type W = crate::W<PWR_PDCRArs>;
#[doc = "Field `PD0` reader - Port A pull-down bit"]
pub type PD0_R = crate::BitReader;
#[doc = "Field `PD0` writer - Port A pull-down bit"]
pub type PD0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD1` reader - Port A pull-down bit"]
pub type PD1_R = crate::BitReader;
#[doc = "Field `PD1` writer - Port A pull-down bit"]
pub type PD1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD2` reader - Port A pull-down bit"]
pub type PD2_R = crate::BitReader;
#[doc = "Field `PD2` writer - Port A pull-down bit"]
pub type PD2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD3` reader - Port A pull-down bit"]
pub type PD3_R = crate::BitReader;
#[doc = "Field `PD3` writer - Port A pull-down bit"]
pub type PD3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD4` reader - Port A pull-down bit"]
pub type PD4_R = crate::BitReader;
#[doc = "Field `PD4` writer - Port A pull-down bit"]
pub type PD4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD5` reader - Port A pull-down bit"]
pub type PD5_R = crate::BitReader;
#[doc = "Field `PD5` writer - Port A pull-down bit"]
pub type PD5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD6` reader - Port A pull-down bit"]
pub type PD6_R = crate::BitReader;
#[doc = "Field `PD6` writer - Port A pull-down bit"]
pub type PD6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD7` reader - Port A pull-down bit"]
pub type PD7_R = crate::BitReader;
#[doc = "Field `PD7` writer - Port A pull-down bit"]
pub type PD7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD8` reader - Port A pull-down bit"]
pub type PD8_R = crate::BitReader;
#[doc = "Field `PD8` writer - Port A pull-down bit"]
pub type PD8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD9` reader - Port A pull-down bit"]
pub type PD9_R = crate::BitReader;
#[doc = "Field `PD9` writer - Port A pull-down bit"]
pub type PD9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD10` reader - Port A pull-down bit"]
pub type PD10_R = crate::BitReader;
#[doc = "Field `PD10` writer - Port A pull-down bit"]
pub type PD10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD11` reader - Port A pull-down bit"]
pub type PD11_R = crate::BitReader;
#[doc = "Field `PD11` writer - Port A pull-down bit"]
pub type PD11_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD12` reader - Port A pull-down bit"]
pub type PD12_R = crate::BitReader;
#[doc = "Field `PD12` writer - Port A pull-down bit"]
pub type PD12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD14` reader - Port A pull-down bit"]
pub type PD14_R = crate::BitReader;
#[doc = "Field `PD14` writer - Port A pull-down bit"]
pub type PD14_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Port A pull-down bit"]
    #[inline(always)]
    pub fn pd0(&self) -> PD0_R {
        PD0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port A pull-down bit"]
    #[inline(always)]
    pub fn pd1(&self) -> PD1_R {
        PD1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port A pull-down bit"]
    #[inline(always)]
    pub fn pd2(&self) -> PD2_R {
        PD2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port A pull-down bit"]
    #[inline(always)]
    pub fn pd3(&self) -> PD3_R {
        PD3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port A pull-down bit"]
    #[inline(always)]
    pub fn pd4(&self) -> PD4_R {
        PD4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port A pull-down bit"]
    #[inline(always)]
    pub fn pd5(&self) -> PD5_R {
        PD5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port A pull-down bit"]
    #[inline(always)]
    pub fn pd6(&self) -> PD6_R {
        PD6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port A pull-down bit"]
    #[inline(always)]
    pub fn pd7(&self) -> PD7_R {
        PD7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port A pull-down bit"]
    #[inline(always)]
    pub fn pd8(&self) -> PD8_R {
        PD8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port A pull-down bit"]
    #[inline(always)]
    pub fn pd9(&self) -> PD9_R {
        PD9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port A pull-down bit"]
    #[inline(always)]
    pub fn pd10(&self) -> PD10_R {
        PD10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port A pull-down bit"]
    #[inline(always)]
    pub fn pd11(&self) -> PD11_R {
        PD11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port A pull-down bit"]
    #[inline(always)]
    pub fn pd12(&self) -> PD12_R {
        PD12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Port A pull-down bit"]
    #[inline(always)]
    pub fn pd14(&self) -> PD14_R {
        PD14_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port A pull-down bit"]
    #[inline(always)]
    #[must_use]
    pub fn pd0(&mut self) -> PD0_W<PWR_PDCRArs> {
        PD0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Port A pull-down bit"]
    #[inline(always)]
    #[must_use]
    pub fn pd1(&mut self) -> PD1_W<PWR_PDCRArs> {
        PD1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Port A pull-down bit"]
    #[inline(always)]
    #[must_use]
    pub fn pd2(&mut self) -> PD2_W<PWR_PDCRArs> {
        PD2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Port A pull-down bit"]
    #[inline(always)]
    #[must_use]
    pub fn pd3(&mut self) -> PD3_W<PWR_PDCRArs> {
        PD3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Port A pull-down bit"]
    #[inline(always)]
    #[must_use]
    pub fn pd4(&mut self) -> PD4_W<PWR_PDCRArs> {
        PD4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Port A pull-down bit"]
    #[inline(always)]
    #[must_use]
    pub fn pd5(&mut self) -> PD5_W<PWR_PDCRArs> {
        PD5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Port A pull-down bit"]
    #[inline(always)]
    #[must_use]
    pub fn pd6(&mut self) -> PD6_W<PWR_PDCRArs> {
        PD6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Port A pull-down bit"]
    #[inline(always)]
    #[must_use]
    pub fn pd7(&mut self) -> PD7_W<PWR_PDCRArs> {
        PD7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Port A pull-down bit"]
    #[inline(always)]
    #[must_use]
    pub fn pd8(&mut self) -> PD8_W<PWR_PDCRArs> {
        PD8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Port A pull-down bit"]
    #[inline(always)]
    #[must_use]
    pub fn pd9(&mut self) -> PD9_W<PWR_PDCRArs> {
        PD9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Port A pull-down bit"]
    #[inline(always)]
    #[must_use]
    pub fn pd10(&mut self) -> PD10_W<PWR_PDCRArs> {
        PD10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Port A pull-down bit"]
    #[inline(always)]
    #[must_use]
    pub fn pd11(&mut self) -> PD11_W<PWR_PDCRArs> {
        PD11_W::new(self, 11)
    }
    #[doc = "Bit 12 - Port A pull-down bit"]
    #[inline(always)]
    #[must_use]
    pub fn pd12(&mut self) -> PD12_W<PWR_PDCRArs> {
        PD12_W::new(self, 12)
    }
    #[doc = "Bit 14 - Port A pull-down bit"]
    #[inline(always)]
    #[must_use]
    pub fn pd14(&mut self) -> PD14_W<PWR_PDCRArs> {
        PD14_W::new(self, 14)
    }
}
#[doc = "PWR port A pull-down control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_pdcra::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_pdcra::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PWR_PDCRArs;
impl crate::RegisterSpec for PWR_PDCRArs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwr_pdcra::R`](R) reader structure"]
impl crate::Readable for PWR_PDCRArs {}
#[doc = "`write(|w| ..)` method takes [`pwr_pdcra::W`](W) writer structure"]
impl crate::Writable for PWR_PDCRArs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWR_PDCRA to value 0"]
impl crate::Resettable for PWR_PDCRArs {
    const RESET_VALUE: u32 = 0;
}
