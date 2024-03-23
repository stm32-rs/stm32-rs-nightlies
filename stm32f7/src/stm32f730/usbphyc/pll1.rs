#[doc = "Register `PLL1` reader"]
pub type R = crate::R<PLL1rs>;
#[doc = "Register `PLL1` writer"]
pub type W = crate::W<PLL1rs>;
#[doc = "Field `PLL1EN` reader - Enable the PLL1 inside PHY"]
pub type PLL1EN_R = crate::BitReader;
#[doc = "Field `PLL1EN` writer - Enable the PLL1 inside PHY"]
pub type PLL1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLL1SEL` reader - : Controls the PHY PLL1 input clock frequency selection"]
pub type PLL1SEL_R = crate::FieldReader;
#[doc = "Field `PLL1SEL` writer - : Controls the PHY PLL1 input clock frequency selection"]
pub type PLL1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Enable the PLL1 inside PHY"]
    #[inline(always)]
    pub fn pll1en(&self) -> PLL1EN_R {
        PLL1EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - : Controls the PHY PLL1 input clock frequency selection"]
    #[inline(always)]
    pub fn pll1sel(&self) -> PLL1SEL_R {
        PLL1SEL_R::new(((self.bits >> 1) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable the PLL1 inside PHY"]
    #[inline(always)]
    #[must_use]
    pub fn pll1en(&mut self) -> PLL1EN_W<PLL1rs> {
        PLL1EN_W::new(self, 0)
    }
    #[doc = "Bits 1:3 - : Controls the PHY PLL1 input clock frequency selection"]
    #[inline(always)]
    #[must_use]
    pub fn pll1sel(&mut self) -> PLL1SEL_W<PLL1rs> {
        PLL1SEL_W::new(self, 1)
    }
}
#[doc = "USBPHYC PLL1 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLL1rs;
impl crate::RegisterSpec for PLL1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll1::R`](R) reader structure"]
impl crate::Readable for PLL1rs {}
#[doc = "`write(|w| ..)` method takes [`pll1::W`](W) writer structure"]
impl crate::Writable for PLL1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLL1 to value 0"]
impl crate::Resettable for PLL1rs {
    const RESET_VALUE: u32 = 0;
}
