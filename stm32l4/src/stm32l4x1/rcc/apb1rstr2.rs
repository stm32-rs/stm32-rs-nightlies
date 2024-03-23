#[doc = "Register `APB1RSTR2` reader"]
pub type R = crate::R<APB1RSTR2rs>;
#[doc = "Register `APB1RSTR2` writer"]
pub type W = crate::W<APB1RSTR2rs>;
#[doc = "Field `LPUART1RST` reader - Low-power UART 1 reset"]
pub type LPUART1RST_R = crate::BitReader;
#[doc = "Field `LPUART1RST` writer - Low-power UART 1 reset"]
pub type LPUART1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C4RST` reader - I2C4 reset"]
pub type I2C4RST_R = crate::BitReader;
#[doc = "Field `I2C4RST` writer - I2C4 reset"]
pub type I2C4RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWPMI1RST` reader - Single wire protocol reset"]
pub type SWPMI1RST_R = crate::BitReader;
#[doc = "Field `SWPMI1RST` writer - Single wire protocol reset"]
pub type SWPMI1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM2RST` reader - Low-power timer 2 reset"]
pub type LPTIM2RST_R = crate::BitReader;
#[doc = "Field `LPTIM2RST` writer - Low-power timer 2 reset"]
pub type LPTIM2RST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Low-power UART 1 reset"]
    #[inline(always)]
    pub fn lpuart1rst(&self) -> LPUART1RST_R {
        LPUART1RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C4 reset"]
    #[inline(always)]
    pub fn i2c4rst(&self) -> I2C4RST_R {
        I2C4RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Single wire protocol reset"]
    #[inline(always)]
    pub fn swpmi1rst(&self) -> SWPMI1RST_R {
        SWPMI1RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Low-power timer 2 reset"]
    #[inline(always)]
    pub fn lptim2rst(&self) -> LPTIM2RST_R {
        LPTIM2RST_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Low-power UART 1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1rst(&mut self) -> LPUART1RST_W<APB1RSTR2rs> {
        LPUART1RST_W::new(self, 0)
    }
    #[doc = "Bit 1 - I2C4 reset"]
    #[inline(always)]
    #[must_use]
    pub fn i2c4rst(&mut self) -> I2C4RST_W<APB1RSTR2rs> {
        I2C4RST_W::new(self, 1)
    }
    #[doc = "Bit 2 - Single wire protocol reset"]
    #[inline(always)]
    #[must_use]
    pub fn swpmi1rst(&mut self) -> SWPMI1RST_W<APB1RSTR2rs> {
        SWPMI1RST_W::new(self, 2)
    }
    #[doc = "Bit 5 - Low-power timer 2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn lptim2rst(&mut self) -> LPTIM2RST_W<APB1RSTR2rs> {
        LPTIM2RST_W::new(self, 5)
    }
}
#[doc = "APB1 peripheral reset register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1rstr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1rstr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB1RSTR2rs;
impl crate::RegisterSpec for APB1RSTR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1rstr2::R`](R) reader structure"]
impl crate::Readable for APB1RSTR2rs {}
#[doc = "`write(|w| ..)` method takes [`apb1rstr2::W`](W) writer structure"]
impl crate::Writable for APB1RSTR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB1RSTR2 to value 0"]
impl crate::Resettable for APB1RSTR2rs {
    const RESET_VALUE: u32 = 0;
}
