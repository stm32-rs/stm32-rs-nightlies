///Register `FMKEYR7` writer
pub type W = crate::W<FMKEYR7rs>;
///Field `FMKEY224` writer - Fast master key bit 224 (i = 31 to 0)
pub type FMKEY224_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY225` writer - Fast master key bit 225 (i = 31 to 0)
pub type FMKEY225_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY226` writer - Fast master key bit 226 (i = 31 to 0)
pub type FMKEY226_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY227` writer - Fast master key bit 227 (i = 31 to 0)
pub type FMKEY227_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY228` writer - Fast master key bit 228 (i = 31 to 0)
pub type FMKEY228_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY229` writer - Fast master key bit 229 (i = 31 to 0)
pub type FMKEY229_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY230` writer - Fast master key bit 230 (i = 31 to 0)
pub type FMKEY230_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY231` writer - Fast master key bit 231 (i = 31 to 0)
pub type FMKEY231_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY232` writer - Fast master key bit 232 (i = 31 to 0)
pub type FMKEY232_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY233` writer - Fast master key bit 233 (i = 31 to 0)
pub type FMKEY233_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY234` writer - Fast master key bit 234 (i = 31 to 0)
pub type FMKEY234_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY235` writer - Fast master key bit 235 (i = 31 to 0)
pub type FMKEY235_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY236` writer - Fast master key bit 236 (i = 31 to 0)
pub type FMKEY236_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY237` writer - Fast master key bit 237 (i = 31 to 0)
pub type FMKEY237_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY238` writer - Fast master key bit 238 (i = 31 to 0)
pub type FMKEY238_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY239` writer - Fast master key bit 239 (i = 31 to 0)
pub type FMKEY239_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY240` writer - Fast master key bit 240 (i = 31 to 0)
pub type FMKEY240_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY241` writer - Fast master key bit 241 (i = 31 to 0)
pub type FMKEY241_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY242` writer - Fast master key bit 242 (i = 31 to 0)
pub type FMKEY242_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY243` writer - Fast master key bit 243 (i = 31 to 0)
pub type FMKEY243_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY244` writer - Fast master key bit 244 (i = 31 to 0)
pub type FMKEY244_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY245` writer - Fast master key bit 245 (i = 31 to 0)
pub type FMKEY245_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY246` writer - Fast master key bit 246 (i = 31 to 0)
pub type FMKEY246_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY247` writer - Fast master key bit 247 (i = 31 to 0)
pub type FMKEY247_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY248` writer - Fast master key bit 248 (i = 31 to 0)
pub type FMKEY248_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY249` writer - Fast master key bit 249 (i = 31 to 0)
pub type FMKEY249_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY250` writer - Fast master key bit 250 (i = 31 to 0)
pub type FMKEY250_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY251` writer - Fast master key bit 251 (i = 31 to 0)
pub type FMKEY251_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY252` writer - Fast master key bit 252 (i = 31 to 0)
pub type FMKEY252_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY253` writer - Fast master key bit 253 (i = 31 to 0)
pub type FMKEY253_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY254` writer - Fast master key bit 254 (i = 31 to 0)
pub type FMKEY254_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY255` writer - Fast master key bit 255 (i = 31 to 0)
pub type FMKEY255_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<FMKEYR7rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Fast master key bit 224 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey224(&mut self) -> FMKEY224_W<'_, FMKEYR7rs> {
        FMKEY224_W::new(self, 0)
    }
    ///Bit 1 - Fast master key bit 225 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey225(&mut self) -> FMKEY225_W<'_, FMKEYR7rs> {
        FMKEY225_W::new(self, 1)
    }
    ///Bit 2 - Fast master key bit 226 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey226(&mut self) -> FMKEY226_W<'_, FMKEYR7rs> {
        FMKEY226_W::new(self, 2)
    }
    ///Bit 3 - Fast master key bit 227 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey227(&mut self) -> FMKEY227_W<'_, FMKEYR7rs> {
        FMKEY227_W::new(self, 3)
    }
    ///Bit 4 - Fast master key bit 228 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey228(&mut self) -> FMKEY228_W<'_, FMKEYR7rs> {
        FMKEY228_W::new(self, 4)
    }
    ///Bit 5 - Fast master key bit 229 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey229(&mut self) -> FMKEY229_W<'_, FMKEYR7rs> {
        FMKEY229_W::new(self, 5)
    }
    ///Bit 6 - Fast master key bit 230 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey230(&mut self) -> FMKEY230_W<'_, FMKEYR7rs> {
        FMKEY230_W::new(self, 6)
    }
    ///Bit 7 - Fast master key bit 231 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey231(&mut self) -> FMKEY231_W<'_, FMKEYR7rs> {
        FMKEY231_W::new(self, 7)
    }
    ///Bit 8 - Fast master key bit 232 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey232(&mut self) -> FMKEY232_W<'_, FMKEYR7rs> {
        FMKEY232_W::new(self, 8)
    }
    ///Bit 9 - Fast master key bit 233 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey233(&mut self) -> FMKEY233_W<'_, FMKEYR7rs> {
        FMKEY233_W::new(self, 9)
    }
    ///Bit 10 - Fast master key bit 234 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey234(&mut self) -> FMKEY234_W<'_, FMKEYR7rs> {
        FMKEY234_W::new(self, 10)
    }
    ///Bit 11 - Fast master key bit 235 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey235(&mut self) -> FMKEY235_W<'_, FMKEYR7rs> {
        FMKEY235_W::new(self, 11)
    }
    ///Bit 12 - Fast master key bit 236 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey236(&mut self) -> FMKEY236_W<'_, FMKEYR7rs> {
        FMKEY236_W::new(self, 12)
    }
    ///Bit 13 - Fast master key bit 237 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey237(&mut self) -> FMKEY237_W<'_, FMKEYR7rs> {
        FMKEY237_W::new(self, 13)
    }
    ///Bit 14 - Fast master key bit 238 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey238(&mut self) -> FMKEY238_W<'_, FMKEYR7rs> {
        FMKEY238_W::new(self, 14)
    }
    ///Bit 15 - Fast master key bit 239 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey239(&mut self) -> FMKEY239_W<'_, FMKEYR7rs> {
        FMKEY239_W::new(self, 15)
    }
    ///Bit 16 - Fast master key bit 240 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey240(&mut self) -> FMKEY240_W<'_, FMKEYR7rs> {
        FMKEY240_W::new(self, 16)
    }
    ///Bit 17 - Fast master key bit 241 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey241(&mut self) -> FMKEY241_W<'_, FMKEYR7rs> {
        FMKEY241_W::new(self, 17)
    }
    ///Bit 18 - Fast master key bit 242 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey242(&mut self) -> FMKEY242_W<'_, FMKEYR7rs> {
        FMKEY242_W::new(self, 18)
    }
    ///Bit 19 - Fast master key bit 243 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey243(&mut self) -> FMKEY243_W<'_, FMKEYR7rs> {
        FMKEY243_W::new(self, 19)
    }
    ///Bit 20 - Fast master key bit 244 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey244(&mut self) -> FMKEY244_W<'_, FMKEYR7rs> {
        FMKEY244_W::new(self, 20)
    }
    ///Bit 21 - Fast master key bit 245 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey245(&mut self) -> FMKEY245_W<'_, FMKEYR7rs> {
        FMKEY245_W::new(self, 21)
    }
    ///Bit 22 - Fast master key bit 246 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey246(&mut self) -> FMKEY246_W<'_, FMKEYR7rs> {
        FMKEY246_W::new(self, 22)
    }
    ///Bit 23 - Fast master key bit 247 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey247(&mut self) -> FMKEY247_W<'_, FMKEYR7rs> {
        FMKEY247_W::new(self, 23)
    }
    ///Bit 24 - Fast master key bit 248 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey248(&mut self) -> FMKEY248_W<'_, FMKEYR7rs> {
        FMKEY248_W::new(self, 24)
    }
    ///Bit 25 - Fast master key bit 249 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey249(&mut self) -> FMKEY249_W<'_, FMKEYR7rs> {
        FMKEY249_W::new(self, 25)
    }
    ///Bit 26 - Fast master key bit 250 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey250(&mut self) -> FMKEY250_W<'_, FMKEYR7rs> {
        FMKEY250_W::new(self, 26)
    }
    ///Bit 27 - Fast master key bit 251 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey251(&mut self) -> FMKEY251_W<'_, FMKEYR7rs> {
        FMKEY251_W::new(self, 27)
    }
    ///Bit 28 - Fast master key bit 252 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey252(&mut self) -> FMKEY252_W<'_, FMKEYR7rs> {
        FMKEY252_W::new(self, 28)
    }
    ///Bit 29 - Fast master key bit 253 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey253(&mut self) -> FMKEY253_W<'_, FMKEYR7rs> {
        FMKEY253_W::new(self, 29)
    }
    ///Bit 30 - Fast master key bit 254 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey254(&mut self) -> FMKEY254_W<'_, FMKEYR7rs> {
        FMKEY254_W::new(self, 30)
    }
    ///Bit 31 - Fast master key bit 255 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey255(&mut self) -> FMKEY255_W<'_, FMKEYR7rs> {
        FMKEY255_W::new(self, 31)
    }
}
/**MCE fast master key 7

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmkeyr7::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#MCE1:FMKEYR7)*/
pub struct FMKEYR7rs;
impl crate::RegisterSpec for FMKEYR7rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`fmkeyr7::W`](W) writer structure
impl crate::Writable for FMKEYR7rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FMKEYR7 to value 0
impl crate::Resettable for FMKEYR7rs {}
