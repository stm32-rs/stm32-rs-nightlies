///Register `FMKEYR6` writer
pub type W = crate::W<FMKEYR6rs>;
///Field `FMKEY192` writer - Fast master key bit 192 (i = 31 to 0)
pub type FMKEY192_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY193` writer - Fast master key bit 193 (i = 31 to 0)
pub type FMKEY193_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY194` writer - Fast master key bit 194 (i = 31 to 0)
pub type FMKEY194_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY195` writer - Fast master key bit 195 (i = 31 to 0)
pub type FMKEY195_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY196` writer - Fast master key bit 196 (i = 31 to 0)
pub type FMKEY196_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY197` writer - Fast master key bit 197 (i = 31 to 0)
pub type FMKEY197_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY198` writer - Fast master key bit 198 (i = 31 to 0)
pub type FMKEY198_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY199` writer - Fast master key bit 199 (i = 31 to 0)
pub type FMKEY199_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY200` writer - Fast master key bit 200 (i = 31 to 0)
pub type FMKEY200_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY201` writer - Fast master key bit 201 (i = 31 to 0)
pub type FMKEY201_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY202` writer - Fast master key bit 202 (i = 31 to 0)
pub type FMKEY202_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY203` writer - Fast master key bit 203 (i = 31 to 0)
pub type FMKEY203_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY204` writer - Fast master key bit 204 (i = 31 to 0)
pub type FMKEY204_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY205` writer - Fast master key bit 205 (i = 31 to 0)
pub type FMKEY205_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY206` writer - Fast master key bit 206 (i = 31 to 0)
pub type FMKEY206_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY207` writer - Fast master key bit 207 (i = 31 to 0)
pub type FMKEY207_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY208` writer - Fast master key bit 208 (i = 31 to 0)
pub type FMKEY208_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY209` writer - Fast master key bit 209 (i = 31 to 0)
pub type FMKEY209_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY210` writer - Fast master key bit 210 (i = 31 to 0)
pub type FMKEY210_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY211` writer - Fast master key bit 211 (i = 31 to 0)
pub type FMKEY211_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY212` writer - Fast master key bit 212 (i = 31 to 0)
pub type FMKEY212_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY213` writer - Fast master key bit 213 (i = 31 to 0)
pub type FMKEY213_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY214` writer - Fast master key bit 214 (i = 31 to 0)
pub type FMKEY214_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY215` writer - Fast master key bit 215 (i = 31 to 0)
pub type FMKEY215_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY216` writer - Fast master key bit 216 (i = 31 to 0)
pub type FMKEY216_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY217` writer - Fast master key bit 217 (i = 31 to 0)
pub type FMKEY217_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY218` writer - Fast master key bit 218 (i = 31 to 0)
pub type FMKEY218_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY219` writer - Fast master key bit 219 (i = 31 to 0)
pub type FMKEY219_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY220` writer - Fast master key bit 220 (i = 31 to 0)
pub type FMKEY220_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY221` writer - Fast master key bit 221 (i = 31 to 0)
pub type FMKEY221_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY222` writer - Fast master key bit 222 (i = 31 to 0)
pub type FMKEY222_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY223` writer - Fast master key bit 223 (i = 31 to 0)
pub type FMKEY223_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<FMKEYR6rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Fast master key bit 192 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey192(&mut self) -> FMKEY192_W<'_, FMKEYR6rs> {
        FMKEY192_W::new(self, 0)
    }
    ///Bit 1 - Fast master key bit 193 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey193(&mut self) -> FMKEY193_W<'_, FMKEYR6rs> {
        FMKEY193_W::new(self, 1)
    }
    ///Bit 2 - Fast master key bit 194 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey194(&mut self) -> FMKEY194_W<'_, FMKEYR6rs> {
        FMKEY194_W::new(self, 2)
    }
    ///Bit 3 - Fast master key bit 195 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey195(&mut self) -> FMKEY195_W<'_, FMKEYR6rs> {
        FMKEY195_W::new(self, 3)
    }
    ///Bit 4 - Fast master key bit 196 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey196(&mut self) -> FMKEY196_W<'_, FMKEYR6rs> {
        FMKEY196_W::new(self, 4)
    }
    ///Bit 5 - Fast master key bit 197 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey197(&mut self) -> FMKEY197_W<'_, FMKEYR6rs> {
        FMKEY197_W::new(self, 5)
    }
    ///Bit 6 - Fast master key bit 198 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey198(&mut self) -> FMKEY198_W<'_, FMKEYR6rs> {
        FMKEY198_W::new(self, 6)
    }
    ///Bit 7 - Fast master key bit 199 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey199(&mut self) -> FMKEY199_W<'_, FMKEYR6rs> {
        FMKEY199_W::new(self, 7)
    }
    ///Bit 8 - Fast master key bit 200 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey200(&mut self) -> FMKEY200_W<'_, FMKEYR6rs> {
        FMKEY200_W::new(self, 8)
    }
    ///Bit 9 - Fast master key bit 201 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey201(&mut self) -> FMKEY201_W<'_, FMKEYR6rs> {
        FMKEY201_W::new(self, 9)
    }
    ///Bit 10 - Fast master key bit 202 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey202(&mut self) -> FMKEY202_W<'_, FMKEYR6rs> {
        FMKEY202_W::new(self, 10)
    }
    ///Bit 11 - Fast master key bit 203 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey203(&mut self) -> FMKEY203_W<'_, FMKEYR6rs> {
        FMKEY203_W::new(self, 11)
    }
    ///Bit 12 - Fast master key bit 204 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey204(&mut self) -> FMKEY204_W<'_, FMKEYR6rs> {
        FMKEY204_W::new(self, 12)
    }
    ///Bit 13 - Fast master key bit 205 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey205(&mut self) -> FMKEY205_W<'_, FMKEYR6rs> {
        FMKEY205_W::new(self, 13)
    }
    ///Bit 14 - Fast master key bit 206 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey206(&mut self) -> FMKEY206_W<'_, FMKEYR6rs> {
        FMKEY206_W::new(self, 14)
    }
    ///Bit 15 - Fast master key bit 207 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey207(&mut self) -> FMKEY207_W<'_, FMKEYR6rs> {
        FMKEY207_W::new(self, 15)
    }
    ///Bit 16 - Fast master key bit 208 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey208(&mut self) -> FMKEY208_W<'_, FMKEYR6rs> {
        FMKEY208_W::new(self, 16)
    }
    ///Bit 17 - Fast master key bit 209 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey209(&mut self) -> FMKEY209_W<'_, FMKEYR6rs> {
        FMKEY209_W::new(self, 17)
    }
    ///Bit 18 - Fast master key bit 210 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey210(&mut self) -> FMKEY210_W<'_, FMKEYR6rs> {
        FMKEY210_W::new(self, 18)
    }
    ///Bit 19 - Fast master key bit 211 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey211(&mut self) -> FMKEY211_W<'_, FMKEYR6rs> {
        FMKEY211_W::new(self, 19)
    }
    ///Bit 20 - Fast master key bit 212 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey212(&mut self) -> FMKEY212_W<'_, FMKEYR6rs> {
        FMKEY212_W::new(self, 20)
    }
    ///Bit 21 - Fast master key bit 213 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey213(&mut self) -> FMKEY213_W<'_, FMKEYR6rs> {
        FMKEY213_W::new(self, 21)
    }
    ///Bit 22 - Fast master key bit 214 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey214(&mut self) -> FMKEY214_W<'_, FMKEYR6rs> {
        FMKEY214_W::new(self, 22)
    }
    ///Bit 23 - Fast master key bit 215 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey215(&mut self) -> FMKEY215_W<'_, FMKEYR6rs> {
        FMKEY215_W::new(self, 23)
    }
    ///Bit 24 - Fast master key bit 216 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey216(&mut self) -> FMKEY216_W<'_, FMKEYR6rs> {
        FMKEY216_W::new(self, 24)
    }
    ///Bit 25 - Fast master key bit 217 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey217(&mut self) -> FMKEY217_W<'_, FMKEYR6rs> {
        FMKEY217_W::new(self, 25)
    }
    ///Bit 26 - Fast master key bit 218 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey218(&mut self) -> FMKEY218_W<'_, FMKEYR6rs> {
        FMKEY218_W::new(self, 26)
    }
    ///Bit 27 - Fast master key bit 219 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey219(&mut self) -> FMKEY219_W<'_, FMKEYR6rs> {
        FMKEY219_W::new(self, 27)
    }
    ///Bit 28 - Fast master key bit 220 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey220(&mut self) -> FMKEY220_W<'_, FMKEYR6rs> {
        FMKEY220_W::new(self, 28)
    }
    ///Bit 29 - Fast master key bit 221 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey221(&mut self) -> FMKEY221_W<'_, FMKEYR6rs> {
        FMKEY221_W::new(self, 29)
    }
    ///Bit 30 - Fast master key bit 222 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey222(&mut self) -> FMKEY222_W<'_, FMKEYR6rs> {
        FMKEY222_W::new(self, 30)
    }
    ///Bit 31 - Fast master key bit 223 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey223(&mut self) -> FMKEY223_W<'_, FMKEYR6rs> {
        FMKEY223_W::new(self, 31)
    }
}
/**MCE fast master key 6

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmkeyr6::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#MCE1:FMKEYR6)*/
pub struct FMKEYR6rs;
impl crate::RegisterSpec for FMKEYR6rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`fmkeyr6::W`](W) writer structure
impl crate::Writable for FMKEYR6rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FMKEYR6 to value 0
impl crate::Resettable for FMKEYR6rs {}
