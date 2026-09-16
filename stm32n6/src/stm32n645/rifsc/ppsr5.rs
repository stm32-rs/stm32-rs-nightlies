///Register `PPSR5` reader
pub type R = crate::R<PPSR5rs>;
///Field `PPEN160` reader - peripheral protection enable 160
pub type PPEN160_R = crate::BitReader;
///Field `PPEN161` reader - peripheral protection enable 161
pub type PPEN161_R = crate::BitReader;
///Field `PPEN162` reader - peripheral protection enable 162
pub type PPEN162_R = crate::BitReader;
///Field `PPEN163` reader - peripheral protection enable 163
pub type PPEN163_R = crate::BitReader;
///Field `PPEN164` reader - peripheral protection enable 164
pub type PPEN164_R = crate::BitReader;
///Field `PPEN165` reader - peripheral protection enable 165
pub type PPEN165_R = crate::BitReader;
///Field `PPEN166` reader - peripheral protection enable 166
pub type PPEN166_R = crate::BitReader;
///Field `PPEN167` reader - peripheral protection enable 167
pub type PPEN167_R = crate::BitReader;
///Field `PPEN168` reader - peripheral protection enable 168
pub type PPEN168_R = crate::BitReader;
///Field `PPEN169` reader - peripheral protection enable 169
pub type PPEN169_R = crate::BitReader;
///Field `PPEN170` reader - peripheral protection enable 170
pub type PPEN170_R = crate::BitReader;
///Field `PPEN171` reader - peripheral protection enable 171
pub type PPEN171_R = crate::BitReader;
///Field `PPEN172` reader - peripheral protection enable 172
pub type PPEN172_R = crate::BitReader;
///Field `PPEN173` reader - peripheral protection enable 173
pub type PPEN173_R = crate::BitReader;
///Field `PPEN174` reader - peripheral protection enable 174
pub type PPEN174_R = crate::BitReader;
///Field `PPEN175` reader - peripheral protection enable 175
pub type PPEN175_R = crate::BitReader;
///Field `PPEN176` reader - peripheral protection enable 176
pub type PPEN176_R = crate::BitReader;
///Field `PPEN177` reader - peripheral protection enable 177
pub type PPEN177_R = crate::BitReader;
///Field `PPEN178` reader - peripheral protection enable 178
pub type PPEN178_R = crate::BitReader;
///Field `PPEN179` reader - peripheral protection enable 179
pub type PPEN179_R = crate::BitReader;
///Field `PPEN180` reader - peripheral protection enable 180
pub type PPEN180_R = crate::BitReader;
///Field `PPEN181` reader - peripheral protection enable 181
pub type PPEN181_R = crate::BitReader;
///Field `PPEN182` reader - peripheral protection enable 182
pub type PPEN182_R = crate::BitReader;
///Field `PPEN183` reader - peripheral protection enable 183
pub type PPEN183_R = crate::BitReader;
///Field `PPEN184` reader - peripheral protection enable 184
pub type PPEN184_R = crate::BitReader;
///Field `PPEN185` reader - peripheral protection enable 185
pub type PPEN185_R = crate::BitReader;
///Field `PPEN186` reader - peripheral protection enable 186
pub type PPEN186_R = crate::BitReader;
///Field `PPEN187` reader - peripheral protection enable 187
pub type PPEN187_R = crate::BitReader;
///Field `PPEN188` reader - peripheral protection enable 188
pub type PPEN188_R = crate::BitReader;
///Field `PPEN189` reader - peripheral protection enable 189
pub type PPEN189_R = crate::BitReader;
///Field `PPEN190` reader - peripheral protection enable 190
pub type PPEN190_R = crate::BitReader;
///Field `PPEN191` reader - peripheral protection enable 191
pub type PPEN191_R = crate::BitReader;
impl R {
    ///Bit 0 - peripheral protection enable 160
    #[inline(always)]
    pub fn ppen160(&self) -> PPEN160_R {
        PPEN160_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - peripheral protection enable 161
    #[inline(always)]
    pub fn ppen161(&self) -> PPEN161_R {
        PPEN161_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - peripheral protection enable 162
    #[inline(always)]
    pub fn ppen162(&self) -> PPEN162_R {
        PPEN162_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - peripheral protection enable 163
    #[inline(always)]
    pub fn ppen163(&self) -> PPEN163_R {
        PPEN163_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - peripheral protection enable 164
    #[inline(always)]
    pub fn ppen164(&self) -> PPEN164_R {
        PPEN164_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - peripheral protection enable 165
    #[inline(always)]
    pub fn ppen165(&self) -> PPEN165_R {
        PPEN165_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - peripheral protection enable 166
    #[inline(always)]
    pub fn ppen166(&self) -> PPEN166_R {
        PPEN166_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - peripheral protection enable 167
    #[inline(always)]
    pub fn ppen167(&self) -> PPEN167_R {
        PPEN167_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - peripheral protection enable 168
    #[inline(always)]
    pub fn ppen168(&self) -> PPEN168_R {
        PPEN168_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - peripheral protection enable 169
    #[inline(always)]
    pub fn ppen169(&self) -> PPEN169_R {
        PPEN169_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - peripheral protection enable 170
    #[inline(always)]
    pub fn ppen170(&self) -> PPEN170_R {
        PPEN170_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - peripheral protection enable 171
    #[inline(always)]
    pub fn ppen171(&self) -> PPEN171_R {
        PPEN171_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - peripheral protection enable 172
    #[inline(always)]
    pub fn ppen172(&self) -> PPEN172_R {
        PPEN172_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - peripheral protection enable 173
    #[inline(always)]
    pub fn ppen173(&self) -> PPEN173_R {
        PPEN173_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - peripheral protection enable 174
    #[inline(always)]
    pub fn ppen174(&self) -> PPEN174_R {
        PPEN174_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - peripheral protection enable 175
    #[inline(always)]
    pub fn ppen175(&self) -> PPEN175_R {
        PPEN175_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - peripheral protection enable 176
    #[inline(always)]
    pub fn ppen176(&self) -> PPEN176_R {
        PPEN176_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - peripheral protection enable 177
    #[inline(always)]
    pub fn ppen177(&self) -> PPEN177_R {
        PPEN177_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - peripheral protection enable 178
    #[inline(always)]
    pub fn ppen178(&self) -> PPEN178_R {
        PPEN178_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - peripheral protection enable 179
    #[inline(always)]
    pub fn ppen179(&self) -> PPEN179_R {
        PPEN179_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - peripheral protection enable 180
    #[inline(always)]
    pub fn ppen180(&self) -> PPEN180_R {
        PPEN180_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - peripheral protection enable 181
    #[inline(always)]
    pub fn ppen181(&self) -> PPEN181_R {
        PPEN181_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - peripheral protection enable 182
    #[inline(always)]
    pub fn ppen182(&self) -> PPEN182_R {
        PPEN182_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - peripheral protection enable 183
    #[inline(always)]
    pub fn ppen183(&self) -> PPEN183_R {
        PPEN183_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - peripheral protection enable 184
    #[inline(always)]
    pub fn ppen184(&self) -> PPEN184_R {
        PPEN184_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - peripheral protection enable 185
    #[inline(always)]
    pub fn ppen185(&self) -> PPEN185_R {
        PPEN185_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - peripheral protection enable 186
    #[inline(always)]
    pub fn ppen186(&self) -> PPEN186_R {
        PPEN186_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - peripheral protection enable 187
    #[inline(always)]
    pub fn ppen187(&self) -> PPEN187_R {
        PPEN187_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - peripheral protection enable 188
    #[inline(always)]
    pub fn ppen188(&self) -> PPEN188_R {
        PPEN188_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - peripheral protection enable 189
    #[inline(always)]
    pub fn ppen189(&self) -> PPEN189_R {
        PPEN189_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - peripheral protection enable 190
    #[inline(always)]
    pub fn ppen190(&self) -> PPEN190_R {
        PPEN190_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - peripheral protection enable 191
    #[inline(always)]
    pub fn ppen191(&self) -> PPEN191_R {
        PPEN191_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PPSR5")
            .field("ppen160", &self.ppen160())
            .field("ppen161", &self.ppen161())
            .field("ppen162", &self.ppen162())
            .field("ppen163", &self.ppen163())
            .field("ppen164", &self.ppen164())
            .field("ppen165", &self.ppen165())
            .field("ppen166", &self.ppen166())
            .field("ppen167", &self.ppen167())
            .field("ppen168", &self.ppen168())
            .field("ppen169", &self.ppen169())
            .field("ppen170", &self.ppen170())
            .field("ppen171", &self.ppen171())
            .field("ppen172", &self.ppen172())
            .field("ppen173", &self.ppen173())
            .field("ppen174", &self.ppen174())
            .field("ppen175", &self.ppen175())
            .field("ppen176", &self.ppen176())
            .field("ppen177", &self.ppen177())
            .field("ppen178", &self.ppen178())
            .field("ppen179", &self.ppen179())
            .field("ppen180", &self.ppen180())
            .field("ppen181", &self.ppen181())
            .field("ppen182", &self.ppen182())
            .field("ppen183", &self.ppen183())
            .field("ppen184", &self.ppen184())
            .field("ppen185", &self.ppen185())
            .field("ppen186", &self.ppen186())
            .field("ppen187", &self.ppen187())
            .field("ppen188", &self.ppen188())
            .field("ppen189", &self.ppen189())
            .field("ppen190", &self.ppen190())
            .field("ppen191", &self.ppen191())
            .finish()
    }
}
/**RIFSC peripheral protection status register 5

You can [`read`](crate::Reg::read) this register and get [`ppsr5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#RIFSC:PPSR5)*/
pub struct PPSR5rs;
impl crate::RegisterSpec for PPSR5rs {
    type Ux = u32;
}
///`read()` method returns [`ppsr5::R`](R) reader structure
impl crate::Readable for PPSR5rs {}
///`reset()` method sets PPSR5 to value 0x3dde_ef7f
impl crate::Resettable for PPSR5rs {
    const RESET_VALUE: u32 = 0x3dde_ef7f;
}
