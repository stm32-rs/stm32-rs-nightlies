///Register `MKEYR6` writer
pub type W = crate::W<MKEYR6rs>;
///Field `MKEY192` writer - Master key bit 192 (i = 31 to 0)
pub type MKEY192_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY193` writer - Master key bit 193 (i = 31 to 0)
pub type MKEY193_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY194` writer - Master key bit 194 (i = 31 to 0)
pub type MKEY194_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY195` writer - Master key bit 195 (i = 31 to 0)
pub type MKEY195_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY196` writer - Master key bit 196 (i = 31 to 0)
pub type MKEY196_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY197` writer - Master key bit 197 (i = 31 to 0)
pub type MKEY197_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY198` writer - Master key bit 198 (i = 31 to 0)
pub type MKEY198_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY199` writer - Master key bit 199 (i = 31 to 0)
pub type MKEY199_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY200` writer - Master key bit 200 (i = 31 to 0)
pub type MKEY200_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY201` writer - Master key bit 201 (i = 31 to 0)
pub type MKEY201_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY202` writer - Master key bit 202 (i = 31 to 0)
pub type MKEY202_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY203` writer - Master key bit 203 (i = 31 to 0)
pub type MKEY203_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY204` writer - Master key bit 204 (i = 31 to 0)
pub type MKEY204_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY205` writer - Master key bit 205 (i = 31 to 0)
pub type MKEY205_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY206` writer - Master key bit 206 (i = 31 to 0)
pub type MKEY206_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY207` writer - Master key bit 207 (i = 31 to 0)
pub type MKEY207_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY208` writer - Master key bit 208 (i = 31 to 0)
pub type MKEY208_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY209` writer - Master key bit 209 (i = 31 to 0)
pub type MKEY209_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY210` writer - Master key bit 210 (i = 31 to 0)
pub type MKEY210_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY211` writer - Master key bit 211 (i = 31 to 0)
pub type MKEY211_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY212` writer - Master key bit 212 (i = 31 to 0)
pub type MKEY212_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY213` writer - Master key bit 213 (i = 31 to 0)
pub type MKEY213_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY214` writer - Master key bit 214 (i = 31 to 0)
pub type MKEY214_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY215` writer - Master key bit 215 (i = 31 to 0)
pub type MKEY215_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY216` writer - Master key bit 216 (i = 31 to 0)
pub type MKEY216_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY217` writer - Master key bit 217 (i = 31 to 0)
pub type MKEY217_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY218` writer - Master key bit 218 (i = 31 to 0)
pub type MKEY218_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY219` writer - Master key bit 219 (i = 31 to 0)
pub type MKEY219_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY220` writer - Master key bit 220 (i = 31 to 0)
pub type MKEY220_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY221` writer - Master key bit 221 (i = 31 to 0)
pub type MKEY221_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY222` writer - Master key bit 222 (i = 31 to 0)
pub type MKEY222_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY223` writer - Master key bit 223 (i = 31 to 0)
pub type MKEY223_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<MKEYR6rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Master key bit 192 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey192(&mut self) -> MKEY192_W<'_, MKEYR6rs> {
        MKEY192_W::new(self, 0)
    }
    ///Bit 1 - Master key bit 193 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey193(&mut self) -> MKEY193_W<'_, MKEYR6rs> {
        MKEY193_W::new(self, 1)
    }
    ///Bit 2 - Master key bit 194 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey194(&mut self) -> MKEY194_W<'_, MKEYR6rs> {
        MKEY194_W::new(self, 2)
    }
    ///Bit 3 - Master key bit 195 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey195(&mut self) -> MKEY195_W<'_, MKEYR6rs> {
        MKEY195_W::new(self, 3)
    }
    ///Bit 4 - Master key bit 196 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey196(&mut self) -> MKEY196_W<'_, MKEYR6rs> {
        MKEY196_W::new(self, 4)
    }
    ///Bit 5 - Master key bit 197 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey197(&mut self) -> MKEY197_W<'_, MKEYR6rs> {
        MKEY197_W::new(self, 5)
    }
    ///Bit 6 - Master key bit 198 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey198(&mut self) -> MKEY198_W<'_, MKEYR6rs> {
        MKEY198_W::new(self, 6)
    }
    ///Bit 7 - Master key bit 199 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey199(&mut self) -> MKEY199_W<'_, MKEYR6rs> {
        MKEY199_W::new(self, 7)
    }
    ///Bit 8 - Master key bit 200 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey200(&mut self) -> MKEY200_W<'_, MKEYR6rs> {
        MKEY200_W::new(self, 8)
    }
    ///Bit 9 - Master key bit 201 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey201(&mut self) -> MKEY201_W<'_, MKEYR6rs> {
        MKEY201_W::new(self, 9)
    }
    ///Bit 10 - Master key bit 202 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey202(&mut self) -> MKEY202_W<'_, MKEYR6rs> {
        MKEY202_W::new(self, 10)
    }
    ///Bit 11 - Master key bit 203 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey203(&mut self) -> MKEY203_W<'_, MKEYR6rs> {
        MKEY203_W::new(self, 11)
    }
    ///Bit 12 - Master key bit 204 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey204(&mut self) -> MKEY204_W<'_, MKEYR6rs> {
        MKEY204_W::new(self, 12)
    }
    ///Bit 13 - Master key bit 205 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey205(&mut self) -> MKEY205_W<'_, MKEYR6rs> {
        MKEY205_W::new(self, 13)
    }
    ///Bit 14 - Master key bit 206 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey206(&mut self) -> MKEY206_W<'_, MKEYR6rs> {
        MKEY206_W::new(self, 14)
    }
    ///Bit 15 - Master key bit 207 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey207(&mut self) -> MKEY207_W<'_, MKEYR6rs> {
        MKEY207_W::new(self, 15)
    }
    ///Bit 16 - Master key bit 208 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey208(&mut self) -> MKEY208_W<'_, MKEYR6rs> {
        MKEY208_W::new(self, 16)
    }
    ///Bit 17 - Master key bit 209 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey209(&mut self) -> MKEY209_W<'_, MKEYR6rs> {
        MKEY209_W::new(self, 17)
    }
    ///Bit 18 - Master key bit 210 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey210(&mut self) -> MKEY210_W<'_, MKEYR6rs> {
        MKEY210_W::new(self, 18)
    }
    ///Bit 19 - Master key bit 211 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey211(&mut self) -> MKEY211_W<'_, MKEYR6rs> {
        MKEY211_W::new(self, 19)
    }
    ///Bit 20 - Master key bit 212 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey212(&mut self) -> MKEY212_W<'_, MKEYR6rs> {
        MKEY212_W::new(self, 20)
    }
    ///Bit 21 - Master key bit 213 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey213(&mut self) -> MKEY213_W<'_, MKEYR6rs> {
        MKEY213_W::new(self, 21)
    }
    ///Bit 22 - Master key bit 214 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey214(&mut self) -> MKEY214_W<'_, MKEYR6rs> {
        MKEY214_W::new(self, 22)
    }
    ///Bit 23 - Master key bit 215 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey215(&mut self) -> MKEY215_W<'_, MKEYR6rs> {
        MKEY215_W::new(self, 23)
    }
    ///Bit 24 - Master key bit 216 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey216(&mut self) -> MKEY216_W<'_, MKEYR6rs> {
        MKEY216_W::new(self, 24)
    }
    ///Bit 25 - Master key bit 217 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey217(&mut self) -> MKEY217_W<'_, MKEYR6rs> {
        MKEY217_W::new(self, 25)
    }
    ///Bit 26 - Master key bit 218 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey218(&mut self) -> MKEY218_W<'_, MKEYR6rs> {
        MKEY218_W::new(self, 26)
    }
    ///Bit 27 - Master key bit 219 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey219(&mut self) -> MKEY219_W<'_, MKEYR6rs> {
        MKEY219_W::new(self, 27)
    }
    ///Bit 28 - Master key bit 220 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey220(&mut self) -> MKEY220_W<'_, MKEYR6rs> {
        MKEY220_W::new(self, 28)
    }
    ///Bit 29 - Master key bit 221 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey221(&mut self) -> MKEY221_W<'_, MKEYR6rs> {
        MKEY221_W::new(self, 29)
    }
    ///Bit 30 - Master key bit 222 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey222(&mut self) -> MKEY222_W<'_, MKEYR6rs> {
        MKEY222_W::new(self, 30)
    }
    ///Bit 31 - Master key bit 223 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey223(&mut self) -> MKEY223_W<'_, MKEYR6rs> {
        MKEY223_W::new(self, 31)
    }
}
/**.MCE master key 6

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mkeyr6::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MCE1:MKEYR6)*/
pub struct MKEYR6rs;
impl crate::RegisterSpec for MKEYR6rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`mkeyr6::W`](W) writer structure
impl crate::Writable for MKEYR6rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MKEYR6 to value 0
impl crate::Resettable for MKEYR6rs {}
