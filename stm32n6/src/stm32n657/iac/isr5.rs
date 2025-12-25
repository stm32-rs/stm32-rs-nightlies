///Register `ISR5` reader
pub type R = crate::R<ISR5rs>;
///Field `IAF160` reader - illegal access interrupt enable for peripheral 160
pub type IAF160_R = crate::BitReader;
///Field `IAF161` reader - illegal access interrupt enable for peripheral 161
pub type IAF161_R = crate::BitReader;
///Field `IAF162` reader - illegal access interrupt enable for peripheral 162
pub type IAF162_R = crate::BitReader;
///Field `IAF163` reader - illegal access interrupt enable for peripheral 163
pub type IAF163_R = crate::BitReader;
///Field `IAF164` reader - illegal access interrupt enable for peripheral 164
pub type IAF164_R = crate::BitReader;
///Field `IAF165` reader - illegal access interrupt enable for peripheral 165
pub type IAF165_R = crate::BitReader;
///Field `IAF166` reader - illegal access interrupt enable for peripheral 166
pub type IAF166_R = crate::BitReader;
///Field `IAF167` reader - illegal access interrupt enable for peripheral 167
pub type IAF167_R = crate::BitReader;
///Field `IAF168` reader - illegal access interrupt enable for peripheral 168
pub type IAF168_R = crate::BitReader;
///Field `IAF169` reader - illegal access interrupt enable for peripheral 169
pub type IAF169_R = crate::BitReader;
///Field `IAF170` reader - illegal access interrupt enable for peripheral 170
pub type IAF170_R = crate::BitReader;
///Field `IAF171` reader - illegal access interrupt enable for peripheral 171
pub type IAF171_R = crate::BitReader;
///Field `IAF172` reader - illegal access interrupt enable for peripheral 172
pub type IAF172_R = crate::BitReader;
///Field `IAF173` reader - illegal access interrupt enable for peripheral 173
pub type IAF173_R = crate::BitReader;
///Field `IAF174` reader - illegal access interrupt enable for peripheral 174
pub type IAF174_R = crate::BitReader;
///Field `IAF175` reader - illegal access interrupt enable for peripheral 175
pub type IAF175_R = crate::BitReader;
///Field `IAF176` reader - illegal access interrupt enable for peripheral 176
pub type IAF176_R = crate::BitReader;
///Field `IAF177` reader - illegal access interrupt enable for peripheral 177
pub type IAF177_R = crate::BitReader;
///Field `IAF178` reader - illegal access interrupt enable for peripheral 178
pub type IAF178_R = crate::BitReader;
///Field `IAF179` reader - illegal access interrupt enable for peripheral 179
pub type IAF179_R = crate::BitReader;
///Field `IAF180` reader - illegal access interrupt enable for peripheral 180
pub type IAF180_R = crate::BitReader;
///Field `IAF181` reader - illegal access interrupt enable for peripheral 181
pub type IAF181_R = crate::BitReader;
///Field `IAF182` reader - illegal access interrupt enable for peripheral 182
pub type IAF182_R = crate::BitReader;
///Field `IAF183` reader - illegal access interrupt enable for peripheral 183
pub type IAF183_R = crate::BitReader;
///Field `IAF184` reader - illegal access interrupt enable for peripheral 184
pub type IAF184_R = crate::BitReader;
///Field `IAF185` reader - illegal access interrupt enable for peripheral 185
pub type IAF185_R = crate::BitReader;
///Field `IAF186` reader - illegal access interrupt enable for peripheral 186
pub type IAF186_R = crate::BitReader;
///Field `IAF187` reader - illegal access interrupt enable for peripheral 187
pub type IAF187_R = crate::BitReader;
///Field `IAF188` reader - illegal access interrupt enable for peripheral 188
pub type IAF188_R = crate::BitReader;
///Field `IAF189` reader - illegal access interrupt enable for peripheral 189
pub type IAF189_R = crate::BitReader;
///Field `IAF190` reader - illegal access interrupt enable for peripheral 190
pub type IAF190_R = crate::BitReader;
///Field `IAF191` reader - illegal access interrupt enable for peripheral 191
pub type IAF191_R = crate::BitReader;
impl R {
    ///Bit 0 - illegal access interrupt enable for peripheral 160
    #[inline(always)]
    pub fn iaf160(&self) -> IAF160_R {
        IAF160_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - illegal access interrupt enable for peripheral 161
    #[inline(always)]
    pub fn iaf161(&self) -> IAF161_R {
        IAF161_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - illegal access interrupt enable for peripheral 162
    #[inline(always)]
    pub fn iaf162(&self) -> IAF162_R {
        IAF162_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - illegal access interrupt enable for peripheral 163
    #[inline(always)]
    pub fn iaf163(&self) -> IAF163_R {
        IAF163_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - illegal access interrupt enable for peripheral 164
    #[inline(always)]
    pub fn iaf164(&self) -> IAF164_R {
        IAF164_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - illegal access interrupt enable for peripheral 165
    #[inline(always)]
    pub fn iaf165(&self) -> IAF165_R {
        IAF165_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - illegal access interrupt enable for peripheral 166
    #[inline(always)]
    pub fn iaf166(&self) -> IAF166_R {
        IAF166_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - illegal access interrupt enable for peripheral 167
    #[inline(always)]
    pub fn iaf167(&self) -> IAF167_R {
        IAF167_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - illegal access interrupt enable for peripheral 168
    #[inline(always)]
    pub fn iaf168(&self) -> IAF168_R {
        IAF168_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - illegal access interrupt enable for peripheral 169
    #[inline(always)]
    pub fn iaf169(&self) -> IAF169_R {
        IAF169_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - illegal access interrupt enable for peripheral 170
    #[inline(always)]
    pub fn iaf170(&self) -> IAF170_R {
        IAF170_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - illegal access interrupt enable for peripheral 171
    #[inline(always)]
    pub fn iaf171(&self) -> IAF171_R {
        IAF171_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - illegal access interrupt enable for peripheral 172
    #[inline(always)]
    pub fn iaf172(&self) -> IAF172_R {
        IAF172_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - illegal access interrupt enable for peripheral 173
    #[inline(always)]
    pub fn iaf173(&self) -> IAF173_R {
        IAF173_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - illegal access interrupt enable for peripheral 174
    #[inline(always)]
    pub fn iaf174(&self) -> IAF174_R {
        IAF174_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - illegal access interrupt enable for peripheral 175
    #[inline(always)]
    pub fn iaf175(&self) -> IAF175_R {
        IAF175_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - illegal access interrupt enable for peripheral 176
    #[inline(always)]
    pub fn iaf176(&self) -> IAF176_R {
        IAF176_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - illegal access interrupt enable for peripheral 177
    #[inline(always)]
    pub fn iaf177(&self) -> IAF177_R {
        IAF177_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - illegal access interrupt enable for peripheral 178
    #[inline(always)]
    pub fn iaf178(&self) -> IAF178_R {
        IAF178_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - illegal access interrupt enable for peripheral 179
    #[inline(always)]
    pub fn iaf179(&self) -> IAF179_R {
        IAF179_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - illegal access interrupt enable for peripheral 180
    #[inline(always)]
    pub fn iaf180(&self) -> IAF180_R {
        IAF180_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - illegal access interrupt enable for peripheral 181
    #[inline(always)]
    pub fn iaf181(&self) -> IAF181_R {
        IAF181_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - illegal access interrupt enable for peripheral 182
    #[inline(always)]
    pub fn iaf182(&self) -> IAF182_R {
        IAF182_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - illegal access interrupt enable for peripheral 183
    #[inline(always)]
    pub fn iaf183(&self) -> IAF183_R {
        IAF183_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - illegal access interrupt enable for peripheral 184
    #[inline(always)]
    pub fn iaf184(&self) -> IAF184_R {
        IAF184_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - illegal access interrupt enable for peripheral 185
    #[inline(always)]
    pub fn iaf185(&self) -> IAF185_R {
        IAF185_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - illegal access interrupt enable for peripheral 186
    #[inline(always)]
    pub fn iaf186(&self) -> IAF186_R {
        IAF186_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - illegal access interrupt enable for peripheral 187
    #[inline(always)]
    pub fn iaf187(&self) -> IAF187_R {
        IAF187_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - illegal access interrupt enable for peripheral 188
    #[inline(always)]
    pub fn iaf188(&self) -> IAF188_R {
        IAF188_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - illegal access interrupt enable for peripheral 189
    #[inline(always)]
    pub fn iaf189(&self) -> IAF189_R {
        IAF189_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - illegal access interrupt enable for peripheral 190
    #[inline(always)]
    pub fn iaf190(&self) -> IAF190_R {
        IAF190_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - illegal access interrupt enable for peripheral 191
    #[inline(always)]
    pub fn iaf191(&self) -> IAF191_R {
        IAF191_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR5")
            .field("iaf160", &self.iaf160())
            .field("iaf161", &self.iaf161())
            .field("iaf162", &self.iaf162())
            .field("iaf163", &self.iaf163())
            .field("iaf164", &self.iaf164())
            .field("iaf165", &self.iaf165())
            .field("iaf166", &self.iaf166())
            .field("iaf167", &self.iaf167())
            .field("iaf168", &self.iaf168())
            .field("iaf169", &self.iaf169())
            .field("iaf170", &self.iaf170())
            .field("iaf171", &self.iaf171())
            .field("iaf172", &self.iaf172())
            .field("iaf173", &self.iaf173())
            .field("iaf174", &self.iaf174())
            .field("iaf175", &self.iaf175())
            .field("iaf176", &self.iaf176())
            .field("iaf177", &self.iaf177())
            .field("iaf178", &self.iaf178())
            .field("iaf179", &self.iaf179())
            .field("iaf180", &self.iaf180())
            .field("iaf181", &self.iaf181())
            .field("iaf182", &self.iaf182())
            .field("iaf183", &self.iaf183())
            .field("iaf184", &self.iaf184())
            .field("iaf185", &self.iaf185())
            .field("iaf186", &self.iaf186())
            .field("iaf187", &self.iaf187())
            .field("iaf188", &self.iaf188())
            .field("iaf189", &self.iaf189())
            .field("iaf190", &self.iaf190())
            .field("iaf191", &self.iaf191())
            .finish()
    }
}
/**IAC interrupt status register 5

You can [`read`](crate::Reg::read) this register and get [`isr5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#IAC:ISR5)*/
pub struct ISR5rs;
impl crate::RegisterSpec for ISR5rs {
    type Ux = u32;
}
///`read()` method returns [`isr5::R`](R) reader structure
impl crate::Readable for ISR5rs {}
///`reset()` method sets ISR5 to value 0
impl crate::Resettable for ISR5rs {}
