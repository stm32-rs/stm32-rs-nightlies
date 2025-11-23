///Register `CIER` reader
pub type R = crate::R<CIERrs>;
///Register `CIER` writer
pub type W = crate::W<CIERrs>;
/**LSI1 ready interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSI1RDYIE {
    ///0: Interrupt disabled
    Disabled = 0,
    ///1: Interrupt enabled
    Enabled = 1,
}
impl From<LSI1RDYIE> for bool {
    #[inline(always)]
    fn from(variant: LSI1RDYIE) -> Self {
        variant as u8 != 0
    }
}
///Field `LSI1RDYIE` reader - LSI1 ready interrupt enable
pub type LSI1RDYIE_R = crate::BitReader<LSI1RDYIE>;
impl LSI1RDYIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LSI1RDYIE {
        match self.bits {
            false => LSI1RDYIE::Disabled,
            true => LSI1RDYIE::Enabled,
        }
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LSI1RDYIE::Disabled
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LSI1RDYIE::Enabled
    }
}
///Field `LSI1RDYIE` writer - LSI1 ready interrupt enable
pub type LSI1RDYIE_W<'a, REG> = crate::BitWriter<'a, REG, LSI1RDYIE>;
impl<'a, REG> LSI1RDYIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LSI1RDYIE::Disabled)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LSI1RDYIE::Enabled)
    }
}
///Field `LSERDYIE` reader - LSE ready interrupt enable
pub use LSI1RDYIE_R as LSERDYIE_R;
///Field `MSIRDYIE` reader - MSI ready interrupt enable
pub use LSI1RDYIE_R as MSIRDYIE_R;
///Field `HSIRDYIE` reader - HSI ready interrupt enable
pub use LSI1RDYIE_R as HSIRDYIE_R;
///Field `HSERDYIE` reader - HSE ready interrupt enable
pub use LSI1RDYIE_R as HSERDYIE_R;
///Field `PLLRDYIE` reader - PLLSYS ready interrupt enable
pub use LSI1RDYIE_R as PLLRDYIE_R;
///Field `PLLSAI1RDYIE` reader - PLLSAI1 ready interrupt enable
pub use LSI1RDYIE_R as PLLSAI1RDYIE_R;
///Field `LSECSSIE` reader - LSE clock security system interrupt enable
pub use LSI1RDYIE_R as LSECSSIE_R;
///Field `HSI48RDYIE` reader - HSI48 ready interrupt enable
pub use LSI1RDYIE_R as HSI48RDYIE_R;
///Field `LSI2RDYIE` reader - LSI2 ready interrupt enable
pub use LSI1RDYIE_R as LSI2RDYIE_R;
///Field `LSERDYIE` writer - LSE ready interrupt enable
pub use LSI1RDYIE_W as LSERDYIE_W;
///Field `MSIRDYIE` writer - MSI ready interrupt enable
pub use LSI1RDYIE_W as MSIRDYIE_W;
///Field `HSIRDYIE` writer - HSI ready interrupt enable
pub use LSI1RDYIE_W as HSIRDYIE_W;
///Field `HSERDYIE` writer - HSE ready interrupt enable
pub use LSI1RDYIE_W as HSERDYIE_W;
///Field `PLLRDYIE` writer - PLLSYS ready interrupt enable
pub use LSI1RDYIE_W as PLLRDYIE_W;
///Field `PLLSAI1RDYIE` writer - PLLSAI1 ready interrupt enable
pub use LSI1RDYIE_W as PLLSAI1RDYIE_W;
///Field `LSECSSIE` writer - LSE clock security system interrupt enable
pub use LSI1RDYIE_W as LSECSSIE_W;
///Field `HSI48RDYIE` writer - HSI48 ready interrupt enable
pub use LSI1RDYIE_W as HSI48RDYIE_W;
///Field `LSI2RDYIE` writer - LSI2 ready interrupt enable
pub use LSI1RDYIE_W as LSI2RDYIE_W;
impl R {
    ///Bit 0 - LSI1 ready interrupt enable
    #[inline(always)]
    pub fn lsi1rdyie(&self) -> LSI1RDYIE_R {
        LSI1RDYIE_R::new((self.bits & 1) != 0)
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
    ///Bit 3 - HSI ready interrupt enable
    #[inline(always)]
    pub fn hsirdyie(&self) -> HSIRDYIE_R {
        HSIRDYIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - HSE ready interrupt enable
    #[inline(always)]
    pub fn hserdyie(&self) -> HSERDYIE_R {
        HSERDYIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - PLLSYS ready interrupt enable
    #[inline(always)]
    pub fn pllrdyie(&self) -> PLLRDYIE_R {
        PLLRDYIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - PLLSAI1 ready interrupt enable
    #[inline(always)]
    pub fn pllsai1rdyie(&self) -> PLLSAI1RDYIE_R {
        PLLSAI1RDYIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 9 - LSE clock security system interrupt enable
    #[inline(always)]
    pub fn lsecssie(&self) -> LSECSSIE_R {
        LSECSSIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - HSI48 ready interrupt enable
    #[inline(always)]
    pub fn hsi48rdyie(&self) -> HSI48RDYIE_R {
        HSI48RDYIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - LSI2 ready interrupt enable
    #[inline(always)]
    pub fn lsi2rdyie(&self) -> LSI2RDYIE_R {
        LSI2RDYIE_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CIER")
            .field("lsi1rdyie", &self.lsi1rdyie())
            .field("lsi2rdyie", &self.lsi2rdyie())
            .field("hsi48rdyie", &self.hsi48rdyie())
            .field("lsecssie", &self.lsecssie())
            .field("pllsai1rdyie", &self.pllsai1rdyie())
            .field("pllrdyie", &self.pllrdyie())
            .field("hserdyie", &self.hserdyie())
            .field("hsirdyie", &self.hsirdyie())
            .field("msirdyie", &self.msirdyie())
            .field("lserdyie", &self.lserdyie())
            .finish()
    }
}
impl W {
    ///Bit 0 - LSI1 ready interrupt enable
    #[inline(always)]
    pub fn lsi1rdyie(&mut self) -> LSI1RDYIE_W<'_, CIERrs> {
        LSI1RDYIE_W::new(self, 0)
    }
    ///Bit 1 - LSE ready interrupt enable
    #[inline(always)]
    pub fn lserdyie(&mut self) -> LSERDYIE_W<'_, CIERrs> {
        LSERDYIE_W::new(self, 1)
    }
    ///Bit 2 - MSI ready interrupt enable
    #[inline(always)]
    pub fn msirdyie(&mut self) -> MSIRDYIE_W<'_, CIERrs> {
        MSIRDYIE_W::new(self, 2)
    }
    ///Bit 3 - HSI ready interrupt enable
    #[inline(always)]
    pub fn hsirdyie(&mut self) -> HSIRDYIE_W<'_, CIERrs> {
        HSIRDYIE_W::new(self, 3)
    }
    ///Bit 4 - HSE ready interrupt enable
    #[inline(always)]
    pub fn hserdyie(&mut self) -> HSERDYIE_W<'_, CIERrs> {
        HSERDYIE_W::new(self, 4)
    }
    ///Bit 5 - PLLSYS ready interrupt enable
    #[inline(always)]
    pub fn pllrdyie(&mut self) -> PLLRDYIE_W<'_, CIERrs> {
        PLLRDYIE_W::new(self, 5)
    }
    ///Bit 6 - PLLSAI1 ready interrupt enable
    #[inline(always)]
    pub fn pllsai1rdyie(&mut self) -> PLLSAI1RDYIE_W<'_, CIERrs> {
        PLLSAI1RDYIE_W::new(self, 6)
    }
    ///Bit 9 - LSE clock security system interrupt enable
    #[inline(always)]
    pub fn lsecssie(&mut self) -> LSECSSIE_W<'_, CIERrs> {
        LSECSSIE_W::new(self, 9)
    }
    ///Bit 10 - HSI48 ready interrupt enable
    #[inline(always)]
    pub fn hsi48rdyie(&mut self) -> HSI48RDYIE_W<'_, CIERrs> {
        HSI48RDYIE_W::new(self, 10)
    }
    ///Bit 11 - LSI2 ready interrupt enable
    #[inline(always)]
    pub fn lsi2rdyie(&mut self) -> LSI2RDYIE_W<'_, CIERrs> {
        LSI2RDYIE_W::new(self, 11)
    }
}
/**Clock interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`cier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#RCC:CIER)*/
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
