///Register `MDIER` reader
pub type R = crate::R<MDIERrs>;
///Register `MDIER` writer
pub type W = crate::W<MDIERrs>;
/**MCMP1IE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCMP1IE {
    ///0: Interrupt disabled
    Disabled = 0,
    ///1: Interrupt enabled
    Enabled = 1,
}
impl From<MCMP1IE> for bool {
    #[inline(always)]
    fn from(variant: MCMP1IE) -> Self {
        variant as u8 != 0
    }
}
///Field `MCMP1IE` reader - MCMP1IE
pub type MCMP1IE_R = crate::BitReader<MCMP1IE>;
impl MCMP1IE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MCMP1IE {
        match self.bits {
            false => MCMP1IE::Disabled,
            true => MCMP1IE::Enabled,
        }
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MCMP1IE::Disabled
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MCMP1IE::Enabled
    }
}
///Field `MCMP1IE` writer - MCMP1IE
pub type MCMP1IE_W<'a, REG> = crate::BitWriter<'a, REG, MCMP1IE>;
impl<'a, REG> MCMP1IE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MCMP1IE::Disabled)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MCMP1IE::Enabled)
    }
}
///Field `MCMP2IE` reader - MCMP2IE
pub use MCMP1IE_R as MCMP2IE_R;
///Field `MCMP3IE` reader - MCMP3IE
pub use MCMP1IE_R as MCMP3IE_R;
///Field `MCMP4IE` reader - MCMP4IE
pub use MCMP1IE_R as MCMP4IE_R;
///Field `MREPIE` reader - MREPIE
pub use MCMP1IE_R as MREPIE_R;
///Field `SYNCIE` reader - SYNCIE
pub use MCMP1IE_R as SYNCIE_R;
///Field `MUPDIE` reader - MUPDIE
pub use MCMP1IE_R as MUPDIE_R;
///Field `MCMP2IE` writer - MCMP2IE
pub use MCMP1IE_W as MCMP2IE_W;
///Field `MCMP3IE` writer - MCMP3IE
pub use MCMP1IE_W as MCMP3IE_W;
///Field `MCMP4IE` writer - MCMP4IE
pub use MCMP1IE_W as MCMP4IE_W;
///Field `MREPIE` writer - MREPIE
pub use MCMP1IE_W as MREPIE_W;
///Field `SYNCIE` writer - SYNCIE
pub use MCMP1IE_W as SYNCIE_W;
///Field `MUPDIE` writer - MUPDIE
pub use MCMP1IE_W as MUPDIE_W;
/**MCMP1DE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCMP1DE {
    ///0: DMA request disabled
    Disabled = 0,
    ///1: DMA request enabled
    Enabled = 1,
}
impl From<MCMP1DE> for bool {
    #[inline(always)]
    fn from(variant: MCMP1DE) -> Self {
        variant as u8 != 0
    }
}
///Field `MCMP1DE` reader - MCMP1DE
pub type MCMP1DE_R = crate::BitReader<MCMP1DE>;
impl MCMP1DE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MCMP1DE {
        match self.bits {
            false => MCMP1DE::Disabled,
            true => MCMP1DE::Enabled,
        }
    }
    ///DMA request disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MCMP1DE::Disabled
    }
    ///DMA request enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MCMP1DE::Enabled
    }
}
///Field `MCMP1DE` writer - MCMP1DE
pub type MCMP1DE_W<'a, REG> = crate::BitWriter<'a, REG, MCMP1DE>;
impl<'a, REG> MCMP1DE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DMA request disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MCMP1DE::Disabled)
    }
    ///DMA request enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MCMP1DE::Enabled)
    }
}
///Field `MCMP2DE` reader - MCMP2DE
pub use MCMP1DE_R as MCMP2DE_R;
///Field `MCMP3DE` reader - MCMP3DE
pub use MCMP1DE_R as MCMP3DE_R;
///Field `MCMP4DE` reader - MCMP4DE
pub use MCMP1DE_R as MCMP4DE_R;
///Field `MREPDE` reader - MREPDE
pub use MCMP1DE_R as MREPDE_R;
///Field `SYNCDE` reader - SYNCDE
pub use MCMP1DE_R as SYNCDE_R;
///Field `MUPDDE` reader - MUPDDE
pub use MCMP1DE_R as MUPDDE_R;
///Field `MCMP2DE` writer - MCMP2DE
pub use MCMP1DE_W as MCMP2DE_W;
///Field `MCMP3DE` writer - MCMP3DE
pub use MCMP1DE_W as MCMP3DE_W;
///Field `MCMP4DE` writer - MCMP4DE
pub use MCMP1DE_W as MCMP4DE_W;
///Field `MREPDE` writer - MREPDE
pub use MCMP1DE_W as MREPDE_W;
///Field `SYNCDE` writer - SYNCDE
pub use MCMP1DE_W as SYNCDE_W;
///Field `MUPDDE` writer - MUPDDE
pub use MCMP1DE_W as MUPDDE_W;
impl R {
    ///Bit 0 - MCMP1IE
    #[inline(always)]
    pub fn mcmp1ie(&self) -> MCMP1IE_R {
        MCMP1IE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - MCMP2IE
    #[inline(always)]
    pub fn mcmp2ie(&self) -> MCMP2IE_R {
        MCMP2IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - MCMP3IE
    #[inline(always)]
    pub fn mcmp3ie(&self) -> MCMP3IE_R {
        MCMP3IE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - MCMP4IE
    #[inline(always)]
    pub fn mcmp4ie(&self) -> MCMP4IE_R {
        MCMP4IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - MREPIE
    #[inline(always)]
    pub fn mrepie(&self) -> MREPIE_R {
        MREPIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - SYNCIE
    #[inline(always)]
    pub fn syncie(&self) -> SYNCIE_R {
        SYNCIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - MUPDIE
    #[inline(always)]
    pub fn mupdie(&self) -> MUPDIE_R {
        MUPDIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 16 - MCMP1DE
    #[inline(always)]
    pub fn mcmp1de(&self) -> MCMP1DE_R {
        MCMP1DE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - MCMP2DE
    #[inline(always)]
    pub fn mcmp2de(&self) -> MCMP2DE_R {
        MCMP2DE_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - MCMP3DE
    #[inline(always)]
    pub fn mcmp3de(&self) -> MCMP3DE_R {
        MCMP3DE_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - MCMP4DE
    #[inline(always)]
    pub fn mcmp4de(&self) -> MCMP4DE_R {
        MCMP4DE_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - MREPDE
    #[inline(always)]
    pub fn mrepde(&self) -> MREPDE_R {
        MREPDE_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - SYNCDE
    #[inline(always)]
    pub fn syncde(&self) -> SYNCDE_R {
        SYNCDE_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - MUPDDE
    #[inline(always)]
    pub fn mupdde(&self) -> MUPDDE_R {
        MUPDDE_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MDIER")
            .field("mcmp1de", &self.mcmp1de())
            .field("mupdde", &self.mupdde())
            .field("syncde", &self.syncde())
            .field("mrepde", &self.mrepde())
            .field("mcmp4de", &self.mcmp4de())
            .field("mcmp3de", &self.mcmp3de())
            .field("mcmp2de", &self.mcmp2de())
            .field("mcmp1ie", &self.mcmp1ie())
            .field("mupdie", &self.mupdie())
            .field("syncie", &self.syncie())
            .field("mrepie", &self.mrepie())
            .field("mcmp4ie", &self.mcmp4ie())
            .field("mcmp3ie", &self.mcmp3ie())
            .field("mcmp2ie", &self.mcmp2ie())
            .finish()
    }
}
impl W {
    ///Bit 0 - MCMP1IE
    #[inline(always)]
    #[must_use]
    pub fn mcmp1ie(&mut self) -> MCMP1IE_W<MDIERrs> {
        MCMP1IE_W::new(self, 0)
    }
    ///Bit 1 - MCMP2IE
    #[inline(always)]
    #[must_use]
    pub fn mcmp2ie(&mut self) -> MCMP2IE_W<MDIERrs> {
        MCMP2IE_W::new(self, 1)
    }
    ///Bit 2 - MCMP3IE
    #[inline(always)]
    #[must_use]
    pub fn mcmp3ie(&mut self) -> MCMP3IE_W<MDIERrs> {
        MCMP3IE_W::new(self, 2)
    }
    ///Bit 3 - MCMP4IE
    #[inline(always)]
    #[must_use]
    pub fn mcmp4ie(&mut self) -> MCMP4IE_W<MDIERrs> {
        MCMP4IE_W::new(self, 3)
    }
    ///Bit 4 - MREPIE
    #[inline(always)]
    #[must_use]
    pub fn mrepie(&mut self) -> MREPIE_W<MDIERrs> {
        MREPIE_W::new(self, 4)
    }
    ///Bit 5 - SYNCIE
    #[inline(always)]
    #[must_use]
    pub fn syncie(&mut self) -> SYNCIE_W<MDIERrs> {
        SYNCIE_W::new(self, 5)
    }
    ///Bit 6 - MUPDIE
    #[inline(always)]
    #[must_use]
    pub fn mupdie(&mut self) -> MUPDIE_W<MDIERrs> {
        MUPDIE_W::new(self, 6)
    }
    ///Bit 16 - MCMP1DE
    #[inline(always)]
    #[must_use]
    pub fn mcmp1de(&mut self) -> MCMP1DE_W<MDIERrs> {
        MCMP1DE_W::new(self, 16)
    }
    ///Bit 17 - MCMP2DE
    #[inline(always)]
    #[must_use]
    pub fn mcmp2de(&mut self) -> MCMP2DE_W<MDIERrs> {
        MCMP2DE_W::new(self, 17)
    }
    ///Bit 18 - MCMP3DE
    #[inline(always)]
    #[must_use]
    pub fn mcmp3de(&mut self) -> MCMP3DE_W<MDIERrs> {
        MCMP3DE_W::new(self, 18)
    }
    ///Bit 19 - MCMP4DE
    #[inline(always)]
    #[must_use]
    pub fn mcmp4de(&mut self) -> MCMP4DE_W<MDIERrs> {
        MCMP4DE_W::new(self, 19)
    }
    ///Bit 20 - MREPDE
    #[inline(always)]
    #[must_use]
    pub fn mrepde(&mut self) -> MREPDE_W<MDIERrs> {
        MREPDE_W::new(self, 20)
    }
    ///Bit 21 - SYNCDE
    #[inline(always)]
    #[must_use]
    pub fn syncde(&mut self) -> SYNCDE_W<MDIERrs> {
        SYNCDE_W::new(self, 21)
    }
    ///Bit 22 - MUPDDE
    #[inline(always)]
    #[must_use]
    pub fn mupdde(&mut self) -> MUPDDE_W<MDIERrs> {
        MUPDDE_W::new(self, 22)
    }
}
/**MDIER4

You can [`read`](crate::Reg::read) this register and get [`mdier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#HRTIM_Master:MDIER)*/
pub struct MDIERrs;
impl crate::RegisterSpec for MDIERrs {
    type Ux = u32;
}
///`read()` method returns [`mdier::R`](R) reader structure
impl crate::Readable for MDIERrs {}
///`write(|w| ..)` method takes [`mdier::W`](W) writer structure
impl crate::Writable for MDIERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets MDIER to value 0
impl crate::Resettable for MDIERrs {
    const RESET_VALUE: u32 = 0;
}
