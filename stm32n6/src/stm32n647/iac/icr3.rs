///Register `ICR3` writer
pub type W = crate::W<ICR3rs>;
///Field `IAF96` writer - illegal access flag clear for peripheral 96
pub type IAF96_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF97` writer - illegal access flag clear for peripheral 97
pub type IAF97_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF98` writer - illegal access flag clear for peripheral 98
pub type IAF98_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF99` writer - illegal access flag clear for peripheral 99
pub type IAF99_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF100` writer - illegal access flag clear for peripheral 100
pub type IAF100_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF101` writer - illegal access flag clear for peripheral 101
pub type IAF101_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF102` writer - illegal access flag clear for peripheral 102
pub type IAF102_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF103` writer - illegal access flag clear for peripheral 103
pub type IAF103_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF104` writer - illegal access flag clear for peripheral 104
pub type IAF104_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF105` writer - illegal access flag clear for peripheral 105
pub type IAF105_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF106` writer - illegal access flag clear for peripheral 106
pub type IAF106_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF107` writer - illegal access flag clear for peripheral 107
pub type IAF107_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF108` writer - illegal access flag clear for peripheral 108
pub type IAF108_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF109` writer - illegal access flag clear for peripheral 109
pub type IAF109_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF110` writer - illegal access flag clear for peripheral 110
pub type IAF110_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF111` writer - illegal access flag clear for peripheral 111
pub type IAF111_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF112` writer - illegal access flag clear for peripheral 112
pub type IAF112_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF113` writer - illegal access flag clear for peripheral 113
pub type IAF113_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF114` writer - illegal access flag clear for peripheral 114
pub type IAF114_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF115` writer - illegal access flag clear for peripheral 115
pub type IAF115_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF116` writer - illegal access flag clear for peripheral 116
pub type IAF116_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF117` writer - illegal access flag clear for peripheral 117
pub type IAF117_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF118` writer - illegal access flag clear for peripheral 118
pub type IAF118_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF119` writer - illegal access flag clear for peripheral 119
pub type IAF119_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF120` writer - illegal access flag clear for peripheral 120
pub type IAF120_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF121` writer - illegal access flag clear for peripheral 121
pub type IAF121_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF122` writer - illegal access flag clear for peripheral 122
pub type IAF122_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF123` writer - illegal access flag clear for peripheral 123
pub type IAF123_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF124` writer - illegal access flag clear for peripheral 124
pub type IAF124_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF125` writer - illegal access flag clear for peripheral 125
pub type IAF125_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF126` writer - illegal access flag clear for peripheral 126
pub type IAF126_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF127` writer - illegal access flag clear for peripheral 127
pub type IAF127_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<ICR3rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - illegal access flag clear for peripheral 96
    #[inline(always)]
    pub fn iaf96(&mut self) -> IAF96_W<'_, ICR3rs> {
        IAF96_W::new(self, 0)
    }
    ///Bit 1 - illegal access flag clear for peripheral 97
    #[inline(always)]
    pub fn iaf97(&mut self) -> IAF97_W<'_, ICR3rs> {
        IAF97_W::new(self, 1)
    }
    ///Bit 2 - illegal access flag clear for peripheral 98
    #[inline(always)]
    pub fn iaf98(&mut self) -> IAF98_W<'_, ICR3rs> {
        IAF98_W::new(self, 2)
    }
    ///Bit 3 - illegal access flag clear for peripheral 99
    #[inline(always)]
    pub fn iaf99(&mut self) -> IAF99_W<'_, ICR3rs> {
        IAF99_W::new(self, 3)
    }
    ///Bit 4 - illegal access flag clear for peripheral 100
    #[inline(always)]
    pub fn iaf100(&mut self) -> IAF100_W<'_, ICR3rs> {
        IAF100_W::new(self, 4)
    }
    ///Bit 5 - illegal access flag clear for peripheral 101
    #[inline(always)]
    pub fn iaf101(&mut self) -> IAF101_W<'_, ICR3rs> {
        IAF101_W::new(self, 5)
    }
    ///Bit 6 - illegal access flag clear for peripheral 102
    #[inline(always)]
    pub fn iaf102(&mut self) -> IAF102_W<'_, ICR3rs> {
        IAF102_W::new(self, 6)
    }
    ///Bit 7 - illegal access flag clear for peripheral 103
    #[inline(always)]
    pub fn iaf103(&mut self) -> IAF103_W<'_, ICR3rs> {
        IAF103_W::new(self, 7)
    }
    ///Bit 8 - illegal access flag clear for peripheral 104
    #[inline(always)]
    pub fn iaf104(&mut self) -> IAF104_W<'_, ICR3rs> {
        IAF104_W::new(self, 8)
    }
    ///Bit 9 - illegal access flag clear for peripheral 105
    #[inline(always)]
    pub fn iaf105(&mut self) -> IAF105_W<'_, ICR3rs> {
        IAF105_W::new(self, 9)
    }
    ///Bit 10 - illegal access flag clear for peripheral 106
    #[inline(always)]
    pub fn iaf106(&mut self) -> IAF106_W<'_, ICR3rs> {
        IAF106_W::new(self, 10)
    }
    ///Bit 11 - illegal access flag clear for peripheral 107
    #[inline(always)]
    pub fn iaf107(&mut self) -> IAF107_W<'_, ICR3rs> {
        IAF107_W::new(self, 11)
    }
    ///Bit 12 - illegal access flag clear for peripheral 108
    #[inline(always)]
    pub fn iaf108(&mut self) -> IAF108_W<'_, ICR3rs> {
        IAF108_W::new(self, 12)
    }
    ///Bit 13 - illegal access flag clear for peripheral 109
    #[inline(always)]
    pub fn iaf109(&mut self) -> IAF109_W<'_, ICR3rs> {
        IAF109_W::new(self, 13)
    }
    ///Bit 14 - illegal access flag clear for peripheral 110
    #[inline(always)]
    pub fn iaf110(&mut self) -> IAF110_W<'_, ICR3rs> {
        IAF110_W::new(self, 14)
    }
    ///Bit 15 - illegal access flag clear for peripheral 111
    #[inline(always)]
    pub fn iaf111(&mut self) -> IAF111_W<'_, ICR3rs> {
        IAF111_W::new(self, 15)
    }
    ///Bit 16 - illegal access flag clear for peripheral 112
    #[inline(always)]
    pub fn iaf112(&mut self) -> IAF112_W<'_, ICR3rs> {
        IAF112_W::new(self, 16)
    }
    ///Bit 17 - illegal access flag clear for peripheral 113
    #[inline(always)]
    pub fn iaf113(&mut self) -> IAF113_W<'_, ICR3rs> {
        IAF113_W::new(self, 17)
    }
    ///Bit 18 - illegal access flag clear for peripheral 114
    #[inline(always)]
    pub fn iaf114(&mut self) -> IAF114_W<'_, ICR3rs> {
        IAF114_W::new(self, 18)
    }
    ///Bit 19 - illegal access flag clear for peripheral 115
    #[inline(always)]
    pub fn iaf115(&mut self) -> IAF115_W<'_, ICR3rs> {
        IAF115_W::new(self, 19)
    }
    ///Bit 20 - illegal access flag clear for peripheral 116
    #[inline(always)]
    pub fn iaf116(&mut self) -> IAF116_W<'_, ICR3rs> {
        IAF116_W::new(self, 20)
    }
    ///Bit 21 - illegal access flag clear for peripheral 117
    #[inline(always)]
    pub fn iaf117(&mut self) -> IAF117_W<'_, ICR3rs> {
        IAF117_W::new(self, 21)
    }
    ///Bit 22 - illegal access flag clear for peripheral 118
    #[inline(always)]
    pub fn iaf118(&mut self) -> IAF118_W<'_, ICR3rs> {
        IAF118_W::new(self, 22)
    }
    ///Bit 23 - illegal access flag clear for peripheral 119
    #[inline(always)]
    pub fn iaf119(&mut self) -> IAF119_W<'_, ICR3rs> {
        IAF119_W::new(self, 23)
    }
    ///Bit 24 - illegal access flag clear for peripheral 120
    #[inline(always)]
    pub fn iaf120(&mut self) -> IAF120_W<'_, ICR3rs> {
        IAF120_W::new(self, 24)
    }
    ///Bit 25 - illegal access flag clear for peripheral 121
    #[inline(always)]
    pub fn iaf121(&mut self) -> IAF121_W<'_, ICR3rs> {
        IAF121_W::new(self, 25)
    }
    ///Bit 26 - illegal access flag clear for peripheral 122
    #[inline(always)]
    pub fn iaf122(&mut self) -> IAF122_W<'_, ICR3rs> {
        IAF122_W::new(self, 26)
    }
    ///Bit 27 - illegal access flag clear for peripheral 123
    #[inline(always)]
    pub fn iaf123(&mut self) -> IAF123_W<'_, ICR3rs> {
        IAF123_W::new(self, 27)
    }
    ///Bit 28 - illegal access flag clear for peripheral 124
    #[inline(always)]
    pub fn iaf124(&mut self) -> IAF124_W<'_, ICR3rs> {
        IAF124_W::new(self, 28)
    }
    ///Bit 29 - illegal access flag clear for peripheral 125
    #[inline(always)]
    pub fn iaf125(&mut self) -> IAF125_W<'_, ICR3rs> {
        IAF125_W::new(self, 29)
    }
    ///Bit 30 - illegal access flag clear for peripheral 126
    #[inline(always)]
    pub fn iaf126(&mut self) -> IAF126_W<'_, ICR3rs> {
        IAF126_W::new(self, 30)
    }
    ///Bit 31 - illegal access flag clear for peripheral 127
    #[inline(always)]
    pub fn iaf127(&mut self) -> IAF127_W<'_, ICR3rs> {
        IAF127_W::new(self, 31)
    }
}
/**IAC interrupt clear register 3

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr3::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#IAC:ICR3)*/
pub struct ICR3rs;
impl crate::RegisterSpec for ICR3rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`icr3::W`](W) writer structure
impl crate::Writable for ICR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICR3 to value 0
impl crate::Resettable for ICR3rs {}
