///Register `MC_MLAHBLPENSETR` reader
pub type R = crate::R<MC_MLAHBLPENSETRrs>;
///Register `MC_MLAHBLPENSETR` writer
pub type W = crate::W<MC_MLAHBLPENSETRrs>;
///Field `SRAM1LPEN` reader - SRAM1LPEN
pub type SRAM1LPEN_R = crate::BitReader;
///Field `SRAM1LPEN` writer - SRAM1LPEN
pub type SRAM1LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRAM2LPEN` reader - SRAM2LPEN
pub type SRAM2LPEN_R = crate::BitReader;
///Field `SRAM2LPEN` writer - SRAM2LPEN
pub type SRAM2LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRAM34LPEN` reader - SRAM34LPEN
pub type SRAM34LPEN_R = crate::BitReader;
///Field `SRAM34LPEN` writer - SRAM34LPEN
pub type SRAM34LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RETRAMLPEN` reader - RETRAMLPEN
pub type RETRAMLPEN_R = crate::BitReader;
///Field `RETRAMLPEN` writer - RETRAMLPEN
pub type RETRAMLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - SRAM1LPEN
    #[inline(always)]
    pub fn sram1lpen(&self) -> SRAM1LPEN_R {
        SRAM1LPEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - SRAM2LPEN
    #[inline(always)]
    pub fn sram2lpen(&self) -> SRAM2LPEN_R {
        SRAM2LPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - SRAM34LPEN
    #[inline(always)]
    pub fn sram34lpen(&self) -> SRAM34LPEN_R {
        SRAM34LPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - RETRAMLPEN
    #[inline(always)]
    pub fn retramlpen(&self) -> RETRAMLPEN_R {
        RETRAMLPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MC_MLAHBLPENSETR")
            .field("sram1lpen", &self.sram1lpen())
            .field("sram2lpen", &self.sram2lpen())
            .field("sram34lpen", &self.sram34lpen())
            .field("retramlpen", &self.retramlpen())
            .finish()
    }
}
impl W {
    ///Bit 0 - SRAM1LPEN
    #[inline(always)]
    pub fn sram1lpen(&mut self) -> SRAM1LPEN_W<'_, MC_MLAHBLPENSETRrs> {
        SRAM1LPEN_W::new(self, 0)
    }
    ///Bit 1 - SRAM2LPEN
    #[inline(always)]
    pub fn sram2lpen(&mut self) -> SRAM2LPEN_W<'_, MC_MLAHBLPENSETRrs> {
        SRAM2LPEN_W::new(self, 1)
    }
    ///Bit 2 - SRAM34LPEN
    #[inline(always)]
    pub fn sram34lpen(&mut self) -> SRAM34LPEN_W<'_, MC_MLAHBLPENSETRrs> {
        SRAM34LPEN_W::new(self, 2)
    }
    ///Bit 4 - RETRAMLPEN
    #[inline(always)]
    pub fn retramlpen(&mut self) -> RETRAMLPEN_W<'_, MC_MLAHBLPENSETRrs> {
        RETRAMLPEN_W::new(self, 4)
    }
}
/**This register is used by the MCU in order to set the PERxLPEN bit of the corresponding peripheral.

You can [`read`](crate::Reg::read) this register and get [`mc_mlahblpensetr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mc_mlahblpensetr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MC_MLAHBLPENSETR)*/
pub struct MC_MLAHBLPENSETRrs;
impl crate::RegisterSpec for MC_MLAHBLPENSETRrs {
    type Ux = u32;
}
///`read()` method returns [`mc_mlahblpensetr::R`](R) reader structure
impl crate::Readable for MC_MLAHBLPENSETRrs {}
///`write(|w| ..)` method takes [`mc_mlahblpensetr::W`](W) writer structure
impl crate::Writable for MC_MLAHBLPENSETRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MC_MLAHBLPENSETR to value 0x17
impl crate::Resettable for MC_MLAHBLPENSETRrs {
    const RESET_VALUE: u32 = 0x17;
}
