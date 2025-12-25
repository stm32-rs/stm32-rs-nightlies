///Register `ICR4` writer
pub type W = crate::W<ICR4rs>;
///Field `IAF128` writer - illegal access flag clear for peripheral 128
pub type IAF128_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF129` writer - illegal access flag clear for peripheral 129
pub type IAF129_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF130` writer - illegal access flag clear for peripheral 130
pub type IAF130_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF131` writer - illegal access flag clear for peripheral 131
pub type IAF131_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF132` writer - illegal access flag clear for peripheral 132
pub type IAF132_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF133` writer - illegal access flag clear for peripheral 133
pub type IAF133_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF134` writer - illegal access flag clear for peripheral 134
pub type IAF134_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF135` writer - illegal access flag clear for peripheral 135
pub type IAF135_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF136` writer - illegal access flag clear for peripheral 136
pub type IAF136_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF137` writer - illegal access flag clear for peripheral 137
pub type IAF137_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF138` writer - illegal access flag clear for peripheral 138
pub type IAF138_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF139` writer - illegal access flag clear for peripheral 139
pub type IAF139_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF140` writer - illegal access flag clear for peripheral 140
pub type IAF140_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF141` writer - illegal access flag clear for peripheral 141
pub type IAF141_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF142` writer - illegal access flag clear for peripheral 142
pub type IAF142_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF143` writer - illegal access flag clear for peripheral 143
pub type IAF143_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF144` writer - illegal access flag clear for peripheral 144
pub type IAF144_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF145` writer - illegal access flag clear for peripheral 145
pub type IAF145_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF146` writer - illegal access flag clear for peripheral 146
pub type IAF146_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF147` writer - illegal access flag clear for peripheral 147
pub type IAF147_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF148` writer - illegal access flag clear for peripheral 148
pub type IAF148_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF149` writer - illegal access flag clear for peripheral 149
pub type IAF149_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF150` writer - illegal access flag clear for peripheral 150
pub type IAF150_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF151` writer - illegal access flag clear for peripheral 151
pub type IAF151_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF152` writer - illegal access flag clear for peripheral 152
pub type IAF152_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF153` writer - illegal access flag clear for peripheral 153
pub type IAF153_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF154` writer - illegal access flag clear for peripheral 154
pub type IAF154_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF155` writer - illegal access flag clear for peripheral 155
pub type IAF155_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF156` writer - illegal access flag clear for peripheral 156
pub type IAF156_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF157` writer - illegal access flag clear for peripheral 157
pub type IAF157_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF158` writer - illegal access flag clear for peripheral 158
pub type IAF158_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF159` writer - illegal access flag clear for peripheral 159
pub type IAF159_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<ICR4rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - illegal access flag clear for peripheral 128
    #[inline(always)]
    pub fn iaf128(&mut self) -> IAF128_W<'_, ICR4rs> {
        IAF128_W::new(self, 0)
    }
    ///Bit 1 - illegal access flag clear for peripheral 129
    #[inline(always)]
    pub fn iaf129(&mut self) -> IAF129_W<'_, ICR4rs> {
        IAF129_W::new(self, 1)
    }
    ///Bit 2 - illegal access flag clear for peripheral 130
    #[inline(always)]
    pub fn iaf130(&mut self) -> IAF130_W<'_, ICR4rs> {
        IAF130_W::new(self, 2)
    }
    ///Bit 3 - illegal access flag clear for peripheral 131
    #[inline(always)]
    pub fn iaf131(&mut self) -> IAF131_W<'_, ICR4rs> {
        IAF131_W::new(self, 3)
    }
    ///Bit 4 - illegal access flag clear for peripheral 132
    #[inline(always)]
    pub fn iaf132(&mut self) -> IAF132_W<'_, ICR4rs> {
        IAF132_W::new(self, 4)
    }
    ///Bit 5 - illegal access flag clear for peripheral 133
    #[inline(always)]
    pub fn iaf133(&mut self) -> IAF133_W<'_, ICR4rs> {
        IAF133_W::new(self, 5)
    }
    ///Bit 6 - illegal access flag clear for peripheral 134
    #[inline(always)]
    pub fn iaf134(&mut self) -> IAF134_W<'_, ICR4rs> {
        IAF134_W::new(self, 6)
    }
    ///Bit 7 - illegal access flag clear for peripheral 135
    #[inline(always)]
    pub fn iaf135(&mut self) -> IAF135_W<'_, ICR4rs> {
        IAF135_W::new(self, 7)
    }
    ///Bit 8 - illegal access flag clear for peripheral 136
    #[inline(always)]
    pub fn iaf136(&mut self) -> IAF136_W<'_, ICR4rs> {
        IAF136_W::new(self, 8)
    }
    ///Bit 9 - illegal access flag clear for peripheral 137
    #[inline(always)]
    pub fn iaf137(&mut self) -> IAF137_W<'_, ICR4rs> {
        IAF137_W::new(self, 9)
    }
    ///Bit 10 - illegal access flag clear for peripheral 138
    #[inline(always)]
    pub fn iaf138(&mut self) -> IAF138_W<'_, ICR4rs> {
        IAF138_W::new(self, 10)
    }
    ///Bit 11 - illegal access flag clear for peripheral 139
    #[inline(always)]
    pub fn iaf139(&mut self) -> IAF139_W<'_, ICR4rs> {
        IAF139_W::new(self, 11)
    }
    ///Bit 12 - illegal access flag clear for peripheral 140
    #[inline(always)]
    pub fn iaf140(&mut self) -> IAF140_W<'_, ICR4rs> {
        IAF140_W::new(self, 12)
    }
    ///Bit 13 - illegal access flag clear for peripheral 141
    #[inline(always)]
    pub fn iaf141(&mut self) -> IAF141_W<'_, ICR4rs> {
        IAF141_W::new(self, 13)
    }
    ///Bit 14 - illegal access flag clear for peripheral 142
    #[inline(always)]
    pub fn iaf142(&mut self) -> IAF142_W<'_, ICR4rs> {
        IAF142_W::new(self, 14)
    }
    ///Bit 15 - illegal access flag clear for peripheral 143
    #[inline(always)]
    pub fn iaf143(&mut self) -> IAF143_W<'_, ICR4rs> {
        IAF143_W::new(self, 15)
    }
    ///Bit 16 - illegal access flag clear for peripheral 144
    #[inline(always)]
    pub fn iaf144(&mut self) -> IAF144_W<'_, ICR4rs> {
        IAF144_W::new(self, 16)
    }
    ///Bit 17 - illegal access flag clear for peripheral 145
    #[inline(always)]
    pub fn iaf145(&mut self) -> IAF145_W<'_, ICR4rs> {
        IAF145_W::new(self, 17)
    }
    ///Bit 18 - illegal access flag clear for peripheral 146
    #[inline(always)]
    pub fn iaf146(&mut self) -> IAF146_W<'_, ICR4rs> {
        IAF146_W::new(self, 18)
    }
    ///Bit 19 - illegal access flag clear for peripheral 147
    #[inline(always)]
    pub fn iaf147(&mut self) -> IAF147_W<'_, ICR4rs> {
        IAF147_W::new(self, 19)
    }
    ///Bit 20 - illegal access flag clear for peripheral 148
    #[inline(always)]
    pub fn iaf148(&mut self) -> IAF148_W<'_, ICR4rs> {
        IAF148_W::new(self, 20)
    }
    ///Bit 21 - illegal access flag clear for peripheral 149
    #[inline(always)]
    pub fn iaf149(&mut self) -> IAF149_W<'_, ICR4rs> {
        IAF149_W::new(self, 21)
    }
    ///Bit 22 - illegal access flag clear for peripheral 150
    #[inline(always)]
    pub fn iaf150(&mut self) -> IAF150_W<'_, ICR4rs> {
        IAF150_W::new(self, 22)
    }
    ///Bit 23 - illegal access flag clear for peripheral 151
    #[inline(always)]
    pub fn iaf151(&mut self) -> IAF151_W<'_, ICR4rs> {
        IAF151_W::new(self, 23)
    }
    ///Bit 24 - illegal access flag clear for peripheral 152
    #[inline(always)]
    pub fn iaf152(&mut self) -> IAF152_W<'_, ICR4rs> {
        IAF152_W::new(self, 24)
    }
    ///Bit 25 - illegal access flag clear for peripheral 153
    #[inline(always)]
    pub fn iaf153(&mut self) -> IAF153_W<'_, ICR4rs> {
        IAF153_W::new(self, 25)
    }
    ///Bit 26 - illegal access flag clear for peripheral 154
    #[inline(always)]
    pub fn iaf154(&mut self) -> IAF154_W<'_, ICR4rs> {
        IAF154_W::new(self, 26)
    }
    ///Bit 27 - illegal access flag clear for peripheral 155
    #[inline(always)]
    pub fn iaf155(&mut self) -> IAF155_W<'_, ICR4rs> {
        IAF155_W::new(self, 27)
    }
    ///Bit 28 - illegal access flag clear for peripheral 156
    #[inline(always)]
    pub fn iaf156(&mut self) -> IAF156_W<'_, ICR4rs> {
        IAF156_W::new(self, 28)
    }
    ///Bit 29 - illegal access flag clear for peripheral 157
    #[inline(always)]
    pub fn iaf157(&mut self) -> IAF157_W<'_, ICR4rs> {
        IAF157_W::new(self, 29)
    }
    ///Bit 30 - illegal access flag clear for peripheral 158
    #[inline(always)]
    pub fn iaf158(&mut self) -> IAF158_W<'_, ICR4rs> {
        IAF158_W::new(self, 30)
    }
    ///Bit 31 - illegal access flag clear for peripheral 159
    #[inline(always)]
    pub fn iaf159(&mut self) -> IAF159_W<'_, ICR4rs> {
        IAF159_W::new(self, 31)
    }
}
/**IAC interrupt clear register 4

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr4::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#IAC:ICR4)*/
pub struct ICR4rs;
impl crate::RegisterSpec for ICR4rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`icr4::W`](W) writer structure
impl crate::Writable for ICR4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICR4 to value 0
impl crate::Resettable for ICR4rs {}
