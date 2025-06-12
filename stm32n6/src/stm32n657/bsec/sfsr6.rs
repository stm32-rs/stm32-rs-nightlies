///Register `SFSR6` reader
pub type R = crate::R<SFSR6rs>;
///Field `SFW192` reader - Shadowed fuse word 192
pub type SFW192_R = crate::BitReader;
///Field `SFW193` reader - Shadowed fuse word 193
pub type SFW193_R = crate::BitReader;
///Field `SFW194` reader - Shadowed fuse word 194
pub type SFW194_R = crate::BitReader;
///Field `SFW195` reader - Shadowed fuse word 195
pub type SFW195_R = crate::BitReader;
///Field `SFW196` reader - Shadowed fuse word 196
pub type SFW196_R = crate::BitReader;
///Field `SFW197` reader - Shadowed fuse word 197
pub type SFW197_R = crate::BitReader;
///Field `SFW198` reader - Shadowed fuse word 198
pub type SFW198_R = crate::BitReader;
///Field `SFW199` reader - Shadowed fuse word 199
pub type SFW199_R = crate::BitReader;
///Field `SFW200` reader - Shadowed fuse word 200
pub type SFW200_R = crate::BitReader;
///Field `SFW201` reader - Shadowed fuse word 201
pub type SFW201_R = crate::BitReader;
///Field `SFW202` reader - Shadowed fuse word 202
pub type SFW202_R = crate::BitReader;
///Field `SFW203` reader - Shadowed fuse word 203
pub type SFW203_R = crate::BitReader;
///Field `SFW204` reader - Shadowed fuse word 204
pub type SFW204_R = crate::BitReader;
///Field `SFW205` reader - Shadowed fuse word 205
pub type SFW205_R = crate::BitReader;
///Field `SFW206` reader - Shadowed fuse word 206
pub type SFW206_R = crate::BitReader;
///Field `SFW207` reader - Shadowed fuse word 207
pub type SFW207_R = crate::BitReader;
///Field `SFW208` reader - Shadowed fuse word 208
pub type SFW208_R = crate::BitReader;
///Field `SFW209` reader - Shadowed fuse word 209
pub type SFW209_R = crate::BitReader;
///Field `SFW210` reader - Shadowed fuse word 210
pub type SFW210_R = crate::BitReader;
///Field `SFW211` reader - Shadowed fuse word 211
pub type SFW211_R = crate::BitReader;
///Field `SFW212` reader - Shadowed fuse word 212
pub type SFW212_R = crate::BitReader;
///Field `SFW213` reader - Shadowed fuse word 213
pub type SFW213_R = crate::BitReader;
///Field `SFW214` reader - Shadowed fuse word 214
pub type SFW214_R = crate::BitReader;
///Field `SFW215` reader - Shadowed fuse word 215
pub type SFW215_R = crate::BitReader;
///Field `SFW216` reader - Shadowed fuse word 216
pub type SFW216_R = crate::BitReader;
///Field `SFW217` reader - Shadowed fuse word 217
pub type SFW217_R = crate::BitReader;
///Field `SFW218` reader - Shadowed fuse word 218
pub type SFW218_R = crate::BitReader;
///Field `SFW219` reader - Shadowed fuse word 219
pub type SFW219_R = crate::BitReader;
///Field `SFW220` reader - Shadowed fuse word 220
pub type SFW220_R = crate::BitReader;
///Field `SFW221` reader - Shadowed fuse word 221
pub type SFW221_R = crate::BitReader;
///Field `SFW222` reader - Shadowed fuse word 222
pub type SFW222_R = crate::BitReader;
///Field `SFW223` reader - Shadowed fuse word 223
pub type SFW223_R = crate::BitReader;
impl R {
    ///Bit 0 - Shadowed fuse word 192
    #[inline(always)]
    pub fn sfw192(&self) -> SFW192_R {
        SFW192_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Shadowed fuse word 193
    #[inline(always)]
    pub fn sfw193(&self) -> SFW193_R {
        SFW193_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Shadowed fuse word 194
    #[inline(always)]
    pub fn sfw194(&self) -> SFW194_R {
        SFW194_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Shadowed fuse word 195
    #[inline(always)]
    pub fn sfw195(&self) -> SFW195_R {
        SFW195_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Shadowed fuse word 196
    #[inline(always)]
    pub fn sfw196(&self) -> SFW196_R {
        SFW196_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Shadowed fuse word 197
    #[inline(always)]
    pub fn sfw197(&self) -> SFW197_R {
        SFW197_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Shadowed fuse word 198
    #[inline(always)]
    pub fn sfw198(&self) -> SFW198_R {
        SFW198_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Shadowed fuse word 199
    #[inline(always)]
    pub fn sfw199(&self) -> SFW199_R {
        SFW199_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Shadowed fuse word 200
    #[inline(always)]
    pub fn sfw200(&self) -> SFW200_R {
        SFW200_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Shadowed fuse word 201
    #[inline(always)]
    pub fn sfw201(&self) -> SFW201_R {
        SFW201_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Shadowed fuse word 202
    #[inline(always)]
    pub fn sfw202(&self) -> SFW202_R {
        SFW202_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Shadowed fuse word 203
    #[inline(always)]
    pub fn sfw203(&self) -> SFW203_R {
        SFW203_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Shadowed fuse word 204
    #[inline(always)]
    pub fn sfw204(&self) -> SFW204_R {
        SFW204_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Shadowed fuse word 205
    #[inline(always)]
    pub fn sfw205(&self) -> SFW205_R {
        SFW205_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Shadowed fuse word 206
    #[inline(always)]
    pub fn sfw206(&self) -> SFW206_R {
        SFW206_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Shadowed fuse word 207
    #[inline(always)]
    pub fn sfw207(&self) -> SFW207_R {
        SFW207_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Shadowed fuse word 208
    #[inline(always)]
    pub fn sfw208(&self) -> SFW208_R {
        SFW208_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Shadowed fuse word 209
    #[inline(always)]
    pub fn sfw209(&self) -> SFW209_R {
        SFW209_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Shadowed fuse word 210
    #[inline(always)]
    pub fn sfw210(&self) -> SFW210_R {
        SFW210_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Shadowed fuse word 211
    #[inline(always)]
    pub fn sfw211(&self) -> SFW211_R {
        SFW211_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Shadowed fuse word 212
    #[inline(always)]
    pub fn sfw212(&self) -> SFW212_R {
        SFW212_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Shadowed fuse word 213
    #[inline(always)]
    pub fn sfw213(&self) -> SFW213_R {
        SFW213_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Shadowed fuse word 214
    #[inline(always)]
    pub fn sfw214(&self) -> SFW214_R {
        SFW214_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Shadowed fuse word 215
    #[inline(always)]
    pub fn sfw215(&self) -> SFW215_R {
        SFW215_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Shadowed fuse word 216
    #[inline(always)]
    pub fn sfw216(&self) -> SFW216_R {
        SFW216_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Shadowed fuse word 217
    #[inline(always)]
    pub fn sfw217(&self) -> SFW217_R {
        SFW217_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Shadowed fuse word 218
    #[inline(always)]
    pub fn sfw218(&self) -> SFW218_R {
        SFW218_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Shadowed fuse word 219
    #[inline(always)]
    pub fn sfw219(&self) -> SFW219_R {
        SFW219_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Shadowed fuse word 220
    #[inline(always)]
    pub fn sfw220(&self) -> SFW220_R {
        SFW220_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Shadowed fuse word 221
    #[inline(always)]
    pub fn sfw221(&self) -> SFW221_R {
        SFW221_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Shadowed fuse word 222
    #[inline(always)]
    pub fn sfw222(&self) -> SFW222_R {
        SFW222_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Shadowed fuse word 223
    #[inline(always)]
    pub fn sfw223(&self) -> SFW223_R {
        SFW223_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SFSR6")
            .field("sfw192", &self.sfw192())
            .field("sfw193", &self.sfw193())
            .field("sfw194", &self.sfw194())
            .field("sfw195", &self.sfw195())
            .field("sfw196", &self.sfw196())
            .field("sfw197", &self.sfw197())
            .field("sfw198", &self.sfw198())
            .field("sfw199", &self.sfw199())
            .field("sfw200", &self.sfw200())
            .field("sfw201", &self.sfw201())
            .field("sfw202", &self.sfw202())
            .field("sfw203", &self.sfw203())
            .field("sfw204", &self.sfw204())
            .field("sfw205", &self.sfw205())
            .field("sfw206", &self.sfw206())
            .field("sfw207", &self.sfw207())
            .field("sfw208", &self.sfw208())
            .field("sfw209", &self.sfw209())
            .field("sfw210", &self.sfw210())
            .field("sfw211", &self.sfw211())
            .field("sfw212", &self.sfw212())
            .field("sfw213", &self.sfw213())
            .field("sfw214", &self.sfw214())
            .field("sfw215", &self.sfw215())
            .field("sfw216", &self.sfw216())
            .field("sfw217", &self.sfw217())
            .field("sfw218", &self.sfw218())
            .field("sfw219", &self.sfw219())
            .field("sfw220", &self.sfw220())
            .field("sfw221", &self.sfw221())
            .field("sfw222", &self.sfw222())
            .field("sfw223", &self.sfw223())
            .finish()
    }
}
/**BSEC shadowed fuses status register 6

You can [`read`](crate::Reg::read) this register and get [`sfsr6::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#BSEC:SFSR6)*/
pub struct SFSR6rs;
impl crate::RegisterSpec for SFSR6rs {
    type Ux = u32;
}
///`read()` method returns [`sfsr6::R`](R) reader structure
impl crate::Readable for SFSR6rs {}
///`reset()` method sets SFSR6 to value 0
impl crate::Resettable for SFSR6rs {}
