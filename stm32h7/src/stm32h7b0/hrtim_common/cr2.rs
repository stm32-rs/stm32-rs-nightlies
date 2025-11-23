///Register `CR2` reader
pub type R = crate::R<CR2rs>;
///Register `CR2` writer
pub type W = crate::W<CR2rs>;
/**Master Timer Software update

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSWU {
    ///1: Force immediate update
    Update = 1,
}
impl From<MSWU> for bool {
    #[inline(always)]
    fn from(variant: MSWU) -> Self {
        variant as u8 != 0
    }
}
///Field `MSWU` reader - Master Timer Software update
pub type MSWU_R = crate::BitReader<MSWU>;
impl MSWU_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<MSWU> {
        match self.bits {
            true => Some(MSWU::Update),
            _ => None,
        }
    }
    ///Force immediate update
    #[inline(always)]
    pub fn is_update(&self) -> bool {
        *self == MSWU::Update
    }
}
///Field `MSWU` writer - Master Timer Software update
pub type MSWU_W<'a, REG> = crate::BitWriter<'a, REG, MSWU>;
impl<'a, REG> MSWU_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Force immediate update
    #[inline(always)]
    pub fn update(self) -> &'a mut crate::W<REG> {
        self.variant(MSWU::Update)
    }
}
///Field `TSWU(A,B,C,D,E)` reader - Timer %s Software Update
pub use MSWU_R as TSWU_R;
///Field `TSWU(A,B,C,D,E)` writer - Timer %s Software Update
pub use MSWU_W as TSWU_W;
/**Master Counter software reset

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MRST {
    ///1: Reset timer
    Reset = 1,
}
impl From<MRST> for bool {
    #[inline(always)]
    fn from(variant: MRST) -> Self {
        variant as u8 != 0
    }
}
///Field `MRST` reader - Master Counter software reset
pub type MRST_R = crate::BitReader<MRST>;
impl MRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<MRST> {
        match self.bits {
            true => Some(MRST::Reset),
            _ => None,
        }
    }
    ///Reset timer
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == MRST::Reset
    }
}
///Field `MRST` writer - Master Counter software reset
pub type MRST_W<'a, REG> = crate::BitWriter<'a, REG, MRST>;
impl<'a, REG> MRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Reset timer
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(MRST::Reset)
    }
}
///Field `TRST(A,B,C,D,E)` reader - Timer %s counter software reset
pub use MRST_R as TRST_R;
///Field `TRST(A,B,C,D,E)` writer - Timer %s counter software reset
pub use MRST_W as TRST_W;
impl R {
    ///Bit 0 - Master Timer Software update
    #[inline(always)]
    pub fn mswu(&self) -> MSWU_R {
        MSWU_R::new((self.bits & 1) != 0)
    }
    ///Timer (A,B,C,D,E) Software Update
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `TASWU` field.</div>
    #[inline(always)]
    pub fn tswu(&self, n: u8) -> TSWU_R {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        TSWU_R::new(((self.bits >> (n + 1)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Timer (A,B,C,D,E) Software Update
    #[inline(always)]
    pub fn tswu_iter(&self) -> impl Iterator<Item = TSWU_R> + '_ {
        (0..5).map(move |n| TSWU_R::new(((self.bits >> (n + 1)) & 1) != 0))
    }
    ///Bit 1 - Timer A Software Update
    #[inline(always)]
    pub fn taswu(&self) -> TSWU_R {
        TSWU_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Timer B Software Update
    #[inline(always)]
    pub fn tbswu(&self) -> TSWU_R {
        TSWU_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Timer C Software Update
    #[inline(always)]
    pub fn tcswu(&self) -> TSWU_R {
        TSWU_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Timer D Software Update
    #[inline(always)]
    pub fn tdswu(&self) -> TSWU_R {
        TSWU_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Timer E Software Update
    #[inline(always)]
    pub fn teswu(&self) -> TSWU_R {
        TSWU_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - Master Counter software reset
    #[inline(always)]
    pub fn mrst(&self) -> MRST_R {
        MRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Timer (A,B,C,D,E) counter software reset
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `TARST` field.</div>
    #[inline(always)]
    pub fn trst(&self, n: u8) -> TRST_R {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        TRST_R::new(((self.bits >> (n + 9)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Timer (A,B,C,D,E) counter software reset
    #[inline(always)]
    pub fn trst_iter(&self) -> impl Iterator<Item = TRST_R> + '_ {
        (0..5).map(move |n| TRST_R::new(((self.bits >> (n + 9)) & 1) != 0))
    }
    ///Bit 9 - Timer A counter software reset
    #[inline(always)]
    pub fn tarst(&self) -> TRST_R {
        TRST_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Timer B counter software reset
    #[inline(always)]
    pub fn tbrst(&self) -> TRST_R {
        TRST_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Timer C counter software reset
    #[inline(always)]
    pub fn tcrst(&self) -> TRST_R {
        TRST_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Timer D counter software reset
    #[inline(always)]
    pub fn tdrst(&self) -> TRST_R {
        TRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Timer E counter software reset
    #[inline(always)]
    pub fn terst(&self) -> TRST_R {
        TRST_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR2")
            .field("mrst", &self.mrst())
            .field("tarst", &self.tarst())
            .field("tbrst", &self.tbrst())
            .field("tcrst", &self.tcrst())
            .field("tdrst", &self.tdrst())
            .field("terst", &self.terst())
            .field("mswu", &self.mswu())
            .field("taswu", &self.taswu())
            .field("tbswu", &self.tbswu())
            .field("tcswu", &self.tcswu())
            .field("tdswu", &self.tdswu())
            .field("teswu", &self.teswu())
            .finish()
    }
}
impl W {
    ///Bit 0 - Master Timer Software update
    #[inline(always)]
    pub fn mswu(&mut self) -> MSWU_W<'_, CR2rs> {
        MSWU_W::new(self, 0)
    }
    ///Timer (A,B,C,D,E) Software Update
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `TASWU` field.</div>
    #[inline(always)]
    pub fn tswu(&mut self, n: u8) -> TSWU_W<'_, CR2rs> {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        TSWU_W::new(self, n + 1)
    }
    ///Bit 1 - Timer A Software Update
    #[inline(always)]
    pub fn taswu(&mut self) -> TSWU_W<'_, CR2rs> {
        TSWU_W::new(self, 1)
    }
    ///Bit 2 - Timer B Software Update
    #[inline(always)]
    pub fn tbswu(&mut self) -> TSWU_W<'_, CR2rs> {
        TSWU_W::new(self, 2)
    }
    ///Bit 3 - Timer C Software Update
    #[inline(always)]
    pub fn tcswu(&mut self) -> TSWU_W<'_, CR2rs> {
        TSWU_W::new(self, 3)
    }
    ///Bit 4 - Timer D Software Update
    #[inline(always)]
    pub fn tdswu(&mut self) -> TSWU_W<'_, CR2rs> {
        TSWU_W::new(self, 4)
    }
    ///Bit 5 - Timer E Software Update
    #[inline(always)]
    pub fn teswu(&mut self) -> TSWU_W<'_, CR2rs> {
        TSWU_W::new(self, 5)
    }
    ///Bit 8 - Master Counter software reset
    #[inline(always)]
    pub fn mrst(&mut self) -> MRST_W<'_, CR2rs> {
        MRST_W::new(self, 8)
    }
    ///Timer (A,B,C,D,E) counter software reset
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `TARST` field.</div>
    #[inline(always)]
    pub fn trst(&mut self, n: u8) -> TRST_W<'_, CR2rs> {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        TRST_W::new(self, n + 9)
    }
    ///Bit 9 - Timer A counter software reset
    #[inline(always)]
    pub fn tarst(&mut self) -> TRST_W<'_, CR2rs> {
        TRST_W::new(self, 9)
    }
    ///Bit 10 - Timer B counter software reset
    #[inline(always)]
    pub fn tbrst(&mut self) -> TRST_W<'_, CR2rs> {
        TRST_W::new(self, 10)
    }
    ///Bit 11 - Timer C counter software reset
    #[inline(always)]
    pub fn tcrst(&mut self) -> TRST_W<'_, CR2rs> {
        TRST_W::new(self, 11)
    }
    ///Bit 12 - Timer D counter software reset
    #[inline(always)]
    pub fn tdrst(&mut self) -> TRST_W<'_, CR2rs> {
        TRST_W::new(self, 12)
    }
    ///Bit 13 - Timer E counter software reset
    #[inline(always)]
    pub fn terst(&mut self) -> TRST_W<'_, CR2rs> {
        TRST_W::new(self, 13)
    }
}
/**Control Register 2

You can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B0.html#HRTIM_Common:CR2)*/
pub struct CR2rs;
impl crate::RegisterSpec for CR2rs {
    type Ux = u32;
}
///`read()` method returns [`cr2::R`](R) reader structure
impl crate::Readable for CR2rs {}
///`write(|w| ..)` method takes [`cr2::W`](W) writer structure
impl crate::Writable for CR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR2 to value 0
impl crate::Resettable for CR2rs {}
