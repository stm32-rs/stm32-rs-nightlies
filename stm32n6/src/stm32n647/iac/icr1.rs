///Register `ICR1` writer
pub type W = crate::W<ICR1rs>;
///Field `IAF32` writer - illegal access flag clear for peripheral 32
pub type IAF32_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF33` writer - illegal access flag clear for peripheral 33
pub type IAF33_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF34` writer - illegal access flag clear for peripheral 34
pub type IAF34_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF35` writer - illegal access flag clear for peripheral 35
pub type IAF35_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF36` writer - illegal access flag clear for peripheral 36
pub type IAF36_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF37` writer - illegal access flag clear for peripheral 37
pub type IAF37_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF38` writer - illegal access flag clear for peripheral 38
pub type IAF38_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF39` writer - illegal access flag clear for peripheral 39
pub type IAF39_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF40` writer - illegal access flag clear for peripheral 40
pub type IAF40_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF41` writer - illegal access flag clear for peripheral 41
pub type IAF41_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF42` writer - illegal access flag clear for peripheral 42
pub type IAF42_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF43` writer - illegal access flag clear for peripheral 43
pub type IAF43_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF44` writer - illegal access flag clear for peripheral 44
pub type IAF44_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF45` writer - illegal access flag clear for peripheral 45
pub type IAF45_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF46` writer - illegal access flag clear for peripheral 46
pub type IAF46_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF47` writer - illegal access flag clear for peripheral 47
pub type IAF47_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF48` writer - illegal access flag clear for peripheral 48
pub type IAF48_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF49` writer - illegal access flag clear for peripheral 49
pub type IAF49_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF50` writer - illegal access flag clear for peripheral 50
pub type IAF50_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF51` writer - illegal access flag clear for peripheral 51
pub type IAF51_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF52` writer - illegal access flag clear for peripheral 52
pub type IAF52_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF53` writer - illegal access flag clear for peripheral 53
pub type IAF53_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF54` writer - illegal access flag clear for peripheral 54
pub type IAF54_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF55` writer - illegal access flag clear for peripheral 55
pub type IAF55_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF56` writer - illegal access flag clear for peripheral 56
pub type IAF56_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF57` writer - illegal access flag clear for peripheral 57
pub type IAF57_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF58` writer - illegal access flag clear for peripheral 58
pub type IAF58_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF59` writer - illegal access flag clear for peripheral 59
pub type IAF59_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF60` writer - illegal access flag clear for peripheral 60
pub type IAF60_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF61` writer - illegal access flag clear for peripheral 61
pub type IAF61_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF62` writer - illegal access flag clear for peripheral 62
pub type IAF62_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF63` writer - illegal access flag clear for peripheral 63
pub type IAF63_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<ICR1rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - illegal access flag clear for peripheral 32
    #[inline(always)]
    pub fn iaf32(&mut self) -> IAF32_W<'_, ICR1rs> {
        IAF32_W::new(self, 0)
    }
    ///Bit 1 - illegal access flag clear for peripheral 33
    #[inline(always)]
    pub fn iaf33(&mut self) -> IAF33_W<'_, ICR1rs> {
        IAF33_W::new(self, 1)
    }
    ///Bit 2 - illegal access flag clear for peripheral 34
    #[inline(always)]
    pub fn iaf34(&mut self) -> IAF34_W<'_, ICR1rs> {
        IAF34_W::new(self, 2)
    }
    ///Bit 3 - illegal access flag clear for peripheral 35
    #[inline(always)]
    pub fn iaf35(&mut self) -> IAF35_W<'_, ICR1rs> {
        IAF35_W::new(self, 3)
    }
    ///Bit 4 - illegal access flag clear for peripheral 36
    #[inline(always)]
    pub fn iaf36(&mut self) -> IAF36_W<'_, ICR1rs> {
        IAF36_W::new(self, 4)
    }
    ///Bit 5 - illegal access flag clear for peripheral 37
    #[inline(always)]
    pub fn iaf37(&mut self) -> IAF37_W<'_, ICR1rs> {
        IAF37_W::new(self, 5)
    }
    ///Bit 6 - illegal access flag clear for peripheral 38
    #[inline(always)]
    pub fn iaf38(&mut self) -> IAF38_W<'_, ICR1rs> {
        IAF38_W::new(self, 6)
    }
    ///Bit 7 - illegal access flag clear for peripheral 39
    #[inline(always)]
    pub fn iaf39(&mut self) -> IAF39_W<'_, ICR1rs> {
        IAF39_W::new(self, 7)
    }
    ///Bit 8 - illegal access flag clear for peripheral 40
    #[inline(always)]
    pub fn iaf40(&mut self) -> IAF40_W<'_, ICR1rs> {
        IAF40_W::new(self, 8)
    }
    ///Bit 9 - illegal access flag clear for peripheral 41
    #[inline(always)]
    pub fn iaf41(&mut self) -> IAF41_W<'_, ICR1rs> {
        IAF41_W::new(self, 9)
    }
    ///Bit 10 - illegal access flag clear for peripheral 42
    #[inline(always)]
    pub fn iaf42(&mut self) -> IAF42_W<'_, ICR1rs> {
        IAF42_W::new(self, 10)
    }
    ///Bit 11 - illegal access flag clear for peripheral 43
    #[inline(always)]
    pub fn iaf43(&mut self) -> IAF43_W<'_, ICR1rs> {
        IAF43_W::new(self, 11)
    }
    ///Bit 12 - illegal access flag clear for peripheral 44
    #[inline(always)]
    pub fn iaf44(&mut self) -> IAF44_W<'_, ICR1rs> {
        IAF44_W::new(self, 12)
    }
    ///Bit 13 - illegal access flag clear for peripheral 45
    #[inline(always)]
    pub fn iaf45(&mut self) -> IAF45_W<'_, ICR1rs> {
        IAF45_W::new(self, 13)
    }
    ///Bit 14 - illegal access flag clear for peripheral 46
    #[inline(always)]
    pub fn iaf46(&mut self) -> IAF46_W<'_, ICR1rs> {
        IAF46_W::new(self, 14)
    }
    ///Bit 15 - illegal access flag clear for peripheral 47
    #[inline(always)]
    pub fn iaf47(&mut self) -> IAF47_W<'_, ICR1rs> {
        IAF47_W::new(self, 15)
    }
    ///Bit 16 - illegal access flag clear for peripheral 48
    #[inline(always)]
    pub fn iaf48(&mut self) -> IAF48_W<'_, ICR1rs> {
        IAF48_W::new(self, 16)
    }
    ///Bit 17 - illegal access flag clear for peripheral 49
    #[inline(always)]
    pub fn iaf49(&mut self) -> IAF49_W<'_, ICR1rs> {
        IAF49_W::new(self, 17)
    }
    ///Bit 18 - illegal access flag clear for peripheral 50
    #[inline(always)]
    pub fn iaf50(&mut self) -> IAF50_W<'_, ICR1rs> {
        IAF50_W::new(self, 18)
    }
    ///Bit 19 - illegal access flag clear for peripheral 51
    #[inline(always)]
    pub fn iaf51(&mut self) -> IAF51_W<'_, ICR1rs> {
        IAF51_W::new(self, 19)
    }
    ///Bit 20 - illegal access flag clear for peripheral 52
    #[inline(always)]
    pub fn iaf52(&mut self) -> IAF52_W<'_, ICR1rs> {
        IAF52_W::new(self, 20)
    }
    ///Bit 21 - illegal access flag clear for peripheral 53
    #[inline(always)]
    pub fn iaf53(&mut self) -> IAF53_W<'_, ICR1rs> {
        IAF53_W::new(self, 21)
    }
    ///Bit 22 - illegal access flag clear for peripheral 54
    #[inline(always)]
    pub fn iaf54(&mut self) -> IAF54_W<'_, ICR1rs> {
        IAF54_W::new(self, 22)
    }
    ///Bit 23 - illegal access flag clear for peripheral 55
    #[inline(always)]
    pub fn iaf55(&mut self) -> IAF55_W<'_, ICR1rs> {
        IAF55_W::new(self, 23)
    }
    ///Bit 24 - illegal access flag clear for peripheral 56
    #[inline(always)]
    pub fn iaf56(&mut self) -> IAF56_W<'_, ICR1rs> {
        IAF56_W::new(self, 24)
    }
    ///Bit 25 - illegal access flag clear for peripheral 57
    #[inline(always)]
    pub fn iaf57(&mut self) -> IAF57_W<'_, ICR1rs> {
        IAF57_W::new(self, 25)
    }
    ///Bit 26 - illegal access flag clear for peripheral 58
    #[inline(always)]
    pub fn iaf58(&mut self) -> IAF58_W<'_, ICR1rs> {
        IAF58_W::new(self, 26)
    }
    ///Bit 27 - illegal access flag clear for peripheral 59
    #[inline(always)]
    pub fn iaf59(&mut self) -> IAF59_W<'_, ICR1rs> {
        IAF59_W::new(self, 27)
    }
    ///Bit 28 - illegal access flag clear for peripheral 60
    #[inline(always)]
    pub fn iaf60(&mut self) -> IAF60_W<'_, ICR1rs> {
        IAF60_W::new(self, 28)
    }
    ///Bit 29 - illegal access flag clear for peripheral 61
    #[inline(always)]
    pub fn iaf61(&mut self) -> IAF61_W<'_, ICR1rs> {
        IAF61_W::new(self, 29)
    }
    ///Bit 30 - illegal access flag clear for peripheral 62
    #[inline(always)]
    pub fn iaf62(&mut self) -> IAF62_W<'_, ICR1rs> {
        IAF62_W::new(self, 30)
    }
    ///Bit 31 - illegal access flag clear for peripheral 63
    #[inline(always)]
    pub fn iaf63(&mut self) -> IAF63_W<'_, ICR1rs> {
        IAF63_W::new(self, 31)
    }
}
/**IAC interrupt clear register 1

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#IAC:ICR1)*/
pub struct ICR1rs;
impl crate::RegisterSpec for ICR1rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`icr1::W`](W) writer structure
impl crate::Writable for ICR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICR1 to value 0
impl crate::Resettable for ICR1rs {}
