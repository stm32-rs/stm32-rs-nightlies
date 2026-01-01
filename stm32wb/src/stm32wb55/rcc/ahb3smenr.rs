///Register `AHB3SMENR` reader
pub type R = crate::R<AHB3SMENRrs>;
///Register `AHB3SMENR` writer
pub type W = crate::W<AHB3SMENRrs>;
///Field `QSPISMEN` reader - QSPISMEN
pub type QSPISMEN_R = crate::BitReader;
///Field `QSPISMEN` writer - QSPISMEN
pub type QSPISMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PKASMEN` reader - PKA accelerator clocks enable during CPU1 sleep mode
pub type PKASMEN_R = crate::BitReader;
///Field `PKASMEN` writer - PKA accelerator clocks enable during CPU1 sleep mode
pub type PKASMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AES2SMEN` reader - AES2 accelerator clocks enable during CPU1 sleep mode
pub type AES2SMEN_R = crate::BitReader;
///Field `AES2SMEN` writer - AES2 accelerator clocks enable during CPU1 sleep mode
pub type AES2SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RNGSMEN` reader - True RNG clocks enable during CPU1 sleep mode
pub type RNGSMEN_R = crate::BitReader;
///Field `RNGSMEN` writer - True RNG clocks enable during CPU1 sleep mode
pub type RNGSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRAM2SMEN` reader - SRAM2a and SRAM2b memory interface clocks enable during CPU1 sleep mode
pub type SRAM2SMEN_R = crate::BitReader;
///Field `SRAM2SMEN` writer - SRAM2a and SRAM2b memory interface clocks enable during CPU1 sleep mode
pub type SRAM2SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLASHSMEN` reader - Flash interface clocks enable during CPU1 sleep mode
pub type FLASHSMEN_R = crate::BitReader;
///Field `FLASHSMEN` writer - Flash interface clocks enable during CPU1 sleep mode
pub type FLASHSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 8 - QSPISMEN
    #[inline(always)]
    pub fn qspismen(&self) -> QSPISMEN_R {
        QSPISMEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 16 - PKA accelerator clocks enable during CPU1 sleep mode
    #[inline(always)]
    pub fn pkasmen(&self) -> PKASMEN_R {
        PKASMEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - AES2 accelerator clocks enable during CPU1 sleep mode
    #[inline(always)]
    pub fn aes2smen(&self) -> AES2SMEN_R {
        AES2SMEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - True RNG clocks enable during CPU1 sleep mode
    #[inline(always)]
    pub fn rngsmen(&self) -> RNGSMEN_R {
        RNGSMEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 24 - SRAM2a and SRAM2b memory interface clocks enable during CPU1 sleep mode
    #[inline(always)]
    pub fn sram2smen(&self) -> SRAM2SMEN_R {
        SRAM2SMEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Flash interface clocks enable during CPU1 sleep mode
    #[inline(always)]
    pub fn flashsmen(&self) -> FLASHSMEN_R {
        FLASHSMEN_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB3SMENR")
            .field("flashsmen", &self.flashsmen())
            .field("sram2smen", &self.sram2smen())
            .field("rngsmen", &self.rngsmen())
            .field("aes2smen", &self.aes2smen())
            .field("pkasmen", &self.pkasmen())
            .field("qspismen", &self.qspismen())
            .finish()
    }
}
impl W {
    ///Bit 8 - QSPISMEN
    #[inline(always)]
    pub fn qspismen(&mut self) -> QSPISMEN_W<'_, AHB3SMENRrs> {
        QSPISMEN_W::new(self, 8)
    }
    ///Bit 16 - PKA accelerator clocks enable during CPU1 sleep mode
    #[inline(always)]
    pub fn pkasmen(&mut self) -> PKASMEN_W<'_, AHB3SMENRrs> {
        PKASMEN_W::new(self, 16)
    }
    ///Bit 17 - AES2 accelerator clocks enable during CPU1 sleep mode
    #[inline(always)]
    pub fn aes2smen(&mut self) -> AES2SMEN_W<'_, AHB3SMENRrs> {
        AES2SMEN_W::new(self, 17)
    }
    ///Bit 18 - True RNG clocks enable during CPU1 sleep mode
    #[inline(always)]
    pub fn rngsmen(&mut self) -> RNGSMEN_W<'_, AHB3SMENRrs> {
        RNGSMEN_W::new(self, 18)
    }
    ///Bit 24 - SRAM2a and SRAM2b memory interface clocks enable during CPU1 sleep mode
    #[inline(always)]
    pub fn sram2smen(&mut self) -> SRAM2SMEN_W<'_, AHB3SMENRrs> {
        SRAM2SMEN_W::new(self, 24)
    }
    ///Bit 25 - Flash interface clocks enable during CPU1 sleep mode
    #[inline(always)]
    pub fn flashsmen(&mut self) -> FLASHSMEN_W<'_, AHB3SMENRrs> {
        FLASHSMEN_W::new(self, 25)
    }
}
/**AHB3 peripheral clocks enable in Sleep and Stop modes register

You can [`read`](crate::Reg::read) this register and get [`ahb3smenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb3smenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#RCC:AHB3SMENR)*/
pub struct AHB3SMENRrs;
impl crate::RegisterSpec for AHB3SMENRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb3smenr::R`](R) reader structure
impl crate::Readable for AHB3SMENRrs {}
///`write(|w| ..)` method takes [`ahb3smenr::W`](W) writer structure
impl crate::Writable for AHB3SMENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB3SMENR to value 0x0307_0100
impl crate::Resettable for AHB3SMENRrs {
    const RESET_VALUE: u32 = 0x0307_0100;
}
