///Register `WIER` reader
pub type R = crate::R<WIERrs>;
///Register `WIER` writer
pub type W = crate::W<WIERrs>;
/**Tearing effect interrupt enable This bit enables the tearing effect interrupt.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEIE {
    ///0: Tearing effect interrupt disabled
    B0x0 = 0,
    ///1: Tearing effect interrupt enabled
    B0x1 = 1,
}
impl From<TEIE> for bool {
    #[inline(always)]
    fn from(variant: TEIE) -> Self {
        variant as u8 != 0
    }
}
///Field `TEIE` reader - Tearing effect interrupt enable This bit enables the tearing effect interrupt.
pub type TEIE_R = crate::BitReader<TEIE>;
impl TEIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TEIE {
        match self.bits {
            false => TEIE::B0x0,
            true => TEIE::B0x1,
        }
    }
    ///Tearing effect interrupt disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TEIE::B0x0
    }
    ///Tearing effect interrupt enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TEIE::B0x1
    }
}
///Field `TEIE` writer - Tearing effect interrupt enable This bit enables the tearing effect interrupt.
pub type TEIE_W<'a, REG> = crate::BitWriter<'a, REG, TEIE>;
impl<'a, REG> TEIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Tearing effect interrupt disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TEIE::B0x0)
    }
    ///Tearing effect interrupt enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TEIE::B0x1)
    }
}
/**End of refresh interrupt enable This bit enables the end of refresh interrupt.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERIE {
    ///0: End of refresh interrupt disabled
    B0x0 = 0,
    ///1: End of refresh interrupt enabled
    B0x1 = 1,
}
impl From<ERIE> for bool {
    #[inline(always)]
    fn from(variant: ERIE) -> Self {
        variant as u8 != 0
    }
}
///Field `ERIE` reader - End of refresh interrupt enable This bit enables the end of refresh interrupt.
pub type ERIE_R = crate::BitReader<ERIE>;
impl ERIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ERIE {
        match self.bits {
            false => ERIE::B0x0,
            true => ERIE::B0x1,
        }
    }
    ///End of refresh interrupt disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ERIE::B0x0
    }
    ///End of refresh interrupt enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ERIE::B0x1
    }
}
///Field `ERIE` writer - End of refresh interrupt enable This bit enables the end of refresh interrupt.
pub type ERIE_W<'a, REG> = crate::BitWriter<'a, REG, ERIE>;
impl<'a, REG> ERIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///End of refresh interrupt disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ERIE::B0x0)
    }
    ///End of refresh interrupt enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ERIE::B0x1)
    }
}
/**PLL lock interrupt enable This bit enables the PLL lock interrupt.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLLIE {
    ///0: PLL lock interrupt disabled
    B0x0 = 0,
    ///1: PLL lock interrupt enabled
    B0x1 = 1,
}
impl From<PLLLIE> for bool {
    #[inline(always)]
    fn from(variant: PLLLIE) -> Self {
        variant as u8 != 0
    }
}
///Field `PLLLIE` reader - PLL lock interrupt enable This bit enables the PLL lock interrupt.
pub type PLLLIE_R = crate::BitReader<PLLLIE>;
impl PLLLIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLLLIE {
        match self.bits {
            false => PLLLIE::B0x0,
            true => PLLLIE::B0x1,
        }
    }
    ///PLL lock interrupt disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PLLLIE::B0x0
    }
    ///PLL lock interrupt enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PLLLIE::B0x1
    }
}
///Field `PLLLIE` writer - PLL lock interrupt enable This bit enables the PLL lock interrupt.
pub type PLLLIE_W<'a, REG> = crate::BitWriter<'a, REG, PLLLIE>;
impl<'a, REG> PLLLIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PLL lock interrupt disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PLLLIE::B0x0)
    }
    ///PLL lock interrupt enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PLLLIE::B0x1)
    }
}
/**PLL unlock interrupt enable This bit enables the PLL unlock interrupt.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLUIE {
    ///0: PLL unlock interrupt disabled
    B0x0 = 0,
    ///1: PLL unlock interrupt enabled
    B0x1 = 1,
}
impl From<PLLUIE> for bool {
    #[inline(always)]
    fn from(variant: PLLUIE) -> Self {
        variant as u8 != 0
    }
}
///Field `PLLUIE` reader - PLL unlock interrupt enable This bit enables the PLL unlock interrupt.
pub type PLLUIE_R = crate::BitReader<PLLUIE>;
impl PLLUIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLLUIE {
        match self.bits {
            false => PLLUIE::B0x0,
            true => PLLUIE::B0x1,
        }
    }
    ///PLL unlock interrupt disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PLLUIE::B0x0
    }
    ///PLL unlock interrupt enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PLLUIE::B0x1
    }
}
///Field `PLLUIE` writer - PLL unlock interrupt enable This bit enables the PLL unlock interrupt.
pub type PLLUIE_W<'a, REG> = crate::BitWriter<'a, REG, PLLUIE>;
impl<'a, REG> PLLUIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PLL unlock interrupt disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PLLUIE::B0x0)
    }
    ///PLL unlock interrupt enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PLLUIE::B0x1)
    }
}
/**Regulator ready interrupt enable This bit enables the regulator ready interrupt.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RRIE {
    ///0: Regulator ready interrupt disabled
    B0x0 = 0,
    ///1: Regulator ready interrupt enabled
    B0x1 = 1,
}
impl From<RRIE> for bool {
    #[inline(always)]
    fn from(variant: RRIE) -> Self {
        variant as u8 != 0
    }
}
///Field `RRIE` reader - Regulator ready interrupt enable This bit enables the regulator ready interrupt.
pub type RRIE_R = crate::BitReader<RRIE>;
impl RRIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RRIE {
        match self.bits {
            false => RRIE::B0x0,
            true => RRIE::B0x1,
        }
    }
    ///Regulator ready interrupt disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RRIE::B0x0
    }
    ///Regulator ready interrupt enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RRIE::B0x1
    }
}
///Field `RRIE` writer - Regulator ready interrupt enable This bit enables the regulator ready interrupt.
pub type RRIE_W<'a, REG> = crate::BitWriter<'a, REG, RRIE>;
impl<'a, REG> RRIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Regulator ready interrupt disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RRIE::B0x0)
    }
    ///Regulator ready interrupt enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RRIE::B0x1)
    }
}
impl R {
    ///Bit 0 - Tearing effect interrupt enable This bit enables the tearing effect interrupt.
    #[inline(always)]
    pub fn teie(&self) -> TEIE_R {
        TEIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - End of refresh interrupt enable This bit enables the end of refresh interrupt.
    #[inline(always)]
    pub fn erie(&self) -> ERIE_R {
        ERIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 9 - PLL lock interrupt enable This bit enables the PLL lock interrupt.
    #[inline(always)]
    pub fn plllie(&self) -> PLLLIE_R {
        PLLLIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - PLL unlock interrupt enable This bit enables the PLL unlock interrupt.
    #[inline(always)]
    pub fn plluie(&self) -> PLLUIE_R {
        PLLUIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 13 - Regulator ready interrupt enable This bit enables the regulator ready interrupt.
    #[inline(always)]
    pub fn rrie(&self) -> RRIE_R {
        RRIE_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WIER")
            .field("teie", &self.teie())
            .field("erie", &self.erie())
            .field("plllie", &self.plllie())
            .field("plluie", &self.plluie())
            .field("rrie", &self.rrie())
            .finish()
    }
}
impl W {
    ///Bit 0 - Tearing effect interrupt enable This bit enables the tearing effect interrupt.
    #[inline(always)]
    pub fn teie(&mut self) -> TEIE_W<WIERrs> {
        TEIE_W::new(self, 0)
    }
    ///Bit 1 - End of refresh interrupt enable This bit enables the end of refresh interrupt.
    #[inline(always)]
    pub fn erie(&mut self) -> ERIE_W<WIERrs> {
        ERIE_W::new(self, 1)
    }
    ///Bit 9 - PLL lock interrupt enable This bit enables the PLL lock interrupt.
    #[inline(always)]
    pub fn plllie(&mut self) -> PLLLIE_W<WIERrs> {
        PLLLIE_W::new(self, 9)
    }
    ///Bit 10 - PLL unlock interrupt enable This bit enables the PLL unlock interrupt.
    #[inline(always)]
    pub fn plluie(&mut self) -> PLLUIE_W<WIERrs> {
        PLLUIE_W::new(self, 10)
    }
    ///Bit 13 - Regulator ready interrupt enable This bit enables the regulator ready interrupt.
    #[inline(always)]
    pub fn rrie(&mut self) -> RRIE_W<WIERrs> {
        RRIE_W::new(self, 13)
    }
}
/**DSI Wrapper interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`wier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F469.html#DSI:WIER)*/
pub struct WIERrs;
impl crate::RegisterSpec for WIERrs {
    type Ux = u32;
}
///`read()` method returns [`wier::R`](R) reader structure
impl crate::Readable for WIERrs {}
///`write(|w| ..)` method takes [`wier::W`](W) writer structure
impl crate::Writable for WIERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WIER to value 0
impl crate::Resettable for WIERrs {}
