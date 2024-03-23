#[doc = "Register `RCC_MC_APB3ENCLRR` reader"]
pub type R = crate::R<RCC_MC_APB3ENCLRRrs>;
#[doc = "Register `RCC_MC_APB3ENCLRR` writer"]
pub type W = crate::W<RCC_MC_APB3ENCLRRrs>;
#[doc = "Field `LPTIM2EN` reader - LPTIM2EN"]
pub type LPTIM2EN_R = crate::BitReader;
#[doc = "Field `LPTIM2EN` writer - LPTIM2EN"]
pub type LPTIM2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM3EN` reader - LPTIM3EN"]
pub type LPTIM3EN_R = crate::BitReader;
#[doc = "Field `LPTIM3EN` writer - LPTIM3EN"]
pub type LPTIM3EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM4EN` reader - LPTIM4EN"]
pub type LPTIM4EN_R = crate::BitReader;
#[doc = "Field `LPTIM4EN` writer - LPTIM4EN"]
pub type LPTIM4EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM5EN` reader - LPTIM5EN"]
pub type LPTIM5EN_R = crate::BitReader;
#[doc = "Field `LPTIM5EN` writer - LPTIM5EN"]
pub type LPTIM5EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAI4EN` reader - SAI4EN"]
pub type SAI4EN_R = crate::BitReader;
#[doc = "Field `SAI4EN` writer - SAI4EN"]
pub type SAI4EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSCFGEN` reader - SYSCFGEN"]
pub type SYSCFGEN_R = crate::BitReader;
#[doc = "Field `SYSCFGEN` writer - SYSCFGEN"]
pub type SYSCFGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VREFEN` reader - VREFEN"]
pub type VREFEN_R = crate::BitReader;
#[doc = "Field `VREFEN` writer - VREFEN"]
pub type VREFEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTSEN` reader - DTSEN"]
pub type DTSEN_R = crate::BitReader;
#[doc = "Field `DTSEN` writer - DTSEN"]
pub type DTSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HDPEN` reader - HDPEN"]
pub type HDPEN_R = crate::BitReader;
#[doc = "Field `HDPEN` writer - HDPEN"]
pub type HDPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LPTIM2EN"]
    #[inline(always)]
    pub fn lptim2en(&self) -> LPTIM2EN_R {
        LPTIM2EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LPTIM3EN"]
    #[inline(always)]
    pub fn lptim3en(&self) -> LPTIM3EN_R {
        LPTIM3EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LPTIM4EN"]
    #[inline(always)]
    pub fn lptim4en(&self) -> LPTIM4EN_R {
        LPTIM4EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LPTIM5EN"]
    #[inline(always)]
    pub fn lptim5en(&self) -> LPTIM5EN_R {
        LPTIM5EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - SAI4EN"]
    #[inline(always)]
    pub fn sai4en(&self) -> SAI4EN_R {
        SAI4EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - SYSCFGEN"]
    #[inline(always)]
    pub fn syscfgen(&self) -> SYSCFGEN_R {
        SYSCFGEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - VREFEN"]
    #[inline(always)]
    pub fn vrefen(&self) -> VREFEN_R {
        VREFEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - DTSEN"]
    #[inline(always)]
    pub fn dtsen(&self) -> DTSEN_R {
        DTSEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - HDPEN"]
    #[inline(always)]
    pub fn hdpen(&self) -> HDPEN_R {
        HDPEN_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LPTIM2EN"]
    #[inline(always)]
    #[must_use]
    pub fn lptim2en(&mut self) -> LPTIM2EN_W<RCC_MC_APB3ENCLRRrs> {
        LPTIM2EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - LPTIM3EN"]
    #[inline(always)]
    #[must_use]
    pub fn lptim3en(&mut self) -> LPTIM3EN_W<RCC_MC_APB3ENCLRRrs> {
        LPTIM3EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - LPTIM4EN"]
    #[inline(always)]
    #[must_use]
    pub fn lptim4en(&mut self) -> LPTIM4EN_W<RCC_MC_APB3ENCLRRrs> {
        LPTIM4EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - LPTIM5EN"]
    #[inline(always)]
    #[must_use]
    pub fn lptim5en(&mut self) -> LPTIM5EN_W<RCC_MC_APB3ENCLRRrs> {
        LPTIM5EN_W::new(self, 3)
    }
    #[doc = "Bit 8 - SAI4EN"]
    #[inline(always)]
    #[must_use]
    pub fn sai4en(&mut self) -> SAI4EN_W<RCC_MC_APB3ENCLRRrs> {
        SAI4EN_W::new(self, 8)
    }
    #[doc = "Bit 11 - SYSCFGEN"]
    #[inline(always)]
    #[must_use]
    pub fn syscfgen(&mut self) -> SYSCFGEN_W<RCC_MC_APB3ENCLRRrs> {
        SYSCFGEN_W::new(self, 11)
    }
    #[doc = "Bit 13 - VREFEN"]
    #[inline(always)]
    #[must_use]
    pub fn vrefen(&mut self) -> VREFEN_W<RCC_MC_APB3ENCLRRrs> {
        VREFEN_W::new(self, 13)
    }
    #[doc = "Bit 16 - DTSEN"]
    #[inline(always)]
    #[must_use]
    pub fn dtsen(&mut self) -> DTSEN_W<RCC_MC_APB3ENCLRRrs> {
        DTSEN_W::new(self, 16)
    }
    #[doc = "Bit 20 - HDPEN"]
    #[inline(always)]
    #[must_use]
    pub fn hdpen(&mut self) -> HDPEN_W<RCC_MC_APB3ENCLRRrs> {
        HDPEN_W::new(self, 20)
    }
}
#[doc = "This register is used to clear the peripheral clock enable bit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mc_apb3enclrr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mc_apb3enclrr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_MC_APB3ENCLRRrs;
impl crate::RegisterSpec for RCC_MC_APB3ENCLRRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_mc_apb3enclrr::R`](R) reader structure"]
impl crate::Readable for RCC_MC_APB3ENCLRRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_mc_apb3enclrr::W`](W) writer structure"]
impl crate::Writable for RCC_MC_APB3ENCLRRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_MC_APB3ENCLRR to value 0"]
impl crate::Resettable for RCC_MC_APB3ENCLRRrs {
    const RESET_VALUE: u32 = 0;
}
