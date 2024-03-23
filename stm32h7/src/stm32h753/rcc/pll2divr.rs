#[doc = "Register `PLL2DIVR` reader"]
pub type R = crate::R<PLL2DIVRrs>;
#[doc = "Register `PLL2DIVR` writer"]
pub type W = crate::W<PLL2DIVRrs>;
#[doc = "Field `DIVN2` reader - Multiplication factor for PLL1 VCO"]
pub type DIVN2_R = crate::FieldReader<u16>;
#[doc = "Field `DIVN2` writer - Multiplication factor for PLL1 VCO"]
pub type DIVN2_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `DIVP2` reader - PLL1 DIVP division factor"]
pub type DIVP2_R = crate::FieldReader;
#[doc = "Field `DIVP2` writer - PLL1 DIVP division factor"]
pub type DIVP2_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 7>;
#[doc = "Field `DIVQ2` reader - PLL1 DIVQ division factor"]
pub type DIVQ2_R = crate::FieldReader;
#[doc = "Field `DIVQ2` writer - PLL1 DIVQ division factor"]
pub type DIVQ2_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 7>;
#[doc = "Field `DIVR2` reader - PLL1 DIVR division factor"]
pub type DIVR2_R = crate::FieldReader;
#[doc = "Field `DIVR2` writer - PLL1 DIVR division factor"]
pub type DIVR2_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:8 - Multiplication factor for PLL1 VCO"]
    #[inline(always)]
    pub fn divn2(&self) -> DIVN2_R {
        DIVN2_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:15 - PLL1 DIVP division factor"]
    #[inline(always)]
    pub fn divp2(&self) -> DIVP2_R {
        DIVP2_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - PLL1 DIVQ division factor"]
    #[inline(always)]
    pub fn divq2(&self) -> DIVQ2_R {
        DIVQ2_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - PLL1 DIVR division factor"]
    #[inline(always)]
    pub fn divr2(&self) -> DIVR2_R {
        DIVR2_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8 - Multiplication factor for PLL1 VCO"]
    #[inline(always)]
    #[must_use]
    pub fn divn2(&mut self) -> DIVN2_W<PLL2DIVRrs> {
        DIVN2_W::new(self, 0)
    }
    #[doc = "Bits 9:15 - PLL1 DIVP division factor"]
    #[inline(always)]
    #[must_use]
    pub fn divp2(&mut self) -> DIVP2_W<PLL2DIVRrs> {
        DIVP2_W::new(self, 9)
    }
    #[doc = "Bits 16:22 - PLL1 DIVQ division factor"]
    #[inline(always)]
    #[must_use]
    pub fn divq2(&mut self) -> DIVQ2_W<PLL2DIVRrs> {
        DIVQ2_W::new(self, 16)
    }
    #[doc = "Bits 24:30 - PLL1 DIVR division factor"]
    #[inline(always)]
    #[must_use]
    pub fn divr2(&mut self) -> DIVR2_W<PLL2DIVRrs> {
        DIVR2_W::new(self, 24)
    }
}
#[doc = "RCC PLL2 Dividers Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll2divr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll2divr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLL2DIVRrs;
impl crate::RegisterSpec for PLL2DIVRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll2divr::R`](R) reader structure"]
impl crate::Readable for PLL2DIVRrs {}
#[doc = "`write(|w| ..)` method takes [`pll2divr::W`](W) writer structure"]
impl crate::Writable for PLL2DIVRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLL2DIVR to value 0x0101_0280"]
impl crate::Resettable for PLL2DIVRrs {
    const RESET_VALUE: u32 = 0x0101_0280;
}
