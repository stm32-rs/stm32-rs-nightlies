///Register `DIER` reader
pub type R = crate::R<DIERrs>;
///Register `DIER` writer
pub type W = crate::W<DIERrs>;
/**MCMP%sIE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMP1IE {
    ///0: Interrupt disabled
    Disabled = 0,
    ///1: Interrupt enabled
    Enabled = 1,
}
impl From<CMP1IE> for bool {
    #[inline(always)]
    fn from(variant: CMP1IE) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPIE(1-4)` reader - MCMP%sIE
pub type CMPIE_R = crate::BitReader<CMP1IE>;
impl CMPIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMP1IE {
        match self.bits {
            false => CMP1IE::Disabled,
            true => CMP1IE::Enabled,
        }
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CMP1IE::Disabled
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CMP1IE::Enabled
    }
}
///Field `CMPIE(1-4)` writer - MCMP%sIE
pub type CMPIE_W<'a, REG> = crate::BitWriter<'a, REG, CMP1IE>;
impl<'a, REG> CMPIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CMP1IE::Disabled)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CMP1IE::Enabled)
    }
}
///Field `REPIE` reader - MREPIE
pub use CMPIE_R as REPIE_R;
///Field `SYNCIE` reader - SYNCIE
pub use CMPIE_R as SYNCIE_R;
///Field `UPDIE` reader - MUPDIE
pub use CMPIE_R as UPDIE_R;
///Field `REPIE` writer - MREPIE
pub use CMPIE_W as REPIE_W;
///Field `SYNCIE` writer - SYNCIE
pub use CMPIE_W as SYNCIE_W;
///Field `UPDIE` writer - MUPDIE
pub use CMPIE_W as UPDIE_W;
/**MCMP%sDE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMP1DE {
    ///0: DMA request disabled
    Disabled = 0,
    ///1: DMA request enabled
    Enabled = 1,
}
impl From<CMP1DE> for bool {
    #[inline(always)]
    fn from(variant: CMP1DE) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPDE(1-4)` reader - MCMP%sDE
pub type CMPDE_R = crate::BitReader<CMP1DE>;
impl CMPDE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMP1DE {
        match self.bits {
            false => CMP1DE::Disabled,
            true => CMP1DE::Enabled,
        }
    }
    ///DMA request disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CMP1DE::Disabled
    }
    ///DMA request enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CMP1DE::Enabled
    }
}
///Field `CMPDE(1-4)` writer - MCMP%sDE
pub type CMPDE_W<'a, REG> = crate::BitWriter<'a, REG, CMP1DE>;
impl<'a, REG> CMPDE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DMA request disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CMP1DE::Disabled)
    }
    ///DMA request enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CMP1DE::Enabled)
    }
}
///Field `REPDE` reader - MREPDE
pub use CMPDE_R as REPDE_R;
///Field `SYNCDE` reader - SYNCDE
pub use CMPDE_R as SYNCDE_R;
///Field `UPDDE` reader - MUPDDE
pub use CMPDE_R as UPDDE_R;
///Field `REPDE` writer - MREPDE
pub use CMPDE_W as REPDE_W;
///Field `SYNCDE` writer - SYNCDE
pub use CMPDE_W as SYNCDE_W;
///Field `UPDDE` writer - MUPDDE
pub use CMPDE_W as UPDDE_W;
impl R {
    ///MCMP(1-4)IE
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CMP1IE` field.</div>
    #[inline(always)]
    pub fn cmpie(&self, n: u8) -> CMPIE_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CMPIE_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///MCMP(1-4)IE
    #[inline(always)]
    pub fn cmpie_iter(&self) -> impl Iterator<Item = CMPIE_R> + '_ {
        (0..4).map(move |n| CMPIE_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - MCMP1IE
    #[inline(always)]
    pub fn cmp1ie(&self) -> CMPIE_R {
        CMPIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - MCMP2IE
    #[inline(always)]
    pub fn cmp2ie(&self) -> CMPIE_R {
        CMPIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - MCMP3IE
    #[inline(always)]
    pub fn cmp3ie(&self) -> CMPIE_R {
        CMPIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - MCMP4IE
    #[inline(always)]
    pub fn cmp4ie(&self) -> CMPIE_R {
        CMPIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - MREPIE
    #[inline(always)]
    pub fn repie(&self) -> REPIE_R {
        REPIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - SYNCIE
    #[inline(always)]
    pub fn syncie(&self) -> SYNCIE_R {
        SYNCIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - MUPDIE
    #[inline(always)]
    pub fn updie(&self) -> UPDIE_R {
        UPDIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///MCMP(1-4)DE
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CMP1DE` field.</div>
    #[inline(always)]
    pub fn cmpde(&self, n: u8) -> CMPDE_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CMPDE_R::new(((self.bits >> (n + 16)) & 1) != 0)
    }
    ///Iterator for array of:
    ///MCMP(1-4)DE
    #[inline(always)]
    pub fn cmpde_iter(&self) -> impl Iterator<Item = CMPDE_R> + '_ {
        (0..4).map(move |n| CMPDE_R::new(((self.bits >> (n + 16)) & 1) != 0))
    }
    ///Bit 16 - MCMP1DE
    #[inline(always)]
    pub fn cmp1de(&self) -> CMPDE_R {
        CMPDE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - MCMP2DE
    #[inline(always)]
    pub fn cmp2de(&self) -> CMPDE_R {
        CMPDE_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - MCMP3DE
    #[inline(always)]
    pub fn cmp3de(&self) -> CMPDE_R {
        CMPDE_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - MCMP4DE
    #[inline(always)]
    pub fn cmp4de(&self) -> CMPDE_R {
        CMPDE_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - MREPDE
    #[inline(always)]
    pub fn repde(&self) -> REPDE_R {
        REPDE_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - SYNCDE
    #[inline(always)]
    pub fn syncde(&self) -> SYNCDE_R {
        SYNCDE_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - MUPDDE
    #[inline(always)]
    pub fn updde(&self) -> UPDDE_R {
        UPDDE_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIER")
            .field("cmp1de", &self.cmp1de())
            .field("cmp2de", &self.cmp2de())
            .field("cmp3de", &self.cmp3de())
            .field("cmp4de", &self.cmp4de())
            .field("updde", &self.updde())
            .field("syncde", &self.syncde())
            .field("repde", &self.repde())
            .field("cmp1ie", &self.cmp1ie())
            .field("cmp2ie", &self.cmp2ie())
            .field("cmp3ie", &self.cmp3ie())
            .field("cmp4ie", &self.cmp4ie())
            .field("updie", &self.updie())
            .field("syncie", &self.syncie())
            .field("repie", &self.repie())
            .finish()
    }
}
impl W {
    ///MCMP(1-4)IE
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CMP1IE` field.</div>
    #[inline(always)]
    pub fn cmpie(&mut self, n: u8) -> CMPIE_W<'_, DIERrs> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CMPIE_W::new(self, n)
    }
    ///Bit 0 - MCMP1IE
    #[inline(always)]
    pub fn cmp1ie(&mut self) -> CMPIE_W<'_, DIERrs> {
        CMPIE_W::new(self, 0)
    }
    ///Bit 1 - MCMP2IE
    #[inline(always)]
    pub fn cmp2ie(&mut self) -> CMPIE_W<'_, DIERrs> {
        CMPIE_W::new(self, 1)
    }
    ///Bit 2 - MCMP3IE
    #[inline(always)]
    pub fn cmp3ie(&mut self) -> CMPIE_W<'_, DIERrs> {
        CMPIE_W::new(self, 2)
    }
    ///Bit 3 - MCMP4IE
    #[inline(always)]
    pub fn cmp4ie(&mut self) -> CMPIE_W<'_, DIERrs> {
        CMPIE_W::new(self, 3)
    }
    ///Bit 4 - MREPIE
    #[inline(always)]
    pub fn repie(&mut self) -> REPIE_W<'_, DIERrs> {
        REPIE_W::new(self, 4)
    }
    ///Bit 5 - SYNCIE
    #[inline(always)]
    pub fn syncie(&mut self) -> SYNCIE_W<'_, DIERrs> {
        SYNCIE_W::new(self, 5)
    }
    ///Bit 6 - MUPDIE
    #[inline(always)]
    pub fn updie(&mut self) -> UPDIE_W<'_, DIERrs> {
        UPDIE_W::new(self, 6)
    }
    ///MCMP(1-4)DE
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CMP1DE` field.</div>
    #[inline(always)]
    pub fn cmpde(&mut self, n: u8) -> CMPDE_W<'_, DIERrs> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CMPDE_W::new(self, n + 16)
    }
    ///Bit 16 - MCMP1DE
    #[inline(always)]
    pub fn cmp1de(&mut self) -> CMPDE_W<'_, DIERrs> {
        CMPDE_W::new(self, 16)
    }
    ///Bit 17 - MCMP2DE
    #[inline(always)]
    pub fn cmp2de(&mut self) -> CMPDE_W<'_, DIERrs> {
        CMPDE_W::new(self, 17)
    }
    ///Bit 18 - MCMP3DE
    #[inline(always)]
    pub fn cmp3de(&mut self) -> CMPDE_W<'_, DIERrs> {
        CMPDE_W::new(self, 18)
    }
    ///Bit 19 - MCMP4DE
    #[inline(always)]
    pub fn cmp4de(&mut self) -> CMPDE_W<'_, DIERrs> {
        CMPDE_W::new(self, 19)
    }
    ///Bit 20 - MREPDE
    #[inline(always)]
    pub fn repde(&mut self) -> REPDE_W<'_, DIERrs> {
        REPDE_W::new(self, 20)
    }
    ///Bit 21 - SYNCDE
    #[inline(always)]
    pub fn syncde(&mut self) -> SYNCDE_W<'_, DIERrs> {
        SYNCDE_W::new(self, 21)
    }
    ///Bit 22 - MUPDDE
    #[inline(always)]
    pub fn updde(&mut self) -> UPDDE_W<'_, DIERrs> {
        UPDDE_W::new(self, 22)
    }
}
/**MDIER4

You can [`read`](crate::Reg::read) this register and get [`dier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM7.html#HRTIM_Master:DIER)*/
pub struct DIERrs;
impl crate::RegisterSpec for DIERrs {
    type Ux = u32;
}
///`read()` method returns [`dier::R`](R) reader structure
impl crate::Readable for DIERrs {}
///`write(|w| ..)` method takes [`dier::W`](W) writer structure
impl crate::Writable for DIERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DIER to value 0
impl crate::Resettable for DIERrs {}
