///Register `CIER` reader
pub type R = crate::R<CIERrs>;
///Register `CIER` writer
pub type W = crate::W<CIERrs>;
/**LSI ready interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSIRDYIE {
    ///0: Interrupt disabled
    Disabled = 0,
    ///1: Interrupt enabled
    Enabled = 1,
}
impl From<LSIRDYIE> for bool {
    #[inline(always)]
    fn from(variant: LSIRDYIE) -> Self {
        variant as u8 != 0
    }
}
///Field `LSIRDYIE` reader - LSI ready interrupt enable
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
    ///Interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LSIRDYIE::Disabled
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LSIRDYIE::Enabled
    }
}
///Field `LSIRDYIE` writer - LSI ready interrupt enable
pub type LSIRDYIE_W<'a, REG> = crate::BitWriter<'a, REG, LSIRDYIE>;
impl<'a, REG> LSIRDYIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LSIRDYIE::Disabled)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LSIRDYIE::Enabled)
    }
}
///Field `LSERDYIE` reader - LSE ready interrupt enable
pub use LSIRDYIE_R as LSERDYIE_R;
///Field `MSIRDYIE` reader - MSI ready interrupt enable
pub use LSIRDYIE_R as MSIRDYIE_R;
///Field `HSIRDYIE` reader - HSI16 ready interrupt enable
pub use LSIRDYIE_R as HSIRDYIE_R;
///Field `HSERDYIE` reader - HSE32 ready interrupt enable
pub use LSIRDYIE_R as HSERDYIE_R;
///Field `PLLRDYIE` reader - PLL ready interrupt enable
pub use LSIRDYIE_R as PLLRDYIE_R;
///Field `LSECSSIE` reader - LSE clock security system interrupt enable
pub use LSIRDYIE_R as LSECSSIE_R;
///Field `LSERDYIE` writer - LSE ready interrupt enable
pub use LSIRDYIE_W as LSERDYIE_W;
///Field `MSIRDYIE` writer - MSI ready interrupt enable
pub use LSIRDYIE_W as MSIRDYIE_W;
///Field `HSIRDYIE` writer - HSI16 ready interrupt enable
pub use LSIRDYIE_W as HSIRDYIE_W;
///Field `HSERDYIE` writer - HSE32 ready interrupt enable
pub use LSIRDYIE_W as HSERDYIE_W;
///Field `PLLRDYIE` writer - PLL ready interrupt enable
pub use LSIRDYIE_W as PLLRDYIE_W;
///Field `LSECSSIE` writer - LSE clock security system interrupt enable
pub use LSIRDYIE_W as LSECSSIE_W;
impl R {
    ///Bit 0 - LSI ready interrupt enable
    #[inline(always)]
    pub fn lsirdyie(&self) -> LSIRDYIE_R {
        LSIRDYIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LSE ready interrupt enable
    #[inline(always)]
    pub fn lserdyie(&self) -> LSERDYIE_R {
        LSERDYIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - MSI ready interrupt enable
    #[inline(always)]
    pub fn msirdyie(&self) -> MSIRDYIE_R {
        MSIRDYIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - HSI16 ready interrupt enable
    #[inline(always)]
    pub fn hsirdyie(&self) -> HSIRDYIE_R {
        HSIRDYIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - HSE32 ready interrupt enable
    #[inline(always)]
    pub fn hserdyie(&self) -> HSERDYIE_R {
        HSERDYIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - PLL ready interrupt enable
    #[inline(always)]
    pub fn pllrdyie(&self) -> PLLRDYIE_R {
        PLLRDYIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 9 - LSE clock security system interrupt enable
    #[inline(always)]
    pub fn lsecssie(&self) -> LSECSSIE_R {
        LSECSSIE_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CIER")
            .field("lsirdyie", &self.lsirdyie())
            .field("lsecssie", &self.lsecssie())
            .field("pllrdyie", &self.pllrdyie())
            .field("hserdyie", &self.hserdyie())
            .field("hsirdyie", &self.hsirdyie())
            .field("msirdyie", &self.msirdyie())
            .field("lserdyie", &self.lserdyie())
            .finish()
    }
}
impl W {
    ///Bit 0 - LSI ready interrupt enable
    #[inline(always)]
    pub fn lsirdyie(&mut self) -> LSIRDYIE_W<CIERrs> {
        LSIRDYIE_W::new(self, 0)
    }
    ///Bit 1 - LSE ready interrupt enable
    #[inline(always)]
    pub fn lserdyie(&mut self) -> LSERDYIE_W<CIERrs> {
        LSERDYIE_W::new(self, 1)
    }
    ///Bit 2 - MSI ready interrupt enable
    #[inline(always)]
    pub fn msirdyie(&mut self) -> MSIRDYIE_W<CIERrs> {
        MSIRDYIE_W::new(self, 2)
    }
    ///Bit 3 - HSI16 ready interrupt enable
    #[inline(always)]
    pub fn hsirdyie(&mut self) -> HSIRDYIE_W<CIERrs> {
        HSIRDYIE_W::new(self, 3)
    }
    ///Bit 4 - HSE32 ready interrupt enable
    #[inline(always)]
    pub fn hserdyie(&mut self) -> HSERDYIE_W<CIERrs> {
        HSERDYIE_W::new(self, 4)
    }
    ///Bit 5 - PLL ready interrupt enable
    #[inline(always)]
    pub fn pllrdyie(&mut self) -> PLLRDYIE_W<CIERrs> {
        PLLRDYIE_W::new(self, 5)
    }
    ///Bit 9 - LSE clock security system interrupt enable
    #[inline(always)]
    pub fn lsecssie(&mut self) -> LSECSSIE_W<CIERrs> {
        LSECSSIE_W::new(self, 9)
    }
}
/**Clock interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`cier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WLE5.html#RCC:CIER)*/
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
