#[doc = "Register `RCC_MC_MLAHBENCLRR` reader"]
pub type R = crate::R<RCC_MC_MLAHBENCLRRrs>;
#[doc = "Register `RCC_MC_MLAHBENCLRR` writer"]
pub type W = crate::W<RCC_MC_MLAHBENCLRRrs>;
#[doc = "Field `RETRAMEN` reader - RETRAMEN"]
pub type RETRAMEN_R = crate::BitReader;
#[doc = "Field `RETRAMEN` writer - RETRAMEN"]
pub type RETRAMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - RETRAMEN"]
    #[inline(always)]
    pub fn retramen(&self) -> RETRAMEN_R {
        RETRAMEN_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - RETRAMEN"]
    #[inline(always)]
    #[must_use]
    pub fn retramen(&mut self) -> RETRAMEN_W<RCC_MC_MLAHBENCLRRrs> {
        RETRAMEN_W::new(self, 4)
    }
}
#[doc = "This register is used to clear the peripheral clock enable bit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mc_mlahbenclrr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mc_mlahbenclrr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_MC_MLAHBENCLRRrs;
impl crate::RegisterSpec for RCC_MC_MLAHBENCLRRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_mc_mlahbenclrr::R`](R) reader structure"]
impl crate::Readable for RCC_MC_MLAHBENCLRRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_mc_mlahbenclrr::W`](W) writer structure"]
impl crate::Writable for RCC_MC_MLAHBENCLRRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_MC_MLAHBENCLRR to value 0x10"]
impl crate::Resettable for RCC_MC_MLAHBENCLRRrs {
    const RESET_VALUE: u32 = 0x10;
}
