#[doc = "Register `SYSCFG_BOOTR` reader"]
pub type R = crate::R<SYSCFG_BOOTRrs>;
#[doc = "Register `SYSCFG_BOOTR` writer"]
pub type W = crate::W<SYSCFG_BOOTRrs>;
#[doc = "Field `BOOT0` reader - BOOT0"]
pub type BOOT0_R = crate::BitReader;
#[doc = "Field `BOOT1` reader - BOOT1"]
pub type BOOT1_R = crate::BitReader;
#[doc = "Field `BOOT2` reader - BOOT2"]
pub type BOOT2_R = crate::BitReader;
#[doc = "Field `BOOT0_PD` reader - BOOT0_PD"]
pub type BOOT0_PD_R = crate::BitReader;
#[doc = "Field `BOOT0_PD` writer - BOOT0_PD"]
pub type BOOT0_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOOT1_PD` reader - BOOT1_PD"]
pub type BOOT1_PD_R = crate::BitReader;
#[doc = "Field `BOOT1_PD` writer - BOOT1_PD"]
pub type BOOT1_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOOT2_PD` reader - BOOT2_PD"]
pub type BOOT2_PD_R = crate::BitReader;
#[doc = "Field `BOOT2_PD` writer - BOOT2_PD"]
pub type BOOT2_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - BOOT0"]
    #[inline(always)]
    pub fn boot0(&self) -> BOOT0_R {
        BOOT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BOOT1"]
    #[inline(always)]
    pub fn boot1(&self) -> BOOT1_R {
        BOOT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - BOOT2"]
    #[inline(always)]
    pub fn boot2(&self) -> BOOT2_R {
        BOOT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - BOOT0_PD"]
    #[inline(always)]
    pub fn boot0_pd(&self) -> BOOT0_PD_R {
        BOOT0_PD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - BOOT1_PD"]
    #[inline(always)]
    pub fn boot1_pd(&self) -> BOOT1_PD_R {
        BOOT1_PD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - BOOT2_PD"]
    #[inline(always)]
    pub fn boot2_pd(&self) -> BOOT2_PD_R {
        BOOT2_PD_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - BOOT0_PD"]
    #[inline(always)]
    #[must_use]
    pub fn boot0_pd(&mut self) -> BOOT0_PD_W<SYSCFG_BOOTRrs> {
        BOOT0_PD_W::new(self, 4)
    }
    #[doc = "Bit 5 - BOOT1_PD"]
    #[inline(always)]
    #[must_use]
    pub fn boot1_pd(&mut self) -> BOOT1_PD_W<SYSCFG_BOOTRrs> {
        BOOT1_PD_W::new(self, 5)
    }
    #[doc = "Bit 6 - BOOT2_PD"]
    #[inline(always)]
    #[must_use]
    pub fn boot2_pd(&mut self) -> BOOT2_PD_W<SYSCFG_BOOTRrs> {
        BOOT2_PD_W::new(self, 6)
    }
}
#[doc = "This register is used to know the state of BOOT pins and to control pull-up to reduce the static power consumption on the pin set to high level. )\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_bootr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syscfg_bootr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYSCFG_BOOTRrs;
impl crate::RegisterSpec for SYSCFG_BOOTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg_bootr::R`](R) reader structure"]
impl crate::Readable for SYSCFG_BOOTRrs {}
#[doc = "`write(|w| ..)` method takes [`syscfg_bootr::W`](W) writer structure"]
impl crate::Writable for SYSCFG_BOOTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYSCFG_BOOTR to value 0"]
impl crate::Resettable for SYSCFG_BOOTRrs {
    const RESET_VALUE: u32 = 0;
}
