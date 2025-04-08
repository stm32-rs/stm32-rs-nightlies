///Register `DCR1` reader
pub type R = crate::R<DCR1rs>;
///Register `DCR1` writer
pub type W = crate::W<DCR1rs>;
///Field `CKMODE` reader - Mode 0 / mode 3
pub type CKMODE_R = crate::BitReader;
///Field `CKMODE` writer - Mode 0 / mode 3
pub type CKMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FRCK` reader - Free running clock
pub type FRCK_R = crate::BitReader;
///Field `FRCK` writer - Free running clock
pub type FRCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DLYBYP` reader - Delay block bypass
pub type DLYBYP_R = crate::BitReader;
///Field `DLYBYP` writer - Delay block bypass
pub type DLYBYP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSHT` reader - Chip-select high time
pub type CSHT_R = crate::FieldReader;
///Field `CSHT` writer - Chip-select high time
pub type CSHT_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `DEVSIZE` reader - Device size
pub type DEVSIZE_R = crate::FieldReader;
///Field `DEVSIZE` writer - Device size
pub type DEVSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `MTYP` reader - Memory type
pub type MTYP_R = crate::FieldReader;
///Field `MTYP` writer - Memory type
pub type MTYP_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bit 0 - Mode 0 / mode 3
    #[inline(always)]
    pub fn ckmode(&self) -> CKMODE_R {
        CKMODE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Free running clock
    #[inline(always)]
    pub fn frck(&self) -> FRCK_R {
        FRCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - Delay block bypass
    #[inline(always)]
    pub fn dlybyp(&self) -> DLYBYP_R {
        DLYBYP_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 8:13 - Chip-select high time
    #[inline(always)]
    pub fn csht(&self) -> CSHT_R {
        CSHT_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    ///Bits 16:20 - Device size
    #[inline(always)]
    pub fn devsize(&self) -> DEVSIZE_R {
        DEVSIZE_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bits 24:26 - Memory type
    #[inline(always)]
    pub fn mtyp(&self) -> MTYP_R {
        MTYP_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCR1")
            .field("ckmode", &self.ckmode())
            .field("frck", &self.frck())
            .field("dlybyp", &self.dlybyp())
            .field("csht", &self.csht())
            .field("devsize", &self.devsize())
            .field("mtyp", &self.mtyp())
            .finish()
    }
}
impl W {
    ///Bit 0 - Mode 0 / mode 3
    #[inline(always)]
    pub fn ckmode(&mut self) -> CKMODE_W<DCR1rs> {
        CKMODE_W::new(self, 0)
    }
    ///Bit 1 - Free running clock
    #[inline(always)]
    pub fn frck(&mut self) -> FRCK_W<DCR1rs> {
        FRCK_W::new(self, 1)
    }
    ///Bit 3 - Delay block bypass
    #[inline(always)]
    pub fn dlybyp(&mut self) -> DLYBYP_W<DCR1rs> {
        DLYBYP_W::new(self, 3)
    }
    ///Bits 8:13 - Chip-select high time
    #[inline(always)]
    pub fn csht(&mut self) -> CSHT_W<DCR1rs> {
        CSHT_W::new(self, 8)
    }
    ///Bits 16:20 - Device size
    #[inline(always)]
    pub fn devsize(&mut self) -> DEVSIZE_W<DCR1rs> {
        DEVSIZE_W::new(self, 16)
    }
    ///Bits 24:26 - Memory type
    #[inline(always)]
    pub fn mtyp(&mut self) -> MTYP_W<DCR1rs> {
        MTYP_W::new(self, 24)
    }
}
/**device configuration register

You can [`read`](crate::Reg::read) this register and get [`dcr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H725.html#OCTOSPI1:DCR1)*/
pub struct DCR1rs;
impl crate::RegisterSpec for DCR1rs {
    type Ux = u32;
}
///`read()` method returns [`dcr1::R`](R) reader structure
impl crate::Readable for DCR1rs {}
///`write(|w| ..)` method takes [`dcr1::W`](W) writer structure
impl crate::Writable for DCR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DCR1 to value 0
impl crate::Resettable for DCR1rs {}
