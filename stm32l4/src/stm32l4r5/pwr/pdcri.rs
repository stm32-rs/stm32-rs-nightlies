#[doc = "Register `PDCRI` reader"]
pub type R = crate::R<PDCRIrs>;
#[doc = "Register `PDCRI` writer"]
pub type W = crate::W<PDCRIrs>;
#[doc = "Field `PD0` reader - Port I pull-down bit 0"]
pub type PD0_R = crate::BitReader;
#[doc = "Field `PD0` writer - Port I pull-down bit 0"]
pub type PD0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD1` reader - Port I pull-down bit 1"]
pub type PD1_R = crate::BitReader;
#[doc = "Field `PD1` writer - Port I pull-down bit 1"]
pub type PD1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD2` reader - Port I pull-down bit 2"]
pub type PD2_R = crate::BitReader;
#[doc = "Field `PD2` writer - Port I pull-down bit 2"]
pub type PD2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD3` reader - Port I pull-down bit 3"]
pub type PD3_R = crate::BitReader;
#[doc = "Field `PD3` writer - Port I pull-down bit 3"]
pub type PD3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD4` reader - Port I pull-down bit 4"]
pub type PD4_R = crate::BitReader;
#[doc = "Field `PD4` writer - Port I pull-down bit 4"]
pub type PD4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD5` reader - Port I pull-down bit 5"]
pub type PD5_R = crate::BitReader;
#[doc = "Field `PD5` writer - Port I pull-down bit 5"]
pub type PD5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD6` reader - Port I pull-down bit 6"]
pub type PD6_R = crate::BitReader;
#[doc = "Field `PD6` writer - Port I pull-down bit 6"]
pub type PD6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD7` reader - Port I pull-down bit 7"]
pub type PD7_R = crate::BitReader;
#[doc = "Field `PD7` writer - Port I pull-down bit 7"]
pub type PD7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD8` reader - Port I pull-down bit 8"]
pub type PD8_R = crate::BitReader;
#[doc = "Field `PD8` writer - Port I pull-down bit 8"]
pub type PD8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD9` reader - Port I pull-down bit 9"]
pub type PD9_R = crate::BitReader;
#[doc = "Field `PD9` writer - Port I pull-down bit 9"]
pub type PD9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD10` reader - Port I pull-down bit 10"]
pub type PD10_R = crate::BitReader;
#[doc = "Field `PD10` writer - Port I pull-down bit 10"]
pub type PD10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD11` reader - Port I pull-down bit 11"]
pub type PD11_R = crate::BitReader;
#[doc = "Field `PD11` writer - Port I pull-down bit 11"]
pub type PD11_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Port I pull-down bit 0"]
    #[inline(always)]
    pub fn pd0(&self) -> PD0_R {
        PD0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port I pull-down bit 1"]
    #[inline(always)]
    pub fn pd1(&self) -> PD1_R {
        PD1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port I pull-down bit 2"]
    #[inline(always)]
    pub fn pd2(&self) -> PD2_R {
        PD2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port I pull-down bit 3"]
    #[inline(always)]
    pub fn pd3(&self) -> PD3_R {
        PD3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port I pull-down bit 4"]
    #[inline(always)]
    pub fn pd4(&self) -> PD4_R {
        PD4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port I pull-down bit 5"]
    #[inline(always)]
    pub fn pd5(&self) -> PD5_R {
        PD5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port I pull-down bit 6"]
    #[inline(always)]
    pub fn pd6(&self) -> PD6_R {
        PD6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port I pull-down bit 7"]
    #[inline(always)]
    pub fn pd7(&self) -> PD7_R {
        PD7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port I pull-down bit 8"]
    #[inline(always)]
    pub fn pd8(&self) -> PD8_R {
        PD8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port I pull-down bit 9"]
    #[inline(always)]
    pub fn pd9(&self) -> PD9_R {
        PD9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port I pull-down bit 10"]
    #[inline(always)]
    pub fn pd10(&self) -> PD10_R {
        PD10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port I pull-down bit 11"]
    #[inline(always)]
    pub fn pd11(&self) -> PD11_R {
        PD11_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port I pull-down bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn pd0(&mut self) -> PD0_W<PDCRIrs> {
        PD0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Port I pull-down bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn pd1(&mut self) -> PD1_W<PDCRIrs> {
        PD1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Port I pull-down bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn pd2(&mut self) -> PD2_W<PDCRIrs> {
        PD2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Port I pull-down bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn pd3(&mut self) -> PD3_W<PDCRIrs> {
        PD3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Port I pull-down bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn pd4(&mut self) -> PD4_W<PDCRIrs> {
        PD4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Port I pull-down bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn pd5(&mut self) -> PD5_W<PDCRIrs> {
        PD5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Port I pull-down bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn pd6(&mut self) -> PD6_W<PDCRIrs> {
        PD6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Port I pull-down bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn pd7(&mut self) -> PD7_W<PDCRIrs> {
        PD7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Port I pull-down bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn pd8(&mut self) -> PD8_W<PDCRIrs> {
        PD8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Port I pull-down bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn pd9(&mut self) -> PD9_W<PDCRIrs> {
        PD9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Port I pull-down bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn pd10(&mut self) -> PD10_W<PDCRIrs> {
        PD10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Port I pull-down bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn pd11(&mut self) -> PD11_W<PDCRIrs> {
        PD11_W::new(self, 11)
    }
}
#[doc = "Power Port I pull-down control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdcri::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdcri::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PDCRIrs;
impl crate::RegisterSpec for PDCRIrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdcri::R`](R) reader structure"]
impl crate::Readable for PDCRIrs {}
#[doc = "`write(|w| ..)` method takes [`pdcri::W`](W) writer structure"]
impl crate::Writable for PDCRIrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PDCRI to value 0"]
impl crate::Resettable for PDCRIrs {
    const RESET_VALUE: u32 = 0;
}
