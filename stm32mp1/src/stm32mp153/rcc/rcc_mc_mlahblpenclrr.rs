#[doc = "Register `RCC_MC_MLAHBLPENCLRR` reader"]
pub type R = crate::R<RCC_MC_MLAHBLPENCLRRrs>;
#[doc = "Register `RCC_MC_MLAHBLPENCLRR` writer"]
pub type W = crate::W<RCC_MC_MLAHBLPENCLRRrs>;
#[doc = "Field `SRAM1LPEN` reader - SRAM1LPEN"]
pub type SRAM1LPEN_R = crate::BitReader;
#[doc = "Field `SRAM1LPEN` writer - SRAM1LPEN"]
pub type SRAM1LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM2LPEN` reader - SRAM2LPEN"]
pub type SRAM2LPEN_R = crate::BitReader;
#[doc = "Field `SRAM2LPEN` writer - SRAM2LPEN"]
pub type SRAM2LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM34LPEN` reader - SRAM34LPEN"]
pub type SRAM34LPEN_R = crate::BitReader;
#[doc = "Field `SRAM34LPEN` writer - SRAM34LPEN"]
pub type SRAM34LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RETRAMLPEN` reader - RETRAMLPEN"]
pub type RETRAMLPEN_R = crate::BitReader;
#[doc = "Field `RETRAMLPEN` writer - RETRAMLPEN"]
pub type RETRAMLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SRAM1LPEN"]
    #[inline(always)]
    pub fn sram1lpen(&self) -> SRAM1LPEN_R {
        SRAM1LPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SRAM2LPEN"]
    #[inline(always)]
    pub fn sram2lpen(&self) -> SRAM2LPEN_R {
        SRAM2LPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SRAM34LPEN"]
    #[inline(always)]
    pub fn sram34lpen(&self) -> SRAM34LPEN_R {
        SRAM34LPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - RETRAMLPEN"]
    #[inline(always)]
    pub fn retramlpen(&self) -> RETRAMLPEN_R {
        RETRAMLPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SRAM1LPEN"]
    #[inline(always)]
    #[must_use]
    pub fn sram1lpen(&mut self) -> SRAM1LPEN_W<RCC_MC_MLAHBLPENCLRRrs> {
        SRAM1LPEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - SRAM2LPEN"]
    #[inline(always)]
    #[must_use]
    pub fn sram2lpen(&mut self) -> SRAM2LPEN_W<RCC_MC_MLAHBLPENCLRRrs> {
        SRAM2LPEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - SRAM34LPEN"]
    #[inline(always)]
    #[must_use]
    pub fn sram34lpen(&mut self) -> SRAM34LPEN_W<RCC_MC_MLAHBLPENCLRRrs> {
        SRAM34LPEN_W::new(self, 2)
    }
    #[doc = "Bit 4 - RETRAMLPEN"]
    #[inline(always)]
    #[must_use]
    pub fn retramlpen(&mut self) -> RETRAMLPEN_W<RCC_MC_MLAHBLPENCLRRrs> {
        RETRAMLPEN_W::new(self, 4)
    }
}
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bit of the corresponding peripheral.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mc_mlahblpenclrr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mc_mlahblpenclrr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_MC_MLAHBLPENCLRRrs;
impl crate::RegisterSpec for RCC_MC_MLAHBLPENCLRRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_mc_mlahblpenclrr::R`](R) reader structure"]
impl crate::Readable for RCC_MC_MLAHBLPENCLRRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_mc_mlahblpenclrr::W`](W) writer structure"]
impl crate::Writable for RCC_MC_MLAHBLPENCLRRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_MC_MLAHBLPENCLRR to value 0x17"]
impl crate::Resettable for RCC_MC_MLAHBLPENCLRRrs {
    const RESET_VALUE: u32 = 0x17;
}
