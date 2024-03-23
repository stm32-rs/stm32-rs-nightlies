#[doc = "Register `CCR` reader"]
pub type R = crate::R<CCRrs>;
#[doc = "Register `CCR` writer"]
pub type W = crate::W<CCRrs>;
#[doc = "Field `PRESC0` reader - PRESC0"]
pub type PRESC0_R = crate::BitReader;
#[doc = "Field `PRESC0` writer - PRESC0"]
pub type PRESC0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESC1` reader - PRESC1"]
pub type PRESC1_R = crate::BitReader;
#[doc = "Field `PRESC1` writer - PRESC1"]
pub type PRESC1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESC2` reader - PRESC2"]
pub type PRESC2_R = crate::BitReader;
#[doc = "Field `PRESC2` writer - PRESC2"]
pub type PRESC2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESC3` reader - PRESC3"]
pub type PRESC3_R = crate::BitReader;
#[doc = "Field `PRESC3` writer - PRESC3"]
pub type PRESC3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VREFEN` reader - VREFEN"]
pub type VREFEN_R = crate::BitReader;
#[doc = "Field `VREFEN` writer - VREFEN"]
pub type VREFEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSEN` reader - TSEN"]
pub type TSEN_R = crate::BitReader;
#[doc = "Field `TSEN` writer - TSEN"]
pub type TSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBATEN` reader - VBATEN"]
pub type VBATEN_R = crate::BitReader;
#[doc = "Field `VBATEN` writer - VBATEN"]
pub type VBATEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 18 - PRESC0"]
    #[inline(always)]
    pub fn presc0(&self) -> PRESC0_R {
        PRESC0_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PRESC1"]
    #[inline(always)]
    pub fn presc1(&self) -> PRESC1_R {
        PRESC1_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - PRESC2"]
    #[inline(always)]
    pub fn presc2(&self) -> PRESC2_R {
        PRESC2_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - PRESC3"]
    #[inline(always)]
    pub fn presc3(&self) -> PRESC3_R {
        PRESC3_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - VREFEN"]
    #[inline(always)]
    pub fn vrefen(&self) -> VREFEN_R {
        VREFEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - TSEN"]
    #[inline(always)]
    pub fn tsen(&self) -> TSEN_R {
        TSEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - VBATEN"]
    #[inline(always)]
    pub fn vbaten(&self) -> VBATEN_R {
        VBATEN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 18 - PRESC0"]
    #[inline(always)]
    #[must_use]
    pub fn presc0(&mut self) -> PRESC0_W<CCRrs> {
        PRESC0_W::new(self, 18)
    }
    #[doc = "Bit 19 - PRESC1"]
    #[inline(always)]
    #[must_use]
    pub fn presc1(&mut self) -> PRESC1_W<CCRrs> {
        PRESC1_W::new(self, 19)
    }
    #[doc = "Bit 20 - PRESC2"]
    #[inline(always)]
    #[must_use]
    pub fn presc2(&mut self) -> PRESC2_W<CCRrs> {
        PRESC2_W::new(self, 20)
    }
    #[doc = "Bit 21 - PRESC3"]
    #[inline(always)]
    #[must_use]
    pub fn presc3(&mut self) -> PRESC3_W<CCRrs> {
        PRESC3_W::new(self, 21)
    }
    #[doc = "Bit 22 - VREFEN"]
    #[inline(always)]
    #[must_use]
    pub fn vrefen(&mut self) -> VREFEN_W<CCRrs> {
        VREFEN_W::new(self, 22)
    }
    #[doc = "Bit 23 - TSEN"]
    #[inline(always)]
    #[must_use]
    pub fn tsen(&mut self) -> TSEN_W<CCRrs> {
        TSEN_W::new(self, 23)
    }
    #[doc = "Bit 24 - VBATEN"]
    #[inline(always)]
    #[must_use]
    pub fn vbaten(&mut self) -> VBATEN_W<CCRrs> {
        VBATEN_W::new(self, 24)
    }
}
#[doc = "ADC common configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCRrs;
impl crate::RegisterSpec for CCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr::R`](R) reader structure"]
impl crate::Readable for CCRrs {}
#[doc = "`write(|w| ..)` method takes [`ccr::W`](W) writer structure"]
impl crate::Writable for CCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCR to value 0"]
impl crate::Resettable for CCRrs {
    const RESET_VALUE: u32 = 0;
}
