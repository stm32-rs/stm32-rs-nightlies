///Register `ICR5` writer
pub type W = crate::W<ICR5rs>;
///Field `IAF160` writer - illegal access flag clear for peripheral 160
pub type IAF160_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF161` writer - illegal access flag clear for peripheral 161
pub type IAF161_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF162` writer - illegal access flag clear for peripheral 162
pub type IAF162_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF163` writer - illegal access flag clear for peripheral 163
pub type IAF163_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF164` writer - illegal access flag clear for peripheral 164
pub type IAF164_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF165` writer - illegal access flag clear for peripheral 165
pub type IAF165_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF166` writer - illegal access flag clear for peripheral 166
pub type IAF166_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF167` writer - illegal access flag clear for peripheral 167
pub type IAF167_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF168` writer - illegal access flag clear for peripheral 168
pub type IAF168_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF169` writer - illegal access flag clear for peripheral 169
pub type IAF169_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF170` writer - illegal access flag clear for peripheral 170
pub type IAF170_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF171` writer - illegal access flag clear for peripheral 171
pub type IAF171_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF172` writer - illegal access flag clear for peripheral 172
pub type IAF172_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF173` writer - illegal access flag clear for peripheral 173
pub type IAF173_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF174` writer - illegal access flag clear for peripheral 174
pub type IAF174_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF175` writer - illegal access flag clear for peripheral 175
pub type IAF175_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF176` writer - illegal access flag clear for peripheral 176
pub type IAF176_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF177` writer - illegal access flag clear for peripheral 177
pub type IAF177_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF178` writer - illegal access flag clear for peripheral 178
pub type IAF178_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF179` writer - illegal access flag clear for peripheral 179
pub type IAF179_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF180` writer - illegal access flag clear for peripheral 180
pub type IAF180_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF181` writer - illegal access flag clear for peripheral 181
pub type IAF181_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF182` writer - illegal access flag clear for peripheral 182
pub type IAF182_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF183` writer - illegal access flag clear for peripheral 183
pub type IAF183_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF184` writer - illegal access flag clear for peripheral 184
pub type IAF184_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF185` writer - illegal access flag clear for peripheral 185
pub type IAF185_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF186` writer - illegal access flag clear for peripheral 186
pub type IAF186_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF187` writer - illegal access flag clear for peripheral 187
pub type IAF187_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF188` writer - illegal access flag clear for peripheral 188
pub type IAF188_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF189` writer - illegal access flag clear for peripheral 189
pub type IAF189_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF190` writer - illegal access flag clear for peripheral 190
pub type IAF190_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF191` writer - illegal access flag clear for peripheral 191
pub type IAF191_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<ICR5rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - illegal access flag clear for peripheral 160
    #[inline(always)]
    pub fn iaf160(&mut self) -> IAF160_W<'_, ICR5rs> {
        IAF160_W::new(self, 0)
    }
    ///Bit 1 - illegal access flag clear for peripheral 161
    #[inline(always)]
    pub fn iaf161(&mut self) -> IAF161_W<'_, ICR5rs> {
        IAF161_W::new(self, 1)
    }
    ///Bit 2 - illegal access flag clear for peripheral 162
    #[inline(always)]
    pub fn iaf162(&mut self) -> IAF162_W<'_, ICR5rs> {
        IAF162_W::new(self, 2)
    }
    ///Bit 3 - illegal access flag clear for peripheral 163
    #[inline(always)]
    pub fn iaf163(&mut self) -> IAF163_W<'_, ICR5rs> {
        IAF163_W::new(self, 3)
    }
    ///Bit 4 - illegal access flag clear for peripheral 164
    #[inline(always)]
    pub fn iaf164(&mut self) -> IAF164_W<'_, ICR5rs> {
        IAF164_W::new(self, 4)
    }
    ///Bit 5 - illegal access flag clear for peripheral 165
    #[inline(always)]
    pub fn iaf165(&mut self) -> IAF165_W<'_, ICR5rs> {
        IAF165_W::new(self, 5)
    }
    ///Bit 6 - illegal access flag clear for peripheral 166
    #[inline(always)]
    pub fn iaf166(&mut self) -> IAF166_W<'_, ICR5rs> {
        IAF166_W::new(self, 6)
    }
    ///Bit 7 - illegal access flag clear for peripheral 167
    #[inline(always)]
    pub fn iaf167(&mut self) -> IAF167_W<'_, ICR5rs> {
        IAF167_W::new(self, 7)
    }
    ///Bit 8 - illegal access flag clear for peripheral 168
    #[inline(always)]
    pub fn iaf168(&mut self) -> IAF168_W<'_, ICR5rs> {
        IAF168_W::new(self, 8)
    }
    ///Bit 9 - illegal access flag clear for peripheral 169
    #[inline(always)]
    pub fn iaf169(&mut self) -> IAF169_W<'_, ICR5rs> {
        IAF169_W::new(self, 9)
    }
    ///Bit 10 - illegal access flag clear for peripheral 170
    #[inline(always)]
    pub fn iaf170(&mut self) -> IAF170_W<'_, ICR5rs> {
        IAF170_W::new(self, 10)
    }
    ///Bit 11 - illegal access flag clear for peripheral 171
    #[inline(always)]
    pub fn iaf171(&mut self) -> IAF171_W<'_, ICR5rs> {
        IAF171_W::new(self, 11)
    }
    ///Bit 12 - illegal access flag clear for peripheral 172
    #[inline(always)]
    pub fn iaf172(&mut self) -> IAF172_W<'_, ICR5rs> {
        IAF172_W::new(self, 12)
    }
    ///Bit 13 - illegal access flag clear for peripheral 173
    #[inline(always)]
    pub fn iaf173(&mut self) -> IAF173_W<'_, ICR5rs> {
        IAF173_W::new(self, 13)
    }
    ///Bit 14 - illegal access flag clear for peripheral 174
    #[inline(always)]
    pub fn iaf174(&mut self) -> IAF174_W<'_, ICR5rs> {
        IAF174_W::new(self, 14)
    }
    ///Bit 15 - illegal access flag clear for peripheral 175
    #[inline(always)]
    pub fn iaf175(&mut self) -> IAF175_W<'_, ICR5rs> {
        IAF175_W::new(self, 15)
    }
    ///Bit 16 - illegal access flag clear for peripheral 176
    #[inline(always)]
    pub fn iaf176(&mut self) -> IAF176_W<'_, ICR5rs> {
        IAF176_W::new(self, 16)
    }
    ///Bit 17 - illegal access flag clear for peripheral 177
    #[inline(always)]
    pub fn iaf177(&mut self) -> IAF177_W<'_, ICR5rs> {
        IAF177_W::new(self, 17)
    }
    ///Bit 18 - illegal access flag clear for peripheral 178
    #[inline(always)]
    pub fn iaf178(&mut self) -> IAF178_W<'_, ICR5rs> {
        IAF178_W::new(self, 18)
    }
    ///Bit 19 - illegal access flag clear for peripheral 179
    #[inline(always)]
    pub fn iaf179(&mut self) -> IAF179_W<'_, ICR5rs> {
        IAF179_W::new(self, 19)
    }
    ///Bit 20 - illegal access flag clear for peripheral 180
    #[inline(always)]
    pub fn iaf180(&mut self) -> IAF180_W<'_, ICR5rs> {
        IAF180_W::new(self, 20)
    }
    ///Bit 21 - illegal access flag clear for peripheral 181
    #[inline(always)]
    pub fn iaf181(&mut self) -> IAF181_W<'_, ICR5rs> {
        IAF181_W::new(self, 21)
    }
    ///Bit 22 - illegal access flag clear for peripheral 182
    #[inline(always)]
    pub fn iaf182(&mut self) -> IAF182_W<'_, ICR5rs> {
        IAF182_W::new(self, 22)
    }
    ///Bit 23 - illegal access flag clear for peripheral 183
    #[inline(always)]
    pub fn iaf183(&mut self) -> IAF183_W<'_, ICR5rs> {
        IAF183_W::new(self, 23)
    }
    ///Bit 24 - illegal access flag clear for peripheral 184
    #[inline(always)]
    pub fn iaf184(&mut self) -> IAF184_W<'_, ICR5rs> {
        IAF184_W::new(self, 24)
    }
    ///Bit 25 - illegal access flag clear for peripheral 185
    #[inline(always)]
    pub fn iaf185(&mut self) -> IAF185_W<'_, ICR5rs> {
        IAF185_W::new(self, 25)
    }
    ///Bit 26 - illegal access flag clear for peripheral 186
    #[inline(always)]
    pub fn iaf186(&mut self) -> IAF186_W<'_, ICR5rs> {
        IAF186_W::new(self, 26)
    }
    ///Bit 27 - illegal access flag clear for peripheral 187
    #[inline(always)]
    pub fn iaf187(&mut self) -> IAF187_W<'_, ICR5rs> {
        IAF187_W::new(self, 27)
    }
    ///Bit 28 - illegal access flag clear for peripheral 188
    #[inline(always)]
    pub fn iaf188(&mut self) -> IAF188_W<'_, ICR5rs> {
        IAF188_W::new(self, 28)
    }
    ///Bit 29 - illegal access flag clear for peripheral 189
    #[inline(always)]
    pub fn iaf189(&mut self) -> IAF189_W<'_, ICR5rs> {
        IAF189_W::new(self, 29)
    }
    ///Bit 30 - illegal access flag clear for peripheral 190
    #[inline(always)]
    pub fn iaf190(&mut self) -> IAF190_W<'_, ICR5rs> {
        IAF190_W::new(self, 30)
    }
    ///Bit 31 - illegal access flag clear for peripheral 191
    #[inline(always)]
    pub fn iaf191(&mut self) -> IAF191_W<'_, ICR5rs> {
        IAF191_W::new(self, 31)
    }
}
/**IAC interrupt clear register 5

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr5::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#IAC:ICR5)*/
pub struct ICR5rs;
impl crate::RegisterSpec for ICR5rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`icr5::W`](W) writer structure
impl crate::Writable for ICR5rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICR5 to value 0
impl crate::Resettable for ICR5rs {}
