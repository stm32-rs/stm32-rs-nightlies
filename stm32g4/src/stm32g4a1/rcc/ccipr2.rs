#[doc = "Register `CCIPR2` reader"]
pub type R = crate::R<CCIPR2rs>;
#[doc = "Register `CCIPR2` writer"]
pub type W = crate::W<CCIPR2rs>;
#[doc = "Field `I2C4SEL` reader - I2C4 clock source selection"]
pub type I2C4SEL_R = crate::FieldReader;
#[doc = "Field `I2C4SEL` writer - I2C4 clock source selection"]
pub type I2C4SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `QSPISEL` reader - Octospi clock source selection"]
pub type QSPISEL_R = crate::FieldReader;
#[doc = "Field `QSPISEL` writer - Octospi clock source selection"]
pub type QSPISEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - I2C4 clock source selection"]
    #[inline(always)]
    pub fn i2c4sel(&self) -> I2C4SEL_R {
        I2C4SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 20:21 - Octospi clock source selection"]
    #[inline(always)]
    pub fn qspisel(&self) -> QSPISEL_R {
        QSPISEL_R::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - I2C4 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn i2c4sel(&mut self) -> I2C4SEL_W<CCIPR2rs> {
        I2C4SEL_W::new(self, 0)
    }
    #[doc = "Bits 20:21 - Octospi clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn qspisel(&mut self) -> QSPISEL_W<CCIPR2rs> {
        QSPISEL_W::new(self, 20)
    }
}
#[doc = "Peripherals independent clock configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccipr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccipr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCIPR2rs;
impl crate::RegisterSpec for CCIPR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccipr2::R`](R) reader structure"]
impl crate::Readable for CCIPR2rs {}
#[doc = "`write(|w| ..)` method takes [`ccipr2::W`](W) writer structure"]
impl crate::Writable for CCIPR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCIPR2 to value 0"]
impl crate::Resettable for CCIPR2rs {
    const RESET_VALUE: u32 = 0;
}
