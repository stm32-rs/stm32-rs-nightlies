///Register `MKEYR4` writer
pub type W = crate::W<MKEYR4rs>;
///Field `MKEY128` writer - Master key bit 128 (i = 31 to 0)
pub type MKEY128_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY129` writer - Master key bit 129 (i = 31 to 0)
pub type MKEY129_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY130` writer - Master key bit 130 (i = 31 to 0)
pub type MKEY130_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY131` writer - Master key bit 131 (i = 31 to 0)
pub type MKEY131_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY132` writer - Master key bit 132 (i = 31 to 0)
pub type MKEY132_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY133` writer - Master key bit 133 (i = 31 to 0)
pub type MKEY133_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY134` writer - Master key bit 134 (i = 31 to 0)
pub type MKEY134_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY135` writer - Master key bit 135 (i = 31 to 0)
pub type MKEY135_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY136` writer - Master key bit 136 (i = 31 to 0)
pub type MKEY136_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY137` writer - Master key bit 137 (i = 31 to 0)
pub type MKEY137_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY138` writer - Master key bit 138 (i = 31 to 0)
pub type MKEY138_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY139` writer - Master key bit 139 (i = 31 to 0)
pub type MKEY139_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY140` writer - Master key bit 140 (i = 31 to 0)
pub type MKEY140_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY141` writer - Master key bit 141 (i = 31 to 0)
pub type MKEY141_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY142` writer - Master key bit 142 (i = 31 to 0)
pub type MKEY142_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY143` writer - Master key bit 143 (i = 31 to 0)
pub type MKEY143_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY144` writer - Master key bit 144 (i = 31 to 0)
pub type MKEY144_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY145` writer - Master key bit 145 (i = 31 to 0)
pub type MKEY145_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY146` writer - Master key bit 146 (i = 31 to 0)
pub type MKEY146_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY147` writer - Master key bit 147 (i = 31 to 0)
pub type MKEY147_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY148` writer - Master key bit 148 (i = 31 to 0)
pub type MKEY148_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY149` writer - Master key bit 149 (i = 31 to 0)
pub type MKEY149_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY150` writer - Master key bit 150 (i = 31 to 0)
pub type MKEY150_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY151` writer - Master key bit 151 (i = 31 to 0)
pub type MKEY151_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY152` writer - Master key bit 152 (i = 31 to 0)
pub type MKEY152_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY153` writer - Master key bit 153 (i = 31 to 0)
pub type MKEY153_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY154` writer - Master key bit 154 (i = 31 to 0)
pub type MKEY154_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY155` writer - Master key bit 155 (i = 31 to 0)
pub type MKEY155_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY156` writer - Master key bit 156 (i = 31 to 0)
pub type MKEY156_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY157` writer - Master key bit 157 (i = 31 to 0)
pub type MKEY157_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY158` writer - Master key bit 158 (i = 31 to 0)
pub type MKEY158_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY159` writer - Master key bit 159 (i = 31 to 0)
pub type MKEY159_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<MKEYR4rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Master key bit 128 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey128(&mut self) -> MKEY128_W<'_, MKEYR4rs> {
        MKEY128_W::new(self, 0)
    }
    ///Bit 1 - Master key bit 129 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey129(&mut self) -> MKEY129_W<'_, MKEYR4rs> {
        MKEY129_W::new(self, 1)
    }
    ///Bit 2 - Master key bit 130 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey130(&mut self) -> MKEY130_W<'_, MKEYR4rs> {
        MKEY130_W::new(self, 2)
    }
    ///Bit 3 - Master key bit 131 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey131(&mut self) -> MKEY131_W<'_, MKEYR4rs> {
        MKEY131_W::new(self, 3)
    }
    ///Bit 4 - Master key bit 132 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey132(&mut self) -> MKEY132_W<'_, MKEYR4rs> {
        MKEY132_W::new(self, 4)
    }
    ///Bit 5 - Master key bit 133 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey133(&mut self) -> MKEY133_W<'_, MKEYR4rs> {
        MKEY133_W::new(self, 5)
    }
    ///Bit 6 - Master key bit 134 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey134(&mut self) -> MKEY134_W<'_, MKEYR4rs> {
        MKEY134_W::new(self, 6)
    }
    ///Bit 7 - Master key bit 135 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey135(&mut self) -> MKEY135_W<'_, MKEYR4rs> {
        MKEY135_W::new(self, 7)
    }
    ///Bit 8 - Master key bit 136 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey136(&mut self) -> MKEY136_W<'_, MKEYR4rs> {
        MKEY136_W::new(self, 8)
    }
    ///Bit 9 - Master key bit 137 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey137(&mut self) -> MKEY137_W<'_, MKEYR4rs> {
        MKEY137_W::new(self, 9)
    }
    ///Bit 10 - Master key bit 138 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey138(&mut self) -> MKEY138_W<'_, MKEYR4rs> {
        MKEY138_W::new(self, 10)
    }
    ///Bit 11 - Master key bit 139 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey139(&mut self) -> MKEY139_W<'_, MKEYR4rs> {
        MKEY139_W::new(self, 11)
    }
    ///Bit 12 - Master key bit 140 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey140(&mut self) -> MKEY140_W<'_, MKEYR4rs> {
        MKEY140_W::new(self, 12)
    }
    ///Bit 13 - Master key bit 141 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey141(&mut self) -> MKEY141_W<'_, MKEYR4rs> {
        MKEY141_W::new(self, 13)
    }
    ///Bit 14 - Master key bit 142 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey142(&mut self) -> MKEY142_W<'_, MKEYR4rs> {
        MKEY142_W::new(self, 14)
    }
    ///Bit 15 - Master key bit 143 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey143(&mut self) -> MKEY143_W<'_, MKEYR4rs> {
        MKEY143_W::new(self, 15)
    }
    ///Bit 16 - Master key bit 144 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey144(&mut self) -> MKEY144_W<'_, MKEYR4rs> {
        MKEY144_W::new(self, 16)
    }
    ///Bit 17 - Master key bit 145 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey145(&mut self) -> MKEY145_W<'_, MKEYR4rs> {
        MKEY145_W::new(self, 17)
    }
    ///Bit 18 - Master key bit 146 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey146(&mut self) -> MKEY146_W<'_, MKEYR4rs> {
        MKEY146_W::new(self, 18)
    }
    ///Bit 19 - Master key bit 147 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey147(&mut self) -> MKEY147_W<'_, MKEYR4rs> {
        MKEY147_W::new(self, 19)
    }
    ///Bit 20 - Master key bit 148 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey148(&mut self) -> MKEY148_W<'_, MKEYR4rs> {
        MKEY148_W::new(self, 20)
    }
    ///Bit 21 - Master key bit 149 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey149(&mut self) -> MKEY149_W<'_, MKEYR4rs> {
        MKEY149_W::new(self, 21)
    }
    ///Bit 22 - Master key bit 150 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey150(&mut self) -> MKEY150_W<'_, MKEYR4rs> {
        MKEY150_W::new(self, 22)
    }
    ///Bit 23 - Master key bit 151 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey151(&mut self) -> MKEY151_W<'_, MKEYR4rs> {
        MKEY151_W::new(self, 23)
    }
    ///Bit 24 - Master key bit 152 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey152(&mut self) -> MKEY152_W<'_, MKEYR4rs> {
        MKEY152_W::new(self, 24)
    }
    ///Bit 25 - Master key bit 153 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey153(&mut self) -> MKEY153_W<'_, MKEYR4rs> {
        MKEY153_W::new(self, 25)
    }
    ///Bit 26 - Master key bit 154 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey154(&mut self) -> MKEY154_W<'_, MKEYR4rs> {
        MKEY154_W::new(self, 26)
    }
    ///Bit 27 - Master key bit 155 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey155(&mut self) -> MKEY155_W<'_, MKEYR4rs> {
        MKEY155_W::new(self, 27)
    }
    ///Bit 28 - Master key bit 156 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey156(&mut self) -> MKEY156_W<'_, MKEYR4rs> {
        MKEY156_W::new(self, 28)
    }
    ///Bit 29 - Master key bit 157 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey157(&mut self) -> MKEY157_W<'_, MKEYR4rs> {
        MKEY157_W::new(self, 29)
    }
    ///Bit 30 - Master key bit 158 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey158(&mut self) -> MKEY158_W<'_, MKEYR4rs> {
        MKEY158_W::new(self, 30)
    }
    ///Bit 31 - Master key bit 159 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey159(&mut self) -> MKEY159_W<'_, MKEYR4rs> {
        MKEY159_W::new(self, 31)
    }
}
/**.MCE master key 4

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mkeyr4::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MCE1:MKEYR4)*/
pub struct MKEYR4rs;
impl crate::RegisterSpec for MKEYR4rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`mkeyr4::W`](W) writer structure
impl crate::Writable for MKEYR4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MKEYR4 to value 0
impl crate::Resettable for MKEYR4rs {}
