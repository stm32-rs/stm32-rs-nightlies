///Register `MKEYR7` writer
pub type W = crate::W<MKEYR7rs>;
///Field `MKEY224` writer - Master key bit 224 (i = 31 to 0)
pub type MKEY224_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY225` writer - Master key bit 225 (i = 31 to 0)
pub type MKEY225_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY226` writer - Master key bit 226 (i = 31 to 0)
pub type MKEY226_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY227` writer - Master key bit 227 (i = 31 to 0)
pub type MKEY227_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY228` writer - Master key bit 228 (i = 31 to 0)
pub type MKEY228_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY229` writer - Master key bit 229 (i = 31 to 0)
pub type MKEY229_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY230` writer - Master key bit 230 (i = 31 to 0)
pub type MKEY230_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY231` writer - Master key bit 231 (i = 31 to 0)
pub type MKEY231_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY232` writer - Master key bit 232 (i = 31 to 0)
pub type MKEY232_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY233` writer - Master key bit 233 (i = 31 to 0)
pub type MKEY233_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY234` writer - Master key bit 234 (i = 31 to 0)
pub type MKEY234_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY235` writer - Master key bit 235 (i = 31 to 0)
pub type MKEY235_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY236` writer - Master key bit 236 (i = 31 to 0)
pub type MKEY236_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY237` writer - Master key bit 237 (i = 31 to 0)
pub type MKEY237_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY238` writer - Master key bit 238 (i = 31 to 0)
pub type MKEY238_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY239` writer - Master key bit 239 (i = 31 to 0)
pub type MKEY239_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY240` writer - Master key bit 240 (i = 31 to 0)
pub type MKEY240_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY241` writer - Master key bit 241 (i = 31 to 0)
pub type MKEY241_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY242` writer - Master key bit 242 (i = 31 to 0)
pub type MKEY242_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY243` writer - Master key bit 243 (i = 31 to 0)
pub type MKEY243_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY244` writer - Master key bit 244 (i = 31 to 0)
pub type MKEY244_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY245` writer - Master key bit 245 (i = 31 to 0)
pub type MKEY245_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY246` writer - Master key bit 246 (i = 31 to 0)
pub type MKEY246_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY247` writer - Master key bit 247 (i = 31 to 0)
pub type MKEY247_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY248` writer - Master key bit 248 (i = 31 to 0)
pub type MKEY248_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY249` writer - Master key bit 249 (i = 31 to 0)
pub type MKEY249_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY250` writer - Master key bit 250 (i = 31 to 0)
pub type MKEY250_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY251` writer - Master key bit 251 (i = 31 to 0)
pub type MKEY251_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY252` writer - Master key bit 252 (i = 31 to 0)
pub type MKEY252_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY253` writer - Master key bit 253 (i = 31 to 0)
pub type MKEY253_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY254` writer - Master key bit 254 (i = 31 to 0)
pub type MKEY254_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY255` writer - Master key bit 255 (i = 31 to 0)
pub type MKEY255_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<MKEYR7rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Master key bit 224 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey224(&mut self) -> MKEY224_W<'_, MKEYR7rs> {
        MKEY224_W::new(self, 0)
    }
    ///Bit 1 - Master key bit 225 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey225(&mut self) -> MKEY225_W<'_, MKEYR7rs> {
        MKEY225_W::new(self, 1)
    }
    ///Bit 2 - Master key bit 226 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey226(&mut self) -> MKEY226_W<'_, MKEYR7rs> {
        MKEY226_W::new(self, 2)
    }
    ///Bit 3 - Master key bit 227 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey227(&mut self) -> MKEY227_W<'_, MKEYR7rs> {
        MKEY227_W::new(self, 3)
    }
    ///Bit 4 - Master key bit 228 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey228(&mut self) -> MKEY228_W<'_, MKEYR7rs> {
        MKEY228_W::new(self, 4)
    }
    ///Bit 5 - Master key bit 229 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey229(&mut self) -> MKEY229_W<'_, MKEYR7rs> {
        MKEY229_W::new(self, 5)
    }
    ///Bit 6 - Master key bit 230 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey230(&mut self) -> MKEY230_W<'_, MKEYR7rs> {
        MKEY230_W::new(self, 6)
    }
    ///Bit 7 - Master key bit 231 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey231(&mut self) -> MKEY231_W<'_, MKEYR7rs> {
        MKEY231_W::new(self, 7)
    }
    ///Bit 8 - Master key bit 232 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey232(&mut self) -> MKEY232_W<'_, MKEYR7rs> {
        MKEY232_W::new(self, 8)
    }
    ///Bit 9 - Master key bit 233 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey233(&mut self) -> MKEY233_W<'_, MKEYR7rs> {
        MKEY233_W::new(self, 9)
    }
    ///Bit 10 - Master key bit 234 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey234(&mut self) -> MKEY234_W<'_, MKEYR7rs> {
        MKEY234_W::new(self, 10)
    }
    ///Bit 11 - Master key bit 235 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey235(&mut self) -> MKEY235_W<'_, MKEYR7rs> {
        MKEY235_W::new(self, 11)
    }
    ///Bit 12 - Master key bit 236 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey236(&mut self) -> MKEY236_W<'_, MKEYR7rs> {
        MKEY236_W::new(self, 12)
    }
    ///Bit 13 - Master key bit 237 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey237(&mut self) -> MKEY237_W<'_, MKEYR7rs> {
        MKEY237_W::new(self, 13)
    }
    ///Bit 14 - Master key bit 238 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey238(&mut self) -> MKEY238_W<'_, MKEYR7rs> {
        MKEY238_W::new(self, 14)
    }
    ///Bit 15 - Master key bit 239 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey239(&mut self) -> MKEY239_W<'_, MKEYR7rs> {
        MKEY239_W::new(self, 15)
    }
    ///Bit 16 - Master key bit 240 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey240(&mut self) -> MKEY240_W<'_, MKEYR7rs> {
        MKEY240_W::new(self, 16)
    }
    ///Bit 17 - Master key bit 241 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey241(&mut self) -> MKEY241_W<'_, MKEYR7rs> {
        MKEY241_W::new(self, 17)
    }
    ///Bit 18 - Master key bit 242 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey242(&mut self) -> MKEY242_W<'_, MKEYR7rs> {
        MKEY242_W::new(self, 18)
    }
    ///Bit 19 - Master key bit 243 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey243(&mut self) -> MKEY243_W<'_, MKEYR7rs> {
        MKEY243_W::new(self, 19)
    }
    ///Bit 20 - Master key bit 244 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey244(&mut self) -> MKEY244_W<'_, MKEYR7rs> {
        MKEY244_W::new(self, 20)
    }
    ///Bit 21 - Master key bit 245 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey245(&mut self) -> MKEY245_W<'_, MKEYR7rs> {
        MKEY245_W::new(self, 21)
    }
    ///Bit 22 - Master key bit 246 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey246(&mut self) -> MKEY246_W<'_, MKEYR7rs> {
        MKEY246_W::new(self, 22)
    }
    ///Bit 23 - Master key bit 247 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey247(&mut self) -> MKEY247_W<'_, MKEYR7rs> {
        MKEY247_W::new(self, 23)
    }
    ///Bit 24 - Master key bit 248 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey248(&mut self) -> MKEY248_W<'_, MKEYR7rs> {
        MKEY248_W::new(self, 24)
    }
    ///Bit 25 - Master key bit 249 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey249(&mut self) -> MKEY249_W<'_, MKEYR7rs> {
        MKEY249_W::new(self, 25)
    }
    ///Bit 26 - Master key bit 250 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey250(&mut self) -> MKEY250_W<'_, MKEYR7rs> {
        MKEY250_W::new(self, 26)
    }
    ///Bit 27 - Master key bit 251 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey251(&mut self) -> MKEY251_W<'_, MKEYR7rs> {
        MKEY251_W::new(self, 27)
    }
    ///Bit 28 - Master key bit 252 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey252(&mut self) -> MKEY252_W<'_, MKEYR7rs> {
        MKEY252_W::new(self, 28)
    }
    ///Bit 29 - Master key bit 253 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey253(&mut self) -> MKEY253_W<'_, MKEYR7rs> {
        MKEY253_W::new(self, 29)
    }
    ///Bit 30 - Master key bit 254 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey254(&mut self) -> MKEY254_W<'_, MKEYR7rs> {
        MKEY254_W::new(self, 30)
    }
    ///Bit 31 - Master key bit 255 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey255(&mut self) -> MKEY255_W<'_, MKEYR7rs> {
        MKEY255_W::new(self, 31)
    }
}
/**.MCE master key 7

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mkeyr7::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#MCE1:MKEYR7)*/
pub struct MKEYR7rs;
impl crate::RegisterSpec for MKEYR7rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`mkeyr7::W`](W) writer structure
impl crate::Writable for MKEYR7rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MKEYR7 to value 0
impl crate::Resettable for MKEYR7rs {}
