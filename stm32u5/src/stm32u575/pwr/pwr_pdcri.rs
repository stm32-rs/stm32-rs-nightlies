#[doc = "Register `PWR_PDCRI` reader"]
pub type R = crate::R<PWR_PDCRIrs>;
#[doc = "Register `PWR_PDCRI` writer"]
pub type W = crate::W<PWR_PDCRIrs>;
#[doc = "Field `PD0` reader - Port I pull-down bit"]
pub type PD0_R = crate::BitReader;
#[doc = "Field `PD0` writer - Port I pull-down bit"]
pub type PD0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD1` reader - Port I pull-down bit"]
pub type PD1_R = crate::BitReader;
#[doc = "Field `PD1` writer - Port I pull-down bit"]
pub type PD1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD2` reader - Port I pull-down bit"]
pub type PD2_R = crate::BitReader;
#[doc = "Field `PD2` writer - Port I pull-down bit"]
pub type PD2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD3` reader - Port I pull-down bit"]
pub type PD3_R = crate::BitReader;
#[doc = "Field `PD3` writer - Port I pull-down bit"]
pub type PD3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD4` reader - Port I pull-down bit"]
pub type PD4_R = crate::BitReader;
#[doc = "Field `PD4` writer - Port I pull-down bit"]
pub type PD4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD5` reader - Port I pull-down bit"]
pub type PD5_R = crate::BitReader;
#[doc = "Field `PD5` writer - Port I pull-down bit"]
pub type PD5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD6` reader - Port I pull-down bit"]
pub type PD6_R = crate::BitReader;
#[doc = "Field `PD6` writer - Port I pull-down bit"]
pub type PD6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD7` reader - Port I pull-down bit"]
pub type PD7_R = crate::BitReader;
#[doc = "Field `PD7` writer - Port I pull-down bit"]
pub type PD7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Port I pull-down bit"]
    #[inline(always)]
    pub fn pd0(&self) -> PD0_R {
        PD0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port I pull-down bit"]
    #[inline(always)]
    pub fn pd1(&self) -> PD1_R {
        PD1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port I pull-down bit"]
    #[inline(always)]
    pub fn pd2(&self) -> PD2_R {
        PD2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port I pull-down bit"]
    #[inline(always)]
    pub fn pd3(&self) -> PD3_R {
        PD3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port I pull-down bit"]
    #[inline(always)]
    pub fn pd4(&self) -> PD4_R {
        PD4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port I pull-down bit"]
    #[inline(always)]
    pub fn pd5(&self) -> PD5_R {
        PD5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port I pull-down bit"]
    #[inline(always)]
    pub fn pd6(&self) -> PD6_R {
        PD6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port I pull-down bit"]
    #[inline(always)]
    pub fn pd7(&self) -> PD7_R {
        PD7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port I pull-down bit"]
    #[inline(always)]
    #[must_use]
    pub fn pd0(&mut self) -> PD0_W<PWR_PDCRIrs> {
        PD0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Port I pull-down bit"]
    #[inline(always)]
    #[must_use]
    pub fn pd1(&mut self) -> PD1_W<PWR_PDCRIrs> {
        PD1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Port I pull-down bit"]
    #[inline(always)]
    #[must_use]
    pub fn pd2(&mut self) -> PD2_W<PWR_PDCRIrs> {
        PD2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Port I pull-down bit"]
    #[inline(always)]
    #[must_use]
    pub fn pd3(&mut self) -> PD3_W<PWR_PDCRIrs> {
        PD3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Port I pull-down bit"]
    #[inline(always)]
    #[must_use]
    pub fn pd4(&mut self) -> PD4_W<PWR_PDCRIrs> {
        PD4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Port I pull-down bit"]
    #[inline(always)]
    #[must_use]
    pub fn pd5(&mut self) -> PD5_W<PWR_PDCRIrs> {
        PD5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Port I pull-down bit"]
    #[inline(always)]
    #[must_use]
    pub fn pd6(&mut self) -> PD6_W<PWR_PDCRIrs> {
        PD6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Port I pull-down bit"]
    #[inline(always)]
    #[must_use]
    pub fn pd7(&mut self) -> PD7_W<PWR_PDCRIrs> {
        PD7_W::new(self, 7)
    }
}
#[doc = "PWR port I pull-down control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_pdcri::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_pdcri::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PWR_PDCRIrs;
impl crate::RegisterSpec for PWR_PDCRIrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwr_pdcri::R`](R) reader structure"]
impl crate::Readable for PWR_PDCRIrs {}
#[doc = "`write(|w| ..)` method takes [`pwr_pdcri::W`](W) writer structure"]
impl crate::Writable for PWR_PDCRIrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWR_PDCRI to value 0"]
impl crate::Resettable for PWR_PDCRIrs {
    const RESET_VALUE: u32 = 0;
}
