///Register `MKEYR5` writer
pub type W = crate::W<MKEYR5rs>;
///Field `MKEY160` writer - Master key bit 160 (i = 31 to 0)
pub type MKEY160_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY161` writer - Master key bit 161 (i = 31 to 0)
pub type MKEY161_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY162` writer - Master key bit 162 (i = 31 to 0)
pub type MKEY162_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY163` writer - Master key bit 163 (i = 31 to 0)
pub type MKEY163_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY164` writer - Master key bit 164 (i = 31 to 0)
pub type MKEY164_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY165` writer - Master key bit 165 (i = 31 to 0)
pub type MKEY165_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY166` writer - Master key bit 166 (i = 31 to 0)
pub type MKEY166_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY167` writer - Master key bit 167 (i = 31 to 0)
pub type MKEY167_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY168` writer - Master key bit 168 (i = 31 to 0)
pub type MKEY168_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY169` writer - Master key bit 169 (i = 31 to 0)
pub type MKEY169_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY170` writer - Master key bit 170 (i = 31 to 0)
pub type MKEY170_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY171` writer - Master key bit 171 (i = 31 to 0)
pub type MKEY171_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY172` writer - Master key bit 172 (i = 31 to 0)
pub type MKEY172_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY173` writer - Master key bit 173 (i = 31 to 0)
pub type MKEY173_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY174` writer - Master key bit 174 (i = 31 to 0)
pub type MKEY174_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY175` writer - Master key bit 175 (i = 31 to 0)
pub type MKEY175_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY176` writer - Master key bit 176 (i = 31 to 0)
pub type MKEY176_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY177` writer - Master key bit 177 (i = 31 to 0)
pub type MKEY177_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY178` writer - Master key bit 178 (i = 31 to 0)
pub type MKEY178_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY179` writer - Master key bit 179 (i = 31 to 0)
pub type MKEY179_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY180` writer - Master key bit 180 (i = 31 to 0)
pub type MKEY180_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY181` writer - Master key bit 181 (i = 31 to 0)
pub type MKEY181_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY182` writer - Master key bit 182 (i = 31 to 0)
pub type MKEY182_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY183` writer - Master key bit 183 (i = 31 to 0)
pub type MKEY183_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY184` writer - Master key bit 184 (i = 31 to 0)
pub type MKEY184_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY185` writer - Master key bit 185 (i = 31 to 0)
pub type MKEY185_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY186` writer - Master key bit 186 (i = 31 to 0)
pub type MKEY186_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY187` writer - Master key bit 187 (i = 31 to 0)
pub type MKEY187_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY188` writer - Master key bit 188 (i = 31 to 0)
pub type MKEY188_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY189` writer - Master key bit 189 (i = 31 to 0)
pub type MKEY189_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY190` writer - Master key bit 190 (i = 31 to 0)
pub type MKEY190_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY191` writer - Master key bit 191 (i = 31 to 0)
pub type MKEY191_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<MKEYR5rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Master key bit 160 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey160(&mut self) -> MKEY160_W<'_, MKEYR5rs> {
        MKEY160_W::new(self, 0)
    }
    ///Bit 1 - Master key bit 161 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey161(&mut self) -> MKEY161_W<'_, MKEYR5rs> {
        MKEY161_W::new(self, 1)
    }
    ///Bit 2 - Master key bit 162 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey162(&mut self) -> MKEY162_W<'_, MKEYR5rs> {
        MKEY162_W::new(self, 2)
    }
    ///Bit 3 - Master key bit 163 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey163(&mut self) -> MKEY163_W<'_, MKEYR5rs> {
        MKEY163_W::new(self, 3)
    }
    ///Bit 4 - Master key bit 164 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey164(&mut self) -> MKEY164_W<'_, MKEYR5rs> {
        MKEY164_W::new(self, 4)
    }
    ///Bit 5 - Master key bit 165 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey165(&mut self) -> MKEY165_W<'_, MKEYR5rs> {
        MKEY165_W::new(self, 5)
    }
    ///Bit 6 - Master key bit 166 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey166(&mut self) -> MKEY166_W<'_, MKEYR5rs> {
        MKEY166_W::new(self, 6)
    }
    ///Bit 7 - Master key bit 167 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey167(&mut self) -> MKEY167_W<'_, MKEYR5rs> {
        MKEY167_W::new(self, 7)
    }
    ///Bit 8 - Master key bit 168 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey168(&mut self) -> MKEY168_W<'_, MKEYR5rs> {
        MKEY168_W::new(self, 8)
    }
    ///Bit 9 - Master key bit 169 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey169(&mut self) -> MKEY169_W<'_, MKEYR5rs> {
        MKEY169_W::new(self, 9)
    }
    ///Bit 10 - Master key bit 170 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey170(&mut self) -> MKEY170_W<'_, MKEYR5rs> {
        MKEY170_W::new(self, 10)
    }
    ///Bit 11 - Master key bit 171 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey171(&mut self) -> MKEY171_W<'_, MKEYR5rs> {
        MKEY171_W::new(self, 11)
    }
    ///Bit 12 - Master key bit 172 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey172(&mut self) -> MKEY172_W<'_, MKEYR5rs> {
        MKEY172_W::new(self, 12)
    }
    ///Bit 13 - Master key bit 173 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey173(&mut self) -> MKEY173_W<'_, MKEYR5rs> {
        MKEY173_W::new(self, 13)
    }
    ///Bit 14 - Master key bit 174 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey174(&mut self) -> MKEY174_W<'_, MKEYR5rs> {
        MKEY174_W::new(self, 14)
    }
    ///Bit 15 - Master key bit 175 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey175(&mut self) -> MKEY175_W<'_, MKEYR5rs> {
        MKEY175_W::new(self, 15)
    }
    ///Bit 16 - Master key bit 176 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey176(&mut self) -> MKEY176_W<'_, MKEYR5rs> {
        MKEY176_W::new(self, 16)
    }
    ///Bit 17 - Master key bit 177 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey177(&mut self) -> MKEY177_W<'_, MKEYR5rs> {
        MKEY177_W::new(self, 17)
    }
    ///Bit 18 - Master key bit 178 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey178(&mut self) -> MKEY178_W<'_, MKEYR5rs> {
        MKEY178_W::new(self, 18)
    }
    ///Bit 19 - Master key bit 179 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey179(&mut self) -> MKEY179_W<'_, MKEYR5rs> {
        MKEY179_W::new(self, 19)
    }
    ///Bit 20 - Master key bit 180 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey180(&mut self) -> MKEY180_W<'_, MKEYR5rs> {
        MKEY180_W::new(self, 20)
    }
    ///Bit 21 - Master key bit 181 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey181(&mut self) -> MKEY181_W<'_, MKEYR5rs> {
        MKEY181_W::new(self, 21)
    }
    ///Bit 22 - Master key bit 182 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey182(&mut self) -> MKEY182_W<'_, MKEYR5rs> {
        MKEY182_W::new(self, 22)
    }
    ///Bit 23 - Master key bit 183 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey183(&mut self) -> MKEY183_W<'_, MKEYR5rs> {
        MKEY183_W::new(self, 23)
    }
    ///Bit 24 - Master key bit 184 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey184(&mut self) -> MKEY184_W<'_, MKEYR5rs> {
        MKEY184_W::new(self, 24)
    }
    ///Bit 25 - Master key bit 185 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey185(&mut self) -> MKEY185_W<'_, MKEYR5rs> {
        MKEY185_W::new(self, 25)
    }
    ///Bit 26 - Master key bit 186 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey186(&mut self) -> MKEY186_W<'_, MKEYR5rs> {
        MKEY186_W::new(self, 26)
    }
    ///Bit 27 - Master key bit 187 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey187(&mut self) -> MKEY187_W<'_, MKEYR5rs> {
        MKEY187_W::new(self, 27)
    }
    ///Bit 28 - Master key bit 188 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey188(&mut self) -> MKEY188_W<'_, MKEYR5rs> {
        MKEY188_W::new(self, 28)
    }
    ///Bit 29 - Master key bit 189 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey189(&mut self) -> MKEY189_W<'_, MKEYR5rs> {
        MKEY189_W::new(self, 29)
    }
    ///Bit 30 - Master key bit 190 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey190(&mut self) -> MKEY190_W<'_, MKEYR5rs> {
        MKEY190_W::new(self, 30)
    }
    ///Bit 31 - Master key bit 191 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey191(&mut self) -> MKEY191_W<'_, MKEYR5rs> {
        MKEY191_W::new(self, 31)
    }
}
/**.MCE master key 5

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mkeyr5::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MCE1:MKEYR5)*/
pub struct MKEYR5rs;
impl crate::RegisterSpec for MKEYR5rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`mkeyr5::W`](W) writer structure
impl crate::Writable for MKEYR5rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MKEYR5 to value 0
impl crate::Resettable for MKEYR5rs {}
