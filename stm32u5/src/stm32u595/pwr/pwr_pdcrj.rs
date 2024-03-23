#[doc = "Register `PWR_PDCRJ` reader"]
pub type R = crate::R<PWR_PDCRJrs>;
#[doc = "Register `PWR_PDCRJ` writer"]
pub type W = crate::W<PWR_PDCRJrs>;
#[doc = "Field `PD0` reader - "]
pub type PD0_R = crate::BitReader;
#[doc = "Field `PD0` writer - "]
pub type PD0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD1` reader - "]
pub type PD1_R = crate::BitReader;
#[doc = "Field `PD1` writer - "]
pub type PD1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD2` reader - "]
pub type PD2_R = crate::BitReader;
#[doc = "Field `PD2` writer - "]
pub type PD2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD3` reader - "]
pub type PD3_R = crate::BitReader;
#[doc = "Field `PD3` writer - "]
pub type PD3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD4` reader - "]
pub type PD4_R = crate::BitReader;
#[doc = "Field `PD4` writer - "]
pub type PD4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD5` reader - "]
pub type PD5_R = crate::BitReader;
#[doc = "Field `PD5` writer - "]
pub type PD5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD6` reader - "]
pub type PD6_R = crate::BitReader;
#[doc = "Field `PD6` writer - "]
pub type PD6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD7` reader - "]
pub type PD7_R = crate::BitReader;
#[doc = "Field `PD7` writer - "]
pub type PD7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD8` reader - "]
pub type PD8_R = crate::BitReader;
#[doc = "Field `PD8` writer - "]
pub type PD8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD9` reader - "]
pub type PD9_R = crate::BitReader;
#[doc = "Field `PD9` writer - "]
pub type PD9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD10` reader - "]
pub type PD10_R = crate::BitReader;
#[doc = "Field `PD10` writer - "]
pub type PD10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD11` reader - "]
pub type PD11_R = crate::BitReader;
#[doc = "Field `PD11` writer - "]
pub type PD11_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pd0(&self) -> PD0_R {
        PD0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pd1(&self) -> PD1_R {
        PD1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pd2(&self) -> PD2_R {
        PD2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pd3(&self) -> PD3_R {
        PD3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pd4(&self) -> PD4_R {
        PD4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pd5(&self) -> PD5_R {
        PD5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn pd6(&self) -> PD6_R {
        PD6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn pd7(&self) -> PD7_R {
        PD7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pd8(&self) -> PD8_R {
        PD8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn pd9(&self) -> PD9_R {
        PD9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pd10(&self) -> PD10_R {
        PD10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pd11(&self) -> PD11_R {
        PD11_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn pd0(&mut self) -> PD0_W<PWR_PDCRJrs> {
        PD0_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn pd1(&mut self) -> PD1_W<PWR_PDCRJrs> {
        PD1_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn pd2(&mut self) -> PD2_W<PWR_PDCRJrs> {
        PD2_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn pd3(&mut self) -> PD3_W<PWR_PDCRJrs> {
        PD3_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn pd4(&mut self) -> PD4_W<PWR_PDCRJrs> {
        PD4_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn pd5(&mut self) -> PD5_W<PWR_PDCRJrs> {
        PD5_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn pd6(&mut self) -> PD6_W<PWR_PDCRJrs> {
        PD6_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn pd7(&mut self) -> PD7_W<PWR_PDCRJrs> {
        PD7_W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn pd8(&mut self) -> PD8_W<PWR_PDCRJrs> {
        PD8_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn pd9(&mut self) -> PD9_W<PWR_PDCRJrs> {
        PD9_W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn pd10(&mut self) -> PD10_W<PWR_PDCRJrs> {
        PD10_W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn pd11(&mut self) -> PD11_W<PWR_PDCRJrs> {
        PD11_W::new(self, 11)
    }
}
#[doc = "PWR port J pull-down control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_pdcrj::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_pdcrj::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PWR_PDCRJrs;
impl crate::RegisterSpec for PWR_PDCRJrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwr_pdcrj::R`](R) reader structure"]
impl crate::Readable for PWR_PDCRJrs {}
#[doc = "`write(|w| ..)` method takes [`pwr_pdcrj::W`](W) writer structure"]
impl crate::Writable for PWR_PDCRJrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWR_PDCRJ to value 0"]
impl crate::Resettable for PWR_PDCRJrs {
    const RESET_VALUE: u32 = 0;
}
