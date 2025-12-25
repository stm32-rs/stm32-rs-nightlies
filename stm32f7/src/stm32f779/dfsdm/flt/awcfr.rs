///Register `AWCFR` reader
pub type R = crate::R<AWCFRrs>;
///Register `AWCFR` writer
pub type W = crate::W<AWCFRrs>;
/**Clear the analog watchdog low threshold flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLRAWLTF0W {
    ///1: Clear the corresponding AWLTF\[y\] bit
    Clear = 1,
}
impl From<CLRAWLTF0W> for bool {
    #[inline(always)]
    fn from(variant: CLRAWLTF0W) -> Self {
        variant as u8 != 0
    }
}
///Field `CLRAWLTF(0-7)` reader - Clear the analog watchdog low threshold flag
pub type CLRAWLTF_R = crate::BitReader<CLRAWLTF0W>;
impl CLRAWLTF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CLRAWLTF0W> {
        match self.bits {
            true => Some(CLRAWLTF0W::Clear),
            _ => None,
        }
    }
    ///Clear the corresponding AWLTF\[y\] bit
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CLRAWLTF0W::Clear
    }
}
///Field `CLRAWLTF(0-7)` writer - Clear the analog watchdog low threshold flag
pub type CLRAWLTF_W<'a, REG> = crate::BitWriter1C<'a, REG, CLRAWLTF0W>;
impl<'a, REG> CLRAWLTF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear the corresponding AWLTF\[y\] bit
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CLRAWLTF0W::Clear)
    }
}
/**Clear the analog watchdog high threshold flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLRAWHTF0W {
    ///1: Clear the corresponding AWHTF\[y\] bit
    Clear = 1,
}
impl From<CLRAWHTF0W> for bool {
    #[inline(always)]
    fn from(variant: CLRAWHTF0W) -> Self {
        variant as u8 != 0
    }
}
///Field `CLRAWHTF(0-7)` reader - Clear the analog watchdog high threshold flag
pub type CLRAWHTF_R = crate::BitReader<CLRAWHTF0W>;
impl CLRAWHTF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CLRAWHTF0W> {
        match self.bits {
            true => Some(CLRAWHTF0W::Clear),
            _ => None,
        }
    }
    ///Clear the corresponding AWHTF\[y\] bit
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CLRAWHTF0W::Clear
    }
}
///Field `CLRAWHTF(0-7)` writer - Clear the analog watchdog high threshold flag
pub type CLRAWHTF_W<'a, REG> = crate::BitWriter1C<'a, REG, CLRAWHTF0W>;
impl<'a, REG> CLRAWHTF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear the corresponding AWHTF\[y\] bit
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CLRAWHTF0W::Clear)
    }
}
impl R {
    ///Clear the analog watchdog low threshold flag
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CLRAWLTF0` field.</div>
    #[inline(always)]
    pub fn clrawltf(&self, n: u8) -> CLRAWLTF_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        CLRAWLTF_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///Clear the analog watchdog low threshold flag
    #[inline(always)]
    pub fn clrawltf_iter(&self) -> impl Iterator<Item = CLRAWLTF_R> + '_ {
        (0..8).map(move |n| CLRAWLTF_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - Clear the analog watchdog low threshold flag
    #[inline(always)]
    pub fn clrawltf0(&self) -> CLRAWLTF_R {
        CLRAWLTF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Clear the analog watchdog low threshold flag
    #[inline(always)]
    pub fn clrawltf1(&self) -> CLRAWLTF_R {
        CLRAWLTF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Clear the analog watchdog low threshold flag
    #[inline(always)]
    pub fn clrawltf2(&self) -> CLRAWLTF_R {
        CLRAWLTF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Clear the analog watchdog low threshold flag
    #[inline(always)]
    pub fn clrawltf3(&self) -> CLRAWLTF_R {
        CLRAWLTF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Clear the analog watchdog low threshold flag
    #[inline(always)]
    pub fn clrawltf4(&self) -> CLRAWLTF_R {
        CLRAWLTF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Clear the analog watchdog low threshold flag
    #[inline(always)]
    pub fn clrawltf5(&self) -> CLRAWLTF_R {
        CLRAWLTF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Clear the analog watchdog low threshold flag
    #[inline(always)]
    pub fn clrawltf6(&self) -> CLRAWLTF_R {
        CLRAWLTF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Clear the analog watchdog low threshold flag
    #[inline(always)]
    pub fn clrawltf7(&self) -> CLRAWLTF_R {
        CLRAWLTF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Clear the analog watchdog high threshold flag
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CLRAWHTF0` field.</div>
    #[inline(always)]
    pub fn clrawhtf(&self, n: u8) -> CLRAWHTF_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        CLRAWHTF_R::new(((self.bits >> (n + 8)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Clear the analog watchdog high threshold flag
    #[inline(always)]
    pub fn clrawhtf_iter(&self) -> impl Iterator<Item = CLRAWHTF_R> + '_ {
        (0..8).map(move |n| CLRAWHTF_R::new(((self.bits >> (n + 8)) & 1) != 0))
    }
    ///Bit 8 - Clear the analog watchdog high threshold flag
    #[inline(always)]
    pub fn clrawhtf0(&self) -> CLRAWHTF_R {
        CLRAWHTF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Clear the analog watchdog high threshold flag
    #[inline(always)]
    pub fn clrawhtf1(&self) -> CLRAWHTF_R {
        CLRAWHTF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Clear the analog watchdog high threshold flag
    #[inline(always)]
    pub fn clrawhtf2(&self) -> CLRAWHTF_R {
        CLRAWHTF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Clear the analog watchdog high threshold flag
    #[inline(always)]
    pub fn clrawhtf3(&self) -> CLRAWHTF_R {
        CLRAWHTF_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Clear the analog watchdog high threshold flag
    #[inline(always)]
    pub fn clrawhtf4(&self) -> CLRAWHTF_R {
        CLRAWHTF_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Clear the analog watchdog high threshold flag
    #[inline(always)]
    pub fn clrawhtf5(&self) -> CLRAWHTF_R {
        CLRAWHTF_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Clear the analog watchdog high threshold flag
    #[inline(always)]
    pub fn clrawhtf6(&self) -> CLRAWHTF_R {
        CLRAWHTF_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Clear the analog watchdog high threshold flag
    #[inline(always)]
    pub fn clrawhtf7(&self) -> CLRAWHTF_R {
        CLRAWHTF_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AWCFR")
            .field("clrawhtf0", &self.clrawhtf0())
            .field("clrawhtf1", &self.clrawhtf1())
            .field("clrawhtf2", &self.clrawhtf2())
            .field("clrawhtf3", &self.clrawhtf3())
            .field("clrawhtf4", &self.clrawhtf4())
            .field("clrawhtf5", &self.clrawhtf5())
            .field("clrawhtf6", &self.clrawhtf6())
            .field("clrawhtf7", &self.clrawhtf7())
            .field("clrawltf0", &self.clrawltf0())
            .field("clrawltf1", &self.clrawltf1())
            .field("clrawltf2", &self.clrawltf2())
            .field("clrawltf3", &self.clrawltf3())
            .field("clrawltf4", &self.clrawltf4())
            .field("clrawltf5", &self.clrawltf5())
            .field("clrawltf6", &self.clrawltf6())
            .field("clrawltf7", &self.clrawltf7())
            .finish()
    }
}
impl W {
    ///Clear the analog watchdog low threshold flag
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CLRAWLTF0` field.</div>
    #[inline(always)]
    pub fn clrawltf(&mut self, n: u8) -> CLRAWLTF_W<'_, AWCFRrs> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        CLRAWLTF_W::new(self, n)
    }
    ///Bit 0 - Clear the analog watchdog low threshold flag
    #[inline(always)]
    pub fn clrawltf0(&mut self) -> CLRAWLTF_W<'_, AWCFRrs> {
        CLRAWLTF_W::new(self, 0)
    }
    ///Bit 1 - Clear the analog watchdog low threshold flag
    #[inline(always)]
    pub fn clrawltf1(&mut self) -> CLRAWLTF_W<'_, AWCFRrs> {
        CLRAWLTF_W::new(self, 1)
    }
    ///Bit 2 - Clear the analog watchdog low threshold flag
    #[inline(always)]
    pub fn clrawltf2(&mut self) -> CLRAWLTF_W<'_, AWCFRrs> {
        CLRAWLTF_W::new(self, 2)
    }
    ///Bit 3 - Clear the analog watchdog low threshold flag
    #[inline(always)]
    pub fn clrawltf3(&mut self) -> CLRAWLTF_W<'_, AWCFRrs> {
        CLRAWLTF_W::new(self, 3)
    }
    ///Bit 4 - Clear the analog watchdog low threshold flag
    #[inline(always)]
    pub fn clrawltf4(&mut self) -> CLRAWLTF_W<'_, AWCFRrs> {
        CLRAWLTF_W::new(self, 4)
    }
    ///Bit 5 - Clear the analog watchdog low threshold flag
    #[inline(always)]
    pub fn clrawltf5(&mut self) -> CLRAWLTF_W<'_, AWCFRrs> {
        CLRAWLTF_W::new(self, 5)
    }
    ///Bit 6 - Clear the analog watchdog low threshold flag
    #[inline(always)]
    pub fn clrawltf6(&mut self) -> CLRAWLTF_W<'_, AWCFRrs> {
        CLRAWLTF_W::new(self, 6)
    }
    ///Bit 7 - Clear the analog watchdog low threshold flag
    #[inline(always)]
    pub fn clrawltf7(&mut self) -> CLRAWLTF_W<'_, AWCFRrs> {
        CLRAWLTF_W::new(self, 7)
    }
    ///Clear the analog watchdog high threshold flag
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CLRAWHTF0` field.</div>
    #[inline(always)]
    pub fn clrawhtf(&mut self, n: u8) -> CLRAWHTF_W<'_, AWCFRrs> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        CLRAWHTF_W::new(self, n + 8)
    }
    ///Bit 8 - Clear the analog watchdog high threshold flag
    #[inline(always)]
    pub fn clrawhtf0(&mut self) -> CLRAWHTF_W<'_, AWCFRrs> {
        CLRAWHTF_W::new(self, 8)
    }
    ///Bit 9 - Clear the analog watchdog high threshold flag
    #[inline(always)]
    pub fn clrawhtf1(&mut self) -> CLRAWHTF_W<'_, AWCFRrs> {
        CLRAWHTF_W::new(self, 9)
    }
    ///Bit 10 - Clear the analog watchdog high threshold flag
    #[inline(always)]
    pub fn clrawhtf2(&mut self) -> CLRAWHTF_W<'_, AWCFRrs> {
        CLRAWHTF_W::new(self, 10)
    }
    ///Bit 11 - Clear the analog watchdog high threshold flag
    #[inline(always)]
    pub fn clrawhtf3(&mut self) -> CLRAWHTF_W<'_, AWCFRrs> {
        CLRAWHTF_W::new(self, 11)
    }
    ///Bit 12 - Clear the analog watchdog high threshold flag
    #[inline(always)]
    pub fn clrawhtf4(&mut self) -> CLRAWHTF_W<'_, AWCFRrs> {
        CLRAWHTF_W::new(self, 12)
    }
    ///Bit 13 - Clear the analog watchdog high threshold flag
    #[inline(always)]
    pub fn clrawhtf5(&mut self) -> CLRAWHTF_W<'_, AWCFRrs> {
        CLRAWHTF_W::new(self, 13)
    }
    ///Bit 14 - Clear the analog watchdog high threshold flag
    #[inline(always)]
    pub fn clrawhtf6(&mut self) -> CLRAWHTF_W<'_, AWCFRrs> {
        CLRAWHTF_W::new(self, 14)
    }
    ///Bit 15 - Clear the analog watchdog high threshold flag
    #[inline(always)]
    pub fn clrawhtf7(&mut self) -> CLRAWHTF_W<'_, AWCFRrs> {
        CLRAWHTF_W::new(self, 15)
    }
}
/**DFSDM analog watchdog clear flag register

You can [`read`](crate::Reg::read) this register and get [`awcfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awcfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AWCFRrs;
impl crate::RegisterSpec for AWCFRrs {
    type Ux = u32;
}
///`read()` method returns [`awcfr::R`](R) reader structure
impl crate::Readable for AWCFRrs {}
///`write(|w| ..)` method takes [`awcfr::W`](W) writer structure
impl crate::Writable for AWCFRrs {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xffff;
}
///`reset()` method sets AWCFR to value 0
impl crate::Resettable for AWCFRrs {}
