///Register `DCR1` reader
pub type R = crate::R<DCR1rs>;
///Register `DCR1` writer
pub type W = crate::W<DCR1rs>;
///Field `CKMODE` reader - clock mode 0
pub type CKMODE_R = crate::BitReader;
///Field `FRCK` reader - Free running clock
pub type FRCK_R = crate::BitReader;
///Field `FRCK` writer - Free running clock
pub type FRCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSHT` reader - Chip-select high time
pub type CSHT_R = crate::FieldReader;
///Field `CSHT` writer - Chip-select high time
pub type CSHT_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `DEVSIZE` reader - Device size
pub type DEVSIZE_R = crate::FieldReader;
///Field `DEVSIZE` writer - Device size
pub type DEVSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `EXTENDMEM` reader - extended memory support
pub type EXTENDMEM_R = crate::BitReader;
///Field `EXTENDMEM` writer - extended memory support
pub type EXTENDMEM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MTYP` reader - Memory type
pub type MTYP_R = crate::FieldReader;
///Field `MTYP` writer - Memory type
pub type MTYP_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bit 0 - clock mode 0
    #[inline(always)]
    pub fn ckmode(&self) -> CKMODE_R {
        CKMODE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Free running clock
    #[inline(always)]
    pub fn frck(&self) -> FRCK_R {
        FRCK_R::new(((self.bits >> 1) & 1) != 0)
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
    ///Bit 21 - extended memory support
    #[inline(always)]
    pub fn extendmem(&self) -> EXTENDMEM_R {
        EXTENDMEM_R::new(((self.bits >> 21) & 1) != 0)
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
            .field("csht", &self.csht())
            .field("devsize", &self.devsize())
            .field("extendmem", &self.extendmem())
            .field("mtyp", &self.mtyp())
            .finish()
    }
}
impl W {
    ///Bit 1 - Free running clock
    #[inline(always)]
    pub fn frck(&mut self) -> FRCK_W<'_, DCR1rs> {
        FRCK_W::new(self, 1)
    }
    ///Bits 8:13 - Chip-select high time
    #[inline(always)]
    pub fn csht(&mut self) -> CSHT_W<'_, DCR1rs> {
        CSHT_W::new(self, 8)
    }
    ///Bits 16:20 - Device size
    #[inline(always)]
    pub fn devsize(&mut self) -> DEVSIZE_W<'_, DCR1rs> {
        DEVSIZE_W::new(self, 16)
    }
    ///Bit 21 - extended memory support
    #[inline(always)]
    pub fn extendmem(&mut self) -> EXTENDMEM_W<'_, DCR1rs> {
        EXTENDMEM_W::new(self, 21)
    }
    ///Bits 24:26 - Memory type
    #[inline(always)]
    pub fn mtyp(&mut self) -> MTYP_W<'_, DCR1rs> {
        MTYP_W::new(self, 24)
    }
}
/**XSPI device configuration register 1

You can [`read`](crate::Reg::read) this register and get [`dcr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#XSPI1:DCR1)*/
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
