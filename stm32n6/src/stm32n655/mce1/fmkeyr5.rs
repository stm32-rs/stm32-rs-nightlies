///Register `FMKEYR5` writer
pub type W = crate::W<FMKEYR5rs>;
///Field `FMKEY160` writer - Fast master key bit 160 (i = 31 to 0)
pub type FMKEY160_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY161` writer - Fast master key bit 161 (i = 31 to 0)
pub type FMKEY161_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY162` writer - Fast master key bit 162 (i = 31 to 0)
pub type FMKEY162_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY163` writer - Fast master key bit 163 (i = 31 to 0)
pub type FMKEY163_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY164` writer - Fast master key bit 164 (i = 31 to 0)
pub type FMKEY164_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY165` writer - Fast master key bit 165 (i = 31 to 0)
pub type FMKEY165_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY166` writer - Fast master key bit 166 (i = 31 to 0)
pub type FMKEY166_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY167` writer - Fast master key bit 167 (i = 31 to 0)
pub type FMKEY167_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY168` writer - Fast master key bit 168 (i = 31 to 0)
pub type FMKEY168_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY169` writer - Fast master key bit 169 (i = 31 to 0)
pub type FMKEY169_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY170` writer - Fast master key bit 170 (i = 31 to 0)
pub type FMKEY170_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY171` writer - Fast master key bit 171 (i = 31 to 0)
pub type FMKEY171_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY172` writer - Fast master key bit 172 (i = 31 to 0)
pub type FMKEY172_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY173` writer - Fast master key bit 173 (i = 31 to 0)
pub type FMKEY173_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY174` writer - Fast master key bit 174 (i = 31 to 0)
pub type FMKEY174_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY175` writer - Fast master key bit 175 (i = 31 to 0)
pub type FMKEY175_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY176` writer - Fast master key bit 176 (i = 31 to 0)
pub type FMKEY176_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY177` writer - Fast master key bit 177 (i = 31 to 0)
pub type FMKEY177_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY178` writer - Fast master key bit 178 (i = 31 to 0)
pub type FMKEY178_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY179` writer - Fast master key bit 179 (i = 31 to 0)
pub type FMKEY179_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY180` writer - Fast master key bit 180 (i = 31 to 0)
pub type FMKEY180_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY181` writer - Fast master key bit 181 (i = 31 to 0)
pub type FMKEY181_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY182` writer - Fast master key bit 182 (i = 31 to 0)
pub type FMKEY182_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY183` writer - Fast master key bit 183 (i = 31 to 0)
pub type FMKEY183_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY184` writer - Fast master key bit 184 (i = 31 to 0)
pub type FMKEY184_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY185` writer - Fast master key bit 185 (i = 31 to 0)
pub type FMKEY185_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY186` writer - Fast master key bit 186 (i = 31 to 0)
pub type FMKEY186_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY187` writer - Fast master key bit 187 (i = 31 to 0)
pub type FMKEY187_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY188` writer - Fast master key bit 188 (i = 31 to 0)
pub type FMKEY188_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY189` writer - Fast master key bit 189 (i = 31 to 0)
pub type FMKEY189_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY190` writer - Fast master key bit 190 (i = 31 to 0)
pub type FMKEY190_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY191` writer - Fast master key bit 191 (i = 31 to 0)
pub type FMKEY191_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<FMKEYR5rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Fast master key bit 160 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey160(&mut self) -> FMKEY160_W<'_, FMKEYR5rs> {
        FMKEY160_W::new(self, 0)
    }
    ///Bit 1 - Fast master key bit 161 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey161(&mut self) -> FMKEY161_W<'_, FMKEYR5rs> {
        FMKEY161_W::new(self, 1)
    }
    ///Bit 2 - Fast master key bit 162 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey162(&mut self) -> FMKEY162_W<'_, FMKEYR5rs> {
        FMKEY162_W::new(self, 2)
    }
    ///Bit 3 - Fast master key bit 163 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey163(&mut self) -> FMKEY163_W<'_, FMKEYR5rs> {
        FMKEY163_W::new(self, 3)
    }
    ///Bit 4 - Fast master key bit 164 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey164(&mut self) -> FMKEY164_W<'_, FMKEYR5rs> {
        FMKEY164_W::new(self, 4)
    }
    ///Bit 5 - Fast master key bit 165 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey165(&mut self) -> FMKEY165_W<'_, FMKEYR5rs> {
        FMKEY165_W::new(self, 5)
    }
    ///Bit 6 - Fast master key bit 166 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey166(&mut self) -> FMKEY166_W<'_, FMKEYR5rs> {
        FMKEY166_W::new(self, 6)
    }
    ///Bit 7 - Fast master key bit 167 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey167(&mut self) -> FMKEY167_W<'_, FMKEYR5rs> {
        FMKEY167_W::new(self, 7)
    }
    ///Bit 8 - Fast master key bit 168 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey168(&mut self) -> FMKEY168_W<'_, FMKEYR5rs> {
        FMKEY168_W::new(self, 8)
    }
    ///Bit 9 - Fast master key bit 169 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey169(&mut self) -> FMKEY169_W<'_, FMKEYR5rs> {
        FMKEY169_W::new(self, 9)
    }
    ///Bit 10 - Fast master key bit 170 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey170(&mut self) -> FMKEY170_W<'_, FMKEYR5rs> {
        FMKEY170_W::new(self, 10)
    }
    ///Bit 11 - Fast master key bit 171 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey171(&mut self) -> FMKEY171_W<'_, FMKEYR5rs> {
        FMKEY171_W::new(self, 11)
    }
    ///Bit 12 - Fast master key bit 172 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey172(&mut self) -> FMKEY172_W<'_, FMKEYR5rs> {
        FMKEY172_W::new(self, 12)
    }
    ///Bit 13 - Fast master key bit 173 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey173(&mut self) -> FMKEY173_W<'_, FMKEYR5rs> {
        FMKEY173_W::new(self, 13)
    }
    ///Bit 14 - Fast master key bit 174 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey174(&mut self) -> FMKEY174_W<'_, FMKEYR5rs> {
        FMKEY174_W::new(self, 14)
    }
    ///Bit 15 - Fast master key bit 175 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey175(&mut self) -> FMKEY175_W<'_, FMKEYR5rs> {
        FMKEY175_W::new(self, 15)
    }
    ///Bit 16 - Fast master key bit 176 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey176(&mut self) -> FMKEY176_W<'_, FMKEYR5rs> {
        FMKEY176_W::new(self, 16)
    }
    ///Bit 17 - Fast master key bit 177 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey177(&mut self) -> FMKEY177_W<'_, FMKEYR5rs> {
        FMKEY177_W::new(self, 17)
    }
    ///Bit 18 - Fast master key bit 178 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey178(&mut self) -> FMKEY178_W<'_, FMKEYR5rs> {
        FMKEY178_W::new(self, 18)
    }
    ///Bit 19 - Fast master key bit 179 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey179(&mut self) -> FMKEY179_W<'_, FMKEYR5rs> {
        FMKEY179_W::new(self, 19)
    }
    ///Bit 20 - Fast master key bit 180 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey180(&mut self) -> FMKEY180_W<'_, FMKEYR5rs> {
        FMKEY180_W::new(self, 20)
    }
    ///Bit 21 - Fast master key bit 181 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey181(&mut self) -> FMKEY181_W<'_, FMKEYR5rs> {
        FMKEY181_W::new(self, 21)
    }
    ///Bit 22 - Fast master key bit 182 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey182(&mut self) -> FMKEY182_W<'_, FMKEYR5rs> {
        FMKEY182_W::new(self, 22)
    }
    ///Bit 23 - Fast master key bit 183 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey183(&mut self) -> FMKEY183_W<'_, FMKEYR5rs> {
        FMKEY183_W::new(self, 23)
    }
    ///Bit 24 - Fast master key bit 184 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey184(&mut self) -> FMKEY184_W<'_, FMKEYR5rs> {
        FMKEY184_W::new(self, 24)
    }
    ///Bit 25 - Fast master key bit 185 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey185(&mut self) -> FMKEY185_W<'_, FMKEYR5rs> {
        FMKEY185_W::new(self, 25)
    }
    ///Bit 26 - Fast master key bit 186 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey186(&mut self) -> FMKEY186_W<'_, FMKEYR5rs> {
        FMKEY186_W::new(self, 26)
    }
    ///Bit 27 - Fast master key bit 187 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey187(&mut self) -> FMKEY187_W<'_, FMKEYR5rs> {
        FMKEY187_W::new(self, 27)
    }
    ///Bit 28 - Fast master key bit 188 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey188(&mut self) -> FMKEY188_W<'_, FMKEYR5rs> {
        FMKEY188_W::new(self, 28)
    }
    ///Bit 29 - Fast master key bit 189 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey189(&mut self) -> FMKEY189_W<'_, FMKEYR5rs> {
        FMKEY189_W::new(self, 29)
    }
    ///Bit 30 - Fast master key bit 190 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey190(&mut self) -> FMKEY190_W<'_, FMKEYR5rs> {
        FMKEY190_W::new(self, 30)
    }
    ///Bit 31 - Fast master key bit 191 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey191(&mut self) -> FMKEY191_W<'_, FMKEYR5rs> {
        FMKEY191_W::new(self, 31)
    }
}
/**MCE fast master key 5

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmkeyr5::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#MCE1:FMKEYR5)*/
pub struct FMKEYR5rs;
impl crate::RegisterSpec for FMKEYR5rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`fmkeyr5::W`](W) writer structure
impl crate::Writable for FMKEYR5rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FMKEYR5 to value 0
impl crate::Resettable for FMKEYR5rs {}
