///Register `CIER` reader
pub type R = crate::R<CIERrs>;
///Register `CIER` writer
pub type W = crate::W<CIERrs>;
/**LSI ready interrupt flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSIRDYIE {
    ///0: Ready interrupt disabled
    Disabled = 0,
    ///1: Ready interrupt enabled
    Enabled = 1,
}
impl From<LSIRDYIE> for bool {
    #[inline(always)]
    fn from(variant: LSIRDYIE) -> Self {
        variant as u8 != 0
    }
}
///Field `LSIRDYIE` reader - LSI ready interrupt flag
pub type LSIRDYIE_R = crate::BitReader<LSIRDYIE>;
impl LSIRDYIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LSIRDYIE {
        match self.bits {
            false => LSIRDYIE::Disabled,
            true => LSIRDYIE::Enabled,
        }
    }
    ///Ready interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LSIRDYIE::Disabled
    }
    ///Ready interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LSIRDYIE::Enabled
    }
}
///Field `LSIRDYIE` writer - LSI ready interrupt flag
pub type LSIRDYIE_W<'a, REG> = crate::BitWriter<'a, REG, LSIRDYIE>;
impl<'a, REG> LSIRDYIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Ready interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LSIRDYIE::Disabled)
    }
    ///Ready interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LSIRDYIE::Enabled)
    }
}
///Field `LSERDYIE` reader - LSE ready interrupt flag
pub use LSIRDYIE_R as LSERDYIE_R;
///Field `HSI16RDYIE` reader - HSI16 ready interrupt flag
pub use LSIRDYIE_R as HSI16RDYIE_R;
///Field `HSERDYIE` reader - HSE ready interrupt flag
pub use LSIRDYIE_R as HSERDYIE_R;
///Field `PLLRDYIE` reader - PLL ready interrupt flag
pub use LSIRDYIE_R as PLLRDYIE_R;
///Field `MSIRDYIE` reader - MSI ready interrupt flag
pub use LSIRDYIE_R as MSIRDYIE_R;
///Field `LSERDYIE` writer - LSE ready interrupt flag
pub use LSIRDYIE_W as LSERDYIE_W;
///Field `HSI16RDYIE` writer - HSI16 ready interrupt flag
pub use LSIRDYIE_W as HSI16RDYIE_W;
///Field `HSERDYIE` writer - HSE ready interrupt flag
pub use LSIRDYIE_W as HSERDYIE_W;
///Field `PLLRDYIE` writer - PLL ready interrupt flag
pub use LSIRDYIE_W as PLLRDYIE_W;
///Field `MSIRDYIE` writer - MSI ready interrupt flag
pub use LSIRDYIE_W as MSIRDYIE_W;
/**LSE CSS interrupt flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSSLSE {
    ///0: LSE CSS interrupt disabled
    Disabled = 0,
    ///1: LSE CSS interrupt enabled
    Enabled = 1,
}
impl From<CSSLSE> for bool {
    #[inline(always)]
    fn from(variant: CSSLSE) -> Self {
        variant as u8 != 0
    }
}
///Field `CSSLSE` reader - LSE CSS interrupt flag
pub type CSSLSE_R = crate::BitReader<CSSLSE>;
impl CSSLSE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSSLSE {
        match self.bits {
            false => CSSLSE::Disabled,
            true => CSSLSE::Enabled,
        }
    }
    ///LSE CSS interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CSSLSE::Disabled
    }
    ///LSE CSS interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CSSLSE::Enabled
    }
}
///Field `CSSLSE` writer - LSE CSS interrupt flag
pub type CSSLSE_W<'a, REG> = crate::BitWriter<'a, REG, CSSLSE>;
impl<'a, REG> CSSLSE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///LSE CSS interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CSSLSE::Disabled)
    }
    ///LSE CSS interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CSSLSE::Enabled)
    }
}
impl R {
    ///Bit 0 - LSI ready interrupt flag
    #[inline(always)]
    pub fn lsirdyie(&self) -> LSIRDYIE_R {
        LSIRDYIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LSE ready interrupt flag
    #[inline(always)]
    pub fn lserdyie(&self) -> LSERDYIE_R {
        LSERDYIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - HSI16 ready interrupt flag
    #[inline(always)]
    pub fn hsi16rdyie(&self) -> HSI16RDYIE_R {
        HSI16RDYIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - HSE ready interrupt flag
    #[inline(always)]
    pub fn hserdyie(&self) -> HSERDYIE_R {
        HSERDYIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - PLL ready interrupt flag
    #[inline(always)]
    pub fn pllrdyie(&self) -> PLLRDYIE_R {
        PLLRDYIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - MSI ready interrupt flag
    #[inline(always)]
    pub fn msirdyie(&self) -> MSIRDYIE_R {
        MSIRDYIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 7 - LSE CSS interrupt flag
    #[inline(always)]
    pub fn csslse(&self) -> CSSLSE_R {
        CSSLSE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CIER")
            .field("csslse", &self.csslse())
            .field("lsirdyie", &self.lsirdyie())
            .field("msirdyie", &self.msirdyie())
            .field("pllrdyie", &self.pllrdyie())
            .field("hserdyie", &self.hserdyie())
            .field("hsi16rdyie", &self.hsi16rdyie())
            .field("lserdyie", &self.lserdyie())
            .finish()
    }
}
impl W {
    ///Bit 0 - LSI ready interrupt flag
    #[inline(always)]
    pub fn lsirdyie(&mut self) -> LSIRDYIE_W<'_, CIERrs> {
        LSIRDYIE_W::new(self, 0)
    }
    ///Bit 1 - LSE ready interrupt flag
    #[inline(always)]
    pub fn lserdyie(&mut self) -> LSERDYIE_W<'_, CIERrs> {
        LSERDYIE_W::new(self, 1)
    }
    ///Bit 2 - HSI16 ready interrupt flag
    #[inline(always)]
    pub fn hsi16rdyie(&mut self) -> HSI16RDYIE_W<'_, CIERrs> {
        HSI16RDYIE_W::new(self, 2)
    }
    ///Bit 3 - HSE ready interrupt flag
    #[inline(always)]
    pub fn hserdyie(&mut self) -> HSERDYIE_W<'_, CIERrs> {
        HSERDYIE_W::new(self, 3)
    }
    ///Bit 4 - PLL ready interrupt flag
    #[inline(always)]
    pub fn pllrdyie(&mut self) -> PLLRDYIE_W<'_, CIERrs> {
        PLLRDYIE_W::new(self, 4)
    }
    ///Bit 5 - MSI ready interrupt flag
    #[inline(always)]
    pub fn msirdyie(&mut self) -> MSIRDYIE_W<'_, CIERrs> {
        MSIRDYIE_W::new(self, 5)
    }
    ///Bit 7 - LSE CSS interrupt flag
    #[inline(always)]
    pub fn csslse(&mut self) -> CSSLSE_W<'_, CIERrs> {
        CSSLSE_W::new(self, 7)
    }
}
/**Clock interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`cier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x1.html#RCC:CIER)*/
pub struct CIERrs;
impl crate::RegisterSpec for CIERrs {
    type Ux = u32;
}
///`read()` method returns [`cier::R`](R) reader structure
impl crate::Readable for CIERrs {}
///`write(|w| ..)` method takes [`cier::W`](W) writer structure
impl crate::Writable for CIERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CIER to value 0
impl crate::Resettable for CIERrs {}
