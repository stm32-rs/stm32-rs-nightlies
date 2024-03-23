#[doc = "Register `RCC_MP_GRSTCSETR` reader"]
pub type R = crate::R<RCC_MP_GRSTCSETRrs>;
#[doc = "Register `RCC_MP_GRSTCSETR` writer"]
pub type W = crate::W<RCC_MP_GRSTCSETRrs>;
#[doc = "Field `MPSYSRST` reader - MPSYSRST"]
pub type MPSYSRST_R = crate::BitReader;
#[doc = "Field `MPSYSRST` writer - MPSYSRST"]
pub type MPSYSRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCURST` reader - MCURST"]
pub type MCURST_R = crate::BitReader;
#[doc = "Field `MCURST` writer - MCURST"]
pub type MCURST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPUP0RST` reader - MPUP0RST"]
pub type MPUP0RST_R = crate::BitReader;
#[doc = "Field `MPUP0RST` writer - MPUP0RST"]
pub type MPUP0RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPUP1RST` reader - MPUP1RST"]
pub type MPUP1RST_R = crate::BitReader;
#[doc = "Field `MPUP1RST` writer - MPUP1RST"]
pub type MPUP1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - MPSYSRST"]
    #[inline(always)]
    pub fn mpsysrst(&self) -> MPSYSRST_R {
        MPSYSRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MCURST"]
    #[inline(always)]
    pub fn mcurst(&self) -> MCURST_R {
        MCURST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - MPUP0RST"]
    #[inline(always)]
    pub fn mpup0rst(&self) -> MPUP0RST_R {
        MPUP0RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MPUP1RST"]
    #[inline(always)]
    pub fn mpup1rst(&self) -> MPUP1RST_R {
        MPUP1RST_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MPSYSRST"]
    #[inline(always)]
    #[must_use]
    pub fn mpsysrst(&mut self) -> MPSYSRST_W<RCC_MP_GRSTCSETRrs> {
        MPSYSRST_W::new(self, 0)
    }
    #[doc = "Bit 1 - MCURST"]
    #[inline(always)]
    #[must_use]
    pub fn mcurst(&mut self) -> MCURST_W<RCC_MP_GRSTCSETRrs> {
        MCURST_W::new(self, 1)
    }
    #[doc = "Bit 4 - MPUP0RST"]
    #[inline(always)]
    #[must_use]
    pub fn mpup0rst(&mut self) -> MPUP0RST_W<RCC_MP_GRSTCSETRrs> {
        MPUP0RST_W::new(self, 4)
    }
    #[doc = "Bit 5 - MPUP1RST"]
    #[inline(always)]
    #[must_use]
    pub fn mpup1rst(&mut self) -> MPUP1RST_W<RCC_MP_GRSTCSETRrs> {
        MPUP1RST_W::new(self, 5)
    }
}
#[doc = "This register is used by the MPU in order to generate either a MCU reset or a system reset or a reset of one of the two MPU processors. Writing has no effect, reading returns the effective values of the corresponding bits. Writing a activates the reset.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_grstcsetr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mp_grstcsetr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_MP_GRSTCSETRrs;
impl crate::RegisterSpec for RCC_MP_GRSTCSETRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_mp_grstcsetr::R`](R) reader structure"]
impl crate::Readable for RCC_MP_GRSTCSETRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_mp_grstcsetr::W`](W) writer structure"]
impl crate::Writable for RCC_MP_GRSTCSETRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_MP_GRSTCSETR to value 0"]
impl crate::Resettable for RCC_MP_GRSTCSETRrs {
    const RESET_VALUE: u32 = 0;
}
