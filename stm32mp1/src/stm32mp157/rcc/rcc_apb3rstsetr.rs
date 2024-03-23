#[doc = "Register `RCC_APB3RSTSETR` reader"]
pub type R = crate::R<RCC_APB3RSTSETRrs>;
#[doc = "Register `RCC_APB3RSTSETR` writer"]
pub type W = crate::W<RCC_APB3RSTSETRrs>;
#[doc = "Field `LPTIM2RST` reader - LPTIM2RST"]
pub type LPTIM2RST_R = crate::BitReader;
#[doc = "Field `LPTIM2RST` writer - LPTIM2RST"]
pub type LPTIM2RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM3RST` reader - LPTIM3RST"]
pub type LPTIM3RST_R = crate::BitReader;
#[doc = "Field `LPTIM3RST` writer - LPTIM3RST"]
pub type LPTIM3RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM4RST` reader - LPTIM4RST"]
pub type LPTIM4RST_R = crate::BitReader;
#[doc = "Field `LPTIM4RST` writer - LPTIM4RST"]
pub type LPTIM4RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM5RST` reader - LPTIM5RST"]
pub type LPTIM5RST_R = crate::BitReader;
#[doc = "Field `LPTIM5RST` writer - LPTIM5RST"]
pub type LPTIM5RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAI4RST` reader - SAI4RST"]
pub type SAI4RST_R = crate::BitReader;
#[doc = "Field `SAI4RST` writer - SAI4RST"]
pub type SAI4RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSCFGRST` reader - SYSCFGRST"]
pub type SYSCFGRST_R = crate::BitReader;
#[doc = "Field `SYSCFGRST` writer - SYSCFGRST"]
pub type SYSCFGRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VREFRST` reader - VREFRST"]
pub type VREFRST_R = crate::BitReader;
#[doc = "Field `VREFRST` writer - VREFRST"]
pub type VREFRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTSRST` reader - DTSRST"]
pub type DTSRST_R = crate::BitReader;
#[doc = "Field `DTSRST` writer - DTSRST"]
pub type DTSRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LPTIM2RST"]
    #[inline(always)]
    pub fn lptim2rst(&self) -> LPTIM2RST_R {
        LPTIM2RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LPTIM3RST"]
    #[inline(always)]
    pub fn lptim3rst(&self) -> LPTIM3RST_R {
        LPTIM3RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LPTIM4RST"]
    #[inline(always)]
    pub fn lptim4rst(&self) -> LPTIM4RST_R {
        LPTIM4RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LPTIM5RST"]
    #[inline(always)]
    pub fn lptim5rst(&self) -> LPTIM5RST_R {
        LPTIM5RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - SAI4RST"]
    #[inline(always)]
    pub fn sai4rst(&self) -> SAI4RST_R {
        SAI4RST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - SYSCFGRST"]
    #[inline(always)]
    pub fn syscfgrst(&self) -> SYSCFGRST_R {
        SYSCFGRST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - VREFRST"]
    #[inline(always)]
    pub fn vrefrst(&self) -> VREFRST_R {
        VREFRST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - DTSRST"]
    #[inline(always)]
    pub fn dtsrst(&self) -> DTSRST_R {
        DTSRST_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LPTIM2RST"]
    #[inline(always)]
    #[must_use]
    pub fn lptim2rst(&mut self) -> LPTIM2RST_W<RCC_APB3RSTSETRrs> {
        LPTIM2RST_W::new(self, 0)
    }
    #[doc = "Bit 1 - LPTIM3RST"]
    #[inline(always)]
    #[must_use]
    pub fn lptim3rst(&mut self) -> LPTIM3RST_W<RCC_APB3RSTSETRrs> {
        LPTIM3RST_W::new(self, 1)
    }
    #[doc = "Bit 2 - LPTIM4RST"]
    #[inline(always)]
    #[must_use]
    pub fn lptim4rst(&mut self) -> LPTIM4RST_W<RCC_APB3RSTSETRrs> {
        LPTIM4RST_W::new(self, 2)
    }
    #[doc = "Bit 3 - LPTIM5RST"]
    #[inline(always)]
    #[must_use]
    pub fn lptim5rst(&mut self) -> LPTIM5RST_W<RCC_APB3RSTSETRrs> {
        LPTIM5RST_W::new(self, 3)
    }
    #[doc = "Bit 8 - SAI4RST"]
    #[inline(always)]
    #[must_use]
    pub fn sai4rst(&mut self) -> SAI4RST_W<RCC_APB3RSTSETRrs> {
        SAI4RST_W::new(self, 8)
    }
    #[doc = "Bit 11 - SYSCFGRST"]
    #[inline(always)]
    #[must_use]
    pub fn syscfgrst(&mut self) -> SYSCFGRST_W<RCC_APB3RSTSETRrs> {
        SYSCFGRST_W::new(self, 11)
    }
    #[doc = "Bit 13 - VREFRST"]
    #[inline(always)]
    #[must_use]
    pub fn vrefrst(&mut self) -> VREFRST_W<RCC_APB3RSTSETRrs> {
        VREFRST_W::new(self, 13)
    }
    #[doc = "Bit 16 - DTSRST"]
    #[inline(always)]
    #[must_use]
    pub fn dtsrst(&mut self) -> DTSRST_W<RCC_APB3RSTSETRrs> {
        DTSRST_W::new(self, 16)
    }
}
#[doc = "This register is used to activate the reset of the corresponding peripheral.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_apb3rstsetr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_apb3rstsetr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_APB3RSTSETRrs;
impl crate::RegisterSpec for RCC_APB3RSTSETRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_apb3rstsetr::R`](R) reader structure"]
impl crate::Readable for RCC_APB3RSTSETRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_apb3rstsetr::W`](W) writer structure"]
impl crate::Writable for RCC_APB3RSTSETRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_APB3RSTSETR to value 0"]
impl crate::Resettable for RCC_APB3RSTSETRrs {
    const RESET_VALUE: u32 = 0;
}
