///Register `FMKEYR4` writer
pub type W = crate::W<FMKEYR4rs>;
///Field `FMKEY128` writer - Fast master key bit 128 (i = 31 to 0)
pub type FMKEY128_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY129` writer - Fast master key bit 129 (i = 31 to 0)
pub type FMKEY129_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY130` writer - Fast master key bit 130 (i = 31 to 0)
pub type FMKEY130_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY131` writer - Fast master key bit 131 (i = 31 to 0)
pub type FMKEY131_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY132` writer - Fast master key bit 132 (i = 31 to 0)
pub type FMKEY132_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY133` writer - Fast master key bit 133 (i = 31 to 0)
pub type FMKEY133_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY134` writer - Fast master key bit 134 (i = 31 to 0)
pub type FMKEY134_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY135` writer - Fast master key bit 135 (i = 31 to 0)
pub type FMKEY135_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY136` writer - Fast master key bit 136 (i = 31 to 0)
pub type FMKEY136_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY137` writer - Fast master key bit 137 (i = 31 to 0)
pub type FMKEY137_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY138` writer - Fast master key bit 138 (i = 31 to 0)
pub type FMKEY138_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY139` writer - Fast master key bit 139 (i = 31 to 0)
pub type FMKEY139_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY140` writer - Fast master key bit 140 (i = 31 to 0)
pub type FMKEY140_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY141` writer - Fast master key bit 141 (i = 31 to 0)
pub type FMKEY141_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY142` writer - Fast master key bit 142 (i = 31 to 0)
pub type FMKEY142_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY143` writer - Fast master key bit 143 (i = 31 to 0)
pub type FMKEY143_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY144` writer - Fast master key bit 144 (i = 31 to 0)
pub type FMKEY144_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY145` writer - Fast master key bit 145 (i = 31 to 0)
pub type FMKEY145_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY146` writer - Fast master key bit 146 (i = 31 to 0)
pub type FMKEY146_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY147` writer - Fast master key bit 147 (i = 31 to 0)
pub type FMKEY147_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY148` writer - Fast master key bit 148 (i = 31 to 0)
pub type FMKEY148_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY149` writer - Fast master key bit 149 (i = 31 to 0)
pub type FMKEY149_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY150` writer - Fast master key bit 150 (i = 31 to 0)
pub type FMKEY150_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY151` writer - Fast master key bit 151 (i = 31 to 0)
pub type FMKEY151_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY152` writer - Fast master key bit 152 (i = 31 to 0)
pub type FMKEY152_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY153` writer - Fast master key bit 153 (i = 31 to 0)
pub type FMKEY153_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY154` writer - Fast master key bit 154 (i = 31 to 0)
pub type FMKEY154_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY155` writer - Fast master key bit 155 (i = 31 to 0)
pub type FMKEY155_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY156` writer - Fast master key bit 156 (i = 31 to 0)
pub type FMKEY156_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY157` writer - Fast master key bit 157 (i = 31 to 0)
pub type FMKEY157_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY158` writer - Fast master key bit 158 (i = 31 to 0)
pub type FMKEY158_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY159` writer - Fast master key bit 159 (i = 31 to 0)
pub type FMKEY159_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<FMKEYR4rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Fast master key bit 128 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey128(&mut self) -> FMKEY128_W<'_, FMKEYR4rs> {
        FMKEY128_W::new(self, 0)
    }
    ///Bit 1 - Fast master key bit 129 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey129(&mut self) -> FMKEY129_W<'_, FMKEYR4rs> {
        FMKEY129_W::new(self, 1)
    }
    ///Bit 2 - Fast master key bit 130 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey130(&mut self) -> FMKEY130_W<'_, FMKEYR4rs> {
        FMKEY130_W::new(self, 2)
    }
    ///Bit 3 - Fast master key bit 131 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey131(&mut self) -> FMKEY131_W<'_, FMKEYR4rs> {
        FMKEY131_W::new(self, 3)
    }
    ///Bit 4 - Fast master key bit 132 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey132(&mut self) -> FMKEY132_W<'_, FMKEYR4rs> {
        FMKEY132_W::new(self, 4)
    }
    ///Bit 5 - Fast master key bit 133 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey133(&mut self) -> FMKEY133_W<'_, FMKEYR4rs> {
        FMKEY133_W::new(self, 5)
    }
    ///Bit 6 - Fast master key bit 134 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey134(&mut self) -> FMKEY134_W<'_, FMKEYR4rs> {
        FMKEY134_W::new(self, 6)
    }
    ///Bit 7 - Fast master key bit 135 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey135(&mut self) -> FMKEY135_W<'_, FMKEYR4rs> {
        FMKEY135_W::new(self, 7)
    }
    ///Bit 8 - Fast master key bit 136 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey136(&mut self) -> FMKEY136_W<'_, FMKEYR4rs> {
        FMKEY136_W::new(self, 8)
    }
    ///Bit 9 - Fast master key bit 137 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey137(&mut self) -> FMKEY137_W<'_, FMKEYR4rs> {
        FMKEY137_W::new(self, 9)
    }
    ///Bit 10 - Fast master key bit 138 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey138(&mut self) -> FMKEY138_W<'_, FMKEYR4rs> {
        FMKEY138_W::new(self, 10)
    }
    ///Bit 11 - Fast master key bit 139 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey139(&mut self) -> FMKEY139_W<'_, FMKEYR4rs> {
        FMKEY139_W::new(self, 11)
    }
    ///Bit 12 - Fast master key bit 140 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey140(&mut self) -> FMKEY140_W<'_, FMKEYR4rs> {
        FMKEY140_W::new(self, 12)
    }
    ///Bit 13 - Fast master key bit 141 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey141(&mut self) -> FMKEY141_W<'_, FMKEYR4rs> {
        FMKEY141_W::new(self, 13)
    }
    ///Bit 14 - Fast master key bit 142 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey142(&mut self) -> FMKEY142_W<'_, FMKEYR4rs> {
        FMKEY142_W::new(self, 14)
    }
    ///Bit 15 - Fast master key bit 143 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey143(&mut self) -> FMKEY143_W<'_, FMKEYR4rs> {
        FMKEY143_W::new(self, 15)
    }
    ///Bit 16 - Fast master key bit 144 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey144(&mut self) -> FMKEY144_W<'_, FMKEYR4rs> {
        FMKEY144_W::new(self, 16)
    }
    ///Bit 17 - Fast master key bit 145 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey145(&mut self) -> FMKEY145_W<'_, FMKEYR4rs> {
        FMKEY145_W::new(self, 17)
    }
    ///Bit 18 - Fast master key bit 146 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey146(&mut self) -> FMKEY146_W<'_, FMKEYR4rs> {
        FMKEY146_W::new(self, 18)
    }
    ///Bit 19 - Fast master key bit 147 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey147(&mut self) -> FMKEY147_W<'_, FMKEYR4rs> {
        FMKEY147_W::new(self, 19)
    }
    ///Bit 20 - Fast master key bit 148 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey148(&mut self) -> FMKEY148_W<'_, FMKEYR4rs> {
        FMKEY148_W::new(self, 20)
    }
    ///Bit 21 - Fast master key bit 149 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey149(&mut self) -> FMKEY149_W<'_, FMKEYR4rs> {
        FMKEY149_W::new(self, 21)
    }
    ///Bit 22 - Fast master key bit 150 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey150(&mut self) -> FMKEY150_W<'_, FMKEYR4rs> {
        FMKEY150_W::new(self, 22)
    }
    ///Bit 23 - Fast master key bit 151 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey151(&mut self) -> FMKEY151_W<'_, FMKEYR4rs> {
        FMKEY151_W::new(self, 23)
    }
    ///Bit 24 - Fast master key bit 152 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey152(&mut self) -> FMKEY152_W<'_, FMKEYR4rs> {
        FMKEY152_W::new(self, 24)
    }
    ///Bit 25 - Fast master key bit 153 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey153(&mut self) -> FMKEY153_W<'_, FMKEYR4rs> {
        FMKEY153_W::new(self, 25)
    }
    ///Bit 26 - Fast master key bit 154 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey154(&mut self) -> FMKEY154_W<'_, FMKEYR4rs> {
        FMKEY154_W::new(self, 26)
    }
    ///Bit 27 - Fast master key bit 155 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey155(&mut self) -> FMKEY155_W<'_, FMKEYR4rs> {
        FMKEY155_W::new(self, 27)
    }
    ///Bit 28 - Fast master key bit 156 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey156(&mut self) -> FMKEY156_W<'_, FMKEYR4rs> {
        FMKEY156_W::new(self, 28)
    }
    ///Bit 29 - Fast master key bit 157 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey157(&mut self) -> FMKEY157_W<'_, FMKEYR4rs> {
        FMKEY157_W::new(self, 29)
    }
    ///Bit 30 - Fast master key bit 158 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey158(&mut self) -> FMKEY158_W<'_, FMKEYR4rs> {
        FMKEY158_W::new(self, 30)
    }
    ///Bit 31 - Fast master key bit 159 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey159(&mut self) -> FMKEY159_W<'_, FMKEYR4rs> {
        FMKEY159_W::new(self, 31)
    }
}
/**MCE fast master key 4

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmkeyr4::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#MCE1:FMKEYR4)*/
pub struct FMKEYR4rs;
impl crate::RegisterSpec for FMKEYR4rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`fmkeyr4::W`](W) writer structure
impl crate::Writable for FMKEYR4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FMKEYR4 to value 0
impl crate::Resettable for FMKEYR4rs {}
