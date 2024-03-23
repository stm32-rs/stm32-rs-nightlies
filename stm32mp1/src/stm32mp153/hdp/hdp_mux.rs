#[doc = "Register `HDP_MUX` reader"]
pub type R = crate::R<HDP_MUXrs>;
#[doc = "Register `HDP_MUX` writer"]
pub type W = crate::W<HDP_MUXrs>;
#[doc = "Field `MUX0` reader - MUX0"]
pub type MUX0_R = crate::FieldReader;
#[doc = "Field `MUX0` writer - MUX0"]
pub type MUX0_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MUX1` reader - MUX1"]
pub type MUX1_R = crate::FieldReader;
#[doc = "Field `MUX1` writer - MUX1"]
pub type MUX1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MUX2` reader - MUX2"]
pub type MUX2_R = crate::FieldReader;
#[doc = "Field `MUX2` writer - MUX2"]
pub type MUX2_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MUX3` reader - MUX3"]
pub type MUX3_R = crate::FieldReader;
#[doc = "Field `MUX3` writer - MUX3"]
pub type MUX3_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MUX4` reader - MUX4"]
pub type MUX4_R = crate::FieldReader;
#[doc = "Field `MUX4` writer - MUX4"]
pub type MUX4_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MUX5` reader - MUX5"]
pub type MUX5_R = crate::FieldReader;
#[doc = "Field `MUX5` writer - MUX5"]
pub type MUX5_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MUX6` reader - MUX6"]
pub type MUX6_R = crate::FieldReader;
#[doc = "Field `MUX6` writer - MUX6"]
pub type MUX6_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MUX7` reader - MUX7"]
pub type MUX7_R = crate::FieldReader;
#[doc = "Field `MUX7` writer - MUX7"]
pub type MUX7_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - MUX0"]
    #[inline(always)]
    pub fn mux0(&self) -> MUX0_R {
        MUX0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - MUX1"]
    #[inline(always)]
    pub fn mux1(&self) -> MUX1_R {
        MUX1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - MUX2"]
    #[inline(always)]
    pub fn mux2(&self) -> MUX2_R {
        MUX2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - MUX3"]
    #[inline(always)]
    pub fn mux3(&self) -> MUX3_R {
        MUX3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - MUX4"]
    #[inline(always)]
    pub fn mux4(&self) -> MUX4_R {
        MUX4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - MUX5"]
    #[inline(always)]
    pub fn mux5(&self) -> MUX5_R {
        MUX5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - MUX6"]
    #[inline(always)]
    pub fn mux6(&self) -> MUX6_R {
        MUX6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - MUX7"]
    #[inline(always)]
    pub fn mux7(&self) -> MUX7_R {
        MUX7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - MUX0"]
    #[inline(always)]
    #[must_use]
    pub fn mux0(&mut self) -> MUX0_W<HDP_MUXrs> {
        MUX0_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - MUX1"]
    #[inline(always)]
    #[must_use]
    pub fn mux1(&mut self) -> MUX1_W<HDP_MUXrs> {
        MUX1_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - MUX2"]
    #[inline(always)]
    #[must_use]
    pub fn mux2(&mut self) -> MUX2_W<HDP_MUXrs> {
        MUX2_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - MUX3"]
    #[inline(always)]
    #[must_use]
    pub fn mux3(&mut self) -> MUX3_W<HDP_MUXrs> {
        MUX3_W::new(self, 12)
    }
    #[doc = "Bits 16:19 - MUX4"]
    #[inline(always)]
    #[must_use]
    pub fn mux4(&mut self) -> MUX4_W<HDP_MUXrs> {
        MUX4_W::new(self, 16)
    }
    #[doc = "Bits 20:23 - MUX5"]
    #[inline(always)]
    #[must_use]
    pub fn mux5(&mut self) -> MUX5_W<HDP_MUXrs> {
        MUX5_W::new(self, 20)
    }
    #[doc = "Bits 24:27 - MUX6"]
    #[inline(always)]
    #[must_use]
    pub fn mux6(&mut self) -> MUX6_W<HDP_MUXrs> {
        MUX6_W::new(self, 24)
    }
    #[doc = "Bits 28:31 - MUX7"]
    #[inline(always)]
    #[must_use]
    pub fn mux7(&mut self) -> MUX7_W<HDP_MUXrs> {
        MUX7_W::new(self, 28)
    }
}
#[doc = "HDP multiplexing\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdp_mux::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdp_mux::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HDP_MUXrs;
impl crate::RegisterSpec for HDP_MUXrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hdp_mux::R`](R) reader structure"]
impl crate::Readable for HDP_MUXrs {}
#[doc = "`write(|w| ..)` method takes [`hdp_mux::W`](W) writer structure"]
impl crate::Writable for HDP_MUXrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HDP_MUX to value 0"]
impl crate::Resettable for HDP_MUXrs {
    const RESET_VALUE: u32 = 0;
}
